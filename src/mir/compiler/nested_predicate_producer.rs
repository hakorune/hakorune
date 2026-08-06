//! Caller-zero producer for the bounded NestedLoopMinimal Recipe.
//!
//! This module consumes the sealed source projection and emits only portable
//! semantic products. Syntax, route selection, and physical ownership stay
//! outside this boundary.

use crate::mir::loop_recipe_contract::{
    verify_source_bound_recipe_v1, LoopBinaryI64OpV1, LoopBindingKeyV1, LoopBlockKeyV1,
    LoopCarrierKeyV1, LoopCompareI64OpV1, LoopConditionV1, LoopItemKeyV1, LoopJoinSigElaboratorV1,
    LoopJoinSigRejectReasonV1, LoopNodeKeyV1, LoopRecipeArtifactV1, LoopRecipeBindingV1,
    LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeItemRowV1, LoopRecipeItemV1,
    LoopRecipeProducerIdV1, LoopRecipeProvenanceV1, LoopRecipeRejectReasonV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1, LoopValueKeyV1,
};
use crate::mir::loop_structural_facts::LoopSourceForestBindingRejectV1;

use super::nested_predicate_projection::{
    NestedObservedRecurrenceOwnerV1, VerifiedNestedLoopSourceProjectionV1,
    VerifiedNestedLoopSourceShapeV1,
};
use super::nested_predicate_source_handoff::{
    NestedPhysicalSourceHandoffRejectV1, VerifiedNestedPhysicalSourceHandoffV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NestedPredicateRecipeProducerRejectV1 {
    RootInitializerValue { index: u32, value: i64 },
    ChildInitializerValue { value: i64 },
    BindingRoleMismatch,
    SourceBinding(LoopSourceForestBindingRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    SourceHandoff(NestedPhysicalSourceHandoffRejectV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNestedPredicateRecipeProductV1 {
    recipe: crate::mir::loop_recipe_contract::VerifiedLoopRecipeV1,
    join_sig: crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV1,
    source_handoff: VerifiedNestedPhysicalSourceHandoffV1,
}

impl VerifiedNestedPredicateRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &crate::mir::loop_recipe_contract::VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn source_handoff(&self) -> &VerifiedNestedPhysicalSourceHandoffV1 {
        &self.source_handoff
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::mir::loop_recipe_contract::VerifiedLoopRecipeV1,
        crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV1,
    ) {
        (self.recipe, self.join_sig)
    }

    /// Consumes the semantic product and splits the one-time source handoff
    /// for the caller-zero physical-topology issuer.
    pub(crate) fn into_topology_input(
        self,
    ) -> (
        crate::mir::loop_recipe_contract::VerifiedLoopRecipeV1,
        crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV1,
        VerifiedNestedPhysicalSourceHandoffV1,
    ) {
        (self.recipe, self.join_sig, self.source_handoff)
    }
}

pub(crate) fn produce_nested_predicate_recipe_v1(
    projection: VerifiedNestedLoopSourceProjectionV1,
) -> Result<VerifiedNestedPredicateRecipeProductV1, NestedPredicateRecipeProducerRejectV1> {
    let (forest_binding, shape, root_frame_key) = projection.into_parts();
    validate_shape(&shape)?;
    let source_handoff =
        VerifiedNestedPhysicalSourceHandoffV1::issue(&forest_binding, shape, root_frame_key)
            .map_err(NestedPredicateRecipeProducerRejectV1::SourceHandoff)?;
    let shape = source_handoff.shape();
    let recipe = nested_recipe(&shape);
    let verified_for_source =
        crate::mir::loop_recipe_contract::LoopRecipeVerifierV1::verify(recipe.clone())
            .map_err(NestedPredicateRecipeProducerRejectV1::Recipe)?;
    let source_binding = forest_binding
        .into_source_binding(&verified_for_source)
        .map_err(NestedPredicateRecipeProducerRejectV1::SourceBinding)?;
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::NestedPredicateV1),
        source_binding,
        recipe,
    );
    let verified_recipe = verify_source_bound_recipe_v1(artifact)
        .map_err(NestedPredicateRecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(NestedPredicateRecipeProducerRejectV1::JoinSig)?;
    Ok(VerifiedNestedPredicateRecipeProductV1 {
        recipe: verified_recipe,
        join_sig,
        source_handoff,
    })
}

fn validate_shape(
    shape: &VerifiedNestedLoopSourceShapeV1,
) -> Result<(), NestedPredicateRecipeProducerRejectV1> {
    for (index, initializer) in shape.root_initializers.iter().enumerate() {
        if initializer.value != 0 {
            return Err(
                NestedPredicateRecipeProducerRejectV1::RootInitializerValue {
                    index: index as u32,
                    value: initializer.value,
                },
            );
        }
    }
    if shape.initialize_child.delta != 0 {
        return Err(
            NestedPredicateRecipeProducerRejectV1::ChildInitializerValue {
                value: shape.initialize_child.delta,
            },
        );
    }
    if shape.bindings[0].recurrence_owner != NestedObservedRecurrenceOwnerV1::Root
        || shape.bindings[1].recurrence_owner != NestedObservedRecurrenceOwnerV1::Root
        || shape.bindings[2].recurrence_owner != NestedObservedRecurrenceOwnerV1::Child
        || !shape.bindings[0].parent_visible
        || !shape.bindings[1].parent_visible
        || shape.bindings[2].parent_visible
    {
        return Err(NestedPredicateRecipeProducerRejectV1::BindingRoleMismatch);
    }
    Ok(())
}

fn nested_recipe(shape: &VerifiedNestedLoopSourceShapeV1) -> LoopRecipeV1 {
    let loop_root = LoopNodeKeyV1::new(0);
    let loop_child = LoopNodeKeyV1::new(1);
    let block_root_condition = LoopBlockKeyV1::new(0);
    let block_root_body = LoopBlockKeyV1::new(1);
    let block_child_condition = LoopBlockKeyV1::new(2);
    let block_child_body = LoopBlockKeyV1::new(3);
    let binding_root = LoopBindingKeyV1::new(0);
    let binding_ancestor = LoopBindingKeyV1::new(1);
    let binding_child = LoopBindingKeyV1::new(2);
    let value_root_input = LoopValueKeyV1::new(0);
    let value_root_read = LoopValueKeyV1::new(1);
    let value_root_bound = LoopValueKeyV1::new(2);
    let value_ancestor_input = LoopValueKeyV1::new(3);
    let value_root_predicate = LoopValueKeyV1::new(4);
    let value_child_init = LoopValueKeyV1::new(5);
    let value_child_read = LoopValueKeyV1::new(6);
    let value_child_predicate = LoopValueKeyV1::new(7);
    let value_child_bound = LoopValueKeyV1::new(8);
    let value_ancestor_read = LoopValueKeyV1::new(9);
    let value_ancestor_delta = LoopValueKeyV1::new(10);
    let value_ancestor_next = LoopValueKeyV1::new(11);
    let value_child_update_read = LoopValueKeyV1::new(12);
    let value_child_delta = LoopValueKeyV1::new(13);
    let value_child_next = LoopValueKeyV1::new(14);
    let value_root_update_read = LoopValueKeyV1::new(15);
    let value_root_delta = LoopValueKeyV1::new(16);
    let value_root_next = LoopValueKeyV1::new(17);
    let item = |key: u32, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item,
    };
    let operation = |operation| LoopRecipeItemV1::Operation { operation };
    let values = (0..=17)
        .map(|raw| LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(raw),
            class: if raw == value_root_predicate.raw() || raw == value_child_predicate.raw() {
                LoopValueClassV1::Bool
            } else {
                LoopValueClassV1::I64
            },
        })
        .collect();
    LoopRecipeV1 {
        root_loop: loop_root,
        loops: vec![
            crate::mir::loop_recipe_contract::LoopNodeV1 {
                key: loop_root,
                parent: None,
                condition: LoopConditionV1::Predicate {
                    block: block_root_condition,
                    value: value_root_predicate,
                },
                body: block_root_body,
            },
            crate::mir::loop_recipe_contract::LoopNodeV1 {
                key: loop_child,
                parent: Some(loop_root),
                condition: LoopConditionV1::Predicate {
                    block: block_child_condition,
                    value: value_child_predicate,
                },
                body: block_child_body,
            },
        ],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: block_root_condition,
                owner_loop: loop_root,
                items: vec![
                    LoopItemKeyV1::new(0),
                    LoopItemKeyV1::new(1),
                    LoopItemKeyV1::new(2),
                ],
            },
            LoopRecipeBlockV1 {
                key: block_root_body,
                owner_loop: loop_root,
                items: vec![
                    LoopItemKeyV1::new(3),
                    LoopItemKeyV1::new(4),
                    LoopItemKeyV1::new(16),
                    LoopItemKeyV1::new(17),
                    LoopItemKeyV1::new(18),
                    LoopItemKeyV1::new(19),
                ],
            },
            LoopRecipeBlockV1 {
                key: block_child_condition,
                owner_loop: loop_child,
                items: vec![
                    LoopItemKeyV1::new(5),
                    LoopItemKeyV1::new(6),
                    LoopItemKeyV1::new(7),
                ],
            },
            LoopRecipeBlockV1 {
                key: block_child_body,
                owner_loop: loop_child,
                items: (8..=15).map(LoopItemKeyV1::new).collect(),
            },
        ],
        items: vec![
            item(
                0,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding: binding_root,
                        result: value_root_read,
                    },
                ),
            ),
            item(
                1,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_root_bound,
                        value: shape.root_condition.bound,
                    },
                ),
            ),
            item(
                2,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Less,
                        left: value_root_read,
                        right: value_root_bound,
                        result: value_root_predicate,
                    },
                ),
            ),
            item(
                3,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_child_init,
                        value: shape.initialize_child.delta,
                    },
                ),
            ),
            item(
                4,
                LoopRecipeItemV1::Loop {
                    loop_key: loop_child,
                },
            ),
            item(
                5,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding: binding_child,
                        result: value_child_read,
                    },
                ),
            ),
            item(
                6,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_child_bound,
                        value: shape.child_condition.bound,
                    },
                ),
            ),
            item(
                7,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Less,
                        left: value_child_read,
                        right: value_child_bound,
                        result: value_child_predicate,
                    },
                ),
            ),
            item(
                8,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding: binding_ancestor,
                        result: value_ancestor_read,
                    },
                ),
            ),
            item(
                9,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_ancestor_delta,
                        value: shape.increment_ancestor.delta,
                    },
                ),
            ),
            item(
                10,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: value_ancestor_read,
                        right: value_ancestor_delta,
                        result: value_ancestor_next,
                    },
                ),
            ),
            item(
                11,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::WriteBinding {
                        binding: binding_ancestor,
                        value: value_ancestor_next,
                    },
                ),
            ),
            item(
                12,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding: binding_child,
                        result: value_child_update_read,
                    },
                ),
            ),
            item(
                13,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_child_delta,
                        value: shape.increment_child.delta,
                    },
                ),
            ),
            item(
                14,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: value_child_update_read,
                        right: value_child_delta,
                        result: value_child_next,
                    },
                ),
            ),
            item(
                15,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::WriteBinding {
                        binding: binding_child,
                        value: value_child_next,
                    },
                ),
            ),
            item(
                16,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding: binding_root,
                        result: value_root_update_read,
                    },
                ),
            ),
            item(
                17,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: value_root_delta,
                        value: shape.increment_root.delta,
                    },
                ),
            ),
            item(
                18,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: value_root_update_read,
                        right: value_root_delta,
                        result: value_root_next,
                    },
                ),
            ),
            item(
                19,
                operation(
                    crate::mir::loop_recipe_contract::LoopOperationV1::WriteBinding {
                        binding: binding_root,
                        value: value_root_next,
                    },
                ),
            ),
        ],
        bindings: vec![
            LoopRecipeBindingV1 {
                key: binding_root,
                label: "root_0".into(),
                class: LoopValueClassV1::I64,
            },
            LoopRecipeBindingV1 {
                key: binding_ancestor,
                label: "root_1".into(),
                class: LoopValueClassV1::I64,
            },
            LoopRecipeBindingV1 {
                key: binding_child,
                label: "child_0".into(),
                class: LoopValueClassV1::I64,
            },
        ],
        values,
        inputs: vec![value_root_input, value_ancestor_input],
        carriers: vec![
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(0),
                owner_loop: loop_root,
                binding: binding_root,
                class: LoopValueClassV1::I64,
                entry_value: value_root_input,
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(1),
                owner_loop: loop_root,
                binding: binding_ancestor,
                class: LoopValueClassV1::I64,
                entry_value: value_ancestor_input,
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(2),
                owner_loop: loop_child,
                binding: binding_child,
                class: LoopValueClassV1::I64,
                entry_value: value_child_init,
            },
        ],
        exits: Vec::new(),
    }
}
