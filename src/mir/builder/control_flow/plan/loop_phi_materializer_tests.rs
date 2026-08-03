use super::*;
use crate::mir::builder::control_flow::plan::loop_phi_materializer_test_support::{
    bb, nested_resume_builder, seed_builder, seeded_builder, standard5_builder,
};
use crate::mir::builder::module_invocation_session::BuilderCoreSeedPolicyV1;
use crate::mir::builder::{
    BuilderCommitReadinessErrorV1, BuilderInvocationConfigV1, MirBuilder,
    ModuleBuilderInvocationSessionV1,
};
use crate::mir::loop_recipe_contract::{
    LoopJoinSigElaboratorV1, LoopRecipeArtifactV1, LoopRecipeVerifierV1,
};
use crate::mir::MirInstruction;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

#[path = "loop_accum_semantic_parity_tests.rs"]
mod accum_semantic_parity_tests;
#[path = "loop_recipe_producer_facade_tests.rs"]
mod recipe_producer_facade_tests;

const GOLDEN: &str = include_str!("../../../loop_recipe_contract/fixtures/accum_nested_v1.json");
pub(crate) const DIRECT_GOLDEN: &str =
    include_str!("../../../loop_recipe_contract/fixtures/accum_direct_v1.json");

fn verified_sig() -> VerifiedLoopJoinSigV1 {
    let artifact: LoopRecipeArtifactV1 = serde_json::from_str(GOLDEN).expect("golden JSON");
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("recipe shape");
    LoopJoinSigElaboratorV1::elaborate(&verified).expect("bounded JoinSig")
}

pub(crate) fn direct_verified_sig() -> VerifiedLoopJoinSigV1 {
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(DIRECT_GOLDEN).expect("direct golden JSON");
    let verified =
        LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("direct recipe shape");
    LoopJoinSigElaboratorV1::elaborate(&verified).expect("direct bounded JoinSig")
}

fn candidate_session(live: &MirBuilder) -> ModuleBuilderInvocationSessionV1 {
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut session = ModuleBuilderInvocationSessionV1::open(live, config);
    seed_builder(session.builder_mut());
    session
}

fn map_input(sig: &VerifiedLoopJoinSigV1) -> LoopLogicalToPhysicalMapInputV1 {
    use LoopJoinPortV1::*;
    let ports = vec![
        (LoopNodeKeyV1::new(0), Preheader, bb(0)),
        (LoopNodeKeyV1::new(0), Header, bb(1)),
        (LoopNodeKeyV1::new(0), Body, bb(2)),
        (LoopNodeKeyV1::new(0), After, bb(3)),
        (LoopNodeKeyV1::new(1), Preheader, bb(2)),
        (LoopNodeKeyV1::new(1), Header, bb(4)),
        (LoopNodeKeyV1::new(1), Body, bb(5)),
        (LoopNodeKeyV1::new(1), After, bb(6)),
    ];
    let port_block = |loop_key, port| {
        ports
            .iter()
            .find(|(key, candidate, _)| *key == loop_key && *candidate == port)
            .map(|(_, _, block)| *block)
            .expect("test port mapping")
    };
    let edge_paths = sig
        .as_sig()
        .loops
        .iter()
        .flat_map(|row| {
            row.edges.iter().map(|edge| {
                let from = port_block(row.key, edge.from);
                let to = port_block(row.key, edge.to);
                LoopPhysicalEdgePathV1::from_parts(row.key, edge.role, vec![from, to], from)
            })
        })
        .collect::<Vec<_>>();
    let mut predecessor_rows = std::collections::BTreeMap::<BasicBlockId, Vec<BasicBlockId>>::new();
    for path in &edge_paths {
        for pair in path.blocks.windows(2) {
            predecessor_rows.entry(pair[1]).or_default().push(pair[0]);
        }
    }
    LoopLogicalToPhysicalMapInputV1 {
        ports,
        values: vec![
            (
                LoopValueKeyV1::new(0),
                ValueId::new(10),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(3),
                ValueId::new(12),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(5),
                ValueId::new(11),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(6),
                ValueId::new(13),
                LoopValueClassV1::I64,
            ),
        ],
        destinations: vec![
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(0),
                ValueId::new(20),
            ),
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(1),
                ValueId::new(21),
            ),
        ],
        predecessors: predecessor_rows.into_iter().collect(),
        edge_paths,
    }
}

