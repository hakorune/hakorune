//! Source-only disposition for direct non-Box statements in selected Script.
//!
//! This owner does not lower AST, inspect Builder state, or choose a child
//! port. It keeps the broad Script compatibility terminal finite while one
//! statement responsibility at a time moves to its existing production owner.

use crate::ast::ASTNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NormalScriptNonBoxStatementDispositionV1 {
    DirectPrint,
    DirectIfStatement,
    DirectFastMemRegion,
    DirectPortAwareExpression,
    DirectStaticConstRuntimeCompletion,
    DirectSelectedUnsupportedStatement,
    DirectBoxOwnedElsewhere,
    TopLevelFunctionImmediateOnly,
}

pub(super) fn classify_normal_script_nonbox_statement_v1(
    statement: &ASTNode,
) -> NormalScriptNonBoxStatementDispositionV1 {
    use NormalScriptNonBoxStatementDispositionV1::{
        DirectBoxOwnedElsewhere, DirectFastMemRegion, DirectIfStatement, DirectPortAwareExpression,
        DirectPrint, DirectSelectedUnsupportedStatement, DirectStaticConstRuntimeCompletion,
        TopLevelFunctionImmediateOnly,
    };

    match statement {
        ASTNode::Print { .. } => DirectPrint,

        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::Me { .. }
        | ASTNode::UnaryOp { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::AwaitExpression { .. }
        | ASTNode::CheckExpr { .. }
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
        | ASTNode::ArrayLiteral { .. }
        | ASTNode::MapLiteral { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::Lambda { .. }
        | ASTNode::BlockExpr { .. }
        | ASTNode::Arrow { .. }
        | ASTNode::GroupedAssignmentExpr { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FieldAccess { .. }
        | ASTNode::Index { .. }
        | ASTNode::New { .. }
        | ASTNode::This { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Call { .. } => DirectPortAwareExpression,

        ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::Loop { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::TryCatch { .. }
        | ASTNode::Throw { .. }
        | ASTNode::Local { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::Outbox { .. }
        | ASTNode::Program { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::Return { .. } => DirectPortAwareExpression,

        ASTNode::If { .. } => DirectIfStatement,

        ASTNode::FastMemRegion { .. } => DirectFastMemRegion,

        ASTNode::LoopRange { .. }
        | ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. } => DirectSelectedUnsupportedStatement,

        ASTNode::StaticConstTable { .. } => DirectStaticConstRuntimeCompletion,

        ASTNode::BoxDeclaration { .. } => DirectBoxOwnedElsewhere,
        ASTNode::FunctionDeclaration { .. } => TopLevelFunctionImmediateOnly,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        classify_normal_script_nonbox_statement_v1,
        NormalScriptNonBoxStatementDispositionV1::{
            DirectFastMemRegion, DirectIfStatement, DirectPortAwareExpression, DirectPrint,
            DirectSelectedUnsupportedStatement, DirectStaticConstRuntimeCompletion,
        },
    };
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn partitions_print_from_each_residual_family() {
        let print = ASTNode::Print {
            expression: Box::new(integer(1)),
            span: Span::unknown(),
        };
        let expression = integer(2);
        let control = ASTNode::If {
            condition: Box::new(integer(1)),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        };
        let return_statement = ASTNode::Return {
            value: Some(Box::new(integer(4))),
            span: Span::unknown(),
        };
        let static_table = ASTNode::StaticConstTable {
            name: "T".to_owned(),
            element_type: "u16".to_owned(),
            values: vec![1, 2],
            span: Span::unknown(),
        };
        let ingress = ASTNode::GlobalVar {
            name: "g".to_owned(),
            value: Box::new(integer(5)),
            span: Span::unknown(),
        };
        let call = ASTNode::FunctionCall {
            name: "f".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        };
        let fastmem = ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(),
            body: vec![integer(6)],
            span: Span::unknown(),
        };

        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&print),
            DirectPrint
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&expression),
            DirectPortAwareExpression
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&control),
            DirectIfStatement
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&ingress),
            DirectSelectedUnsupportedStatement
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&call),
            DirectPortAwareExpression
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&return_statement),
            DirectPortAwareExpression
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&static_table),
            DirectStaticConstRuntimeCompletion
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&fastmem),
            DirectFastMemRegion
        );
    }

    #[test]
    fn selected_direct_print_keeps_function_typeop_parity() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let print = ASTNode::Print {
            expression: Box::new(ASTNode::FunctionCall {
                name: "isType".to_owned(),
                arguments: vec![
                    integer(42),
                    ASTNode::Literal {
                        value: LiteralValue::String("Integer".to_owned()),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let program = ASTNode::Program {
            statements: vec![print],
            span: Span::unknown(),
        };
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(program.clone(), Some("script-print-typeop.hako"))
            .expect("legacy Print TypeOp");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            program,
            Some("script-print-typeop.hako"),
            HashMap::new(),
        )
        .expect("normal Print request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Print TypeOp");

        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }
}
