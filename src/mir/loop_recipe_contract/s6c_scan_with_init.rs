//! The sole V2 Recipe producer for the typed S6C `ScanWithInit` cohort.
//!
//! The producer consumes the closed source Facts product.  It owns the
//! Recipe-local key table and keeps the Facts, Recipe, role seal, and Join
//! closure together.  No source walk, Artifact claim, or physical identity is
//! performed here.

use crate::mir::loop_structural_facts::{
    S6CScanWithInitFactsRefV1, VerifiedS6CScanWithInitFactsV1,
};

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::join_sig::{
    issue_sole_root_carrier_join_closure_v2, LoopJoinBranchArmTransferRefV2,
    LoopJoinBranchExitTargetV2, LoopJoinClosureRejectV2, LoopJoinEdgeRoleV1,
    LoopJoinLogicalTransferRejectV2, LoopJoinLogicalTransferViewV2, VerifiedLoopJoinClosureV2,
};
use super::schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopNodeV2,
    LoopOperationV2, LoopRecipeBindingV2, LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeExitV2,
    LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopValueClassV2,
};
use super::typed_schema_v2::{
    LoopRecipeV2RejectReason, LoopRecipeVerifierV2, VerifiedLoopRecipeV2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum S6CScanWithInitRecipeProducerRejectV2 {
    SourceRoleMismatch(&'static str),
    DomainCardinality(&'static str),
    RoleMismatch(&'static str),
    Recipe(LoopRecipeV2RejectReason),
    Join(LoopJoinClosureRejectV2),
    Transfer(LoopJoinLogicalTransferRejectV2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DefinedRoleV2 {
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
}

impl DefinedRoleV2 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WriteRoleV2 {
    item: LoopItemKeyV1,
    binding: LoopBindingKeyV1,
    value: LoopValueKeyV1,
}

impl WriteRoleV2 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn binding(self) -> LoopBindingKeyV1 {
        self.binding
    }

    pub(crate) const fn value(self) -> LoopValueKeyV1 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExitRoleV2 {
    item: LoopItemKeyV1,
    exit: LoopExitKeyV1,
}

impl ExitRoleV2 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn exit(self) -> LoopExitKeyV1 {
        self.exit
    }
}

/// Fixed role-to-key authority for the exact 15-item S6C Recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerifiedS6CScanWithInitRecipeRolesV2 {
    root_loop: LoopNodeKeyV1,
    condition_block: LoopBlockKeyV1,
    body_block: LoopBlockKeyV1,
    text_eq_then_block: LoopBlockKeyV1,
    index_binding: LoopBindingKeyV1,
    index_carrier: LoopCarrierKeyV1,
    subject_input: LoopValueKeyV1,
    needle_input: LoopValueKeyV1,
    index_input: LoopValueKeyV1,
    condition_index_read: DefinedRoleV2,
    length_call: DefinedRoleV2,
    less_condition: DefinedRoleV2,
    body_index_read: DefinedRoleV2,
    slice_one: DefinedRoleV2,
    slice_end_add: DefinedRoleV2,
    substring_call: DefinedRoleV2,
    text_equal: DefinedRoleV2,
    text_equal_if: LoopItemKeyV1,
    return_index_read: DefinedRoleV2,
    loop_return: ExitRoleV2,
    step_index_read: DefinedRoleV2,
    step_one: DefinedRoleV2,
    step_add: DefinedRoleV2,
    step_write: WriteRoleV2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitRecipeRolesRefV2<'a> {
    roles: &'a VerifiedS6CScanWithInitRecipeRolesV2,
}

impl S6CScanWithInitRecipeRolesRefV2<'_> {
    pub(crate) const fn root_loop(self) -> LoopNodeKeyV1 {
        self.roles.root_loop
    }
    pub(crate) const fn condition_block(self) -> LoopBlockKeyV1 {
        self.roles.condition_block
    }
    pub(crate) const fn body_block(self) -> LoopBlockKeyV1 {
        self.roles.body_block
    }
    pub(crate) const fn text_eq_then_block(self) -> LoopBlockKeyV1 {
        self.roles.text_eq_then_block
    }
    pub(crate) const fn index_binding(self) -> LoopBindingKeyV1 {
        self.roles.index_binding
    }
    pub(crate) const fn index_carrier(self) -> LoopCarrierKeyV1 {
        self.roles.index_carrier
    }
    pub(crate) const fn subject_input(self) -> LoopValueKeyV1 {
        self.roles.subject_input
    }
    pub(crate) const fn needle_input(self) -> LoopValueKeyV1 {
        self.roles.needle_input
    }
    pub(crate) const fn index_input(self) -> LoopValueKeyV1 {
        self.roles.index_input
    }
    pub(crate) const fn condition_index_read(self) -> DefinedRoleV2 {
        self.roles.condition_index_read
    }
    pub(crate) const fn length_call(self) -> DefinedRoleV2 {
        self.roles.length_call
    }
    pub(crate) const fn less_condition(self) -> DefinedRoleV2 {
        self.roles.less_condition
    }
    pub(crate) const fn body_index_read(self) -> DefinedRoleV2 {
        self.roles.body_index_read
    }
    pub(crate) const fn slice_one(self) -> DefinedRoleV2 {
        self.roles.slice_one
    }
    pub(crate) const fn slice_end_add(self) -> DefinedRoleV2 {
        self.roles.slice_end_add
    }
    pub(crate) const fn substring_call(self) -> DefinedRoleV2 {
        self.roles.substring_call
    }
    pub(crate) const fn text_equal(self) -> DefinedRoleV2 {
        self.roles.text_equal
    }
    pub(crate) const fn text_equal_if(self) -> LoopItemKeyV1 {
        self.roles.text_equal_if
    }
    pub(crate) const fn return_index_read(self) -> DefinedRoleV2 {
        self.roles.return_index_read
    }
    pub(crate) const fn loop_return(self) -> ExitRoleV2 {
        self.roles.loop_return
    }
    pub(crate) const fn step_index_read(self) -> DefinedRoleV2 {
        self.roles.step_index_read
    }
    pub(crate) const fn step_one(self) -> DefinedRoleV2 {
        self.roles.step_one
    }
    pub(crate) const fn step_add(self) -> DefinedRoleV2 {
        self.roles.step_add
    }
    pub(crate) const fn step_write(self) -> WriteRoleV2 {
        self.roles.step_write
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedS6CJoinRoleSealV2 {
    after_loop: LoopNodeKeyV1,
    after_binding: LoopBindingKeyV1,
    after_class: LoopValueClassV2,
    branch_count: usize,
    return_summary_count: usize,
    backedge_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CVerifiedRecipeReadViewV2<'a> {
    recipe: &'a VerifiedLoopRecipeV2,
}

impl S6CVerifiedRecipeReadViewV2<'_> {
    pub(crate) fn root_loop(self) -> LoopNodeKeyV1 {
        self.recipe.root_loop()
    }
    pub(crate) fn loop_count(self) -> usize {
        self.recipe_counts().0
    }
    pub(crate) fn block_count(self) -> usize {
        self.recipe_counts().1
    }
    pub(crate) fn item_count(self) -> usize {
        self.recipe_counts().2
    }
    pub(crate) fn value_count(self) -> usize {
        self.recipe_counts().3
    }

    fn recipe_counts(self) -> (usize, usize, usize, usize) {
        let recipe = self.recipe.as_recipe();
        (
            recipe.loops.len(),
            recipe.blocks.len(),
            recipe.items.len(),
            recipe.values.len(),
        )
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedS6CScanWithInitRecipeProductV2 {
    facts: VerifiedS6CScanWithInitFactsV1,
    recipe: VerifiedLoopRecipeV2,
    roles: VerifiedS6CScanWithInitRecipeRolesV2,
    join: VerifiedLoopJoinClosureV2,
    join_role_seal: VerifiedS6CJoinRoleSealV2,
}

impl VerifiedS6CScanWithInitRecipeProductV2 {
    pub(crate) fn with_product<R>(
        &self,
        callback: impl for<'product> FnOnce(S6CScanWithInitRecipeProductRefV2<'product>) -> R,
    ) -> R {
        self.facts.with_facts(|facts| {
            let transfer = self
                .join
                .logical_transfer_view()
                .expect("verified S6C Join transfer seal");
            callback(S6CScanWithInitRecipeProductRefV2 {
                facts,
                recipe: S6CVerifiedRecipeReadViewV2 {
                    recipe: &self.recipe,
                },
                roles: S6CScanWithInitRecipeRolesRefV2 { roles: &self.roles },
                transfer,
                join_role_seal: self.join_role_seal,
            })
        })
    }
}

#[derive(Debug)]
pub(crate) struct S6CScanWithInitRecipeProductRefV2<'a> {
    facts: S6CScanWithInitFactsRefV1<'a>,
    recipe: S6CVerifiedRecipeReadViewV2<'a>,
    roles: S6CScanWithInitRecipeRolesRefV2<'a>,
    transfer: LoopJoinLogicalTransferViewV2<'a>,
    join_role_seal: VerifiedS6CJoinRoleSealV2,
}

impl S6CScanWithInitRecipeProductRefV2<'_> {
    pub(crate) const fn facts(&self) -> S6CScanWithInitFactsRefV1<'_> {
        self.facts
    }
    pub(crate) const fn recipe(&self) -> S6CVerifiedRecipeReadViewV2<'_> {
        self.recipe
    }
    pub(crate) const fn roles(&self) -> S6CScanWithInitRecipeRolesRefV2<'_> {
        self.roles
    }
    pub(crate) fn logical_transfer(&self) -> &LoopJoinLogicalTransferViewV2<'_> {
        &self.transfer
    }
    pub(crate) const fn join_role_seal(&self) -> VerifiedS6CJoinRoleSealV2 {
        self.join_role_seal
    }
}

