//! Private physical owner for the legacy raw ordinary `new` body.
//!
//! This is a behavior-neutral BoxShape extraction. It owns no source or
//! semantic admission; the selected direct-body Raw cohort enters through the
//! package-claim port before this physical consumer runs.

use super::{CallTarget, Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
    RawOrdinaryNewClaimPortV1,
};
use crate::mir::definitions::call_unified::Callee;
use crate::mir::normal_callable_semantic_package::OrdinaryNewConstructorDispositionV1;
use crate::mir::slot_registry::resolve_slot_by_type_name;
use hakorune_mir_defs::CanonicalGlobalTargetV1;

pub(in crate::mir::builder) fn lower_ordinary_raw_new_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    class: &str,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1 + RawOrdinaryNewClaimPortV1,
{
    let claim = port.try_take_ordinary_new_claim(class, arguments.len())?;
    let constructor = claim.map(|claim| claim.constructor());
    if let Some(OrdinaryNewConstructorDispositionV1::Birth(recipe)) = constructor.as_ref() {
        let physical_arity = arguments.len().checked_add(1).ok_or_else(|| {
            "[freeze:contract][ordinary-new/abi/physical-arity-overflow]".to_owned()
        })?;
        recipe
            .abi()
            .validate(arguments.len(), physical_arity)
            .map_err(|error| format!("[freeze:contract][ordinary-new/abi/{error:?}]"))?;
    }
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

    if let Some(constructor) = constructor {
        if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
            let effects = recipe.physical_effect_mask();
            builder.emit_instruction(MirInstruction::call(
                None,
                Callee::BirthConstructor { key: recipe.target(), receiver: dst },
                arg_values,
                effects,
            ))?;
        }
        return Ok(dst);
    }

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
            let target = CanonicalGlobalTargetV1::new_static_box_method(
                class.into(),
                "birth".into(),
                arity as u32,
            )
            .map_err(|error| format!("[freeze:contract][ordinary-new/birth/{error:?}]"))?;
            builder.emit_legacy_call(None, CallTarget::Global(target), argv)?;
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
