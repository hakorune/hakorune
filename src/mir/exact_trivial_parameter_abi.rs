//! Exact source-spelling authority for trivial parameter ABI rows.
//!
//! This module classifies source declarations only. It does not admit a
//! function, allocate parameter values, or validate runtime arguments.
//!
//! Scope note: `usize` is admitted here as a parameter-only exact spelling
//! (it executes on the `i64`/`MirType::Integer` lane per
//! usize-semantic-foundation-ssot). The shared `ExactTrivialScalarAbiV1`
//! stays `i64`-only so `: usize` return annotations and usize literals keep
//! their existing rejections.

use crate::mir::function::MirParamDecl;
use crate::mir::MirType;

/// Parameter-scoped scalar spelling. Deliberately separate from
/// `ExactTrivialScalarAbiV1` so the return ABI stays i64-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactTrivialParameterScalarV1 {
    I64,
    Usize,
}

impl ExactTrivialParameterScalarV1 {
    const fn mir_type(self) -> MirType {
        MirType::Integer
    }

    const fn source_type_name(self) -> &'static str {
        match self {
            Self::I64 => "i64",
            Self::Usize => "usize",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExactTrivialParameterAbiV1 {
    scalar: ExactTrivialParameterScalarV1,
}

impl ExactTrivialParameterAbiV1 {
    pub(crate) const I64: Self = Self {
        scalar: ExactTrivialParameterScalarV1::I64,
    };

    pub(crate) const USIZE: Self = Self {
        scalar: ExactTrivialParameterScalarV1::Usize,
    };

    pub(crate) const fn classify(source_type_name: &str) -> Option<Self> {
        match source_type_name.as_bytes() {
            b"i64" => Some(Self::I64),
            b"usize" => Some(Self::USIZE),
            _ => None,
        }
    }

    /// True when this is the exact `i64` spelling. Consumption sites that
    /// require an i64-typed parameter must test this, not `is_some()`.
    pub(crate) const fn is_i64(self) -> bool {
        matches!(self.scalar, ExactTrivialParameterScalarV1::I64)
    }

    pub(crate) const fn mir_type(self) -> MirType {
        self.scalar.mir_type()
    }

    pub(crate) const fn source_type_name(self) -> &'static str {
        self.scalar.source_type_name()
    }

    pub(crate) fn mir_param_decl(self, source_name: &str) -> MirParamDecl {
        MirParamDecl {
            name: source_name.to_string(),
            declared_type_name: Some(self.source_type_name().to_string()),
            implicit_receiver: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExactTrivialParameterAbiV1;
    use crate::mir::MirType;

    #[test]
    fn accepts_exact_i64_and_usize_source_spellings() {
        assert_eq!(
            ExactTrivialParameterAbiV1::classify("i64"),
            Some(ExactTrivialParameterAbiV1::I64)
        );
        assert_eq!(
            ExactTrivialParameterAbiV1::classify("usize"),
            Some(ExactTrivialParameterAbiV1::USIZE)
        );
        for rejected in ["int", "Integer", "IntegerBox", "I64", " i64", "u64"] {
            assert_eq!(ExactTrivialParameterAbiV1::classify(rejected), None);
        }
    }

    #[test]
    fn usize_projects_to_the_integer_lane_without_i64_alias() {
        assert_eq!(ExactTrivialParameterAbiV1::USIZE.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialParameterAbiV1::USIZE.source_type_name(), "usize");
        assert!(!ExactTrivialParameterAbiV1::USIZE.is_i64());
        assert!(ExactTrivialParameterAbiV1::I64.is_i64());
        assert_eq!(
            ExactTrivialParameterAbiV1::USIZE.mir_param_decl("value"),
            crate::mir::function::MirParamDecl {
                name: "value".to_string(),
                declared_type_name: Some("usize".to_string()),
                implicit_receiver: false,
            }
        );
    }
}