impl VerifiedS6CJoinRoleSealV2 {
    #[cfg(test)]
    pub(crate) const fn backedge_count(self) -> usize {
        self.backedge_count
    }
}

pub(crate) fn produce_s6c_scan_with_init_recipe_v2(
    facts: VerifiedS6CScanWithInitFactsV1,
) -> Result<VerifiedS6CScanWithInitRecipeProductV2, S6CScanWithInitRecipeProducerRejectV2> {
    let roles = fixed_roles();
    let recipe = LoopRecipeVerifierV2::verify(build_recipe())
        .map_err(S6CScanWithInitRecipeProducerRejectV2::Recipe)?;
    verify_recipe_domains(&recipe)?;
    verify_roles_against_recipe(&recipe, &roles)?;
    verify_source_roles(&facts)?;
    let join = issue_sole_root_carrier_join_closure_v2(&recipe)
        .map_err(S6CScanWithInitRecipeProducerRejectV2::Join)?;
    let transfer = join
        .logical_transfer_view()
        .map_err(S6CScanWithInitRecipeProducerRejectV2::Transfer)?;
    let join_role_seal = verify_join_transfer(&transfer, &roles)?;
    Ok(VerifiedS6CScanWithInitRecipeProductV2 {
        facts,
        recipe,
        roles,
        join,
        join_role_seal,
    })
}

