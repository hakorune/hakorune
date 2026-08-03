//! Caller-zero symbolic physical topology for bounded Nested Predicate.
//!
//! This is the bridge between source-bound semantic products and the later
//! canonical-session adapter. It names ports, edges, aliases, and predecessor
//! seals but never allocates MIR blocks/values or writes PHI/SSA state.

use crate::mir::loop_recipe_contract::{
    LoopJoinEdgeRoleV1, LoopNodeKeyV1, LoopRecipeV1, VerifiedLoopJoinSigV1, VerifiedLoopRecipeV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ScopeId, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::nested_predicate_producer::VerifiedNestedPredicateRecipeProductV1;
use super::nested_predicate_projection::{
    NestedChildBodyRoleV1, NestedObservedRecurrenceOwnerV1, NestedRootBodyRoleV1,
};
use super::nested_predicate_source_handoff::VerifiedNestedPhysicalSourceHandoffV1;

const ROOT: LoopNodeKeyV1 = LoopNodeKeyV1::new(0);
const CHILD: LoopNodeKeyV1 = LoopNodeKeyV1::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedPhysicalStageV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NestedPhysicalPortRefV1 {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) stage: NestedPhysicalStageV1,
}

impl NestedPhysicalPortRefV1 {
    const fn new(loop_key: LoopNodeKeyV1, stage: NestedPhysicalStageV1) -> Self {
        Self { loop_key, stage }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NestedParentResumePortV1 {
    pub(crate) parent_loop: LoopNodeKeyV1,
    pub(crate) child_loop: LoopNodeKeyV1,
}

impl NestedParentResumePortV1 {
    const fn new(parent_loop: LoopNodeKeyV1, child_loop: LoopNodeKeyV1) -> Self {
        Self {
            parent_loop,
            child_loop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedPhysicalNodeRefV1 {
    Port(NestedPhysicalPortRefV1),
    ParentResume(NestedParentResumePortV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NestedPortAliasV1 {
    pub(crate) alias: NestedPhysicalPortRefV1,
    pub(crate) canonical: NestedPhysicalPortRefV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedPhysicalEdgeRoleV1 {
    Enter,
    PredicateTrue,
    PredicateFalse,
    BodyToStep,
    StepToHeader,
    ChildAfterToParentResume,
    ParentResumeToStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NestedPhysicalEdgeRefV1 {
    pub(crate) from: NestedPhysicalNodeRefV1,
    pub(crate) to: NestedPhysicalNodeRefV1,
    pub(crate) role: NestedPhysicalEdgeRoleV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedPhysicalExpansionStepV1 {
    Edge(NestedPhysicalEdgeRefV1),
    ChildLoop(LoopNodeKeyV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedLogicalExpansionV1 {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) logical_role: LoopJoinEdgeRoleV1,
    pub(crate) steps: Box<[NestedPhysicalExpansionStepV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedNestedTopologyPredecessorSealV1 {
    pub(crate) target: NestedPhysicalNodeRefV1,
    pub(crate) incoming: Box<[NestedPhysicalEdgeRefV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NestedCarrierVisibilityV1 {
    ParentVisible,
    ChildLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedCarrierDestinationV1 {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) recipe_binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
    pub(crate) source_binding: BindingRefV1,
    pub(crate) header: NestedPhysicalPortRefV1,
    pub(crate) resume: Option<NestedPhysicalNodeRefV1>,
    pub(crate) lexical_scope: ScopeId,
    pub(crate) recurrence_owner: NestedObservedRecurrenceOwnerV1,
    pub(crate) visibility: NestedCarrierVisibilityV1,
    pub(crate) parent_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedPhysicalSourceRoleV1 {
    RootInitializerI(SourceExprSiteV1),
    RootInitializerSum(SourceExprSiteV1),
    RootPredicate(SourceExprSiteV1),
    RootPredicateFalse(SourceExprSiteV1),
    RootBodyChildEntry(SourceStmtSiteV1),
    ChildPredicate(SourceExprSiteV1),
    ChildPredicateFalse(SourceExprSiteV1),
    ChildAncestorUpdate(SourceStmtSiteV1),
    ChildUpdate(SourceStmtSiteV1),
    RootUpdate(SourceStmtSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPhysicalSourceRoleBindingV1 {
    pub(crate) role: NestedPhysicalSourceRoleV1,
    pub(crate) destination: NestedPhysicalNodeRefV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedPhysicalTopologyRejectV1 {
    RecipeShape,
    JoinSigShape,
    SourceForestShape,
    SourceRoleShape,
    CarrierShape,
}

/// Sealed physical-independent topology. The later canonical adapter owns all
/// MIR block/value allocation and must compare owner/frame before consuming it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPhysicalTopologyV1 {
    owner: FunctionOwnerIdV1,
    root_frame_key: LoopExecutionFrameKeyV1,
    ports: [NestedPhysicalPortRefV1; 10],
    parent_resume: NestedParentResumePortV1,
    child_preheader_alias: NestedPortAliasV1,
    edges: Box<[NestedPhysicalEdgeRefV1]>,
    logical_expansions: Box<[NestedLogicalExpansionV1]>,
    predecessor_seals: Box<[VerifiedNestedTopologyPredecessorSealV1]>,
    carriers: [NestedCarrierDestinationV1; 3],
    source_roles: Box<[NestedPhysicalSourceRoleBindingV1]>,
}

impl VerifiedNestedPhysicalTopologyV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn ports(&self) -> &[NestedPhysicalPortRefV1; 10] {
        &self.ports
    }

    pub(crate) fn parent_resume(&self) -> NestedParentResumePortV1 {
        self.parent_resume
    }

    pub(crate) fn child_preheader_alias(&self) -> NestedPortAliasV1 {
        self.child_preheader_alias
    }

    pub(crate) fn edges(&self) -> &[NestedPhysicalEdgeRefV1] {
        &self.edges
    }

    pub(crate) fn logical_expansions(&self) -> &[NestedLogicalExpansionV1] {
        &self.logical_expansions
    }

    pub(crate) fn predecessor_seals(&self) -> &[VerifiedNestedTopologyPredecessorSealV1] {
        &self.predecessor_seals
    }

    pub(crate) fn carriers(&self) -> &[NestedCarrierDestinationV1; 3] {
        &self.carriers
    }

    pub(crate) fn source_roles(&self) -> &[NestedPhysicalSourceRoleBindingV1] {
        &self.source_roles
    }
}

/// Consumes the semantic product and its one-time source handoff. No caller is
/// wired to production; this is a sealed input for the later canonical adapter.
pub(crate) fn issue_nested_predicate_physical_topology_v1(
    product: VerifiedNestedPredicateRecipeProductV1,
) -> Result<VerifiedNestedPhysicalTopologyV1, NestedPhysicalTopologyRejectV1> {
    let (recipe, join_sig, handoff) = product.into_topology_input();
    issue_from_parts(recipe, join_sig, handoff)
}

fn issue_from_parts(
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
    handoff: VerifiedNestedPhysicalSourceHandoffV1,
) -> Result<VerifiedNestedPhysicalTopologyV1, NestedPhysicalTopologyRejectV1> {
    validate_recipe(recipe.as_recipe())?;
    validate_join_sig(join_sig.as_sig())?;
    validate_source_handoff(&handoff)?;
    let parent_resume = NestedParentResumePortV1::new(ROOT, CHILD);
    let root = |stage| NestedPhysicalPortRefV1::new(ROOT, stage);
    let child = |stage| NestedPhysicalPortRefV1::new(CHILD, stage);
    let resume = NestedPhysicalNodeRefV1::ParentResume(parent_resume);
    let root_body = NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::Body));
    let root_step = NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::Step));
    let root_header = NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::Header));
    let child_after = NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::After));
    let child_header = NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Header));
    let edge = |from, to, role| NestedPhysicalEdgeRefV1 { from, to, role };
    let edges = vec![
        edge(
            NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::Preheader)),
            root_header,
            NestedPhysicalEdgeRoleV1::Enter,
        ),
        edge(
            root_header,
            root_body,
            NestedPhysicalEdgeRoleV1::PredicateTrue,
        ),
        edge(
            root_header,
            NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::After)),
            NestedPhysicalEdgeRoleV1::PredicateFalse,
        ),
        edge(
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Preheader)),
            child_header,
            NestedPhysicalEdgeRoleV1::Enter,
        ),
        edge(
            child_header,
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Body)),
            NestedPhysicalEdgeRoleV1::PredicateTrue,
        ),
        edge(
            child_header,
            child_after,
            NestedPhysicalEdgeRoleV1::PredicateFalse,
        ),
        edge(
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Body)),
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Step)),
            NestedPhysicalEdgeRoleV1::BodyToStep,
        ),
        edge(
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Step)),
            child_header,
            NestedPhysicalEdgeRoleV1::StepToHeader,
        ),
        edge(
            child_after,
            resume,
            NestedPhysicalEdgeRoleV1::ChildAfterToParentResume,
        ),
        edge(
            resume,
            root_step,
            NestedPhysicalEdgeRoleV1::ParentResumeToStep,
        ),
        edge(
            root_step,
            root_header,
            NestedPhysicalEdgeRoleV1::StepToHeader,
        ),
    ];
    let edge_ref = |index: usize| edges[index];
    let logical_expansions = vec![
        NestedLogicalExpansionV1 {
            loop_key: ROOT,
            logical_role: LoopJoinEdgeRoleV1::Backedge,
            steps: vec![
                NestedPhysicalExpansionStepV1::ChildLoop(CHILD),
                NestedPhysicalExpansionStepV1::Edge(edge_ref(8)),
                NestedPhysicalExpansionStepV1::Edge(edge_ref(9)),
                NestedPhysicalExpansionStepV1::Edge(edge_ref(10)),
            ]
            .into_boxed_slice(),
        },
        NestedLogicalExpansionV1 {
            loop_key: CHILD,
            logical_role: LoopJoinEdgeRoleV1::Backedge,
            steps: vec![
                NestedPhysicalExpansionStepV1::Edge(edge_ref(6)),
                NestedPhysicalExpansionStepV1::Edge(edge_ref(7)),
            ]
            .into_boxed_slice(),
        },
    ]
    .into_boxed_slice();
    let predecessor_seals = vec![
        seal(root_header, vec![edge_ref(0), edge_ref(10)]),
        seal(root_body, vec![edge_ref(1)]),
        seal(
            NestedPhysicalNodeRefV1::Port(root(NestedPhysicalStageV1::After)),
            vec![edge_ref(2)],
        ),
        seal(root_step, vec![edge_ref(9)]),
        seal(child_header, vec![edge_ref(3), edge_ref(7)]),
        seal(
            NestedPhysicalNodeRefV1::Port(child(NestedPhysicalStageV1::Body)),
            vec![edge_ref(4)],
        ),
        seal(child_after, vec![edge_ref(5)]),
        seal(resume, vec![edge_ref(8)]),
    ]
    .into_boxed_slice();
    let bindings = handoff.bindings();
    let carriers = [
        carrier(
            ROOT,
            0,
            &bindings[0],
            root(NestedPhysicalStageV1::Header),
            Some(NestedPhysicalNodeRefV1::Port(root(
                NestedPhysicalStageV1::After,
            ))),
            NestedCarrierVisibilityV1::ParentVisible,
        ),
        carrier(
            ROOT,
            1,
            &bindings[1],
            root(NestedPhysicalStageV1::Header),
            Some(NestedPhysicalNodeRefV1::Port(root(
                NestedPhysicalStageV1::After,
            ))),
            NestedCarrierVisibilityV1::ParentVisible,
        ),
        carrier(
            CHILD,
            2,
            &bindings[2],
            child(NestedPhysicalStageV1::Header),
            None,
            NestedCarrierVisibilityV1::ChildLocal,
        ),
    ];
    let source_roles = source_roles(&handoff, root_body, root_step, root_header, child_header);
    Ok(VerifiedNestedPhysicalTopologyV1 {
        owner: handoff.owner(),
        root_frame_key: handoff.root_frame_key().clone(),
        ports: [
            root(NestedPhysicalStageV1::Preheader),
            root(NestedPhysicalStageV1::Header),
            root(NestedPhysicalStageV1::Body),
            root(NestedPhysicalStageV1::Step),
            root(NestedPhysicalStageV1::After),
            child(NestedPhysicalStageV1::Preheader),
            child(NestedPhysicalStageV1::Header),
            child(NestedPhysicalStageV1::Body),
            child(NestedPhysicalStageV1::Step),
            child(NestedPhysicalStageV1::After),
        ],
        parent_resume,
        child_preheader_alias: NestedPortAliasV1 {
            alias: child(NestedPhysicalStageV1::Preheader),
            canonical: root(NestedPhysicalStageV1::Body),
        },
        edges: edges.into_boxed_slice(),
        logical_expansions,
        predecessor_seals,
        carriers,
        source_roles,
    })
}

