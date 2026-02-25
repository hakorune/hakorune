use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::extract_loop_increment_plan;

mod complex_step;
mod next_step;
mod shared;
mod var_step;

pub(crate) fn canon_loop_increment_for_var(
    body: &[ASTNode],
    loop_var: &str,
    allow_var_step: bool,
) -> Option<ASTNode> {
    match extract_loop_increment_plan(body, loop_var) {
        Ok(Some(inc)) => Some(inc),
        _ => {
            if !allow_var_step {
                return None;
            }
            var_step::extract_var_step_increment(body, loop_var)
                .or_else(|| next_step::extract_next_i_increment(body, loop_var))
                .or_else(|| complex_step::extract_complex_step_increment(body, loop_var))
        }
    }
}