fn fixed_roles() -> VerifiedS6CScanWithInitRecipeRolesV2 {
    let defined = |item, result| DefinedRoleV2 { item, result };
    VerifiedS6CScanWithInitRecipeRolesV2 {
        root_loop: LoopNodeKeyV1::new(0),
        condition_block: LoopBlockKeyV1::new(0),
        body_block: LoopBlockKeyV1::new(1),
        text_eq_then_block: LoopBlockKeyV1::new(2),
        index_binding: LoopBindingKeyV1::new(0),
        index_carrier: LoopCarrierKeyV1::new(0),
        subject_input: LoopValueKeyV1::new(0),
        needle_input: LoopValueKeyV1::new(1),
        index_input: LoopValueKeyV1::new(2),
        condition_index_read: defined(LoopItemKeyV1::new(0), LoopValueKeyV1::new(3)),
        length_call: defined(LoopItemKeyV1::new(1), LoopValueKeyV1::new(4)),
        less_condition: defined(LoopItemKeyV1::new(2), LoopValueKeyV1::new(5)),
        body_index_read: defined(LoopItemKeyV1::new(3), LoopValueKeyV1::new(6)),
        slice_one: defined(LoopItemKeyV1::new(4), LoopValueKeyV1::new(7)),
        slice_end_add: defined(LoopItemKeyV1::new(5), LoopValueKeyV1::new(8)),
        substring_call: defined(LoopItemKeyV1::new(6), LoopValueKeyV1::new(9)),
        text_equal: defined(LoopItemKeyV1::new(7), LoopValueKeyV1::new(10)),
        text_equal_if: LoopItemKeyV1::new(8),
        return_index_read: defined(LoopItemKeyV1::new(9), LoopValueKeyV1::new(11)),
        loop_return: ExitRoleV2 {
            item: LoopItemKeyV1::new(10),
            exit: LoopExitKeyV1::new(0),
        },
        step_index_read: defined(LoopItemKeyV1::new(11), LoopValueKeyV1::new(12)),
        step_one: defined(LoopItemKeyV1::new(12), LoopValueKeyV1::new(13)),
        step_add: defined(LoopItemKeyV1::new(13), LoopValueKeyV1::new(14)),
        step_write: WriteRoleV2 {
            item: LoopItemKeyV1::new(14),
            binding: LoopBindingKeyV1::new(0),
            value: LoopValueKeyV1::new(14),
        },
    }
}