fn validate_recipe(recipe: &LoopRecipeV1) -> Result<(), NestedPhysicalTopologyRejectV1> {
    if recipe.loops.len() != 2
        || recipe.loops[0].key != ROOT
        || recipe.loops[0].parent.is_some()
        || recipe.loops[1].key != CHILD
        || recipe.loops[1].parent != Some(ROOT)
        || recipe.carriers.len() != 3
        || recipe
            .carriers
            .iter()
            .map(|carrier| (carrier.owner_loop, carrier.binding.raw()))
            .collect::<Vec<_>>()
            != vec![(ROOT, 0), (ROOT, 1), (CHILD, 2)]
    {
        return Err(NestedPhysicalTopologyRejectV1::RecipeShape);
    }
    Ok(())
}

fn validate_join_sig(
    sig: &crate::mir::loop_recipe_contract::LoopJoinSigV1,
) -> Result<(), NestedPhysicalTopologyRejectV1> {
    if sig.loops.len() != 2 {
        return Err(NestedPhysicalTopologyRejectV1::JoinSigShape);
    }
    for (index, row) in sig.loops.iter().enumerate() {
        let expected = if index == 0 { ROOT } else { CHILD };
        if row.key != expected
            || (index == 0 && row.parent.is_some())
            || (index == 1 && row.parent != Some(ROOT))
            || row.edges.iter().map(|edge| edge.role).collect::<Vec<_>>()
                != vec![
                    LoopJoinEdgeRoleV1::Enter,
                    LoopJoinEdgeRoleV1::PredicateTrue,
                    LoopJoinEdgeRoleV1::PredicateFalse,
                    LoopJoinEdgeRoleV1::Backedge,
                ]
        {
            return Err(NestedPhysicalTopologyRejectV1::JoinSigShape);
        }
    }
    Ok(())
}

