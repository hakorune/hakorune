use crate::ASTNode;

impl ASTNode {
    /// FunctionDeclarationのパラメータ数を取得
    pub fn get_param_count(&self) -> usize {
        match self {
            ASTNode::FunctionDeclaration { params, .. } => params.len(),
            _ => 0,
        }
    }

    /// Returns true if this node contains a `return` statement (recursively).
    ///
    /// Scope boundaries (`lambda` / `function` / `box`) stop the search.
    pub fn contains_return_stmt(&self) -> bool {
        fn contains(node: &ASTNode) -> bool {
            match node {
                ASTNode::Return { .. } => true,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                _ => node.any_child(contains),
            }
        }

        contains(self)
    }

    /// Returns true if this node contains `break` or `continue` (recursively).
    ///
    /// Scope boundaries (`lambda` / `function` / `box`) stop the search.
    pub fn contains_break_continue(&self) -> bool {
        fn contains(node: &ASTNode) -> bool {
            match node {
                ASTNode::Break { .. } | ASTNode::Continue { .. } => true,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                _ => node.any_child(contains),
            }
        }

        contains(self)
    }

    pub fn contains_non_local_exit(&self) -> bool {
        match self {
            ASTNode::Return { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Throw { .. } => true,

            // Scope boundary: exits inside nested function/box/lambda do not escape.
            ASTNode::Lambda { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::BoxDeclaration { .. }
            | ASTNode::StaticConstTable { .. } => false,

            _ => self.any_child(ASTNode::contains_non_local_exit),
        }
    }

    /// Returns true if this node contains a non-local exit *outside of nested loops*.
    ///
    /// Contract:
    /// - `return` / `throw` are always treated as non-local exits (unless inside a scope boundary).
    /// - `break` / `continue` are treated as non-local exits only when they occur outside any
    ///   `loop` / `while` / `for` (loop_depth == 0).
    /// - Scope boundaries (`lambda` / `function` / `box`) stop the search.
    ///
    /// This is an observation helper used by Facts-level recipe builders where nested loops are
    /// permitted, but exits that would escape the surrounding block must be rejected.
    pub fn contains_non_local_exit_outside_loops(&self) -> bool {
        fn contains(node: &ASTNode, loop_depth: usize) -> bool {
            match node {
                ASTNode::Return { .. } | ASTNode::Throw { .. } => true,
                ASTNode::Break { .. } | ASTNode::Continue { .. } => loop_depth == 0,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                ASTNode::Program { statements, .. } => {
                    statements.iter().any(|s| contains(s, loop_depth))
                }
                ASTNode::Assignment { target, value, .. } => {
                    contains(target, loop_depth) || contains(value, loop_depth)
                }
                ASTNode::Print { expression, .. } => contains(expression, loop_depth),
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    contains(condition, loop_depth)
                        || then_body.iter().any(|s| contains(s, loop_depth))
                        || else_body
                            .as_ref()
                            .is_some_and(|b| b.iter().any(|s| contains(s, loop_depth)))
                }
                ASTNode::Loop {
                    condition, body, ..
                } => {
                    contains(condition, loop_depth)
                        || body
                            .iter()
                            .any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::LoopRange {
                    start, end, body, ..
                } => {
                    contains(start, loop_depth)
                        || contains(end, loop_depth)
                        || body
                            .iter()
                            .any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::UsingStatement { .. }
                | ASTNode::ImportStatement { .. }
                | ASTNode::BuildGate { .. } => false,
                ASTNode::FromCall { .. } => false,
                _ => node.any_child(|child| contains(child, loop_depth)),
            }
        }

        contains(self, 0)
    }
}
