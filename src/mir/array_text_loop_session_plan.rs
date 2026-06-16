//! Passive proof surface for array text loop-session lowering.
//!
//! This module does not select or lower routes. It only names the proof fields
//! required before a backend may reuse a read-only array text session across a
//! loop/window.

use super::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextLoopSessionRejectReason {
    DifferentArrayHandle,
    UnknownLoopRegion,
    ArrayMutationInRegion,
    DropOrPublicationBoundary,
    IndexDomainUnproven,
}

impl ArrayTextLoopSessionRejectReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DifferentArrayHandle => "different_array_handle",
            Self::UnknownLoopRegion => "unknown_loop_region",
            Self::ArrayMutationInRegion => "array_mutation_in_region",
            Self::DropOrPublicationBoundary => "drop_or_publication_boundary",
            Self::IndexDomainUnproven => "index_domain_unproven",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextLoopSessionPlan {
    loop_header: BasicBlockId,
    loop_exit: BasicBlockId,
    array_value: ValueId,
    index_value: ValueId,
    len_call_count: usize,
    same_array_handle: bool,
    read_only_region: bool,
    no_mutation_region: bool,
    no_drop_or_publication_boundary: bool,
    index_domain_guarded: bool,
}

impl ArrayTextLoopSessionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        loop_header: BasicBlockId,
        loop_exit: BasicBlockId,
        array_value: ValueId,
        index_value: ValueId,
        len_call_count: usize,
        same_array_handle: bool,
        read_only_region: bool,
        no_mutation_region: bool,
        no_drop_or_publication_boundary: bool,
        index_domain_guarded: bool,
    ) -> Self {
        Self {
            loop_header,
            loop_exit,
            array_value,
            index_value,
            len_call_count,
            same_array_handle,
            read_only_region,
            no_mutation_region,
            no_drop_or_publication_boundary,
            index_domain_guarded,
        }
    }

    pub fn loop_header(&self) -> BasicBlockId {
        self.loop_header
    }

    pub fn loop_exit(&self) -> BasicBlockId {
        self.loop_exit
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn len_call_count(&self) -> usize {
        self.len_call_count
    }

    pub fn backend_session_lowering_allowed(&self) -> bool {
        self.len_call_count > 0
            && self.same_array_handle
            && self.read_only_region
            && self.no_mutation_region
            && self.no_drop_or_publication_boundary
            && self.index_domain_guarded
    }

    pub fn first_reject_reason(&self) -> Option<ArrayTextLoopSessionRejectReason> {
        if !self.same_array_handle {
            return Some(ArrayTextLoopSessionRejectReason::DifferentArrayHandle);
        }
        if !self.read_only_region {
            return Some(ArrayTextLoopSessionRejectReason::UnknownLoopRegion);
        }
        if !self.no_mutation_region {
            return Some(ArrayTextLoopSessionRejectReason::ArrayMutationInRegion);
        }
        if !self.no_drop_or_publication_boundary {
            return Some(ArrayTextLoopSessionRejectReason::DropOrPublicationBoundary);
        }
        if !self.index_domain_guarded {
            return Some(ArrayTextLoopSessionRejectReason::IndexDomainUnproven);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan_with_flags(
        same_array_handle: bool,
        read_only_region: bool,
        no_mutation_region: bool,
        no_drop_or_publication_boundary: bool,
        index_domain_guarded: bool,
    ) -> ArrayTextLoopSessionPlan {
        ArrayTextLoopSessionPlan::new(
            BasicBlockId::new(10),
            BasicBlockId::new(20),
            ValueId::new(1),
            ValueId::new(2),
            3,
            same_array_handle,
            read_only_region,
            no_mutation_region,
            no_drop_or_publication_boundary,
            index_domain_guarded,
        )
    }

    #[test]
    fn complete_plan_allows_backend_session_lowering() {
        let plan = plan_with_flags(true, true, true, true, true);
        assert!(plan.backend_session_lowering_allowed());
        assert_eq!(plan.first_reject_reason(), None);
    }

    #[test]
    fn mutation_rejects_backend_session_lowering() {
        let plan = plan_with_flags(true, true, false, true, true);
        assert!(!plan.backend_session_lowering_allowed());
        assert_eq!(
            plan.first_reject_reason(),
            Some(ArrayTextLoopSessionRejectReason::ArrayMutationInRegion)
        );
    }

    #[test]
    fn unguarded_index_rejects_backend_session_lowering() {
        let plan = plan_with_flags(true, true, true, true, false);
        assert!(!plan.backend_session_lowering_allowed());
        assert_eq!(
            plan.first_reject_reason(),
            Some(ArrayTextLoopSessionRejectReason::IndexDomainUnproven)
        );
    }
}
