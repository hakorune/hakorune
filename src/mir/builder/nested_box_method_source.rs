//! Behavior-neutral owner for nested Box method lowering.
//!
//! The following T2 replaces the legacy unlocated transport with one
//! function-relative source product.  This S0 deliberately preserves the
//! existing collector admission and transport so the two production callers
//! have one small home before that cutover.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::MirBuilder;

use super::module_lowering_invocation::LegacyChildDraftAdmissionV1;
use super::raw_invocation_source_transport::{
    RawInvocationSourceContextV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
    RawUnlocatedPortalV1,
};
use super::recursive_child_lowering::{
    normalize_instance_box_method_input_v1, RawInvocationChildPortV1,
};

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn lower_static_box_method_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    builder: &mut MirBuilder,
    function_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
) -> Result<(), String> {
    require_nested_box_source_v1(port, "static")?;
    builder.observe_legacy_method_lowering_v1(&function_name, &body, None);
    let admission = LegacyChildDraftAdmissionV1::legacy_symbol(function_name.clone(), params.len());
    let pending = port
        .with_source_transport_v1(
            RawInvocationSourceTransportV1::unlocated((), RawUnlocatedPortalV1::NestedBoxAdmission),
            |port, ()| {
                port.capture_static_box_method_pending_v1(
                    builder,
                    function_name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                )
            },
        )
        .map_err(|error| error.to_string())?;
    port.commit_legacy_nested_box_method_pending_v1(pending, admission)
        .map_err(|error| error.to_string())
}

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn lower_instance_box_method_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    builder: &mut MirBuilder,
    function_name: String,
    box_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
) -> Result<(), String> {
    require_nested_box_source_v1(port, "instance")?;
    let (params, param_decls) =
        normalize_instance_box_method_input_v1(&function_name, params, param_decls);
    builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&box_name));
    let admission =
        LegacyChildDraftAdmissionV1::legacy_symbol(function_name.clone(), params.len() + 1);
    let pending = port
        .with_source_transport_v1(
            RawInvocationSourceTransportV1::unlocated((), RawUnlocatedPortalV1::NestedBoxAdmission),
            |port, ()| {
                port.capture_normalized_instance_box_method_pending_v1(
                    builder,
                    function_name,
                    box_name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                )
            },
        )
        .map_err(|error| error.to_string())?;
    port.commit_legacy_nested_box_method_pending_v1(pending, admission)
        .map_err(|error| error.to_string())
}

fn require_nested_box_source_v1(
    port: &RawInvocationChildPortV1<'_, '_>,
    kind: &str,
) -> Result<(), String> {
    if matches!(
        port.current_source_context_v1(),
        Some(RawInvocationSourceContextV1::Located { .. })
            | Some(RawInvocationSourceContextV1::UnlocatedCompatibility(
                RawUnlocatedPortalV1::NestedBoxAdmission
            ))
    ) {
        Ok(())
    } else {
        Err(format!(
            "[freeze:contract][raw-invocation/nested-{kind}-box-missing-site]"
        ))
    }
}
