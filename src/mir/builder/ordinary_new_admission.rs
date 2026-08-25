//! Private physical owner for the legacy raw ordinary `new` body.
//!
//! This is a behavior-neutral BoxShape extraction. It owns no source or
//! semantic admission; the later D2c cutover will replace this body with the
//! package-claim consumer for the selected direct-body Raw cohort.

use super::{CallTarget, Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use crate::mir::slot_registry::resolve_slot_by_type_name;

pub(in crate::mir::builder) fn lower_ordinary_raw_new_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    class: &str,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
{
    let mut arg_values = Vec::new();
    for arg in arguments {
        arg_values.push(drive_legacy_expression_v1(builder, port, arg)?);
    }

    let dst = builder.next_value_id();
    builder.emit_instruction(MirInstruction::NewBox {
        dst,
        box_type: class.to_owned(),
        args: arg_values.clone(),
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, super::MirType::Box(class.to_owned()));
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(dst, class.to_owned());

    // Prefer a lowered global `<Class>.birth/Arity`; retain the
    // builtin/plugin compatibility policy otherwise.
    if class != "StringBox" {
        let arity = arg_values.len();
        let lowered = crate::mir::builder::calls::function_lowering::generate_method_function_name(
            &class, "birth", arity,
        );
        let use_lowered = port.with_function_headers(|headers| match headers {
            Some(view) => view.contains_symbol(&lowered),
            None => builder
                .current_module
                .as_ref()
                .is_some_and(|module| module.functions.contains_key(&lowered)),
        });
        if use_lowered {
            let mut argv: Vec<ValueId> = Vec::with_capacity(1 + arity);
            argv.push(dst);
            argv.extend(arg_values.iter().copied());
            builder.emit_legacy_call(None, CallTarget::Global(lowered), argv)?;
        } else {
            let is_user_box = builder.comp_ctx.user_defined_boxes.contains_key(class);
            let allow_builtin_birth = crate::config::env::builder_birth_inject_builtins();
            if !is_user_box && allow_builtin_birth {
                let birt_mid = resolve_slot_by_type_name(&class, "birth");
                builder.emit_box_or_plugin_call(
                    None,
                    dst,
                    "birth".to_string(),
                    birt_mid,
                    arg_values,
                    EffectMask::READ.add(Effect::ReadHeap),
                )?;
            }
        }
    }
    Ok(dst)
}
