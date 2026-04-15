use crate::ast::ASTNode;

use super::facts_helpers::{
    declares_local_var, extract_step_var_from_tail, is_loop_cond_var_lt_var,
};
use super::facts_types::LoopBundleResolverV0ShapePins;

pub(super) fn try_match_loop_bundle_resolver_v0_shape_pins(
    condition: &ASTNode,
    body: &[ASTNode],
    debug_reject: &dyn Fn(&str),
) -> Option<LoopBundleResolverV0ShapePins> {
    let Some((loop_var, limit_var)) = is_loop_cond_var_lt_var(condition) else {
        debug_reject("cond_not_var_lt_var");
        return None;
    };

    if body.len() < 2 {
        debug_reject("body_too_short");
        return None;
    }

    let Some(last) = body.last() else {
        debug_reject("body_last_missing");
        return None;
    };
    let Some(step_var) = extract_step_var_from_tail(last, &loop_var) else {
        debug_reject("tail_not_loopvar_eq_stepvar");
        return None;
    };

    let Some(first) = body.first() else {
        debug_reject("body_first_missing");
        return None;
    };
    if !declares_local_var(first, &step_var) {
        debug_reject("step_var_not_declared_as_first_local");
        return None;
    }

    if !body.iter().any(ASTNode::contains_return_stmt) {
        debug_reject("no_return_in_body");
        return None;
    }

    Some(LoopBundleResolverV0ShapePins {
        loop_var,
        limit_var,
        step_var,
    })
}
