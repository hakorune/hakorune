//! The same source SUM SCC closure, including original binary transfers.
use super::*;

/// Kosaraju condensation over union AND conditional-result dependencies. Each
/// component must be grounded independently; upstream atoms cannot rescue an
/// empty downstream cycle. Requirements constrain values, they do not union.
pub(super) fn solve(nodes: &[Node]) -> Vec<Option<SourceValueSumV1>> {
    fn postorder(node: usize, nodes: &[Node], seen: &mut BTreeSet<usize>, order: &mut Vec<usize>) {
        if !seen.insert(node) {
            return;
        }
        for dependency in nodes[node].dependencies() {
            postorder(dependency, nodes, seen, order);
        }
        order.push(node);
    }
    fn assign(
        node: usize,
        reverse: &[Vec<usize>],
        component: usize,
        groups: &mut [usize],
        members: &mut Vec<usize>,
    ) {
        if groups[node] != usize::MAX {
            return;
        }
        groups[node] = component;
        members.push(node);
        for dependency in &reverse[node] {
            assign(*dependency, reverse, component, groups, members);
        }
    }
    let mut order = Vec::new();
    let mut seen = BTreeSet::new();
    let mut reverse = vec![Vec::new(); nodes.len()];
    for (node, row) in nodes.iter().enumerate() {
        for dependency in row.dependencies() {
            reverse[dependency].push(node);
        }
        postorder(node, nodes, &mut seen, &mut order);
    }
    let mut groups = vec![usize::MAX; nodes.len()];
    let mut components = Vec::new();
    for node in order.into_iter().rev() {
        if groups[node] == usize::MAX {
            let mut members = Vec::new();
            assign(node, &reverse, components.len(), &mut groups, &mut members);
            components.push(members);
        }
    }
    fn close(
        component: usize,
        components: &[Vec<usize>],
        groups: &[usize],
        nodes: &[Node],
        done: &mut BTreeSet<usize>,
        values: &mut [Option<SourceValueSumV1>],
    ) {
        if !done.insert(component) {
            return;
        }
        let members = &components[component];
        let mut valid = true;
        let mut grounded = false;
        for node in members {
            valid &= !nodes[*node].invalid;
            grounded |= nodes[*node].unconditional;
            for dependency in nodes[*node]
                .dependencies()
                .filter(|dependency| groups[*dependency] != component)
            {
                close(groups[dependency], components, groups, nodes, done, values);
                valid &= values[dependency].is_some();
                if nodes[*node].union.contains(&dependency) {
                    grounded |= values[dependency].is_some();
                }
            }
        }
        // A conditional output can ground only after ALL premises were
        // independently proved outside this component. A spare literal must
        // never justify its other, recursively assumed I64 premise.
        grounded |= members.iter().any(|node| {
            !nodes[*node].atoms.is_empty()
                && nodes[*node].required_i64.iter().all(|dependency| {
                    groups[*dependency] != component
                        && values[*dependency]
                            .as_ref()
                            .is_some_and(SourceValueSumV1::signed_only)
                })
                && nodes[*node].required_text.iter().all(|dependency| {
                    groups[*dependency] != component
                        && values[*dependency]
                            .as_ref()
                            .is_some_and(SourceValueSumV1::text_only)
                })
        });
        grounded |= members.iter().any(|node| {
            nodes[*node].binary.as_ref().is_some_and(|binary| {
                binary
                    .operands
                    .iter()
                    .all(|dependency| groups[*dependency] != component)
                    && match (&values[binary.operands[0]], &values[binary.operands[1]]) {
                        (Some(lhs), Some(rhs)) => binary.transfer(&lhs.0, &rhs.0).is_some(),
                        _ => false,
                    }
            })
        });
        if !valid || !grounded {
            return;
        }
        let mut candidate: BTreeMap<_, _> = members
            .iter()
            .map(|node| (*node, nodes[*node].atoms.clone()))
            .collect();
        loop {
            let mut changed = false;
            for node in members {
                let mut union = candidate[node].clone();
                for dependency in &nodes[*node].union {
                    union.extend(
                        if groups[*dependency] == component {
                            candidate[dependency].iter()
                        } else {
                            values[*dependency]
                                .as_ref()
                                .expect("closed external dependency")
                                .0
                                .iter()
                        }
                        .cloned(),
                    );
                }
                if let Some(binary) = &nodes[*node].binary {
                    let domain = |dependency: usize| {
                        if groups[dependency] == component {
                            &candidate[&dependency]
                        } else {
                            &values[dependency]
                                .as_ref()
                                .expect("closed external dependency")
                                .0
                        }
                    };
                    if let Some(result) =
                        binary.transfer(domain(binary.operands[0]), domain(binary.operands[1]))
                    {
                        union.extend(result);
                    }
                }
                if union != candidate[node] {
                    candidate.insert(*node, union);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for node in members {
            let value = SourceValueSumV1(candidate[node].clone());
            valid &= !value.0.is_empty()
                && nodes[*node]
                    .constraints
                    .iter()
                    .all(|name| value.permits_object(name));
            valid &= nodes[*node].binary.as_ref().is_none_or(|binary| {
                let domain = |dependency: usize| {
                    if groups[dependency] == component {
                        &candidate[&dependency]
                    } else {
                        &values[dependency]
                            .as_ref()
                            .expect("closed external dependency")
                            .0
                    }
                };
                binary
                    .transfer(domain(binary.operands[0]), domain(binary.operands[1]))
                    .is_some()
            });
            valid &= nodes[*node].required_i64.iter().all(|dependency| {
                if groups[*dependency] == component {
                    SourceValueSumV1(candidate[dependency].clone()).signed_only()
                } else {
                    values[*dependency]
                        .as_ref()
                        .is_some_and(SourceValueSumV1::signed_only)
                }
            });
            valid &= nodes[*node].required_text.iter().all(|dependency| {
                if groups[*dependency] == component {
                    SourceValueSumV1(candidate[dependency].clone()).text_only()
                } else {
                    values[*dependency]
                        .as_ref()
                        .is_some_and(SourceValueSumV1::text_only)
                }
            });
        }
        if valid {
            for node in members {
                values[*node] = Some(SourceValueSumV1(candidate.remove(node).unwrap()));
            }
        }
    }
    let mut values = vec![None; nodes.len()];
    let mut done = BTreeSet::new();
    for component in 0..components.len() {
        close(
            component,
            &components,
            &groups,
            nodes,
            &mut done,
            &mut values,
        );
    }
    values
}
