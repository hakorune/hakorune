use std::fs;
use std::io::Write;
use std::path::Path;

/// Invocation-owned MIR JSON input.
///
/// The directory is kept alive for as long as the consumer borrows `path`.
/// Keeping the owner private prevents callers from accidentally sharing or
/// deleting another invocation's input.
pub(super) struct BackendInputJsonFile {
    _directory: tempfile::TempDir,
    file: tempfile::TempPath,
}

impl BackendInputJsonFile {
    pub(super) fn path(&self) -> &Path {
        self.file.as_ref()
    }
}

pub(super) fn prepare_backend_input_json_file(
    mir_json: &str,
) -> Result<BackendInputJsonFile, String> {
    // Create the owner before opening the file so every write failure also
    // drops the invocation's private directory.
    let directory =
        tempfile::tempdir().map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    let mut f = tempfile::Builder::new()
        .prefix("hako_llvm_in_")
        .suffix(".json")
        .tempfile_in(directory.path())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    f.write_all(mir_json.as_bytes())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    Ok(BackendInputJsonFile {
        _directory: directory,
        file: f.into_temp_path(),
    })
}

pub(super) fn write_backend_text_file(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|e| {
        format!(
            "[llvmemit/tmp/write-failed] path={} error={}",
            path.display(),
            e
        )
    })
}

pub(super) fn remove_backend_temp_file(path: &Path) {
    let _ = fs::remove_file(path);
}

pub(super) fn ensure_backend_output_parent(out_path: &Path) {
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

pub(super) fn ensure_backend_artifact_written(path: &Path, kind: &str) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    Err(format!("{} not produced", kind))
}

#[cfg(test)]
mod tests {
    use super::{ensure_backend_artifact_written, prepare_backend_input_json_file};
    use std::fs;

    #[test]
    fn invocation_inputs_are_unique_and_drop_independently() {
        let first = prepare_backend_input_json_file("{\"id\":1}").unwrap();
        let second = prepare_backend_input_json_file("{\"id\":2}").unwrap();
        assert_ne!(first.path(), second.path());
        assert_eq!(fs::read(first.path()).unwrap(), b"{\"id\":1}");
        assert_eq!(fs::read(second.path()).unwrap(), b"{\"id\":2}");

        let second_path = second.path().to_path_buf();
        drop(first);
        assert_eq!(fs::read(&second_path).unwrap(), b"{\"id\":2}");
        drop(second);
        assert!(!second_path.exists());
    }

    #[test]
    fn input_survives_consumer_error_until_owner_drop() {
        let input = prepare_backend_input_json_file("{\"id\":3}").unwrap();
        let input_path = input.path().to_path_buf();
        let output_dir = tempfile::tempdir().unwrap();
        let result =
            ensure_backend_artifact_written(&output_dir.path().join("missing.o"), "object");
        assert!(result.is_err());
        assert_eq!(fs::read(&input_path).unwrap(), b"{\"id\":3}");
        drop(input);
        assert!(!input_path.exists());
    }
}
