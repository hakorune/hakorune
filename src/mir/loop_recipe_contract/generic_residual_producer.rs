//! Caller-zero Generic residual Recipe producer.
//!
//! This profile consumes the sealed policy demand, emits the existing
//! portable Recipe envelope for the bounded single-induction
//! integer-progression source profile, verifies it, and delegates
//! logical elaboration to the one shared JoinSig owner. It does not
//! inspect syntax, mint `GenericLoopV0`/`GenericG0` provenance, or
//! allocate physical IDs.

use std::collections::BTreeMap;

use crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1;
use crate::mir::compiler::generic_residual_typed_map::{
    GenericResidualBodyRowV1, GenericResidualBodyStepV1, GenericResidualBoundV1,
    GenericResidualTypedCompareV1,
};
use crate::mir::loop_route_policy::VerifiedGenericResidualPolicyDemandV1;
use crate::mir::resolved_semantics::BindingRefV1;

use super::error::LoopRecipeRejectReasonV1;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};
use super::join_sig::{LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, VerifiedLoopJoinSigV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopNodeV1, LoopOperationV1,
    LoopRecipeArtifactV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1,
};
use super::verify::{verify_source_bound_recipe_v1, LoopRecipeVerifierV1, VerifiedLoopRecipeV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericResidualRecipeProducerRejectV1 {
    PolicyFrameMismatch,
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedGenericResidualRecipeProductV1 {
    policy_receipt: crate::mir::loop_route_policy::VerifiedGenericResidualPolicyReceiptV1,
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedGenericResidualRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn policy_receipt(
        &self,
    ) -> &crate::mir::loop_route_policy::VerifiedGenericResidualPolicyReceiptV1 {
        &self.policy_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::mir::loop_route_policy::VerifiedGenericResidualPolicyReceiptV1,
        VerifiedLoopRecipeV1,
        VerifiedLoopJoinSigV1,
    ) {
        (self.policy_receipt, self.recipe, self.join_sig)
    }
}

pub(crate) fn produce_generic_residual_recipe_v1(
    demand: VerifiedGenericResidualPolicyDemandV1,
) -> Result<VerifiedGenericResidualRecipeProductV1, GenericResidualRecipeProducerRejectV1> {
    let (policy_receipt, map) = demand.into_parts();
    let (source_root, projection, carrier, condition, body_rows, carrier_step, _frame) =
        map.into_parts();
    if !policy_receipt
        .frame_key()
        .matches(projection.root_frame_key())
    {
        return Err(GenericResidualRecipeProducerRejectV1::PolicyFrameMismatch);
    }
    let recipe = generic_residual_recipe(carrier, &condition, &body_rows, &carrier_step);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(GenericResidualRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::GenericResidualV1),
        source_binding,
        recipe,
    );
    let verified_recipe = verify_source_bound_recipe_v1(artifact)
        .map_err(GenericResidualRecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(GenericResidualRecipeProducerRejectV1::JoinSig)?;
    Ok(VerifiedGenericResidualRecipeProductV1 {
        policy_receipt,
        recipe: verified_recipe,
        join_sig,
    })
}

fn compare_op(operator: SyntaxBinaryOperatorV1) -> LoopCompareI64OpV1 {
    match operator {
        SyntaxBinaryOperatorV1::Less => LoopCompareI64OpV1::Less,
        SyntaxBinaryOperatorV1::LessEqual => LoopCompareI64OpV1::LessEqual,
        SyntaxBinaryOperatorV1::Equal => LoopCompareI64OpV1::Equal,
        // The typed map seals only the compare vocabulary; any other
        // operator cannot reach this producer.
        _ => unreachable!("typed Generic residual compare operator outside the sealed vocabulary"),
    }
}

fn binary_op(operator: SyntaxBinaryOperatorV1) -> LoopBinaryI64OpV1 {
    match operator {
        SyntaxBinaryOperatorV1::Add => LoopBinaryI64OpV1::Add,
        SyntaxBinaryOperatorV1::Subtract => LoopBinaryI64OpV1::Sub,
        // The typed map seals only Add/Subtract for steps.
        _ => unreachable!("typed Generic residual step operator outside the sealed vocabulary"),
    }
}

/// Canonical key emitter for the bounded Generic residual Recipe. It
/// assigns dense value/item/carrier keys in emission order so the
/// produced envelope is deterministic.
struct RecipeEmitterV1 {
    values: Vec<LoopRecipeValueV1>,
    items: Vec<LoopRecipeItemRowV1>,
    inputs: Vec<LoopValueKeyV1>,
    carriers: Vec<LoopRecipeCarrierV1>,
    next_value: u32,
    next_item: u32,
    next_carrier: u32,
}

impl RecipeEmitterV1 {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            items: Vec::new(),
            inputs: Vec::new(),
            carriers: Vec::new(),
            next_value: 0,
            next_item: 0,
            next_carrier: 0,
        }
    }

    fn alloc_value(&mut self, class: LoopValueClassV1) -> LoopValueKeyV1 {
        let key = LoopValueKeyV1::new(self.next_value);
        self.next_value += 1;
        self.values.push(LoopRecipeValueV1 { key, class });
        key
    }

    fn alloc_input(&mut self) -> LoopValueKeyV1 {
        let key = self.alloc_value(LoopValueClassV1::I64);
        self.inputs.push(key);
        key
    }

    fn push_carrier(
        &mut self,
        owner_loop: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        entry_value: LoopValueKeyV1,
    ) {
        self.carriers.push(LoopRecipeCarrierV1 {
            key: LoopCarrierKeyV1::new(self.next_carrier),
            owner_loop,
            binding,
            class: LoopValueClassV1::I64,
            entry_value,
        });
        self.next_carrier += 1;
    }

    fn push_item(
        &mut self,
        block_items: &mut Vec<LoopItemKeyV1>,
        operation: LoopOperationV1,
    ) -> LoopItemKeyV1 {
        let key = LoopItemKeyV1::new(self.next_item);
        self.next_item += 1;
        self.items.push(LoopRecipeItemRowV1 {
            key,
            item: LoopRecipeItemV1::Operation { operation },
        });
        block_items.push(key);
        key
    }

    fn push_const(&mut self, block_items: &mut Vec<LoopItemKeyV1>, literal: i64) -> LoopValueKeyV1 {
        let result = self.alloc_value(LoopValueClassV1::I64);
        self.push_item(
            block_items,
            LoopOperationV1::ConstI64 {
                result,
                value: literal,
            },
        );
        result
    }

    fn push_read(
        &mut self,
        block_items: &mut Vec<LoopItemKeyV1>,
        binding: LoopBindingKeyV1,
    ) -> LoopValueKeyV1 {
        let result = self.alloc_value(LoopValueClassV1::I64);
        self.push_item(
            block_items,
            LoopOperationV1::ReadBinding { binding, result },
        );
        result
    }

    fn push_binary(
        &mut self,
        block_items: &mut Vec<LoopItemKeyV1>,
        op: SyntaxBinaryOperatorV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
    ) -> LoopValueKeyV1 {
        let result = self.alloc_value(LoopValueClassV1::I64);
        self.push_item(
            block_items,
            LoopOperationV1::BinaryI64 {
                op: binary_op(op),
                left,
                right,
                result,
            },
        );
        result
    }
}

