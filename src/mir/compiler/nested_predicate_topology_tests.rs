use super::nested_predicate_producer::produce_nested_predicate_recipe_v1;
use super::nested_predicate_producer_tests::{nested_function, projection_for};
use super::nested_predicate_topology::{
    issue_nested_predicate_physical_topology_v1, NestedCarrierVisibilityV1,
    NestedPhysicalEdgeRoleV1, NestedPhysicalExpansionStepV1, NestedPhysicalNodeRefV1,
    NestedPhysicalStageV1,
};
use crate::mir::loop_recipe_contract::{LoopJoinEdgeRoleV1, LoopNodeKeyV1};

const ROOT: LoopNodeKeyV1 = LoopNodeKeyV1::new(0);
const CHILD: LoopNodeKeyV1 = LoopNodeKeyV1::new(1);

#[test]
fn nested_topology_seals_two_standard5_port_sets_and_resume() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    let topology = issue_nested_predicate_physical_topology_v1(product).expect("nested topology");
    assert_eq!(topology.ports().len(), 10);
    assert_eq!(topology.edges().len(), 11);
    assert_eq!(topology.parent_resume().parent_loop, ROOT);
    assert_eq!(topology.parent_resume().child_loop, CHILD);
    assert_eq!(
        topology.child_preheader_alias().alias.stage,
        NestedPhysicalStageV1::Preheader
    );
    assert_eq!(
        topology.child_preheader_alias().canonical.stage,
        NestedPhysicalStageV1::Body
    );
}

#[test]
fn nested_topology_expands_root_backedge_through_child_resume() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    let topology = issue_nested_predicate_physical_topology_v1(product).expect("nested topology");
    let root_expansion = topology
        .logical_expansions()
        .iter()
        .find(|expansion| expansion.loop_key == ROOT)
        .expect("root expansion");
    assert_eq!(root_expansion.logical_role, LoopJoinEdgeRoleV1::Backedge);
    assert!(matches!(
        root_expansion.steps[0],
        NestedPhysicalExpansionStepV1::ChildLoop(child)
            if child == CHILD
    ));
    assert!(topology.edges().iter().all(|edge| {
        !(edge.role == NestedPhysicalEdgeRoleV1::StepToHeader
            && matches!(edge.from, NestedPhysicalNodeRefV1::Port(port)
                if port.loop_key == ROOT
                    && port.stage == NestedPhysicalStageV1::Body))
    }));
}

#[test]
fn nested_topology_drops_child_j_at_parent_resume() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    let topology = issue_nested_predicate_physical_topology_v1(product).expect("nested topology");
    let carriers = topology.carriers();
    assert_eq!(
        carriers[0].visibility,
        NestedCarrierVisibilityV1::ParentVisible
    );
    assert_eq!(
        carriers[1].visibility,
        NestedCarrierVisibilityV1::ParentVisible
    );
    assert_eq!(
        carriers[2].visibility,
        NestedCarrierVisibilityV1::ChildLocal
    );
    assert!(carriers[0].resume.is_some());
    assert!(carriers[1].resume.is_some());
    assert!(carriers[2].resume.is_none());
    assert!(!topology
        .predecessor_seals()
        .iter()
        .any(|seal| seal.incoming.iter().any(|edge| {
            matches!(edge.from, NestedPhysicalNodeRefV1::Port(port)
                if port.loop_key == CHILD
                    && port.stage == NestedPhysicalStageV1::After)
                && matches!(edge.to, NestedPhysicalNodeRefV1::Port(port)
                    if port.loop_key == ROOT
                        && port.stage == NestedPhysicalStageV1::After)
        })));
}

#[test]
fn nested_topology_seals_source_roles_and_symbolic_predecessors() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    let topology = issue_nested_predicate_physical_topology_v1(product).expect("nested topology");
    assert_eq!(topology.source_roles().len(), 10);
    assert_eq!(topology.predecessor_seals().len(), 8);
    assert!(topology
        .predecessor_seals()
        .iter()
        .all(|seal| !seal.incoming.is_empty()));
}