fn materializer_input(sig: &VerifiedLoopJoinSigV1) -> VerifiedLoopLogicalToPhysicalMapV1 {
    VerifiedLoopLogicalToPhysicalMapV1::try_new(sig, map_input(sig)).expect("sealed map")
}

fn direct_map_input(sig: &VerifiedLoopJoinSigV1) -> LoopLogicalToPhysicalMapInputV1 {
    use LoopJoinPortV1::*;
    let ports = vec![
        (LoopNodeKeyV1::new(0), Preheader, bb(0)),
        (LoopNodeKeyV1::new(0), Header, bb(1)),
        (LoopNodeKeyV1::new(0), Body, bb(2)),
        (LoopNodeKeyV1::new(0), After, bb(4)),
    ];
    let edge_paths = sig
        .as_sig()
        .loops
        .iter()
        .flat_map(|row| {
            row.edges.iter().map(|edge| {
                let (blocks, terminal) = match edge.role {
                    LoopJoinEdgeRoleV1::Enter => (vec![bb(0), bb(1)], bb(0)),
                    LoopJoinEdgeRoleV1::PredicateTrue => (vec![bb(1), bb(2)], bb(1)),
                    LoopJoinEdgeRoleV1::PredicateFalse => (vec![bb(1), bb(4)], bb(1)),
                    LoopJoinEdgeRoleV1::Backedge => (vec![bb(2), bb(3), bb(1)], bb(3)),
                    role => panic!("unexpected direct edge role: {role:?}"),
                };
                LoopPhysicalEdgePathV1::from_parts(row.key, edge.role, blocks, terminal)
            })
        })
        .collect::<Vec<_>>();
    LoopLogicalToPhysicalMapInputV1 {
        ports,
        values: vec![
            (
                LoopValueKeyV1::new(0),
                ValueId::new(10),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(1),
                ValueId::new(12),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(7),
                ValueId::new(11),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(10),
                ValueId::new(13),
                LoopValueClassV1::I64,
            ),
        ],
        destinations: vec![
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(0),
                ValueId::new(20),
            ),
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(1),
                ValueId::new(21),
            ),
        ],
        predecessors: vec![
            (bb(1), vec![bb(0), bb(3)]),
            (bb(2), vec![bb(1)]),
            (bb(3), vec![bb(2)]),
            (bb(4), vec![bb(1)]),
        ],
        edge_paths,
    }
}

pub(crate) fn direct_materializer_input(sig: &VerifiedLoopJoinSigV1) -> VerifiedLoopLogicalToPhysicalMapV1 {
    VerifiedLoopLogicalToPhysicalMapV1::try_new(sig, direct_map_input(sig)).expect("direct map")
}