fn validate_source_handoff(
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
) -> Result<(), NestedPhysicalTopologyRejectV1> {
    if handoff.forest_parent_receipt().parent_indices() != [None, Some(0)]
        || handoff.forest_parent_receipt().member_count() != 2
    {
        return Err(NestedPhysicalTopologyRejectV1::SourceForestShape);
    }
    let shape = handoff.shape();
    if shape.root_body_roles
        != [
            NestedRootBodyRoleV1::LocalJ,
            NestedRootBodyRoleV1::InitializeJ,
            NestedRootBodyRoleV1::ChildLoop,
            NestedRootBodyRoleV1::IncrementRoot,
        ]
        || shape.child_body_roles
            != [
                NestedChildBodyRoleV1::IncrementAncestor,
                NestedChildBodyRoleV1::IncrementChild,
            ]
        || shape.root_condition.binding != handoff.bindings()[0].binding
        || shape.increment_root.binding != handoff.bindings()[0].binding
        || shape.increment_ancestor.binding != handoff.bindings()[1].binding
        || shape.child_condition.binding != handoff.bindings()[2].binding
        || shape.increment_child.binding != handoff.bindings()[2].binding
    {
        return Err(NestedPhysicalTopologyRejectV1::SourceRoleShape);
    }
    Ok(())
}

