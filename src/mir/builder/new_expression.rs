//! Raw `new` expression preparation and lowering.
//!
//! This module owns only the legacy raw construction route. It does not issue
//! semantic plans or alter the canonical Dynamic production path.

use super::{ConstValue, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
    RawOrdinaryNewClaimPortV1,
};

pub(in crate::mir::builder) struct PreparedRawNewExpressionV1 {
    class: String,
    route: PreparedRawNewExpressionRouteV1,
    ordinary_claim:
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
    selected_ordinary_claim: bool,
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
    /// Only evaluated expressions occupy the structured child demand queue.
    pub(in crate::mir::builder) fn evaluated_argument_count(&self) -> usize {
        match &self.route {
            PreparedRawNewExpressionRouteV1::Core13Pure { arguments }
            | PreparedRawNewExpressionRouteV1::Ordinary { arguments } => {
                usize::from(!self.selected_ordinary_claim) * arguments.len()
            }
            PreparedRawNewExpressionRouteV1::IntegerLiteral { .. } => 0,
        }
    }

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
            ordinary_claim: None,
            selected_ordinary_claim: false,
            field_initializers,
            _seal: PreparedRawNewExpressionSealV1,
        })
    }

    pub(in crate::mir::builder) fn prepare_ordinary_claim_v1<Port>(
        &mut self,
        builder: &MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawOrdinaryNewClaimPortV1,
    {
        let PreparedRawNewExpressionRouteV1::Ordinary { arguments } = &self.route else {
            return Ok(());
        };
        let claim = port.try_take_ordinary_new_claim(&self.class, arguments.len())?;
        if claim
            .as_ref()
            .is_some_and(|claim| claim.argument_rows().is_err())
        {
            return Err("[freeze:contract][ordinary-new/argument-source-unavailable]".into());
        }
        let selected = match &claim {
            Some(claim) => port.prepare_ordinary_new_emission(builder, claim)?,
            None => false,
        };
        self.ordinary_claim = claim;
        self.selected_ordinary_claim = selected;
        Ok(())
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
        Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1 + RawOrdinaryNewClaimPortV1,
    {
        let PreparedRawNewExpressionV1 {
            class,
            route,
            ordinary_claim,
            selected_ordinary_claim,
            field_initializers,
            _seal: _,
        } = prepared;
        let ordinary = matches!(&route, PreparedRawNewExpressionRouteV1::Ordinary { .. });
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
            PreparedRawNewExpressionRouteV1::Ordinary { arguments: _ }
                if selected_ordinary_claim =>
            {
                port.emit_ordinary_new_claim(
                    self,
                    ordinary_claim.expect("selected ordinary claim"),
                )?
            }
            PreparedRawNewExpressionRouteV1::Ordinary { arguments } => {
                super::ordinary_new_admission::lower_ordinary_raw_new_with_port_v1(
                    self,
                    port,
                    &class,
                    arguments,
                    ordinary_claim,
                )?
            }
        };

        self.build_box_field_initializers_with_port_v1(port, dst, &class, field_initializers)?;
        if ordinary {
            port.complete_ordinary_new_expression(&class, dst)?;
        }
        Ok(dst)
    }
}

#[cfg(test)]
#[path = "new_expression_tests.rs"]
mod tests;
