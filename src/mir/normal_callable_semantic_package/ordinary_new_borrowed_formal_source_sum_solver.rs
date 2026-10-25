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
    fn domain<'a>(
        dependency: usize,
        component: usize,
        groups: &[usize],
        candidate: &'a BTreeMap<usize, BTreeSet<super::SourceValueAlternativeV1>>,
        values: &'a [Option<SourceValueSumV1>],
    ) -> Option<&'a BTreeSet<super::SourceValueAlternativeV1>> {
        if groups[dependency] == component {
            Some(&candidate[&dependency])
        } else {
            values[dependency].as_ref().map(|value| &value.0)
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
            }
            for dependency in nodes[*node]
                .union
                .iter()
                .chain(&nodes[*node].required_i64)
                .chain(&nodes[*node].required_text)
                .chain(nodes[*node].binary.iter().map(|binary| &binary.operands[0]))
                .filter(|dependency| groups[**dependency] != component)
            {
                valid &= values[*dependency].is_some();
                if nodes[*node].union.contains(dependency) {
                    grounded |= values[*dependency].is_some();
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
                let lhs = binary.operands[0];
                if groups[lhs] == component {
                    return false;
                }
                let Some(lhs_value) = values[lhs].as_ref() else {
                    return false;
                };
                let rhs = binary.operands[1];
                (!binary.requires_rhs_domain(&lhs_value.0) || groups[rhs] != component)
                    && binary
                        .transfer(&lhs_value.0, values[rhs].as_ref().map(|value| &value.0))
                        .is_some()
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
                    if let Some(lhs) =
                        domain(binary.operands[0], component, groups, &candidate, values)
                    {
                        let rhs = domain(binary.operands[1], component, groups, &candidate, values);
                        if let Some(result) = binary.transfer(lhs, rhs) {
                            union.extend(result);
                        }
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
                domain(binary.operands[0], component, groups, &candidate, values).is_some_and(
                    |lhs| {
                        binary
                            .transfer(
                                lhs,
                                domain(binary.operands[1], component, groups, &candidate, values),
                            )
                            .is_some()
                    },
                )
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

#[cfg(test)]
mod tests {
    use super::*;
    fn atom(value: SourceValueAlternativeV1) -> Node {
        Node {
            atoms: BTreeSet::from([value]),
            unconditional: true,
            ..Default::default()
        }
    }
    #[test]
    fn source_sum_components_require_independent_ground_and_all_i64_premises() {
        use SourceValueAlternativeV1::{Other, SignedI64, Text};
        // P=Text|Q, Q=R, R=Q: upstream Text cannot rescue a seedless SCC.
        let mut p = atom(Text);
        p.union.push(1);
        let mut q = Node::default();
        q.union.push(2);
        let mut r = Node::default();
        r.union.push(1);
        let mut nodes = vec![p, q, r];
        assert!(solve(&nodes).iter().all(Option::is_none));
        nodes[1].atoms.insert(Text);
        nodes[1].unconditional = true;
        assert!(solve(&nodes)
            .iter()
            .all(|value| value.as_ref().unwrap().0 == BTreeSet::from([Text])));
        // P=f(P,7): a spare closed argument must not prove its cyclic premise.
        let p = Node {
            union: vec![1],
            ..Default::default()
        };
        let f = Node {
            atoms: BTreeSet::from([SignedI64]),
            required_i64: vec![0, 2],
            ..Default::default()
        };
        let mut nodes = vec![p, f, atom(SignedI64)];
        assert!(solve(&nodes)[0].is_none());
        // f(7) has independently closed premises and a conditional result.
        nodes[1].required_i64 = vec![2];
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());
        // An independently grounded recursive I64 cohort is different.
        nodes[0].union.push(2);
        nodes[1].required_i64 = vec![0, 2];
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());
        nodes[2] = atom(Other);
        assert!(
            solve(&nodes)[0].is_none(),
            "all I64 premises must close exactly"
        );
        nodes[2] = Node {
            invalid: true,
            ..Default::default()
        };
        assert!(
            solve(&nodes)[0].is_none(),
            "unknown siblings survive condensation"
        );
        // P=P.substring(0,1): closed bounds cannot ground receiver Text.
        let mut nodes = vec![
            Node {
                union: vec![1],
                ..Default::default()
            },
            Node {
                atoms: BTreeSet::from([Text]),
                required_text: vec![0],
                required_i64: vec![2],
                ..Default::default()
            },
            atom(SignedI64),
        ];
        assert!(solve(&nodes)[0].is_none());
        nodes[0].union.push(3);
        nodes.push(atom(Text));
        assert!(
            solve(&nodes)[0].as_ref().unwrap().text_only(),
            "independent Text grounds its recursive cohort"
        );
        nodes[3] = atom(Other);
        assert!(
            solve(&nodes)[0].is_none(),
            "mixed receiver is not Text-only"
        );
        nodes[3] = atom(Text);
        nodes[2] = atom(Text);
        assert!(solve(&nodes)[0].is_none(), "Text bounds are not signed I64");
        // A binary contributes no speculative atom. External operands may
        // ground it; a cyclic operand plus a spare literal may not.
        use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
        let binary = |lhs, rhs, operator| {
            use crate::mir::resolved_semantics::{
                FunctionOwnerIssuerV1, ResolvedBinaryExpressionSourceV1, SourceNodeSiteV1,
                SourcePathSegmentV1 as Path,
            };
            let site = |roles| SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(roles));
            Node {
                binary: Some(SourceBinaryTransferV1 {
                    owner: FunctionOwnerIssuerV1::new_for_compilation()
                        .unwrap()
                        .issue()
                        .unwrap(),
                    source: ResolvedBinaryExpressionSourceV1::from_parts_for_test(
                        site(vec![Path::Body(0)]),
                        operator,
                        site(vec![Path::Body(0), Path::Lhs]),
                        site(vec![Path::Body(0), Path::Rhs]),
                    ),
                    operands: [lhs, rhs],
                }),
                ..Default::default()
            }
        };
        let nodes = vec![binary(1, 2, Op::Add), atom(Text), atom(Text)];
        assert!(solve(&nodes)[0].as_ref().unwrap().text_only());
        let mut nodes = vec![
            Node {
                union: vec![1],
                ..Default::default()
            },
            binary(0, 2, Op::Add),
            atom(Text),
        ];
        assert!(
            solve(&nodes)[0].is_none(),
            "cyclic concatenation has no initial source value"
        );
        nodes[0].union.push(3);
        nodes.push(atom(Text));
        assert!(solve(&nodes)[0].as_ref().unwrap().text_only());
        nodes[2].atoms.insert(Other);
        assert!(
            solve(&nodes)[0].as_ref().unwrap().text_only(),
            "String + any returns Text on its normal path"
        );
        let unknown_rhs = vec![
            binary(0, 1, Op::Add),
            atom(Text),
            Node {
                invalid: true,
                ..Default::default()
            },
        ];
        assert!(
            solve(&unknown_rhs)[0]
                .as_ref()
                .is_some_and(SourceValueSumV1::text_only),
            "a definite String left operand does not need a closed RHS type domain"
        );
        let reverse = vec![binary(1, 2, Op::Add), atom(Other), atom(Text)];
        assert!(
            solve(&reverse)[0].is_none(),
            "the right Text operand does not reverse String + any"
        );
        // This refusal is multiplication, not the now-proven String+Dynamic Add.
        nodes[1] = binary(0, 2, Op::Multiply);
        nodes[2] = atom(SignedI64);
        assert!(solve(&nodes)[0].is_none(), "String*Dynamic is not selected");
        nodes[3] = atom(SignedI64);
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());
    }
}
