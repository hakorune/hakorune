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

    /// The scalar spellings a declared function result may project to (R1, R11). The one table
    /// of declared result spellings is `DeclaredResultKindV1::classify`.
    pub(crate) const fn classify_result(source_type_name: &str) -> Option<Self> {
        match DeclaredResultKindV1::classify(source_type_name) {
            Some(kind) => kind.scalar_abi(),
            None => None,
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

/// The one table of declared function-result spellings (R1, S1): a scalar projection, or an
/// owned `String` that has no scalar projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclaredResultKindV1 {
    Scalar(ExactTrivialScalarAbiV1),
    OwnedText,
    /// A box of the source that name resolution found (O1); it has no scalar projection.
    OwnedObject,
}

impl DeclaredResultKindV1 {
    pub(crate) const fn classify(source_type_name: &str) -> Option<Self> {
        match source_type_name.as_bytes() {
            b"Bool" => Some(Self::Scalar(ExactTrivialScalarAbiV1::Bool)),
            b"String" => Some(Self::OwnedText),
            _ => match ExactTrivialScalarAbiV1::classify(source_type_name) {
                Some(abi) => Some(Self::Scalar(abi)),
                None => None,
            },
        }
    }

    /// A known non-scalar result has no scalar ABI; it is not an unknown spelling.
    pub(crate) const fn scalar_abi(self) -> Option<ExactTrivialScalarAbiV1> {
        match self {
            Self::Scalar(abi) => Some(abi),
            Self::OwnedText | Self::OwnedObject => None,
        }
    }

    /// Canonical type syntax, without granting declaration coverage or a runtime ABI.
    pub(crate) fn nominal_reference(spelling: &str) -> Option<(String, bool)> {
        use crate::ast::TypeReferenceV1 as Ref;
        let (reference, nullable) = match Ref::parse(spelling).ok()? {
            Ref::Nullable(inner) => (*inner, true), reference => (reference, false),
        };
        match reference { Ref::Name(name) if Self::classify(&name).is_none() => Some((name, nullable)), _ => None }
    }

    /// O1: the table above, then a declared box the source resolved (never a spelling).
    pub(crate) fn classify_resolved(source_type_name: &str, declared_box: Option<&str>) -> Option<Self> {
        Self::classify(source_type_name).or_else(|| {
            Self::nominal_reference(source_type_name)
                .filter(|(name, _)| declared_box == Some(name.as_str()))
                .map(|_| Self::OwnedObject)
        })
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
    fn declared_result_table_reads_scalar_and_owned_text_spellings_only() {
        use super::DeclaredResultKindV1 as Kind;
        assert_eq!(Kind::classify("String"), Some(Kind::OwnedText));
        assert_eq!(Kind::classify("Bool"), Some(Kind::Scalar(ExactTrivialScalarAbiV1::Bool)));
        assert_eq!(Kind::classify("i64"), Some(Kind::Scalar(ExactTrivialScalarAbiV1::I64)));
        // Owned text has no scalar projection, and no other spelling is a text result.
        assert_eq!(ExactTrivialScalarAbiV1::classify_result("String"), None);
        for rejected in ["StringBox", "string", "Text", "String?", " String"] {
            assert_eq!(Kind::classify(rejected), None);
        }
    }

    #[test]
    fn a_box_name_is_an_object_result_only_when_name_resolution_found_that_box() {
        use super::DeclaredResultKindV1 as Kind;
        assert_eq!(Kind::classify_resolved("Leaf", Some("Leaf")), Some(Kind::OwnedObject));
        assert_eq!(Kind::classify_resolved("Leaf", None), None);
        assert_eq!(Kind::classify_resolved("Leaf", Some("Other")), None);
        // The table still decides the builtin spellings first.
        assert_eq!(Kind::classify_resolved("String", Some("String")), Some(Kind::OwnedText));
    }

    #[test]
    fn exact_i64_projects_to_existing_integer_representation() {
        assert_eq!(ExactTrivialScalarAbiV1::I64.mir_type(), MirType::Integer);
        assert_eq!(ExactTrivialScalarAbiV1::I64.source_type_name(), "i64");
    }
}
