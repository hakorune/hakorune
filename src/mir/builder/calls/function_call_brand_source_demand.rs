//! Exact callable-owned Brand disposition for one raw `FunctionCall`.
//!
//! The resolver-issued callable projection is the only semantic issuer here.
//! This port snapshots that row before preflight; it never re-derives a Brand
//! from an AST name or from the mutable compatibility map.

use crate::ast::ASTNode;
use crate::mir::builder::raw_invocation_source_transport::RawSourceTransportPortV1;
use crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1;
use crate::mir::builder::recursive_child_lowering::{
    RawInvocationChildPortV1, RawLegacyChildLoweringPortV1,
};
use crate::mir::resolved_semantics::SourceNodeSiteV1;

use super::super::brand_constructor_lowering_projection::ProjectedBrandConstructorV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawBrandCallAuthorityV1 {
    RelationlessCompatibility,
    InstalledNonBrand,
    InstalledConstructor(ProjectedBrandConstructorV1),
}

pub(in crate::mir::builder) trait BrandConstructorSourcePortV1 {
    fn brand_call_authority_v1(
        &mut self,
        call: &ASTNode,
    ) -> Result<RawBrandCallAuthorityV1, String>;
}

impl BrandConstructorSourcePortV1 for RawLegacyChildLoweringPortV1 {
    fn brand_call_authority_v1(
        &mut self,
        _call: &ASTNode,
    ) -> Result<RawBrandCallAuthorityV1, String> {
        Ok(RawBrandCallAuthorityV1::RelationlessCompatibility)
    }
}

impl<Port> BrandConstructorSourcePortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: BrandConstructorSourcePortV1,
{
    fn brand_call_authority_v1(
        &mut self,
        call: &ASTNode,
    ) -> Result<RawBrandCallAuthorityV1, String> {
        self.child_mut().brand_call_authority_v1(call)
    }
}

impl BrandConstructorSourcePortV1 for RawInvocationChildPortV1<'_, '_> {
    fn brand_call_authority_v1(
        &mut self,
        call: &ASTNode,
    ) -> Result<RawBrandCallAuthorityV1, String> {
        let ASTNode::FunctionCall {
            name, arguments, ..
        } = call
        else {
            return Ok(RawBrandCallAuthorityV1::RelationlessCompatibility);
        };
        let Some(ledger) = self.callable_ledger.clone() else {
            return Ok(RawBrandCallAuthorityV1::RelationlessCompatibility);
        };
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| "[freeze:contract][callable-brand/missing-source-context]".to_owned())?;
        let site = context
            .site()
            .cloned()
            .ok_or_else(|| "[freeze:contract][callable-brand/missing-source-site]".to_owned())?;
        let row = ledger
            .borrow_mut()
            .take_brand_constructor(&site)
            .map_err(|error| format!("[freeze:contract][callable-brand/{error:?}]"))?;
        let Some(row) = row else {
            return Ok(RawBrandCallAuthorityV1::InstalledNonBrand);
        };
        validate_constructor_call(&context, &site, call, name, arguments, &row)?;
        Ok(RawBrandCallAuthorityV1::InstalledConstructor(row))
    }
}

fn validate_constructor_call(
    context: &super::super::raw_invocation_source_transport::RawInvocationSourceContextV1,
    site: &SourceNodeSiteV1,
    call: &ASTNode,
    name: &str,
    arguments: &[ASTNode],
    row: &ProjectedBrandConstructorV1,
) -> Result<(), String> {
    if row.call_site() != site || row.name() != name {
        return Err("[freeze:contract][callable-brand/call-site-or-name-drift]".to_owned());
    }
    if arguments.is_empty() {
        return Ok(());
    }
    let operand = context
        .child_expression(
            call,
            crate::mir::resolved_semantics::ExprChildRoleV1::CallArgument(0),
        )?
        .site()
        .cloned()
        .ok_or_else(|| "[freeze:contract][callable-brand/missing-operand-site]".to_owned())?;
    if row.operand_site() != &operand {
        return Err("[freeze:contract][callable-brand/operand-site-drift]".to_owned());
    }
    Ok(())
}
