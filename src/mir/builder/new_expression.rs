//! Raw `new` expression preparation and lowering.
//!
//! This module owns only the legacy raw construction route. It does not issue
//! semantic plans or alter the canonical Dynamic production path.

use super::{ConstValue, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};

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
                super::ordinary_new_admission::lower_ordinary_raw_new_with_port_v1(
                    self, port, &class, arguments,
                )?
            }
        };

        self.build_box_field_initializers_with_port_v1(port, dst, &class, field_initializers)?;
        Ok(dst)
    }
}

#[cfg(test)]
#[path = "new_expression_tests.rs"]
mod tests;
