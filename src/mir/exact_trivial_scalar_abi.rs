//! Site-neutral exact scalar ABI for the first trivial callable rows.
//!
//! This module owns only the exact source spelling to physical MIR scalar
//! projection. Parameter and return sites wrap this substrate with their own
//! admission witnesses; this module never admits a function by itself.

use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactTrivialScalarAbiV1 {
    I64,
    /// Result projection only (R11): `classify` never yields it, so a `Bool` parameter, typed
    /// literal or field spelling is not admitted by this type.
    Bool,
}

impl ExactTrivialScalarAbiV1 {
    pub(crate) const fn classify(source_type_name: &str) -> Option<Self> {
        match source_type_name.as_bytes() {
            b"i64" => Some(Self::I64),
            _ => None,
        }
    }

    /// The spellings a declared function result may project to: `i64`, and `Bool` (R1, R11).
    pub(crate) const fn classify_result(source_type_name: &str) -> Option<Self> {
        match source_type_name.as_bytes() {
            b"Bool" => Some(Self::Bool),
            _ => Self::classify(source_type_name),
        }
    }

    pub(crate) const fn mir_type(self) -> MirType {
        match self {
            Self::I64 => MirType::Integer,
            Self::Bool => MirType::Bool,
        }
    }

    pub(crate) const fn source_type_name(self) -> &'static str {
        match self {
            Self::I64 => "i64",
            Self::Bool => "Bool",
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
    fn bool_is_a_result_projection_only() {
        assert_eq!(ExactTrivialScalarAbiV1::classify("Bool"), None);
        assert_eq!(ExactTrivialScalarAbiV1::classify_result("Bool"), Some(ExactTrivialScalarAbiV1::Bool));
        assert_eq!(ExactTrivialScalarAbiV1::classify_result("i64"), Some(ExactTrivialScalarAbiV1::I64));
        for rejected in ["bool", "BoolBox", "boolean", " Bool", "Bool "] {
            assert_eq!(ExactTrivialScalarAbiV1::classify_result(rejected), None);
        }
        assert_eq!(ExactTrivialScalarAbiV1::Bool.mir_type(), MirType::Bool);
    }

    #[test]
    fn exact_i64_projects_to_existing_integer_representation() {
        assert_eq!(ExactTrivialScalarAbiV1::I64.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialScalarAbiV1::I64.source_type_name(), "i64");
    }
}
