/*!
 * MIR-owned DirectArray access plans.
 *
 * This is the first behavior-preserving seam that lifts exact ArrayBox
 * get/set candidates out of backend string/method-name recognition.  The v0
 * plan is metadata-only: it records checked DirectArrayI64 candidates derived
 * from `generic_method_routes`, and later lowering slices may consume it.
 */

mod proofs;

use crate::mir::{BasicBlockId, MirFunction, ValueId};
use proofs::{
    array_store_value_arg, caller_precondition_proves_branchless_store,
    checked_direct_array_lowering_site_is_cfg_safe,
    range_index_proves_branchless_append_or_overwrite_store, stack_top_pop_proves_branchless_load,
    stack_top_pop_store_index_origins,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayAccessOp {
    Load,
    Store,
}

impl DirectArrayAccessOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Load => "load",
            Self::Store => "store",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayBoundsPolicy {
    Checked,
    ProvedUnchecked,
}

impl DirectArrayBoundsPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::ProvedUnchecked => "proved_unchecked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayProofKind {
    ExactFrontContract,
    RangeIndex,
    StackTopPop,
    CallerPrecondition,
}

impl DirectArrayProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactFrontContract => "exact_front_contract",
            Self::RangeIndex => "range_index",
            Self::StackTopPop => "stack_top_pop",
            Self::CallerPrecondition => "caller_precondition",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayFallbackPolicy {
    AllowChecked,
    FailFast,
}

impl DirectArrayFallbackPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowChecked => "allow_checked",
            Self::FailFast => "fail_fast",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayCfgShape {
    CheckedBranching,
    Branchless,
}

impl DirectArrayCfgShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CheckedBranching => "checked_branching",
            Self::Branchless => "branchless",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayStoreSemantics {
    NotStore,
    AppendOrOverwrite,
    OverwriteExisting,
}

impl DirectArrayStoreSemantics {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStore => "not_store",
            Self::AppendOrOverwrite => "append_or_overwrite",
            Self::OverwriteExisting => "overwrite_existing",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectArrayAccessPlan {
    block: BasicBlockId,
    instruction_index: usize,
    op: DirectArrayAccessOp,
    receiver_value: ValueId,
    index_value: ValueId,
    value_value: Option<ValueId>,
    result_value: Option<ValueId>,
    array_kind: &'static str,
    element_type: &'static str,
    route: &'static str,
    bounds_policy: DirectArrayBoundsPolicy,
    proof_kind: DirectArrayProofKind,
    proof_ids: Vec<&'static str>,
    fallback_policy: DirectArrayFallbackPolicy,
    cfg_shape: DirectArrayCfgShape,
    store_semantics: DirectArrayStoreSemantics,
}

impl DirectArrayAccessPlan {
    fn checked(
        block: BasicBlockId,
        instruction_index: usize,
        op: DirectArrayAccessOp,
        receiver_value: ValueId,
        index_value: ValueId,
        value_value: Option<ValueId>,
        result_value: Option<ValueId>,
    ) -> Self {
        let route = match op {
            DirectArrayAccessOp::Load => "direct_array_i64_load",
            DirectArrayAccessOp::Store => "direct_array_i64_store",
        };
        Self {
            block,
            instruction_index,
            op,
            receiver_value,
            index_value,
            value_value,
            result_value,
            array_kind: "DirectArrayI64",
            element_type: "i64",
            route,
            bounds_policy: DirectArrayBoundsPolicy::Checked,
            proof_kind: DirectArrayProofKind::ExactFrontContract,
            proof_ids: vec![DirectArrayProofKind::ExactFrontContract.as_str()],
            fallback_policy: DirectArrayFallbackPolicy::AllowChecked,
            cfg_shape: DirectArrayCfgShape::CheckedBranching,
            store_semantics: match op {
                DirectArrayAccessOp::Load => DirectArrayStoreSemantics::NotStore,
                DirectArrayAccessOp::Store => DirectArrayStoreSemantics::AppendOrOverwrite,
            },
        }
    }

