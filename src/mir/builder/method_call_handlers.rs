//! Method call handlers for MIR builder
//!
//! This module contains specialized handlers for different types of method calls,
//! following the Single Responsibility Principle.

use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::SameModuleCallableNamespaceV1;
use crate::mir::builder::calls::function_lowering;
use crate::mir::builder::calls::{
    emit_standard_value_terminal_raw_v1, AssociatedMethodCallArgumentsV1,
    LegacyMethodCallArgumentsV1, MethodCallArgumentDescentV1, MethodCallDescentPortV1,
    MethodCallValueTerminalPortV1,
};
use crate::mir::builder::me_call_header_observation::{
    prepare_me_lowered_call_v1, MethodCallLoweringPortV1, PreparedMeReceiverV1,
};
use crate::mir::builder::{MirBuilder, ValueId};
use crate::mir::TypeOpKind;

use super::record_helper_args::{
    PreparedRecordHelperInlineV1, PreparedSameModuleHelperSetterInlineV1,
};

/// Read-only standard-method route decision.  It carries helper eligibility
/// evidence, but cannot descend through arguments or emit MIR by itself.
#[derive(Debug)]
pub(in crate::mir::builder) enum PreparedStandardMethodExecutionV1 {
    WeakLoad,
    UpgradeRejected,
    RecordHelper(PreparedRecordHelperInlineV1),
    Setter(PreparedSameModuleHelperSetterInlineV1),
    Unified,
}

impl PreparedStandardMethodExecutionV1 {
    pub(in crate::mir::builder) const fn is_unified(&self) -> bool {
        matches!(self, Self::Unified)
    }
}

/// Read-only `me.method(...)` subroute decision.  It preserves the existing
/// route precedence while making Standard(Unified) distinguishable before any
/// argument descent or MIR emission.
#[derive(Debug)]
pub(in crate::mir::builder) enum PreparedMeCallExecutionV1 {
    InlineRecord {
        receiver: ValueId,
        prepared: PreparedRecordHelperInlineV1,
    },
    InlineSetter {
        receiver: ValueId,
        prepared: PreparedSameModuleHelperSetterInlineV1,
    },
    LoweredGlobal {
        owner: String,
        prepared: crate::mir::builder::me_call_header_observation::PreparedMeLoweredCallV1,
    },
    Standard {
        receiver: ValueId,
        prepared: PreparedStandardMethodExecutionV1,
    },
    StaticFallback {
        owner: String,
    },
    NotApplicable,
}

impl PreparedMeCallExecutionV1 {
    pub(in crate::mir::builder) const fn is_standard_unified(&self) -> bool {
        matches!(self, Self::Standard { prepared, .. } if prepared.is_unified())
    }
}

/// Me-call 専用のポリシー箱。
///
/// - 責務:
///   - me.method(...) を「インスタンス呼び出し」か「static メソッド呼び出し」か判定する。
///   - static box 文脈で実体のない receiver を生まないように、静的メソッド降下にフォールバックする。
struct MeCallPolicyBox;

fn current_enclosing_box_name(builder: &MirBuilder) -> Option<String> {
    if let Some(cls) = builder
        .function_state
        .current_function
        .as_ref()
        .and_then(|f| {
            f.signature
                .name
                .split_once('.')
                .map(|(cls, _)| cls.to_string())
        })
    {
        return Some(cls);
    }

    builder.comp_ctx.current_static_box.clone()
}

impl MeCallPolicyBox {
    fn resolve_me_call<Port>(
        builder: &mut MirBuilder,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<Option<ValueId>, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        let prepared = Self::prepare(builder, method, arguments, descent)?;
        Self::execute(builder, method, arguments, descent, prepared)
    }

