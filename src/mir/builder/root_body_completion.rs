//! HEADERPORT0-I0-BODYDRAIN0-S0: root-body completion witness.
//!
//! This product is intentionally disconnected from lowering and publication.
//! It records only that recursive body descent has closed its child scopes,
//! header loans, and pending terminals before a root result disposition is
//! sealed.  It owns no Builder, collector, function map, or fact store.

#[cfg(test)]
use std::sync::atomic::{AtomicU64, Ordering};

use super::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::ValueId;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootBodyCompletionErrorV1 {
    OpenChildScopes { count: usize },
    OpenHeaderLoans { count: usize },
    OpenPendingTerminals { count: usize },
    ForeignToken,
    ForeignBrand,
    TokenKindMismatch,
    NoOpenToken,
    AlreadyDriven,
}

impl std::fmt::Display for RootBodyCompletionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][root_body_completion] {self:?}"
        )
    }
}

impl std::error::Error for RootBodyCompletionErrorV1 {}

/// The root body may either produce one value or explicitly complete without
/// a value.  The disposition is source/lowering-local and is not a type fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootBodyResultV1 {
    Value(ValueId),
    NoValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootBodyActivityKindV1 {
    Child,
    HeaderLoan,
    PendingTerminal,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RootBodyActivityTokenV1 {
    brand: ModuleInvocationBrandV1,
    owner: u64,
    kind: RootBodyActivityKindV1,
    _seal: RootBodyActivityTokenSealV1,
}

#[derive(Debug)]
struct RootBodyActivityTokenSealV1;

/// A short-lived tracker used by the future root driver.  Every activity is
/// represented by a consuming token so the completion witness cannot be
/// produced while recursive child/header/pending work remains open.
#[derive(Debug)]
pub(in crate::mir::builder) struct RootBodyCompletionTrackerV1 {
    brand: ModuleInvocationBrandV1,
    owner: u64,
    open_children: usize,
    open_header_loans: usize,
    open_pending_terminals: usize,
    completed_children: usize,
    _seal: RootBodyCompletionTrackerSealV1,
}

/// BODY0-only typestate: a fresh tracker must be consumed before root-body
/// activity starts, and only this active wrapper may seal the final witness.
#[derive(Debug)]
pub(in crate::mir::builder) struct ActiveRootBodyCompletionTrackerV1 {
    tracker: RootBodyCompletionTrackerV1,
    _seal: ActiveRootBodyCompletionTrackerSealV1,
}

#[derive(Debug)]
struct ActiveRootBodyCompletionTrackerSealV1;

#[derive(Debug)]
struct RootBodyCompletionTrackerSealV1;

/// A non-Clone, single-use witness that recursive root-body descent is closed.
#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedRootBodyV1 {
    brand: ModuleInvocationBrandV1,
    result: RootBodyResultV1,
    completed_children: usize,
    _seal: CompletedRootBodySealV1,
}

#[derive(Debug)]
struct CompletedRootBodySealV1;

#[cfg(test)]
static NEXT_ROOT_BODY_OWNER: AtomicU64 = AtomicU64::new(1);

