//! Source-owned result classes for expression-value admission.
//!
//! This product is deliberately narrower than the existing callable-result
//! catalog.  It records the source class of one callable (`i64` or `String`)
//! and the exact expression sites used to prove that class.  It has no MIR
//! type, ValueId, receiver lookup, or fallback edge.  Expression-If
//! admission consumes this product in a later slice.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SourceResultClassV1 {
    I64,
    String,
}

impl SourceResultClassV1 {
    fn from_declared_type(name: &str) -> Option<Self> {
        match name {
            "i64" | "Integer" | "Int" => Some(Self::I64),
            "Str" | "String" => Some(Self::String),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceValueOperationV1 {
    Literal {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
    Binding {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
    StringConcat {
        site: SourceExprSiteV1,
    },
    ConditionalValue {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceBoolFactV1 {
    owner: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    null_operand: SourceExprSiteV1,
}

impl SourceBoolFactV1 {
    pub(crate) fn owner(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.owner
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) fn null_operand(&self) -> &SourceExprSiteV1 {
        &self.null_operand
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceResultProductErrorV1 {
    ForeignCallable,
    UnsupportedDeclaredResult,
    MissingReturn,
    UnknownExpression(SourceExprSiteV1),
    MixedReturnClasses,
    DeclaredResultMismatch {
        declared: SourceResultClassV1,
        inferred: SourceResultClassV1,
    },
    UnsupportedStatement,
    InvalidConditionalValue(SourceExprSiteV1),
    NullableOperandNotString(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedSourceResultProductV1 {
    catalog_identity: usize,
    owner: CanonicalSameModuleCallableKeyV1,
    result: SourceResultClassV1,
    operations: Box<[SourceValueOperationV1]>,
    bool_facts: Box<[SourceBoolFactV1]>,
    _seal: SourceResultProductSealV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceResultProductSealV1;

impl VerifiedSourceResultProductV1 {
    pub(crate) const fn catalog_identity(&self) -> usize {
        self.catalog_identity
    }

    pub(crate) fn owner(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.owner
    }

    pub(crate) const fn result(&self) -> SourceResultClassV1 {
        self.result
    }

    pub(crate) fn operations(&self) -> &[SourceValueOperationV1] {
        &self.operations
    }

    pub(crate) fn bool_facts(&self) -> &[SourceBoolFactV1] {
        &self.bool_facts
    }
}

/// Issue one source-result product from the already sealed declaration row.
///
/// This first issuer intentionally accepts direct values and conditional
/// values only.  Static/core call rows remain owned by their source target
/// issuers and are added by the next bounded slice; an unknown call therefore
/// fails before any Builder effect is possible.
pub(crate) fn issue_source_result_product_v1(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &CanonicalSameModuleCallableKeyV1,
) -> Result<VerifiedSourceResultProductV1, SourceResultProductErrorV1> {
    let declaration = declarations
        .declaration(owner)
        .ok_or(SourceResultProductErrorV1::ForeignCallable)?;
    let mut state = InferenceState {
        owner: owner.clone(),
        locals: BTreeMap::new(),
        operations: Vec::new(),
        bool_facts: Vec::new(),
        returns: Vec::new(),
    };
    infer_statements(&mut state, declaration.body(), &[])?;
    let inferred = state
        .returns
        .iter()
        .copied()
        .try_fold(None, |class, next| match (class, next) {
            (None, class) => Ok(Some(class)),
            (Some(previous), class) if previous == class => Ok(Some(previous)),
            _ => Err(SourceResultProductErrorV1::MixedReturnClasses),
        })?
        .ok_or(SourceResultProductErrorV1::MissingReturn)?;
    if let Some(declared_name) = declaration.return_type_name() {
        let declared = SourceResultClassV1::from_declared_type(declared_name)
            .ok_or(SourceResultProductErrorV1::UnsupportedDeclaredResult)?;
        if declared != inferred {
            return Err(SourceResultProductErrorV1::DeclaredResultMismatch { declared, inferred });
        }
    }
    Ok(VerifiedSourceResultProductV1 {
        catalog_identity: declarations as *const _ as usize,
        owner: owner.clone(),
        result: inferred,
        operations: state.operations.into_boxed_slice(),
        bool_facts: state.bool_facts.into_boxed_slice(),
        _seal: SourceResultProductSealV1,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprClassV1 {
    I64,
    String,
    Bool,
    Null,
}

impl ExprClassV1 {
    fn result(self) -> Option<SourceResultClassV1> {
        match self {
            Self::I64 => Some(SourceResultClassV1::I64),
            Self::String => Some(SourceResultClassV1::String),
            Self::Bool | Self::Null => None,
        }
    }
}

struct InferenceState {
    owner: CanonicalSameModuleCallableKeyV1,
    locals: BTreeMap<String, ExprClassV1>,
    operations: Vec<SourceValueOperationV1>,
    bool_facts: Vec<SourceBoolFactV1>,
    returns: Vec<SourceResultClassV1>,
}

fn infer_statements(
    state: &mut InferenceState,
    statements: &[ASTNode],
    prefix: &[SourcePathSegmentV1],
) -> Result<(), SourceResultProductErrorV1> {
    for (index, statement) in statements.iter().enumerate() {
        let mut path = prefix.to_vec();
        path.push(SourcePathSegmentV1::Body(index as u32));
        match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } if variables.len() == initial_values.len() => {
                for (slot, (name, initial)) in variables.iter().zip(initial_values).enumerate() {
                    let Some(initial) = initial else {
                        continue;
                    };
                    let mut site_path = path.clone();
                    site_path.push(SourcePathSegmentV1::Initializer(slot as u32));
                    let class = infer_expr(state, initial, &site_path)?;
                    state.locals.insert(name.clone(), class);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return Err(SourceResultProductErrorV1::UnsupportedStatement);
                };
                let mut site_path = path.clone();
                site_path.push(SourcePathSegmentV1::Value);
                let class = infer_expr(state, value, &site_path)?;
                state.locals.insert(name.clone(), class);
            }
            ASTNode::Return {
                value: Some(value), ..
            } => {
                let mut site_path = path.clone();
                site_path.push(SourcePathSegmentV1::Value);
                let class = infer_expr(state, value, &site_path)?;
                state.returns.push(class.result().ok_or_else(|| {
                    SourceResultProductErrorV1::UnknownExpression(site(&site_path))
                })?);
            }
            ASTNode::Return { value: None, .. } => {
                return Err(SourceResultProductErrorV1::UnknownExpression(site(&path)));
            }
            _ => return Err(SourceResultProductErrorV1::UnsupportedStatement),
        }
    }
    Ok(())
}

fn infer_expr(
    state: &mut InferenceState,
    expression: &ASTNode,
    path: &[SourcePathSegmentV1],
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let expr_site = site(path);
    let class = match expression {
        ASTNode::Literal { value, .. } => match value {
            LiteralValue::Integer(_) | LiteralValue::TypedInteger { .. } => ExprClassV1::I64,
            LiteralValue::String(_) => ExprClassV1::String,
            LiteralValue::Bool(_) => ExprClassV1::Bool,
            LiteralValue::Null => ExprClassV1::Null,
            LiteralValue::Float(_) | LiteralValue::Void => {
                return Err(SourceResultProductErrorV1::UnknownExpression(expr_site));
            }
        },
        ASTNode::Variable { name, .. } => {
            let Some(class) = state.locals.get(name).copied() else {
                return Err(SourceResultProductErrorV1::UnknownExpression(expr_site));
            };
            state.operations.push(SourceValueOperationV1::Binding {
                site: expr_site.clone(),
                class: class.result().ok_or_else(|| {
                    SourceResultProductErrorV1::UnknownExpression(expr_site.clone())
                })?,
            });
            class
        }
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let mut left_path = path.to_vec();
            left_path.push(SourcePathSegmentV1::Lhs);
            let mut right_path = path.to_vec();
            right_path.push(SourcePathSegmentV1::Rhs);
            let lhs = infer_expr(state, left, &left_path)?;
            let rhs = infer_expr(state, right, &right_path)?;
            match operator {
                BinaryOperator::Add if lhs == ExprClassV1::String && rhs == ExprClassV1::String => {
                    state.operations.push(SourceValueOperationV1::StringConcat {
                        site: expr_site.clone(),
                    });
                    ExprClassV1::String
                }
                BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
                | BinaryOperator::BitAnd
                | BinaryOperator::BitOr
                | BinaryOperator::BitXor
                | BinaryOperator::Shl
                | BinaryOperator::Shr
                    if lhs == ExprClassV1::I64 && rhs == ExprClassV1::I64 =>
                {
                    ExprClassV1::I64
                }
                BinaryOperator::Equal | BinaryOperator::NotEqual => {
                    if *operator == BinaryOperator::NotEqual
                        && (lhs == ExprClassV1::Null || rhs == ExprClassV1::Null)
                    {
                        if !((lhs == ExprClassV1::String && rhs == ExprClassV1::Null)
                            || (lhs == ExprClassV1::Null && rhs == ExprClassV1::String))
                        {
                            return Err(SourceResultProductErrorV1::NullableOperandNotString(
                                expr_site,
                            ));
                        }
                        let (lhs_path, null_path) = if lhs == ExprClassV1::String {
                            (left_path, right_path)
                        } else {
                            (right_path, left_path)
                        };
                        state.bool_facts.push(SourceBoolFactV1 {
                            owner: state.owner.clone(),
                            site: expr_site.clone(),
                            lhs: site(&lhs_path),
                            null_operand: site(&null_path),
                        });
                    }
                    ExprClassV1::Bool
                }
                _ => return Err(SourceResultProductErrorV1::UnknownExpression(expr_site)),
            }
        }
        ASTNode::UnaryOp { operand, .. } => {
            let mut operand_path = path.to_vec();
            operand_path.push(SourcePathSegmentV1::Operand);
            if infer_expr(state, operand, &operand_path)? != ExprClassV1::I64 {
                return Err(SourceResultProductErrorV1::UnknownExpression(expr_site));
            }
            ExprClassV1::I64
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } if prelude_stmts.is_empty() => {
            let mut tail_path = path.to_vec();
            tail_path.push(SourcePathSegmentV1::BlockExprTail);
            infer_expr(state, tail_expr, &tail_path)?
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let Some(else_body) = else_body else {
                return Err(SourceResultProductErrorV1::InvalidConditionalValue(
                    expr_site,
                ));
            };
            let mut condition_path = path.to_vec();
            condition_path.push(SourcePathSegmentV1::IfCondition);
            let _ = infer_expr(state, condition, &condition_path)?;
            let then_class =
                infer_block_tail(state, then_body, path, SourcePathSegmentV1::IfThenBody)?;
            let else_class =
                infer_block_tail(state, else_body, path, SourcePathSegmentV1::IfElseBody)?;
            if then_class != else_class {
                return Err(SourceResultProductErrorV1::MixedReturnClasses);
            }
            let result = then_class
                .result()
                .ok_or_else(|| SourceResultProductErrorV1::UnknownExpression(expr_site.clone()))?;
            state
                .operations
                .push(SourceValueOperationV1::ConditionalValue {
                    site: expr_site.clone(),
                    class: result,
                });
            then_class
        }
        _ => return Err(SourceResultProductErrorV1::UnknownExpression(expr_site)),
    };
    if let (Some(result), ASTNode::Literal { .. }) = (class.result(), expression) {
        state.operations.push(SourceValueOperationV1::Literal {
            site: expr_site,
            class: result,
        });
    }
    Ok(class)
}

fn infer_block_tail(
    state: &mut InferenceState,
    body: &[ASTNode],
    parent: &[SourcePathSegmentV1],
    container: SourcePathSegmentV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    if body.len() != 1 {
        return Err(SourceResultProductErrorV1::InvalidConditionalValue(site(
            parent,
        )));
    }
    let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = &body[0]
    else {
        return Err(SourceResultProductErrorV1::InvalidConditionalValue(site(
            parent,
        )));
    };
    if !prelude_stmts.is_empty() {
        return Err(SourceResultProductErrorV1::InvalidConditionalValue(site(
            parent,
        )));
    }
    let mut path = parent.to_vec();
    path.push(container);
    path.push(match path.last() {
        Some(SourcePathSegmentV1::IfThenBody) => SourcePathSegmentV1::IfThen(0),
        Some(SourcePathSegmentV1::IfElseBody) => SourcePathSegmentV1::IfElse(0),
        _ => {
            return Err(SourceResultProductErrorV1::InvalidConditionalValue(site(
                parent,
            )))
        }
    });
    path.push(SourcePathSegmentV1::BlockExprTail);
    infer_expr(state, tail_expr, &path)
}

fn site(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::SameModuleCallableNamespaceV1;
    use crate::parser::NyashParser;

    fn issue(
        source: &str,
        owner: &str,
        method: &str,
    ) -> Result<VerifiedSourceResultProductV1, SourceResultProductErrorV1> {
        let root = NyashParser::parse_from_string(source).expect("fixture parses");
        let declarations =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
        let key = declarations
            .declarations()
            .find_map(|(key, _)| {
                (key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod
                    && key.owner() == owner
                    && key.name() == method)
                    .then_some(key.clone())
            })
            .unwrap();
        issue_source_result_product_v1(&declarations, &key)
    }

    #[test]
    fn source_result_product_preserves_string_conditional_and_null_fact() {
        let product = issue(
            r#"static box Helpers { value() { local sval = "x" return sval != null ? { "a" } : { "b" } } }"#,
            "Helpers",
            "value",
        )
        .unwrap();
        assert_eq!(product.result(), SourceResultClassV1::String);
        assert!(product.operations().iter().any(|row| matches!(
            row,
            SourceValueOperationV1::ConditionalValue {
                class: SourceResultClassV1::String,
                ..
            }
        )));
        assert_eq!(product.bool_facts().len(), 1);
    }

    #[test]
    fn source_result_product_rejects_mixed_conditional_classes() {
        let result = issue(
            r#"static box Helpers { value() { return 1 != 0 ? { 1 } : { "b" } } }"#,
            "Helpers",
            "value",
        );
        assert!(matches!(
            result,
            Err(SourceResultProductErrorV1::MixedReturnClasses)
        ));
    }

    #[test]
    fn source_result_product_rejects_nullable_guard_without_string_binding() {
        let result = issue(
            r#"static box Helpers { value() { return 1 != null ? { 1 } : { 0 } } }"#,
            "Helpers",
            "value",
        );
        assert!(matches!(
            result,
            Err(SourceResultProductErrorV1::NullableOperandNotString(_))
        ));
    }

    #[test]
    fn source_result_product_rejects_foreign_owner_before_source_walk() {
        let root =
            NyashParser::parse_from_string("static box Helpers { value() { return 1 } }").unwrap();
        let declarations =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
        let foreign = CanonicalSameModuleCallableKeyV1::static_box_method("Other", "value", 0);
        assert!(matches!(
            issue_source_result_product_v1(&declarations, &foreign),
            Err(SourceResultProductErrorV1::ForeignCallable)
        ));
    }
}
