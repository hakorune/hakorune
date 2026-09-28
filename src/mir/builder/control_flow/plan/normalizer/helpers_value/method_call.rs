//! Value-position `MethodCall` lowering split from `lower.rs` at the
//! 800-line boundary. Admission consults (static publication, core-method
//! rows, lexical instance-call dispositions, `me`/`this` locator) all live
//! in the shared ports/helpers; this file only sequences the arm.

use super::*;
use crate::ast::{ASTNode, LiteralValue};

impl super::super::PlanNormalizer {
    pub(in crate::mir::builder) fn lower_method_call_value_input<'input, P>(
        port: &P,
        input: &P::ExprInput<'input>,
        object: &ASTNode,
        method: &String,
        arguments: &[ASTNode],
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, Vec<CoreEffectPlan>), String>
    where
        P: LoopPlanExpressionPortV1 + 'input,
    {
        let exact_source_call = port
            .exact_source_method_call(input, method, arguments.len() as u32)
            .map_err(|error| format!("[normalizer] {error}"))?;
        let mut arg_ids = Vec::new();
        let mut arg_effects = Vec::new();
        for (index, _) in arguments.iter().enumerate() {
            let argument = port
                .child_expr(input, ExprChildRoleV1::CallArgument(index as u32))
                .map_err(|error| error.render())?;
            let (arg_id, mut effects) =
                Self::lower_value_input(port, argument, builder, phi_bindings)?;
            arg_ids.push(arg_id);
            arg_effects.append(&mut effects);
        }
        let call_source = port.call_source(input).map_err(|error| error.render())?;

        let result_id = builder.next_value_id();
        let result_type = exact_source_call
            .as_ref()
            .map(|call| call.result_type())
            .unwrap_or_else(|| match object {
                ASTNode::Variable { name, .. } if name == "env" => {
                    extern_calls::get_env_method_return_type("env", method)
                        .unwrap_or(MirType::Unknown)
                }
                _ => MirType::Unknown,
            });
        builder
            .function_state
            .type_ctx
            .set_type(result_id, result_type);

        if let Some(exact_source_call) = exact_source_call {
            if let Some(target) = exact_source_call.static_target() {
                if target.namespace()
                    != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
                    || target.name() != method
                    || target.arity() != arguments.len() as u32
                {
                    return Err(
                        "[freeze:contract][callable-loop/static-publication-target]".to_owned(),
                    );
                }
                arg_effects.push(CoreEffectPlan::GlobalCall {
                    source: call_source.clone(),
                    dst: Some(result_id),
                    func: target.mir_symbol_projection(),
                    args: arg_ids,
                });
            } else {
                if !matches!(object, ASTNode::Variable { .. }) {
                    return Err(
                        "[freeze:contract][callable-loop/core-method-receiver-shape]".to_owned(),
                    );
                }
                let receiver = exact_source_call.receiver().ok_or_else(|| {
                    "[freeze:contract][callable-loop/core-method-receiver-missing]".to_owned()
                })?;
                arg_effects.push(CoreEffectPlan::MethodCall {
                    source: call_source.clone(),
                    dst: Some(result_id),
                    object: receiver,
                    method: method.clone(),
                    args: arg_ids,
                    effects: exact_source_call.effects(),
                });
            }
        } else {
            match object {
                ASTNode::Variable { name, .. } if name == "env" => {
                    let Some((iface_name, method_name, effects, returns_value)) =
                        extern_calls::get_env_method_spec("env", method)
                    else {
                        return Err(format!(
                            "[normalizer] env method not supported: {}",
                            method
                        ));
                    };
                    if !returns_value {
                        return Err(format!(
                            "[normalizer] env method used as value: {}",
                            method
                        ));
                    }
                    arg_effects.push(CoreEffectPlan::ExternCall {
                        source: call_source.clone(),
                        dst: Some(result_id),
                        iface_name,
                        method_name,
                        args: arg_ids,
                        effects,
                    });
                }
                ASTNode::Variable { name, .. } => {
                    if let Some(plan) = declared_instance_call_effect(
                        port,
                        input,
                        method,
                        arguments.len() as u32,
                        Some(result_id),
                        arg_ids.clone(),
                        call_source.clone(),
                        "[normalizer]",
                    )? {
                        arg_effects.push(plan);
                    } else if let Some(value_id) =
                        Self::lookup_variable_value(builder, phi_bindings, name)
                    {
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            source: call_source.clone(),
                            dst: Some(result_id),
                            object: value_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    } else if builder.comp_ctx.user_defined_boxes.contains_key(name) {
                        let func = format!("{}.{}/{}", name, method, arguments.len());
                        arg_effects.push(CoreEffectPlan::GlobalCall {
                            source: call_source.clone(),
                            dst: Some(result_id),
                            func,
                            args: arg_ids,
                        });
                    } else {
                        return Err(format!(
                            "[normalizer] Method call object {} not found",
                            name
                        ));
                    }
                }
                ASTNode::Literal {
                    value: LiteralValue::String(_),
                    ..
                } => {
                    let object = port
                        .child_expr(input, ExprChildRoleV1::Receiver)
                        .map_err(|error| error.render())?;
                    let (object_id, mut object_effects) =
                        Self::lower_value_input(port, object, builder, phi_bindings)?;
                    arg_effects.append(&mut object_effects);
                    arg_effects.push(CoreEffectPlan::MethodCall {
                        source: call_source.clone(),
                        dst: Some(result_id),
                        object: object_id,
                        method: method.clone(),
                        args: arg_ids,
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
                ASTNode::Me { .. } | ASTNode::This { .. } => {
                    arg_effects.push(me_this_method_call_effect(
                        port,
                        object,
                        || {
                            port.child_expr(input, ExprChildRoleV1::Receiver)
                                .map_err(|error| error.render())
                        },
                        builder,
                        phi_bindings,
                        input,
                        method,
                        arg_ids,
                        arguments.len(),
                        Some(result_id),
                        call_source.clone(),
                        "[normalizer]",
                        "[normalizer] me.method() without bound receiver".to_string(),
                        "[normalizer] this.method() without current_static_box".to_string(),
                    )?);
                }
                ASTNode::FieldAccess { .. }
                | ASTNode::ThisField { .. }
                | ASTNode::MeField { .. } => {
                    let object = port
                        .child_expr(input, ExprChildRoleV1::Receiver)
                        .map_err(|error| error.render())?;
                    let (object_id, mut object_effects) =
                        Self::lower_value_input(port, object, builder, phi_bindings)?;
                    arg_effects.append(&mut object_effects);
                    arg_effects.push(CoreEffectPlan::MethodCall {
                        source: call_source.clone(),
                        dst: Some(result_id),
                        object: object_id,
                        method: method.clone(),
                        args: arg_ids,
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
                ASTNode::MethodCall {
                    object: _callee,
                    method: _,
                    arguments: _,
                    ..
                } => {
                    // Nested receiver calls must materialize the full inner call result.
                    // Lowering only the inner callee base (for example `arr` in
                    // `arr.get(idx).length()`) loses the receiver chain and
                    // misbinds the outer method to the wrong object.
                    let object = port
                        .child_expr(input, ExprChildRoleV1::Receiver)
                        .map_err(|error| error.render())?;
                    let (object_id, mut object_effects) =
                        Self::lower_value_input(port, object, builder, phi_bindings)?;
                    arg_effects.append(&mut object_effects);
                    arg_effects.push(CoreEffectPlan::MethodCall {
                        source: call_source.clone(),
                        dst: Some(result_id),
                        object: object_id,
                        method: method.clone(),
                        args: arg_ids,
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
                _ => {
                    let object = port
                        .child_expr(input, ExprChildRoleV1::Receiver)
                        .map_err(|error| error.render())?;
                    let (object_id, mut object_effects) =
                        Self::lower_value_input(port, object, builder, phi_bindings)?;
                    arg_effects.append(&mut object_effects);
                    arg_effects.push(CoreEffectPlan::MethodCall {
                        source: call_source,
                        dst: Some(result_id),
                        object: object_id,
                        method: method.clone(),
                        args: arg_ids,
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
            }
        }

        Ok((result_id, arg_effects))
    }
}
