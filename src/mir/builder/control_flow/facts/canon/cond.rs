//! Analysis-only canonical views for boolean conditions (no rewrite).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CondCanon {
    BoolOr(Vec<CondAtom>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CondAtom {
    Cmp {
        var: String,
        op: BinaryOperator,
        lit: LiteralValue,
    },
    ModEqZero {
        var: String,
        k: i64,
    },
}

pub(crate) fn canon_bool_or_condition(ast: &ASTNode) -> Option<CondCanon> {
    let mut atoms = Vec::new();
    if !collect_or_atoms(ast, &mut atoms) {
        return None;
    }
    if atoms.is_empty() {
        return None;
    }
    Some(CondCanon::BoolOr(atoms))
}

fn collect_or_atoms(ast: &ASTNode, out: &mut Vec<CondAtom>) -> bool {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left,
            right,
            ..
        } => collect_or_atoms(left, out) && collect_or_atoms(right, out),
        _ => match canon_atom(ast) {
            Some(atom) => {
                out.push(atom);
                true
            }
            None => false,
        },
    }
}

fn canon_atom(ast: &ASTNode) -> Option<CondAtom> {
    canon_mod_eq_zero(ast).or_else(|| canon_cmp_atom(ast))
}

fn canon_cmp_atom(ast: &ASTNode) -> Option<CondAtom> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = ast
    else {
        return None;
    };

    if !matches!(
        operator,
        BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
    ) {
        return None;
    }

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    let ASTNode::Literal { value, .. } = right.as_ref() else {
        return None;
    };

    Some(CondAtom::Cmp {
        var: name.clone(),
        op: operator.clone(),
        lit: value.clone(),
    })
}

fn canon_mod_eq_zero(ast: &ASTNode) -> Option<CondAtom> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = ast
    else {
        return None;
    };

    mod_eq_zero_side(left, right).or_else(|| mod_eq_zero_side(right, left))
}

fn mod_eq_zero_side(lhs: &ASTNode, rhs: &ASTNode) -> Option<CondAtom> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Modulo,
        left,
        right,
        ..
    } = lhs
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::Integer(k),
        ..
    } = right.as_ref()
    else {
        return None;
    };
    if *k <= 0 {
        return None;
    }

    match rhs {
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        } => Some(CondAtom::ModEqZero {
            var: name.clone(),
            k: *k,
        }),
        _ => None,
    }
}
