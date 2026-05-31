/*!
 * MIR-owned DirectArray access plans.
 *
 * This is the first behavior-preserving seam that lifts exact ArrayBox
 * get/set candidates out of backend string/method-name recognition.  The v0
 * plan is metadata-only: it records checked DirectArrayI64 candidates derived
 * from `generic_method_routes`, and later lowering slices may consume it.
 */

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
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
    let def_map = build_value_def_map(function);
    let mut stack_top_pop_values = Vec::new();
    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("ArrayBox") {
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
                        stack_top_pop_values.push(resolve_value_origin(
                            function,
                            &def_map,
                            result_value,
                        ));
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
                } else if stack_top_pop_values
                    .iter()
                    .any(|origin| *origin == resolve_value_origin(function, &def_map, index_value))
                {
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

fn stack_top_pop_proves_branchless_load(
    function: &MirFunction,
    block_id: BasicBlockId,
    index_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let Some(stack_top_value) = binop_sub_one_lhs(function, index_value) else {
        return false;
    };
    predecessor_branch_proves_nonzero_on_edge(function, &def_map, block_id, stack_top_value)
}

fn caller_precondition_proves_branchless_store(
    function: &MirFunction,
    receiver_value: ValueId,
    index_value: ValueId,
    value_value: ValueId,
) -> bool {
    if function.signature.name != "HakoAllocPageModel.releaseLocalKnownLive/1" {
        return false;
    }
    let Some(me_value) = function.params.first().copied() else {
        return false;
    };
    let Some(block_id_value) = function.params.get(1).copied() else {
        return false;
    };
    let def_map = build_value_def_map(function);
    if value_origin_is_field_get(function, &def_map, receiver_value, me_value, "block_used") {
        return resolve_value_origin(function, &def_map, index_value)
            == resolve_value_origin(function, &def_map, block_id_value)
            && value_is_integer_const(function, value_value, 0);
    }
    if value_origin_is_field_get(function, &def_map, receiver_value, me_value, "local_free") {
        return value_origin_is_field_get(
            function,
            &def_map,
            index_value,
            me_value,
            "local_free_top",
        ) && resolve_value_origin(function, &def_map, value_value)
            == resolve_value_origin(function, &def_map, block_id_value);
    }
    false
}

fn value_origin_is_field_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value_id: ValueId,
    base_value: ValueId,
    expected_field: &str,
) -> bool {
    let value_origin = resolve_value_origin(function, def_map, value_id);
    let base_origin = resolve_value_origin(function, def_map, base_value);
    function.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::FieldGet {
                    dst,
                    base,
                    field,
                    ..
                } if resolve_value_origin(function, def_map, *dst) == value_origin
                    && resolve_value_origin(function, def_map, *base) == base_origin
                    && field == expected_field
            )
        })
    })
}

fn binop_sub_one_lhs(function: &MirFunction, value_id: ValueId) -> Option<ValueId> {
    let def_map = build_value_def_map(function);
    let origin = resolve_value_origin(function, &def_map, value_id);
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Sub,
                    lhs,
                    rhs,
                } if resolve_value_origin(function, &def_map, *dst) == origin
                    && value_is_integer_const(function, *rhs, 1) =>
                {
                    Some(*lhs)
                }
                _ => None,
            })
    })
}

fn predecessor_branch_proves_nonzero_on_edge(
    function: &MirFunction,
    def_map: &ValueDefMap,
    target_block: BasicBlockId,
    value: ValueId,
) -> bool {
    let value_origin = resolve_value_origin(function, def_map, value);
    function.blocks.values().any(|pred| {
        let Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) = &pred.terminator
        else {
            return false;
        };
        let Some((op, lhs, rhs)) = compare_def(function, *condition) else {
            return false;
        };
        let lhs_matches = resolve_value_origin(function, def_map, lhs) == value_origin
            && value_is_integer_const(function, rhs, 0);
        let rhs_matches = resolve_value_origin(function, def_map, rhs) == value_origin
            && value_is_integer_const(function, lhs, 0);
        if !(lhs_matches || rhs_matches) {
            return false;
        }
        match op {
            CompareOp::Eq => *else_bb == target_block,
            CompareOp::Ne => *then_bb == target_block,
            _ => false,
        }
    })
}

