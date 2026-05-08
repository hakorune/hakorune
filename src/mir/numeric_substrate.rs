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
pub struct NumericTypeName {
    pub signedness: NumericSignedness,
    pub width: NumericWidth,
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

pub(crate) fn is_numeric_integer_type_name(name: &str) -> bool {
    classify_numeric_type_name(name).is_some()
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
