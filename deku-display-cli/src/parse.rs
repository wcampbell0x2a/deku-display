use anyhow::{Context, Result};
use syn::{Attribute, Fields, Item, Lit};

use crate::model::{EnumInfo, FieldInfo, StructInfo, TypeInfo, VariantInfo};

/// Parse a Rust source file and extract all types annotated with
/// `#[deku_display(...)]` or `#[deku(...)]` (with `#[derive(DekuRead)]` / `#[derive(DekuWrite)]`).
pub fn parse_file(source: &str) -> Result<Vec<TypeInfo>> {
    let file = syn::parse_file(source).context("failed to parse Rust source file")?;
    let mut types = Vec::new();

    for item in &file.items {
        match item {
            Item::Struct(item_struct) => {
                if let Some(info) = try_parse_struct(item_struct)? {
                    types.push(TypeInfo::Struct(info));
                }
            }
            Item::Enum(item_enum) => {
                if let Some(info) = try_parse_enum(item_enum)? {
                    types.push(TypeInfo::Enum(info));
                }
            }
            _ => {}
        }
    }

    Ok(types)
}

/// Parse `#[deku_display(...)]` attributes (bit_width, bitheader) from an attribute list.
fn parse_deku_display_attrs(
    attrs: &[Attribute],
    context_name: &str,
) -> Result<Option<(u8, Option<String>)>> {
    let mut bit_width: Option<u8> = None;
    let mut bitheader: Option<String> = None;

    for attr in attrs {
        if !attr.path().is_ident("deku_display") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("bit_width") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                bit_width = Some(parse_num_lit(&lit)?);
                return Ok(());
            }
            if meta.path.is_ident("bitheader") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                bitheader = Some(lit.value());
                return Ok(());
            }
            // Skip unknown attributes
            let _ = meta.value().and_then(|v| v.parse::<Lit>());
            Ok(())
        })
        .with_context(|| format!("failed to parse #[deku_display] on '{}'", context_name))?;
    }

    match bit_width {
        Some(bw) => Ok(Some((bw, bitheader))),
        None => Ok(None),
    }
}

/// Parse `#[deku(...)]` on an enum to extract `id_type`, `bits`, and `bytes`.
fn parse_deku_enum_attrs(attrs: &[Attribute]) -> Result<(Option<String>, Option<u8>)> {
    let mut id_type: Option<String> = None;
    let mut bits: Option<u8> = None;
    let mut bytes: Option<u8> = None;

    for attr in attrs {
        if !attr.path().is_ident("deku") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id_type") || meta.path.is_ident("type") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                id_type = Some(lit.value());
                return Ok(());
            }
            if meta.path.is_ident("bits") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                bits = Some(parse_num_lit(&lit)?);
                return Ok(());
            }
            if meta.path.is_ident("bytes") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                bytes = Some(parse_num_lit(&lit)?);
                return Ok(());
            }
            // Skip unknown deku attributes
            if meta.value().is_ok() {
                let _ = meta.input.parse::<syn::Lit>();
            }
            Ok(())
        })?;
    }

    // Compute id_bits: explicit bits > bytes*8 > infer from id_type
    let id_bits = if let Some(b) = bits {
        Some(b)
    } else if let Some(b) = bytes {
        Some(b * 8)
    } else if let Some(ref ty) = id_type {
        type_to_bits(ty)
    } else {
        None
    };

    Ok((id_type, id_bits.map(|b| b)))
}

/// Parse `#[deku(id = ..., id_pat = ...)]` on a variant.
fn parse_variant_deku_attrs(attrs: &[Attribute]) -> Result<(Option<String>, Option<String>)> {
    let mut id_value: Option<String> = None;
    let mut id_pat: Option<String> = None;

    for attr in attrs {
        if !attr.path().is_ident("deku") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                id_value = Some(lit_to_string(&lit));
                return Ok(());
            }
            if meta.path.is_ident("id_pat") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                id_pat = Some(lit.value());
                return Ok(());
            }
            // Skip unknown
            if meta.value().is_ok() {
                let _ = meta.input.parse::<syn::Lit>();
            }
            Ok(())
        })?;
    }

    Ok((id_value, id_pat))
}

