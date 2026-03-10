use std::collections::HashSet;

use anyhow::Result;

use crate::model::{EnumInfo, FieldInfo, TypeInfo, VariantInfo};

/// Generate RST sphinxcontrib-bitfield output for a single type.
pub fn generate(info: &TypeInfo) -> Result<String> {
    match info {
        TypeInfo::Struct(s) => generate_bitfield(&s.fields, s.bit_width as usize),
        TypeInfo::Enum(e) => {
            let mut parts = Vec::new();
            for variant in &e.variants {
                let fields = variant_fields_with_id(e, variant);
                if fields.is_empty() {
                    continue;
                }
                parts.push(generate_bitfield(&fields, e.bit_width as usize)?);
            }
            Ok(parts.join("\n\n"))
        }
    }
}

/// Generate a full RST page with TOC, anchors, and cross-references for multiple types.
pub fn generate_page(types: &[TypeInfo]) -> Result<String> {
    let type_names: HashSet<String> = types.iter().map(|t| t.name().to_string()).collect();

    let mut output = String::new();

    // Page title
    output.push_str("Protocol Types\n");
    output.push_str("==============\n\n");
    output.push_str(".. contents::\n");
    output.push_str("    :local:\n\n");

    for ty in types {
        match ty {
            TypeInfo::Struct(s) => {
                // Anchor + section header
                output.push_str(&format!(".. _{}:\n\n", s.name));
                let underline = "-".repeat(s.name.len());
                output.push_str(&format!("{}\n{}\n\n", s.name, underline));

                // Bitfield diagram
                output.push_str(&generate_bitfield(&s.fields, s.bit_width as usize)?);
                output.push('\n');

                // Cross-references
                let crossrefs = collect_crossrefs(&s.fields, &type_names);
                if !crossrefs.is_empty() {
                    output.push('\n');
                    output.push_str(&format_crossrefs(&crossrefs));
                }

                output.push('\n');
            }
            TypeInfo::Enum(e) => {
                // Anchor + section header
                output.push_str(&format!(".. _{}:\n\n", e.name));
                let underline = "-".repeat(e.name.len());
                output.push_str(&format!("{}\n{}\n\n", e.name, underline));

                // ID table
                output.push_str(&generate_id_table(e));

                // ID source note
                if e.id_bits.is_some() {
                    output.push('\n');
                    if e.id_read {
                        output.push_str(&format!(
                            "*ID is read from the data ({} bits).*\n",
                            e.id_bits.unwrap()
                        ));
                    } else {
                        output.push_str(&format!(
                            "*ID is from previous context ({} bits).*\n",
                            e.id_bits.unwrap()
                        ));
                    }
                }

                output.push('\n');

                // Per-variant sub-sections
                for variant in &e.variants {
                    let fields = variant_fields_with_id(e, variant);
                    if fields.is_empty() {
                        continue;
                    }

                    let anchor = format!("{}-{}", e.name, variant.name);
                    let heading = format!("{}::{}", e.name, variant.name);
                    let sub_underline = "~".repeat(heading.len());

                    output.push_str(&format!("\n.. _{}:\n\n", anchor));
                    output.push_str(&format!("{}\n{}\n\n", heading, sub_underline));

                    output
                        .push_str(&generate_bitfield(&fields, e.bit_width as usize)?);
                    output.push('\n');

                    let crossrefs = collect_crossrefs(&fields, &type_names);
                    if !crossrefs.is_empty() {
                        output.push('\n');
                        output.push_str(&format_crossrefs(&crossrefs));
                        output.push('\n');
                    }
                }

                output.push('\n');
            }
        }
    }

    // Trim trailing whitespace
    let output = output.trim_end().to_string();
    Ok(output)
}

/// Build the field list for a variant, prepending the enum discriminant if present.
fn variant_fields_with_id(e: &EnumInfo, variant: &VariantInfo) -> Vec<FieldInfo> {
    let mut fields = Vec::new();

    // Prepend ID field if the enum has a discriminant width
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

    // Only include variant if it has resolvable fields (or is unit with just the ID)
    if variant.fields.is_empty() {
        // Unit variant — show just the ID field if present
        if fields.is_empty() {
            return Vec::new();
        }
        return fields;
    }

    // Check all fields are resolvable before including
    for f in &variant.fields {
        if f.resolved_bits().is_err() {
            return Vec::new(); // Skip variants with unresolvable fields
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

fn generate_bitfield(fields: &[FieldInfo], bit_width: usize) -> Result<String> {
    let mut entries = Vec::new();
    let mut total_bits: usize = 0;

    for field in fields {
        let bits = field.resolved_bits()? as usize;
        total_bits += bits;
    }

    // sphinxcontrib-bitfield requires bits > 4
    if total_bits < 5 {
        return Ok(String::new());
    }

    total_bits = 0;
    for field in fields {
        let bits = field.resolved_bits()? as usize;
        total_bits += bits;

        if field.unused {
            entries.push(format!("{{ \"bits\": {} }}", bits));
        } else {
            entries.push(format!(
                "{{ \"name\": \"{}\", \"bits\": {} }}",
                field.name, bits
            ));
        }
    }

    let lanes = (total_bits + bit_width - 1) / bit_width;

    let mut output = String::new();
    output.push_str(".. bitfield::\n");
    output.push_str(&format!("    :bits: {}\n", total_bits));
    output.push_str(&format!("    :lanes: {}\n", lanes));
    output.push_str("    :hflip:\n");
    output.push_str("    :vflip:\n");
    output.push_str("\n    [\n");

    for (i, entry) in entries.iter().enumerate() {
        if i < entries.len() - 1 {
            output.push_str(&format!("        {},\n", entry));
        } else {
            output.push_str(&format!("        {}\n", entry));
        }
    }

    output.push_str("    ]");

    Ok(output)
}

/// Generate an ID-to-variant mapping table for an enum.
fn generate_id_table(info: &EnumInfo) -> String {
    let mut output = String::new();
    output.push_str(".. list-table:: Variant IDs\n");
    output.push_str("    :header-rows: 1\n\n");
    output.push_str("    * - ID\n");
    output.push_str("      - Variant\n");

    for variant in &info.variants {
        let id_str = if let Some(ref val) = variant.id_value {
            val.clone()
        } else if let Some(ref pat) = variant.id_pat {
            pat.clone()
        } else {
            "—".to_string()
        };
        output.push_str(&format!("    * - ``{}``\n", id_str));
        output.push_str(&format!("      - {}\n", variant.name));
    }

    output
}

/// Collect (field_name, type_name) pairs where type_name matches a known type.
fn collect_crossrefs(fields: &[FieldInfo], type_names: &HashSet<String>) -> Vec<(String, String)> {
    let mut refs = Vec::new();
    for field in fields {
        if type_names.contains(&field.type_name) {
            refs.push((field.name.clone(), field.type_name.clone()));
        }
    }
    refs
}

fn format_crossrefs(refs: &[(String, String)]) -> String {
    let mut output = String::from("**Cross-references:** ");
    let links: Vec<String> = refs
        .iter()
        .map(|(field, ty)| format!("{}: :ref:`{}`", field, ty))
        .collect();
    output.push_str(&links.join(", "));
    output.push('\n');
    output
}
