/// A parsed type: either a struct or an enum annotated with `#[deku_display(...)]`.
pub enum TypeInfo {
    Struct(StructInfo),
    Enum(EnumInfo),
}

impl TypeInfo {
    /// The type name.
    pub fn name(&self) -> &str {
        match self {
            TypeInfo::Struct(s) => &s.name,
            TypeInfo::Enum(e) => &e.name,
        }
    }
}

/// Information about a struct annotated with `#[deku_display(...)]`.
pub struct StructInfo {
    pub name: String,
    pub bit_width: u8,
    pub bitheader: Option<String>,
    pub fields: Vec<FieldInfo>,
}

/// Information about an enum annotated with `#[deku_display(...)]`.
pub struct EnumInfo {
    pub name: String,
    pub bit_width: u8,
    pub bitheader: Option<String>,
    /// Bit width of the discriminant (from `id_type` or `bits` on the enum).
    pub id_bits: Option<u32>,
    /// Whether the ID is read from the data stream (`id_type` or `type` present).
    /// If false and `id_bits` is set, the ID comes from explicit `bits`/`bytes` attrs.
    /// If `id_bits` is None, the ID comes from context.
    pub id_read: bool,
    pub variants: Vec<VariantInfo>,
}

/// Information about a single variant in a deku_display enum.
pub struct VariantInfo {
    pub name: String,
    /// The `#[deku(id = ...)]` value, if present.
    pub id_value: Option<String>,
    /// The `#[deku(id_pat = ...)]` pattern, if present.
    pub id_pat: Option<String>,
    pub fields: Vec<FieldInfo>,
}

/// Information about a single field in a deku_display struct.
pub struct FieldInfo {
    pub name: String,
    pub bits: Option<u32>,
    pub type_name: String,
    pub unused: bool,
}

impl FieldInfo {
    /// Resolve the bit count for this field: explicit `bits` attribute, or inferred from type name.
    pub fn resolved_bits(&self) -> anyhow::Result<u32> {
        if let Some(bits) = self.bits {
            return Ok(bits);
        }
        match self.type_name.as_str() {
            "u8" | "i8" | "bool" => Ok(8),
            "u16" | "i16" => Ok(16),
            "u32" | "i32" | "f32" => Ok(32),
            "u64" | "i64" | "f64" => Ok(64),
            "u128" | "i128" => Ok(128),
            _ => anyhow::bail!(
                "field '{}' has type '{}' with no explicit #[deku(bits = \"N\")] — \
                 cannot infer bit width for non-primitive types",
                self.name,
                self.type_name
            ),
        }
    }
}
