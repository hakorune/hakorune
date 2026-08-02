//! Verified Script receipts for structured lowering descendants.

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    project_source_node_v1, EnumVariantAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootSemanticDispositionV1, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
    SourceStmtSiteV1, VerifiedResolvedScriptV1, VerifiedScriptRootDemandWindowV1,
};

#[derive(Debug)]
pub(super) struct ScriptOperationalDemandReceiptPackV1 {
    record_literal_demands: Box<[VerifiedScriptRecordLiteralDemandV1]>,
    enum_variant_demands: Box<[VerifiedScriptEnumVariantDemandV1]>,
    enum_match_demands: Box<[VerifiedScriptEnumMatchDemandV1]>,
    qmark_propagations: Box<[VerifiedScriptQMarkPropagationV1]>,
    match_controls: Box<[VerifiedScriptMatchControlDemandV1]>,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptRecordLiteralDemandV1 {
    pub(super) site: SourceExprSiteV1,
    pub(super) explicit_field_count: u32,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptEnumVariantDemandV1 {
    pub(super) site: SourceExprSiteV1,
    pub(super) admission: EnumVariantAdmissionV1,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptEnumMatchDemandV1 {
    pub(super) site: SourceExprSiteV1,
}

/// A source-only authorization for the existing QMark control/result owner.
#[derive(Debug)]
pub(super) struct VerifiedScriptQMarkPropagationV1 {
    pub(super) site: SourceExprSiteV1,
    pub(super) operand_site: SourceExprSiteV1,
    pub(super) target: ScriptQMarkPropagationTargetV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScriptQMarkPropagationTargetV1 {
    CurrentScriptOwner,
}

/// Source-only receipt for a root Match expression.
#[derive(Debug)]
pub(super) struct VerifiedScriptMatchControlDemandV1 {
    pub(super) site: SourceExprSiteV1,
    pub(super) arm_count: u32,
}

impl ScriptOperationalDemandReceiptPackV1 {
    pub(super) fn seal(
        source: &PreparedNormalDefaultProgramRootV1,
        product: &VerifiedResolvedScriptV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, String> {
        let record_literal_demands = product
            .record_literal_demands()
            .map(|(site, explicit_field_count)| {
                let projected = project_source_node_v1(source.source_ast(), site.node())
                    .ok_or_else(|| {
                        "[mir/script-semantic/record-projection] missing exact RecordLiteral"
                            .to_owned()
                    })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::RecordLiteral { fields, .. },
                ) = projected
                else {
                    return Err(
                        "[mir/script-semantic/record-site] expected RecordLiteral".to_owned()
                    );
                };
                if fields.len() != explicit_field_count as usize {
                    return Err("[mir/script-semantic/record-cardinality] mismatch".to_owned());
                }
                Ok(VerifiedScriptRecordLiteralDemandV1 {
                    site: site.clone(),
                    explicit_field_count,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let enum_variant_demands = product
            .enum_variant_demands()
            .map(|(site, admission)| {
                let projected = project_source_node_v1(source.source_ast(), site.node())
                    .ok_or_else(|| {
                        "[mir/script-semantic/enum-variant-projection] missing exact FromCall"
                            .to_owned()
                    })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::FromCall { arguments, .. },
                ) = projected
                else {
                    return Err(
                        "[mir/script-semantic/enum-variant-site] expected FromCall".to_owned()
                    );
                };
                if arguments.len() != admission.argument_count() as usize {
                    return Err(
                        "[mir/script-semantic/enum-variant-cardinality] mismatch".to_owned()
                    );
                }
                for index in 0..admission.argument_count() {
                    let child_site = SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Argument(index))
                        .expr();
                    if !matches!(
                        project_source_node_v1(source.source_ast(), child_site.node()),
                        Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                            _
                        ))
                    ) {
                        return Err(
                            "[mir/script-semantic/enum-variant-argument-site] expected node"
                                .to_owned(),
                        );
                    }
                }
                Ok(VerifiedScriptEnumVariantDemandV1 {
                    site: site.clone(),
                    admission: admission.clone(),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let enum_match_demands = product
            .enum_match_demands()
            .map(|site| {
                let projected = project_source_node_v1(source.source_ast(), site.node())
                    .ok_or_else(|| {
                        "[mir/script-semantic/enum-match-projection] missing exact EnumMatchExpr"
                            .to_owned()
                    })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::EnumMatchExpr { .. },
                ) = projected
                else {
                    return Err(
                        "[mir/script-semantic/enum-match-site] expected EnumMatchExpr".to_owned(),
                    );
                };
                let scrutinee_site = SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::EnumMatchScrutinee)
                    .expr();
                if !matches!(
                    project_source_node_v1(source.source_ast(), scrutinee_site.node()),
                    Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                        _
                    ))
                ) {
                    return Err(
                        "[mir/script-semantic/enum-match-scrutinee-site] expected node".to_owned(),
                    );
                }
                Ok(VerifiedScriptEnumMatchDemandV1 { site: site.clone() })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let qmark_propagations = product
            .qmark_propagation_sites()
            .map(|site| {
                let source_statement_index =
                    program_statement_index(&SourceStmtSiteV1::from_node(site.node().clone()))?;
                let Some(entry) = window.entry_at(source_statement_index) else {
                    return Err("[mir/script-semantic/qmark-window] missing root demand".to_owned());
                };
                if entry.site().node() != site.node()
                    || !matches!(
                        entry.semantic(),
                        ScriptRootSemanticDispositionV1::Resolved(
                            ScriptRootResolvedDemandV1::QMarkPropagation(_)
                        )
                    )
                {
                    return Err("[mir/script-semantic/qmark-window] source mismatch".to_owned());
                }
                let projected = project_source_node_v1(source.source_ast(), site.node())
                    .ok_or_else(|| {
                        "[mir/script-semantic/qmark-projection] missing QMark".to_owned()
                    })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::QMarkPropagate { .. },
                ) = projected
                else {
                    return Err(
                        "[mir/script-semantic/qmark-site] expected QMarkPropagate".to_owned()
                    );
                };
                let operand_site = SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::QMarkOperand)
                    .expr();
                if !matches!(
                    project_source_node_v1(source.source_ast(), operand_site.node()),
                    Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                        _
                    ))
                ) {
                    return Err("[mir/script-semantic/qmark-operand-site] expected node".to_owned());
                }
                Ok(VerifiedScriptQMarkPropagationV1 {
                    site: site.clone(),
                    operand_site,
                    target: ScriptQMarkPropagationTargetV1::CurrentScriptOwner,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let match_controls = product
            .match_control_sites()
            .map(|site| {
                let source_statement_index =
                    program_statement_index(&SourceStmtSiteV1::from_node(site.node().clone()))?;
                let Some(entry) = window.entry_at(source_statement_index) else {
                    return Err("[mir/script-semantic/match-window] missing root demand".to_owned());
                };
                if entry.site().node() != site.node()
                    || !matches!(
                        entry.semantic(),
                        ScriptRootSemanticDispositionV1::Resolved(
                            ScriptRootResolvedDemandV1::MatchControl(_)
                        )
                    )
                {
                    return Err("[mir/script-semantic/match-window] source mismatch".to_owned());
                }
                let projected = project_source_node_v1(source.source_ast(), site.node())
                    .ok_or_else(|| {
                        "[mir/script-semantic/match-projection] missing MatchExpr".to_owned()
                    })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::MatchExpr { arms, .. },
                ) = projected
                else {
                    return Err("[mir/script-semantic/match-site] expected MatchExpr".to_owned());
                };
                let arm_count = u32::try_from(arms.len())
                    .map_err(|_| "[mir/script-semantic/match-arm-count] overflow".to_owned())?;
                let mut roles = Vec::with_capacity(arms.len() + 2);
                roles.push(SourcePathSegmentV1::MatchScrutinee);
                roles.extend((0..arm_count).map(SourcePathSegmentV1::MatchArm));
                roles.push(SourcePathSegmentV1::MatchElse);
                for role in roles {
                    let child_site = SourcePathV1::from_node(site.node()).child(role).expr();
                    if !matches!(
                        project_source_node_v1(source.source_ast(), child_site.node()),
                        Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                            _
                        ))
                    ) {
                        return Err(
                            "[mir/script-semantic/match-child-site] expected node".to_owned()
                        );
                    }
                }
                Ok(VerifiedScriptMatchControlDemandV1 {
                    site: site.clone(),
                    arm_count,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Self {
            record_literal_demands: record_literal_demands.into_boxed_slice(),
            enum_variant_demands: enum_variant_demands.into_boxed_slice(),
            enum_match_demands: enum_match_demands.into_boxed_slice(),
            qmark_propagations: qmark_propagations.into_boxed_slice(),
            match_controls: match_controls.into_boxed_slice(),
        })
    }

    pub(super) fn record_literal_demands(&self) -> &[VerifiedScriptRecordLiteralDemandV1] {
        &self.record_literal_demands
    }

    pub(super) fn enum_variant_demands(&self) -> &[VerifiedScriptEnumVariantDemandV1] {
        &self.enum_variant_demands
    }

    pub(super) fn enum_match_demands(&self) -> &[VerifiedScriptEnumMatchDemandV1] {
        &self.enum_match_demands
    }

    pub(super) fn qmark_propagations(&self) -> &[VerifiedScriptQMarkPropagationV1] {
        &self.qmark_propagations
    }

    pub(super) fn match_controls(&self) -> &[VerifiedScriptMatchControlDemandV1] {
        &self.match_controls
    }
}

fn program_statement_index(site: &SourceStmtSiteV1) -> Result<usize, String> {
    match site.node().segments() {
        [SourcePathSegmentV1::ProgramBodyRoot, SourcePathSegmentV1::ProgramBody(index)] => {
            Ok(*index as usize)
        }
        _ => Err("[mir/script-semantic/window-site] expected ProgramBody ordinal".to_owned()),
    }
}
