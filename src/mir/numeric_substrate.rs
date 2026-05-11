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

/// Exact MIR-side numeric type metadata.
///
/// This is deliberately distinct from `MirType::Integer`: it records the
/// resolved signedness/width and the source spelling, but it does not change
/// runtime values or existing lowerers by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) struct ExactNumericMirType {
    pub source_name: String,
    pub kind: NumericKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) struct ExactNumericMirSignature {
    pub params: Vec<Option<ExactNumericMirType>>,
    pub return_type: Option<ExactNumericMirType>,
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

#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) fn exact_numeric_mir_type_from_declared_name(
    declared_type_name: Option<&str>,
    target: NumericTarget,
) -> Option<ExactNumericMirType> {
    let source_name = declared_type_name?.to_string();
    let kind = classify_numeric_kind_for_target(&source_name, target)?;
    Some(ExactNumericMirType { source_name, kind })
}

#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) fn exact_numeric_mir_signature_from_declared_names<'a>(
    param_type_names: impl IntoIterator<Item = Option<&'a str>>,
    return_type_name: Option<&'a str>,
    target: NumericTarget,
) -> ExactNumericMirSignature {
    ExactNumericMirSignature {
        params: param_type_names
            .into_iter()
            .map(|name| exact_numeric_mir_type_from_declared_name(name, target))
            .collect(),
        return_type: exact_numeric_mir_type_from_declared_name(return_type_name, target),
    }
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

    #[test]
    fn exact_numeric_mir_type_preserves_source_name_and_resolved_kind() {
        let pointer64 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits64,
        };

        assert_eq!(
            exact_numeric_mir_type_from_declared_name(Some("usize"), pointer64),
            Some(ExactNumericMirType {
                source_name: "usize".to_string(),
                kind: NumericKind {
                    signedness: NumericSignedness::Unsigned,
                    width: NumericResolvedWidth::Bits64,
                },
            })
        );
        assert_eq!(
            exact_numeric_mir_type_from_declared_name(Some("IntegerBox"), pointer64),
            None
        );
        assert_eq!(
            exact_numeric_mir_type_from_declared_name(None, pointer64),
            None
        );
    }

    #[test]
    fn exact_numeric_mir_signature_keeps_non_numeric_slots_empty() {
        let pointer32 = NumericTarget {
            pointer_width: NumericResolvedWidth::Bits32,
        };
        let signature = exact_numeric_mir_signature_from_declared_names(
            [Some("usize"), Some("StringBox"), Some("i64"), None],
            Some("isize"),
            pointer32,
        );

        assert_eq!(signature.params.len(), 4);
        assert_eq!(
            signature.params[0],
            Some(ExactNumericMirType {
                source_name: "usize".to_string(),
                kind: NumericKind {
                    signedness: NumericSignedness::Unsigned,
                    width: NumericResolvedWidth::Bits32,
                },
            })
        );
        assert_eq!(signature.params[1], None);
        assert_eq!(
            signature.params[2],
            Some(ExactNumericMirType {
                source_name: "i64".to_string(),
                kind: NumericKind {
                    signedness: NumericSignedness::Signed,
                    width: NumericResolvedWidth::Bits64,
                },
            })
        );
        assert_eq!(signature.params[3], None);
        assert_eq!(
            signature.return_type,
            Some(ExactNumericMirType {
                source_name: "isize".to_string(),
                kind: NumericKind {
                    signedness: NumericSignedness::Signed,
                    width: NumericResolvedWidth::Bits32,
                },
            })
        );
    }
}
