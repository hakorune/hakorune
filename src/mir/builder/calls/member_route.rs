//! Member-call route planning and emission handoff.
//!
//! This box decides the member-call lowering lane once and then emits from the
//! selected plan without re-probing receiver shape.

use super::super::me_call_header_observation::MethodCallLoweringPortV1;
use super::super::normal_script_semantic_lowering_state::ScriptDirectStaticClaimTakeV1;
use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::super::{MirBuilder, ValueId};
use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{
    lower_method_call_receiver_v1, AssociatedMethodCallArgumentsV1, MethodCallArgumentDescentV1,
};
use super::receiver_binding::ReceiverNormalizationPlan;
use super::script_direct_static_physical_bridge::lower_claimed_script_direct_static_v1;
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

    pub(in crate::mir::builder) fn build_member_method_call_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        let route_plan = {
            let syntax = port.method_call_syntax(input)?;
            self.plan_member_call_route(syntax.receiver(), syntax.method())?
        };

        self.execute_prepared_member_call_route_v1(port, input, route_plan)
    }

    pub(in crate::mir::builder) fn build_member_method_call_with_claim_ingress_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1 + RecursiveChildLoweringPortV1,
    {
        let route_plan = {
            let syntax = port.method_call_syntax(input)?;
            self.plan_member_call_route(syntax.receiver(), syntax.method())?
        };

        match route_plan {
            MemberCallRoutePlan::StaticReceiver { box_name } => {
                let claim = {
                    let syntax = port.method_call_syntax(input)?;
                    port.take_script_direct_static_claim_v1(
                        &box_name,
                        syntax.method(),
                        syntax.receiver(),
                        syntax.arguments(),
                    )?
                };
                if let ScriptDirectStaticClaimTakeV1::Claimed(claimed) = claim {
                    return lower_claimed_script_direct_static_v1(self, port, input, claimed);
                }
                let (method, arguments) = {
                    let syntax = port.method_call_syntax(input)?;
                    (syntax.method().to_owned(), syntax.arguments())
                };
                let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                self.handle_static_method_call_with_descent(
                    &box_name,
                    &method,
                    arguments,
                    &mut descent,
                )
            }
            route_plan => self.execute_prepared_member_call_route_v1(port, input, route_plan),
        }
    }

    /// Executes exactly one existing member-route plan without re-planning.
    ///
    /// The split lets candidate-only callers inspect an already-selected
    /// route, while the ordinary facade preserves the existing plan-once then
    /// execute-once behavior.
    pub(in crate::mir::builder) fn execute_prepared_member_call_route_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
        route_plan: MemberCallRoutePlan,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        let (method, arguments) = {
            let syntax = port.method_call_syntax(input)?;
            (syntax.method().to_string(), syntax.arguments())
        };

        match route_plan {
            MemberCallRoutePlan::StaticReceiver { box_name } => {
                let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                self.handle_static_method_call_with_descent(
                    &box_name,
                    &method,
                    arguments,
                    &mut descent,
                )
            }
            MemberCallRoutePlan::EnvMethod { spec } => {
                let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                let arg_values = descent.lower_all(self)?;
                descent.finish_env_value_terminal(self, &spec, arg_values)
            }
            MemberCallRoutePlan::ReceiverNormalized { plan } => match plan {
                ReceiverNormalizationPlan::MeCall => {
                    let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                    self.handle_me_method_call_with_descent(&method, arguments, &mut descent)?
                        .ok_or_else(|| {
                            format!("[member-call-route] unresolved me receiver for {}", method)
                        })
                }
                ReceiverNormalizationPlan::StaticThis { box_name } => {
                    let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                    self.handle_static_method_call_with_descent(
                        &box_name,
                        &method,
                        arguments,
                        &mut descent,
                    )
                }
            },
            MemberCallRoutePlan::Standard => {
                let object_value = lower_method_call_receiver_v1(self, port, input)?;

                if crate::config::env::builder_static_call_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[P287-DEBUG] After build_expression: object_value={:?}",
                        object_value
                    ));
                }

                let receiver = port.method_call_syntax(input)?.receiver();
                self.trace_receiver_if_enabled(receiver, object_value);

                let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
                self.handle_standard_method_call_with_descent(
                    object_value,
                    method,
                    arguments,
                    &mut descent,
                )
            }
        }
    }
}
