//! Raw `new` expression preparation and lowering.
//!
//! This module owns only the legacy raw construction route. It does not issue
//! semantic plans or alter the canonical Dynamic production path.

use super::{CallTarget, ConstValue, Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use crate::mir::slot_registry::resolve_slot_by_type_name;

pub(in crate::mir::builder) struct PreparedRawNewExpressionV1 {
    class: String,
    route: PreparedRawNewExpressionRouteV1,
    field_initializers: Vec<(String, ASTNode)>,
    _seal: PreparedRawNewExpressionSealV1,
}

enum PreparedRawNewExpressionRouteV1 {
    Core13Pure { arguments: Vec<ASTNode> },
    IntegerLiteral { value: i64 },
    Ordinary { arguments: Vec<ASTNode> },
}

struct PreparedRawNewExpressionSealV1;

impl PreparedRawNewExpressionV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        class: String,
        arguments: Vec<ASTNode>,
        field_initializers: Vec<(String, ASTNode)>,
    ) -> Result<Self, String> {
        if builder.is_record_constructor_class(&class) {
            if !field_initializers.is_empty() {
                return Err(format!(
                    "[box-init/record-reject] record={} does not support new-box field initializers",
                    class
                ));
            }
            return Err(format!(
                "[record-construction/escape] record={} supported_use=local-field-read",
                class
            ));
        }
        let route = if crate::config::env::mir_core13_pure() {
            PreparedRawNewExpressionRouteV1::Core13Pure { arguments }
        } else if let [ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        }] = arguments.as_slice()
        {
            if class == "IntegerBox" {
                PreparedRawNewExpressionRouteV1::IntegerLiteral { value: *value }
            } else {
                PreparedRawNewExpressionRouteV1::Ordinary { arguments }
            }
        } else {
            PreparedRawNewExpressionRouteV1::Ordinary { arguments }
        };
        Ok(Self {
            class,
            route,
            field_initializers,
            _seal: PreparedRawNewExpressionSealV1,
        })
    }
}

impl MirBuilder {
    /// Lower one prepared ordinary `new` route with the caller's child port.
    pub(in crate::mir::builder) fn lower_prepared_raw_new_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawNewExpressionV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
    {
        let PreparedRawNewExpressionV1 {
            class,
            route,
            field_initializers,
            _seal: _,
        } = prepared;
        // Phase 9.78a: Unified Box creation using NewBox instruction
        // Core-13 pure mode: emit ExternCall(env.box.new) with type name const only
        let dst = match route {
            PreparedRawNewExpressionRouteV1::Core13Pure { arguments } => {
                // Emit Const String for type name（ConstantEmissionBox）
                let ty_id =
                    crate::mir::builder::emission::constant::emit_string(self, class.clone())?;
                // Evaluate arguments (pass through to env.box.new shim)
                let mut arg_vals: Vec<ValueId> = Vec::with_capacity(arguments.len());
                for a in arguments {
                    arg_vals.push(drive_legacy_expression_v1(self, port, a)?);
                }
                // Build arg list: [type, a1, a2, ...]
                let mut args: Vec<ValueId> = Vec::with_capacity(1 + arg_vals.len());
                args.push(ty_id);
                args.extend(arg_vals);
                // Call env.box.new
                // 📦 Hotfix 3: Use next_value_id() to respect function parameter reservation
                let dst = self.next_value_id();
                self.emit_extern_call_with_effects(
                    "env.box",
                    "new",
                    args,
                    Some(dst),
                    EffectMask::PURE,
                )?;
                // 型注釈（最小）
                self.function_state
                    .type_ctx
                    .value_types
                    .insert(dst, super::MirType::Box(class.clone()));
                dst
            }
            PreparedRawNewExpressionRouteV1::IntegerLiteral { value } => {
                // Optimization: Primitive wrappers → emit Const directly when possible
                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                })?;
                self.function_state
                    .type_ctx
                    .value_types
                    .insert(dst, super::MirType::Integer);
                dst
            }
            PreparedRawNewExpressionRouteV1::Ordinary { arguments } => {
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(drive_legacy_expression_v1(self, port, arg)?);
                }

                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::NewBox {
                    dst,
                    box_type: class.clone(),
                    args: arg_values.clone(),
                })?;
                self.function_state
                    .type_ctx
                    .value_types
                    .insert(dst, super::MirType::Box(class.clone()));
                self.function_state
                    .type_ctx
                    .value_origin_newbox
                    .insert(dst, class.clone());

                // Prefer a lowered global `<Class>.birth/Arity`; retain the
                // builtin/plugin compatibility policy otherwise.
                if class != "StringBox" {
                    let arity = arg_values.len();
                    let lowered =
                    crate::mir::builder::calls::function_lowering::generate_method_function_name(
                        &class, "birth", arity,
                    );
                    let use_lowered = port.with_function_headers(|headers| match headers {
                        Some(view) => view.contains_symbol(&lowered),
                        None => self
                            .current_module
                            .as_ref()
                            .is_some_and(|module| module.functions.contains_key(&lowered)),
                    });
                    if use_lowered {
                        let mut argv: Vec<ValueId> = Vec::with_capacity(1 + arity);
                        argv.push(dst);
                        argv.extend(arg_values.iter().copied());
                        self.emit_legacy_call(None, CallTarget::Global(lowered), argv)?;
                    } else {
                        let is_user_box = self.comp_ctx.user_defined_boxes.contains_key(&class);
                        let allow_builtin_birth =
                            crate::config::env::builder_birth_inject_builtins();
                        if !is_user_box && allow_builtin_birth {
                            let birt_mid = resolve_slot_by_type_name(&class, "birth");
                            self.emit_box_or_plugin_call(
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
                dst
            }
        };

        self.build_box_field_initializers_with_port_v1(port, dst, &class, field_initializers)?;
        Ok(dst)
    }
}

#[cfg(test)]
#[path = "new_expression_tests.rs"]
mod tests;
