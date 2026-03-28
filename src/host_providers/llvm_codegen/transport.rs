use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::Opts;

pub(super) fn build_backend_temp_input_path() -> PathBuf {
    std::env::temp_dir().join("hako_llvm_in.json")
}

pub(super) fn prepare_backend_input_json_file(mir_json: &str) -> Result<PathBuf, String> {
    let in_path = build_backend_temp_input_path();
    let mut f =
        fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    f.write_all(mir_json.as_bytes())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    Ok(in_path)
}

pub(super) fn resolve_backend_object_output(opts: &Opts) -> PathBuf {
    if let Some(p) = opts.out.clone() {
        p
    } else {
        std::env::temp_dir().join("hako_llvm_out.o")
    }
}

pub(super) fn ensure_backend_output_parent(out_path: &Path) {
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}
