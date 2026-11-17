//! Resolve's immutable declarations, typed source nodes and control contracts.
use crate::ast::Span;
pub(super) use crate::ast::{BinaryOperator as Binary, UnaryOperator as Unary};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Type {
    I64,
    Bool,
    Unit,
}
impl Type {
    /// Explicit representation projection, never inferred from physical values.
    pub fn mir(self) -> crate::mir::MirType {
        match self {
            Self::I64 => crate::mir::MirType::Integer,
            Self::Bool => crate::mir::MirType::Bool,
            Self::Unit => crate::mir::MirType::Void,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct DeclarationId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct BindingId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct NodeId(pub usize);

#[derive(Debug)]
pub(super) struct Declaration {
    pub id: DeclarationId,
    pub key: CanonicalSameModuleCallableKeyV1,
    pub owner: Option<String>,
    pub parameters: Vec<(String, Type)>,
    pub result: Type,
    pub span: Span,
    pub io: bool,
}
#[derive(Debug)]
pub struct DeclarationTable {
    pub(super) entries: Vec<Declaration>,
}
#[derive(Debug)]
pub struct ResolvedModule {
    pub(super) declarations: DeclarationTable,
    pub(super) functions: Vec<ResolvedFunction>,
    pub(super) entry: DeclarationId,
}
#[derive(Debug)]
pub(super) struct ResolvedFunction {
    pub declaration: DeclarationId,
    pub bindings: Vec<Binding>,
    pub body: Vec<Statement>,
}
#[derive(Debug)]
pub(super) struct Binding {
    pub ty: Type,
    pub readonly: bool,
}
#[derive(Debug)]
pub(super) struct Statement {
    pub id: NodeId,
    pub span: Span,
    pub kind: StatementKind,
}
#[derive(Debug)]
pub(super) enum StatementKind {
    Local {
        binding: BindingId,
        value: Expression,
    },
    Assign {
        binding: BindingId,
        value: Expression,
    },
    Compound {
        binding: BindingId,
        operator: Binary,
        value: Expression,
    },
    If {
        condition: Expression,
        yes: Vec<Statement>,
        no: Vec<Statement>,
        carriers: Vec<BindingId>,
    },
    Loop(LoopContract),
    Break(NodeId),
    Continue(NodeId),
    Return(Expression),
    Expression(Expression),
    Scope(Vec<Statement>),
}
#[derive(Debug)]
pub(super) struct LoopContract {
    pub test: LoopTest,
    pub body: Vec<Statement>,
    pub carriers: Vec<BindingId>,
    pub assigned: Vec<BindingId>,
}
#[derive(Debug)]
pub(super) enum LoopTest {
    Condition {
        expression: Expression,
        always: bool,
    },
    Range {
        index: BindingId,
        start: Expression,
        end: Expression,
    },
}
#[derive(Debug)]
pub(super) struct Expression {
    pub id: NodeId,
    pub span: Span,
    pub ty: Type,
    pub kind: ExpressionKind,
}
#[derive(Debug)]
pub(super) enum ExpressionKind {
    Integer(i64),
    Bool(bool),
    Read(BindingId),
    Unary {
        operator: Unary,
        operand: Box<Expression>,
    },
    Binary {
        operator: Binary,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call(ResolvedCall),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallTarget {
    Declaration(DeclarationId),
    Print,
}
#[derive(Debug)]
pub(super) struct ResolvedCall {
    pub target: CallTarget,
    /// All admitted round-one operations use existing non-Fault scalar contracts.
    pub may_fault: bool,
    /// Receiver is a role, never an invented source argument. Empty in round 1.
    pub receiver: Option<Box<Expression>>,
    pub arguments: Vec<Expression>,
}
/// Every source rejection originates in Resolve; no physical MIR is attached.
#[derive(Debug)]
pub struct Refusal {
    pub code: &'static str,
    pub detail: String,
    pub span: Span,
}
impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[blueprint/resolve/{}] {} at {}:{}",
            self.code, self.detail, self.span.line, self.span.column
        )
    }
}
impl std::error::Error for Refusal {}
