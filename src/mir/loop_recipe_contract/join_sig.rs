//! Pure logical join obligations for a verified recursive Loop recipe.
//!
//! This module is deliberately caller-zero.  It consumes only the portable
//! recipe and emits deterministic logical rows; physical identities and MIR
//! mutation belong to the later consumer.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig_branch::{direct_branch_row, is_supported_loop_branch_pair, loop_exit_edge};
use super::schema::{
    LoopConditionV1, LoopExitKindV1, LoopOperationV1, LoopRecipeItemV1, LoopRecipeV1,
    LoopValueClassV1,
};
use super::verify::VerifiedLoopRecipeV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopJoinPortV1 {
    Preheader,
    Header,
    Body,
    After,
    FunctionExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopJoinEdgeRoleV1 {
    Enter,
    PredicateTrue,
    PredicateFalse,
    BodyEntry,
    Backedge,
    Break,
    Continue,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinPayloadV1 {
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) value: LoopValueKeyV1,
    pub(crate) class: LoopValueClassV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinEdgeV1 {
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) payload: Vec<LoopJoinPayloadV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinLoopV1 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) carriers: Vec<LoopJoinPayloadV1>,
    pub(crate) edges: Vec<LoopJoinEdgeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinSigV1 {
    pub(crate) loops: Vec<LoopJoinLoopV1>,
    pub(crate) branches: Vec<LoopJoinBranchV1>,
}

/// Caller-zero logical evidence for the bounded LoopTrue branch shape.
///
/// This is deliberately not a CFG edge or a PHI plan.  It records the source
/// If item and its two direct exits so a later physical consumer can decide
/// how to materialize the already-verified choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchV1 {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_exit: LoopJoinBranchExitV1,
    pub(crate) else_exit: LoopJoinBranchExitV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExitV1 {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target_loop: LoopNodeKeyV1,
    pub(crate) payload: Vec<LoopJoinPayloadV1>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopJoinSigV1(LoopJoinSigV1);

impl VerifiedLoopJoinSigV1 {
    pub(crate) fn as_sig(&self) -> &LoopJoinSigV1 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinSigRejectReasonV1 {
    MissingCarrierClosure {
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    },
    BindingNotAvailable {
        binding: LoopBindingKeyV1,
    },
    ValueNotAvailable {
        value: LoopValueKeyV1,
    },
    UnreachableItem {
        item: LoopItemKeyV1,
    },
    BranchMergeMismatch {
        item: LoopItemKeyV1,
    },
    UnsupportedExit {
        item: LoopItemKeyV1,
    },
    UnsupportedNestedPredicate {
        loop_key: LoopNodeKeyV1,
    },
}

pub(crate) struct LoopJoinSigElaboratorV1;

impl LoopJoinSigElaboratorV1 {
    pub(crate) fn elaborate(
        verified: &VerifiedLoopRecipeV1,
    ) -> Result<VerifiedLoopJoinSigV1, LoopJoinSigRejectReasonV1> {
        let recipe = verified.as_recipe();
        let mut rows = Vec::with_capacity(recipe.loops.len());
        let mut branches = Vec::new();
        let mut available = recipe.inputs.iter().copied().collect::<BTreeSet<_>>();
        let mut bindings = BTreeMap::new();
        seed_carriers(recipe, verified.root_loop(), &mut bindings, &mut available);
        let _ = elaborate_loop(
            recipe,
            verified.root_loop(),
            &mut bindings,
            &mut available,
            &mut rows,
            &mut branches,
        )?;
        rows.sort_by_key(|row| row.key);
        branches.sort_by_key(|branch| (branch.owner_loop, branch.if_item));
        Ok(VerifiedLoopJoinSigV1(LoopJoinSigV1 {
            loops: rows,
            branches,
        }))
    }
}

#[derive(Debug)]
pub(super) struct Flow {
    pub(super) bindings: BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    pub(super) available: BTreeSet<LoopValueKeyV1>,
    pub(super) exit: Option<(LoopItemKeyV1, LoopExitKindV1)>,
    pub(super) alternate_exit: Option<(LoopItemKeyV1, LoopExitKindV1)>,
}

fn elaborate_loop(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    inherited: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoopV1>,
    branches: &mut Vec<LoopJoinBranchV1>,
) -> Result<Flow, LoopJoinSigRejectReasonV1> {
    let node = recipe
        .loops
        .get(key.raw() as usize)
        .expect("verified recipe has canonical loop keys");
    let parent_bindings = inherited.clone();
    let parent_available = available.clone();
    let mut local = parent_bindings.clone();
    let mut local_available = parent_available.clone();
    seed_carriers(recipe, key, &mut local, &mut local_available);
    if node.parent.is_some()
        && matches!(node.condition, LoopConditionV1::Predicate { .. })
        && !is_bounded_nested_predicate(recipe, key)
    {
        return Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate { loop_key: key });
    }
    let mut edges = vec![LoopJoinEdgeV1 {
        from: LoopJoinPortV1::Preheader,
        to: LoopJoinPortV1::Header,
        role: LoopJoinEdgeRoleV1::Enter,
        payload: visible_payloads(recipe, key, &local)?,
    }];

    if let LoopConditionV1::Predicate { block, value } = node.condition {
        let flow = process_block(
            recipe,
            key,
            block,
            &mut local,
            &mut local_available,
            rows,
            branches,
        )?;
        if flow.exit.is_some() || flow.alternate_exit.is_some() {
            return Err(LoopJoinSigRejectReasonV1::UnsupportedExit {
                item: block_item(recipe, block),
            });
        }
        local = flow.bindings;
        local_available = flow.available;
        require_value(&local_available, value)?;
        edges.push(LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Header,
            to: LoopJoinPortV1::Body,
            role: LoopJoinEdgeRoleV1::PredicateTrue,
            payload: visible_payloads(recipe, key, &local)?,
        });
        edges.push(LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Header,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::PredicateFalse,
            payload: visible_payloads(recipe, key, &local)?,
        });
    } else {
        edges.push(LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Header,
            to: LoopJoinPortV1::Body,
            role: LoopJoinEdgeRoleV1::BodyEntry,
            payload: visible_payloads(recipe, key, &local)?,
        });
    }

    let entry_bindings = local.keys().copied().collect::<BTreeSet<_>>();
    let body_flow = process_block(
        recipe,
        key,
        node.body,
        &mut local,
        &mut local_available,
        rows,
        branches,
    )?;
    for binding in body_flow.bindings.keys() {
        if !entry_bindings.contains(binding)
            && !recipe
                .carriers
                .iter()
                .any(|carrier| carrier.owner_loop == key && carrier.binding == *binding)
        {
            return Err(LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                loop_key: key,
                binding: *binding,
            });
        }
    }
    let body_payload = visible_payloads(recipe, key, &body_flow.bindings)?;
    let propagated_exit = if let Some(else_exit) = body_flow.alternate_exit {
        let then_exit = body_flow
            .exit
            .expect("alternate branch exit always has a primary exit");
        if !is_supported_loop_branch_pair(key, then_exit.1, else_exit.1) {
            return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: then_exit.0 });
        }
        edges.push(loop_exit_edge(then_exit.1, body_payload.clone()));
        edges.push(loop_exit_edge(else_exit.1, body_payload));
        None
    } else {
        match body_flow.exit {
            None => {
                edges.push(LoopJoinEdgeV1 {
                    from: LoopJoinPortV1::Body,
                    to: LoopJoinPortV1::Header,
                    role: LoopJoinEdgeRoleV1::Backedge,
                    payload: body_payload,
                });
                None
            }
            Some((item, LoopExitKindV1::Continue { target_loop })) if target_loop == key => {
                edges.push(LoopJoinEdgeV1 {
                    from: LoopJoinPortV1::Body,
                    to: LoopJoinPortV1::Header,
                    role: LoopJoinEdgeRoleV1::Continue,
                    payload: body_payload,
                });
                let _ = item;
                None
            }
            Some((item, LoopExitKindV1::Break { target_loop })) if target_loop == key => {
                edges.push(LoopJoinEdgeV1 {
                    from: LoopJoinPortV1::Body,
                    to: LoopJoinPortV1::After,
                    role: LoopJoinEdgeRoleV1::Break,
                    payload: body_payload,
                });
                let _ = item;
                None
            }
            Some((item, exit @ LoopExitKindV1::Return { .. })) => {
                edges.push(LoopJoinEdgeV1 {
                    from: LoopJoinPortV1::Body,
                    to: LoopJoinPortV1::FunctionExit,
                    role: LoopJoinEdgeRoleV1::Return,
                    payload: body_payload,
                });
                Some((item, exit))
            }
            Some((item, _)) => return Err(LoopJoinSigRejectReasonV1::UnsupportedExit { item }),
        }
    };

    let carriers = payloads(recipe, key, &body_flow.bindings)?;
    rows.push(LoopJoinLoopV1 {
        key,
        parent: node.parent,
        carriers,
        edges,
    });
    // A child may update an inherited binding (for example `sum`), but its
    // own recurrence locals (for example nested `j`) end at the child edge.
    // Keep this lexical/recurrence boundary in the logical elaborator; the
    // later physical session owns the actual resume edge and PHI material.
    let resumed = project_parent_flow(
        Flow {
            bindings: body_flow.bindings,
            available: body_flow.available,
            exit: propagated_exit,
            alternate_exit: None,
        },
        &parent_bindings,
        &parent_available,
    );
    *inherited = resumed.bindings.clone();
    *available = resumed.available.clone();
    Ok(resumed)
}

