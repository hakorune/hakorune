//! Source-path and pre-If definition witnesses for the portable If mapper.
//!
//! This module owns only deterministic source correspondence helpers. It does
//! not map expressions, select a route, or emit physical MIR.

use crate::mir::if_recipe_contract::{IfSourcePathStepV1, IfSourcePathV1};
use crate::mir::resolved_semantics::{
    SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::product::{TrivialBindingDefinitionOriginV1, VerifiedTrivialCanonicalOwnerV1};
use super::recipe_facts::VerifiedTrivialIfRecipeFactsV1;
use super::recipe_mapper::IfRecipeMapRejectV1;

pub(super) fn root_body_index(site: &SourceStmtSiteV1) -> Result<u32, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index)] => Ok(*index),
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "if_node" }),
    }
}

pub(super) fn source_binding(
    facts: &VerifiedTrivialIfRecipeFactsV1,
    origin: crate::mir::resolved_semantics::FunctionOriginV1,
    root_index: u32,
    explicit_else: bool,
) -> Result<crate::mir::if_recipe_contract::IfRecipeSourceBindingV1, IfRecipeMapRejectV1> {
    let then_assignment = facts
        .then_assignment()
        .ok_or(IfRecipeMapRejectV1::MissingAssignment { branch: "then" })?;
    let mut claims = vec![
        crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::IfNode,
            path: if_node_path(facts.if_site(), root_index)?,
        },
        crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::Condition,
            path: condition_path(facts.condition(), root_index)?,
        },
        crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::ThenAssignment,
            path: assignment_path(then_assignment.statement(), root_index, true)?,
        },
    ];
    if explicit_else {
        let else_assignment = facts
            .else_assignment()
            .ok_or(IfRecipeMapRejectV1::MissingAssignment { branch: "else" })?;
        claims.push(crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::ElseAssignment,
            path: assignment_path(else_assignment.statement(), root_index, false)?,
        });
    } else {
        claims.push(crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::ImplicitBaseline,
            path: implicit_baseline_path(root_index),
        });
    }
    for call_site in facts.direct_call_sites().into_iter().flatten() {
        claims.push(crate::mir::if_recipe_contract::IfSourceClaimV1 {
            role: crate::mir::if_recipe_contract::IfSourceClaimRoleV1::DirectStaticCall,
            path: direct_call_path(call_site, root_index)?,
        });
    }
    Ok(crate::mir::if_recipe_contract::IfRecipeSourceBindingV1 {
        owner: crate::mir::if_recipe_contract::IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal: origin.compilation_unit_ordinal(),
            function_ordinal: origin.function_ordinal(),
        },
        claims,
    })
}

fn direct_call_path(
    site: &SourceExprSiteV1,
    root: u32,
) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfThen(item), SourcePathSegmentV1::Value]
            if *index == root =>
        {
            Ok(IfRecipeSourcePath::then_value_path(root, *item))
        }
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfElse(item), SourcePathSegmentV1::Value]
            if *index == root =>
        {
            Ok(IfRecipeSourcePath::else_value_path(root, *item))
        }
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch {
            role: "direct_static_call",
        }),
    }
}

fn if_node_path(site: &SourceStmtSiteV1, root: u32) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    if root_body_index(site)? == root {
        Ok(IfSourcePathV1 {
            steps: vec![IfSourcePathStepV1::BodyItem { index: root }],
        })
    } else {
        Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "if_node" })
    }
}

fn condition_path(
    site: &SourceExprSiteV1,
    root: u32,
) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfCondition] if *index == root => {
            Ok(IfSourcePathV1 {
                steps: vec![
                    IfSourcePathStepV1::BodyItem { index: root },
                    IfSourcePathStepV1::IfCondition,
                ],
            })
        }
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "condition" }),
    }
}

fn assignment_path(
    site: &SourceStmtSiteV1,
    root: u32,
    then_branch: bool,
) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfThen(item)]
            if then_branch && *index == root =>
        {
            Ok(IfRecipeSourcePath::then_path(root, *item))
        }
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfElse(item)]
            if !then_branch && *index == root =>
        {
            Ok(IfRecipeSourcePath::else_path(root, *item))
        }
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch {
            role: if then_branch {
                "then_assignment"
            } else {
                "else_assignment"
            },
        }),
    }
}

fn implicit_baseline_path(root: u32) -> IfSourcePathV1 {
    IfSourcePathV1 {
        steps: vec![
            IfSourcePathStepV1::BodyItem { index: root },
            IfSourcePathStepV1::IfImplicitBaseline,
        ],
    }
}

struct IfRecipeSourcePath;

impl IfRecipeSourcePath {
    fn then_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfThenItem { index: item },
            ],
        }
    }

    fn else_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfElseItem { index: item },
            ],
        }
    }

    fn then_value_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfThenItem { index: item },
                IfSourcePathStepV1::AssignmentValue,
            ],
        }
    }

    fn else_value_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfElseItem { index: item },
                IfSourcePathStepV1::AssignmentValue,
            ],
        }
    }
}

pub(super) fn verify_entry_definition(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    if_site: &SourceStmtSiteV1,
) -> Result<(), IfRecipeMapRejectV1> {
    let mut found = false;
    let mut found_after = false;
    for definition in profile.definitions() {
        if definition.binding() != binding {
            continue;
        }
        match definition.origin() {
            TrivialBindingDefinitionOriginV1::Declaration(site) => match site {
                SourceBindingSiteV1::Parameter { .. } | SourceBindingSiteV1::Receiver => {
                    found = true;
                }
                SourceBindingSiteV1::Local { statement, .. }
                | SourceBindingSiteV1::Outbox { statement, .. }
                | SourceBindingSiteV1::Nowait { statement } => {
                    if statement.node().segments() < if_site.node().segments() {
                        found = true;
                    } else {
                        found_after = true;
                    }
                }
                _ => {}
            },
            TrivialBindingDefinitionOriginV1::Assignment(_) => {}
        }
    }
    if found {
        Ok(())
    } else if found_after {
        Err(IfRecipeMapRejectV1::EntryDefinitionAfterIf)
    } else {
        Err(IfRecipeMapRejectV1::EntryDefinitionMissing)
    }
}
