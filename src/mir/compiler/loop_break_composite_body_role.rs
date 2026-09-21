//! Resolver-owned body roles for the composite LoopBreak source bridge.
//!
//! This module observes one already-sealed source function and records only
//! source sites plus resolver-issued control evidence. It deliberately does
//! not issue Recipe items, JoinSig keys, or physical identities.

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedStmtV1};
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, RegionId, ResolvedControlTransferV1, ResolvedExitRecordV1,
    ResolvedExitSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopBreakCompositeBodyRoleRejectV1 {
    ForeignOwner,
    SourceNavigation,
    UnsupportedShape(SourceStmtSiteV1),
    MissingLoopMember(SourceStmtSiteV1),
    MissingExit(SourceStmtSiteV1),
    ExitOutsideForest(SourceStmtSiteV1),
    DuplicateSite(SourceStmtSiteV1),
    DuplicateSourceCall(SourceExprSiteV1),
    SourceCallOutsideRole(SourceExprSiteV1),
    SourceCallAmbiguous(SourceExprSiteV1),
    ForestCoverage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompositeLoopEvidenceV1 {
    site: SourceStmtSiteV1,
    condition_site: SourceExprSiteV1,
    member_index: u32,
    parent_index: Option<u32>,
    frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
}

impl CompositeLoopEvidenceV1 {
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(in crate::mir) const fn member_index(&self) -> u32 {
        self.member_index
    }

    pub(in crate::mir) const fn parent_index(&self) -> Option<u32> {
        self.parent_index
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompositeLoopBodyRoleV1 {
    Statement {
        site: SourceStmtSiteV1,
    },
    If {
        site: SourceStmtSiteV1,
        condition_site: SourceExprSiteV1,
        then_body: Box<[CompositeLoopBodyRoleV1]>,
        else_body: Option<Box<[CompositeLoopBodyRoleV1]>>,
    },
    Loop {
        evidence: CompositeLoopEvidenceV1,
        body: Box<[CompositeLoopBodyRoleV1]>,
    },
    Exit {
        site: SourceStmtSiteV1,
        branch_owner: Option<SourceStmtSiteV1>,
        record: ResolvedExitRecordV1,
    },
}

impl CompositeLoopBodyRoleV1 {
    pub(crate) fn contains_control(&self) -> bool {
        match self {
            Self::Statement { .. } => false,
            Self::Exit { .. } | Self::Loop { .. } => true,
            Self::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().any(Self::contains_control)
                    || else_body
                        .as_deref()
                        .is_some_and(|body| body.iter().any(Self::contains_control))
            }
        }
    }

    pub(in crate::mir) fn as_statement(&self) -> Option<&SourceStmtSiteV1> {
        match self {
            Self::Statement { site } => Some(site),
            _ => None,
        }
    }

    pub(in crate::mir) fn as_if(
        &self,
    ) -> Option<(
        &SourceStmtSiteV1,
        &SourceExprSiteV1,
        &[CompositeLoopBodyRoleV1],
        Option<&[CompositeLoopBodyRoleV1]>,
    )> {
        match self {
            Self::If {
                site,
                condition_site,
                then_body,
                else_body,
            } => Some((site, condition_site, then_body, else_body.as_deref())),
            _ => None,
        }
    }

    pub(in crate::mir) fn as_loop(
        &self,
    ) -> Option<(&CompositeLoopEvidenceV1, &[CompositeLoopBodyRoleV1])> {
        match self {
            Self::Loop { evidence, body } => Some((evidence, body)),
            _ => None,
        }
    }

    pub(in crate::mir) fn as_exit(
        &self,
    ) -> Option<(&SourceStmtSiteV1, &ResolvedExitRecordV1)> {
        match self {
            Self::Exit { site, record, .. } => Some((site, record)),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn count_loops(&self) -> usize {
        match self {
            Self::Loop { body, .. } => 1 + body.iter().map(Self::count_loops).sum::<usize>(),
            Self::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().map(Self::count_loops).sum::<usize>()
                    + else_body
                        .as_deref()
                        .map(|body| body.iter().map(Self::count_loops).sum())
                        .unwrap_or(0)
            }
            Self::Statement { .. } | Self::Exit { .. } => 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn count_exits(&self) -> usize {
        match self {
            Self::Exit { .. } => 1,
            Self::Loop { body, .. } => body.iter().map(Self::count_exits).sum(),
            Self::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().map(Self::count_exits).sum::<usize>()
                    + else_body
                        .as_deref()
                        .map(|body| body.iter().map(Self::count_exits).sum())
                        .unwrap_or(0)
            }
            Self::Statement { .. } => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBreakCompositeBodyRoleMapV1 {
    root: CompositeLoopBodyRoleV1,
    method_call_sites: Box<[SourceExprSiteV1]>,
}

impl VerifiedLoopBreakCompositeBodyRoleMapV1 {
    #[cfg(test)]
    pub(crate) fn root(&self) -> &CompositeLoopBodyRoleV1 {
        &self.root
    }

    pub(in crate::mir) fn root_for_recipe(&self) -> &CompositeLoopBodyRoleV1 {
        &self.root
    }

    #[cfg(test)]
    pub(crate) fn method_call_sites(&self) -> &[SourceExprSiteV1] {
        &self.method_call_sites
    }
}

pub(crate) fn issue_loop_break_composite_body_role_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root_stmt: &LocatedStmtV1<'_>,
    root_body: &LocatedBodyV1<'_>,
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
) -> Result<VerifiedLoopBreakCompositeBodyRoleMapV1, LoopBreakCompositeBodyRoleRejectV1> {
    if input.owner() != root_stmt.owner() || input.owner() != root_body.site().owner() {
        return Err(LoopBreakCompositeBodyRoleRejectV1::ForeignOwner);
    }
    let members = issue_loop_evidence(input, forest)?;
    let mut seen_sites = std::collections::BTreeSet::new();
    let root = issue_loop_role(
        input,
        root_stmt,
        root_body,
        &members,
        forest,
        &mut seen_sites,
    )?;
    let mut observed_loops = std::collections::BTreeSet::new();
    collect_loop_sites(&root, &mut observed_loops);
    let expected_loops = members
        .iter()
        .map(|(evidence, _)| evidence.site.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if observed_loops != expected_loops {
        return Err(LoopBreakCompositeBodyRoleRejectV1::ForestCoverage);
    }
    let root_segments = root_stmt.site().node().segments();
    let mut method_call_sites = Vec::new();
    let mut seen_method_calls = std::collections::BTreeSet::new();
    for (site, _) in input.function().method_calls() {
        if !seen_method_calls.insert(site.clone()) {
            return Err(LoopBreakCompositeBodyRoleRejectV1::DuplicateSourceCall(
                site.clone(),
            ));
        }
        if site.node().segments().starts_with(root_segments) {
            let mut matches = 0usize;
            role_call_matches(&root, site, &mut matches);
            match matches {
                0 => {
                    return Err(
                        LoopBreakCompositeBodyRoleRejectV1::SourceCallOutsideRole(site.clone()),
                    )
                }
                1 => method_call_sites.push(site.clone()),
                _ => {
                    return Err(
                        LoopBreakCompositeBodyRoleRejectV1::SourceCallAmbiguous(site.clone()),
                    )
                }
            }
        }
    }
    Ok(VerifiedLoopBreakCompositeBodyRoleMapV1 {
        root,
        method_call_sites: method_call_sites.into_boxed_slice(),
    })
}

fn role_call_matches(
    role: &CompositeLoopBodyRoleV1,
    site: &SourceExprSiteV1,
    matches: &mut usize,
) {
    let role_segments = match role {
        CompositeLoopBodyRoleV1::Statement { site }
        | CompositeLoopBodyRoleV1::If { site, .. }
        | CompositeLoopBodyRoleV1::Exit { site, .. } => site.node().segments(),
        CompositeLoopBodyRoleV1::Loop { evidence, .. } => evidence.site.node().segments(),
    };
    if !site.node().segments().starts_with(role_segments) {
        return;
    }
    let children: &[CompositeLoopBodyRoleV1] = match role {
        CompositeLoopBodyRoleV1::Statement { .. } | CompositeLoopBodyRoleV1::Exit { .. } => &[],
        CompositeLoopBodyRoleV1::Loop { body, .. } => body,
        CompositeLoopBodyRoleV1::If {
            then_body,
            else_body,
            ..
        } => {
            let mut child_matches = 0usize;
            for child in then_body.iter() {
                role_call_matches(child, site, &mut child_matches);
            }
            if let Some(else_body) = else_body.as_deref() {
                for child in else_body {
                    role_call_matches(child, site, &mut child_matches);
                }
            }
            *matches += if child_matches == 0 { 1 } else { child_matches };
            return;
        }
    };
    let before = *matches;
    for child in children {
        role_call_matches(child, site, matches);
    }
    if *matches == before {
        *matches += 1;
    }
}

fn issue_loop_evidence(
    input: ResolvedFunctionLoweringInputV1<'_>,
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
) -> Result<Vec<(CompositeLoopEvidenceV1, RegionId)>, LoopBreakCompositeBodyRoleRejectV1> {
    if forest.owner() != input.owner() {
        return Err(LoopBreakCompositeBodyRoleRejectV1::ForeignOwner);
    }
    let mut members = Vec::with_capacity(forest.member_sites().len());
    for (index, site) in forest.member_sites().iter().enumerate() {
        let stmt = input
            .source()
            .exact_stmt(site)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
        let condition = input
            .source()
            .child_expr_from_stmt(&stmt, ExprChildRoleV1::LoopCondition)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
        let resolved = input
            .function()
            .resolved_loop_source(site)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::MissingLoopMember(site.clone()))?;
        let region = input
            .function()
            .loop_region_bundle(site)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::MissingLoopMember(site.clone()))?
            .loop_pair()
            .region();
        let parent_index = forest
            .forest_binding()
            .members()
            .get(index)
            .ok_or(LoopBreakCompositeBodyRoleRejectV1::ForestCoverage)?
            .parent_index();
        members.push((
            CompositeLoopEvidenceV1 {
                site: site.clone(),
                condition_site: condition.site().clone(),
                member_index: index as u32,
                parent_index,
                frame_key: resolved.frame_key(),
            },
            region,
        ));
    }
    Ok(members)
}

fn issue_loop_role(
    input: ResolvedFunctionLoweringInputV1<'_>,
    stmt: &LocatedStmtV1<'_>,
    body: &LocatedBodyV1<'_>,
    members: &[(CompositeLoopEvidenceV1, RegionId)],
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    seen_sites: &mut std::collections::BTreeSet<SourceStmtSiteV1>,
) -> Result<CompositeLoopBodyRoleV1, LoopBreakCompositeBodyRoleRejectV1> {
    let Some((evidence, _)) = members.iter().find(|(evidence, _)| evidence.site() == stmt.site())
    else {
        return Err(LoopBreakCompositeBodyRoleRejectV1::MissingLoopMember(
            stmt.site().clone(),
        ));
    };
    let roles = issue_body_roles(input, body, members, forest, seen_sites)?;
    Ok(CompositeLoopBodyRoleV1::Loop {
        evidence: evidence.clone(),
        body: roles,
    })
}

fn issue_body_roles(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &LocatedBodyV1<'_>,
    members: &[(CompositeLoopEvidenceV1, RegionId)],
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    seen: &mut std::collections::BTreeSet<SourceStmtSiteV1>,
) -> Result<Box<[CompositeLoopBodyRoleV1]>, LoopBreakCompositeBodyRoleRejectV1> {
    let mut roles = Vec::with_capacity(body.statements().len());
    for index in 0..body.statements().len() {
        let stmt = input
            .source()
            .body_stmt(body, index)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
        if !seen.insert(stmt.site().clone()) {
            return Err(LoopBreakCompositeBodyRoleRejectV1::DuplicateSite(
                stmt.site().clone(),
            ));
        }
        roles.push(issue_statement_role(input, &stmt, members, forest, seen)?);
    }
    Ok(roles.into_boxed_slice())
}

fn issue_statement_role(
    input: ResolvedFunctionLoweringInputV1<'_>,
    stmt: &LocatedStmtV1<'_>,
    members: &[(CompositeLoopEvidenceV1, RegionId)],
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    seen: &mut std::collections::BTreeSet<SourceStmtSiteV1>,
) -> Result<CompositeLoopBodyRoleV1, LoopBreakCompositeBodyRoleRejectV1> {
    if let Some((_evidence, _)) = members.iter().find(|(evidence, _)| evidence.site() == stmt.site())
    {
        let body = input
            .source()
            .child_body_from_stmt(stmt, BodyChildRoleV1::LoopBody)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
        return issue_loop_role(input, stmt, &body, members, forest, seen);
    }

    if matches!(stmt.node(), ASTNode::Loop { .. }) {
        return Err(LoopBreakCompositeBodyRoleRejectV1::MissingLoopMember(
            stmt.site().clone(),
        ));
    }
    if matches!(stmt.node(), ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. })
    {
        let record = input
            .function()
            .resolved_exit(&ResolvedExitSiteV1::Statement(stmt.site().clone()))
            .copied()
            .ok_or_else(|| LoopBreakCompositeBodyRoleRejectV1::MissingExit(stmt.site().clone()))?;
        let target_is_known = match record.transfer() {
            ResolvedControlTransferV1::Break { target_loop }
            | ResolvedControlTransferV1::Continue { target_loop } => {
                members.iter().any(|(_, region)| *region == target_loop)
            }
            ResolvedControlTransferV1::Return { .. } => true,
        };
        if !target_is_known {
            return Err(LoopBreakCompositeBodyRoleRejectV1::ExitOutsideForest(
                stmt.site().clone(),
            ));
        }
        let branch_owner = enclosing_if_site(input.function(), record.source_region());
        return Ok(CompositeLoopBodyRoleV1::Exit {
            site: stmt.site().clone(),
            branch_owner,
            record,
        });
    }

    if let ASTNode::If { else_body, .. } = stmt.node() {
        let then_body = input
            .source()
            .child_body_from_stmt(stmt, BodyChildRoleV1::IfThen)
            .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
        let then_roles = issue_body_roles(input, &then_body, members, forest, seen)?;
        let else_roles = if else_body.is_some() {
            let body = input
                .source()
                .child_body_from_stmt(stmt, BodyChildRoleV1::IfElse)
                .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
            Some(issue_body_roles(input, &body, members, forest, seen)?)
        } else {
            None
        };
        let explicit = then_roles.iter().any(CompositeLoopBodyRoleV1::contains_control)
            || else_roles
                .as_deref()
                .is_some_and(|body| body.iter().any(CompositeLoopBodyRoleV1::contains_control));
        if explicit {
            let condition = input
                .source()
                .child_expr_from_stmt(stmt, ExprChildRoleV1::IfCondition)
                .map_err(|_| LoopBreakCompositeBodyRoleRejectV1::SourceNavigation)?;
            return Ok(CompositeLoopBodyRoleV1::If {
                site: stmt.site().clone(),
                condition_site: condition.site().clone(),
                then_body: then_roles,
                else_body: else_roles,
            });
        }
    }
    if matches!(
        stmt.node(),
        ASTNode::ScopeBox { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::TryCatch { .. }
    ) {
        return Err(LoopBreakCompositeBodyRoleRejectV1::UnsupportedShape(
            stmt.site().clone(),
        ));
    }
    Ok(CompositeLoopBodyRoleV1::Statement {
        site: stmt.site().clone(),
    })
}

fn collect_loop_sites(
    role: &CompositeLoopBodyRoleV1,
    sites: &mut std::collections::BTreeSet<SourceStmtSiteV1>,
) {
    match role {
        CompositeLoopBodyRoleV1::Loop { evidence, body } => {
            sites.insert(evidence.site.clone());
            for child in body.iter() {
                collect_loop_sites(child, sites);
            }
        }
        CompositeLoopBodyRoleV1::If {
            then_body,
            else_body,
            ..
        } => {
            for child in then_body.iter() {
                collect_loop_sites(child, sites);
            }
            if let Some(else_body) = else_body.as_deref() {
                for child in else_body {
                    collect_loop_sites(child, sites);
                }
            }
        }
        CompositeLoopBodyRoleV1::Statement { .. } | CompositeLoopBodyRoleV1::Exit { .. } => {}
    }
}

fn enclosing_if_site(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    source_region: RegionId,
) -> Option<SourceStmtSiteV1> {
    let mut current = source_region;
    loop {
        for site in function.if_region_sites() {
            let Ok(bundle) = function.if_region_bundle(site) else {
                continue;
            };
            if bundle.then_pair().region() == current
                || bundle.else_pair().is_some_and(|pair| pair.region() == current)
            {
                return Some(site.clone());
            }
        }
        let record = function.region(current)?;
        current = record.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::{role_call_matches, CompositeLoopBodyRoleV1};
    use crate::mir::resolved_semantics::{
        SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    };

    fn stmt(index: u32) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
        ]))
    }

    fn expr(index: u32) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
            SourcePathSegmentV1::Value,
        ]))
    }

    #[test]
    fn source_call_coverage_rejects_outside_role() {
        let role = CompositeLoopBodyRoleV1::Statement { site: stmt(1) };
        let mut matches = 0;
        role_call_matches(&role, &expr(2), &mut matches);
        assert_eq!(matches, 0);
    }

    #[test]
    fn source_call_coverage_accepts_one_nested_role() {
        let root = stmt(1);
        let child = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfThen(0),
        ]));
        let role = CompositeLoopBodyRoleV1::If {
            site: root,
            condition_site: expr(1),
            then_body: vec![CompositeLoopBodyRoleV1::Statement { site: child }]
                .into_boxed_slice(),
            else_body: None,
        };
        let call = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ]));
        let mut matches = 0;
        role_call_matches(&role, &call, &mut matches);
        assert_eq!(matches, 1);
    }

    #[test]
    fn source_call_coverage_reports_ambiguous_duplicate_branch() {
        let root = stmt(1);
        let duplicate = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfThen(0),
        ]));
        let role = CompositeLoopBodyRoleV1::If {
            site: root,
            condition_site: expr(1),
            then_body: vec![CompositeLoopBodyRoleV1::Statement {
                site: duplicate.clone(),
            }]
            .into_boxed_slice(),
            else_body: Some(
                vec![CompositeLoopBodyRoleV1::Statement { site: duplicate }]
                    .into_boxed_slice(),
            ),
        };
        let call = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ]));
        let mut matches = 0;
        role_call_matches(&role, &call, &mut matches);
        assert_eq!(matches, 2);
    }
}
