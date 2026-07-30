//! Source-only disposition for direct non-Box statements in selected Script.
//!
//! This owner does not lower AST, inspect Builder state, or choose a child
//! port. It keeps the broad Script compatibility terminal finite while one
//! statement responsibility at a time moves to its existing production owner.

use crate::ast::ASTNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NormalScriptNonBoxStatementDispositionV1 {
    DirectPrint,
    DirectPortAwareExpression,
    StatementControlCompatibility,
    DeclarationIngressCompatibility,
    DirectBoxOwnedElsewhere,
    TopLevelFunctionImmediateOnly,
}

pub(super) fn classify_normal_script_nonbox_statement_v1(
    statement: &ASTNode,
) -> NormalScriptNonBoxStatementDispositionV1 {
    use NormalScriptNonBoxStatementDispositionV1::{
        DeclarationIngressCompatibility, DirectBoxOwnedElsewhere, DirectPortAwareExpression,
        DirectPrint, StatementControlCompatibility, TopLevelFunctionImmediateOnly,
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
        | ASTNode::Call { .. }
        | ASTNode::Return { .. } => DirectPortAwareExpression,

        ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::If { .. }
        | ASTNode::Loop { .. }
        | ASTNode::LoopRange { .. }
        | ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::FastMemRegion { .. }
        | ASTNode::TryCatch { .. }
        | ASTNode::Throw { .. }
        | ASTNode::Local { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::Outbox { .. } => StatementControlCompatibility,

        ASTNode::Program { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. }
        | ASTNode::StaticConstTable { .. } => DeclarationIngressCompatibility,

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
            DeclarationIngressCompatibility, DirectPortAwareExpression, DirectPrint,
            StatementControlCompatibility,
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
        let control = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_owned(),
                span: Span::unknown(),
            }),
            value: Box::new(integer(3)),
            span: Span::unknown(),
        };
        let return_statement = ASTNode::Return {
            value: Some(Box::new(integer(4))),
            span: Span::unknown(),
        };
        let ingress = ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        };
        let call = ASTNode::FunctionCall {
            name: "f".to_owned(),
            arguments: Vec::new(),
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
            StatementControlCompatibility
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&ingress),
            DeclarationIngressCompatibility
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&call),
            DirectPortAwareExpression
        );
        assert_eq!(
            classify_normal_script_nonbox_statement_v1(&return_statement),
            DirectPortAwareExpression
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