fn compare_def(function: &MirFunction, value_id: ValueId) -> Option<(CompareOp, ValueId, ValueId)> {
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Compare { dst, op, lhs, rhs } if *dst == value_id => {
                    Some((*op, *lhs, *rhs))
                }
                _ => None,
            })
    })
}

fn range_index_proves_branchless_append_or_overwrite_store(
    function: &MirFunction,
    block_id: BasicBlockId,
    receiver_value: ValueId,
    index_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let index_origin = resolve_value_origin(function, &def_map, index_value);
    let receiver_origin = resolve_value_origin(function, &def_map, receiver_value);
    function.metadata.range_index_facts.iter().any(|fact| {
        fact.body_bb == block_id
            && resolve_value_origin(function, &def_map, fact.index_value) == index_origin
            && fact.step == 1
            && fact.end_exclusive
            && fact.index_body_read_only
            && !fact.loop_carried_writes_supported
            && value_is_integer_const(function, fact.lower_value, 0)
            && direct_array_extent_v0_proves_upper_bound(
                function,
                receiver_origin,
                fact.upper_exclusive_value,
            )
    })
}

fn value_is_integer_const(function: &MirFunction, value_id: ValueId, expected: i64) -> bool {
    integer_const_value(function, value_id)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn direct_array_extent_v0_proves_upper_bound(
    function: &MirFunction,
    receiver_value: ValueId,
    end_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let receiver_origin = resolve_value_origin(function, &def_map, receiver_value);
    let end_origin = resolve_value_origin(function, &def_map, end_value);
    function
        .metadata
        .direct_array_extent_facts
        .iter()
        .any(|fact| {
            resolve_value_origin(function, &def_map, fact.receiver_value) == receiver_origin
                && resolve_value_origin(function, &def_map, fact.lower_bound_value) == end_origin
                && fact.stable_in_region
                && direct_array_region_stability_v0_proves(
                    function,
                    fact.region_stability_fact_id,
                    receiver_origin,
                )
        })
}

fn direct_array_region_stability_v0_proves(
    function: &MirFunction,
    fact_id: u32,
    receiver_origin: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    function.metadata.region_stability_facts.iter().any(|fact| {
        fact.fact_id == fact_id
            && fact.stable_in_region
            && resolve_value_origin(function, &def_map, fact.region_value) == receiver_origin
    })
}

fn integer_const_value(function: &MirFunction, value_id: ValueId) -> Option<i64> {
    let def_map = build_value_def_map(function);
    let origin = resolve_value_origin(function, &def_map, value_id);
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } if resolve_value_origin(function, &def_map, *dst) == origin => Some(*actual),
                _ => None,
            })
    })
}

fn checked_direct_array_lowering_site_is_cfg_safe(
    function: &MirFunction,
    block_id: BasicBlockId,
) -> bool {
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    // The current checked DirectArray C lowering emits a local success/fail
    // branch and rejoins before the original block terminator.  That changes
    // the concrete predecessor label seen by successor PHIs, so v0 only marks
    // checked plans for blocks whose direct successors do not begin with PHI.
    // Proved-unchecked plans are branchless and are handled in a later slice.
    block.successors_from_terminator().iter().all(|successor| {
        function
            .blocks
            .get(successor)
            .map(|successor_block| {
                !matches!(
                    successor_block.instructions.first(),
                    Some(MirInstruction::Phi { .. })
                )
            })
            .unwrap_or(false)
    })
}

