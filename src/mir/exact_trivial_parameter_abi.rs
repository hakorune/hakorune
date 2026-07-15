//! Exact source-spelling authority for the first trivial parameter ABI row.
//!
//! This module classifies source declarations only. It does not admit a
//! function, allocate parameter values, or validate runtime arguments.

use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::function::MirParamDecl;
use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExactTrivialParameterAbiV1 {
    scalar: ExactTrivialScalarAbiV1,
}

impl ExactTrivialParameterAbiV1 {
    pub(crate) const I64: Self = Self {
        scalar: ExactTrivialScalarAbiV1::I64,
    };

    pub(crate) const fn classify(source_type_name: &str) -> Option<Self> {
        match ExactTrivialScalarAbiV1::classify(source_type_name) {
            Some(scalar) => Some(Self { scalar }),
            _ => None,
        }
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
    fn accepts_only_exact_i64_source_spelling() {
        assert_eq!(
            ExactTrivialParameterAbiV1::classify("i64"),
            Some(ExactTrivialParameterAbiV1::I64)
        );
        for rejected in ["int", "Integer", "IntegerBox", "I64", " i64", "i64 "] {
            assert_eq!(ExactTrivialParameterAbiV1::classify(rejected), None);
        }
    }

    #[test]
    fn exact_i64_projects_to_existing_integer_representation() {
        assert_eq!(ExactTrivialParameterAbiV1::I64.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialParameterAbiV1::I64.source_type_name(), "i64");
        assert_eq!(
            ExactTrivialParameterAbiV1::I64.mir_param_decl("value"),
            crate::mir::function::MirParamDecl {
                name: "value".to_string(),
                declared_type_name: Some("i64".to_string()),
                implicit_receiver: false,
            }
        );
    }
}
