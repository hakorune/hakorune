//! Exact source-spelling authority for the first trivial parameter ABI row.
//!
//! This module classifies source declarations only. It does not admit a
//! function, allocate parameter values, or validate runtime arguments.

use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactTrivialParameterAbiV1 {
    I64,
}

impl ExactTrivialParameterAbiV1 {
    pub(crate) const fn classify(source_type_name: &str) -> Option<Self> {
        match source_type_name.as_bytes() {
            b"i64" => Some(Self::I64),
            _ => None,
        }
    }

    pub(crate) const fn mir_type(self) -> MirType {
        match self {
            Self::I64 => MirType::Integer,
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
    }
}
