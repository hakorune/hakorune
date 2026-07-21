//! HEADERPORT0-I0-MAINPENDING0-S0: root-main completion handoff.
//!
//! The handoff keeps the final header loan short-lived.  A pending root draft
//! owns only the completed function, root-body witness, and source tag; it
//! never stores a header view, Builder, collector, or fallback capability.

use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::root_body_completion::CompletedRootBodyV1;
use crate::mir::{FunctionSignature, MirFunction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum MainHeaderSourceV1 {
    InvocationCollector,
    ModuleCompatibility,
}

/// A short-lived, read-only header loan.  It cannot be stored in a pending
/// draft because the returned draft owns only the source tag.
pub(in crate::mir::builder) struct MainHeaderLoanV1<'headers> {
    headers: &'headers dyn FunctionSignatureLookupV1,
    source: MainHeaderSourceV1,
    _seal: MainHeaderLoanSealV1,
}

#[derive(Debug)]
struct MainHeaderLoanSealV1;

impl<'headers> MainHeaderLoanV1<'headers> {
    pub(in crate::mir::builder) fn new(
        headers: &'headers dyn FunctionSignatureLookupV1,
        source: MainHeaderSourceV1,
    ) -> Self {
        Self {
            headers,
            source,
            _seal: MainHeaderLoanSealV1,
        }
    }

    pub(in crate::mir::builder) fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
        self.headers.signature(symbol)
    }

    pub(in crate::mir::builder) fn source(&self) -> MainHeaderSourceV1 {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct MainDraftIdentityV1 {
    symbol: Box<str>,
    arity: usize,
}

impl MainDraftIdentityV1 {
    pub(in crate::mir::builder) fn root() -> Self {
        Self {
            symbol: "main".into(),
            arity: 0,
        }
    }

    pub(in crate::mir::builder) fn new(symbol: impl Into<Box<str>>, arity: usize) -> Self {
        Self {
            symbol: symbol.into(),
            arity,
        }
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum MainPendingDraftErrorV1 {
    SymbolMismatch { expected: String, actual: String },
    ArityMismatch { expected: usize, actual: usize },
}

impl std::fmt::Display for MainPendingDraftErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][main_pending] {self:?}")
    }
}

impl std::error::Error for MainPendingDraftErrorV1 {}

/// Inputs owned until root function completion begins.  The header loan is
/// supplied only to the consuming `finish` operation and is never retained.
#[derive(Debug)]
pub(in crate::mir::builder) struct MainCompletionRequestV1 {
    identity: MainDraftIdentityV1,
    root_body: CompletedRootBodyV1,
    returns_value: bool,
    _seal: MainCompletionRequestSealV1,
}

#[derive(Debug)]
struct MainCompletionRequestSealV1;

/// One unpublished root draft after the final header loan has ended.
#[derive(Debug)]
pub(in crate::mir::builder) struct PendingMainDraftV1 {
    draft: MirFunction,
    root_body: CompletedRootBodyV1,
    identity: MainDraftIdentityV1,
    returns_value: bool,
    header_source: MainHeaderSourceV1,
    _seal: PendingMainDraftSealV1,
}

#[derive(Debug)]
struct PendingMainDraftSealV1;

impl MainCompletionRequestV1 {
    pub(in crate::mir::builder) fn new(
        identity: MainDraftIdentityV1,
        root_body: CompletedRootBodyV1,
        returns_value: bool,
    ) -> Self {
        Self {
            identity,
            root_body,
            returns_value,
            _seal: MainCompletionRequestSealV1,
        }
    }

    pub(in crate::mir::builder) fn finish(
        self,
        draft: MirFunction,
        headers: MainHeaderLoanV1<'_>,
    ) -> Result<PendingMainDraftV1, MainPendingDraftErrorV1> {
        let actual_symbol = draft.signature.name.clone();
        if actual_symbol != self.identity.symbol() {
            return Err(MainPendingDraftErrorV1::SymbolMismatch {
                expected: self.identity.symbol().to_owned(),
                actual: actual_symbol,
            });
        }
        let actual_arity = draft.signature.params.len();
        if actual_arity != self.identity.arity() {
            return Err(MainPendingDraftErrorV1::ArityMismatch {
                expected: self.identity.arity(),
                actual: actual_arity,
            });
        }

        Ok(PendingMainDraftV1 {
            draft,
            root_body: self.root_body,
            identity: self.identity,
            returns_value: self.returns_value,
            header_source: headers.source(),
            _seal: PendingMainDraftSealV1,
        })
    }
}

impl PendingMainDraftV1 {
    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(in crate::mir::builder) fn root_body(&self) -> &CompletedRootBodyV1 {
        &self.root_body
    }

    pub(in crate::mir::builder) fn identity(&self) -> &MainDraftIdentityV1 {
        &self.identity
    }

    pub(in crate::mir::builder) fn returns_value(&self) -> bool {
        self.returns_value
    }

    pub(in crate::mir::builder) fn header_source(&self) -> MainHeaderSourceV1 {
        self.header_source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::{BasicBlockId, EffectMask, MirType};

    fn root_body() -> CompletedRootBodyV1 {
        RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::NoValue)
            .unwrap()
    }

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn finish_consumes_short_header_loan_without_storing_it() {
        let module = crate::mir::MirModule::new("headers".into());
        let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root_body(), false);
        let loan = MainHeaderLoanV1::new(&module, MainHeaderSourceV1::InvocationCollector);
        assert_eq!(loan.signature("missing"), None);
        let pending = request.finish(draft("main", 0), loan).unwrap();
        assert_eq!(pending.identity().symbol(), "main");
        assert_eq!(
            pending.header_source(),
            MainHeaderSourceV1::InvocationCollector
        );
        assert!(!pending.returns_value());
    }

    #[test]
    fn foreign_symbol_or_arity_is_rejected_before_pending_product() {
        let module = crate::mir::MirModule::new("headers".into());
        let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root_body(), false);
        let loan = MainHeaderLoanV1::new(&module, MainHeaderSourceV1::ModuleCompatibility);
        assert_eq!(
            request.finish(draft("other", 0), loan).unwrap_err(),
            MainPendingDraftErrorV1::SymbolMismatch {
                expected: "main".to_owned(),
                actual: "other".to_owned(),
            }
        );

        let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root_body(), false);
        let loan = MainHeaderLoanV1::new(&module, MainHeaderSourceV1::ModuleCompatibility);
        assert_eq!(
            request.finish(draft("main", 1), loan).unwrap_err(),
            MainPendingDraftErrorV1::ArityMismatch {
                expected: 0,
                actual: 1,
            }
        );
    }
}
