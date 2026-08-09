use std::collections::{BTreeMap, BTreeSet};

use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::super::join_sig_branch::{branch_row, is_supported_loop_branch_pair};
use super::super::verify::VerifiedLoopRecipeV1;
use super::model::{
    LoopJoinBranch, LoopJoinEdge, LoopJoinEdgeRoleV1, LoopJoinLoop, LoopJoinPayload, LoopJoinSig,
    LoopJoinSigRejectReasonV1, VerifiedLoopJoinSig, VerifiedLoopJoinSigV1,
};
use super::port::{loop_exit_edge, port_bindings};
use super::recipe_view::{
    LoopJoinConditionView, LoopJoinExitView, LoopJoinItemView, LoopJoinOperationFamily,
    LoopJoinOperationView, LoopJoinRecipeView, LoopJoinValueUses, LoopRecipeV1JoinView,
};
use super::visibility::{
    block_item, has_only_operations, payloads, seed_carriers, visible_payloads_from_view,
};

#[derive(Debug)]
pub(in crate::mir::loop_recipe_contract) struct Flow<C> {
    pub(in crate::mir::loop_recipe_contract) bindings: BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    pub(in crate::mir::loop_recipe_contract) available: BTreeSet<LoopValueKeyV1>,
    pub(in crate::mir::loop_recipe_contract) exit: Option<(LoopItemKeyV1, LoopJoinExitView)>,
    pub(in crate::mir::loop_recipe_contract) exit_payload: Option<Vec<LoopJoinPayload<C>>>,
    pub(in crate::mir::loop_recipe_contract) alternate_exit:
        Option<(LoopItemKeyV1, LoopJoinExitView)>,
    pub(in crate::mir::loop_recipe_contract) alternate_exit_payload:
        Option<Vec<LoopJoinPayload<C>>>,
    pub(in crate::mir::loop_recipe_contract) side_exits: Vec<FlowExit<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::loop_recipe_contract) struct FlowExit<C> {
    pub(in crate::mir::loop_recipe_contract) item: LoopItemKeyV1,
    pub(in crate::mir::loop_recipe_contract) kind: LoopJoinExitView,
    pub(in crate::mir::loop_recipe_contract) payload: Vec<LoopJoinPayload<C>>,
}

pub(crate) struct LoopJoinSigElaboratorV1;

impl LoopJoinSigElaboratorV1 {
    pub(crate) fn elaborate(
        verified: &VerifiedLoopRecipeV1,
    ) -> Result<VerifiedLoopJoinSigV1, LoopJoinSigRejectReasonV1> {
        elaborate_view(&LoopRecipeV1JoinView::verified(verified))
    }
}

pub(super) fn elaborate_view<V: LoopJoinRecipeView>(
    recipe: &V,
) -> Result<VerifiedLoopJoinSig<V::Class, V::BranchTarget>, LoopJoinSigRejectReasonV1> {
    let mut rows = Vec::with_capacity(recipe.loop_count());
    let mut branches = Vec::new();
    let mut available = recipe.inputs().iter().copied().collect();
    let mut bindings = BTreeMap::new();
    elaborate_loop(
        recipe,
        recipe.root_loop(),
        &mut bindings,
        &mut available,
        &mut rows,
        &mut branches,
    )?;
    rows.sort_by_key(|row| row.key);
    branches.sort_by_key(|branch| (branch.owner_loop, branch.if_item));
    let port_bindings = port_bindings(&rows)?;
    Ok(VerifiedLoopJoinSig::from_sig(LoopJoinSig {
        loops: rows,
        branches,
        port_bindings,
    }))
}

