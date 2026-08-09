use std::collections::{BTreeMap, BTreeSet};

use super::model::{
    LoopJoinEdge, LoopJoinEdgeRoleV1, LoopJoinLoop, LoopJoinPayload, LoopJoinPortBinding,
    LoopJoinPortV1, LoopJoinSigRejectReasonV1,
};
use super::recipe_view::LoopJoinExitView;

pub(super) fn loop_exit_edge<C>(
    exit: LoopJoinExitView,
    payload: Vec<LoopJoinPayload<C>>,
) -> LoopJoinEdge<C> {
    match exit {
        LoopJoinExitView::Break { .. } => LoopJoinEdge {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::Break,
            payload,
        },
        LoopJoinExitView::Continue { .. } => LoopJoinEdge {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Continue,
            payload,
        },
        LoopJoinExitView::Return { .. } => LoopJoinEdge {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::FunctionExit,
            role: LoopJoinEdgeRoleV1::Return,
            payload,
        },
    }
}

pub(in crate::mir::loop_recipe_contract) fn port_bindings<C: Copy + Eq>(
    rows: &[LoopJoinLoop<C>],
) -> Result<Vec<LoopJoinPortBinding<C>>, LoopJoinSigRejectReasonV1> {
    let mut classes = BTreeMap::new();
    let mut edge_sets =
        BTreeMap::<(super::super::ids::LoopNodeKeyV1, LoopJoinPortV1), BTreeSet<_>>::new();

    for row in rows {
        for edge in &row.edges {
            let port = match edge.to {
                LoopJoinPortV1::Header | LoopJoinPortV1::After => edge.to,
                _ => continue,
            };
            let mut bindings = BTreeSet::new();
            for payload in &edge.payload {
                if !bindings.insert(payload.binding) {
                    return Err(LoopJoinSigRejectReasonV1::DuplicatePortBinding {
                        loop_key: row.key,
                        port,
                        binding: payload.binding,
                    });
                }
                let key = (row.key, port, payload.binding);
                if let Some(existing) = classes.get(&key) {
                    if *existing != payload.class {
                        return Err(LoopJoinSigRejectReasonV1::PortBindingClassMismatch {
                            loop_key: row.key,
                            port,
                            binding: payload.binding,
                        });
                    }
                } else {
                    classes.insert(key, payload.class);
                }
            }
            let set_key = (row.key, port);
            if let Some(expected) = edge_sets.get(&set_key) {
                if expected != &bindings {
                    return Err(LoopJoinSigRejectReasonV1::PortBindingSetMismatch {
                        loop_key: row.key,
                        port,
                    });
                }
            } else {
                edge_sets.insert(set_key, bindings);
            }
        }
    }

    Ok(classes
        .into_iter()
        .map(|((loop_key, port, binding), class)| LoopJoinPortBinding {
            loop_key,
            port,
            binding,
            class,
        })
        .collect())
}