fn is_bounded_nested_predicate(recipe: &LoopRecipeV1, key: LoopNodeKeyV1) -> bool {
    let Some(node) = recipe.loops.get(key.raw() as usize) else {
        return false;
    };
    let Some(parent_key) = node.parent else {
        return false;
    };
    let Some(parent) = recipe.loops.get(parent_key.raw() as usize) else {
        return false;
    };
    if parent.parent.is_some() || !matches!(parent.condition, LoopConditionV1::Predicate { .. }) {
        return false;
    }
    let LoopConditionV1::Predicate { block, .. } = node.condition else {
        return false;
    };
    if !block_has_only_operations(recipe, block, is_nested_predicate_condition_operation)
        || !block_has_only_operations(recipe, node.body, is_nested_predicate_body_operation)
    {
        return false;
    }
    if recipe
        .loops
        .iter()
        .any(|candidate| candidate.parent == Some(key))
    {
        return false;
    }
    let Some(parent_body) = recipe.blocks.get(parent.body.raw() as usize) else {
        return false;
    };
    let mut child_items = 0;
    for item_key in &parent_body.items {
        let Some(row) = recipe.items.get(item_key.raw() as usize) else {
            return false;
        };
        match row.item {
            LoopRecipeItemV1::Loop { loop_key } if loop_key == key => child_items += 1,
            LoopRecipeItemV1::Operation { .. } => {}
            LoopRecipeItemV1::If { .. }
            | LoopRecipeItemV1::Loop { .. }
            | LoopRecipeItemV1::Exit { .. } => {
                return false;
            }
        }
    }
    child_items == 1
}

