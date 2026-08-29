//! 🎯 箱理論: Call構築 Orchestrator (Refactored 755→311 lines)
//!
//! # 責務
//! ASTからCall構築の統合制御（orchestration only, no implementation）
//! - direct FunctionCall post-argument completion
//! - build_method_call: メソッド呼び出し構築
//! - `PreparedRawFromCallV1`: from式のenum/ordinary routeをeffect前に一度だけ選択
//!
//! # Delegation Strategy (実装は専用モジュールへ委譲)
//! - `debug_method_routing`: Debug tracing（179 lines）
//! - `function_call_preflight_route`: one pre-effect direct-call route
//! - `special_method_handlers`: Special method detection（122 lines）
//! - `static_resolution`: Static receiver resolution（182 lines）
//! - `receiver_binding`: Receiver normalization（54 lines）
//!
//! # Refactoring History
//! - Before: 755 lines monolithic implementation
//! - After: 311 lines orchestrator + 4 extracted modules (537 lines total)
//! - Net reduction: -444 lines of complexity in build.rs

use super::super::me_call_header_observation::MethodCallLoweringPortV1;
use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::super::static_result_publication_ingress::StaticResultPublicationIngressPortV1;
use super::super::{Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
#[allow(unused_imports)]
use super::debug_method_routing::*;
use crate::ast::ASTNode;
use crate::mir::builder::calls::function_call_preflight_route::PreparedRawOrdinaryFunctionCompletionV1;
use crate::mir::builder::calls::{
    drive_call_arguments_v1, drive_call_arguments_with_expected_sites_v1,
};
use crate::mir::builder::exprs_enum_match::{
    prepare_raw_enum_variant_header_v1, PreparedRawEnumVariantHeaderV1,
};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, AppMainDirectCallDispositionPortV1, RawAstChildLoweringPortV1,
    RawFunctionHeaderLookupPortV1,
};
use crate::mir::policies::source_method_typeop_route::{
    classify_source_method_typeop_route_v1, SourceMethodTypeOpDispositionV1,
};
use crate::mir::Callee;

pub(in crate::mir::builder) struct PreparedRawFromCallV1 {
    route: PreparedRawFromCallRouteV1,
}

enum PreparedRawFromCallRouteV1 {
    EnumVariant {
        parent: String,
        method: String,
        arguments: Vec<ASTNode>,
        header: PreparedRawEnumVariantHeaderV1,
    },
    Ordinary {
        parent: String,
        method: String,
        arguments: Vec<ASTNode>,
    },
}

impl PreparedRawFromCallV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        parent: String,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<Self, String> {
        let route = match prepare_raw_enum_variant_header_v1(builder, &parent, &method, &arguments)?
        {
            Some(header) => PreparedRawFromCallRouteV1::EnumVariant {
                parent,
                method,
                arguments,
                header,
            },
            None => PreparedRawFromCallRouteV1::Ordinary {
                parent,
                method,
                arguments,
            },
        };
        Ok(Self { route })
    }
}

fn lower_prepared_targeted_call_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    callee: Callee,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
{
    let arg_values = drive_call_arguments_v1(builder, port, arguments.as_slice())?;
    port.with_function_headers(|_lookup| {
        builder.emit_prepared_cataloged_call_v1(callee, arg_values)
    })
}

impl MirBuilder {
    /// Complete the ordinary direct-call route after pre-effect selection.
    pub(super) fn lower_prepared_raw_ordinary_function_completion_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        completion: PreparedRawOrdinaryFunctionCompletionV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1
            + RawFunctionHeaderLookupPortV1
            + AppMainDirectCallDispositionPortV1,
    {
        match completion {
            PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { argument } => {
                let value = drive_legacy_expression_v1(self, port, argument)?;
                self.build_str_normalization(value)
            }
            PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted { callee, arguments } => {
                lower_prepared_targeted_call_v1(self, port, callee, arguments)
            }
            PreparedRawOrdinaryFunctionCompletionV1::AppMainTargeted { arguments } => {
                self.lower_prepared_app_main_direct_call_v1(port, arguments)
            }
            PreparedRawOrdinaryFunctionCompletionV1::Rejected { error } => Err(error),
        }
    }

