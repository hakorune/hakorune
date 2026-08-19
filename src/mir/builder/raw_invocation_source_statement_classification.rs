//! Statement-location classification for raw invocation source transport.
//!
//! This child owns only the existing finite AST classification. Source-path
//! construction and temporal transport remain in `raw_invocation_source_transport`.

use crate::ast::ASTNode;

use super::raw_invocation_source_transport::RawUnlocatedPortalV1;

pub(super) fn reason_for_non_box_statement(statement: &ASTNode) -> RawUnlocatedPortalV1 {
    match statement {
        ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::AwaitExpression { .. }
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::ArrayLiteral { .. }
        | ASTNode::MapLiteral { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::Arrow { .. }
        | ASTNode::Throw { .. }
        | ASTNode::FunctionDeclaration { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. }
        | ASTNode::StaticConstTable { .. }
        | ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::UnaryOp { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::CheckExpr { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FieldAccess { .. }
        | ASTNode::Index { .. }
        | ASTNode::New { .. }
        | ASTNode::This { .. }
        | ASTNode::Me { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. }
        | ASTNode::Outbox { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::ExplicitExternCall { .. }
        | ASTNode::Call { .. } => RawUnlocatedPortalV1::CallObject,

        ASTNode::Lambda { .. }
        | ASTNode::Program { .. }
        | ASTNode::BoxDeclaration { .. }
        | ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::Return { .. }
        | ASTNode::Release { .. }
        | ASTNode::Local { .. }
        | ASTNode::Print { .. }
        | ASTNode::GroupedAssignmentExpr { .. }
        | ASTNode::If { .. }
        | ASTNode::Loop { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::FastMemRegion { .. }
        | ASTNode::BlockExpr { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::LoopRange { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
        | ASTNode::TryCatch { .. } => {
            unreachable!("[freeze:contract][raw-invocation/direct-box-classifier]")
        }
    }
}

pub(super) fn is_bare_function_call_statement(statement: &ASTNode) -> bool {
    matches!(statement, ASTNode::FunctionCall { .. })
}

pub(super) fn is_located_scalar_statement(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::Assignment { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::GroupedAssignmentExpr { .. }
            | ASTNode::Print { .. }
            | ASTNode::Return { .. }
            | ASTNode::Local { .. }
            | ASTNode::Nowait { .. }
    )
}

pub(super) fn is_located_zero_child_runtime_completion(statement: &ASTNode) -> bool {
    matches!(statement, ASTNode::StaticConstTable { .. })
}

pub(super) fn is_located_lambda_statement(statement: &ASTNode) -> bool {
    matches!(statement, ASTNode::Lambda { .. })
}

pub(super) fn is_located_control_or_diagnostic_terminal(statement: &ASTNode) -> bool {
    if super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(
        statement,
    ) {
        return true;
    }
    matches!(
        statement,
        ASTNode::Program { .. }
            | ASTNode::If { .. }
            | ASTNode::Loop { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::LoopRange { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
            | ASTNode::TryCatch { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    };
    use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};
    use crate::mir::resolved_semantics::ExprChildRoleV1;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn bare_calls_keep_the_call_object_compatibility_reason() {
        let function = ASTNode::FunctionCall {
            name: "BlockId".to_owned(),
            arguments: vec![integer(1)],
            span: Span::unknown(),
        };
        let method = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "BlockId".to_owned(),
                span: Span::unknown(),
            }),
            method: "unwrap".to_owned(),
            arguments: vec![integer(1)],
            span: Span::unknown(),
        };

        assert_eq!(
            reason_for_non_box_statement(&function),
            RawUnlocatedPortalV1::CallObject
        );
        assert_eq!(
            reason_for_non_box_statement(&method),
            RawUnlocatedPortalV1::CallObject
        );
        assert!(is_bare_function_call_statement(&function));
        assert!(!is_bare_function_call_statement(&method));
        assert!(!is_located_scalar_statement(&function));
        assert!(!is_located_control_or_diagnostic_terminal(&method));
    }

    #[test]
    fn scalar_lambda_and_zero_child_rows_keep_their_existing_classes() {
        let local = ASTNode::Local {
            variables: vec!["x".to_owned()],
            initial_values: vec![Some(Box::new(integer(1)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        };
        let lambda = ASTNode::Lambda {
            params: Vec::new(),
            body: vec![integer(1)],
            span: Span::unknown(),
        };
        let table = ASTNode::StaticConstTable {
            name: "T".to_owned(),
            element_type: "i64".to_owned(),
            values: Vec::new(),
            span: Span::unknown(),
        };

        assert!(is_located_scalar_statement(&local));
        assert!(is_located_lambda_statement(&lambda));
        assert!(is_located_zero_child_runtime_completion(&table));
    }

    #[test]
    fn control_rows_keep_their_existing_class() {
        let conditional = ASTNode::If {
            condition: Box::new(integer(1)),
            then_body: vec![integer(2)],
            else_body: Some(vec![integer(3)]),
            span: Span::unknown(),
        };
        assert!(is_located_control_or_diagnostic_terminal(&conditional));
    }

    #[test]
    fn installed_callable_bare_call_keeps_exact_body_site() {
        let function = ASTNode::FunctionCall {
            name: "BlockId".to_owned(),
            arguments: vec![integer(1)],
            span: Span::unknown(),
        };
        let root = RawInvocationRootLineageV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
        );
        let (_, context) = RawInvocationSourceContextV1::from_transport(
            RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root),
        );
        let (_, child) = RawInvocationSourceContextV1::from_transport(
            context.body_statement(function.clone(), 4),
        );
        assert!(matches!(
            child,
            RawInvocationSourceContextV1::Located { .. }
        ));
        assert_eq!(
            child.site().expect("call site").segments(),
            &[crate::mir::resolved_semantics::SourcePathSegmentV1::Body(4)]
        );
        let operand = child
            .child_expression(&function, ExprChildRoleV1::CallArgument(0))
            .expect("call operand site");
        assert_eq!(
            operand.site().expect("operand site").segments(),
            &[
                crate::mir::resolved_semantics::SourcePathSegmentV1::Body(4),
                crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(0),
            ]
        );
    }

    #[test]
    fn raw_and_script_roots_keep_bare_calls_unlocated() {
        let function = ASTNode::FunctionCall {
            name: "BlockId".to_owned(),
            arguments: vec![integer(1)],
            span: Span::unknown(),
        };
        let roots = [
            RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
                0,
                "Main",
                "main",
                "Main.main/0",
                0,
            )),
            RawInvocationRootLineageV1::ScriptRoot,
        ];
        for root in roots {
            let (_, context) = RawInvocationSourceContextV1::from_transport(
                RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root),
            );
            let (_, child) = RawInvocationSourceContextV1::from_transport(
                context.body_statement(function.clone(), 4),
            );
            assert!(matches!(
                child,
                RawInvocationSourceContextV1::UnlocatedCompatibility(
                    RawUnlocatedPortalV1::CallObject
                )
            ));
        }
    }
}