fn block_has_only_operations(
    recipe: &LoopRecipeV1,
    block: LoopBlockKeyV1,
    allowed: fn(LoopOperationV1) -> bool,
) -> bool {
    let Some(block) = recipe.blocks.get(block.raw() as usize) else {
        return false;
    };
    block.items.iter().all(|item_key| {
        let Some(row) = recipe.items.get(item_key.raw() as usize) else {
            return false;
        };
        matches!(&row.item, LoopRecipeItemV1::Operation { operation } if allowed(*operation))
    })
}

fn is_nested_predicate_condition_operation(operation: LoopOperationV1) -> bool {
    matches!(
        operation,
        LoopOperationV1::ReadBinding { .. }
            | LoopOperationV1::ConstI64 { .. }
            | LoopOperationV1::CompareI64 { .. }
    )
}

fn is_nested_predicate_body_operation(operation: LoopOperationV1) -> bool {
    matches!(
        operation,
        LoopOperationV1::ReadBinding { .. }
            | LoopOperationV1::ConstI64 { .. }
            | LoopOperationV1::BinaryI64 { .. }
            | LoopOperationV1::CompareI64 { .. }
            | LoopOperationV1::WriteBinding { .. }
    )
}

fn project_parent_flow(
    flow: Flow,
    parent_bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    parent_available: &BTreeSet<LoopValueKeyV1>,
) -> Flow {
    let bindings = parent_bindings
        .iter()
        .map(|(binding, before)| {
            (
                *binding,
                flow.bindings.get(binding).copied().unwrap_or(*before),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut available = parent_available.clone();
    available.extend(bindings.values().copied());
    Flow {
        bindings,
        available,
        exit: flow.exit,
        alternate_exit: flow.alternate_exit,
    }
}

fn process_block(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    key: LoopBlockKeyV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoopV1>,
    branches: &mut Vec<LoopJoinBranchV1>,
) -> Result<Flow, LoopJoinSigRejectReasonV1> {
    let block = recipe
        .blocks
        .get(key.raw() as usize)
        .expect("verified recipe has canonical block keys");
    let mut flow = Flow {
        bindings: bindings.clone(),
        available: available.clone(),
        exit: None,
        alternate_exit: None,
    };
    for item_key in &block.items {
        if flow.exit.is_some() || flow.alternate_exit.is_some() {
            return Err(LoopJoinSigRejectReasonV1::UnreachableItem { item: *item_key });
        }
        let row = recipe
            .items
            .get(item_key.raw() as usize)
            .expect("verified recipe has canonical item keys");
        match row.item {
            LoopRecipeItemV1::Operation { operation } => {
                process_operation(
                    owner_loop,
                    operation,
                    &mut flow.bindings,
                    &mut flow.available,
                )?;
            }
            LoopRecipeItemV1::Loop { loop_key } => {
                let child = elaborate_loop(
                    recipe,
                    loop_key,
                    &mut flow.bindings,
                    &mut flow.available,
                    rows,
                    branches,
                )?;
                flow.bindings = child.bindings;
                flow.available = child.available;
                flow.exit = child.exit;
                flow.alternate_exit = child.alternate_exit;
            }
            LoopRecipeItemV1::If {
                condition,
                then_block,
                else_block,
            } => {
                require_value(&flow.available, condition)?;
                let mut then_bindings = flow.bindings.clone();
                let mut then_available = flow.available.clone();
                let then_flow = process_block(
                    recipe,
                    owner_loop,
                    then_block,
                    &mut then_bindings,
                    &mut then_available,
                    rows,
                    branches,
                )?;
                let explicit_else_block = else_block;
                let else_flow = if let Some(else_block) = explicit_else_block {
                    let mut else_bindings = flow.bindings.clone();
                    let mut else_available = flow.available.clone();
                    let else_flow = process_block(
                        recipe,
                        owner_loop,
                        else_block,
                        &mut else_bindings,
                        &mut else_available,
                        rows,
                        branches,
                    )?;
                    else_flow
                } else {
                    Flow {
                        bindings: flow.bindings.clone(),
                        available: flow.available.clone(),
                        exit: None,
                        alternate_exit: None,
                    }
                };
                if then_flow.alternate_exit.is_some() || else_flow.alternate_exit.is_some() {
                    return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: *item_key });
                }
                if then_flow.exit.is_some() && else_flow.exit.is_some() {
                    if then_flow.bindings != else_flow.bindings
                        || then_flow.available != else_flow.available
                    {
                        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch {
                            item: *item_key,
                        });
                    }
                    let branch = direct_branch_row(
                        recipe,
                        owner_loop,
                        *item_key,
                        condition,
                        then_block,
                        explicit_else_block,
                        &then_flow,
                        &else_flow,
                    )?;
                    branches.push(branch);
                    flow.bindings = then_flow.bindings;
                    flow.available = then_flow.available;
                    flow.exit = then_flow.exit;
                    flow.alternate_exit = else_flow.exit;
                    continue;
                }
                if then_flow.exit != else_flow.exit
                    || then_flow.bindings != else_flow.bindings
                    || then_flow.available != else_flow.available
                {
                    return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: *item_key });
                }
                flow.bindings = then_flow.bindings;
                flow.available = then_flow.available;
                flow.exit = then_flow.exit;
                flow.alternate_exit = None;
            }
            LoopRecipeItemV1::Exit { exit } => {
                let exit_row = recipe
                    .exits
                    .get(exit.raw() as usize)
                    .expect("verified recipe has canonical exit keys");
                flow.exit = Some((*item_key, exit_row.kind));
            }
        }
    }
    *bindings = flow.bindings.clone();
    *available = flow.available.clone();
    Ok(flow)
}