pub(super) fn elaborate_loop<V: LoopJoinRecipeView>(
    recipe: &V,
    key: LoopNodeKeyV1,
    inherited: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoop<V::Class>>,
    branches: &mut Vec<LoopJoinBranch<V::Class, V::BranchTarget>>,
) -> Result<Flow<V::Class>, LoopJoinSigRejectReasonV1> {
    let node = recipe
        .loop_at(key)
        .expect("verified recipe has canonical loop keys");
    let parent_bindings = inherited.clone();
    let parent_available = available.clone();
    let mut local = parent_bindings.clone();
    let mut local_available = parent_available.clone();
    seed_carriers(recipe, key, &mut local, &mut local_available);
    if node.parent.is_some()
        && matches!(node.condition, LoopJoinConditionView::Predicate { .. })
        && !is_bounded_nested_predicate(recipe, key)
    {
        return Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate { loop_key: key });
    }
    let mut edges = vec![LoopJoinEdge {
        from: super::model::LoopJoinPortV1::Preheader,
        to: super::model::LoopJoinPortV1::Header,
        role: LoopJoinEdgeRoleV1::Enter,
        payload: visible_payloads_from_view(recipe, key, &local)?,
    }];

    if let LoopJoinConditionView::Predicate { block, value } = node.condition {
        let flow = process_block(
            recipe,
            key,
            block,
            &mut local,
            &mut local_available,
            rows,
            branches,
        )?;
        if flow.exit.is_some() || flow.alternate_exit.is_some() || !flow.side_exits.is_empty() {
            return Err(LoopJoinSigRejectReasonV1::UnsupportedExit {
                item: block_item(recipe, block),
            });
        }
        local = flow.bindings;
        local_available = flow.available;
        require_value(&local_available, value)?;
        edges.push(LoopJoinEdge {
            from: super::model::LoopJoinPortV1::Header,
            to: super::model::LoopJoinPortV1::Body,
            role: LoopJoinEdgeRoleV1::PredicateTrue,
            payload: visible_payloads_from_view(recipe, key, &local)?,
        });
        edges.push(LoopJoinEdge {
            from: super::model::LoopJoinPortV1::Header,
            to: super::model::LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::PredicateFalse,
            payload: visible_payloads_from_view(recipe, key, &local)?,
        });
    } else {
        edges.push(LoopJoinEdge {
            from: super::model::LoopJoinPortV1::Header,
            to: super::model::LoopJoinPortV1::Body,
            role: LoopJoinEdgeRoleV1::BodyEntry,
            payload: visible_payloads_from_view(recipe, key, &local)?,
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
        let has_carrier = (0..recipe.carrier_count()).any(|index| {
            let carrier = recipe
                .carrier_at(index)
                .expect("verified Recipe view has dense carrier rows");
            carrier.owner_loop == key && carrier.binding == *binding
        });
        if !entry_bindings.contains(binding) && !has_carrier {
            return Err(LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                loop_key: key,
                binding: *binding,
            });
        }
    }
    let body_payload = visible_payloads_from_view(recipe, key, &body_flow.bindings)?;
    for side_exit in &body_flow.side_exits {
        if recipe.branch_exit_target(key, side_exit.kind).is_none() {
            return Err(LoopJoinSigRejectReasonV1::UnsupportedExit {
                item: side_exit.item,
            });
        }
        edges.push(loop_exit_edge(side_exit.kind, side_exit.payload.clone()));
    }
    let propagated_exit = if let Some(else_exit) = body_flow.alternate_exit {
        let then_exit = body_flow
            .exit
            .expect("alternate branch exit always has a primary exit");
        if !is_supported_loop_branch_pair(recipe, key, then_exit.1, else_exit.1) {
            return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: then_exit.0 });
        }
        edges.push(loop_exit_edge(
            then_exit.1,
            body_flow
                .exit_payload
                .clone()
                .expect("primary branch exit has payload"),
        ));
        edges.push(loop_exit_edge(
            else_exit.1,
            body_flow
                .alternate_exit_payload
                .clone()
                .expect("alternate branch exit has payload"),
        ));
        None
    } else {
        match body_flow.exit {
            None => {
                edges.push(LoopJoinEdge {
                    from: super::model::LoopJoinPortV1::Body,
                    to: super::model::LoopJoinPortV1::Header,
                    role: LoopJoinEdgeRoleV1::Backedge,
                    payload: body_payload,
                });
                None
            }
            Some((item, LoopJoinExitView::Continue { target_loop })) if target_loop == key => {
                edges.push(LoopJoinEdge {
                    from: super::model::LoopJoinPortV1::Body,
                    to: super::model::LoopJoinPortV1::Header,
                    role: LoopJoinEdgeRoleV1::Continue,
                    payload: body_payload,
                });
                let _ = item;
                None
            }
            Some((item, LoopJoinExitView::Break { target_loop })) if target_loop == key => {
                edges.push(LoopJoinEdge {
                    from: super::model::LoopJoinPortV1::Body,
                    to: super::model::LoopJoinPortV1::After,
                    role: LoopJoinEdgeRoleV1::Break,
                    payload: body_payload,
                });
                let _ = item;
                None
            }
            Some((item, exit @ LoopJoinExitView::Return { .. })) => {
                edges.push(LoopJoinEdge {
                    from: super::model::LoopJoinPortV1::Body,
                    to: super::model::LoopJoinPortV1::FunctionExit,
                    role: LoopJoinEdgeRoleV1::Return,
                    payload: body_payload,
                });
                Some((item, exit))
            }
            Some((item, _)) => return Err(LoopJoinSigRejectReasonV1::UnsupportedExit { item }),
        }
    };

    let carriers = payloads(recipe, key, &body_flow.bindings)?;
    rows.push(LoopJoinLoop {
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
            exit_payload: None,
            alternate_exit: None,
            alternate_exit_payload: None,
            side_exits: Vec::new(),
        },
        &parent_bindings,
        &parent_available,
    );
    *inherited = resumed.bindings.clone();
    *available = resumed.available.clone();
    Ok(resumed)
}

