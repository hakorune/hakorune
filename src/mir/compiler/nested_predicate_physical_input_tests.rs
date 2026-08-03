use super::nested_predicate_physical_input::{
    NestedPhysicalBlockProjectionRejectV1, VerifiedNestedPhysicalBlockProjectionV1,
    VerifiedNestedPhysicalCandidateInputV1,
};
use super::nested_predicate_producer::produce_nested_predicate_recipe_v1;
use super::nested_predicate_producer_tests::{nested_function, projection_for};
use super::nested_predicate_topology::{
    issue_nested_predicate_physical_emission_input_v1, NestedPhysicalNodeRefV1,
    NestedPhysicalPortRefV1, NestedPhysicalStageV1, NestedPortAliasV1,
};
use crate::mir::resolved_semantics::{loop_execution_frame_key_for_test, FunctionOwnerIssuerV1};
use crate::mir::BasicBlockId;

fn emission_input() -> super::nested_predicate_topology::VerifiedNestedPhysicalEmissionInputV1 {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    issue_nested_predicate_physical_emission_input_v1(product).expect("nested emission input")
}

fn blocks(
    input: &super::nested_predicate_topology::VerifiedNestedPhysicalEmissionInputV1,
) -> VerifiedNestedPhysicalBlockProjectionV1 {
    VerifiedNestedPhysicalBlockProjectionV1::try_new(
        input,
        input.topology().owner(),
        input.topology().root_frame_key(),
        BasicBlockId::new(100),
        [
            BasicBlockId::new(101),
            BasicBlockId::new(102),
            BasicBlockId::new(103),
            BasicBlockId::new(104),
        ],
        [
            BasicBlockId::new(105),
            BasicBlockId::new(106),
            BasicBlockId::new(107),
            BasicBlockId::new(108),
        ],
        BasicBlockId::new(109),
    )
    .expect("block projection")
}

#[test]
fn p0_preserves_semantic_pair_and_maps_alias_and_resume() {
    let input = emission_input();
    let block_projection = blocks(&input);
    assert_eq!(block_projection.symbolic_node_count(), 11);
    assert_eq!(block_projection.unique_physical_block_count(), 10);
    assert_eq!(
        block_projection.block(NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1 {
            loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(1),
            stage: NestedPhysicalStageV1::Preheader,
        },)),
        BasicBlockId::new(102)
    );
    assert_ne!(
        block_projection.block(NestedPhysicalNodeRefV1::ParentResume(
            input.topology().parent_resume()
        )),
        block_projection.block(NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1 {
            loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            stage: NestedPhysicalStageV1::After,
        }))
    );
    let candidate = VerifiedNestedPhysicalCandidateInputV1::new(input, block_projection);
    assert_eq!(candidate.emission().recipe().as_recipe().loops.len(), 2);
    assert_eq!(candidate.emission().join_sig().as_sig().loops.len(), 2);
}

#[test]
fn p0_rejects_foreign_owner_and_frame() {
    let input = emission_input();
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("issuer");
    let foreign = issuer.issue().expect("foreign owner");
    let error = VerifiedNestedPhysicalBlockProjectionV1::try_new(
        &input,
        foreign,
        input.topology().root_frame_key(),
        BasicBlockId::new(100),
        [
            BasicBlockId::new(101),
            BasicBlockId::new(102),
            BasicBlockId::new(103),
            BasicBlockId::new(104),
        ],
        [
            BasicBlockId::new(105),
            BasicBlockId::new(106),
            BasicBlockId::new(107),
            BasicBlockId::new(108),
        ],
        BasicBlockId::new(109),
    )
    .unwrap_err();
    assert_eq!(error, NestedPhysicalBlockProjectionRejectV1::OwnerMismatch);

    let error = VerifiedNestedPhysicalBlockProjectionV1::try_new(
        &input,
        input.topology().owner(),
        &loop_execution_frame_key_for_test(),
        BasicBlockId::new(100),
        [
            BasicBlockId::new(101),
            BasicBlockId::new(102),
            BasicBlockId::new(103),
            BasicBlockId::new(104),
        ],
        [
            BasicBlockId::new(105),
            BasicBlockId::new(106),
            BasicBlockId::new(107),
            BasicBlockId::new(108),
        ],
        BasicBlockId::new(109),
    )
    .unwrap_err();
    assert_eq!(error, NestedPhysicalBlockProjectionRejectV1::FrameMismatch);
}

#[test]
fn p0_rejects_duplicate_block_and_alias_mismatch() {
    let input = emission_input();
    let error = VerifiedNestedPhysicalBlockProjectionV1::try_new(
        &input,
        input.topology().owner(),
        input.topology().root_frame_key(),
        BasicBlockId::new(100),
        [
            BasicBlockId::new(101),
            BasicBlockId::new(102),
            BasicBlockId::new(103),
            BasicBlockId::new(104),
        ],
        [
            BasicBlockId::new(105),
            BasicBlockId::new(106),
            BasicBlockId::new(107),
            BasicBlockId::new(104),
        ],
        BasicBlockId::new(109),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        NestedPhysicalBlockProjectionRejectV1::DuplicatePhysicalBlock(_)
    ));

    let alias = NestedPortAliasV1 {
        alias: NestedPhysicalPortRefV1 {
            loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(1),
            stage: NestedPhysicalStageV1::Preheader,
        },
        canonical: NestedPhysicalPortRefV1 {
            loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            stage: NestedPhysicalStageV1::After,
        },
    };
    let error = VerifiedNestedPhysicalBlockProjectionV1::try_new_with_alias_for_test(
        &input,
        input.topology().owner(),
        input.topology().root_frame_key(),
        BasicBlockId::new(100),
        [
            BasicBlockId::new(101),
            BasicBlockId::new(102),
            BasicBlockId::new(103),
            BasicBlockId::new(104),
        ],
        [
            BasicBlockId::new(105),
            BasicBlockId::new(106),
            BasicBlockId::new(107),
            BasicBlockId::new(108),
        ],
        BasicBlockId::new(109),
        alias,
    )
    .unwrap_err();
    assert_eq!(
        error,
        NestedPhysicalBlockProjectionRejectV1::TopologyAliasMismatch
    );
}
