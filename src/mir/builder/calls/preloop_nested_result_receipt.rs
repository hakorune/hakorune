//! One-shot pre-loop nested-result receipt ownership.
//!
//! This box joins the exact retained Integer source association with one
//! successful generic physical Call receipt. It retains both until the outer
//! call succeeds, then emits a destination-only receipt. It owns no Call
//! emission, source lookup, `MirType`, `type_ctx`, retry, or fallback policy.

use crate::mir::builder::calls::unified_emitter::CompletedUnifiedValueCallEmissionV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::ValueId;

/// Exact source authority paired with the successful selected inner Call.
#[derive(Debug)]
pub(super) struct ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog> {
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    physical: CompletedUnifiedValueCallEmissionV1,
    _seal: ReachedPreloopNestedPhysicalCallSealV1,
}

#[derive(Debug)]
struct ReachedPreloopNestedPhysicalCallSealV1;

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

    pub(super) const fn final_destination(&self) -> ValueId {
        self.physical.final_destination()
    }

    pub(super) fn complete_after_outer_success(self) -> EmittedNestedInstanceCallV1 {
        EmittedNestedInstanceCallV1 {
            final_destination: self.physical.final_destination(),
            _seal: EmittedNestedInstanceCallSealV1,
        }
    }

    pub(super) fn discard(self) {
        self.source.discard();
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

    pub(super) fn discard(self) {}
}
