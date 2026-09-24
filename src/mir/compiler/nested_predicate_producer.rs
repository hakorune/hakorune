//! Caller-zero producer for the bounded NestedLoopMinimal Recipe.
//!
//! This module consumes the sealed source projection and emits only portable
//! semantic products. Syntax, route selection, and physical ownership stay
//! outside this boundary.

use crate::mir::loop_recipe_contract::{
    issue_initialized_local_input_source_set_v1, issue_source_bound_core_from_artifact_v1,
    resolve_loop_binding_declaration_v1, LoopBinaryI64OpV1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopBindingKeyV1, LoopBlockKeyV1,
    LoopCarrierKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopInitializedLocalInputSourceRelationV1, LoopInitializedLocalInputSourceSetRejectV1,
    LoopItemKeyV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopNodeKeyV1,
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1, LoopRecipeArtifactV1,
    LoopRecipeBindingRelationV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProducerIdV1, LoopRecipeProvenanceV1,
    LoopRecipeRejectReasonV1, LoopRecipeV1, LoopRecipeValueV1, LoopValueClassV1, LoopValueKeyV1,
    VerifiedLoopInitializedLocalInputSourceSetV1, VerifiedLoopOperationEffectProductV1,
};
use crate::mir::loop_structural_facts::{
    LoopSourceForestBindingRejectV1, NestedPredicateConditionEvidenceV1,
    NestedPredicateUpdateEvidenceV1,
};
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

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
    BindingDeclaration(BindingRefV1),
    BindingInitializer(BindingRefV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    SourceHandoff(NestedPhysicalSourceHandoffRejectV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

/// Physical-ready caller-zero product: the operation/effect Core and the
/// initialized-local input set are co-sealed with the verified Recipe; the
/// one-time source handoff stays attached for the dedicated topology issuer.
#[derive(Debug)]
pub(crate) struct VerifiedNestedPredicateRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
    source_handoff: VerifiedNestedPhysicalSourceHandoffV1,
}

impl VerifiedNestedPredicateRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &crate::mir::loop_recipe_contract::VerifiedLoopRecipeV1 {
        self.operations.core().recipe()
    }

    pub(crate) fn join_sig(&self) -> &crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV1 {
        self.operations.core().join_sig()
    }

    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }

    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }

    pub(crate) fn source_handoff(&self) -> &VerifiedNestedPhysicalSourceHandoffV1 {
        &self.source_handoff
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
        VerifiedNestedPhysicalSourceHandoffV1,
    ) {
        (self.operations, self.inputs, self.source_handoff)
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
        let (operations, _inputs, handoff) = self.into_parts();
        let (core, _evidence) = operations.into_parts();
        let (recipe, join_sig) = core.into_recipe_sig();
        (recipe, join_sig, handoff)
    }
}