fn nested_resume_map_input(sig: &VerifiedLoopJoinSigV1) -> LoopLogicalToPhysicalMapInputV1 {
    use LoopJoinPortV1::*;
    let ports = vec![
        (LoopNodeKeyV1::new(0), Preheader, bb(0)),
        (LoopNodeKeyV1::new(0), Header, bb(1)),
        (LoopNodeKeyV1::new(0), Body, bb(2)),
        (LoopNodeKeyV1::new(0), After, bb(9)),
        (LoopNodeKeyV1::new(1), Preheader, bb(2)),
        (LoopNodeKeyV1::new(1), Header, bb(4)),
        (LoopNodeKeyV1::new(1), Body, bb(5)),
        (LoopNodeKeyV1::new(1), After, bb(6)),
    ];
    let edge_paths = sig
        .as_sig()
        .loops
        .iter()
        .flat_map(|row| {
            row.edges.iter().map(|edge| {
                let blocks = match (row.key, edge.role) {
                    (loop_key, LoopJoinEdgeRoleV1::Enter) if loop_key == LoopNodeKeyV1::new(0) => {
                        vec![bb(0), bb(1)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::PredicateTrue)
                        if loop_key == LoopNodeKeyV1::new(0) =>
                    {
                        vec![bb(1), bb(2)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::PredicateFalse)
                        if loop_key == LoopNodeKeyV1::new(0) =>
                    {
                        vec![bb(1), bb(9)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::Continue)
                        if loop_key == LoopNodeKeyV1::new(0) =>
                    {
                        vec![bb(2), bb(7), bb(8), bb(1)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::Enter) if loop_key == LoopNodeKeyV1::new(1) => {
                        vec![bb(2), bb(4)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::BodyEntry)
                        if loop_key == LoopNodeKeyV1::new(1) =>
                    {
                        vec![bb(4), bb(5)]
                    }
                    (loop_key, LoopJoinEdgeRoleV1::Break) if loop_key == LoopNodeKeyV1::new(1) => {
                        vec![bb(5), bb(6)]
                    }
                    (loop_key, role) => {
                        panic!("unexpected nested edge loop={loop_key:?} role={role:?}")
                    }
                };
                let terminal = blocks[blocks.len() - 2];
                LoopPhysicalEdgePathV1::from_parts(row.key, edge.role, blocks, terminal)
            })
        })
        .collect::<Vec<_>>();
    LoopLogicalToPhysicalMapInputV1 {
        ports,
        values: vec![
            (
                LoopValueKeyV1::new(0),
                ValueId::new(10),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(3),
                ValueId::new(12),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(5),
                ValueId::new(11),
                LoopValueClassV1::I64,
            ),
            (
                LoopValueKeyV1::new(6),
                ValueId::new(13),
                LoopValueClassV1::I64,
            ),
        ],
        destinations: vec![
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(0),
                ValueId::new(20),
            ),
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(1),
                ValueId::new(21),
            ),
        ],
        predecessors: vec![
            (bb(1), vec![bb(0), bb(8)]),
            (bb(2), vec![bb(1)]),
            (bb(4), vec![bb(2)]),
            (bb(5), vec![bb(4)]),
            (bb(6), vec![bb(5)]),
            (bb(7), vec![bb(2), bb(6)]),
            (bb(8), vec![bb(7)]),
            (bb(9), vec![bb(1)]),
        ],
        edge_paths,
    }
}

fn remap_block(block: BasicBlockId) -> BasicBlockId {
    match block {
        block if block == bb(0) => bb(40),
        block if block == bb(1) => bb(17),
        block if block == bb(2) => bb(91),
        block if block == bb(3) => bb(73),
        block if block == bb(4) => bb(29),
        other => other,
    }
}

fn remap_value(value: ValueId) -> ValueId {
    match value {
        value if value == ValueId::new(10) => ValueId::new(201),
        value if value == ValueId::new(11) => ValueId::new(303),
        value if value == ValueId::new(12) => ValueId::new(202),
        value if value == ValueId::new(13) => ValueId::new(404),
        value if value == ValueId::new(20) => ValueId::new(505),
        value if value == ValueId::new(21) => ValueId::new(606),
        other => other,
    }
}

fn permuted_direct_map_input(sig: &VerifiedLoopJoinSigV1) -> LoopLogicalToPhysicalMapInputV1 {
    let mut input = direct_map_input(sig);
    for (_, _, block) in &mut input.ports {
        *block = remap_block(*block);
    }
    let old_predecessors = std::mem::take(&mut input.predecessors);
    input.predecessors = old_predecessors
        .into_iter()
        .map(|(block, predecessors)| {
            (
                remap_block(block),
                predecessors.into_iter().map(remap_block).collect(),
            )
        })
        .collect();
    for path in &mut input.edge_paths {
        for block in &mut path.blocks {
            *block = remap_block(*block);
        }
        path.terminal_predecessor = remap_block(path.terminal_predecessor);
    }
    for (_, value, _) in &mut input.values {
        *value = remap_value(*value);
    }
    for (_, _, destination) in &mut input.destinations {
        *destination = remap_value(*destination);
    }
    input
}

fn alpha_normalized_direct_digest(
    sig: &VerifiedLoopJoinSigV1,
    map: &VerifiedLoopLogicalToPhysicalMapV1,
) -> String {
    let mut bindings = BTreeSet::new();
    let mut values = BTreeSet::new();
    for row in &sig.as_sig().loops {
        for carrier in &row.carriers {
            bindings.insert(carrier.binding);
            values.insert(carrier.value);
        }
        for edge in &row.edges {
            for payload in &edge.payload {
                bindings.insert(payload.binding);
                values.insert(payload.value);
            }
        }
    }
    let binding_labels = bindings
        .into_iter()
        .enumerate()
        .map(|(index, key)| (key, format!("b{index}")))
        .collect::<BTreeMap<_, _>>();
    let value_labels = values
        .into_iter()
        .enumerate()
        .map(|(index, key)| (key, format!("v{index}")))
        .collect::<BTreeMap<_, _>>();
    let mut digest = String::new();
    for (loop_index, row) in sig.as_sig().loops.iter().enumerate() {
        writeln!(&mut digest, "loop=l{loop_index}").unwrap();
        for edge in &row.edges {
            let paths = map
                .edge_paths
                .get(&(row.key, edge.role))
                .expect("sealed edge path");
            let shapes = paths
                .iter()
                .map(|path| format!("len{}:terminal{}", path.blocks.len(), path.blocks.len() - 2))
                .collect::<Vec<_>>();
            let payload = edge
                .payload
                .iter()
                .map(|entry| {
                    format!(
                        "{}:{}:{:?}",
                        binding_labels[&entry.binding], value_labels[&entry.value], entry.class
                    )
                })
                .collect::<Vec<_>>();
            writeln!(
                &mut digest,
                "edge={:?}:{:?}->{:?}:paths={:?}:payload={:?}",
                edge.role, edge.from, edge.to, shapes, payload
            )
            .unwrap();
        }
    }
    for pending in map.pending_phis(sig).expect("sealed PHI rows") {
        let row = sig
            .as_sig()
            .loops
            .iter()
            .find(|row| row.key == pending.loop_key)
            .expect("pending loop row");
        let input_roles = pending
            .inputs
            .iter()
            .map(|(predecessor, value)| {
                let role = row
                    .edges
                    .iter()
                    .filter(|edge| is_header_input(edge.role))
                    .find(|edge| {
                        map.edge_paths
                            .get(&(row.key, edge.role))
                            .is_some_and(|paths| {
                                paths
                                    .iter()
                                    .any(|path| path.terminal_predecessor == *predecessor)
                            })
                    })
                    .map(|edge| format!("{:?}", edge.role))
                    .unwrap_or_else(|| "unknown".to_string());
                format!(
                    "{role}:{}",
                    value_labels[&value_key_for_physical(row, map, *value)]
                )
            })
            .collect::<Vec<_>>();
        writeln!(
            &mut digest,
            "phi:{}:{:?}:{:?}",
            binding_labels[&pending.binding], pending.class, input_roles
        )
        .unwrap();
    }
    digest
}

fn value_key_for_physical(
    row: &crate::mir::loop_recipe_contract::LoopJoinLoopV1,
    map: &VerifiedLoopLogicalToPhysicalMapV1,
    physical: ValueId,
) -> LoopValueKeyV1 {
    row.edges
        .iter()
        .flat_map(|edge| edge.payload.iter())
        .find(|payload| {
            map.values
                .get(&payload.value)
                .is_some_and(|(value, _)| *value == physical)
        })
        .map(|payload| payload.value)
        .expect("physical value has logical payload")
}

#[test]
fn map_rejects_duplicate_predecessor_before_builder_effect() {
    let sig = verified_sig();
    let mut input = map_input(&sig);
    input.predecessors[0].1.push(bb(0));
    let error = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, input).unwrap_err();
    assert!(error.to_string().contains("duplicate predecessor"));
}

#[test]
fn map_rejects_missing_edge_path_before_builder_effect() {
    let sig = verified_sig();
    let mut input = map_input(&sig);
    input.edge_paths.retain(|path| {
        !(path.loop_key == LoopNodeKeyV1::new(0) && path.role == LoopJoinEdgeRoleV1::Enter)
    });
    let error = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, input).unwrap_err();
    assert!(error.to_string().contains("missing physical edge path"));
}

#[test]
fn direct_standard5_witness_uses_step_as_header_phi_predecessor() {
    let sig = direct_verified_sig();
    let mut builder = standard5_builder();
    let receipt = materialize_loop_phis(&mut builder, &sig, direct_materializer_input(&sig))
        .expect("direct Standard5 PHI materialization");
    assert_eq!(receipt.sites().len(), 2);
    assert_eq!(
        receipt.sites()[0].inputs.as_ref(),
        &[(bb(0), ValueId::new(10)), (bb(3), ValueId::new(13))]
    );
    assert_eq!(
        receipt.sites()[1].inputs.as_ref(),
        &[(bb(0), ValueId::new(12)), (bb(3), ValueId::new(11))]
    );
}

#[test]
fn direct_readbinding_fixture_closes_dynamic_carrier_values() {
    let sig = direct_verified_sig();
    let row = sig.as_sig().loops.first().expect("direct loop row");
    let backedge = row
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
        .expect("direct backedge");
    let payload = backedge
        .payload
        .iter()
        .map(|entry| (entry.binding, entry.value))
        .collect::<Vec<_>>();
    assert_eq!(
        payload,
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(10)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(7))
        ]
    );
}

