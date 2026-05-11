use serde_json::json;
use std::io::Write;

mod agg_local;
mod decls;
mod emitters;
mod helpers;
mod io;
mod metadata;
mod order;
mod placement_effect;
mod plan_metadata;
mod plans;
mod root;
mod route_json;
#[cfg(test)]
mod tests;

#[cfg(test)]
use decls::{
    collect_sorted_enum_decl_values, collect_sorted_user_box_decl_values,
    collect_static_data_plan_values, collect_typed_object_plan_values,
};
pub use io::{
    emit_mir_json_for_harness, emit_mir_json_for_harness_bin, emit_mir_json_string_for_harness_bin,
};
#[cfg(test)]
use order::ordered_harness_functions;
use root::build_mir_json_root;