fn process_operation(
    owner_loop: LoopNodeKeyV1,
    operation: LoopOperationV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
) -> Result<(), LoopJoinSigRejectReasonV1> {
    let require = |value| require_value(available, value);
    match operation {
        LoopOperationV1::ReadBinding { binding, result } => {
            let value = bindings
                .get(&binding)
                .copied()
                .ok_or(LoopJoinSigRejectReasonV1::BindingNotAvailable { binding })?;
            require(value)?;
            available.insert(result);
        }
        LoopOperationV1::ConstI64 { result, .. } => {
            available.insert(result);
        }
        LoopOperationV1::BinaryI64 {
            left,
            right,
            result,
            ..
        }
        | LoopOperationV1::CompareI64 {
            left,
            right,
            result,
            ..
        } => {
            require(left)?;
            require(right)?;
            available.insert(result);
        }
        LoopOperationV1::WriteBinding { binding, value } => {
            require(value)?;
            if !bindings.contains_key(&binding) {
                return Err(LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                    loop_key: owner_loop,
                    binding,
                });
            }
            bindings.insert(binding, value);
        }
    }
    Ok(())
}

fn seed_carriers(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
) {
    for carrier in recipe
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == key)
    {
        bindings.insert(carrier.binding, carrier.entry_value);
        available.insert(carrier.entry_value);
    }
}