fn carrier(
    owner_loop: LoopNodeKeyV1,
    binding: u32,
    evidence: &super::nested_predicate_projection::NestedBindingEvidenceV1,
    header: NestedPhysicalPortRefV1,
    resume: Option<NestedPhysicalNodeRefV1>,
    visibility: NestedCarrierVisibilityV1,
) -> NestedCarrierDestinationV1 {
    NestedCarrierDestinationV1 {
        owner_loop,
        recipe_binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(binding),
        source_binding: evidence.binding,
        header,
        resume,
        lexical_scope: evidence.lexical_scope,
        recurrence_owner: evidence.recurrence_owner,
        visibility,
        parent_visible: evidence.parent_visible,
    }
}

fn seal(
    target: NestedPhysicalNodeRefV1,
    incoming: Vec<NestedPhysicalEdgeRefV1>,
) -> VerifiedNestedTopologyPredecessorSealV1 {
    VerifiedNestedTopologyPredecessorSealV1 {
        target,
        incoming: incoming.into_boxed_slice(),
    }
}

fn source_roles(
    handoff: &VerifiedNestedPhysicalSourceHandoffV1,
    root_body: NestedPhysicalNodeRefV1,
    root_step: NestedPhysicalNodeRefV1,
    root_header: NestedPhysicalNodeRefV1,
    child_header: NestedPhysicalNodeRefV1,
) -> Box<[NestedPhysicalSourceRoleBindingV1]> {
    let conditions = handoff.conditions();
    let updates = handoff.updates();
    let initializers = handoff.root_initializers();
    vec![
        role(
            NestedPhysicalSourceRoleV1::RootInitializerI(initializers[0].value_site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                ROOT,
                NestedPhysicalStageV1::Preheader,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::RootInitializerSum(initializers[1].value_site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                ROOT,
                NestedPhysicalStageV1::Preheader,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::RootPredicate(conditions[0].site.clone()),
            root_header,
        ),
        role(
            NestedPhysicalSourceRoleV1::RootPredicateFalse(conditions[0].site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                ROOT,
                NestedPhysicalStageV1::After,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::RootBodyChildEntry(handoff.child_site().clone()),
            root_body,
        ),
        role(
            NestedPhysicalSourceRoleV1::ChildPredicate(conditions[1].site.clone()),
            child_header,
        ),
        role(
            NestedPhysicalSourceRoleV1::ChildPredicateFalse(conditions[1].site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                CHILD,
                NestedPhysicalStageV1::After,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::ChildAncestorUpdate(updates[2].statement_site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                CHILD,
                NestedPhysicalStageV1::Body,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::ChildUpdate(updates[3].statement_site.clone()),
            NestedPhysicalNodeRefV1::Port(NestedPhysicalPortRefV1::new(
                CHILD,
                NestedPhysicalStageV1::Step,
            )),
        ),
        role(
            NestedPhysicalSourceRoleV1::RootUpdate(updates[1].statement_site.clone()),
            root_step,
        ),
    ]
    .into_boxed_slice()
}

fn role(
    role: NestedPhysicalSourceRoleV1,
    destination: NestedPhysicalNodeRefV1,
) -> NestedPhysicalSourceRoleBindingV1 {
    NestedPhysicalSourceRoleBindingV1 { role, destination }
}
