//! One declaration identity and signature, shared by every later stage.
use crate::ast::Span;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Type {
    I64,
    Bool,
    Unit,
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
}

#[derive(Debug)]
pub struct DeclarationTable {
    pub(super) entries: Vec<Declaration>,
}

#[derive(Debug)]
pub struct ResolvedModule {
    pub(super) declarations: DeclarationTable,
    pub(super) functions: Vec<ResolvedFunction>,
}

#[derive(Debug)]
pub(super) struct ResolvedFunction {
    pub declaration: DeclarationId,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub(super) struct Statement {
    pub id: NodeId,
    pub span: Span,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub(super) enum StatementKind {
    Return(Expression),
}

#[derive(Debug)]
pub(super) struct Expression {
    pub id: NodeId,
    pub span: Span,
    pub ty: Type,
    pub kind: ExpressionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Binary {
    Add,
}

#[derive(Debug)]
pub(super) enum ExpressionKind {
    Integer(i64),
    Bool(bool),
    Binary {
        op: Binary,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

/// Named source rejection, owned exclusively by Resolve. No MIR is attached.
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
