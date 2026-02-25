//! Phase 29bg P1: PatternEscapeMapFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    has_break_statement, has_continue_statement, has_return_statement,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct EscapeCaseFacts {
    pub match_lit: String,
    pub replace_lit: String,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum EscapeDefaultFacts {
    Char,
    Literal(String),
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PatternEscapeMapFacts {
    pub loop_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub haystack_var: String,
    pub out_var: String,
    pub cases: Vec<EscapeCaseFacts>,
    pub default_case: EscapeDefaultFacts,
}

pub(in crate::mir::builder) fn try_extract_pattern_escape_map_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<PatternEscapeMapFacts>, Freeze> {
    let Some(loop_var) = match_loop_condition(condition) else {
        return Ok(None);
    };

    if body.len() != 3 {
        return Ok(None);
    }
    if has_break_statement(body) || has_continue_statement(body) || has_return_statement(body) {
        return Ok(None);
    }

    let Some((ch_var, haystack_var)) = match_local_substring(&body[0], &loop_var) else {
        return Ok(None);
    };

    let mut out_var: Option<String> = None;
    let mut cases = Vec::new();
    let mut default_case: Option<EscapeDefaultFacts> = None;
    if !collect_escape_cases(
        &body[1],
        &ch_var,
        &mut out_var,
        &mut cases,
        &mut default_case,
    ) {
        return Ok(None);
    }
    let Some(out_var) = out_var else {
        return Ok(None);
    };
    let Some(default_case) = default_case else {
        return Ok(None);
    };

    let Some(loop_increment) = match_loop_increment_stmt(&body[2], &loop_var) else {
        return Ok(None);
    };

    Ok(Some(PatternEscapeMapFacts {
        loop_var,
        loop_condition: condition.clone(),
        loop_increment,
        haystack_var,
        out_var,
        cases,
        default_case,
    }))
}

fn match_loop_condition(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(right.as_ref(), ASTNode::Variable { .. }) {
        return None;
    }

    Some(loop_var.clone())
}

fn match_local_substring(stmt: &ASTNode, loop_var: &str) -> Option<(String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let ch_var = variables[0].clone();
    let Some(init) = &initial_values[0] else {
        return None;
    };

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = init.as_ref()
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }

    let ASTNode::Variable { name: haystack_var, .. } = object.as_ref() else {
        return None;
    };

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !match_add_var_lit(&arguments[1], loop_var, 1) {
        return None;
    }

    Some((ch_var, haystack_var.clone()))
}

fn collect_escape_cases(
    stmt: &ASTNode,
    ch_var: &str,
    out_var: &mut Option<String>,
    cases: &mut Vec<EscapeCaseFacts>,
    default_case: &mut Option<EscapeDefaultFacts>,
) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(else_body) = else_body else {
        return false;
    };
    let Some(then_stmt) = unwrap_single_stmt(then_body) else {
        return false;
    };
    let Some(else_stmt) = unwrap_single_stmt(else_body) else {
        return false;
    };

    let Some((match_lit, replace_lit)) = match_case_then(condition, then_stmt, ch_var, out_var)
    else {
        return false;
    };
    cases.push(EscapeCaseFacts {
        match_lit,
        replace_lit,
    });

    if let ASTNode::If { .. } = else_stmt {
        return collect_escape_cases(else_stmt, ch_var, out_var, cases, default_case);
    }

    let Some(default) = match_default_assignment(else_stmt, ch_var, out_var) else {
        return false;
    };
    *default_case = Some(default);
    true
}

fn unwrap_single_stmt(body: &[ASTNode]) -> Option<&ASTNode> {
    match body.len() {
        1 => match &body[0] {
            ASTNode::ScopeBox { body, .. } => unwrap_single_stmt(body),
            _ => Some(&body[0]),
        },
        _ => None,
    }
}

fn match_case_then(
    condition: &ASTNode,
    stmt: &ASTNode,
    ch_var: &str,
    out_var: &mut Option<String>,
) -> Option<(String, String)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let match_lit = match (left.as_ref(), right.as_ref()) {
        (ASTNode::Variable { name, .. }, ASTNode::Literal { value, .. }) if name == ch_var => {
            literal_string(value)?
        }
        (ASTNode::Literal { value, .. }, ASTNode::Variable { name, .. }) if name == ch_var => {
            literal_string(value)?
        }
        _ => return None,
    };

    let (target_var, replacement) = match_out_add_assignment(stmt, ch_var)?;
    if out_var.as_ref().map(|s| s.as_str()) != Some(target_var.as_str()) {
        if out_var.is_some() {
            return None;
        }
        *out_var = Some(target_var.clone());
    }

    let replace_lit = match replacement {
        EscapeDefaultFacts::Literal(lit) => lit,
        EscapeDefaultFacts::Char => return None,
    };

    Some((match_lit, replace_lit))
}

