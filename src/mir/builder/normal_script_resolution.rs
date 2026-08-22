//! Script forest outcome adapter for the selected normal root lifecycle.
//!
//! This child keeps resolver outcome preservation out of the already
//! near-limit lifecycle orchestrator. It does not issue a window, target,
//! Recipe, or lowering route.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptForestOutcomeV1,
    ScriptResolverDeferredV1, ScriptSyntaxViewV1, VerifiedScriptRootDemandWindowV1,
};

use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;

#[derive(Debug)]
pub(super) enum NormalScriptResolutionV1<'source> {
    Complete(VerifiedScriptSemanticSourceV1<'source>),
    Deferred(ScriptResolverDeferredV1),
}

pub(super) fn resolve_normal_script_source_v1<'source>(
    source_ast: &'source ASTNode,
    window: Option<&VerifiedScriptRootDemandWindowV1>,
    declaration_facts: &PreparedNormalProgramDeclarationFactsV1,
    resolver: &mut FunctionSemanticResolverSessionV1,
) -> Result<Option<NormalScriptResolutionV1<'source>>, Box<str>> {
    let Some(window) = window else {
        return Ok(None);
    };
    let view = ScriptSyntaxViewV1::from_program(source_ast).ok_or_else(|| {
        Box::<str>::from("[mir/script-semantic/source-root] expected Program")
    })?;
    let outcome = declaration_facts.with_record_schema_demand_view(|record_schemas| {
        declaration_facts.with_enum_variant_demand_view(|enum_variants| {
            declaration_facts.with_enum_match_demand_view(|enum_matches| {
                declaration_facts.with_brand_catalog(|brand_catalog| {
                    resolver.resolve_script_forest_with_declaration_views(
                        view,
                        window,
                        record_schemas,
                        enum_variants,
                        enum_matches,
                        brand_catalog,
                    )
                })
            })
        })
    })
    .map_err(|error| -> Box<str> { format!("[mir/script-semantic/seal] {error:?}").into() })?;
    match outcome {
        ResolveScriptForestOutcomeV1::Complete(forest) => Ok(Some(
            NormalScriptResolutionV1::Complete(
                VerifiedScriptSemanticSourceV1::seal_ast_with_forest(
                    source_ast, forest, window,
                )
                .map_err(|error| -> Box<str> { error.to_string().into() })?,
            ),
        )),
        ResolveScriptForestOutcomeV1::Deferred(deferred) => {
            Ok(Some(NormalScriptResolutionV1::Deferred(deferred)))
        }
    }
}