fn generic_residual_recipe(
    carrier: BindingRefV1,
    condition: &GenericResidualTypedCompareV1,
    body_rows: &[GenericResidualBodyRowV1],
    carrier_step: &GenericResidualBodyStepV1,
) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);

    let mut emitter = RecipeEmitterV1::new();

    // Boundary-visible bindings: the induction carrier and, when the
    // bound is a different variable, the read-only bound binding. Both
    // are loop carriers whose entry value is a Recipe input. Body-local
    // declarations are per-iteration SSA values, not boundary slots —
    // they never become bindings, carriers, or fabricated inputs.
    let carrier_key = LoopBindingKeyV1::new(0);
    let mut bindings = vec![LoopRecipeBindingV1 {
        key: carrier_key,
        label: "generic_residual_carrier".into(),
        class: LoopValueClassV1::I64,
    }];
    let carrier_entry = emitter.alloc_input();
    emitter.push_carrier(loop_key, carrier_key, carrier_entry);
    let bound_key = match &condition.bound {
        GenericResidualBoundV1::Binding { binding, .. } if *binding != carrier => {
            let key = LoopBindingKeyV1::new(1);
            bindings.push(LoopRecipeBindingV1 {
                key,
                label: "generic_residual_bound".into(),
                class: LoopValueClassV1::I64,
            });
            let bound_entry = emitter.alloc_input();
            emitter.push_carrier(loop_key, key, bound_entry);
            Some(key)
        }
        _ => None,
    };

    // Condition block: carrier read + bound (const or bound read) + compare.
    let mut condition_items = Vec::new();
    let cond_read = emitter.push_read(&mut condition_items, carrier_key);
    let cond_bound = match &condition.bound {
        GenericResidualBoundV1::Integer { value, .. } => {
            emitter.push_const(&mut condition_items, *value)
        }
        GenericResidualBoundV1::Binding { binding, .. } if *binding == carrier => cond_read,
        GenericResidualBoundV1::Binding { .. } => {
            emitter.push_read(&mut condition_items, bound_key.expect("bound binding key"))
        }
    };
    let cond_predicate = emitter.alloc_value(LoopValueClassV1::Bool);
    emitter.push_item(
        &mut condition_items,
        LoopOperationV1::CompareI64 {
            op: compare_op(condition.operator),
            left: cond_read,
            right: cond_bound,
            result: cond_predicate,
        },
    );

    // Body rows in source order. A body-local declaration is a fresh
    // per-iteration SSA value; a body-local rebind is an SSA rename —
    // neither touches a boundary binding. The terminal carrier step is
    // the only boundary write.
    let mut body_items = Vec::new();
    let mut ssa: BTreeMap<BindingRefV1, LoopValueKeyV1> = BTreeMap::new();
    for row in body_rows {
        match row {
            GenericResidualBodyRowV1::Declaration(decl) => {
                let value = emitter.push_const(&mut body_items, decl.literal);
                ssa.insert(decl.binding, value);
            }
            GenericResidualBodyRowV1::Rebind(step) => {
                let current = ssa
                    .get(&step.binding)
                    .copied()
                    .expect("typed map seals rebinds after their in-body declaration");
                let delta = emitter.push_const(&mut body_items, step.delta);
                let stepped =
                    emitter.push_binary(&mut body_items, step.operator, current, delta);
                ssa.insert(step.binding, stepped);
            }
        }
    }
    let carrier_read = emitter.push_read(&mut body_items, carrier_key);
    let carrier_delta = emitter.push_const(&mut body_items, carrier_step.delta);
    let carrier_next = emitter.push_binary(
        &mut body_items,
        carrier_step.operator,
        carrier_read,
        carrier_delta,
    );
    emitter.push_item(
        &mut body_items,
        LoopOperationV1::WriteBinding {
            binding: carrier_key,
            value: carrier_next,
        },
    );

    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: cond_predicate,
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: condition_items,
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: body_items,
            },
        ],
        items: emitter.items,
        bindings,
        values: emitter.values,
        inputs: emitter.inputs,
        carriers: emitter.carriers,
        exits: Vec::new(),
    }
}
