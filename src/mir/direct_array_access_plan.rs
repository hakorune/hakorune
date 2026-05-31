/*!
 * MIR-owned DirectArray access plans.
 *
 * This is the first behavior-preserving seam that lifts exact ArrayBox
 * get/set candidates out of backend string/method-name recognition.  The v0
 * plan is metadata-only: it records checked DirectArrayI64 candidates derived
 * from `generic_method_routes`, and later lowering slices may consume it.
 */

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

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
    LoopRange,
    StackTopPop,
    CallerPrecondition,
}

impl DirectArrayProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactFrontContract => "exact_front_contract",
            Self::LoopRange => "loop_range",
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
}

impl DirectArrayAccessPlan {
    fn new(
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
}

pub fn refresh_function_direct_array_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("ArrayBox") {
            continue;
        }
        if !checked_direct_array_lowering_site_is_cfg_safe(function, route.block()) {
            continue;
        }
        let Some(index_value) = route.key_value() else {
            continue;
        };
        match route.route_kind_tag() {
            "array_slot_load_any" => plans.push(DirectArrayAccessPlan::new(
                route.block(),
                route.instruction_index(),
                DirectArrayAccessOp::Load,
                route.receiver_value(),
                index_value,
                None,
                route.result_value(),
            )),
            "array_store_any" => {
                let Some(value_value) =
                    call_arg(function, route.block(), route.instruction_index(), 1)
                else {
                    continue;
                };
                plans.push(DirectArrayAccessPlan::new(
                    route.block(),
                    route.instruction_index(),
                    DirectArrayAccessOp::Store,
                    route.receiver_value(),
                    index_value,
                    Some(value_value),
                    route.result_value(),
                ));
            }
            _ => {}
        }
    }
    function.metadata.direct_array_access_plans = plans;
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
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
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

        let store = &function.metadata.direct_array_access_plans[1];
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(store.instruction_index(), 2);
        assert_eq!(store.receiver_value(), ValueId::new(2));
        assert_eq!(store.index_value(), ValueId::new(1));
        assert_eq!(store.value_value(), Some(ValueId::new(3)));
        assert_eq!(store.result_value(), Some(ValueId::new(6)));
        assert_eq!(store.route(), "direct_array_i64_store");
    }
}