    fn prepare<Port>(
        builder: &MirBuilder,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<PreparedMeCallExecutionV1, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        let Some(owner) = current_enclosing_box_name(builder) else {
            return Ok(PreparedMeCallExecutionV1::NotApplicable);
        };
        let me = current_bound_me_value(builder);

        if let Some(receiver) = me {
            if let Some(prepared) = builder.prepare_record_helper_inline(
                SameModuleCallableNamespaceV1::InstanceBoxMethod,
                &owner,
                method,
                arguments,
            )? {
                return Ok(PreparedMeCallExecutionV1::InlineRecord { receiver, prepared });
            }
            if let Some(prepared) =
                builder.prepare_same_module_helper_setter_inline(&owner, method, arguments)?
            {
                return Ok(PreparedMeCallExecutionV1::InlineSetter { receiver, prepared });
            }
        }

        let symbol =
            function_lowering::generate_method_function_name(&owner, method, arguments.len());
        let observation = descent.observe_me_call_parameters(builder, &symbol);
        if let Some(prepared) = prepare_me_lowered_call_v1(observation, me) {
            return Ok(PreparedMeCallExecutionV1::LoweredGlobal { owner, prepared });
        }

        if let Some(receiver) = me {
            let prepared =
                builder.prepare_standard_method_execution_v1(receiver, method, arguments)?;
            return Ok(PreparedMeCallExecutionV1::Standard { receiver, prepared });
        }

        Ok(PreparedMeCallExecutionV1::StaticFallback { owner })
    }

    fn execute<Port>(
        builder: &mut MirBuilder,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
        prepared: PreparedMeCallExecutionV1,
    ) -> Result<Option<ValueId>, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        match prepared {
            PreparedMeCallExecutionV1::InlineRecord { receiver, prepared } => builder
                .execute_prepared_record_helper_inline(prepared, arguments, Some(receiver), descent)
                .map(Some),
            PreparedMeCallExecutionV1::InlineSetter { receiver, prepared } => builder
                .execute_prepared_same_module_helper_setter_inline(
                    prepared,
                    arguments,
                    Some(receiver),
                    descent,
                )
                .map(Some),
            PreparedMeCallExecutionV1::LoweredGlobal { owner, prepared } => {
                Self::execute_lowered_global(builder, &owner, method, arguments, descent, prepared)
                    .map(Some)
            }
            PreparedMeCallExecutionV1::Standard { receiver, prepared } => {
                if let Some(result) = builder.execute_prepared_standard_method_execution_v1(
                    prepared, receiver, arguments, descent,
                )? {
                    return Ok(Some(result));
                }
                let arg_values = descent.lower_all(builder)?;
                descent
                    .finish_standard_value_terminal(
                        builder,
                        receiver,
                        method.to_string(),
                        arg_values,
                    )
                    .map(Some)
            }
            PreparedMeCallExecutionV1::StaticFallback { owner } => builder
                .handle_static_method_call_with_descent(&owner, method, arguments, descent)
                .map(Some),
            PreparedMeCallExecutionV1::NotApplicable => Ok(None),
        }
    }

    fn execute_lowered_global<Port>(
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
        prepared: crate::mir::builder::me_call_header_observation::PreparedMeLoweredCallV1,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        let symbol =
            function_lowering::generate_method_function_name(owner, method, arguments.len());
        let arg_values = descent.lower_all(builder)?;
        let (expected_params, receiver) = prepared.into_parts();
        let provided_static = arg_values.len();
        let provided_instance = arg_values.len() + 1;

        let call_args: Vec<ValueId> = match receiver {
            PreparedMeReceiverV1::Instance { me } => {
                let Some(me) = me else {
                    return Err(format!(
                        "[me-call] missing receiver for instance method {}",
                        symbol
                    ));
                };
                if expected_params != provided_instance {
                    Self::check_lowered_global_arity(
                        &symbol,
                        expected_params,
                        provided_instance,
                        true,
                    )?;
                }
                let mut values = Vec::with_capacity(provided_instance);
                values.push(me);
                values.extend(arg_values);
                values
            }
            PreparedMeReceiverV1::Static => {
                if expected_params != provided_static {
                    Self::check_lowered_global_arity(
                        &symbol,
                        expected_params,
                        provided_static,
                        false,
                    )?;
                }
                arg_values
            }
        };
        let checked_source_arity = u32::try_from(arguments.len()).map_err(|_| {
            format!(
                "[me-call] source arity exceeds u32 for {}.{}: {}",
                owner,
                method,
                arguments.len()
            )
        })?;
        descent.finish_me_lowered_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            call_args,
        )
    }

    fn check_lowered_global_arity(
        symbol: &str,
        expected: usize,
        provided: usize,
        instance: bool,
    ) -> Result<(), String> {
        let shape = if instance { "instance" } else { "static" };
        let provided_shape = if instance { "args(+me)" } else { "args" };
        if crate::config::env::builder_me_call_arity_strict() {
            return Err(format!(
                "[me-call] arity mismatch ({}): {}: declared {} params, got {} {}",
                shape, symbol, expected, provided, provided_shape
            ));
        }
        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0().log.warn(&format!(
                "[me-call] arity mismatch ({}): {}: declared {} params, got {} {}",
                shape, symbol, expected, provided, provided_shape
            ));
        }
        Ok(())
    }
}