    fn lower_prepared_app_main_direct_call_v1<Port>(
        &mut self,
        port: &mut Port,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1
            + RawFunctionHeaderLookupPortV1
            + AppMainDirectCallDispositionPortV1,
    {
        // Take the owned row first.  Its borrow ends before recursive
        // argument descent, so nested calls can use the same affine loan.
        let row = port.take_app_main_direct_call_disposition_v1()?;
        let expected_sites = row.argument_sites().to_vec();
        let emission = row.into_emission();
        let arg_values = drive_call_arguments_with_expected_sites_v1(
            self,
            port,
            arguments.as_slice(),
            expected_sites.as_slice(),
        )?;
        let dst = self.next_value_id();
        let instruction = emission.materialize(dst, arg_values).map_err(|error| {
            format!("[freeze:contract][app-main-direct-call/materialization] {error:?}")
        })?;
        self.emit_instruction(instruction)?;
        Ok(dst)
    }

    // Build method call: object.method(arguments)
    pub fn build_method_call(
        &mut self,
        object: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let input =
            super::method_call_descent::RawLegacyMethodCallInputV1::new(object, method, arguments);
        let mut port = super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
        self.build_method_call_from_input_v1(&mut port, &input)
    }

    pub(in crate::mir::builder) fn build_method_call_from_input_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
    {
        self.build_method_call_from_input_with_route_v1(port, input, |builder, port, input| {
            builder.build_member_method_call_v1(port, input)
        })
    }

    pub(in crate::mir::builder) fn build_method_call_from_input_with_claim_ingress_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1
            + RecursiveChildLoweringPortV1
            + StaticResultPublicationIngressPortV1,
    {
        self.build_method_call_from_input_with_route_v1(port, input, |builder, port, input| {
            builder.build_member_method_call_with_claim_ingress_v1(port, input)
        })
    }

    fn build_method_call_from_input_with_route_v1<Port, Route>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
        route: Route,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
        Route:
            FnOnce(&mut MirBuilder, &mut Port, &Port::MethodCallInput) -> Result<ValueId, String>,
    {
        let typeop = {
            let syntax = port.method_call_syntax(input)?;
            match classify_source_method_typeop_route_v1(syntax.method(), syntax.arguments()) {
                SourceMethodTypeOpDispositionV1::TypeOp { kind, type_name } => {
                    Some((kind.spelling().to_owned(), type_name.to_string()))
                }
                SourceMethodTypeOpDispositionV1::Ordinary => None,
            }
        };
        if let Some((method, type_name)) = typeop {
            let object_value =
                super::method_call_descent::lower_method_call_receiver_v1(self, port, input)?;
            let mut completion =
                super::method_call_descent::AssociatedMethodCallArgumentsV1::new(port, input);
            return self.handle_typeop_method_with_terminal(
                object_value,
                &method,
                &type_name,
                &mut completion,
            );
        }

        // Capture syntax before incrementing so syntax errors cannot alter entry depth.
        let method = port.method_call_syntax(input)?.method().to_string();

        // Debug: Check recursion depth
        const MAX_METHOD_DEPTH: usize = 100;
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_METHOD_DEPTH {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.error(&format!(
                "[FATAL] build_method_call recursion depth exceeded {}",
                MAX_METHOD_DEPTH
            ));
            ring0
                .log
                .error(&format!("[FATAL] Current depth: {}", self.recursion_depth));
            ring0.log.error(&format!("[FATAL] Method: {}", method));
            let error = format!(
                "build_method_call recursion depth exceeded: {}",
                self.recursion_depth
            );
            self.recursion_depth -= 1;
            return Err(error);
        }

        let result = self.build_method_call_impl_with_route_v1(port, input, route);
        self.recursion_depth -= 1;
        result
    }

    fn build_method_call_impl_with_route_v1<Port, Route>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
        route: Route,
    ) -> Result<ValueId, String>
    where
        Port: MethodCallLoweringPortV1,
        Route:
            FnOnce(&mut MirBuilder, &mut Port, &Port::MethodCallInput) -> Result<ValueId, String>,
    {
        {
            let syntax = port.method_call_syntax(input)?;
            self.trace_method_call_if_enabled(syntax.receiver(), syntax.method());
        }

        match super::reserved_method_route::build_reserved_method_call_v1(self, port, input)? {
            super::reserved_method_route::ReservedMethodCallOutcomeV1::Ordinary => {}
            super::reserved_method_route::ReservedMethodCallOutcomeV1::Emitted(value) => {
                return Ok(value)
            }
        }

        route(self, port, input)
    }

    /// Lower one prepared `from` route without dropping the caller's child port.
    pub(in crate::mir::builder) fn lower_prepared_raw_from_call_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawFromCallV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let (parent, method, arguments) = match prepared.route {
            PreparedRawFromCallRouteV1::EnumVariant {
                parent,
                method,
                arguments,
                header,
            } => {
                return self.lower_prepared_raw_enum_variant_with_port_v1(
                    port, parent, method, arguments, header,
                )
            }
            PreparedRawFromCallRouteV1::Ordinary {
                parent,
                method,
                arguments,
            } => (parent, method, arguments),
        };

        let arg_values = drive_call_arguments_v1(self, port, &arguments)?;
        let parent_value = crate::mir::builder::emission::constant::emit_string(self, parent)?;
        let result_id = self.next_value_id();
        self.emit_box_or_plugin_call(
            Some(result_id),
            parent_value,
            method,
            None,
            arg_values,
            EffectMask::READ.add(Effect::ReadHeap),
        )?;
        Ok(result_id)
    }

    // ========================================
    // Private helper methods (small functions)
    // ========================================

    /// Build call arguments from AST
    pub(in crate::mir::builder) fn build_call_args(
        &mut self,
        args: &[ASTNode],
    ) -> Result<Vec<ValueId>, String> {
        super::call_argument_descent::drive_raw_call_arguments_v1(self, args)
    }

    fn emit_prepared_cataloged_call_v1(
        &mut self,
        callee: Callee,
        args: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        self.emit_instruction(MirInstruction::call(
            Some(dst),
            callee,
            args,
            EffectMask::READ.add(Effect::ReadHeap),
        ))?;
        Ok(dst)
    }
}

