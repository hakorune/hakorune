//! Function-relative source owner for nested Box method lowering.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::MirBuilder;

use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    normalize_instance_box_method_input_v1, RawInvocationChildPortV1,
};

/// Owned method-map selection and lowering input.  The map key is preserved
/// where it coexists with the owned body; the raw port must not reconstruct it.
pub(in crate::mir::builder) struct NestedBoxMethodLoweringInputV1 {
    method_key: String,
    function_name: String,
    kind: NestedBoxMethodKindV1,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

pub(in crate::mir::builder) enum NestedBoxMethodKindV1 {
    Static,
    Instance { owner: String },
}

impl NestedBoxMethodLoweringInputV1 {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn static_method(
        method_key: String,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Self {
        Self {
            method_key,
            function_name,
            kind: NestedBoxMethodKindV1::Static,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn instance_method(
        method_key: String,
        function_name: String,
        owner: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Self {
        Self {
            method_key,
            function_name,
            kind: NestedBoxMethodKindV1::Instance { owner },
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        String,
        String,
        NestedBoxMethodKindV1,
        Vec<String>,
        Vec<ParamDecl>,
        Option<String>,
        Vec<ASTNode>,
        Vec<String>,
        DeclarationAttrs,
    ) {
        (
            self.method_key,
            self.function_name,
            self.kind,
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}

struct PreparedNestedBoxMethodSourceV1 {
    lineage: RawInvocationRootLineageV1,
}

impl PreparedNestedBoxMethodSourceV1 {
    fn from_located_parent(
        parent: Option<RawInvocationSourceContextV1>,
        method_key: String,
    ) -> Result<Self, String> {
        let Some(RawInvocationSourceContextV1::Located { site, .. }) = parent else {
            return Err(
                "[freeze:contract][raw-invocation/nested-box-missing-located-parent]".to_owned(),
            );
        };
        Ok(Self {
            lineage: RawInvocationRootLineageV1::nested_box_method(site, method_key),
        })
    }

    fn transport(&self) -> RawInvocationSourceTransportV1<()> {
        RawInvocationSourceTransportV1::root((), self.lineage.clone())
    }
}

pub(in crate::mir::builder) fn lower_nested_box_method_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    builder: &mut MirBuilder,
    input: NestedBoxMethodLoweringInputV1,
) -> Result<(), String> {
    let (method_key, function_name, kind, params, param_decls, return_type_name, body, uses, attrs) =
        input.into_parts();
    let source = PreparedNestedBoxMethodSourceV1::from_located_parent(
        port.current_source_context_v1(),
        method_key,
    )?;
    match kind {
        NestedBoxMethodKindV1::Static => {
            builder.observe_legacy_method_lowering_v1(&function_name, &body, None);
            let arity = params.len();
            let pending = port
                .with_source_transport_v1(source.transport(), |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        function_name.clone(),
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )
                })
                .map_err(|error| error.to_string())?;
            port.commit_legacy_nested_box_method_symbol_pending_v1(pending, function_name, arity)
                .map_err(|error| error.to_string())
        }
        NestedBoxMethodKindV1::Instance { owner } => {
            let (params, param_decls) =
                normalize_instance_box_method_input_v1(&function_name, params, param_decls);
            builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&owner));
            let arity = params.len() + 1;
            let pending = port
                .with_source_transport_v1(source.transport(), |port, ()| {
                    port.capture_normalized_instance_box_method_pending_v1(
                        builder,
                        function_name.clone(),
                        owner,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )
                })
                .map_err(|error| error.to_string())?;
            port.commit_legacy_nested_box_method_symbol_pending_v1(pending, function_name, arity)
                .map_err(|error| error.to_string())
        }
    }
}

#[cfg(test)]
#[path = "nested_box_method_source_tests.rs"]
mod tests;