fn build_recipe() -> LoopRecipeV2 {
    let b = LoopBindingKeyV1::new(0);
    let v = |raw| LoopValueKeyV1::new(raw);
    let i = |raw| LoopItemKeyV1::new(raw);
    let operation = |raw, operation| LoopRecipeItemRowV2 {
        key: i(raw),
        item: LoopRecipeItemV2::Operation { operation },
    };
    LoopRecipeV2 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV2 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: v(5),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: vec![
            block(0, &[0, 1, 2]),
            block(1, &[3, 4, 5, 6, 7, 8, 11, 12, 13, 14]),
            block(2, &[9, 10]),
        ],
        items: vec![
            operation(
                0,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(3),
                },
            ),
            operation(
                1,
                LoopOperationV2::CallSlot {
                    receiver: Some(v(0)),
                    args: vec![],
                    result: Some(v(4)),
                },
            ),
            operation(
                2,
                LoopOperationV2::CompareI64 {
                    op: LoopCompareI64OpV2::Less,
                    left: v(3),
                    right: v(4),
                    result: v(5),
                },
            ),
            operation(
                3,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(6),
                },
            ),
            operation(
                4,
                LoopOperationV2::ConstI64 {
                    result: v(7),
                    value: 1,
                },
            ),
            operation(
                5,
                LoopOperationV2::BinaryI64 {
                    op: LoopBinaryI64OpV2::Add,
                    left: v(6),
                    right: v(7),
                    result: v(8),
                },
            ),
            operation(
                6,
                LoopOperationV2::CallSlot {
                    receiver: Some(v(0)),
                    args: vec![v(6), v(8)],
                    result: Some(v(9)),
                },
            ),
            operation(
                7,
                LoopOperationV2::TextEq {
                    left: v(9),
                    right: v(1),
                    result: v(10),
                },
            ),
            LoopRecipeItemRowV2 {
                key: i(8),
                item: LoopRecipeItemV2::If {
                    condition: v(10),
                    then_block: LoopBlockKeyV1::new(2),
                    else_block: None,
                },
            },
            operation(
                9,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(11),
                },
            ),
            LoopRecipeItemRowV2 {
                key: i(10),
                item: LoopRecipeItemV2::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            operation(
                11,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(12),
                },
            ),
            operation(
                12,
                LoopOperationV2::ConstI64 {
                    result: v(13),
                    value: 1,
                },
            ),
            operation(
                13,
                LoopOperationV2::BinaryI64 {
                    op: LoopBinaryI64OpV2::Add,
                    left: v(12),
                    right: v(13),
                    result: v(14),
                },
            ),
            operation(
                14,
                LoopOperationV2::WriteBinding {
                    binding: b,
                    value: v(14),
                },
            ),
        ],
        bindings: vec![LoopRecipeBindingV2 {
            key: b,
            label: "index".to_owned(),
            class: LoopValueClassV2::I64,
        }],
        values: (0..15)
            .map(|raw| LoopRecipeValueV2 {
                key: v(raw),
                class: value_class(raw),
            })
            .collect(),
        inputs: vec![v(0), v(1), v(2)],
        carriers: vec![LoopRecipeCarrierV2 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            binding: b,
            class: LoopValueClassV2::I64,
            entry_value: v(2),
        }],
        exits: vec![LoopRecipeExitV2 {
            key: LoopExitKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            kind: LoopExitKindV2::Return { value: Some(v(11)) },
        }],
    }
}

