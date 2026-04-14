//! GenericLoop body lowering helpers (apply-only).

mod carriers;
mod helpers;
mod terminality;
mod v0;
mod v1;

pub(in crate::mir::builder) use carriers::{
    finalize_generic_loop_v1_carriers, prepare_generic_loop_v1_carriers,
};
pub(in crate::mir::builder) use terminality::{
    body_plans_exit_on_all_paths, plans_require_continue_edge,
};
pub(in crate::mir::builder) use v0::lower_generic_loop_v0_body;
pub(in crate::mir::builder) use v1::lower_generic_loop_v1_body;

const GENERIC_LOOP_ERR: &str = "[normalizer] generic loop v0";