#[cfg(test)]
mod raw_from_route_tests {
    use super::*;
    use crate::ast::{EnumVariantDecl, FieldDecl, LiteralValue, Span};
    use crate::mir::builder::calls::method_call_descent::RawLegacyMethodCallInputV1;
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn null() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Null,
            span: Span::unknown(),
        }
    }

    fn variant(name: &str, payload_type_name: Option<&str>) -> EnumVariantDecl {
        EnumVariantDecl {
            name: name.to_string(),
            payload_type_name: payload_type_name.map(str::to_string),
            record_field_decls: vec![],
            tuple_payload_type_names: vec![],
        }
    }

    fn prepare_error(result: Result<PreparedRawFromCallV1, String>) -> String {
        match result {
            Ok(_) => panic!("expected raw From preparation to fail"),
            Err(error) => error,
        }
    }

    #[test]
    fn raw_from_route_selects_enum_or_ordinary_once_before_lowering() {
        let mut builder = MirBuilder::new();
        builder.comp_ctx.register_enum_decl(
            "Result".to_string(),
            vec!["T".to_string(), "E".to_string()],
            vec![variant("Ok", Some("T")), variant("Err", Some("E"))],
        );

        let enum_route = PreparedRawFromCallV1::prepare(
            &builder,
            "Result".to_string(),
            "Ok".to_string(),
            vec![int(1)],
        )
        .expect("known enum variant must prepare");
        assert!(matches!(
            enum_route.route,
            PreparedRawFromCallRouteV1::EnumVariant { .. }
        ));

        let ordinary_route = PreparedRawFromCallV1::prepare(
            &builder,
            "Parent".to_string(),
            "build".to_string(),
            vec![int(2)],
        )
        .expect("unknown enum owner remains ordinary From");
        assert!(matches!(
            ordinary_route.route,
            PreparedRawFromCallRouteV1::Ordinary { .. }
        ));
    }

    #[test]
    fn raw_enum_route_preserves_payload_arity_and_nullish_error_precedence() {
        let mut builder = MirBuilder::new();
        builder.comp_ctx.register_enum_decl(
            "Option".to_string(),
            vec!["T".to_string()],
            vec![variant("None", None), variant("Some", Some("T"))],
        );
        builder.comp_ctx.register_enum_decl(
            "RecordResult".to_string(),
            vec![],
            vec![EnumVariantDecl {
                name: "Ok".to_string(),
                payload_type_name: None,
                record_field_decls: vec![FieldDecl {
                    name: "value".to_string(),
                    declared_type_name: Some("i64".to_string()),
                    is_weak: false,
                    default_value: None,
                }],
                tuple_payload_type_names: vec![],
            }],
        );

        let record = prepare_error(PreparedRawFromCallV1::prepare(
            &builder,
            "RecordResult".to_string(),
            "Ok".to_string(),
            vec![],
        ));
        assert!(record.contains("record/tuple payload lowering"));

        let arity = prepare_error(PreparedRawFromCallV1::prepare(
            &builder,
            "Option".to_string(),
            "Some".to_string(),
            vec![],
        ));
        assert!(arity.contains("expects 1 arg(s), got 0"));

        let nullish = prepare_error(PreparedRawFromCallV1::prepare(
            &builder,
            "Option".to_string(),
            "Some".to_string(),
            vec![null()],
        ));
        assert!(nullish.contains("nullish"));
    }

    #[test]
    fn method_depth_overflow_restores_entry_depth_without_publication() {
        let _ = std::panic::catch_unwind(|| {
            crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
        });
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("method_depth_overflow/0".to_string());
        builder.recursion_depth = 100;
        let input = RawLegacyMethodCallInputV1::new(
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
            "routeMethod".to_string(),
            vec![],
        );
        let mut port = RawLegacyChildLoweringPortV1;

        let error = builder
            .build_method_call_from_input_v1(&mut port, &input)
            .expect_err("method depth overflow must reject");

        assert!(error.contains("101"));
        assert_eq!(builder.recursion_depth, 100);
        assert!(builder.current_function_instructions().is_empty());
    }

    #[test]
    fn method_call_error_restores_nonzero_entry_depth() {
        let _ = std::panic::catch_unwind(|| {
            crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
        });
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("method_depth_error/0".to_string());
        builder.recursion_depth = 7;
        let input = RawLegacyMethodCallInputV1::new(
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
            "routeMethod".to_string(),
            vec![],
        );
        let mut port = RawLegacyChildLoweringPortV1;

        let _ = builder.build_method_call_from_input_v1(&mut port, &input);

        assert_eq!(builder.recursion_depth, 7);
    }
}