fn block(raw: u32, items: &[u32]) -> LoopRecipeBlockV2 {
    LoopRecipeBlockV2 {
        key: LoopBlockKeyV1::new(raw),
        owner_loop: LoopNodeKeyV1::new(0),
        items: items.iter().copied().map(LoopItemKeyV1::new).collect(),
    }
}

fn value_class(raw: u32) -> LoopValueClassV2 {
    match raw {
        0 | 1 | 9 => LoopValueClassV2::Text,
        5 | 10 => LoopValueClassV2::Bool,
        _ => LoopValueClassV2::I64,
    }
}

fn verify_recipe_domains(
    recipe: &VerifiedLoopRecipeV2,
) -> Result<(), S6CScanWithInitRecipeProducerRejectV2> {
    let r = recipe.as_recipe();
    for (name, actual, expected) in [
        ("loops", r.loops.len(), 1),
        ("blocks", r.blocks.len(), 3),
        ("items", r.items.len(), 15),
        ("bindings", r.bindings.len(), 1),
        ("inputs", r.inputs.len(), 3),
        ("values", r.values.len(), 15),
        ("carriers", r.carriers.len(), 1),
        ("exits", r.exits.len(), 1),
    ] {
        if actual != expected {
            return Err(S6CScanWithInitRecipeProducerRejectV2::DomainCardinality(
                name,
            ));
        }
    }
    Ok(())
}

fn verify_roles_against_recipe(
    recipe: &VerifiedLoopRecipeV2,
    roles: &VerifiedS6CScanWithInitRecipeRolesV2,
) -> Result<(), S6CScanWithInitRecipeProducerRejectV2> {
    let r = recipe.as_recipe();
    if r.root_loop != roles.root_loop
        || r.loops[0].body != roles.body_block
        || r.loops[0].condition
            != (LoopConditionV2::Predicate {
                block: roles.condition_block,
                value: roles.less_condition.result,
            })
    {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch("loop"));
    }
    if r.blocks[roles.condition_block.raw() as usize].items
        != vec![
            roles.condition_index_read.item,
            roles.length_call.item,
            roles.less_condition.item,
        ]
        || r.blocks[roles.body_block.raw() as usize].items
            != vec![
                roles.body_index_read.item,
                roles.slice_one.item,
                roles.slice_end_add.item,
                roles.substring_call.item,
                roles.text_equal.item,
                roles.text_equal_if,
                roles.step_index_read.item,
                roles.step_one.item,
                roles.step_add.item,
                roles.step_write.item,
            ]
        || r.blocks[roles.text_eq_then_block.raw() as usize].items
            != vec![roles.return_index_read.item, roles.loop_return.item]
    {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "block items",
        ));
    }
    if r.inputs != vec![roles.subject_input, roles.needle_input, roles.index_input]
        || r.carriers[0].key != roles.index_carrier
        || r.carriers[0].binding != roles.index_binding
        || r.exits[0].key != roles.loop_return.exit
    {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "domain keys",
        ));
    }
    Ok(())
}