fn array_store_value_arg(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    key_value: ValueId,
) -> Option<ValueId> {
    let block = function.blocks.get(&block)?;
    let def_map = build_value_def_map(function);
    let key_origin = resolve_value_origin(function, &def_map, key_value);
    match block.instructions.get(instruction_index)? {
        MirInstruction::Call { args, .. } => args
            .iter()
            .position(|arg| resolve_value_origin(function, &def_map, *arg) == key_origin)
            .and_then(|index| args.get(index + 1).copied()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::function::{DirectArrayExtentProofKind, LoopRangeFact};
    use crate::mir::range_index_fact::refresh_function_range_index_facts;
    use crate::mir::{
        BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType,
    };

    fn make_function() -> MirFunction {
        make_named_function("main", vec![])
    }

    fn make_named_function(name: &str, params: Vec<MirType>) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params,
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn refresh_records_checked_load_and_store_plans_from_array_routes() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(0),
        });
        block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
        block.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![1, 3]));

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_access_plans(&mut function);

        assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
        let load = &function.metadata.direct_array_access_plans[0];
        assert_eq!(load.op(), DirectArrayAccessOp::Load);
        assert_eq!(load.block(), BasicBlockId::new(0));
        assert_eq!(load.instruction_index(), 1);
        assert_eq!(load.receiver_value(), ValueId::new(2));
        assert_eq!(load.index_value(), ValueId::new(1));
        assert_eq!(load.value_value(), None);
        assert_eq!(load.result_value(), Some(ValueId::new(5)));
        assert_eq!(load.bounds_policy(), DirectArrayBoundsPolicy::Checked);
        assert_eq!(load.proof_kind(), DirectArrayProofKind::ExactFrontContract);
        assert_eq!(load.proof_ids(), &["exact_front_contract"]);
        assert_eq!(
            load.fallback_policy(),
            DirectArrayFallbackPolicy::AllowChecked
        );
        assert_eq!(load.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
        assert_eq!(load.store_semantics(), DirectArrayStoreSemantics::NotStore);

        let store = &function.metadata.direct_array_access_plans[1];
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(store.instruction_index(), 2);
        assert_eq!(store.receiver_value(), ValueId::new(2));
        assert_eq!(store.index_value(), ValueId::new(1));
        assert_eq!(store.value_value(), Some(ValueId::new(3)));
        assert_eq!(store.result_value(), Some(ValueId::new(6)));
        assert_eq!(store.route(), "direct_array_i64_store");
        assert_eq!(store.proof_ids(), &["exact_front_contract"]);
        assert_eq!(store.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
        assert_eq!(
            store.store_semantics(),
            DirectArrayStoreSemantics::AppendOrOverwrite
        );
    }

    #[test]
    fn refresh_records_range_index_store_as_branchless_proved_unchecked_plan() {
        let mut function = make_function();
        let body_bb = BasicBlockId::new(1);
        function.add_block(BasicBlock::new(body_bb));
        let entry = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        let body = function.blocks.get_mut(&body_bb).expect("body");
        body.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![4, 3]));
        function.metadata.loop_range_facts.push(LoopRangeFact {
            index_name: "i".to_string(),
            start_value: ValueId::new(10),
            end_value: ValueId::new(11),
            index_phi: ValueId::new(4),
            preheader_bb: BasicBlockId::new(0),
            header_bb: BasicBlockId::new(2),
            body_bb,
            step_bb: BasicBlockId::new(3),
            exit_bb: BasicBlockId::new(4),
            step: 1,
            end_exclusive: true,
            index_read_only: true,
            body_local_writes_supported: true,
            loop_carried_writes_supported: false,
            body_writes_supported: false,
        });

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_range_index_facts(&mut function);
        function
            .metadata
            .region_stability_facts
            .push(crate::mir::function::RegionStabilityFact {
                fact_id: 0,
                region_value: ValueId::new(2),
                scope_bb: body_bb,
                proof_kind: crate::mir::function::RegionStabilityProofKind::ProducerInvariant,
                stable_in_region: true,
            });
        function.metadata.direct_array_extent_facts.push(
            crate::mir::function::DirectArrayExtentFact {
                receiver_value: ValueId::new(2),
                lower_bound_value: ValueId::new(11),
                proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
                region_stability_fact_id: 0,
                stable_in_region: true,
            },
        );
        refresh_function_direct_array_access_plans(&mut function);

        assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
        let store = &function.metadata.direct_array_access_plans[0];
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(store.block(), body_bb);
        assert_eq!(store.instruction_index(), 0);
        assert_eq!(store.index_value(), ValueId::new(4));
        assert_eq!(
            store.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(store.proof_kind(), DirectArrayProofKind::RangeIndex);
        assert_eq!(store.proof_ids(), &["range_index"]);
        assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
        assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
        assert_eq!(
            store.store_semantics(),
            DirectArrayStoreSemantics::AppendOrOverwrite
        );
    }

    #[test]
    fn refresh_keeps_range_index_store_checked_without_extent_proof() {
        let mut function = make_function();
        let body_bb = BasicBlockId::new(1);
        function.add_block(BasicBlock::new(body_bb));
        let entry = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        let body = function.blocks.get_mut(&body_bb).expect("body");
        body.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![4, 3]));
        function.metadata.loop_range_facts.push(LoopRangeFact {
            index_name: "i".to_string(),
            start_value: ValueId::new(10),
            end_value: ValueId::new(11),
            index_phi: ValueId::new(4),
            preheader_bb: BasicBlockId::new(0),
            header_bb: BasicBlockId::new(2),
            body_bb,
            step_bb: BasicBlockId::new(3),
            exit_bb: BasicBlockId::new(4),
            step: 1,
            end_exclusive: true,
            index_read_only: true,
            body_local_writes_supported: true,
            loop_carried_writes_supported: false,
            body_writes_supported: false,
        });

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_access_plans(&mut function);

        assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
        let store = &function.metadata.direct_array_access_plans[0];
        assert_eq!(store.bounds_policy(), DirectArrayBoundsPolicy::Checked);
        assert_eq!(store.proof_kind(), DirectArrayProofKind::ExactFrontContract);
        assert_eq!(store.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
    }

    #[test]
    fn refresh_records_stack_top_pop_load_and_store_as_branchless_proved_unchecked_plans() {
        let mut function = make_function();
        let body_bb = BasicBlockId::new(1);
        let reject_bb = BasicBlockId::new(2);
        function.add_block(BasicBlock::new(body_bb));
        function.add_block(BasicBlock::new(reject_bb));

        let entry = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        entry.add_instruction(MirInstruction::Compare {
            dst: ValueId::new(11),
            op: CompareOp::Eq,
            lhs: ValueId::new(2),
            rhs: ValueId::new(10),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(11),
            then_bb: reject_bb,
            else_bb: body_bb,
            then_edge_args: None,
            else_edge_args: None,
        });

        let body = function.blocks.get_mut(&body_bb).expect("body");
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(1),
        });
        body.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(13),
            op: BinaryOp::Sub,
            lhs: ValueId::new(2),
            rhs: ValueId::new(12),
        });
        body.add_instruction(method_call(Some(14), "ArrayBox", "get", 3, vec![13]));
        body.add_instruction(method_call(Some(15), "ArrayBox", "set", 4, vec![14, 5]));

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_access_plans(&mut function);

        assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
        let load = &function.metadata.direct_array_access_plans[0];
        assert_eq!(load.op(), DirectArrayAccessOp::Load);
        assert_eq!(
            load.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
        assert_eq!(load.proof_ids(), &["stack_top_pop"]);
        assert_eq!(load.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
        assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
        assert_eq!(load.store_semantics(), DirectArrayStoreSemantics::NotStore);

        let store = &function.metadata.direct_array_access_plans[1];
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(
            store.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(store.proof_kind(), DirectArrayProofKind::StackTopPop);
        assert_eq!(store.proof_ids(), &["stack_top_pop"]);
        assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
        assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
        assert_eq!(
            store.store_semantics(),
            DirectArrayStoreSemantics::OverwriteExisting
        );
    }

    #[test]
    fn refresh_records_release_known_live_stores_as_caller_precondition_plans() {
        let mut function = make_named_function(
            "HakoAllocPageModel.releaseLocalKnownLive/1",
            vec![
                MirType::Box("HakoAllocPageModel".to_string()),
                MirType::Integer,
            ],
        );
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(3),
            base: ValueId::new(0),
            field: "block_used".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        block.add_instruction(method_call(Some(6), "ArrayBox", "set", 3, vec![1, 10]));
        block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(11),
            base: ValueId::new(0),
            field: "local_free_top".to_string(),
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(12),
            base: ValueId::new(0),
            field: "local_free".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        block.add_instruction(method_call(Some(15), "ArrayBox", "set", 12, vec![11, 1]));

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_access_plans(&mut function);

        assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
        for store in &function.metadata.direct_array_access_plans {
            assert_eq!(store.op(), DirectArrayAccessOp::Store);
            assert_eq!(
                store.bounds_policy(),
                DirectArrayBoundsPolicy::ProvedUnchecked
            );
            assert_eq!(store.proof_kind(), DirectArrayProofKind::CallerPrecondition);
            assert_eq!(store.proof_ids(), &["caller_precondition"]);
            assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
            assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
            assert_eq!(
                store.store_semantics(),
                DirectArrayStoreSemantics::OverwriteExisting
            );
        }
    }
}
