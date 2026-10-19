//! Typed field storage vocabulary shared by MIR plan metadata.
//!
//! This module is passive. It names backend-readable storage classes, but it
//! does not infer storage, mutate MIR, or enable backend lowering.

/// Backend-readable storage class for typed object and aggregate layout plans.
///
/// Exact numeric variants preserve source storage names. Current execution may
/// still use the dynamic integer lane until a backend claims native exact slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedObjectFieldStorage {
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    Handle,
    /// Source-declared untyped strong slot. Value tags and residence are
    /// independently checked; this is neither integer nor handle storage.
    Dynamic,
}

impl TypedObjectFieldStorage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::ISize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::USize => "usize",
            Self::Handle => "handle",
            Self::Dynamic => "dynamic",
        }
    }

    pub fn uses_integer_lane(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::ISize
            | Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::USize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_field_storage_names_match_metadata_surface() {
        assert_eq!(TypedObjectFieldStorage::I8.as_str(), "i8");
        assert_eq!(TypedObjectFieldStorage::I64.as_str(), "i64");
        assert_eq!(TypedObjectFieldStorage::USize.as_str(), "usize");
        assert_eq!(TypedObjectFieldStorage::Handle.as_str(), "handle");
        assert_eq!(TypedObjectFieldStorage::Dynamic.as_str(), "dynamic");
    }

    #[test]
    fn handle_is_not_integer_lane_storage() {
        assert!(TypedObjectFieldStorage::I64.uses_integer_lane());
        assert!(TypedObjectFieldStorage::USize.uses_integer_lane());
        assert!(!TypedObjectFieldStorage::Handle.uses_integer_lane());
        assert!(!TypedObjectFieldStorage::Dynamic.uses_integer_lane());
    }
}