#[test]
fn direct_standard5_witness_rejects_body_to_header_shortcut() {
    let sig = direct_verified_sig();
    let mut input = direct_map_input(&sig);
    let path = input
        .edge_paths
        .iter_mut()
        .find(|path| path.role == LoopJoinEdgeRoleV1::Backedge)
        .expect("backedge path");
    path.blocks = vec![bb(2), bb(1)].into_boxed_slice();
    path.terminal_predecessor = bb(2);
    let error = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, input).unwrap_err();
    assert!(error.to_string().contains("predecessor mismatch"));
}

#[test]
fn direct_alpha_digest_ignores_physical_id_allocation() {
    let sig = direct_verified_sig();
    let left = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, direct_map_input(&sig))
        .expect("left direct map");
    let right = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, permuted_direct_map_input(&sig))
        .expect("permuted direct map");
    let left_digest = alpha_normalized_direct_digest(&sig, &left);
    let right_digest = alpha_normalized_direct_digest(&sig, &right);
    assert_eq!(left_digest, right_digest);
    assert!(!left_digest.contains("201"));
    assert!(!left_digest.contains("505"));
}

#[test]
fn nested_golden_witness_keeps_child_after_parent_resume_explicit() {
    let sig = verified_sig();
    let payload_keys = |edge: &crate::mir::loop_recipe_contract::LoopJoinEdgeV1| {
        edge.payload
            .iter()
            .map(|payload| (payload.binding, payload.value))
            .collect::<Vec<_>>()
    };
    let root = sig
        .as_sig()
        .loops
        .iter()
        .find(|row| row.key == LoopNodeKeyV1::new(0))
        .expect("root JoinSig row");
    let child = sig
        .as_sig()
        .loops
        .iter()
        .find(|row| row.key == LoopNodeKeyV1::new(1))
        .expect("child JoinSig row");
    let root_continue_edge = root
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Continue)
        .expect("root continue edge");
    let child_enter_edge = child
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Enter)
        .expect("child enter edge");
    let child_break_edge = child
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Break)
        .expect("child break edge");
    assert_eq!(
        payload_keys(child_enter_edge),
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(0)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(3)),
        ]
    );
    assert_eq!(
        payload_keys(child_break_edge),
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(0)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(6)),
        ]
    );
    assert_eq!(
        payload_keys(root_continue_edge),
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(5)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(6)),
        ]
    );
    let map = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, nested_resume_map_input(&sig))
        .expect("nested physical witness");
    let root_continue = map
        .edge_paths
        .get(&(LoopNodeKeyV1::new(0), LoopJoinEdgeRoleV1::Continue))
        .expect("root continue path");
    assert_eq!(root_continue.len(), 1);
    assert_eq!(
        root_continue[0].blocks.as_ref(),
        &[bb(2), bb(7), bb(8), bb(1)]
    );
    assert_eq!(root_continue[0].terminal_predecessor, bb(8));

    let child_break = map
        .edge_paths
        .get(&(LoopNodeKeyV1::new(1), LoopJoinEdgeRoleV1::Break))
        .expect("child break path");
    assert_eq!(child_break[0].blocks.as_ref(), &[bb(5), bb(6)]);
    assert_eq!(child_break[0].terminal_predecessor, bb(5));

    let parent_resume_paths = [vec![bb(6), bb(7)], vec![bb(7), bb(8), bb(1)]];
    for path in &parent_resume_paths {
        for pair in path.windows(2) {
            assert!(map
                .predecessors
                .get(&pair[1])
                .expect("parent-resume predecessor witness")
                .contains(&pair[0]));
        }
    }
    assert_ne!(parent_resume_paths[0].as_slice(), &[bb(6), bb(8)]);
}