fn verify_source_roles(
    facts: &VerifiedS6CScanWithInitFactsV1,
) -> Result<(), S6CScanWithInitRecipeProducerRejectV2> {
    facts.with_facts(|view| {
        let source = view.source();
        let calls = source.calls();
        let typed = calls.typed();
        let expected = [
            (
                super::super::callable_semantic_batch::S6CTypedInputRoleV1::Subject,
                super::super::callable_semantic_batch::S6CLogicalValueClassV1::Text,
            ),
            (
                super::super::callable_semantic_batch::S6CTypedInputRoleV1::Needle,
                super::super::callable_semantic_batch::S6CLogicalValueClassV1::Text,
            ),
            (
                super::super::callable_semantic_batch::S6CTypedInputRoleV1::Index,
                super::super::callable_semantic_batch::S6CLogicalValueClassV1::I64,
            ),
        ];
        let actual = typed
            .inputs()
            .iter()
            .map(|row| (row.role(), row.class()))
            .collect::<Vec<_>>();
        if actual != expected {
            return Err(S6CScanWithInitRecipeProducerRejectV2::SourceRoleMismatch(
                "inputs",
            ));
        }
        if typed.binaries().len() != 4
            || typed.index_update().form()
                != crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain
        {
            return Err(S6CScanWithInitRecipeProducerRejectV2::SourceRoleMismatch(
                "typed relation",
            ));
        }
        Ok(())
    })
}

fn verify_join_transfer(
    transfer: &LoopJoinLogicalTransferViewV2<'_>,
    roles: &VerifiedS6CScanWithInitRecipeRolesV2,
) -> Result<VerifiedS6CJoinRoleSealV2, S6CScanWithInitRecipeProducerRejectV2> {
    let after = transfer.after();
    if after.loop_key() != roles.root_loop
        || after.binding() != roles.index_binding
        || after.class() != LoopValueClassV2::I64
    {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch("After"));
    }
    if transfer.branches().len() != 1 || transfer.summary_transfers().len() != 1 {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "Join cardinality",
        ));
    }
    let branch = transfer.branches()[0];
    if branch.owner_loop != roles.root_loop
        || branch.if_item != roles.text_equal_if
        || branch.condition != roles.text_equal.result
    {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "branch",
        ));
    }
    match branch.then_arm {
        LoopJoinBranchArmTransferRefV2::Exit(exit)
            if exit.exit_item == roles.loop_return.item
                && exit.target == LoopJoinBranchExitTargetV2::FunctionExit
                && exit.role == LoopJoinEdgeRoleV1::Return => {}
        _ => {
            return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
                "then arm",
            ))
        }
    }
    if !matches!(
        branch.else_arm,
        LoopJoinBranchArmTransferRefV2::Fallthrough { .. }
    ) {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "else arm",
        ));
    }
    let backedges = transfer
        .boundaries()
        .iter()
        .filter(|row| row.role == LoopJoinEdgeRoleV1::Backedge)
        .count();
    if backedges != 1 || transfer.summary_transfers()[0].role != LoopJoinEdgeRoleV1::Return {
        return Err(S6CScanWithInitRecipeProducerRejectV2::RoleMismatch(
            "Join exits",
        ));
    }
    Ok(VerifiedS6CJoinRoleSealV2 {
        after_loop: after.loop_key(),
        after_binding: after.binding(),
        after_class: after.class(),
        branch_count: 1,
        return_summary_count: 1,
        backedge_count: 1,
    })
}