impl RootBodyCompletionTrackerV1 {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) const fn completed_children(&self) -> usize {
        self.completed_children
    }

    pub(in crate::mir::builder) const fn is_fresh(&self) -> bool {
        self.open_children == 0
            && self.open_header_loans == 0
            && self.open_pending_terminals == 0
            && self.completed_children == 0
    }

    pub(in crate::mir::builder) fn new_for_brand(brand: ModuleInvocationBrandV1) -> Self {
        Self {
            brand,
            owner: 0,
            open_children: 0,
            open_header_loans: 0,
            open_pending_terminals: 0,
            completed_children: 0,
            _seal: RootBodyCompletionTrackerSealV1,
        }
    }

    pub(in crate::mir::builder) fn begin_root_body(
        self,
    ) -> Result<ActiveRootBodyCompletionTrackerV1, RootBodyCompletionErrorV1> {
        if !self.is_fresh() {
            return Err(RootBodyCompletionErrorV1::AlreadyDriven);
        }
        Ok(ActiveRootBodyCompletionTrackerV1 {
            tracker: self,
            _seal: ActiveRootBodyCompletionTrackerSealV1,
        })
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn new() -> Self {
        Self {
            brand: ModuleInvocationBrandV1::legacy_test(),
            owner: NEXT_ROOT_BODY_OWNER.fetch_add(1, Ordering::Relaxed),
            open_children: 0,
            open_header_loans: 0,
            open_pending_terminals: 0,
            completed_children: 0,
            _seal: RootBodyCompletionTrackerSealV1,
        }
    }

    pub(in crate::mir::builder) fn begin_child(&mut self) -> RootBodyActivityTokenV1 {
        self.open_children += 1;
        self.token(RootBodyActivityKindV1::Child)
    }

    pub(in crate::mir::builder) fn begin_header_loan(&mut self) -> RootBodyActivityTokenV1 {
        self.open_header_loans += 1;
        self.token(RootBodyActivityKindV1::HeaderLoan)
    }

    pub(in crate::mir::builder) fn begin_pending_terminal(&mut self) -> RootBodyActivityTokenV1 {
        self.open_pending_terminals += 1;
        self.token(RootBodyActivityKindV1::PendingTerminal)
    }

    pub(in crate::mir::builder) fn close_child(
        &mut self,
        token: RootBodyActivityTokenV1,
    ) -> Result<(), RootBodyCompletionErrorV1> {
        Self::close_token(
            self.brand,
            self.owner,
            token,
            RootBodyActivityKindV1::Child,
            &mut self.open_children,
        )?;
        self.completed_children += 1;
        Ok(())
    }

    pub(in crate::mir::builder) fn close_header_loan(
        &mut self,
        token: RootBodyActivityTokenV1,
    ) -> Result<(), RootBodyCompletionErrorV1> {
        Self::close_token(
            self.brand,
            self.owner,
            token,
            RootBodyActivityKindV1::HeaderLoan,
            &mut self.open_header_loans,
        )
    }

    pub(in crate::mir::builder) fn close_pending_terminal(
        &mut self,
        token: RootBodyActivityTokenV1,
    ) -> Result<(), RootBodyCompletionErrorV1> {
        Self::close_token(
            self.brand,
            self.owner,
            token,
            RootBodyActivityKindV1::PendingTerminal,
            &mut self.open_pending_terminals,
        )
    }

    pub(in crate::mir::builder) fn complete(
        self,
        result: RootBodyResultV1,
    ) -> Result<CompletedRootBodyV1, RootBodyCompletionErrorV1> {
        if self.open_children != 0 {
            return Err(RootBodyCompletionErrorV1::OpenChildScopes {
                count: self.open_children,
            });
        }
        if self.open_header_loans != 0 {
            return Err(RootBodyCompletionErrorV1::OpenHeaderLoans {
                count: self.open_header_loans,
            });
        }
        if self.open_pending_terminals != 0 {
            return Err(RootBodyCompletionErrorV1::OpenPendingTerminals {
                count: self.open_pending_terminals,
            });
        }
        Ok(CompletedRootBodyV1 {
            brand: self.brand,
            result,
            completed_children: self.completed_children,
            _seal: CompletedRootBodySealV1,
        })
    }

    fn token(&self, kind: RootBodyActivityKindV1) -> RootBodyActivityTokenV1 {
        RootBodyActivityTokenV1 {
            brand: self.brand,
            owner: self.owner,
            kind,
            _seal: RootBodyActivityTokenSealV1,
        }
    }

    fn close_token(
        brand: ModuleInvocationBrandV1,
        owner: u64,
        token: RootBodyActivityTokenV1,
        expected: RootBodyActivityKindV1,
        count: &mut usize,
    ) -> Result<(), RootBodyCompletionErrorV1> {
        if token.brand != brand {
            return Err(RootBodyCompletionErrorV1::ForeignBrand);
        }
        if token.owner != owner {
            return Err(RootBodyCompletionErrorV1::ForeignToken);
        }
        if token.kind != expected {
            return Err(RootBodyCompletionErrorV1::TokenKindMismatch);
        }
        if *count == 0 {
            return Err(RootBodyCompletionErrorV1::NoOpenToken);
        }
        *count -= 1;
        Ok(())
    }
}