    fn proved_unchecked_range_index_store(
        block: BasicBlockId,
        instruction_index: usize,
        receiver_value: ValueId,
        index_value: ValueId,
        value_value: ValueId,
        result_value: Option<ValueId>,
    ) -> Self {
        Self {
            block,
            instruction_index,
            op: DirectArrayAccessOp::Store,
            receiver_value,
            index_value,
            value_value: Some(value_value),
            result_value,
            array_kind: "DirectArrayI64",
            element_type: "i64",
            route: "direct_array_i64_store",
            bounds_policy: DirectArrayBoundsPolicy::ProvedUnchecked,
            proof_kind: DirectArrayProofKind::RangeIndex,
            proof_ids: vec![DirectArrayProofKind::RangeIndex.as_str()],
            fallback_policy: DirectArrayFallbackPolicy::FailFast,
            cfg_shape: DirectArrayCfgShape::Branchless,
            // RangeIndex v0 proves a sequential 0..end fill. The branchless
            // lowerer preserves Array.set append-or-overwrite semantics by
            // updating len to max(len, index + 1), so this is not the legacy
            // raw overwrite-only unchecked store.
            store_semantics: DirectArrayStoreSemantics::AppendOrOverwrite,
        }
    }

    fn proved_unchecked_stack_top_pop_load(
        block: BasicBlockId,
        instruction_index: usize,
        receiver_value: ValueId,
        index_value: ValueId,
        result_value: Option<ValueId>,
    ) -> Self {
        Self {
            block,
            instruction_index,
            op: DirectArrayAccessOp::Load,
            receiver_value,
            index_value,
            value_value: None,
            result_value,
            array_kind: "DirectArrayI64",
            element_type: "i64",
            route: "direct_array_i64_load",
            bounds_policy: DirectArrayBoundsPolicy::ProvedUnchecked,
            proof_kind: DirectArrayProofKind::StackTopPop,
            proof_ids: vec![DirectArrayProofKind::StackTopPop.as_str()],
            fallback_policy: DirectArrayFallbackPolicy::FailFast,
            cfg_shape: DirectArrayCfgShape::Branchless,
            store_semantics: DirectArrayStoreSemantics::NotStore,
        }
    }

    fn proved_unchecked_stack_top_pop_store(
        block: BasicBlockId,
        instruction_index: usize,
        receiver_value: ValueId,
        index_value: ValueId,
        value_value: ValueId,
        result_value: Option<ValueId>,
    ) -> Self {
        Self {
            block,
            instruction_index,
            op: DirectArrayAccessOp::Store,
            receiver_value,
            index_value,
            value_value: Some(value_value),
            result_value,
            array_kind: "DirectArrayI64",
            element_type: "i64",
            route: "direct_array_i64_store",
            bounds_policy: DirectArrayBoundsPolicy::ProvedUnchecked,
            proof_kind: DirectArrayProofKind::StackTopPop,
            proof_ids: vec![DirectArrayProofKind::StackTopPop.as_str()],
            fallback_policy: DirectArrayFallbackPolicy::FailFast,
            cfg_shape: DirectArrayCfgShape::Branchless,
            store_semantics: DirectArrayStoreSemantics::OverwriteExisting,
        }
    }

    fn proved_unchecked_caller_precondition_store(
        block: BasicBlockId,
        instruction_index: usize,
        receiver_value: ValueId,
        index_value: ValueId,
        value_value: ValueId,
        result_value: Option<ValueId>,
    ) -> Self {
        Self {
            block,
            instruction_index,
            op: DirectArrayAccessOp::Store,
            receiver_value,
            index_value,
            value_value: Some(value_value),
            result_value,
            array_kind: "DirectArrayI64",
            element_type: "i64",
            route: "direct_array_i64_store",
            bounds_policy: DirectArrayBoundsPolicy::ProvedUnchecked,
            proof_kind: DirectArrayProofKind::CallerPrecondition,
            proof_ids: vec![DirectArrayProofKind::CallerPrecondition.as_str()],
            fallback_policy: DirectArrayFallbackPolicy::FailFast,
            cfg_shape: DirectArrayCfgShape::Branchless,
            store_semantics: DirectArrayStoreSemantics::OverwriteExisting,
        }
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn op(&self) -> DirectArrayAccessOp {
        self.op
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn value_value(&self) -> Option<ValueId> {
        self.value_value
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.result_value
    }

    pub fn array_kind(&self) -> &'static str {
        self.array_kind
    }

    pub fn element_type(&self) -> &'static str {
        self.element_type
    }