#[test]
fn nested_golden_root_materializer_uses_parent_resume_terminal() {
    let sig = verified_sig();
    let mut builder = nested_resume_builder();
    let map = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, nested_resume_map_input(&sig))
        .expect("nested physical witness");
    let receipt =
        materialize_loop_phis(&mut builder, &sig, map).expect("nested root PHI materialization");
    assert_eq!(receipt.sites().len(), 2);
    assert_eq!(
        receipt.sites()[0].inputs.as_ref(),
        &[(bb(0), ValueId::new(10)), (bb(8), ValueId::new(11))]
    );
    assert_eq!(
        receipt.sites()[1].inputs.as_ref(),
        &[(bb(0), ValueId::new(12)), (bb(8), ValueId::new(13))]
    );
}

#[test]
fn materializer_emits_exact_accum_header_phis() {
    let sig = verified_sig();
    let mut builder = seeded_builder();
    let before = builder.function_state.variable_ctx.variable_map.clone();
    let receipt = materialize_loop_phis(&mut builder, &sig, materializer_input(&sig))
        .expect("PHI materialization");
    assert_eq!(receipt.sites().len(), 2);
    assert_eq!(
        receipt.sites()[0].inputs.as_ref(),
        &[(bb(0), ValueId::new(10)), (bb(2), ValueId::new(11))]
    );
    assert_eq!(
        receipt.sites()[1].inputs.as_ref(),
        &[(bb(0), ValueId::new(12)), (bb(2), ValueId::new(13))]
    );
    let function = builder.function_state.current_function.as_ref().unwrap();
    let phis = function
        .get_block(bb(1))
        .unwrap()
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } => Some((*dst, inputs.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(phis.len(), 2);
    assert_eq!(builder.function_state.variable_ctx.variable_map, before);
}

#[test]
fn provisional_failure_rolls_back_empty_phi() {
    let sig = verified_sig();
    let mut builder = seeded_builder();
    let map = materializer_input(&sig);
    let error = materialize_impl(&mut builder, &sig, map, Some(0)).unwrap_err();
    assert!(error.to_string().contains("txn_abort"));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(bb(1))
        .unwrap()
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
}

#[test]
fn stale_cfg_witness_rejects_before_phi_effect() {
    let sig = verified_sig();
    let mut builder = seeded_builder();
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(bb(1))
        .unwrap()
        .predecessors
        .remove(&bb(2));
    let error = materialize_loop_phis(&mut builder, &sig, materializer_input(&sig)).unwrap_err();
    assert!(error.to_string().contains("sealed predecessor mismatch"));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(bb(1))
        .unwrap()
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
}

#[test]
fn fresh_builder_reuse_is_deterministic() {
    let sig = verified_sig();
    let mut left = seeded_builder();
    let mut right = seeded_builder();
    let left_receipt = materialize_loop_phis(&mut left, &sig, materializer_input(&sig)).unwrap();
    let right_receipt = materialize_loop_phis(&mut right, &sig, materializer_input(&sig)).unwrap();
    assert_eq!(left_receipt, right_receipt);
}

#[test]
fn candidate_abort_after_m6b_effect_allows_fresh_retry() {
    let live = MirBuilder::new();
    let before = live.loop_candidate_test_fingerprint();
    let sig = verified_sig();
    let mut first = candidate_session(&live);
    let first_receipt = materialize_loop_phis(first.builder_mut(), &sig, materializer_input(&sig))
        .expect("first candidate materialization");
    let first_error = first.prepare_external_commit().unwrap_err();
    assert_eq!(
        first_error,
        BuilderCommitReadinessErrorV1::CurrentFunctionOpen
    );
    assert_eq!(live.loop_candidate_test_fingerprint(), before);

    let mut second = candidate_session(&live);
    let second_receipt =
        materialize_loop_phis(second.builder_mut(), &sig, materializer_input(&sig))
            .expect("fresh candidate materialization");
    assert_eq!(first_receipt, second_receipt);
    let second_error = second.prepare_external_commit().unwrap_err();
    assert_eq!(
        second_error,
        BuilderCommitReadinessErrorV1::CurrentFunctionOpen
    );
    assert_eq!(live.loop_candidate_test_fingerprint(), before);
}
