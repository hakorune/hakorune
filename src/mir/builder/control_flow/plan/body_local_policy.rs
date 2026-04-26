//! Phase 92 P3: BodyLocal policy routing (Box)
//!
//! Purpose: make the "promotion vs read-only slot vs reject" decision explicit,
//! so loop-break routing code does not look like it "falls back" after failure.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::support::condition_scope::LoopConditionScope;

use super::body_local_policy_runner::classify_body_local_policy_route;
pub use super::body_local_policy_types::BodyLocalRoute;

pub fn classify_loop_break_body_local_route(
    builder: &MirBuilder,
    loop_var_name: &str,
    scope: &LoopScopeShape,
    break_condition_node: &ASTNode,
    cond_scope: &LoopConditionScope,
    body: &[ASTNode],
) -> PolicyDecision<BodyLocalRoute> {
    classify_body_local_policy_route(
        builder,
        loop_var_name,
        scope,
        break_condition_node,
        cond_scope,
        body,
    )
}
