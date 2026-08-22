//! Route-neutral input for source-aware Facts/Recipe planning.
//!
//! This input deliberately carries only the AST slice, the captured policy,
//! and diagnostic-only labels.  It does not carry route classification,
//! static box state, function-body capture state, Builder state, or a lookup
//! table.
//! witness.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopFactsPlannerInputV1<'a> {
    pub(super) condition: &'a ASTNode,
    pub(super) body: &'a [ASTNode],
    pub(super) policy: GenericLoopFactsPolicyFrameV1,
    pub(super) function_name: Box<str>,
    pub(super) debug_enabled: bool,
}

impl<'a> CallableLoopFactsPlannerInputV1<'a> {
    pub(in crate::mir::builder) fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        policy: GenericLoopFactsPolicyFrameV1,
        function_name: Box<str>,
        debug_enabled: bool,
    ) -> Self {
        Self {
            condition,
            body,
            policy,
            function_name,
            debug_enabled,
        }
    }
}
