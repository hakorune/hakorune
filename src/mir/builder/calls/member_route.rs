//! Member-call route planning and emission handoff.
//!
//! This box decides the member-call lowering lane once and then emits from the
//! selected plan without re-probing receiver shape.

use super::super::{MirBuilder, ValueId};
use super::extern_calls::EnvMethodSpec;
use super::receiver_binding::ReceiverNormalizationPlan;
use super::special_handlers;
use crate::ast::ASTNode;

pub(in crate::mir::builder) enum MemberCallRoutePlan {
    StaticReceiver { box_name: String },
    EnvMethod { spec: EnvMethodSpec },
    ReceiverNormalized { plan: ReceiverNormalizationPlan },
    Standard,
}

impl MirBuilder {
    pub(in crate::mir::builder) fn plan_member_call_route(
        &mut self,
        object: &ASTNode,
        method: &str,
    ) -> Result<MemberCallRoutePlan, String> {
        if let Some(box_name) = self.resolve_static_receiver_box_name(object) {
            return Ok(MemberCallRoutePlan::StaticReceiver { box_name });
        }
        if let Some(spec) = super::extern_calls::resolve_env_method_call(object, method) {
            return Ok(MemberCallRoutePlan::EnvMethod { spec });
        }
        if let Some(plan) = self.classify_this_me_method_call(object)? {
            return Ok(MemberCallRoutePlan::ReceiverNormalized { plan });
        }
        Ok(MemberCallRoutePlan::Standard)
    }

    pub(in crate::mir::builder) fn emit_member_call_from_plan(
        &mut self,
        route_plan: MemberCallRoutePlan,
        object: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        match route_plan {
            MemberCallRoutePlan::StaticReceiver { box_name } => {
                self.handle_static_method_call(&box_name, &method, &arguments)
            }
            MemberCallRoutePlan::EnvMethod { spec } => {
                self.emit_resolved_env_method_call(&spec, &arguments)
            }
            MemberCallRoutePlan::ReceiverNormalized { plan } => match plan {
                ReceiverNormalizationPlan::MeCall => self
                    .handle_me_method_call(&method, &arguments)?
                    .ok_or_else(|| {
                        format!("[member-call-route] unresolved me receiver for {}", method)
                    }),
                ReceiverNormalizationPlan::StaticThis { box_name } => {
                    self.handle_static_method_call(&box_name, &method, &arguments)
                }
            },
            MemberCallRoutePlan::Standard => {
                let object_value = self.build_expression(object.clone())?;

                if crate::config::env::builder_static_call_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[P287-DEBUG] After build_expression: object_value={:?}",
                        object_value
                    ));
                }

                self.trace_receiver_if_enabled(&object, object_value);

                if let Some(type_name) = special_handlers::is_typeop_method(&method, &arguments) {
                    return self.handle_typeop_method(object_value, &method, &type_name);
                }

                self.handle_standard_method_call(object_value, method, &arguments)
            }
        }
    }

    pub(in crate::mir::builder) fn emit_resolved_env_method_call(
        &mut self,
        spec: &EnvMethodSpec,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String> {
        let arg_values = self.build_call_args(arguments)?;
        let result_id = self.next_value_id();
        let dst = if spec.returns { Some(result_id) } else { None };
        self.emit_extern_call_with_effects(
            &spec.iface_name,
            &spec.method_name,
            arg_values,
            dst,
            spec.effects,
        )?;
        if spec.returns {
            Ok(result_id)
        } else {
            crate::mir::builder::emission::constant::emit_void(self)
        }
    }
}