pub(crate) fn produce_nested_predicate_recipe_v1(
    projection: VerifiedNestedLoopSourceProjectionV1,
    function: &VerifiedResolvedFunctionV1,
) -> Result<VerifiedNestedPredicateRecipeProductV1, NestedPredicateRecipeProducerRejectV1> {
    let (forest_binding, shape, root_frame_key) = projection.into_parts();
    validate_shape(&shape)?;
    let source_handoff =
        VerifiedNestedPhysicalSourceHandoffV1::issue(&forest_binding, shape, root_frame_key)
            .map_err(NestedPredicateRecipeProducerRejectV1::SourceHandoff)?;
    let shape = source_handoff.shape();
    let root_site = shape.root_site.clone();
    let child_site = shape.child_site.clone();
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
    let verified_recipe = crate::mir::loop_recipe_contract::verify_source_bound_recipe_v1(
        artifact.clone(),
    )
    .map_err(NestedPredicateRecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(NestedPredicateRecipeProducerRejectV1::JoinSig)?;
    let owner = shape.bindings[0].binding.owner();
    let binding_rows = binding_relations(function, shape)?;
    let effects = effect_relations(owner, &root_site, &child_site, shape);
    let core = issue_source_bound_core_from_artifact_v1(
        artifact,
        join_sig,
        owner,
        binding_rows,
        effects,
    )
    .map_err(NestedPredicateRecipeProducerRejectV1::Core)?;
    let input_rows = input_relations(function, shape)?;
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(NestedPredicateRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(owner, &root_site, &child_site, shape);
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(NestedPredicateRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedNestedPredicateRecipeProductV1 {
        operations,
        inputs: input_set,
        source_handoff,
    })
}

fn declaration_site(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Result<
    crate::mir::loop_recipe_contract::ResolvedLoopBindingDeclarationV1,
    NestedPredicateRecipeProducerRejectV1,
> {
    resolve_loop_binding_declaration_v1(function, binding)
        .ok_or(NestedPredicateRecipeProducerRejectV1::BindingDeclaration(binding))
}

fn binding_relations(
    function: &VerifiedResolvedFunctionV1,
    shape: &VerifiedNestedLoopSourceShapeV1,
) -> Result<Vec<LoopRecipeBindingRelationV1>, NestedPredicateRecipeProducerRejectV1> {
    [
        (LoopBindingKeyV1::new(0), shape.root_condition.binding),
        (LoopBindingKeyV1::new(1), shape.increment_ancestor.binding),
        (LoopBindingKeyV1::new(2), shape.child_condition.binding),
    ]
    .into_iter()
    .map(|(recipe_binding, source_binding)| {
        Ok(LoopRecipeBindingRelationV1::new(
            recipe_binding,
            source_binding,
            LoopValueClassV1::I64,
            BindingOriginV1::Source(declaration_site(function, source_binding)?.declaration),
        ))
    })
    .collect()
}

fn input_relations(
    function: &VerifiedResolvedFunctionV1,
    shape: &VerifiedNestedLoopSourceShapeV1,
) -> Result<Vec<LoopInitializedLocalInputSourceRelationV1>, NestedPredicateRecipeProducerRejectV1>
{
    [
        (LoopValueKeyV1::new(0), shape.root_condition.binding),
        (LoopValueKeyV1::new(3), shape.increment_ancestor.binding),
    ]
    .into_iter()
    .map(|(recipe_value, source_binding)| {
        let declaration = declaration_site(function, source_binding)?;
        let initializer = declaration.initializer.clone().ok_or(
            NestedPredicateRecipeProducerRejectV1::BindingInitializer(source_binding),
        )?;
        Ok(LoopInitializedLocalInputSourceRelationV1::new(
            declaration.declaration,
            initializer,
            source_binding,
            recipe_value,
            LoopValueClassV1::I64,
        ))
    })
    .collect()
}

fn update_effects(
    owner: FunctionOwnerIdV1,
    update: &NestedPredicateUpdateEvidenceV1,
    recipe_binding: LoopBindingKeyV1,
    read_ordinal: u32,
) -> [LoopBindingEffectRelationV1; 2] {
    let expr = |site: &SourceExprSiteV1| {
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site.clone()))
    };
    [
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead {
                ordinal: read_ordinal,
            },
            recipe_binding,
            update.binding,
            LoopValueClassV1::I64,
            expr(&update.lhs_site),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            recipe_binding,
            update.binding,
            LoopValueClassV1::I64,
            expr(&update.target_site),
        ),
    ]
}

fn effect_relations(
    owner: FunctionOwnerIdV1,
    root_site: &SourceStmtSiteV1,
    child_site: &SourceStmtSiteV1,
    shape: &VerifiedNestedLoopSourceShapeV1,
) -> Vec<LoopBindingEffectRelationV1> {
    let root = LoopBindingKeyV1::new(0);
    let ancestor = LoopBindingKeyV1::new(1);
    let child = LoopBindingKeyV1::new(2);
    let expr = |site: &SourceExprSiteV1| {
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site.clone()))
    };
    let carrier = |carrier: u32,
                   recipe_binding,
                   source_binding,
                   source_loop: &SourceStmtSiteV1| {
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            recipe_binding,
            source_binding,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: source_loop.clone(),
                carrier: LoopCarrierKeyV1::new(carrier),
            },
        )
    };
    let read = |ordinal: u32, recipe_binding, source_binding, site: &SourceExprSiteV1| {
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal },
            recipe_binding,
            source_binding,
            LoopValueClassV1::I64,
            expr(site),
        )
    };
    let mut effects = vec![
        carrier(0, root, shape.root_condition.binding, root_site),
        carrier(1, ancestor, shape.increment_ancestor.binding, root_site),
        carrier(2, child, shape.child_condition.binding, child_site),
        read(0, root, shape.root_condition.binding, &shape.root_condition.lhs_site),
        read(
            0,
            child,
            shape.child_condition.binding,
            &shape.child_condition.lhs_site,
        ),
    ];
    effects.extend(update_effects(owner, &shape.increment_ancestor, ancestor, 0));
    effects.extend(update_effects(owner, &shape.increment_child, child, 1));
    effects.extend(update_effects(owner, &shape.increment_root, root, 1));
    effects
}