fn current_bound_me_value(builder: &MirBuilder) -> Option<ValueId> {
    builder
        .function_state
        .variable_ctx
        .variable_map
        .get("me")
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
    use crate::mir::builder::calls::{AssociatedMethodCallArgumentsV1, RawLegacyMethodCallInputV1};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::parser::NyashParser;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn instruction_count(builder: &MirBuilder) -> usize {
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .map(|block| block.instructions.len())
            .sum()
    }

    #[test]
    fn prepared_me_standard_unified_is_effect_free_until_execute() {
        let arguments = vec![integer(1)];
        let input = RawLegacyMethodCallInputV1::new(
            ASTNode::Me {
                span: Span::unknown(),
            },
            "routeMethod".to_string(),
            arguments.clone(),
        );
        let mut builder = MirBuilder::new();
        let root = NyashParser::parse_from_string(
            "static box RouteCatalogSentinel { noop() { return 0 } }",
        )
        .unwrap();
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
        builder
            .comp_ctx
            .install_callable_declaration_catalog(catalog)
            .unwrap();
        builder.enter_function_for_test("RouteOwner.caller/0".to_string());
        let me = builder.build_expression(integer(9)).unwrap();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("me".to_string(), me);
        let before = instruction_count(&builder);
        let mut port = RawLegacyChildLoweringPortV1;
        let mut descent = AssociatedMethodCallArgumentsV1::new(&mut port, &input);

        let prepared =
            MeCallPolicyBox::prepare(&builder, "routeMethod", &arguments, &mut descent).unwrap();
        assert!(prepared.is_standard_unified());
        assert_eq!(
            instruction_count(&builder),
            before,
            "prepare must not emit MIR"
        );

        let result = MeCallPolicyBox::execute(
            &mut builder,
            "routeMethod",
            &arguments,
            &mut descent,
            prepared,
        )
        .unwrap()
        .expect("bound me standard route must execute");
        assert!(instruction_count(&builder) > before);
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| &block.instructions)
            .any(|instruction| matches!(
                instruction,
                crate::mir::MirInstruction::Call { dst, .. } if *dst == Some(result)
            )));
    }
}