impl ActiveRootBodyCompletionTrackerV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.tracker.brand
    }

    pub(in crate::mir::builder) fn tracker_mut(&mut self) -> &mut RootBodyCompletionTrackerV1 {
        &mut self.tracker
    }

    pub(in crate::mir::builder) fn seal_root_body(
        self,
        result: RootBodyResultV1,
    ) -> Result<CompletedRootBodyV1, RootBodyCompletionErrorV1> {
        self.tracker.complete(result)
    }
}

impl CompletedRootBodyV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }
    pub(in crate::mir::builder) fn result(&self) -> RootBodyResultV1 {
        self.result
    }

    pub(in crate::mir::builder) fn completed_children(&self) -> usize {
        self.completed_children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_root_body_seals_explicit_no_value() {
        let completed = RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::NoValue)
            .unwrap();
        assert_eq!(completed.result(), RootBodyResultV1::NoValue);
        assert_eq!(completed.completed_children(), 0);
    }

    #[test]
    fn nested_activity_closes_before_value_witness() {
        let mut tracker = RootBodyCompletionTrackerV1::new();
        let child = tracker.begin_child();
        let header = tracker.begin_header_loan();
        let pending = tracker.begin_pending_terminal();
        tracker.close_pending_terminal(pending).unwrap();
        tracker.close_header_loan(header).unwrap();
        tracker.close_child(child).unwrap();

        let completed = tracker
            .complete(RootBodyResultV1::Value(ValueId::new(7)))
            .unwrap();
        assert_eq!(completed.result(), RootBodyResultV1::Value(ValueId::new(7)));
        assert_eq!(completed.completed_children(), 1);
    }

    #[test]
    fn open_activity_rejects_completion_before_builder_effects() {
        let mut tracker = RootBodyCompletionTrackerV1::new();
        let _child = tracker.begin_child();
        assert_eq!(
            tracker.complete(RootBodyResultV1::NoValue).unwrap_err(),
            RootBodyCompletionErrorV1::OpenChildScopes { count: 1 }
        );
    }

    #[test]
    fn foreign_and_mismatched_tokens_fail_closed() {
        let mut first = RootBodyCompletionTrackerV1::new();
        let mut second = RootBodyCompletionTrackerV1::new();
        let child = first.begin_child();
        let header = second.begin_header_loan();

        assert_eq!(
            first.close_child(header).unwrap_err(),
            RootBodyCompletionErrorV1::ForeignToken
        );

        first.close_child(child).unwrap();

        let mut mismatch_tracker = RootBodyCompletionTrackerV1::new();
        let mismatch_child = mismatch_tracker.begin_child();
        let mismatch_header = mismatch_tracker.begin_header_loan();
        assert_eq!(
            mismatch_tracker.close_child(mismatch_header).unwrap_err(),
            RootBodyCompletionErrorV1::TokenKindMismatch
        );
        mismatch_tracker.close_child(mismatch_child).unwrap();
    }

    #[test]
    fn body0_typestate_requires_begin_before_seal() {
        let active = RootBodyCompletionTrackerV1::new()
            .begin_root_body()
            .unwrap();
        let completed = active.seal_root_body(RootBodyResultV1::NoValue).unwrap();
        assert_eq!(completed.result(), RootBodyResultV1::NoValue);
    }
}
