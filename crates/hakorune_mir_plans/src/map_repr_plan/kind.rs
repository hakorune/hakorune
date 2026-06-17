#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapReprKind {
    GenericHashRuntime,
    LocalI64KeyMapShadow,
    FixedStatic,
    FixedSmallLinear,
    FixedOpenAddress,
    EnumKeyDense,
    InternedKeyHash,
    InternedKeyFixed,
}

impl MapReprKind {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::GenericHashRuntime => "generic_hash_runtime",
            Self::LocalI64KeyMapShadow => "local_i64_key_map_shadow",
            Self::FixedStatic => "fixed_static",
            Self::FixedSmallLinear => "fixed_small_linear",
            Self::FixedOpenAddress => "fixed_open_address",
            Self::EnumKeyDense => "enum_key_dense",
            Self::InternedKeyHash => "interned_key_hash",
            Self::InternedKeyFixed => "interned_key_fixed",
        }
    }
}

impl std::fmt::Display for MapReprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}