/// Check if a type has `#[derive(...DekuRead...)]` or `#[derive(...DekuWrite...)]`.
fn has_deku_derive(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        if let Ok(nested) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) {
            for path in &nested {
                if let Some(ident) = path.segments.last() {
                    let name = ident.ident.to_string();
                    if name == "DekuRead" || name == "DekuWrite" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Infer bit_width from fields: sum all resolvable field bits, then pick the
/// largest standard width (8, 16, 32) that evenly divides the total.
fn infer_bit_width(fields: &[FieldInfo]) -> Option<u8> {
    let mut total: u32 = 0;
    for f in fields {
        match f.resolved_bits() {
            Ok(b) => total += b,
            Err(_) => return None,
        }
    }
    if total == 0 {
        return None;
    }
    Some(32)
}

fn try_parse_struct(item: &syn::ItemStruct) -> Result<Option<StructInfo>> {
    // First try #[deku_display(...)]
    if let Some((bit_width, bitheader)) =
        parse_deku_display_attrs(&item.attrs, &item.ident.to_string())?
    {
        let fields = match &item.fields {
            Fields::Named(named) => parse_named_fields(&named.named)?,
            _ => return Ok(None),
        };
        return Ok(Some(StructInfo {
            name: item.ident.to_string(),
            bit_width,
            bitheader,
            fields,
        }));
    }

    // Fall back to #[derive(DekuRead/DekuWrite)] structs
    if !has_deku_derive(&item.attrs) {
        return Ok(None);
    }

    let fields = match &item.fields {
        Fields::Named(named) => parse_named_fields(&named.named)?,
        _ => return Ok(None),
    };

    // Filter to only fields whose bit width we can resolve (drops Vec<T>, etc.)
    let resolvable_fields: Vec<FieldInfo> = fields
        .into_iter()
        .filter(|f| f.resolved_bits().is_ok())
        .collect();

    let bit_width = match infer_bit_width(&resolvable_fields) {
        Some(w) => w,
        None => return Ok(None),
    };

    Ok(Some(StructInfo {
        name: item.ident.to_string(),
        bit_width,
        bitheader: None,
        fields: resolvable_fields,
    }))
}

fn try_parse_enum(item: &syn::ItemEnum) -> Result<Option<EnumInfo>> {
    let deku_display = parse_deku_display_attrs(&item.attrs, &item.ident.to_string())?;
    let has_deku = has_deku_derive(&item.attrs);

    if deku_display.is_none() && !has_deku {
        return Ok(None);
    }

    // Parse enum-level deku attrs for id_type/bits
    let (id_type, id_bits_from_deku) = parse_deku_enum_attrs(&item.attrs)?;
    let id_bits = id_bits_from_deku.map(|b| b as u32);

    let mut variants = Vec::new();
    for variant in &item.variants {
        let fields = parse_variant_fields(&variant.fields)?;
        let (mut id_value, id_pat) = parse_variant_deku_attrs(&variant.attrs)?;

        // Fall back to the Rust discriminant expression (e.g. `Variant = 1`)
        if id_value.is_none() && id_pat.is_none() {
            if let Some((_, ref expr)) = variant.discriminant {
                id_value = Some(expr_to_string(expr));
            }
        }

        variants.push(VariantInfo {
            name: variant.ident.to_string(),
            id_value,
            id_pat,
            fields,
        });
    }

    // Determine bit_width
    let bit_width = if let Some((bw, _)) = deku_display {
        bw
    } else {
        // Infer from id_bits + first variant's fields
        let first_with_fields = variants.iter().find(|v| !v.fields.is_empty());
        let field_bits: u32 = first_with_fields
            .and_then(|v| {
                v.fields
                    .iter()
                    .map(|f| f.resolved_bits().ok())
                    .sum::<Option<u32>>()
            })
            .unwrap_or(0);
        let total = id_bits.unwrap_or(0) + field_bits;
        if total == 0 {
            // Unit-only enum with id_bits
            if let Some(ib) = id_bits {
                ib as u8
            } else {
                return Ok(None);
            }
        } else {
            infer_bit_width_from_total(total)
        }
    };

    let bitheader = deku_display.and_then(|(_, bh)| bh);

    Ok(Some(EnumInfo {
        name: item.ident.to_string(),
        bit_width,
        bitheader,
        id_bits,
        id_read: id_type.is_some(),
        variants,
    }))
}

/// Parse fields from a variant — handles both named and unnamed (tuple) fields.
fn parse_variant_fields(fields: &Fields) -> Result<Vec<FieldInfo>> {
    match fields {
        Fields::Named(named) => parse_named_fields(&named.named),
        Fields::Unnamed(unnamed) => {
            let mut result = Vec::new();
            for (i, field) in unnamed.unnamed.iter().enumerate() {
                let type_name = type_to_string(&field.ty);
                let (bits, unused) = parse_field_attrs(&field.attrs)?;
                result.push(FieldInfo {
                    name: format!("field_{}", i),
                    bits,
                    type_name,
                    unused,
                });
            }
            Ok(result)
        }
        Fields::Unit => Ok(Vec::new()),
    }
}

/// Parse named fields into a Vec<FieldInfo>.
fn parse_named_fields(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Result<Vec<FieldInfo>> {
    let mut result = Vec::new();
    for field in fields {
        let name = field.ident.as_ref().unwrap().to_string();
        let type_name = type_to_string(&field.ty);
        let (bits, unused) = parse_field_attrs(&field.attrs)?;
        result.push(FieldInfo {
            name,
            bits,
            type_name,
            unused,
        });
    }
    Ok(result)
}

/// Parse field-level attributes: `#[deku(bits = "N")]`, `#[deku(bytes = "N")]`,
/// `#[deku_display(bits = "N")]`, `#[deku_display(unused)]`.
fn parse_field_attrs(attrs: &[Attribute]) -> Result<(Option<u32>, bool)> {
    let mut bits: Option<u32> = None;
    let mut unused = false;

    for attr in attrs {
        let is_deku = attr.path().is_ident("deku");
        let is_deku_display = attr.path().is_ident("deku_display");
        if !is_deku && !is_deku_display {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("bits") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                bits = Some(parse_num_lit(&lit)?.into());
                return Ok(());
            }
            if meta.path.is_ident("bytes") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                let b: u32 = parse_num_lit(&lit)?.into();
                bits = Some(b * 8);
                return Ok(());
            }
            if meta.path.is_ident("unused") {
                unused = true;
                return Ok(());
            }
            // Skip unknown attributes (e.g., deku's `endian`, `count`, etc.)
            if meta.value().is_ok() {
                // Already consumed the `=`, parse and discard the value
                let _ = meta.input.parse::<syn::Lit>();
            }
            Ok(())
        })?;
    }

    Ok((bits, unused))
}

/// Parse a literal that may be either an integer (`32`) or a string containing an integer (`"32"`).
fn parse_num_lit(lit: &Lit) -> syn::parse::Result<u8> {
    match lit {
        Lit::Int(int_lit) => int_lit.base10_parse::<u8>(),
        Lit::Str(str_lit) => {
            let val: u8 = str_lit.value().parse().map_err(|_| {
                syn::Error::new(
                    str_lit.span(),
                    format!("invalid integer: {}", str_lit.value()),
                )
            })?;
            Ok(val)
        }
        _ => Err(syn::Error::new(
            syn::spanned::Spanned::span(lit),
            "expected integer or string literal",
        )),
    }
}

/// Convert a literal to its string representation.
fn lit_to_string(lit: &Lit) -> String {
    match lit {
        Lit::Int(i) => i.to_string(),
        Lit::Str(s) => s.value(),
        Lit::Byte(b) => format!("{}", b.value()),
        _ => "?".to_string(),
    }
}

/// Convert a syn::Type to a simple string (e.g., "u8", "u32", "MyStruct").
fn type_to_string(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident.to_string();
        }
    }
    "unknown".to_string()
}

/// Get bit width of a primitive type name.
fn type_to_bits(ty: &str) -> Option<u8> {
    match ty {
        "u8" | "i8" | "bool" => Some(8),
        "u16" | "i16" => Some(16),
        "u32" | "i32" | "f32" => Some(32),
        "u64" | "i64" | "f64" => Some(64),
        "u128" | "i128" => Some(128),
        _ => None,
    }
}

/// Convert a syn::Expr to a simple string representation for discriminant values.
fn expr_to_string(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Lit(lit) => lit_to_string(&lit.lit),
        syn::Expr::Unary(unary) => {
            let inner = expr_to_string(&unary.expr);
            match unary.op {
                syn::UnOp::Neg(_) => format!("-{inner}"),
                syn::UnOp::Not(_) => format!("!{inner}"),
                _ => inner,
            }
        }
        _ => "?".to_string(),
    }
}

fn infer_bit_width_from_total(_total: u32) -> u8 {
    32
}
