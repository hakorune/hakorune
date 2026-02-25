use super::types::{StepNode, StepTree};

impl StepTree {
    pub fn signature_basis_string(&self) -> String {
        let mut kinds = Vec::new();
        collect_node_kinds(&self.root, &mut kinds);
        let kinds = kinds.join(",");
        self.contract.signature_basis_string(&kinds)
    }
}

pub(super) fn collect_node_kinds(node: &StepNode, out: &mut Vec<String>) {
    match node {
        StepNode::Block(nodes) => {
            out.push("Block".to_string());
            for n in nodes {
                collect_node_kinds(n, out);
            }
        }
        StepNode::If {
            then_branch,
            else_branch,
            ..
        } => {
            out.push("If".to_string());
            collect_node_kinds(then_branch, out);
            if let Some(else_branch) = else_branch {
                collect_node_kinds(else_branch, out);
            }
        }
        StepNode::Loop { body, .. } => {
            out.push("Loop".to_string());
            collect_node_kinds(body, out);
        }
        StepNode::Stmt { kind, .. } => {
            out.push(format!("Stmt({})", kind.to_compact_string()));
        }
    }
}
