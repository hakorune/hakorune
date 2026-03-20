use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub fn emit_dummy_object(out: &Path) -> Result<()> {
    let tmp = temporary_dummy_mir_path(out);
    fs::write(&tmp, build_dummy_mir_json())
        .with_context(|| format!("failed to write dummy MIR JSON: {}", tmp.display()))?;
    let result = super::boundary_driver_ffi::emit_object_from_json(&tmp, out);
    let _ = fs::remove_file(&tmp);
    result
}

pub fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    super::boundary_driver_ffi::emit_object_from_json(input, out)
}

pub fn link_object_to_exe(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir: Option<&Path>,
    extra_libs: Option<&str>,
) -> Result<()> {
    super::boundary_driver_ffi::link_object_to_exe(obj, out_exe, nyrt_dir, extra_libs)
}

fn build_dummy_mir_json() -> String {
    r#"{"kind":"MIR","schema_version":"1.0","metadata":{"extern_c":[]},"functions":[{"name":"ny_main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"ret","value":1}]}]}]}"#
        .to_string()
}

fn temporary_dummy_mir_path(out: &Path) -> PathBuf {
    let filename = out
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("nyllvmc_boundary_dummy");
    std::env::temp_dir().join(format!("{}.{}.mir.json", filename, std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_mir_json_has_entry_function() {
        let text = build_dummy_mir_json();
        assert!(text.contains("\"name\":\"ny_main\""));
        assert!(text.contains("\"schema_version\":\"1.0\""));
    }
}