    pub fn route(&self) -> &'static str {
        self.route
    }

    pub fn bounds_policy(&self) -> DirectArrayBoundsPolicy {
        self.bounds_policy
    }

    pub fn proof_kind(&self) -> DirectArrayProofKind {
        self.proof_kind
    }

    pub fn proof_ids(&self) -> &[&'static str] {
        &self.proof_ids
    }

    pub fn fallback_policy(&self) -> DirectArrayFallbackPolicy {
        self.fallback_policy
    }

    pub fn cfg_shape(&self) -> DirectArrayCfgShape {
        self.cfg_shape
    }

    pub fn store_semantics(&self) -> DirectArrayStoreSemantics {
        self.store_semantics
    }
}

pub fn refresh_function_direct_array_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let mut stack_top_pop_values = stack_top_pop_store_index_origins();
    for route in &function.metadata.generic_method_routes {
        if !matches!(
            route.receiver_origin_box(),
            Some("ArrayBox" | "DirectArrayI64")
        ) {
            continue;
        }
        let Some(index_value) = route.key_value() else {
            continue;
        };
        match route.route_kind_tag() {
            "array_slot_load_any" => {
                if stack_top_pop_proves_branchless_load(function, route.block(), index_value) {
                    plans.push(DirectArrayAccessPlan::proved_unchecked_stack_top_pop_load(
                        route.block(),
                        route.instruction_index(),
                        route.receiver_value(),
                        index_value,
                        route.result_value(),
                    ));
                    if let Some(result_value) = route.result_value() {
                        stack_top_pop_values.record(function, result_value);
                    }
                } else if !checked_direct_array_lowering_site_is_cfg_safe(function, route.block()) {
                    continue;
                } else {
                    plans.push(DirectArrayAccessPlan::checked(
                        route.block(),
                        route.instruction_index(),
                        DirectArrayAccessOp::Load,
                        route.receiver_value(),
                        index_value,
                        None,
                        route.result_value(),
                    ));
                }
            }
            "array_store_any" => {
                let Some(value_value) = array_store_value_arg(
                    function,
                    route.block(),
                    route.instruction_index(),
                    index_value,
                ) else {
                    continue;
                };
                if range_index_proves_branchless_append_or_overwrite_store(
                    function,
                    route.block(),
                    route.receiver_value(),
                    index_value,
                ) {
                    plans.push(DirectArrayAccessPlan::proved_unchecked_range_index_store(
                        route.block(),
                        route.instruction_index(),
                        route.receiver_value(),
                        index_value,
                        value_value,
                        route.result_value(),
                    ));
                } else if stack_top_pop_values.contains(function, index_value) {
                    plans.push(DirectArrayAccessPlan::proved_unchecked_stack_top_pop_store(
                        route.block(),
                        route.instruction_index(),
                        route.receiver_value(),
                        index_value,
                        value_value,
                        route.result_value(),
                    ));
                } else if caller_precondition_proves_branchless_store(
                    function,
                    route.receiver_value(),
                    index_value,
                    value_value,
                ) {
                    plans.push(
                        DirectArrayAccessPlan::proved_unchecked_caller_precondition_store(
                            route.block(),
                            route.instruction_index(),
                            route.receiver_value(),
                            index_value,
                            value_value,
                            route.result_value(),
                        ),
                    );
                } else if checked_direct_array_lowering_site_is_cfg_safe(function, route.block()) {
                    plans.push(DirectArrayAccessPlan::checked(
                        route.block(),
                        route.instruction_index(),
                        DirectArrayAccessOp::Store,
                        route.receiver_value(),
                        index_value,
                        Some(value_value),
                        route.result_value(),
                    ));
                }
            }
            _ => {}
        }
    }
    function.metadata.direct_array_access_plans = plans;
}

#[cfg(test)]
mod tests;
