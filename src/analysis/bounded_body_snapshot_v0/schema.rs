#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotLimitsV0 {
    pub max_depth: usize,
    pub max_node_count: usize,
    pub max_children_per_body: usize,
    pub max_arguments: usize,
    pub max_literal_bytes: usize,
    pub max_atom_bytes: usize,
    pub max_total_text_bytes: usize,
}

impl SnapshotLimitsV0 {
    pub const SCHEMA: Self = Self {
        max_depth: 64,
        max_node_count: 32_768,
        max_children_per_body: 2_048,
        max_arguments: 128,
        max_literal_bytes: 65_536,
        max_atom_bytes: 1_024,
        max_total_text_bytes: 4_194_304,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireClassificationV0 {
    Accepted,
    KnownUnsupported,
    SchemaMismatchStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireStmtKindV0 {
    Local,
    Expr,
    If,
    Loop,
    LoopRange,
    Return,
    Break,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireExprKindV0 {
    Int,
    Str,
    Bool,
    Null,
    Var,
    Binary,
    Compare,
    Logical,
    Call,
    Method,
    Field,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireNodeKindV0 {
    Stmt(WireStmtKindV0),
    Expr(WireExprKindV0),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChildRoleV0 {
    Expr,
    Cond,
    Then,
    Else,
    Body,
    Start,
    End,
    Lhs,
    Rhs,
    Recv,
    Args,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperatorV0 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperatorV0 {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperatorV0 {
    And,
    Or,
}