fn match_default_assignment(
    stmt: &ASTNode,
    ch_var: &str,
    out_var: &mut Option<String>,
) -> Option<EscapeDefaultFacts> {
    let (target_var, replacement) = match_out_add_assignment(stmt, ch_var)?;
    if out_var.as_ref().map(|s| s.as_str()) != Some(target_var.as_str()) {
        if out_var.is_some() {
            return None;
        }
        *out_var = Some(target_var.clone());
    }

    match replacement {
        EscapeDefaultFacts::Literal(lit) => Some(EscapeDefaultFacts::Literal(lit)),
        EscapeDefaultFacts::Char => {
            if target_var == ch_var {
                return None;
            }
            Some(EscapeDefaultFacts::Char)
        }
    }
}

fn match_out_add_assignment(stmt: &ASTNode, ch_var: &str) -> Option<(String, EscapeDefaultFacts)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name: out_var, .. } = target.as_ref() else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == out_var) {
        return None;
    }

    let replacement = match right.as_ref() {
        ASTNode::Variable { name, .. } => {
            if name != ch_var {
                return None;
            }
            EscapeDefaultFacts::Char
        }
        ASTNode::Literal { value, .. } => {
            EscapeDefaultFacts::Literal(literal_string(value)?)
        }
        _ => return None,
    };

    Some((out_var.clone(), replacement))
}

fn literal_string(value: &LiteralValue) -> Option<String> {
    match value {
        LiteralValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn match_add_var_lit(expr: &ASTNode, var: &str, lit: i64) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let (var_side, lit_side) = match (left.as_ref(), right.as_ref()) {
        (ASTNode::Variable { name, .. }, ASTNode::Literal { value, .. }) => (name, value),
        (ASTNode::Literal { value, .. }, ASTNode::Variable { name, .. }) => (name, value),
        _ => return false,
    };
    matches!(lit_side, LiteralValue::Integer(v) if *v == lit) && var_side == var
}

fn match_loop_increment_stmt(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }

    Some(value.as_ref().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::parser::NyashParser;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_str(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn escape_map_matches_nested_if_chain() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(v("n")),
            span: Span::unknown(),
        };
        let ch_stmt = ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(lit_int(1)),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        };

        let if_chain = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(v("ch")),
                right: Box::new(lit_str("\\")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(v("out")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("out")),
                    right: Box::new(lit_str("\\\\")),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Assignment {
                target: Box::new(v("out")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("out")),
                    right: Box::new(v("ch")),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        };

        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_extract_pattern_escape_map_facts(&condition, &[ch_stmt, if_chain, step])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.out_var, "out");
        assert_eq!(facts.haystack_var, "s");
        assert_eq!(facts.cases.len(), 1);
    }

    #[test]
    fn escape_map_matches_json_quote_minimal() {
        let src = r#"
static box StringHelpers {
  json_quote(s) {
    local out = ""
    local i = 0
    local n = s.length()
    loop (i < n) {
      local ch = s.substring(i, i+1)
      if ch == "\\" { out = out + "\\\\" }
      else { if ch == "\"" { out = out + "\\\"" } else {
        if ch == "\n" { out = out + "\\n" } else {
          if ch == "\r" { out = out + "\\r" } else {
            if ch == "\t" { out = out + "\\t" } else { out = out + ch }
          }
        }
      }}
      i = i + 1
    }
    return "\"" + out + "\""
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse ok");

        let mut loop_condition = None;
        let mut loop_body = None;
        if let ASTNode::Program { statements, .. } = ast {
            for stmt in statements {
                if let ASTNode::BoxDeclaration { name, methods, .. } = stmt {
                    if name != "StringHelpers" {
                        continue;
                    }
                    if let Some(ASTNode::FunctionDeclaration { body, .. }) =
                        methods.get("json_quote")
                    {
                        for node in body {
                            if let ASTNode::Loop { condition, body, .. } = node {
                                loop_condition = Some(condition.clone());
                                loop_body = Some(body.clone());
                                break;
                            }
                        }
                    }
                }
            }
        }

        let condition = loop_condition.expect("loop condition");
        let body = loop_body.expect("loop body");
        let facts = try_extract_pattern_escape_map_facts(&condition, &body)
            .expect("extract ok");
        assert!(
            facts.is_some(),
            "escape_map facts should match json_quote subset"
        );
    }
}
