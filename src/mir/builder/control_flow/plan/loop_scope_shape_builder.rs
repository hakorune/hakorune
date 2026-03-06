//! LoopScopeShapeBuilder - AST-based LoopScopeShape initialization
//!
//! Phase 171-172: Issue 4
//!
//! Provides unified construction methods for LoopScopeShape across the 4 primary routes.
//! This eliminates the 50-60 lines of duplicated initialization code in each route branch.
//!
//! # Responsibility
//!
//! - Provide factory methods for creating LoopScopeShape with common configurations
//! - Extract body_locals from loop body AST when needed
//! - Maintain consistent initialization defaults across routes
//!
//! # Phase 183-3: AST-Based Construction Context
//!
//! This builder constructs LoopScopeShape from **AST nodes** during MIR building.
//! For LoopForm-based construction (JoinIR lowering), see:
//! - `src/mir/join_ir/lowering/loop_scope_shape/builder.rs`
//!
//! Both builders maintain consistent field initialization for LoopScopeShape.
//!
//! # Usage
//!
//! ```rust
//! // loop_simple_while / if_phi_join: empty body_locals
//! let scope = LoopScopeShapeBuilder::empty_body_locals(
//!     header, body, latch, exit, pinned
//! );
//!
//! // loop_break / loop_continue_only: extract body_locals from AST
//! let scope = LoopScopeShapeBuilder::with_body_locals(
//!     header, body, latch, exit, pinned, loop_body
//! );
//! ```

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::BasicBlockId;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct LoopScopeShapeBuilder;

impl LoopScopeShapeBuilder {
    /// Create LoopScopeShape with empty body_locals
    ///
    /// Used by loop_simple_while and if_phi_join,
    /// which don't require body-local variable analysis.
    ///
    /// # Arguments
    ///
    /// * `header` - Header block ID
    /// * `body` - Body block ID
    /// * `latch` - Latch block ID
    /// * `exit` - Exit block ID
    /// * `pinned` - Pinned variables (typically empty for current routes)
    pub fn empty_body_locals(
        header: BasicBlockId,
        body: BasicBlockId,
        latch: BasicBlockId,
        exit: BasicBlockId,
        pinned: BTreeSet<String>,
    ) -> LoopScopeShape {
        LoopScopeShape {
            header,
            body,
            latch,
            exit,
            pinned,
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        }
    }

    /// Create LoopScopeShape with body_locals extracted from AST
    ///
    /// Used by loop_break and loop_continue_only,
    /// which require body-local variable classification.
    /// This is critical for trim-route support and carrier promotion analysis.
    ///
    /// # Arguments
    ///
    /// * `header` - Header block ID
    /// * `body` - Body block ID
    /// * `latch` - Latch block ID
    /// * `exit` - Exit block ID
    /// * `pinned` - Pinned variables (typically empty for current routes)
    /// * `loop_body` - AST nodes of the loop body for local variable extraction
    pub fn with_body_locals(
        header: BasicBlockId,
        body: BasicBlockId,
        latch: BasicBlockId,
        exit: BasicBlockId,
        pinned: BTreeSet<String>,
        loop_body: &[ASTNode],
    ) -> LoopScopeShape {
        let body_locals = Self::extract_body_locals(loop_body);
        LoopScopeShape {
            header,
            body,
            latch,
            exit,
            pinned,
            carriers: BTreeSet::new(),
            body_locals,
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        }
    }

    /// Extract local variables defined in loop body
    ///
    /// Scans the loop body AST for `local` declarations and collects variable names.
    /// This is used for proper variable classification in LoopConditionScopeBox analysis.
    ///
    /// # Arguments
    ///
    /// * `body` - AST nodes of the loop body
    ///
    /// # Returns
    ///
    /// Set of variable names declared with `local` keyword in the loop body
    fn extract_body_locals(body: &[ASTNode]) -> BTreeSet<String> {
        let mut locals = BTreeSet::new();
        for stmt in body {
            if let ASTNode::Local { variables, .. } = stmt {
                for var_name in variables {
                    locals.insert(var_name.clone());
                }
            }
        }
        locals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_body_locals() {
        let scope = LoopScopeShapeBuilder::empty_body_locals(
            BasicBlockId(1),
            BasicBlockId(2),
            BasicBlockId(3),
            BasicBlockId(4),
            BTreeSet::new(),
        );

        assert_eq!(scope.header, BasicBlockId(1));
        assert_eq!(scope.body, BasicBlockId(2));
        assert_eq!(scope.latch, BasicBlockId(3));
        assert_eq!(scope.exit, BasicBlockId(4));
        assert!(scope.body_locals.is_empty());
        assert!(scope.carriers.is_empty());
        assert!(scope.pinned.is_empty());
    }

    #[test]
    fn test_with_body_locals_extracts_local_variables() {
        use crate::ast::Span;
        let body = vec![
            ASTNode::Local {
                variables: vec!["x".to_string(), "y".to_string()],
                initial_values: vec![],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["z".to_string()],
                initial_values: vec![],
                span: Span::unknown(),
            },
        ];

        let scope = LoopScopeShapeBuilder::with_body_locals(
            BasicBlockId(1),
            BasicBlockId(2),
            BasicBlockId(3),
            BasicBlockId(4),
            BTreeSet::new(),
            &body,
        );

        assert_eq!(scope.body_locals.len(), 3);
        assert!(scope.body_locals.contains("x"));
        assert!(scope.body_locals.contains("y"));
        assert!(scope.body_locals.contains("z"));
    }

    #[test]
    fn test_extract_body_locals_ignores_non_local_nodes() {
        use crate::ast::Span;
        let body = vec![
            ASTNode::Local {
                variables: vec!["a".to_string()],
                initial_values: vec![],
                span: Span::unknown(),
            },
            // Use a different AST node type that doesn't declare locals
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(42),
                span: Span::unknown(),
            },
        ];

        let scope = LoopScopeShapeBuilder::with_body_locals(
            BasicBlockId(0),
            BasicBlockId(0),
            BasicBlockId(0),
            BasicBlockId(0),
            BTreeSet::new(),
            &body,
        );

        assert_eq!(scope.body_locals.len(), 1);
        assert!(scope.body_locals.contains("a"));
    }
}
