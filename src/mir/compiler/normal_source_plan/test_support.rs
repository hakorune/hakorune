use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::PreparedNormalSourcePlanInputV1;


pub(super) fn input(source: ASTNode) -> PreparedNormalSourcePlanInputV1 {
    PreparedNormalSourcePlanInputV1::new(source, "normal-source-plan0-test")
}

pub(super) fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

pub(super) fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(super) fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

pub(super) fn function(name: &str, arity: usize, is_static: bool) -> ASTNode {
    let params = (0..arity)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.to_owned(),
                declared_type_name: None,
            })
            .collect(),
        params,
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

pub(super) fn function_with_body(name: &str, body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    let mut function = function(name, 0, is_static);
    let ASTNode::FunctionDeclaration {
        body: function_body,
        ..
    } = &mut function
    else {
        unreachable!()
    };
    *function_body = body;
    function
}

pub(super) fn value_return(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}
