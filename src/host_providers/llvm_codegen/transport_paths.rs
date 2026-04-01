use std::path::{Path, PathBuf};

use super::Opts;

pub(super) fn build_backend_temp_input_path() -> PathBuf {
    std::env::temp_dir().join("hako_llvm_in.json")
}

pub(super) fn build_backend_compare_source_path(out_path: &Path, lane_tag: &str) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_bridge");
    std::env::temp_dir().join(format!(
        "{}.{}.{}.driver.hako",
        filename,
        std::process::id(),
        lane_tag
    ))
}

pub(super) fn resolve_backend_object_output(opts: &Opts) -> PathBuf {
    if let Some(p) = opts.out.clone() {
        p
    } else {
        std::env::temp_dir().join("hako_llvm_out.o")
    }
}
