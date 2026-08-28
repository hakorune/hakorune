//! Exact callable-owned Brand disposition for one raw `FunctionCall`.
//!
//! The resolver-issued callable projection is the only semantic issuer here.
//! This port snapshots that row before preflight; it never re-derives a Brand
//! from an AST name or from the mutable compatibility map.

use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::raw_invocation_source_transport::RawSourceTransportPortV1;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1;
use crate::mir::builder::recursive_child_lowering::{
    RawInvocationChildPortV1, RawLegacyChildLoweringPortV1,
};
use crate::mir::resolved_semantics::SourceNodeSiteV1;

use super::super::brand_constructor_lowering_projection::ProjectedBrandConstructorV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawBrandCallAuthorityV1 {
    RelationlessCompatibility,
    /// A semantic ScriptRoot has an existing source owner but no callable
    /// target product.  Preserve its compatibility terminal explicitly;
    /// absence of a callable ledger is never itself a resolver authority.
    ScriptRootParkedCompatibility,
    /// A ledger-less raw ScriptRoot is an explicit compatibility source for
    /// ordinary Program/AppMain calls.  It carries no target authority.
    RawScriptRootParkedCompatibility,
    /// A ledger-less raw-root Main(locator) is kept distinct from ScriptRoot
    /// so its compatibility provenance cannot be inferred from absence.
    RawRootMainParkedCompatibility,
    /// The direct legacy facade is an explicit compatibility owner.  It has
    /// no source ledger and never becomes a target resolver.
    RawLegacyParkedCompatibility,
    /// The raw call is inside the exact installed App Main owner scope.  The
    /// target itself remains in the package loan; this variant carries no
    /// name/arity or physical symbol and therefore cannot become a resolver.
    InstalledAppMain,
    InstalledNonBrand {
        caller: Option<CanonicalSameModuleCallableKeyV1>,
    },
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
        Ok(RawBrandCallAuthorityV1::RawLegacyParkedCompatibility)
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
        let context = self.current_source_context_v1();
        if self.semantic_ledger.is_some()
            && context
                .as_ref()
                .is_some_and(is_semantic_script_root_compatibility_context_v1)
        {
            return Ok(RawBrandCallAuthorityV1::ScriptRootParkedCompatibility);
        }
        if self.semantic_ledger.is_none() && self.callable_ledger.is_none() {
            if context
                .as_ref()
                .is_some_and(is_raw_script_root_compatibility_context_v1)
            {
                return Ok(RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility);
            }
            if context
                .as_ref()
                .is_some_and(is_raw_root_main_compatibility_context_v1)
            {
                return Ok(RawBrandCallAuthorityV1::RawRootMainParkedCompatibility);
            }
        }
        let Some(ledger) = self.callable_ledger.clone() else {
            return Ok(RawBrandCallAuthorityV1::RelationlessCompatibility);
        };
        let context = context
            .ok_or_else(|| "[freeze:contract][callable-brand/missing-source-context]".to_owned())?;
        let caller = match &context {
            RawInvocationSourceContextV1::Located {
                root: RawInvocationRootLineageV1::Cataloged(caller),
                ..
            } => Some(caller.clone()),
            _ => None,
        };
        let site = context
            .site()
            .cloned()
            .ok_or_else(|| "[freeze:contract][callable-brand/missing-source-site]".to_owned())?;
        let row = ledger
            .borrow_mut()
            .take_brand_constructor(&site)
            .map_err(|error| format!("[freeze:contract][callable-brand/{error:?}]"))?;
        let Some(row) = row else {
            if self.is_app_main_direct_call_scope_v1() {
                return Ok(RawBrandCallAuthorityV1::InstalledAppMain);
            }
            return Ok(RawBrandCallAuthorityV1::InstalledNonBrand { caller });
        };
        validate_constructor_call(&context, &site, call, name, arguments, &row)?;
        Ok(RawBrandCallAuthorityV1::InstalledConstructor(row))
    }
}

fn is_semantic_script_root_compatibility_context_v1(
    context: &RawInvocationSourceContextV1,
) -> bool {
    match context {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            ..
        } => true,
        RawInvocationSourceContextV1::UnlocatedCompatibility {
            expected_lineage: Some(RawInvocationRootLineageV1::ScriptRoot),
            ..
        } => true,
        _ => false,
    }
}

fn is_raw_script_root_compatibility_context_v1(context: &RawInvocationSourceContextV1) -> bool {
    match context {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            ..
        }
        | RawInvocationSourceContextV1::UnlocatedCompatibility {
            expected_lineage: Some(RawInvocationRootLineageV1::ScriptRoot),
            ..
        } => true,
        _ => false,
    }
}

fn is_raw_root_main_compatibility_context_v1(context: &RawInvocationSourceContextV1) -> bool {
    match context {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Main(_),
            ..
        }
        | RawInvocationSourceContextV1::UnlocatedCompatibility {
            expected_lineage: Some(RawInvocationRootLineageV1::Main(_)),
            ..
        } => true,
        _ => false,
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

#[cfg(test)]
mod script_root_context_tests {
    use super::*;
    use crate::mir::builder::raw_invocation_source_transport::RawUnlocatedPortalV1;
    use crate::mir::resolved_semantics::{SourceBodyKindV1, SourcePathSegmentV1};

    fn located(root: RawInvocationRootLineageV1) -> RawInvocationSourceContextV1 {
        RawInvocationSourceContextV1::Located {
            root,
            site: SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::FunctionBody]),
            body_kind: Some(SourceBodyKindV1::Function),
        }
    }

    #[test]
    fn only_script_root_lineage_is_named_script_compatibility() {
        assert!(is_semantic_script_root_compatibility_context_v1(&located(
            RawInvocationRootLineageV1::ScriptRoot,
        )));
        assert!(is_semantic_script_root_compatibility_context_v1(
            &RawInvocationSourceContextV1::UnlocatedCompatibility {
                reason: RawUnlocatedPortalV1::CallObject,
                expected_lineage: Some(RawInvocationRootLineageV1::ScriptRoot),
            }
        ));
        assert!(!is_semantic_script_root_compatibility_context_v1(
            &RawInvocationSourceContextV1::UnlocatedCompatibility {
                reason: RawUnlocatedPortalV1::CallObject,
                expected_lineage: None,
            }
        ));
        assert!(is_raw_script_root_compatibility_context_v1(&located(
            RawInvocationRootLineageV1::ScriptRoot,
        )));
        assert!(is_raw_root_main_compatibility_context_v1(&located(
            RawInvocationRootLineageV1::Main(crate::mir::builder::RawSourceLocatorV1::for_test(
                0,
                "Main",
                "main",
                "Main.main/0",
                0,
            )),
        )));
        assert!(!is_raw_root_main_compatibility_context_v1(&located(
            RawInvocationRootLineageV1::ScriptRoot,
        )));
    }
}
