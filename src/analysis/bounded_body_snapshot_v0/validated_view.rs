//! Normalized read-only projection over a fully validated ProgramV0 body.

use super::strict_json::StrictJsonValue;
use super::{
    AtomKeyV0, ChildRoleV0, TextClassV0, ValidatedProgramV0BodyView, WireExprKindV0,
    WireNodeKindV0, WireStmtKindV0,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatedTextV0<'a> {
    pub value: &'a str,
    pub utf8_byte_len: usize,
    pub class: TextClassV0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidatedAtomValueV0<'a> {
    I64(i64),
    Bool(bool),
    Text(ValidatedTextV0<'a>),
    Null,
}

#[derive(Clone, Copy, Debug)]
pub struct ValidatedNodeV0<'a> {
    value: &'a StrictJsonValue,
    kind: WireNodeKindV0,
}

impl ValidatedProgramV0BodyView {
    pub fn source_program_version(&self) -> i32 {
        0
    }

    pub fn body_node(&self, index: usize) -> Option<ValidatedNodeV0<'_>> {
        let value = self.root.object_field("body")?.array_items()?.get(index)?;
        Some(ValidatedNodeV0::statement(value))
    }
}

impl<'a> ValidatedNodeV0<'a> {
    fn statement(value: &'a StrictJsonValue) -> Self {
        let kind = WireStmtKindV0::from_wire_text(required_string(value, "type"))
            .expect("validated statement kind must remain accepted");
        Self {
            value,
            kind: WireNodeKindV0::Stmt(kind),
        }
    }

    fn expression(value: &'a StrictJsonValue) -> Self {
        let kind = WireExprKindV0::from_wire_text(required_string(value, "type"))
            .expect("validated expression kind must remain accepted");
        Self {
            value,
            kind: WireNodeKindV0::Expr(kind),
        }
    }

    pub fn kind(self) -> WireNodeKindV0 {
        self.kind
    }

    pub fn atoms(self) -> Vec<(AtomKeyV0, ValidatedAtomValueV0<'a>)> {
        let atom = match self.kind {
            WireNodeKindV0::Stmt(WireStmtKindV0::Local) => {
                text_atom(self.value, "name", AtomKeyV0::Name, TextClassV0::Atom)
            }
            WireNodeKindV0::Stmt(WireStmtKindV0::LoopRange) => text_atom(
                self.value,
                "var_name",
                AtomKeyV0::VarName,
                TextClassV0::Atom,
            ),
            WireNodeKindV0::Stmt(_) => return Vec::new(),
            WireNodeKindV0::Expr(WireExprKindV0::Int) => (
                AtomKeyV0::Value,
                ValidatedAtomValueV0::I64(
                    required(self.value, "value")
                        .exact_i64()
                        .expect("validated Int must normalize to i64"),
                ),
            ),
            WireNodeKindV0::Expr(WireExprKindV0::Str) => {
                text_atom(self.value, "value", AtomKeyV0::Value, TextClassV0::Literal)
            }
            WireNodeKindV0::Expr(WireExprKindV0::Bool) => (
                AtomKeyV0::Value,
                ValidatedAtomValueV0::Bool(
                    required(self.value, "value")
                        .boolean()
                        .expect("validated Bool must remain bool"),
                ),
            ),
            WireNodeKindV0::Expr(WireExprKindV0::Null) => {
                (AtomKeyV0::Value, ValidatedAtomValueV0::Null)
            }
            WireNodeKindV0::Expr(WireExprKindV0::Var | WireExprKindV0::Call) => {
                text_atom(self.value, "name", AtomKeyV0::Name, TextClassV0::Atom)
            }
            WireNodeKindV0::Expr(WireExprKindV0::Method) => {
                text_atom(self.value, "method", AtomKeyV0::Method, TextClassV0::Atom)
            }
            WireNodeKindV0::Expr(WireExprKindV0::Field) => {
                text_atom(self.value, "field", AtomKeyV0::Field, TextClassV0::Atom)
            }
            WireNodeKindV0::Expr(
                WireExprKindV0::Binary | WireExprKindV0::Compare | WireExprKindV0::Logical,
            ) => text_atom(self.value, "op", AtomKeyV0::Op, TextClassV0::Atom),
        };
        vec![atom]
    }

