/*!
 * MIR-owned DirectArray access plans.
 *
 * This is the first behavior-preserving seam that lifts exact ArrayBox
 * get/set candidates out of backend string/method-name recognition.  The v0
 * plan is metadata-only: it records checked DirectArrayI64 candidates derived
 * from `generic_method_routes`, and later lowering slices may consume it.
 */

use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, ValueId};

const DIRECT_ARRAY_I64_DEFAULT_CAPACITY_V0: i64 = 64;

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
            fallback_policy: DirectArrayFallbackPolicy::FailFast,
            cfg_shape: DirectArrayCfgShape::Branchless,
            // RangeIndex v0 proves a sequential 0..end fill. The branchless
            // lowerer preserves Array.set append-or-overwrite semantics by
            // updating len to max(len, index + 1), so this is not the legacy
            // raw overwrite-only unchecked store.
            store_semantics: DirectArrayStoreSemantics::AppendOrOverwrite,
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
    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("ArrayBox") {
            continue;
        }
        let Some(index_value) = route.key_value() else {
            continue;
        };
        match route.route_kind_tag() {
            "array_slot_load_any" => {
                if !checked_direct_array_lowering_site_is_cfg_safe(function, route.block()) {
                    continue;
                }
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
            "array_store_any" => {
                let Some(value_value) =
                    call_arg(function, route.block(), route.instruction_index(), 1)
                else {
                    continue;
                };
                if range_index_proves_branchless_append_or_overwrite_store(
                    function,
                    route.block(),
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

fn range_index_proves_branchless_append_or_overwrite_store(
    function: &MirFunction,
    block_id: BasicBlockId,
    index_value: ValueId,
) -> bool {
    function.metadata.range_index_facts.iter().any(|fact| {
        fact.body_bb == block_id
            && fact.index_value == index_value
            && fact.step == 1
            && fact.end_exclusive
            && fact.index_body_read_only
            && !fact.loop_carried_writes_supported
            && value_is_integer_const(function, fact.lower_value, 0)
            && direct_array_extent_v0_proves_upper_bound(function, fact.upper_exclusive_value)
    })
}

fn value_is_integer_const(function: &MirFunction, value_id: ValueId, expected: i64) -> bool {
    integer_const_value(function, value_id)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn direct_array_extent_v0_proves_upper_bound(function: &MirFunction, end_value: ValueId) -> bool {
    // Until DirectArrayExtentFact exists, v0 only accepts constant loop upper
    // bounds that fit the DirectArrayI64 birth capacity used by the exact
    // front. Dynamic `capacity` bounds stay on the checked path.
    integer_const_value(function, end_value)
        .map(|upper| (0..=DIRECT_ARRAY_I64_DEFAULT_CAPACITY_V0).contains(&upper))
        .unwrap_or(false)
}

fn integer_const_value(function: &MirFunction, value_id: ValueId) -> Option<i64> {
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } if *dst == value_id => Some(*actual),
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

fn call_arg(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    arg_index: usize,
) -> Option<ValueId> {
    let block = function.blocks.get(&block)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Call { args, .. } => args.get(arg_index).copied(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::function::LoopRangeFact;
    use crate::mir::range_index_fact::refresh_function_range_index_facts;
    use crate::mir::{
        BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType,
    };

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
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
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(3),
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
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(store.block(), body_bb);
        assert_eq!(store.instruction_index(), 0);
        assert_eq!(store.index_value(), ValueId::new(4));
        assert_eq!(
            store.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(store.proof_kind(), DirectArrayProofKind::RangeIndex);
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
}
