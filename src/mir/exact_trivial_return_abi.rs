//! Exact source-spelling authority for the first trivial return ABI row.
//!
//! This module classifies source declarations only. It does not admit a
//! function, select a terminal value, or install runtime metadata.

use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExactTrivialReturnAbiV1 {
    scalar: ExactTrivialScalarAbiV1,
}

impl ExactTrivialReturnAbiV1 {
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
}

#[cfg(test)]
mod tests {
    use super::ExactTrivialReturnAbiV1;
    use crate::mir::MirType;

    #[test]
    fn accepts_only_exact_i64_source_spelling() {
        assert_eq!(
            ExactTrivialReturnAbiV1::classify("i64"),
            Some(ExactTrivialReturnAbiV1::I64)
        );
        for rejected in ["int", "Integer", "IntegerBox", "I64", " i64", "i64 "] {
            assert_eq!(ExactTrivialReturnAbiV1::classify(rejected), None);
        }
    }

    #[test]
    fn exact_i64_projects_to_existing_integer_representation() {
        assert_eq!(ExactTrivialReturnAbiV1::I64.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialReturnAbiV1::I64.source_type_name(), "i64");
    }
}