    pub fn children(self) -> Vec<(ChildRoleV0, ValidatedNodeV0<'a>)> {
        let mut children = Vec::new();
        match self.kind {
            WireNodeKindV0::Stmt(
                WireStmtKindV0::Local | WireStmtKindV0::Expr | WireStmtKindV0::Return,
            ) => push_expr(
                &mut children,
                ChildRoleV0::Expr,
                required(self.value, "expr"),
            ),
            WireNodeKindV0::Stmt(WireStmtKindV0::If) => {
                push_expr(
                    &mut children,
                    ChildRoleV0::Cond,
                    required(self.value, "cond"),
                );
                push_stmt_body(
                    &mut children,
                    ChildRoleV0::Then,
                    required(self.value, "then"),
                );
                if let Some(otherwise) = self.value.object_field("else") {
                    if !matches!(otherwise, StrictJsonValue::Null) {
                        push_stmt_body(&mut children, ChildRoleV0::Else, otherwise);
                    }
                }
            }
            WireNodeKindV0::Stmt(WireStmtKindV0::Loop) => {
                push_expr(
                    &mut children,
                    ChildRoleV0::Cond,
                    required(self.value, "cond"),
                );
                push_stmt_body(
                    &mut children,
                    ChildRoleV0::Body,
                    required(self.value, "body"),
                );
            }
            WireNodeKindV0::Stmt(WireStmtKindV0::LoopRange) => {
                push_expr(
                    &mut children,
                    ChildRoleV0::Start,
                    required(self.value, "start"),
                );
                push_expr(&mut children, ChildRoleV0::End, required(self.value, "end"));
                push_stmt_body(
                    &mut children,
                    ChildRoleV0::Body,
                    required(self.value, "body"),
                );
            }
            WireNodeKindV0::Stmt(WireStmtKindV0::Break | WireStmtKindV0::Continue)
            | WireNodeKindV0::Expr(
                WireExprKindV0::Int
                | WireExprKindV0::Str
                | WireExprKindV0::Bool
                | WireExprKindV0::Null
                | WireExprKindV0::Var,
            ) => {}
            WireNodeKindV0::Expr(
                WireExprKindV0::Binary | WireExprKindV0::Compare | WireExprKindV0::Logical,
            ) => {
                push_expr(&mut children, ChildRoleV0::Lhs, required(self.value, "lhs"));
                push_expr(&mut children, ChildRoleV0::Rhs, required(self.value, "rhs"));
            }
            WireNodeKindV0::Expr(WireExprKindV0::Call) => {
                push_expr_list(
                    &mut children,
                    ChildRoleV0::Args,
                    required(self.value, "args"),
                );
            }
            WireNodeKindV0::Expr(WireExprKindV0::Method) => {
                push_expr(
                    &mut children,
                    ChildRoleV0::Recv,
                    required(self.value, "recv"),
                );
                push_expr_list(
                    &mut children,
                    ChildRoleV0::Args,
                    required(self.value, "args"),
                );
            }
            WireNodeKindV0::Expr(WireExprKindV0::Field) => {
                push_expr(
                    &mut children,
                    ChildRoleV0::Recv,
                    required(self.value, "recv"),
                );
            }
        }
        children
    }
}

fn required<'a>(value: &'a StrictJsonValue, field: &str) -> &'a StrictJsonValue {
    value
        .object_field(field)
        .expect("validated node must retain required field")
}

fn required_string<'a>(value: &'a StrictJsonValue, field: &str) -> &'a str {
    required(value, field)
        .string()
        .expect("validated text field must remain string")
}

fn text_atom<'a>(
    value: &'a StrictJsonValue,
    field: &str,
    key: AtomKeyV0,
    class: TextClassV0,
) -> (AtomKeyV0, ValidatedAtomValueV0<'a>) {
    let text = required_string(value, field);
    (
        key,
        ValidatedAtomValueV0::Text(ValidatedTextV0 {
            value: text,
            utf8_byte_len: text.len(),
            class,
        }),
    )
}

fn push_expr<'a>(
    output: &mut Vec<(ChildRoleV0, ValidatedNodeV0<'a>)>,
    role: ChildRoleV0,
    value: &'a StrictJsonValue,
) {
    output.push((role, ValidatedNodeV0::expression(value)));
}

fn push_stmt_body<'a>(
    output: &mut Vec<(ChildRoleV0, ValidatedNodeV0<'a>)>,
    role: ChildRoleV0,
    value: &'a StrictJsonValue,
) {
    for statement in value
        .array_items()
        .expect("validated body must remain an array")
    {
        output.push((role, ValidatedNodeV0::statement(statement)));
    }
}

fn push_expr_list<'a>(
    output: &mut Vec<(ChildRoleV0, ValidatedNodeV0<'a>)>,
    role: ChildRoleV0,
    value: &'a StrictJsonValue,
) {
    for expression in value
        .array_items()
        .expect("validated args must remain an array")
    {
        output.push((role, ValidatedNodeV0::expression(expression)));
    }
}
