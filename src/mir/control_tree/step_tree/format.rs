use crate::ast::LiteralValue;

use super::types::{AstSummary, StepNode, StepStmtKind, StepTree};

impl StepTree {
    pub fn to_compact_string(&self) -> String {
        let mut out = String::new();
        self.root.write_compact(&mut out, 0);
        out
    }
}

impl StepNode {
    fn write_compact(&self, out: &mut String, indent: usize) {
        let pad = "  ".repeat(indent);
        match self {
            StepNode::Block(nodes) => {
                out.push_str(&format!("{pad}Block(len={})\n", nodes.len()));
                for n in nodes {
                    n.write_compact(out, indent + 1);
                }
            }
            StepNode::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                out.push_str(&format!("{pad}If(cond={})\n", cond.to_compact_string()));
                out.push_str(&format!("{pad}  then:\n"));
                then_branch.write_compact(out, indent + 2);
                if let Some(else_branch) = else_branch {
                    out.push_str(&format!("{pad}  else:\n"));
                    else_branch.write_compact(out, indent + 2);
                }
            }
            StepNode::Loop { cond, body, .. } => {
                out.push_str(&format!("{pad}Loop(cond={})\n", cond.to_compact_string()));
                body.write_compact(out, indent + 1);
            }
            StepNode::Stmt { kind, .. } => {
                out.push_str(&format!("{pad}Stmt({})\n", kind.to_compact_string()));
            }
        }
    }
}

impl StepStmtKind {
    pub(super) fn to_compact_string(&self) -> String {
        match self {
            StepStmtKind::LocalDecl { vars } => format!("local({})", vars.join(",")),
            StepStmtKind::Assign { target, .. } => match target {
                Some(name) => format!("assign({name})"),
                None => "assign(?)".to_string(),
            },
            StepStmtKind::Print => "print".to_string(),
            StepStmtKind::Return { value_ast } => {
                if value_ast.is_some() {
                    "return(value)".to_string()
                } else {
                    "return(void)".to_string()
                }
            }
            StepStmtKind::Break => "break".to_string(),
            StepStmtKind::Continue => "continue".to_string(),
            StepStmtKind::Other(name) => format!("other:{name}"),
        }
    }
}

impl AstSummary {
    pub(super) fn to_compact_string(&self) -> String {
        match self {
            AstSummary::Variable(name) => format!("var:{name}"),
            AstSummary::Literal(lit) => format!("lit:{}", lit_to_sig_string(lit)),
            AstSummary::Unary { op, expr } => format!("({op:?} {})", expr.to_compact_string()),
            AstSummary::Binary { op, lhs, rhs } => {
                format!("({} {} {})", lhs.to_compact_string(), op, rhs.to_compact_string())
            }
            AstSummary::Other(k) => format!("other:{k}"),
        }
    }
}

fn lit_to_sig_string(lit: &LiteralValue) -> String {
    match lit {
        LiteralValue::String(s) => format!("str:{}", escape_sig_atom(s)),
        LiteralValue::Integer(i) => format!("int:{i}"),
        LiteralValue::Float(f) => format!("float:{:016x}", f.to_bits()),
        LiteralValue::Bool(b) => format!("bool:{}", if *b { 1 } else { 0 }),
        LiteralValue::Null => "null".to_string(),
        LiteralValue::Void => "void".to_string(),
    }
}

fn escape_sig_atom(s: &str) -> String {
    // Minimal stable escaping for signature strings.
    s.replace('\\', "\\\\").replace('|', "\\|").replace(',', "\\,")
}
