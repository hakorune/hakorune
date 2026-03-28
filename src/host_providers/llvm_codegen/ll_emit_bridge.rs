use std::path::PathBuf;

use super::ll_emit_compare_driver;
use super::Opts;

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    ll_emit_compare_driver::mir_json_to_object_hako_ll_compare(mir_json, opts)
}
