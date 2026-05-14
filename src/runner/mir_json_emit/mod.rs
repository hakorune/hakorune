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
    collect_array_record_autouse_eligibility_plan_values,
    collect_array_record_materialization_boundary_plan_values,
    collect_array_record_packed_autouse_pilot_plan_values,
    collect_array_record_storage_plan_values,
    collect_hako_alloc_aligned_small_packed_store_pilot_plan_values,
    collect_hako_alloc_huge_page_packed_store_pilot_plan_values, collect_record_layout_plan_values,
    collect_source_packed_array_autouse_pilot_plan_values,
    collect_sorted_enum_decl_values, collect_sorted_record_decl_values,
    collect_sorted_user_box_decl_values, collect_static_data_plan_values,
    collect_typed_object_plan_values,
};
pub use io::{
    emit_mir_json_for_harness, emit_mir_json_for_harness_bin, emit_mir_json_string_for_harness_bin,
};
#[cfg(test)]
use order::ordered_harness_functions;
use root::build_mir_json_root;
