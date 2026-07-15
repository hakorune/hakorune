//! Site-neutral exact scalar ABI for the first trivial callable rows.
//!
//! This module owns only the exact source spelling to physical MIR scalar
//! projection. Parameter and return sites wrap this substrate with their own
//! admission witnesses; this module never admits a function by itself.

use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactTrivialScalarAbiV1 {
    I64,
}

impl ExactTrivialScalarAbiV1 {
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

    pub(crate) const fn source_type_name(self) -> &'static str {
        match self {
            Self::I64 => "i64",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExactTrivialScalarAbiV1;
    use crate::mir::MirType;

    #[test]
    fn accepts_only_exact_i64_source_spelling() {
        assert_eq!(
            ExactTrivialScalarAbiV1::classify("i64"),
            Some(ExactTrivialScalarAbiV1::I64)
        );
        for rejected in ["int", "Integer", "IntegerBox", "I64", " i64", "i64 "] {
            assert_eq!(ExactTrivialScalarAbiV1::classify(rejected), None);
        }
    }

    #[test]
    fn exact_i64_projects_to_existing_integer_representation() {
        assert_eq!(ExactTrivialScalarAbiV1::I64.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialScalarAbiV1::I64.source_type_name(), "i64");
    }
}
