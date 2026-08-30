//! Focused evidence for the static current-owner `me.method(...)` handoff.
//!
//! These tests deliberately exercise the small physical bridge rather than
//! the legacy header path.  A declared instance method remains a separate
//! sibling and is tested as such below.

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::calls::call_argument_descent::CallArgumentDescentPortV1;
use crate::mir::builder::calls::lower_target_only_static_result_publication_v1;
use crate::mir::builder::calls::method_call_descent::{
    AssociatedMethodCallArgumentsV1, MethodCallDescentPortV1, MethodCallSyntaxViewV1,
};
use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, MirBuilder, SameModuleCallableNamespaceV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    CurrentOwnerStaticCallTargetErrorV1, VerifiedSourceMethodCallSiteV1,
    VerifiedSourceStaticCallTargetCatalogV1,
};
use crate::mir::{Callee, MirInstruction, ValueId};
use crate::parser::NyashParser;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]))
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_owned());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current test function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

fn static_target(owner: &str, method: &str, arity: usize) -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_static_box_method(owner, method, arity)
}

#[derive(Default)]
struct ArgumentPort {
    lowered: Vec<usize>,
    fail_at: Option<usize>,
}

struct MethodInput {
    receiver: ASTNode,
    method: String,
    arguments: Vec<ASTNode>,
}

impl RecursiveChildLoweringPortV1 for ArgumentPort {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("method-call fixture does not lower bodies")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("method-call fixture does not lower statements")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        // The argument index is recorded by argument_expression_input.  The
        // actual expression still uses the production recursive lowering.
        crate::mir::builder::recursive_child_lowering::drive_raw_legacy_expression_v1(
            builder, input,
        )
    }
}

impl CallArgumentDescentPortV1 for ArgumentPort {
    type ArgumentsInput = [ASTNode];

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        input.len()
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        input.get(index)
    }

    fn argument_expression_input(
        &mut self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        self.lowered.push(index);
        if self.fail_at == Some(index) {
            return Err(format!("argument-failure-{index}"));
        }
        input
            .get(index)
            .cloned()
            .ok_or_else(|| format!("missing-argument-{index}"))
    }
}

impl MethodCallDescentPortV1 for ArgumentPort {
    type MethodCallInput = MethodInput;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String> {
        Ok(MethodCallSyntaxViewV1::new(
            &input.receiver,
            &input.method,
            &input.arguments,
        ))
    }

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.receiver.clone())
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(&input.arguments)
    }
}

fn lower_static_target(
    builder: &mut MirBuilder,
    port: &mut ArgumentPort,
    method: &str,
    arguments: Vec<ASTNode>,
    target_arity: usize,
) -> Result<ValueId, String> {
    let input = MethodInput {
        receiver: ASTNode::Me {
            span: Span::unknown(),
        },
        method: method.to_owned(),
        arguments,
    };
    let target = static_target("Helpers", method, target_arity);
    let mut descent = AssociatedMethodCallArgumentsV1::new(port, &input);
    lower_target_only_static_result_publication_v1(
        builder,
        &mut descent,
        target,
        input.arguments.len(),
    )
}

#[test]
fn static_current_owner_target_is_taken_before_header_and_arguments() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder("Helpers.caller/0");
        let mut port = ArgumentPort::default();
        let error = lower_static_target(&mut builder, &mut port, "target", vec![integer(1)], 2)
            .expect_err("source arity must be checked before descent");

        assert!(error.contains("static-target-only/source-arity"));
        assert!(port.lowered.is_empty(), "no argument may be lowered first");
        assert!(instructions(&builder)
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Call { .. })));
    });
}

#[test]
fn static_current_owner_lowers_source_arguments_once_without_receiver_prefix() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder("Helpers.caller/0");
        let mut port = ArgumentPort::default();
        let result = lower_static_target(
            &mut builder,
            &mut port,
            "target",
            vec![integer(11), integer(22)],
            2,
        )
        .expect("exact static current-owner target should lower");

        assert_eq!(port.lowered, [0, 1]);
        let call = instructions(&builder)
            .into_iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Global(target)),
                    args,
                    ..
                } if target.display_name() == "Helpers.target/2" => Some((dst, args)),
                _ => None,
            })
            .expect("static current-owner bridge must emit a typed global call");
        assert_eq!(call.0, result);
        assert_eq!(call.1.len(), 2, "static me.method has no receiver prefix");
    });
}

#[test]
fn static_current_owner_missing_target_rejects_before_arguments() {
    let source = "static box Helpers { caller(x) { return me.absent(x) } }";
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
        &NyashParser::parse_from_string(source).unwrap(),
    )
    .unwrap();
    let caller = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Helpers",
            "caller",
            1,
        )
        .unwrap()
        .key()
        .clone();
    let call = VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, site()).unwrap();
    let imports =
        crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(&declarations, [])
            .unwrap();
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [])
        .unwrap()
        .extend_current_owner([&call]);

    assert_eq!(
        targets.unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
            owner: "Helpers".into(),
            method: "absent".into(),
            arity: 1,
        }
    );
}

#[test]
fn outside_static_current_owner_preserves_declared_instance_sibling() {
    let instance =
        CanonicalSameModuleCallableKeyV1::test_instance_box_method("Helpers", "target", 1);
    assert_eq!(
        instance.namespace(),
        SameModuleCallableNamespaceV1::InstanceBoxMethod
    );
    assert!(instance.canonical_global_target_v1().is_err());
}

#[test]
fn static_current_owner_argument_failure_does_not_emit_retry_or_fallback() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let mut builder = builder("Helpers.caller/0");
        let mut port = ArgumentPort {
            fail_at: Some(1),
            ..ArgumentPort::default()
        };
        let error = lower_static_target(
            &mut builder,
            &mut port,
            "target",
            vec![integer(11), integer(22)],
            2,
        )
        .expect_err("the second argument failure must abort the bridge");

        assert_eq!(error, "argument-failure-1");
        assert_eq!(port.lowered, [0, 1]);
        assert!(instructions(&builder)
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Call { .. })));
    });
}
