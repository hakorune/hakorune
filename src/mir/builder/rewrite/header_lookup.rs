//! HEADERPORT0-ACCESS0-P0: lookup-only rewrite projection.
//!
//! This surface observes completed function headers without performing MIR
//! mutation.  The legacy rewrite implementation remains the production
//! owner until the invocation terminal cutover supplies this view explicitly.

use super::super::builder_method_index::method_candidates_from_headers;
use super::super::function_signature_lookup::FunctionSignatureLookupV1;

/// Read-only header projection needed by Known/unique rewrite policy.
///
/// It intentionally owns neither a `MirBuilder`, a module map, a receiver
/// fact, nor a rewrite decision.  Callers borrow it for one classification
/// observation and must perform emission through their own terminal.
pub(in crate::mir::builder) struct KnownRewriteHeaderViewV1<'headers> {
    headers: &'headers dyn FunctionSignatureLookupV1,
}

impl<'headers> KnownRewriteHeaderViewV1<'headers> {
    pub(in crate::mir::builder) fn new(headers: &'headers dyn FunctionSignatureLookupV1) -> Self {
        Self { headers }
    }

    pub(in crate::mir::builder) fn contains_symbol(&self, symbol: &str) -> bool {
        self.headers.contains_symbol(symbol)
    }

    pub(in crate::mir::builder) fn parameter_count(&self, symbol: &str) -> Option<usize> {
        self.headers
            .signature(symbol)
            .map(|signature| signature.params.len())
    }

    pub(in crate::mir::builder) fn prepend_receiver(
        &self,
        symbol: &str,
        argument_count: usize,
    ) -> bool {
        !matches!(self.parameter_count(symbol), Some(count) if count == argument_count)
    }

    pub(in crate::mir::builder) fn unique_suffix_candidates(
        &self,
        method: &str,
        argument_count: usize,
    ) -> Vec<String> {
        method_candidates_from_headers(self.headers, method, argument_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{EffectMask, FunctionSignature, MirType};
    struct FakeHeaders {
        signatures: Vec<(String, FunctionSignature)>,
    }

    impl FunctionSignatureLookupV1 for FakeHeaders {
        fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
            self.signatures
                .iter()
                .find(|(name, _)| name == symbol)
                .map(|(_, signature)| signature)
        }

        fn contains_symbol(&self, symbol: &str) -> bool {
            self.signatures.iter().any(|(name, _)| name == symbol)
        }

        fn symbol_count(&self) -> usize {
            self.signatures.len()
        }

        fn visit_symbols(&self, visitor: &mut dyn FnMut(&str)) {
            for (symbol, _) in &self.signatures {
                visitor(symbol);
            }
        }
    }

    fn signature(name: &str, params: usize) -> FunctionSignature {
        FunctionSignature {
            name: name.to_owned(),
            params: vec![MirType::Integer; params],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn header_view_preserves_signature_arity_policy() {
        let signatures = vec![("User.f/1".to_owned(), signature("User.f/1", 1))];
        let headers = FakeHeaders { signatures };
        let view = KnownRewriteHeaderViewV1::new(&headers);

        assert!(view.contains_symbol("User.f/1"));
        assert!(!view.contains_symbol("User.missing/0"));
        assert_eq!(view.parameter_count("User.f/1"), Some(1));
        assert_eq!(view.parameter_count("User.missing/0"), None);
        assert!(!view.prepend_receiver("User.f/1", 1));
        assert!(view.prepend_receiver("User.f/1", 0));
    }

    #[test]
    fn header_view_uses_shared_unique_suffix_policy() {
        let signatures = vec![
            ("Other.g/1".to_owned(), signature("Other.g/1", 1)),
            ("Other.f/1".to_owned(), signature("Other.f/1", 1)),
            ("User.f/1".to_owned(), signature("User.f/1", 1)),
        ];
        let headers = FakeHeaders { signatures };
        let view = KnownRewriteHeaderViewV1::new(&headers);

        assert!(view.unique_suffix_candidates("missing", 0).is_empty());
        assert_eq!(view.unique_suffix_candidates("f", 1).len(), 2);
        assert_eq!(view.unique_suffix_candidates("g", 1), vec!["Other.g/1"]);
    }

    #[test]
    fn header_view_missing_symbol_has_no_compatibility_fallback() {
        let explicit = FakeHeaders {
            signatures: vec![("Other.f/1".to_owned(), signature("Other.f/1", 1))],
        };
        let stale = FakeHeaders {
            signatures: vec![("User.f/1".to_owned(), signature("User.f/1", 1))],
        };
        let view = KnownRewriteHeaderViewV1::new(&explicit);
        let stale_view = KnownRewriteHeaderViewV1::new(&stale);

        assert!(!view.contains_symbol("User.f/1"));
        assert_eq!(view.parameter_count("User.f/1"), None);
        assert!(view.unique_suffix_candidates("User.f", 1).is_empty());
        assert!(stale_view.contains_symbol("User.f/1"));
    }

    #[test]
    fn header_view_known_arity_matrix_keeps_static_and_instance_shapes() {
        let headers = FakeHeaders {
            signatures: vec![
                ("User.static/1".to_owned(), signature("User.static/1", 1)),
                (
                    "User.instance/2".to_owned(),
                    signature("User.instance/2", 2),
                ),
            ],
        };
        let view = KnownRewriteHeaderViewV1::new(&headers);

        assert_eq!(view.parameter_count("User.static/1"), Some(1));
        assert!(!view.prepend_receiver("User.static/1", 1));
        assert!(view.prepend_receiver("User.static/1", 0));
        assert_eq!(view.parameter_count("User.instance/2"), Some(2));
        assert!(!view.prepend_receiver("User.instance/2", 2));
        assert!(view.prepend_receiver("User.instance/2", 1));
    }
}
