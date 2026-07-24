//! BODY0-S0 neutral source recipe.
//!
//! This contract is shared by the compiler source-facts producer and the
//! Builder body terminal.  It deliberately carries no AST, Builder, or
//! publication authority.  The first executable grammar is LinearScalar0.

use crate::ast::{BinaryOperator, LiteralValue, Span};

#[derive(Debug)]
pub(crate) struct RawRootBodyRecipeV1 {
    entry: RawRootBodyEntryV1,
    statements: Box<[RawLinearScalarStmtV1]>,
    _seal: RawRootBodyRecipeSealV1,
}

#[derive(Debug, PartialEq)]
pub(crate) enum RawRootBodyEntryV1 {
    Script,
    AppMain0Void { top_level_statement: usize },
}

#[derive(Debug, PartialEq)]
pub(crate) struct RawRootBodySourceSiteV1 {
    path: Box<[usize]>,
    span: Span,
}

impl RawRootBodySourceSiteV1 {
    pub(crate) fn new(path: &[usize], span: Span) -> Self {
        Self {
            path: path.to_vec().into_boxed_slice(),
            span,
        }
    }

    pub(crate) fn path(&self) -> &[usize] {
        &self.path
    }

    pub(crate) const fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawLinearUnaryOperatorV1 {
    Minus,
    Not,
    BitNot,
}

#[derive(Debug, PartialEq)]
pub(crate) enum RawLinearScalarExprV1 {
    Literal {
        value: LiteralValue,
        site: RawRootBodySourceSiteV1,
    },
    Variable {
        name: Box<str>,
        site: RawRootBodySourceSiteV1,
    },
    Unary {
        operator: RawLinearUnaryOperatorV1,
        operand: Box<Self>,
        site: RawRootBodySourceSiteV1,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Self>,
        right: Box<Self>,
        site: RawRootBodySourceSiteV1,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum RawLinearScalarStmtV1 {
    Expr {
        expression: RawLinearScalarExprV1,
        site: RawRootBodySourceSiteV1,
    },
    Print {
        expression: RawLinearScalarExprV1,
        site: RawRootBodySourceSiteV1,
    },
    Assignment {
        target: Box<str>,
        value: RawLinearScalarExprV1,
        site: RawRootBodySourceSiteV1,
    },
    CompoundAssignment {
        target: Box<str>,
        operator: BinaryOperator,
        value: RawLinearScalarExprV1,
        site: RawRootBodySourceSiteV1,
    },
    Local {
        variables: Box<[Box<str>]>,
        initialized: Box<[Option<RawLinearScalarExprV1>]>,
        site: RawRootBodySourceSiteV1,
    },
}

#[derive(Debug)]
pub(crate) struct RawRootBodyRecipeSealV1;

impl RawRootBodyRecipeV1 {
    pub(crate) fn from_parts(
        entry: RawRootBodyEntryV1,
        statements: Box<[RawLinearScalarStmtV1]>,
    ) -> Result<Self, RawRootBodyRecipeErrorV1> {
        let mut paths = std::collections::BTreeSet::new();
        for statement in &statements {
            collect_statement_paths(statement, &mut paths)?;
        }
        Ok(Self {
            entry,
            statements,
            _seal: RawRootBodyRecipeSealV1,
        })
    }

    pub(crate) fn entry(&self) -> &RawRootBodyEntryV1 {
        &self.entry
    }

    pub(crate) fn statements(&self) -> &[RawLinearScalarStmtV1] {
        &self.statements
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RawRootBodyRecipeErrorV1 {
    DuplicateSourcePath { path: Box<[usize]> },
    UnsupportedStatement { path: Box<[usize]> },
    UnsupportedOperator { path: Box<[usize]> },
}

fn collect_statement_paths(
    statement: &RawLinearScalarStmtV1,
    paths: &mut std::collections::BTreeSet<Box<[usize]>>,
) -> Result<(), RawRootBodyRecipeErrorV1> {
    let site = match statement {
        // An expression statement and its expression are the same source AST
        // node.  The expression site is the canonical provenance; inserting
        // the wrapper site as a second path would reject every literal-only
        // statement as a false duplicate.
        RawLinearScalarStmtV1::Expr { .. } => None,
        RawLinearScalarStmtV1::Print { site, .. }
        | RawLinearScalarStmtV1::Assignment { site, .. }
        | RawLinearScalarStmtV1::CompoundAssignment { site, .. }
        | RawLinearScalarStmtV1::Local { site, .. } => Some(site),
    };
    if let Some(site) = site {
        insert_path(site, paths)?;
    }
    match statement {
        RawLinearScalarStmtV1::Expr { expression, .. }
        | RawLinearScalarStmtV1::Print { expression, .. } => collect_expr_paths(expression, paths),
        RawLinearScalarStmtV1::Assignment { value, .. }
        | RawLinearScalarStmtV1::CompoundAssignment { value, .. } => {
            collect_expr_paths(value, paths)
        }
        RawLinearScalarStmtV1::Local { initialized, .. } => {
            for expression in initialized.iter().flatten() {
                collect_expr_paths(expression, paths)?;
            }
            Ok(())
        }
    }
}

fn collect_expr_paths(
    expression: &RawLinearScalarExprV1,
    paths: &mut std::collections::BTreeSet<Box<[usize]>>,
) -> Result<(), RawRootBodyRecipeErrorV1> {
    let site = match expression {
        RawLinearScalarExprV1::Literal { site, .. }
        | RawLinearScalarExprV1::Variable { site, .. }
        | RawLinearScalarExprV1::Unary { site, .. }
        | RawLinearScalarExprV1::Binary { site, .. } => site,
    };
    insert_path(site, paths)?;
    match expression {
        RawLinearScalarExprV1::Unary { operand, .. } => collect_expr_paths(operand, paths),
        RawLinearScalarExprV1::Binary { left, right, .. } => {
            collect_expr_paths(left, paths)?;
            collect_expr_paths(right, paths)
        }
        RawLinearScalarExprV1::Literal { .. } | RawLinearScalarExprV1::Variable { .. } => Ok(()),
    }
}

fn insert_path(
    site: &RawRootBodySourceSiteV1,
    paths: &mut std::collections::BTreeSet<Box<[usize]>>,
) -> Result<(), RawRootBodyRecipeErrorV1> {
    if paths.insert(site.path.clone()) {
        Ok(())
    } else {
        Err(RawRootBodyRecipeErrorV1::DuplicateSourcePath {
            path: site.path.clone(),
        })
    }
}
