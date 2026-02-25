//! ReadDigitsBreakConditionBox (Phase 104)
//!
//! Responsibility (analysis only):
//! - For `loop(true)` read_digits_from shape, extract:
//!   - the EOS break condition (`if ch == "" { break }`)
//!   - the digit literal set used in the final `if <is_digit> { ... } else { break }`

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::join_ir::lowering::error_tags;

pub(crate) struct ReadDigitsBreakConditionBox;

impl ReadDigitsBreakConditionBox {
    pub(crate) fn extract_eos_and_digit_condition(
        body: &[ASTNode],
    ) -> Result<(String, ASTNode, ASTNode), String> {
        if body.is_empty() {
            return Err("[phase104/read-digits] empty loop body".to_string());
        }

        let (digit_cond, has_else_break) = match &body[body.len() - 1] {
            ASTNode::If {
                condition,
                else_body: Some(else_body),
                ..
            } => (
                condition.as_ref().clone(),
                else_body.len() == 1 && matches!(else_body[0], ASTNode::Break { .. }),
            ),
            _ => return Err("[phase104/read-digits] last statement is not if-else".to_string()),
        };
        if !has_else_break {
            return Err("[phase104/read-digits] last if does not have `else { break }`".to_string());
        }

        let (ch_var, eos_cond) = find_eos_break_condition(body).ok_or_else(|| {
            error_tags::freeze_with_hint(
                "phase104/read_digits/missing_eos_guard",
                "missing `if ch == \"\" { break }` guard",
                "add 'if ch == \"\" { break }' before digit check",
            )
        })?;

        let mut digit_literals: Vec<String> = Vec::new();
        let mut digit_var_name: Option<String> = None;
        collect_eq_string_literals(&digit_cond, &mut digit_literals, &mut digit_var_name)?;

        let digit_var_name = digit_var_name.ok_or_else(|| {
            "[phase104/read-digits] digit condition does not reference a variable".to_string()
        })?;
        if digit_var_name != ch_var {
            return Err(format!(
                "[phase104/read-digits] digit condition var '{}' != eos var '{}'",
                digit_var_name, ch_var
            ));
        }

        digit_literals.sort();
        digit_literals.dedup();

        if digit_literals.is_empty() {
            return Err("[phase104/read-digits] digit condition has no string literals".to_string());
        }

        // Phase 104 minimal: require the canonical digit set.
        let expected: Vec<String> = (0..=9).map(|d| d.to_string()).collect();
        if digit_literals != expected {
            return Err(error_tags::freeze_with_hint(
                "phase104/read_digits/digit_set_mismatch",
                &format!("digit condition literal set mismatch: got={:?}, expected={:?}", digit_literals, expected),
                "use explicit 'ch == \"0\" || ... || ch == \"9\"'",
            ));
        }

        Ok((ch_var, eos_cond, digit_cond))
    }
}

fn find_eos_break_condition(body: &[ASTNode]) -> Option<(String, ASTNode)> {
    for stmt in body {
        let (cond, then_body, else_body) = match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => (condition, then_body, else_body),
            _ => continue,
        };

        if else_body.is_some() {
            continue;
        }
        if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
            continue;
        }
        if let Some(var_name) = ch_eq_empty_var(cond.as_ref()) {
            return Some((var_name, cond.as_ref().clone()));
        }
    }
    None
}

fn ch_eq_empty_var(cond: &ASTNode) -> Option<String> {
    match cond {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            let name = match left.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => return None,
            };
            match right.as_ref() {
                ASTNode::Literal {
                    value: LiteralValue::String(s),
                    ..
                } if s.is_empty() => Some(name),
                _ => None,
            }
        }
        _ => None,
    }
}