fn is_bounded_nested_predicate<V: LoopJoinRecipeView>(recipe: &V, key: LoopNodeKeyV1) -> bool {
    let Some(node) = recipe.loop_at(key) else {
        return false;
    };
    let Some(parent_key) = node.parent else {
        return false;
    };
    let Some(parent) = recipe.loop_at(parent_key) else {
        return false;
    };
    if parent.parent.is_some()
        || !matches!(parent.condition, LoopJoinConditionView::Predicate { .. })
    {
        return false;
    }
    let LoopJoinConditionView::Predicate { block, .. } = node.condition else {
        return false;
    };
    if !has_only_operations(recipe, block, is_nested_predicate_condition_operation)
        || !has_only_operations(recipe, node.body, is_nested_predicate_body_operation)
    {
        return false;
    }
    if (0..recipe.loop_count()).any(|raw| {
        recipe
            .loop_at(LoopNodeKeyV1::new(raw as u32))
            .is_some_and(|candidate| candidate.parent == Some(key))
    }) {
        return false;
    }
    let Some(parent_body) = recipe.block_at(parent.body) else {
        return false;
    };
    let mut child_items = 0;
    for item_key in parent_body.items {
        match recipe.item_at(*item_key) {
            Some(LoopJoinItemView::Loop { loop_key }) if loop_key == key => child_items += 1,
            Some(LoopJoinItemView::Operation(_)) => {}
            Some(
                LoopJoinItemView::If { .. }
                | LoopJoinItemView::Loop { .. }
                | LoopJoinItemView::Exit { .. },
            )
            | None => {
                return false;
            }
        }
    }
    child_items == 1
}

fn is_nested_predicate_condition_operation(operation: LoopJoinOperationFamily) -> bool {
    matches!(
        operation,
        LoopJoinOperationFamily::ReadBinding
            | LoopJoinOperationFamily::ConstI64
            | LoopJoinOperationFamily::CompareI64
    )
}

fn is_nested_predicate_body_operation(operation: LoopJoinOperationFamily) -> bool {
    matches!(
        operation,
        LoopJoinOperationFamily::ReadBinding
            | LoopJoinOperationFamily::ConstI64
            | LoopJoinOperationFamily::BinaryI64
            | LoopJoinOperationFamily::CompareI64
            | LoopJoinOperationFamily::WriteBinding
    )
}

fn project_parent_flow<C>(
    flow: Flow<C>,
    parent_bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    parent_available: &BTreeSet<LoopValueKeyV1>,
) -> Flow<C> {
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
        exit_payload: flow.exit_payload,
        alternate_exit: flow.alternate_exit,
        alternate_exit_payload: flow.alternate_exit_payload,
        side_exits: Vec::new(),
    }
}

