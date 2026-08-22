//! Exact source spelling for the semantic Text parameter contract.
//!
//! This is a formal source demand, not a runtime handle or physical wire.
//! The callable-parameter issuer is the only production-shaped classifier.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExactTextFormalAbiV1 {
    _private: (),
}

impl ExactTextFormalAbiV1 {
    pub(crate) const STRING_BOX: Self = Self { _private: () };

    pub(crate) fn classify(source_type_name: &str) -> Option<Self> {
        if source_type_name == "StringBox" {
            Some(Self::STRING_BOX)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExactTextFormalAbiV1;

    #[test]
    fn accepts_only_the_admitted_stringbox_spelling() {
        assert_eq!(
            ExactTextFormalAbiV1::classify("StringBox"),
            Some(ExactTextFormalAbiV1::STRING_BOX)
        );
        for rejected in ["String", "stringbox", "StringBox ", " StringBox"] {
            assert_eq!(ExactTextFormalAbiV1::classify(rejected), None);
        }
    }
}
