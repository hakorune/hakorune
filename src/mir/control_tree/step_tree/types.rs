use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::control_tree::step_tree_contract_box::StepTreeContract;

#[derive(Debug, Clone, PartialEq)]
pub struct StepTree {
    pub root: StepNode,
    pub features: StepTreeFeatures,
    pub contract: StepTreeContract,
    pub signature: StepTreeSignature,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StepTreeFeatures {
    pub has_if: bool,
    pub has_loop: bool,
    pub has_break: bool,
    pub has_continue: bool,
    pub has_return: bool,
    pub max_if_depth: u32,
    pub max_loop_depth: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepNode {
    Block(Vec<StepNode>),
    If {
        cond: AstSummary,
        cond_ast: AstNodeHandle,
        then_branch: Box<StepNode>,
        else_branch: Option<Box<StepNode>>,
        span: Span,
    },
    Loop {
        cond: AstSummary,
        cond_ast: AstNodeHandle,
        body: Box<StepNode>,
        span: Span,
    },
    Stmt { kind: StepStmtKind, span: Span },
}

/// AST 参照の軽量ハンドル（Phase 119: dev-only 観測用）
///
/// SSOT: cond は AST 参照を保持する。
/// - 将来的に AstExprId 等に移行可能。
/// - Phase 119 では Clone を持つ Box<ASTNode> で実装（dev-only なので許容）。
#[derive(Debug, Clone, PartialEq)]
pub struct AstNodeHandle(pub Box<ASTNode>);

#[derive(Debug, Clone, PartialEq)]
pub enum StepStmtKind {
    LocalDecl { vars: Vec<String> },
    Assign {
        target: Option<String>,
        /// Phase 128: assignment value AST (for Normalized lowering)
        value_ast: Option<AstNodeHandle>,
    },
    Print,
    Return {
        /// Phase 123: return value AST (for Normalized lowering)
        value_ast: Option<AstNodeHandle>,
    },
    Break,
    Continue,
    Other(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstSummary {
    Variable(String),
    Literal(LiteralValue),
    Unary {
        op: UnaryOperator,
        expr: Box<AstSummary>,
    },
    Binary {
        op: BinaryOperator,
        lhs: Box<AstSummary>,
        rhs: Box<AstSummary>,
    },
    Other(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExitKind {
    Return,
    Break,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepCapability {
    If,
    Loop,
    NestedIf,
    NestedLoop,
    Return,
    Break,
    Continue,
    TryCatch,
    Throw,
    Lambda,
    While,
    ForRange,
    Match,
    Arrow,
}

// StepTreeContract moved to step_tree_contract_box.rs (Phase 120)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepTreeSignature(pub u64);

impl StepTreeSignature {
    pub fn from_basis_string(basis: &str) -> Self {
        // FNV-1a 64-bit (stable, no external deps).
        let mut hash: u64 = 0xcbf29ce484222325;
        for b in basis.as_bytes() {
            hash ^= *b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        StepTreeSignature(hash)
    }

    pub fn to_hex(self) -> String {
        format!("{:016x}", self.0)
    }
}

impl StepNode {
    pub(super) fn with_span(self, span: Span) -> StepNode {
        match self {
            StepNode::Block(nodes) => StepNode::Block(nodes),
            StepNode::If {
                cond,
                cond_ast,
                then_branch,
                else_branch,
                ..
            } => StepNode::If {
                cond,
                cond_ast,
                then_branch,
                else_branch,
                span,
            },
            StepNode::Loop {
                cond, cond_ast, body, ..
            } => StepNode::Loop {
                cond,
                cond_ast,
                body,
                span,
            },
            StepNode::Stmt { kind, .. } => StepNode::Stmt { kind, span },
        }
    }
}
