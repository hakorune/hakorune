use crate::ast::ParamDecl;

use super::model::ResolverMethodParameterSyntaxV1;

/// Preserve the existing neutral `ParamDecl` fallback projection.
///
/// This issuer intentionally has no parser brand, parameter ordinal receipt,
/// or transfer classification. The subsequent source-seal I0 replaces this
/// neutral projection with the complete sibling catalog atomically.
pub(in crate::parser) fn project_neutral_parameter_syntax_v1(
    param_decls: &[ParamDecl],
    params: &[String],
) -> Box<[ResolverMethodParameterSyntaxV1]> {
    ParamDecl::with_name_fallback(param_decls, params)
        .iter()
        .map(|param| {
            ResolverMethodParameterSyntaxV1::from_neutral_syntax(
                param.name.clone(),
                param.declared_type_name.clone(),
            )
        })
        .collect()
}
