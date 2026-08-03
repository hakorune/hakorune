//! Pure logical join obligations for a verified recursive Loop recipe.
//!
//! This module is deliberately caller-zero.  It consumes only the portable
//! recipe and emits deterministic logical rows; physical identities and MIR
//! mutation belong to the later consumer.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
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
        let mut available = recipe.inputs.iter().copied().collect::<BTreeSet<_>>();
        let mut bindings = BTreeMap::new();
        seed_carriers(recipe, verified.root_loop(), &mut bindings, &mut available);
        let _ = elaborate_loop(
            recipe,
            verified.root_loop(),
            &mut bindings,
            &mut available,
            &mut rows,
        )?;
        rows.sort_by_key(|row| row.key);
        Ok(VerifiedLoopJoinSigV1(LoopJoinSigV1 { loops: rows }))
    }
}

#[derive(Debug)]
struct Flow {
    bindings: BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: BTreeSet<LoopValueKeyV1>,
    exit: Option<(LoopItemKeyV1, LoopExitKindV1)>,
}

fn elaborate_loop(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    inherited: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoopV1>,
) -> Result<Flow, LoopJoinSigRejectReasonV1> {
    let node = recipe
        .loops
        .get(key.raw() as usize)
        .expect("verified recipe has canonical loop keys");
    let mut local = inherited.clone();
    let mut local_available = available.clone();
    seed_carriers(recipe, key, &mut local, &mut local_available);
    if node.parent.is_some() && matches!(node.condition, LoopConditionV1::Predicate { .. }) {
        return Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate { loop_key: key });
    }
    let mut edges = vec![LoopJoinEdgeV1 {
        from: LoopJoinPortV1::Preheader,
        to: LoopJoinPortV1::Header,
        role: LoopJoinEdgeRoleV1::Enter,
        payload: visible_payloads(recipe, key, &local)?,
    }];

    if let LoopConditionV1::Predicate { block, value } = node.condition {
        let flow = process_block(recipe, key, block, &mut local, &mut local_available, rows)?;
        if flow.exit.is_some() {
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
    let propagated_exit = match body_flow.exit {
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
    };

    let carriers = payloads(recipe, key, &body_flow.bindings)?;
    rows.push(LoopJoinLoopV1 {
        key,
        parent: node.parent,
        carriers,
        edges,
    });
    *inherited = body_flow.bindings.clone();
    *available = body_flow.available.clone();
    Ok(Flow {
        bindings: body_flow.bindings,
        available: body_flow.available,
        exit: propagated_exit,
    })
}

fn process_block(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    key: LoopBlockKeyV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
    rows: &mut Vec<LoopJoinLoopV1>,
) -> Result<Flow, LoopJoinSigRejectReasonV1> {
    let block = recipe
        .blocks
        .get(key.raw() as usize)
        .expect("verified recipe has canonical block keys");
    let mut flow = Flow {
        bindings: bindings.clone(),
        available: available.clone(),
        exit: None,
    };
    for item_key in &block.items {
        if flow.exit.is_some() {
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
                )?;
                flow.bindings = child.bindings;
                flow.available = child.available;
                flow.exit = child.exit;
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
                )?;
                let (else_bindings, else_available, else_exit) =
                    if let Some(else_block) = else_block {
                        let mut else_bindings = flow.bindings.clone();
                        let mut else_available = flow.available.clone();
                        let else_flow = process_block(
                            recipe,
                            owner_loop,
                            else_block,
                            &mut else_bindings,
                            &mut else_available,
                            rows,
                        )?;
                        (else_flow.bindings, else_flow.available, else_flow.exit)
                    } else {
                        (flow.bindings.clone(), flow.available.clone(), None)
                    };
                if then_flow.exit != else_exit
                    || then_flow.bindings != else_bindings
                    || then_flow.available != else_available
                {
                    return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: *item_key });
                }
                flow.bindings = then_flow.bindings;
                flow.available = then_flow.available;
                flow.exit = then_flow.exit;
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

fn visible_payloads(
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
