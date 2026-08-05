//! Script-only positive receipt admission for the shared shadow traversal.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::source_site::SourceExprSiteV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::product::ShadowResolveErrorV0;
use super::resolver::ShadowResolverV0;

impl<'ast, 'schema> ShadowResolverV0<'ast, 'schema> {
    pub(super) fn admit_record_literal(
        &mut self,
        site: SourceExprSiteV1,
        record_type_name: &str,
        fields: &[(String, ASTNode)],
    ) -> Result<(), ShadowResolveErrorV0> {
        let Some(admission) = self
            .record_schema_demand
            .and_then(|schemas| schemas.admit_fully_explicit_literal(record_type_name, fields))
        else {
            return Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: "RecordLiteral",
                site,
            });
        };
        if self
            .record_literal_demands
            .insert(site.clone(), admission.explicit_field_count())
            .is_some()
        {
            return Err(ShadowResolveErrorV0::DuplicateRecordLiteralDemand { site });
        }
        Ok(())
    }

    pub(super) fn admit_enum_variant(
        &mut self,
        site: SourceExprSiteV1,
        enum_name: &str,
        variant_name: &str,
        arguments: &[ASTNode],
    ) -> Result<(), ShadowResolveErrorV0> {
        let Some(admission) = self
            .enum_variant_demand
            .and_then(|variants| variants.admit_direct_variant(enum_name, variant_name, arguments))
        else {
            return Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: "FromCall",
                site,
            });
        };
        if self
            .enum_variant_demands
            .insert(site.clone(), admission)
            .is_some()
        {
            return Err(ShadowResolveErrorV0::DuplicateEnumVariantDemand { site });
        }
        Ok(())
    }

    /// Script semantics owns only the direct EnumMatch scrutinee demand.
    /// Arm syntax stays with the existing EnumMatch lowering owner.
    pub(super) fn resolve_direct_enum_match(
        &mut self,
        expression: &'ast ASTNode,
        enum_name: &str,
        scrutinee: &'ast ASTNode,
        arms: &[crate::ast::EnumMatchArm],
        else_expr: Option<&'ast ASTNode>,
        path: &super::path::ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let site = path.expr();
        let Some(_) = self
            .enum_match_demand
            .and_then(|demand| demand.admit_direct_enum_match(enum_name, arms, else_expr))
        else {
            return Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: "EnumMatchExpr",
                site,
            });
        };
        if !self.enum_match_demands.insert(site.clone()) {
            return Err(ShadowResolveErrorV0::DuplicateEnumMatchDemand { site });
        }
        self.resolve_expr(
            scrutinee,
            &path.child(
                ExprChildRoleV1::EnumMatchScrutinee
                    .segment_for(expression)
                    .expect("[freeze:contract][source_path/enum_match_scrutinee]"),
            ),
        )
    }

    pub(super) fn admit_qmark_propagation(
        &mut self,
        site: SourceExprSiteV1,
    ) -> Result<(), ShadowResolveErrorV0> {
        if !self.qmark_propagation_sites.insert(site.clone()) {
            return Err(ShadowResolveErrorV0::DuplicateQMarkPropagation { site });
        }
        Ok(())
    }

    pub(super) fn admit_match_control(
        &mut self,
        site: SourceExprSiteV1,
    ) -> Result<(), ShadowResolveErrorV0> {
        if !self.match_control_sites.insert(site.clone()) {
            return Err(ShadowResolveErrorV0::DuplicateMatchControl { site });
        }
        Ok(())
    }
}
