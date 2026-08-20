//! Resolver-issued source continuation for the Complete Script root.
//!
//! This product keeps the already-sealed body-shape relations alive until a
//! later Recipe producer can consume them.  It owns no Recipe key, ValueId,
//! join signature, result ABI, or physical block.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BodyShapeRelationV1, BodyStatementShapeV1, FunctionOwnerIdV1, ScriptRootResolvedDemandV1,
    ScriptRootSemanticDispositionV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
    VerifiedScriptRootDemandWindowV1, VerifiedSemanticOwnerForestV1,
    VerifiedSemanticOwnerProductV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ScriptSourceContinuationIssueV1 {
    RootCardinality,
    RootProduct,
    RootOwnerMismatch,
    MissingStatementWindow(SourceStmtSiteV1),
    WindowStatementNotInBodyShape(SourceStmtSiteV1),
    ReturnNotFinal(SourceStmtSiteV1),
    ReturnAdmissionMismatch(SourceStmtSiteV1),
    MissingParent(SourceExprSiteV1),
    DuplicateParent(SourceExprSiteV1),
    DanglingParent(SourceNodeSiteV1),
    ParentCycle(SourceNodeSiteV1),
    DuplicateMethodCall(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ScriptSourceContinuationTerminalV1 {
    Sequence(SourceStmtSiteV1),
    Return(SourceStmtSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedScriptSourceContinuationRowV1 {
    owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    parent_relations: Box<[BodyShapeRelationV1]>,
    terminal: ScriptSourceContinuationTerminalV1,
}

impl VerifiedScriptSourceContinuationRowV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(super) fn parent_relations(&self) -> &[BodyShapeRelationV1] {
        &self.parent_relations
    }

    pub(super) const fn terminal(&self) -> &ScriptSourceContinuationTerminalV1 {
        &self.terminal
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedScriptSourceContinuationV1 {
    owner: FunctionOwnerIdV1,
    rows: BTreeMap<SourceExprSiteV1, VerifiedScriptSourceContinuationRowV1>,
}

impl VerifiedScriptSourceContinuationV1 {
    pub(super) fn issue(
        forest: &VerifiedSemanticOwnerForestV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, ScriptSourceContinuationIssueV1> {
        let [root] = forest.roots() else {
            return Err(ScriptSourceContinuationIssueV1::RootCardinality);
        };
        let Some(product) = forest
            .semantic_owner(*root)
            .and_then(VerifiedSemanticOwnerProductV1::as_script)
        else {
            return Err(ScriptSourceContinuationIssueV1::RootProduct);
        };
        let owner = product.core().data().owner;
        let body_shape = product.body_shape();
        if body_shape.owner() != owner {
            return Err(ScriptSourceContinuationIssueV1::RootOwnerMismatch);
        }
        validate_statement_window(body_shape.statements(), window)?;

        let expression_sites = body_shape
            .expressions()
            .iter()
            .map(|expression| expression_site(expression))
            .collect::<BTreeSet<_>>();
        let mut rows = BTreeMap::new();
        for (site, method) in product.method_calls() {
            if method.owner() != owner {
                return Err(ScriptSourceContinuationIssueV1::RootOwnerMismatch);
            }
            let (parent_relations, terminal) = find_terminal(
                site,
                body_shape.statements(),
                &expression_sites,
                body_shape.relations(),
                window,
            )?;
            let row = VerifiedScriptSourceContinuationRowV1 {
                owner,
                call_site: site.clone(),
                parent_relations,
                terminal,
            };
            if rows.insert(site.clone(), row).is_some() {
                return Err(ScriptSourceContinuationIssueV1::DuplicateMethodCall(
                    site.clone(),
                ));
            }
        }
        Ok(Self { owner, rows })
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn row(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptSourceContinuationRowV1> {
        self.rows.get(site)
    }

    pub(super) fn rows(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedScriptSourceContinuationRowV1)> {
        self.rows.iter()
    }
}

fn validate_statement_window(
    statements: &[BodyStatementShapeV1],
    window: &VerifiedScriptRootDemandWindowV1,
) -> Result<(), ScriptSourceContinuationIssueV1> {
    let mut covered = BTreeSet::new();
    for (index, statement) in statements.iter().enumerate() {
        let Some((window_index, entry)) = window
            .entries()
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.site().node() == statement.site().node())
        else {
            return Err(ScriptSourceContinuationIssueV1::MissingStatementWindow(
                statement.site().clone(),
            ));
        };
        covered.insert(statement.site().node().clone());
        if statement.is_return() {
            if !window.is_final_ordinal(window_index) || index + 1 != statements.len() {
                return Err(ScriptSourceContinuationIssueV1::ReturnNotFinal(
                    statement.site().clone(),
                ));
            }
            if !matches!(
                entry.semantic(),
                ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(
                    _
                ))
            ) {
                return Err(ScriptSourceContinuationIssueV1::ReturnAdmissionMismatch(
                    statement.site().clone(),
                ));
            }
        }
    }
    for entry in window.entries() {
        // Complete Script windows also retain transparent/transferred and
        // diagnostic boundaries.  Those entries deliberately never enter
        // the resolver body-shape traversal, so absence from the shape is
        // not a continuation gap.  Only an admitted semantic row may claim
        // a body-shape statement here.
        if !matches!(
            entry.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(_)
        ) {
            continue;
        }
        if !covered.contains(entry.site().node()) {
            return Err(
                ScriptSourceContinuationIssueV1::WindowStatementNotInBodyShape(
                    entry.site().clone(),
                ),
            );
        }
    }
    Ok(())
}

fn find_terminal(
    call_site: &SourceExprSiteV1,
    statements: &[BodyStatementShapeV1],
    expression_sites: &BTreeSet<SourceExprSiteV1>,
    relations: &[BodyShapeRelationV1],
    window: &VerifiedScriptRootDemandWindowV1,
) -> Result<
    (
        Box<[BodyShapeRelationV1]>,
        ScriptSourceContinuationTerminalV1,
    ),
    ScriptSourceContinuationIssueV1,
> {
    let mut current = call_site.node().clone();
    let mut seen = BTreeSet::new();
    let mut path = Vec::new();
    loop {
        if !seen.insert(current.clone()) {
            return Err(ScriptSourceContinuationIssueV1::ParentCycle(current));
        }
        if let Some(statement) = statements
            .iter()
            .find(|statement| statement.site().node() == &current)
        {
            let (index, entry) = window
                .entries()
                .iter()
                .enumerate()
                .find(|(_, entry)| entry.site().node() == &current)
                .ok_or_else(|| {
                    ScriptSourceContinuationIssueV1::MissingStatementWindow(
                        statement.site().clone(),
                    )
                })?;
            let terminal = if statement.is_return() {
                if !window.is_final_ordinal(index) {
                    return Err(ScriptSourceContinuationIssueV1::ReturnNotFinal(
                        statement.site().clone(),
                    ));
                }
                if !matches!(
                    entry.semantic(),
                    ScriptRootSemanticDispositionV1::Resolved(
                        ScriptRootResolvedDemandV1::ReturnExit(_)
                    )
                ) {
                    return Err(ScriptSourceContinuationIssueV1::ReturnAdmissionMismatch(
                        statement.site().clone(),
                    ));
                }
                ScriptSourceContinuationTerminalV1::Return(statement.site().clone())
            } else {
                ScriptSourceContinuationTerminalV1::Sequence(statement.site().clone())
            };
            return Ok((path.into_boxed_slice(), terminal));
        }
        if !expression_sites.contains(&SourceExprSiteV1::from_node(current.clone())) {
            return Err(ScriptSourceContinuationIssueV1::DanglingParent(current));
        }
        let parents = relations
            .iter()
            .filter(|relation| relation.child().node() == &current)
            .collect::<Vec<_>>();
        let relation = match parents.as_slice() {
            [] => {
                return Err(ScriptSourceContinuationIssueV1::MissingParent(
                    SourceExprSiteV1::from_node(current),
                ))
            }
            [relation] => *relation,
            _ => {
                return Err(ScriptSourceContinuationIssueV1::DuplicateParent(
                    SourceExprSiteV1::from_node(current),
                ))
            }
        };
        current = relation.parent().clone();
        path.push(relation.clone());
    }
}

fn expression_site(
    expression: &crate::mir::resolved_semantics::BodyExpressionShapeV1,
) -> SourceExprSiteV1 {
    match expression {
        crate::mir::resolved_semantics::BodyExpressionShapeV1::Variable { site, .. }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::QualifiedReceiver { site }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { site, .. }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { site, .. }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::MethodCall { site, .. }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::BlockExpr { site }
        | crate::mir::resolved_semantics::BodyExpressionShapeV1::Other { site, .. } => site.clone(),
    }
}