fn process_block<V: LoopJoinRecipeView>(
    recipe: &V,
    owner_loop: LoopNodeKeyV1,
    key: LoopBlockKeyV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoop<V::Class>>,
    branches: &mut Vec<LoopJoinBranch<V::Class, V::BranchTarget>>,
) -> Result<Flow<V::Class>, LoopJoinSigRejectReasonV1> {
    let block = recipe
        .block_at(key)
        .expect("verified recipe has canonical block keys");
    let mut flow = Flow {
        bindings: bindings.clone(),
        available: available.clone(),
        exit: None,
        exit_payload: None,
        alternate_exit: None,
        alternate_exit_payload: None,
        side_exits: Vec::new(),
    };
    for item_key in block.items {
        if flow.exit.is_some() || flow.alternate_exit.is_some() {
            return Err(LoopJoinSigRejectReasonV1::UnreachableItem { item: *item_key });
        }
        let row = recipe
            .item_at(*item_key)
            .expect("verified recipe has canonical item keys");
        match row {
            LoopJoinItemView::Operation(operation) => {
                process_operation(
                    owner_loop,
                    operation,
                    &mut flow.bindings,
                    &mut flow.available,
                )?;
            }
            LoopJoinItemView::Loop { loop_key } => {
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
                flow.exit_payload = child.exit_payload;
                flow.alternate_exit = child.alternate_exit;
                flow.alternate_exit_payload = child.alternate_exit_payload;
                flow.side_exits.extend(child.side_exits);
            }
            LoopJoinItemView::If {
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
                    process_block(
                        recipe,
                        owner_loop,
                        else_block,
                        &mut else_bindings,
                        &mut else_available,
                        rows,
                        branches,
                    )?
                } else {
                    Flow {
                        bindings: flow.bindings.clone(),
                        available: flow.available.clone(),
                        exit: None,
                        exit_payload: None,
                        alternate_exit: None,
                        alternate_exit_payload: None,
                        side_exits: Vec::new(),
                    }
                };
                if then_flow.alternate_exit.is_some() || else_flow.alternate_exit.is_some() {
                    return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: *item_key });
                }
                let then_has_exit = then_flow.exit.is_some();
                let else_has_exit = else_flow.exit.is_some();
                if then_has_exit ^ else_has_exit {
                    if !then_flow.side_exits.is_empty() || !else_flow.side_exits.is_empty() {
                        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch {
                            item: *item_key,
                        });
                    }
                    branches.push(branch_row(
                        recipe,
                        owner_loop,
                        *item_key,
                        condition,
                        then_block,
                        explicit_else_block,
                        &then_flow,
                        &else_flow,
                    )?);
                    if let Some((item, kind)) = then_flow.exit {
                        flow.side_exits.push(FlowExit {
                            item,
                            kind,
                            payload: then_flow
                                .exit_payload
                                .clone()
                                .expect("terminal branch has payload"),
                        });
                    }
                    if let Some((item, kind)) = else_flow.exit {
                        flow.side_exits.push(FlowExit {
                            item,
                            kind,
                            payload: else_flow
                                .exit_payload
                                .clone()
                                .expect("terminal branch has payload"),
                        });
                    }
                    let normal = if then_has_exit { else_flow } else { then_flow };
                    flow.bindings = normal.bindings;
                    flow.available = normal.available;
                    flow.exit = None;
                    flow.exit_payload = None;
                    flow.alternate_exit = None;
                    flow.alternate_exit_payload = None;
                    continue;
                }
                if then_flow.exit.is_some() && else_flow.exit.is_some() {
                    if !then_flow.side_exits.is_empty() || !else_flow.side_exits.is_empty() {
                        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch {
                            item: *item_key,
                        });
                    }
                    let branch = branch_row(
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
                    flow.exit_payload = then_flow.exit_payload;
                    flow.alternate_exit = else_flow.exit;
                    flow.alternate_exit_payload = else_flow.exit_payload;
                    continue;
                }
                if then_flow.exit != else_flow.exit
                    || then_flow.bindings != else_flow.bindings
                    || then_flow.available != else_flow.available
                    || !then_flow.side_exits.is_empty()
                    || !else_flow.side_exits.is_empty()
                {
                    return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: *item_key });
                }
                flow.bindings = then_flow.bindings;
                flow.available = then_flow.available;
                flow.exit = then_flow.exit;
                flow.exit_payload = then_flow.exit_payload;
                flow.alternate_exit = None;
                flow.alternate_exit_payload = None;
            }
            LoopJoinItemView::Exit { exit } => {
                let exit_row = recipe
                    .exit_at(exit)
                    .expect("verified recipe has canonical exit keys");
                flow.exit = Some((*item_key, exit_row));
                flow.exit_payload = Some(visible_payloads_from_view(
                    recipe,
                    owner_loop,
                    &flow.bindings,
                )?);
            }
        }
    }
    *bindings = flow.bindings.clone();
    *available = flow.available.clone();
    Ok(flow)
}

fn process_operation<'a>(
    owner_loop: LoopNodeKeyV1,
    operation: LoopJoinOperationView<'a>,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
) -> Result<(), LoopJoinSigRejectReasonV1> {
    let require = |value| require_value(available, value);
    match operation {
        LoopJoinOperationView::ReadBinding { binding, result } => {
            let value = bindings
                .get(&binding)
                .copied()
                .ok_or(LoopJoinSigRejectReasonV1::BindingNotAvailable { binding })?;
            require(value)?;
            available.insert(result);
        }
        LoopJoinOperationView::Define { uses, result, .. } => {
            match uses {
                LoopJoinValueUses::None => {}
                LoopJoinValueUses::Two(left, right) => {
                    require(left)?;
                    require(right)?;
                }
                LoopJoinValueUses::Call { receiver, args } => {
                    if let Some(receiver) = receiver {
                        require(receiver)?;
                    }
                    for argument in args {
                        require(*argument)?;
                    }
                }
            }
            if let Some(result) = result {
                available.insert(result);
            }
        }
        LoopJoinOperationView::WriteBinding { binding, value } => {
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
