/*!
 * Numeric substrate vocabulary for allocator-grade MIR metadata.
 *
 * This module owns the fixed-width and pointer-sized integer type names used by
 * substrate rows. It is intentionally metadata-only today: values still execute
 * on the current dynamic Integer/i64 lane until a later row adds exact
 * width/range/overflow semantics.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericSignedness {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
    Pointer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericResolvedWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTarget {
    pointer_width: NumericResolvedWidth,
}

impl NumericTarget {
    #[cfg(target_pointer_width = "32")]
    pub(crate) const HOST: Self = Self {
        pointer_width: NumericResolvedWidth::Bits32,
    };

    #[cfg(target_pointer_width = "64")]
    pub(crate) const HOST: Self = Self {
        pointer_width: NumericResolvedWidth::Bits64,
    };

    pub(crate) const fn host() -> Self {
        Self::HOST
    }

    pub(crate) const fn pointer_width(self) -> NumericResolvedWidth {
        self.pointer_width
    }
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Hakorune numeric substrate requires an explicit 32-bit or 64-bit pointer target");

impl NumericWidth {
    pub(crate) const fn resolve_for_target(self, target: NumericTarget) -> NumericResolvedWidth {
        match self {
            Self::Bits8 => NumericResolvedWidth::Bits8,
            Self::Bits16 => NumericResolvedWidth::Bits16,
            Self::Bits32 => NumericResolvedWidth::Bits32,
            Self::Bits64 => NumericResolvedWidth::Bits64,
            Self::Pointer => target.pointer_width(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTypeName {
    pub signedness: NumericSignedness,
    pub width: NumericWidth,
}

impl NumericTypeName {
    pub(crate) const fn kind_for_target(self, target: NumericTarget) -> NumericKind {
        NumericKind {
            signedness: self.signedness,
            width: self.width.resolve_for_target(target),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericKind {
    pub signedness: NumericSignedness,
    pub width: NumericResolvedWidth,
}

pub(crate) fn classify_numeric_type_name(name: &str) -> Option<NumericTypeName> {
    let (signedness, width) = match name {
        "i8" => (NumericSignedness::Signed, NumericWidth::Bits8),
        "i16" => (NumericSignedness::Signed, NumericWidth::Bits16),
        "i32" => (NumericSignedness::Signed, NumericWidth::Bits32),
        "i64" => (NumericSignedness::Signed, NumericWidth::Bits64),
        "isize" => (NumericSignedness::Signed, NumericWidth::Pointer),
        "u8" => (NumericSignedness::Unsigned, NumericWidth::Bits8),
        "u16" => (NumericSignedness::Unsigned, NumericWidth::Bits16),
        "u32" => (NumericSignedness::Unsigned, NumericWidth::Bits32),
        "u64" => (NumericSignedness::Unsigned, NumericWidth::Bits64),
        "usize" => (NumericSignedness::Unsigned, NumericWidth::Pointer),
        _ => return None,
    };
    Some(NumericTypeName { signedness, width })
}

pub(crate) fn classify_numeric_kind_for_target(
    name: &str,
    target: NumericTarget,
) -> Option<NumericKind> {
    classify_numeric_type_name(name).map(|type_name| type_name.kind_for_target(target))
}

pub(crate) fn is_numeric_integer_type_name(name: &str) -> bool {
    classify_numeric_kind_for_target(name, NumericTarget::host()).is_some()
}

pub(crate) fn is_inline_i64_storage_type_name(name: &str) -> bool {
    is_numeric_integer_type_name(name)
        || matches!(
            name,
            "IntegerBox" | "Integer" | "BoolBox" | "Bool" | "bool" | "i1"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_fixed_width_and_pointer_sized_names() {
        assert_eq!(
            classify_numeric_type_name("i8"),
            Some(NumericTypeName {
                signedness: NumericSignedness::Signed,
                width: NumericWidth::Bits8,
            })
        );
        assert_eq!(
            classify_numeric_type_name("u64"),
            Some(NumericTypeName {
                signedness: NumericSignedness::Unsigned,
                width: NumericWidth::Bits64,
            })
        );
        assert_eq!(
            classify_numeric_type_name("usize"),
            Some(NumericTypeName {
                signedness: NumericSignedness::Unsigned,
                width: NumericWidth::Pointer,
            })
        );
        assert_eq!(classify_numeric_type_name("IntegerBox"), None);
        assert_eq!(classify_numeric_type_name("String"), None);
    }

    #[test]
    fn host_target_width_matches_rust_compilation_target() {
        #[cfg(target_pointer_width = "32")]
        assert_eq!(
            NumericTarget::host().pointer_width(),
            NumericResolvedWidth::Bits32
        );
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            NumericTarget::host().pointer_width(),
            NumericResolvedWidth::Bits64
        );
    }

    #[test]
    fn resolves_pointer_sized_kinds_through_target_width() {
        let pointer64 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits64,
        };
        let pointer32 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits32,
        };
        assert_eq!(
            classify_numeric_kind_for_target("usize", pointer64),
            Some(NumericKind {
                signedness: NumericSignedness::Unsigned,
                width: NumericResolvedWidth::Bits64,
            })
        );
        assert_eq!(
            classify_numeric_kind_for_target("isize", pointer32),
            Some(NumericKind {
                signedness: NumericSignedness::Signed,
                width: NumericResolvedWidth::Bits32,
            })
        );
    }

    #[test]
    fn fixed_width_kinds_do_not_depend_on_target_width() {
        let pointer32 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits32,
        };
        let pointer64 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits64,
        };
        assert_eq!(
            classify_numeric_kind_for_target("u32", pointer32),
            classify_numeric_kind_for_target("u32", pointer64)
        );
    }

    #[test]
    fn inline_i64_storage_keeps_legacy_scalar_aliases() {
        for name in ["IntegerBox", "Integer", "BoolBox", "Bool", "bool", "i1"] {
            assert!(is_inline_i64_storage_type_name(name));
        }
        for name in ["i16", "u32", "usize", "isize"] {
            assert!(is_inline_i64_storage_type_name(name));
        }
        for name in ["FloatBox", "StringBox", "String", "Ptr"] {
            assert!(!is_inline_i64_storage_type_name(name));
        }
    }
}