impl MirBuilder {
    /// Handle source static calls after route selection.
    pub(in crate::mir::builder) fn handle_static_method_call_with_descent<Port>(
        &mut self,
        box_name: &str,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
    {
        if crate::config::env::joinir_dev::debug_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[handle_static_method_call] ENTRY: box_name={} method={}",
                box_name, method
            ));
        }

        // Compose lowered function name: BoxName.method/N
        let func_name = format!("{}.{}/{}", box_name, method, arguments.len());
        if arguments.is_empty() {
            if let Some(fact) = self.comp_ctx.static_scalar_method_fact(&func_name).cloned() {
                return crate::mir::builder::static_scalar_facts::emit_static_scalar_fact_const(
                    self, &fact,
                );
            }
        }
        if let Some(result) = self.try_inline_record_helper_call_with_descent(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            box_name,
            method,
            arguments,
            None,
            descent,
        )? {
            return Ok(result);
        }

        // Build argument values
        let arg_values = descent.lower_all(self)?;
        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0()
                .log
                .info(&format!("[builder] static-call {}", func_name));
        }

        let checked_source_arity = u32::try_from(arguments.len()).map_err(|_| {
            format!(
                "[static-call] source arity exceeds u32 for {}.{}: {}",
                box_name,
                method,
                arguments.len()
            )
        })?;
        descent.finish_static_global_value_terminal(
            self,
            box_name,
            method,
            checked_source_arity,
            arg_values,
        )
    }

    /// Handle TypeOp method calls: value.is("Type") and value.as("Type")
    pub(super) fn handle_typeop_method_with_terminal<Port>(
        &mut self,
        object_value: ValueId,
        method: &str,
        type_name: &str,
        completion: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
    {
        let mir_ty = Self::parse_type_name_to_mir(type_name);
        let op = if method == "is" {
            TypeOpKind::Check
        } else {
            TypeOpKind::Cast
        };
        completion.finish_typeop_value_terminal(self, object_value, op, mir_ty)
    }

    /// Handle source me.method() calls within static box context.
    pub(in crate::mir::builder) fn handle_me_method_call_with_descent<Port>(
        &mut self,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<Option<ValueId>, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        MeCallPolicyBox::resolve_me_call(self, method, arguments, descent)
    }

    /// Handle standard Box/Plugin method calls.
    pub(super) fn handle_standard_method_call(
        &mut self,
        object_value: ValueId,
        method: String,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String> {
        let mut descent = LegacyMethodCallArgumentsV1::new(arguments);
        let prepared =
            self.prepare_standard_method_execution_v1(object_value, &method, arguments)?;
        if let Some(result) = self.execute_prepared_standard_method_execution_v1(
            prepared,
            object_value,
            arguments,
            &mut descent,
        )? {
            return Ok(result);
        }
        let arg_values = descent.lower_all(self)?;
        emit_standard_value_terminal_raw_v1(self, object_value, method, arg_values)
    }

    pub(in crate::mir::builder) fn handle_standard_method_call_with_descent<Port>(
        &mut self,
        object_value: ValueId,
        method: String,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
    {
        let prepared =
            self.prepare_standard_method_execution_v1(object_value, &method, arguments)?;
        if let Some(result) = self.execute_prepared_standard_method_execution_v1(
            prepared,
            object_value,
            arguments,
            descent,
        )? {
            return Ok(result);
        }
        let arg_values = descent.lower_all(self)?;
        descent.finish_standard_value_terminal(self, object_value, method, arg_values)
    }

    fn prepare_standard_method_execution_v1(
        &self,
        object_value: ValueId,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<PreparedStandardMethodExecutionV1, String> {
        if crate::config::env::joinir_dev::debug_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[handle_standard_method_call] ENTRY: method={} object=%{}",
                method, object_value.0
            ));
        }

        // Phase 285A0.1: WeakRef.weak_to_strong() → WeakRef(Load)
        // SSOT: docs/reference/language/lifecycle.md:179 - weak_to_strong() returns Box | null
        if method == "weak_to_strong" && arguments.is_empty() {
            return Ok(PreparedStandardMethodExecutionV1::WeakLoad);
        }

        // Phase 285A0.1: upgrade() is deprecated - Fail-Fast
        if method == "upgrade" && arguments.is_empty() {
            return Ok(PreparedStandardMethodExecutionV1::UpgradeRejected);
        }

        // RECORD-VALUE-HELPER-001 follow-up:
        // Some source routes lower `me.helper(fields)` through the standard
        // receiver path instead of the dedicated me-call path. Keep the same
        // same-owner scalarization contract here, but only when the receiver is
        // the current `me` value.
        if self
            .function_state
            .variable_ctx
            .variable_map
            .get("me")
            .copied()
            .is_some_and(|me| me == object_value)
        {
            if let Some(cls) = current_enclosing_box_name(self) {
                if let Some(prepared) = self.prepare_record_helper_inline(
                    SameModuleCallableNamespaceV1::InstanceBoxMethod,
                    &cls,
                    method,
                    arguments,
                )? {
                    return Ok(PreparedStandardMethodExecutionV1::RecordHelper(prepared));
                }
            }
        }

        if let Some(prepared) = self.prepare_same_module_helper_setter_inline_from_receiver(
            object_value,
            method,
            arguments,
        )? {
            return Ok(PreparedStandardMethodExecutionV1::Setter(prepared));
        }
        Ok(PreparedStandardMethodExecutionV1::Unified)
    }

    fn execute_prepared_standard_method_execution_v1(
        &mut self,
        prepared: PreparedStandardMethodExecutionV1,
        object_value: ValueId,
        arguments: &[ASTNode],
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<Option<ValueId>, String> {
        match prepared {
            PreparedStandardMethodExecutionV1::WeakLoad => {
                self.emit_weak_load(object_value).map(Some)
            }
            PreparedStandardMethodExecutionV1::UpgradeRejected => {
                Err("WeakRef uses weak_to_strong(), not upgrade()".to_string())
            }
            PreparedStandardMethodExecutionV1::RecordHelper(prepared) => self
                .execute_prepared_record_helper_inline(
                    prepared,
                    arguments,
                    Some(object_value),
                    descent,
                )
                .map(Some),
            PreparedStandardMethodExecutionV1::Setter(prepared) => self
                .execute_prepared_same_module_helper_setter_inline(
                    prepared,
                    arguments,
                    Some(object_value),
                    descent,
                )
                .map(Some),
            PreparedStandardMethodExecutionV1::Unified => Ok(None),
        }
    }
}
