//! As-written operator and literal shapes shared by typed loop producers.

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};

use super::located::LocatedExprV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SyntaxBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLiteralShapeV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    Other,
}

pub(crate) fn binary_operator_shape(operator: &BinaryOperator) -> SyntaxBinaryOperatorV1 {
    match operator {
        BinaryOperator::Add => SyntaxBinaryOperatorV1::Add,
        BinaryOperator::Subtract => SyntaxBinaryOperatorV1::Subtract,
        BinaryOperator::Multiply => SyntaxBinaryOperatorV1::Multiply,
        BinaryOperator::Divide => SyntaxBinaryOperatorV1::Divide,
        BinaryOperator::Modulo => SyntaxBinaryOperatorV1::Modulo,
        BinaryOperator::BitAnd => SyntaxBinaryOperatorV1::BitAnd,
        BinaryOperator::BitOr => SyntaxBinaryOperatorV1::BitOr,
        BinaryOperator::BitXor => SyntaxBinaryOperatorV1::BitXor,
        BinaryOperator::Shl => SyntaxBinaryOperatorV1::Shl,
        BinaryOperator::Shr => SyntaxBinaryOperatorV1::Shr,
        BinaryOperator::Equal => SyntaxBinaryOperatorV1::Equal,
        BinaryOperator::NotEqual => SyntaxBinaryOperatorV1::NotEqual,
        BinaryOperator::Less => SyntaxBinaryOperatorV1::Less,
        BinaryOperator::Greater => SyntaxBinaryOperatorV1::Greater,
        BinaryOperator::LessEqual => SyntaxBinaryOperatorV1::LessEqual,
        BinaryOperator::GreaterEqual => SyntaxBinaryOperatorV1::GreaterEqual,
        BinaryOperator::And => SyntaxBinaryOperatorV1::And,
        BinaryOperator::Or => SyntaxBinaryOperatorV1::Or,
    }
}

pub(crate) fn literal_shape(value: &LiteralValue) -> SourceLiteralShapeV1 {
    match value {
        LiteralValue::Integer(value) => SourceLiteralShapeV1::Integer(*value),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => SourceLiteralShapeV1::TypedInteger {
            value: *value,
            declared_type_name: declared_type_name.clone().into_boxed_str(),
        },
        _ => SourceLiteralShapeV1::Other,
    }
}

pub(crate) fn literal_shape_from_expr(expr: &LocatedExprV1<'_>) -> Option<SourceLiteralShapeV1> {
    match expr.node() {
        ASTNode::Literal { value, .. } => Some(literal_shape(value)),
        _ => None,
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn add(name: &str, delta: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(name)),
            right: Box::new(integer(delta)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

pub(crate) fn direct_accum_function_for_test() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "accum".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into(), "sum".into()],
                initial_values: vec![Some(Box::new(integer(0))), Some(Box::new(integer(0)))],
                declared_type_names: vec![None, None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(3)),
                    span: Span::unknown(),
                }),
                body: vec![add("sum", 1), add("i", 1)],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}