fn payloads(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayloadV1>, LoopJoinSigRejectReasonV1> {
    recipe
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == key)
        .map(|carrier| {
            let value = bindings.get(&carrier.binding).copied().ok_or(
                LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                    loop_key: key,
                    binding: carrier.binding,
                },
            )?;
            Ok(LoopJoinPayloadV1 {
                binding: carrier.binding,
                value,
                class: carrier.class,
            })
        })
        .collect()
}

pub(super) fn visible_payloads(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayloadV1>, LoopJoinSigRejectReasonV1> {
    let mut lineage = Vec::new();
    let mut cursor = Some(key);
    while let Some(loop_key) = cursor {
        lineage.push(loop_key);
        cursor = recipe
            .loops
            .get(loop_key.raw() as usize)
            .and_then(|node| node.parent);
    }
    lineage.reverse();
    lineage
        .into_iter()
        .flat_map(|owner| {
            recipe
                .carriers
                .iter()
                .filter(move |carrier| carrier.owner_loop == owner)
        })
        .map(|carrier| {
            let value = bindings.get(&carrier.binding).copied().ok_or(
                LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                    loop_key: carrier.owner_loop,
                    binding: carrier.binding,
                },
            )?;
            Ok(LoopJoinPayloadV1 {
                binding: carrier.binding,
                value,
                class: carrier.class,
            })
        })
        .collect()
}

fn require_value(
    available: &BTreeSet<LoopValueKeyV1>,
    value: LoopValueKeyV1,
) -> Result<(), LoopJoinSigRejectReasonV1> {
    if available.contains(&value) {
        Ok(())
    } else {
        Err(LoopJoinSigRejectReasonV1::ValueNotAvailable { value })
    }
}

fn block_item(recipe: &LoopRecipeV1, key: LoopBlockKeyV1) -> LoopItemKeyV1 {
    recipe
        .blocks
        .get(key.raw() as usize)
        .and_then(|block| block.items.first().copied())
        .unwrap_or(LoopItemKeyV1::new(0))
}
