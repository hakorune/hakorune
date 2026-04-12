use serde_json::json;
use std::io::Write;

mod agg_local;
mod decls;
mod emitters;
mod helpers;
mod io;
mod order;
mod placement_effect;
mod plans;
mod root;
#[cfg(test)]
mod tests;

use agg_local::build_agg_local_scalarization_routes_json;
use decls::{collect_sorted_enum_decl_values, collect_sorted_user_box_decl_values};
pub use io::{
    emit_mir_json_for_harness, emit_mir_json_for_harness_bin, emit_mir_json_string_for_harness_bin,
};
use order::ordered_harness_functions;
use placement_effect::build_placement_effect_routes_json;
use plans::build_string_kernel_plan_json;
use root::build_mir_json_root;