fn update_evidence(
    start: u32,
    block: u32,
    owner_loop: u32,
    update: &NestedPredicateUpdateEvidenceV1,
    source_loop: &SourceStmtSiteV1,
    owner: FunctionOwnerIdV1,
) -> [LoopOperationSourceEvidenceV1; 4] {
    let row = |item: u32, site: SourceExprSiteV1, binding| {
        LoopOperationSourceEvidenceV1::new(
            LoopItemKeyV1::new(item),
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
            source_loop.clone(),
            LoopNodeKeyV1::new(owner_loop),
            LoopBlockKeyV1::new(block),
            binding,
        )
    };
    [
        row(start, update.lhs_site.clone(), Some(update.binding)),
        row(start + 1, update.value_site.clone(), None),
        row(start + 2, update.value_site.clone(), None),
        row(start + 3, update.target_site.clone(), Some(update.binding)),
    ]
}

fn condition_evidence(
    start: u32,
    block: u32,
    owner_loop: u32,
    condition: &NestedPredicateConditionEvidenceV1,
    source_loop: &SourceStmtSiteV1,
    owner: FunctionOwnerIdV1,
) -> [LoopOperationSourceEvidenceV1; 3] {
    let row = |item: u32, site: SourceExprSiteV1, binding| {
        LoopOperationSourceEvidenceV1::new(
            LoopItemKeyV1::new(item),
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
            source_loop.clone(),
            LoopNodeKeyV1::new(owner_loop),
            LoopBlockKeyV1::new(block),
            binding,
        )
    };
    [
        row(start, condition.lhs_site.clone(), Some(condition.binding)),
        row(start + 1, condition.site.clone(), None),
        row(start + 2, condition.site.clone(), None),
    ]
}

fn operation_evidence(
    owner: FunctionOwnerIdV1,
    root_site: &SourceStmtSiteV1,
    child_site: &SourceStmtSiteV1,
    shape: &VerifiedNestedLoopSourceShapeV1,
) -> Vec<LoopOperationSourceEvidenceV1> {
    let mut rows = Vec::with_capacity(16);
    rows.extend(condition_evidence(
        0,
        0,
        0,
        &shape.root_condition,
        root_site,
        owner,
    ));
    rows.push(LoopOperationSourceEvidenceV1::new(
        LoopItemKeyV1::new(3),
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(
            owner,
            shape.initialize_child.value_site.clone(),
        )),
        root_site.clone(),
        LoopNodeKeyV1::new(0),
        LoopBlockKeyV1::new(1),
        None,
    ));
    rows.extend(condition_evidence(
        5,
        2,
        1,
        &shape.child_condition,
        child_site,
        owner,
    ));
    rows.extend(update_evidence(
        8,
        3,
        1,
        &shape.increment_ancestor,
        child_site,
        owner,
    ));
    rows.extend(update_evidence(
        12,
        3,
        1,
        &shape.increment_child,
        child_site,
        owner,
    ));
    rows.extend(update_evidence(
        16,
        1,
        0,
        &shape.increment_root,
        root_site,
        owner,
    ));
    rows
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

pub(crate) fn nested_recipe(shape: &VerifiedNestedLoopSourceShapeV1) -> LoopRecipeV1 {
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
