use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::transport_paths;

pub(super) fn prepare_backend_input_json_file(mir_json: &str) -> Result<PathBuf, String> {
    let in_path = transport_paths::build_backend_temp_input_path();
    let mut f =
        fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    f.write_all(mir_json.as_bytes())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    Ok(in_path)
}

pub(super) fn ensure_backend_output_parent(out_path: &Path) {
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}
