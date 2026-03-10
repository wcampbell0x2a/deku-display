use anyhow::Result;

use crate::model::{EnumInfo, FieldInfo, StructInfo, TypeInfo, VariantInfo};

/// Generate LaTeX bytefield output for a single type.
pub fn generate(info: &TypeInfo) -> Result<String> {
    match info {
        TypeInfo::Struct(s) => generate_struct(s),
        TypeInfo::Enum(e) => generate_enum(e),
    }
}

fn generate_bytefield(
    bit_width: u8,
    bitheader: &Option<String>,
    fields: &[FieldInfo],
    caption: &str,
) -> Result<String> {
    let bitheader_str = bitheader
        .as_deref()
        .map(String::from)
        .unwrap_or_else(|| format!("0-{}", bit_width - 1));

    let mut output = String::new();
    output.push_str("\\begin{figure}\n");
    output.push_str(&format!("\\begin{{bytefield}}{{{}}}\n", bit_width));
    output.push_str(&format!("\\bitheader{{{}}} \\\\\n", bitheader_str));

    let mut total_bits: u32 = 0;
    for field in fields {
        let bits = field.resolved_bits()?;

        let (option, label) = if field.unused {
            ("[bgcolor=lightgray]", String::new())
        } else {
            ("", field.name.replace('_', "\\_"))
        };

        output.push_str(&format!("\\bitbox{{{}}}{}{{{}}}", bits, option, label));

        total_bits += bits;
        if total_bits % (bit_width as u32) == 0 {
            output.push_str(" \\\\\n");
        } else {
            output.push_str(" & ");
        }
    }

    output.push_str("\\end{bytefield}\n");
    output.push_str(&format!("\\caption{{{}}}\n", caption));
    output.push_str("\\end{figure}");

    Ok(output)
}

fn generate_struct(info: &StructInfo) -> Result<String> {
    generate_bytefield(info.bit_width, &info.bitheader, &info.fields, &info.name)
}

fn generate_enum(info: &EnumInfo) -> Result<String> {
    let mut parts = Vec::new();
    for variant in &info.variants {
        let fields = variant_fields_with_id(info, variant);
        if fields.is_empty() {
            continue;
        }
        let caption = format!("{}::{}", info.name, variant.name);
        parts.push(generate_bytefield(
            info.bit_width,
            &info.bitheader,
            &fields,
            &caption,
        )?);
    }
    Ok(parts.join("\n"))
}

/// Build the field list for a variant, prepending the enum discriminant if present.
fn variant_fields_with_id(e: &EnumInfo, variant: &VariantInfo) -> Vec<FieldInfo> {
    let mut fields = Vec::new();

    if let Some(id_bits) = e.id_bits {
        let id_label = if let Some(ref val) = variant.id_value {
            format!("id={}", val)
        } else if let Some(ref pat) = variant.id_pat {
            format!("id={}", pat)
        } else {
            "id".to_string()
        };
        fields.push(FieldInfo {
            name: id_label,
            bits: Some(id_bits),
            type_name: String::new(),
            unused: false,
        });
    }

    if variant.fields.is_empty() {
        if fields.is_empty() {
            return Vec::new();
        }
        return fields;
    }

    for f in &variant.fields {
        if f.resolved_bits().is_err() {
            return Vec::new();
        }
    }

    fields.extend(variant.fields.iter().map(|f| FieldInfo {
        name: f.name.clone(),
        bits: f.bits,
        type_name: f.type_name.clone(),
        unused: f.unused,
    }));

    fields
}
