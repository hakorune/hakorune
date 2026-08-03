//! Production seam for the first verified If recipe.
//!
//! This adapter owns pre-effect demand production and single-use admission.
//! The canonical session remains the physical CFG/SSA/PHI owner; this module
//! must not allocate blocks, emit MIR, select routes, or retry a different
//! lowering path.

use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::if_recipe_contract::{
    IfPhysicalInputRejectReasonV1, IfRecipeSourceOwnerV1, IfSourcePathStepV1,
    VerifiedIfPhysicalInputV1,
};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};
use crate::mir::resolved_value_profile::{
    map_trivial_if_recipe_v1, product::{TrivialRepresentationV1, VerifiedTrivialCanonicalOwnerV1},
    IfRecipeMapRejectV1,
};

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipePreflightV1 {
    NotThisShape,
    Selected(CanonicalIfPhysicalDemandV1),
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeProducerRejectV1 {
    Mapper(IfRecipeMapRejectV1),
    PhysicalInput(IfPhysicalInputRejectReasonV1),
    Correspondence(CanonicalIfRecipeCorrespondenceRejectV1),
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeCorrespondenceRejectV1 {
    MissingEntryWitness,
    MissingThenAssignment,
    MissingElseAssignment,
    MissingContinuationRead,
    BindingMismatch,
    RepresentationMismatch,
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeAdmissionRejectV1 {
    SourceOwnerMismatch,
    MissingIfClaim,
    InvalidIfClaimPath,
    IfControlCardinality { found: usize },
    IfControlSiteMismatch,
    SelectedIfNotConsumed,
    SelectedIfConsumedTwice,
    UnexpectedIfSite,
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeAdmissionDispositionV1 {
    NotSelected,
    Selected(CanonicalIfRecipeAdmissionV1),
}

impl CanonicalIfRecipeAdmissionDispositionV1 {
    pub(in crate::mir::builder::resolved_lowering) fn is_not_selected(&self) -> bool {
        matches!(self, Self::NotSelected)
    }

    pub(in crate::mir::builder::resolved_lowering) fn take_if(
        &mut self,
        statement: &LocatedStmtV1<'_>,
    ) -> Result<CanonicalIfPhysicalDemandV1, CanonicalIfRecipeAdmissionRejectV1> {
        match self {
            Self::NotSelected => Err(CanonicalIfRecipeAdmissionRejectV1::MissingIfClaim),
            Self::Selected(admission) => admission.take_site(statement.site()),
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(
        self,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        match self {
            Self::NotSelected => Ok(()),
            Self::Selected(admission) => admission.finish(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalIfRecipeAdmissionV1 {
    expected_site: SourceStmtSiteV1,
    state: CanonicalIfRecipeAdmissionStateV1,
}

#[derive(Debug)]
enum CanonicalIfRecipeAdmissionStateV1 {
    Pending(CanonicalIfPhysicalDemandV1),
    Consumed,
}

/// A one-shot, same-pass physical demand.  The portable artifact remains
/// paired with its JoinSig until the dedicated physicalizer consumes it; the
/// correspondence receipt prevents a later AST/name lookup from becoming a
/// second source of branch or PHI identity.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalIfPhysicalDemandV1 {
    physical_input: VerifiedIfPhysicalInputV1,
    correspondence: CanonicalIfPhysicalCorrespondenceV1,
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalIfPhysicalCorrespondenceV1 {
    if_site: SourceStmtSiteV1,
    condition: SourceExprSiteV1,
    entry_binding: BindingRefV1,
    representation: TrivialRepresentationV1,
    then_assignment: SourceStmtSiteV1,
    then_value: SourceExprSiteV1,
    else_assignment: SourceStmtSiteV1,
    else_value: SourceExprSiteV1,
    continuation_read: SourceExprSiteV1,
}

impl CanonicalIfPhysicalDemandV1 {
    pub(in crate::mir::builder::resolved_lowering) fn into_parts(
        self,
    ) -> (
        VerifiedIfPhysicalInputV1,
        CanonicalIfPhysicalCorrespondenceV1,
    ) {
        (self.physical_input, self.correspondence)
    }
}

impl CanonicalIfPhysicalCorrespondenceV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn if_site(&self) -> &SourceStmtSiteV1 {
        &self.if_site
    }

    pub(in crate::mir::builder::resolved_lowering) const fn condition(
        &self,
    ) -> &SourceExprSiteV1 {
        &self.condition
    }

    pub(in crate::mir::builder::resolved_lowering) const fn entry_binding(&self) -> BindingRefV1 {
        self.entry_binding
    }

    pub(in crate::mir::builder::resolved_lowering) const fn representation(
        &self,
    ) -> TrivialRepresentationV1 {
        self.representation
    }

    pub(in crate::mir::builder::resolved_lowering) const fn then_assignment(
        &self,
    ) -> &SourceStmtSiteV1 {
        &self.then_assignment
    }

    pub(in crate::mir::builder::resolved_lowering) const fn then_value(
        &self,
    ) -> &SourceExprSiteV1 {
        &self.then_value
    }

    pub(in crate::mir::builder::resolved_lowering) const fn else_assignment(
        &self,
    ) -> &SourceStmtSiteV1 {
        &self.else_assignment
    }

    pub(in crate::mir::builder::resolved_lowering) const fn else_value(
        &self,
    ) -> &SourceExprSiteV1 {
        &self.else_value
    }

    pub(in crate::mir::builder::resolved_lowering) const fn continuation_read(
        &self,
    ) -> &SourceExprSiteV1 {
        &self.continuation_read
    }
}

pub(in crate::mir::builder::resolved_lowering) fn produce_trivial_if_physical_input_v1(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> Result<CanonicalIfRecipePreflightV1, CanonicalIfRecipeProducerRejectV1> {
    let Some(facts) = profile.recipe_facts() else {
        return Ok(CanonicalIfRecipePreflightV1::NotThisShape);
    };
    let correspondence = correspondence_from_facts(facts).map_err(
        CanonicalIfRecipeProducerRejectV1::Correspondence,
    )?;
    let artifact = map_trivial_if_recipe_v1(profile, source_function)
        .map_err(CanonicalIfRecipeProducerRejectV1::Mapper)?;
    let physical_input = VerifiedIfPhysicalInputV1::from_artifact(artifact)
        .map_err(CanonicalIfRecipeProducerRejectV1::PhysicalInput)?;
    Ok(CanonicalIfRecipePreflightV1::Selected(
        CanonicalIfPhysicalDemandV1 {
            physical_input,
            correspondence,
        },
    ))
}

pub(in crate::mir::builder::resolved_lowering) fn admit_trivial_if_recipe_v1(
    preflight: CanonicalIfRecipePreflightV1,
    source_function: &VerifiedResolvedFunctionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
) -> Result<CanonicalIfRecipeAdmissionDispositionV1, CanonicalIfRecipeAdmissionRejectV1> {
    let CanonicalIfRecipePreflightV1::Selected(demand) = preflight else {
        return Ok(CanonicalIfRecipeAdmissionDispositionV1::NotSelected);
    };
    let physical_input = &demand.physical_input;
    let source_binding = physical_input
        .artifact()
        .source_binding()
        .as_source_binding();
    if !source_owner_matches(source_binding.owner, source_function) {
        return Err(CanonicalIfRecipeAdmissionRejectV1::SourceOwnerMismatch);
    }
    let root_index = source_binding
        .claims
        .first()
        .and_then(|claim| claim.path.steps.first())
        .and_then(|step| match step {
            IfSourcePathStepV1::BodyItem { index } => Some(*index),
            _ => None,
        })
        .ok_or(CanonicalIfRecipeAdmissionRejectV1::MissingIfClaim)?;
    if source_binding
        .claims
        .first()
        .map(|claim| claim.path.steps.len())
        != Some(1)
    {
        return Err(CanonicalIfRecipeAdmissionRejectV1::InvalidIfClaimPath);
    }

    let mut sites = if_control.exact_if_sites();
    let expected_site = sites
        .next()
        .cloned()
        .ok_or(CanonicalIfRecipeAdmissionRejectV1::IfControlCardinality { found: 0 })?;
    if sites.next().is_some() {
        return Err(CanonicalIfRecipeAdmissionRejectV1::IfControlCardinality {
            found: if_control.row_count(),
        });
    }
    if !matches!(
        expected_site.node().segments(),
        [SourcePathSegmentV1::Body(index)] if *index == root_index
    ) {
        return Err(CanonicalIfRecipeAdmissionRejectV1::IfControlSiteMismatch);
    }
    Ok(CanonicalIfRecipeAdmissionDispositionV1::Selected(
        CanonicalIfRecipeAdmissionV1 {
            expected_site,
            state: CanonicalIfRecipeAdmissionStateV1::Pending(demand),
        },
    ))
}

impl CanonicalIfRecipeAdmissionV1 {
    fn take_site(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Result<CanonicalIfPhysicalDemandV1, CanonicalIfRecipeAdmissionRejectV1> {
        if site != &self.expected_site {
            return Err(CanonicalIfRecipeAdmissionRejectV1::UnexpectedIfSite);
        }
        let state = std::mem::replace(
            &mut self.state,
            CanonicalIfRecipeAdmissionStateV1::Consumed,
        );
        match state {
            CanonicalIfRecipeAdmissionStateV1::Pending(demand) => Ok(demand),
            CanonicalIfRecipeAdmissionStateV1::Consumed => {
                Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfConsumedTwice)
            }
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(
        self,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        match self.state {
            CanonicalIfRecipeAdmissionStateV1::Consumed => Ok(()),
            CanonicalIfRecipeAdmissionStateV1::Pending(_) => {
                Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfNotConsumed)
            }
        }
    }
}

fn correspondence_from_facts(
    facts: &crate::mir::resolved_value_profile::VerifiedTrivialIfRecipeFactsV1,
) -> Result<CanonicalIfPhysicalCorrespondenceV1, CanonicalIfRecipeCorrespondenceRejectV1> {
    let entry = facts
        .entry_witness()
        .ok_or(CanonicalIfRecipeCorrespondenceRejectV1::MissingEntryWitness)?;
    let then_assignment = facts
        .then_assignment()
        .ok_or(CanonicalIfRecipeCorrespondenceRejectV1::MissingThenAssignment)?;
    let else_assignment = facts
        .else_assignment()
        .ok_or(CanonicalIfRecipeCorrespondenceRejectV1::MissingElseAssignment)?;
    let continuation_read = facts
        .continuation_read()
        .ok_or(CanonicalIfRecipeCorrespondenceRejectV1::MissingContinuationRead)?;
    if entry.binding() != then_assignment.binding() || entry.binding() != else_assignment.binding() {
        return Err(CanonicalIfRecipeCorrespondenceRejectV1::BindingMismatch);
    }
    if entry.representation() != then_assignment.representation()
        || entry.representation() != else_assignment.representation()
    {
        return Err(CanonicalIfRecipeCorrespondenceRejectV1::RepresentationMismatch);
    }
    Ok(CanonicalIfPhysicalCorrespondenceV1 {
        if_site: facts.if_site().clone(),
        condition: facts.condition().clone(),
        entry_binding: entry.binding(),
        representation: entry.representation(),
        then_assignment: then_assignment.statement().clone(),
        then_value: then_assignment.value().clone(),
        else_assignment: else_assignment.statement().clone(),
        else_value: else_assignment.value().clone(),
        continuation_read: continuation_read.clone(),
    })
}

fn source_owner_matches(
    owner: IfRecipeSourceOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> bool {
    let origin = source_function.function_origin();
    matches!(
        owner,
        IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal,
            function_ordinal,
        } if compilation_unit_ordinal == origin.compilation_unit_ordinal()
            && function_ordinal == origin.function_ordinal()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::if_recipe_contract::*;
    use crate::mir::resolved_semantics::{
        BindingRefV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };
    use hakorune_mir_core::BindingId;

    fn stmt_site(index: u32) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
        ]))
    }

    fn expr_site(index: u32, role: SourcePathSegmentV1) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
            role,
        ]))
    }

    fn artifact() -> VerifiedIfRecipeArtifactV1 {
        let binding = IfBindingKeyV1::new(0);
        let values = (0..7)
            .map(|raw| IfRecipeValueV1 {
                key: IfValueKeyV1::new(raw),
                class: if raw == 3 {
                    IfValueClassV1::Bool
                } else {
                    IfValueClassV1::I64
                },
            })
            .collect();
        let claim = |role, steps| IfSourceClaimV1 {
            role,
            path: IfSourcePathV1 { steps },
        };
        let source_binding = IfRecipeSourceBindingV1 {
            owner: IfRecipeSourceOwnerV1::FunctionBody {
                compilation_unit_ordinal: 0,
                function_ordinal: 0,
            },
            claims: vec![
                claim(
                    IfSourceClaimRoleV1::IfNode,
                    vec![IfSourcePathStepV1::BodyItem { index: 1 }],
                ),
                claim(
                    IfSourceClaimRoleV1::Condition,
                    vec![
                        IfSourcePathStepV1::BodyItem { index: 1 },
                        IfSourcePathStepV1::IfCondition,
                    ],
                ),
                claim(
                    IfSourceClaimRoleV1::ThenAssignment,
                    vec![
                        IfSourcePathStepV1::BodyItem { index: 1 },
                        IfSourcePathStepV1::IfThenItem { index: 0 },
                    ],
                ),
                claim(
                    IfSourceClaimRoleV1::ElseAssignment,
                    vec![
                        IfSourcePathStepV1::BodyItem { index: 1 },
                        IfSourcePathStepV1::IfElseItem { index: 0 },
                    ],
                ),
            ],
        };
        let item = |key, operation| IfRecipeItemRowV1 {
            key: IfItemKeyV1::new(key),
            operation,
        };
        IfRecipeVerifierV1::verify_artifact(IfRecipeArtifactV1::new(
            IfRecipeProvenanceV1 {
                profile: IfRecipeProfileV1::ResolvedTrivialExplicitElse,
            },
            source_binding,
            IfRecipeV1 {
                condition_block: IfRecipeBlockV1 {
                    key: IfBlockKeyV1::new(0),
                    role: IfBlockRoleV1::Condition,
                    items: vec![
                        item(
                            0,
                            IfOperationV1::ReadBinding {
                                binding,
                                result: IfValueKeyV1::new(1),
                            },
                        ),
                        item(
                            1,
                            IfOperationV1::ConstI64 {
                                result: IfValueKeyV1::new(2),
                                value: 1,
                            },
                        ),
                        item(
                            2,
                            IfOperationV1::CompareI64 {
                                op: IfCompareOpV1::Less,
                                left: IfValueKeyV1::new(1),
                                right: IfValueKeyV1::new(2),
                                result: IfValueKeyV1::new(3),
                            },
                        ),
                    ],
                },
                then_block: IfRecipeBlockV1 {
                    key: IfBlockKeyV1::new(1),
                    role: IfBlockRoleV1::Then,
                    items: vec![
                        item(
                            3,
                            IfOperationV1::ConstI64 {
                                result: IfValueKeyV1::new(4),
                                value: 1,
                            },
                        ),
                        item(
                            4,
                            IfOperationV1::WriteBinding {
                                binding,
                                value: IfValueKeyV1::new(4),
                            },
                        ),
                    ],
                },
                else_block: Some(IfRecipeBlockV1 {
                    key: IfBlockKeyV1::new(2),
                    role: IfBlockRoleV1::Else,
                    items: vec![
                        item(
                            5,
                            IfOperationV1::ConstI64 {
                                result: IfValueKeyV1::new(5),
                                value: 2,
                            },
                        ),
                        item(
                            6,
                            IfOperationV1::WriteBinding {
                                binding,
                                value: IfValueKeyV1::new(5),
                            },
                        ),
                    ],
                }),
                continuation_block: IfRecipeBlockV1 {
                    key: IfBlockKeyV1::new(3),
                    role: IfBlockRoleV1::Continuation,
                    items: vec![item(
                        7,
                        IfOperationV1::ReadBinding {
                            binding,
                            result: IfValueKeyV1::new(6),
                        },
                    )],
                },
                else_disposition: IfElseDispositionV1::Explicit,
                condition: IfValueKeyV1::new(3),
                inputs: vec![IfValueKeyV1::new(0)],
                bindings: vec![IfRecipeBindingV1 {
                    key: binding,
                    role: IfBindingRoleV1::MergeTarget,
                    class: IfValueClassV1::I64,
                }],
                values,
                joins: vec![IfJoinRowV1 {
                    binding,
                    class: IfValueClassV1::I64,
                    entry_value: IfValueKeyV1::new(0),
                    then_value: IfValueKeyV1::new(4),
                    else_value: IfValueKeyV1::new(5),
                }],
                continuation: IfContinuationV1 {
                    required_read: binding,
                },
            },
        ))
        .expect("test artifact verifies")
    }

    fn demand() -> CanonicalIfPhysicalDemandV1 {
        let mut issuer = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
            .expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let binding = BindingRefV1::new(owner, BindingId::new(0));
        let input = VerifiedIfPhysicalInputV1::from_artifact(artifact()).expect("physical input");
        CanonicalIfPhysicalDemandV1 {
            physical_input: input,
            correspondence: CanonicalIfPhysicalCorrespondenceV1 {
                if_site: stmt_site(1),
                condition: expr_site(1, SourcePathSegmentV1::IfCondition),
                entry_binding: binding,
                representation: TrivialRepresentationV1::InlineI64,
                then_assignment: stmt_site(2),
                then_value: expr_site(2, SourcePathSegmentV1::Value),
                else_assignment: stmt_site(3),
                else_value: expr_site(3, SourcePathSegmentV1::Value),
                continuation_read: expr_site(4, SourcePathSegmentV1::Value),
            },
        }
    }

    fn admission() -> CanonicalIfRecipeAdmissionV1 {
        CanonicalIfRecipeAdmissionV1 {
            expected_site: stmt_site(1),
            state: CanonicalIfRecipeAdmissionStateV1::Pending(demand()),
        }
    }

    #[test]
    fn selected_demand_is_consumed_once() {
        let mut admission = admission();
        assert!(admission.take_site(&stmt_site(1)).is_ok());
        assert!(matches!(
            admission.take_site(&stmt_site(1)),
            Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfConsumedTwice)
        ));
        assert!(admission.finish().is_ok());
    }

    #[test]
    fn selected_demand_must_be_consumed_before_finish() {
        assert!(matches!(
            admission().finish(),
            Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfNotConsumed)
        ));
    }
}