fn collect_eq_string_literals(
    cond: &ASTNode,
    out: &mut Vec<String>,
    var_name: &mut Option<String>,
) -> Result<(), String> {
    match cond {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left,
            right,
            ..
        } => {
            collect_eq_string_literals(left.as_ref(), out, var_name)?;
            collect_eq_string_literals(right.as_ref(), out, var_name)?;
            Ok(())
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            let (name, lit) = match (left.as_ref(), right.as_ref()) {
                (ASTNode::Variable { name, .. }, ASTNode::Literal { value: LiteralValue::String(s), .. }) => {
                    (name.clone(), s.clone())
                }
                (ASTNode::Literal { value: LiteralValue::String(s), .. }, ASTNode::Variable { name, .. }) => {
                    (name.clone(), s.clone())
                }
                _ => {
                    return Err("[phase104/read-digits] digit condition must be OR of `ch == \"d\"`".to_string());
                }
            };

            if lit.len() != 1 || !lit.chars().all(|c| c.is_ascii_digit()) {
                return Err(format!(
                    "[phase104/read-digits] non-digit literal in digit condition: {:?}",
                    lit
                ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: span(),
        }
    }

    fn eq(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn or(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn if_then_break(cond: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Break { span: span() }],
            else_body: None,
            span: span(),
        }
    }

    fn if_else_break(cond: ASTNode, then_body: Vec<ASTNode>) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body,
            else_body: Some(vec![ASTNode::Break { span: span() }]),
            span: span(),
        }
    }

    fn digit_chain(var_name: &str, digits: &[&str]) -> ASTNode {
        let mut it = digits.iter();
        let first = it
            .next()
            .expect("digits must be non-empty");
        let mut acc = eq(var(var_name), str_lit(first));
        for d in it {
            acc = or(acc, eq(var(var_name), str_lit(d)));
        }
        acc
    }

    #[test]
    fn test_extract_ok_eos_and_digit_condition() {
        let digit_cond = digit_chain(
            "ch",
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        );

        let body = vec![
            if_then_break(eq(var("ch"), str_lit(""))),
            if_else_break(
                digit_cond.clone(),
                vec![
                    ASTNode::Assignment {
                        target: Box::new(var("out")),
                        value: Box::new(ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(var("out")),
                            right: Box::new(var("ch")),
                            span: span(),
                        }),
                        span: span(),
                    },
                    ASTNode::Assignment {
                        target: Box::new(var("i")),
                        value: Box::new(ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(var("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: span(),
                            }),
                            span: span(),
                        }),
                        span: span(),
                    },
                ],
            ),
        ];

        let (ch_var, eos_cond, extracted_digit_cond) =
            ReadDigitsBreakConditionBox::extract_eos_and_digit_condition(&body).unwrap();

        assert_eq!(ch_var, "ch");
        assert_eq!(eos_cond, eq(var("ch"), str_lit("")));
        assert_eq!(extracted_digit_cond, digit_cond);
    }

    #[test]
    fn test_extract_rejects_missing_digit_literal() {
        let digit_cond = digit_chain(
            "ch",
            &["0", "1", "2", "3", "4", "5", "6", "7", "8"], // missing "9"
        );
        let body = vec![
            if_then_break(eq(var("ch"), str_lit(""))),
            if_else_break(digit_cond, vec![ASTNode::Break { span: span() }]),
        ];

        let err = ReadDigitsBreakConditionBox::extract_eos_and_digit_condition(&body)
            .unwrap_err();
        assert!(err.contains("literal set mismatch"));
    }

    #[test]
    fn test_extract_rejects_mixed_var_names() {
        let digit_cond = or(eq(var("ch"), str_lit("0")), eq(var("x"), str_lit("1")));
        let body = vec![
            if_then_break(eq(var("ch"), str_lit(""))),
            if_else_break(digit_cond, vec![ASTNode::Break { span: span() }]),
        ];

        let err = ReadDigitsBreakConditionBox::extract_eos_and_digit_condition(&body)
            .unwrap_err();
        assert!(err.contains("mixed variable names"));
    }
}

            match var_name.as_deref() {
                None => *var_name = Some(name),
                Some(prev) if prev == name => {}
                Some(prev) => {
                    return Err(format!(
                        "[phase104/read-digits] mixed variable names in digit condition: '{}' vs '{}'",
                        prev, name
                    ));
                }
            }

            out.push(lit);
            Ok(())
        }
        _ => Err("[phase104/read-digits] digit condition must be OR-chain of equality checks".to_string()),
    }
}
