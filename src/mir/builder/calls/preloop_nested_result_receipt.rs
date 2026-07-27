//! One-shot pre-loop nested-result receipt ownership.
//!
//! This box joins the exact retained Integer source association with one
//! successful generic physical Call receipt. It retains both until the outer
//! call succeeds, then emits a destination-only receipt. It owns no Call
//! emission, source lookup, `MirType`, `type_ctx`, retry, or fallback policy.

use crate::mir::builder::calls::unified_emitter::CompletedUnifiedValueCallEmissionV1;
use crate::mir::source_instance_result_contract::{
    PreparedPreloopLocatedArgumentV1, RetainedNestedInstanceResultRebindAuthorityV1,
};
use crate::mir::ValueId;

use super::preloop_located_argument_ingress::{
    PreloopLocatedArgumentIngressErrorV1, PreloopLocatedArgumentIngressStageV1,
    RejectedPreloopLocatedArgumentIngressV1,
};

/// Exact source authority paired with the successful selected inner Call.
#[derive(Debug)]
pub(super) struct ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog> {
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    physical: CompletedUnifiedValueCallEmissionV1,
    _seal: ReachedPreloopNestedPhysicalCallSealV1,
}

#[derive(Debug)]
struct ReachedPreloopNestedPhysicalCallSealV1;

#[derive(Debug)]
pub(super) struct OwnedPreloopNestedPhysicalPartsV1 {
    pub(super) nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
    pub(super) inner_call: CompletedUnifiedValueCallEmissionV1,
}

#[derive(Debug)]
pub(super) struct OwnedPreloopOuterPhysicalPartsV1 {
    pub(super) nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
    pub(super) inner_call: CompletedUnifiedValueCallEmissionV1,
    pub(super) outer_call: CompletedUnifiedValueCallEmissionV1,
}

impl OwnedPreloopOuterPhysicalPartsV1 {
    pub(super) fn discard(self) {
        self.nested_result.discard();
        let _ = (self.inner_call, self.outer_call);
    }
}

#[derive(Debug)]
pub(super) enum OwnedPreloopPhysicalProgressV1 {
    Source {
        nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
    },
    Inner(OwnedPreloopNestedPhysicalPartsV1),
    Outer(OwnedPreloopOuterPhysicalPartsV1),
}

impl OwnedPreloopPhysicalProgressV1 {
    pub(super) fn source(nested_result: RetainedNestedInstanceResultRebindAuthorityV1) -> Self {
        Self::Source { nested_result }
    }

    pub(super) fn discard(self) {
        match self {
            Self::Source { nested_result } => nested_result.discard(),
            Self::Inner(parts) => {
                parts.nested_result.discard();
                let _ = parts.inner_call;
            }
            Self::Outer(parts) => parts.discard(),
        }
    }
}

impl<'site, 'view, 'catalog> ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog> {
    pub(super) fn prepare(
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        physical: CompletedUnifiedValueCallEmissionV1,
    ) -> Self {
        Self {
            source,
            physical,
            _seal: ReachedPreloopNestedPhysicalCallSealV1,
        }
    }

    pub(super) const fn selected_index(&self) -> u32 {
        self.source.selected().index()
    }

    pub(super) fn selected_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.source.selected().child().site()
    }

    pub(super) fn caller(&self) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        self.source.selected().parent().caller()
    }

    pub(super) fn outer_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.source.selected().parent().site()
    }

    pub(super) const fn final_destination(&self) -> ValueId {
        self.physical.final_destination()
    }

    pub(super) fn complete_after_outer_success(self) -> EmittedNestedInstanceCallV1 {
        EmittedNestedInstanceCallV1 {
            final_destination: self.physical.final_destination(),
            _seal: EmittedNestedInstanceCallSealV1,
        }
    }

    pub(super) fn reject_outer_terminal(
        self,
        detail: String,
    ) -> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
        RejectedPreloopLocatedArgumentIngressV1::after_physical(
            self,
            PreloopLocatedArgumentIngressStageV1::OuterTerminal,
            PreloopLocatedArgumentIngressErrorV1::OuterTerminal {
                detail: detail.into_boxed_str(),
            },
        )
    }

    pub(super) fn reject_outer_not_completed(
        self,
    ) -> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
        RejectedPreloopLocatedArgumentIngressV1::after_physical(
            self,
            PreloopLocatedArgumentIngressStageV1::Completion,
            PreloopLocatedArgumentIngressErrorV1::OuterTerminalNotCompleted,
        )
    }

    pub(super) fn discard(self) {
        self.source.discard();
    }

    pub(super) fn into_owned_parts_v1(self) -> OwnedPreloopNestedPhysicalPartsV1 {
        OwnedPreloopNestedPhysicalPartsV1 {
            nested_result: self.source.into_completed_retained_rebind_authority(),
            inner_call: self.physical,
        }
    }
}

/// Exact selected-inner authority paired with the successful containing
/// physical Call. The outer destination can only come from `outer`.
#[derive(Debug)]
pub(super) struct ReachedPreloopOuterPhysicalCallV1<'site, 'view, 'catalog> {
    inner: ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>,
    outer: CompletedUnifiedValueCallEmissionV1,
    _seal: ReachedPreloopOuterPhysicalCallSealV1,
}

#[derive(Debug)]
struct ReachedPreloopOuterPhysicalCallSealV1;

impl<'site, 'view, 'catalog> ReachedPreloopOuterPhysicalCallV1<'site, 'view, 'catalog> {
    pub(super) fn prepare(
        inner: ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>,
        outer: CompletedUnifiedValueCallEmissionV1,
    ) -> Self {
        Self {
            inner,
            outer,
            _seal: ReachedPreloopOuterPhysicalCallSealV1,
        }
    }

    pub(super) const fn inner(
        &self,
    ) -> &ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog> {
        &self.inner
    }

    pub(super) const fn outer_destination(&self) -> ValueId {
        self.outer.final_destination()
    }

    pub(super) fn discard(self) {
        self.inner.discard();
    }

    pub(super) fn into_owned_parts_v1(self) -> OwnedPreloopOuterPhysicalPartsV1 {
        let inner = self.inner.into_owned_parts_v1();
        OwnedPreloopOuterPhysicalPartsV1 {
            nested_result: inner.nested_result,
            inner_call: inner.inner_call,
            outer_call: self.outer,
        }
    }
}

/// Final source-qualified nested instance result.
///
/// The private seal proves that an exact Integer source association survived
/// one successful selected inner Call and the containing outer call.
#[derive(Debug)]
pub(super) struct EmittedNestedInstanceCallV1 {
    final_destination: ValueId,
    _seal: EmittedNestedInstanceCallSealV1,
}

#[derive(Debug)]
struct EmittedNestedInstanceCallSealV1;

impl EmittedNestedInstanceCallV1 {
    pub(super) const fn final_destination(&self) -> ValueId {
        self.final_destination
    }

    #[cfg(test)]
    pub(super) const fn from_destination_for_test(final_destination: ValueId) -> Self {
        Self {
            final_destination,
            _seal: EmittedNestedInstanceCallSealV1,
        }
    }

    pub(super) fn discard(self) {}
}
