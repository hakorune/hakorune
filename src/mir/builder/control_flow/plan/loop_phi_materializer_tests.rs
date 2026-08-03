use super::*;
use crate::mir::builder::control_flow::plan::loop_phi_materializer_test_support::{
    bb, seed_builder, seeded_builder,
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

const GOLDEN: &str = include_str!("../../../loop_recipe_contract/fixtures/accum_nested_v1.json");

fn verified_sig() -> VerifiedLoopJoinSigV1 {
    let artifact: LoopRecipeArtifactV1 = serde_json::from_str(GOLDEN).expect("golden JSON");
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("recipe shape");
    LoopJoinSigElaboratorV1::elaborate(&verified).expect("bounded JoinSig")
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
