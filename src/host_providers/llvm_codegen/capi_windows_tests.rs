use super::compile_via_capi_keep;
use crate::host_providers::llvm_codegen::boundary_default_object_opts;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn native_c_compiler() -> PathBuf {
    ["gcc", "clang", "cc"]
        .into_iter()
        .find_map(|candidate| {
            let status = Command::new(candidate).arg("--version").status().ok()?;
            status
                .success()
                .then(|| which::which(candidate).unwrap_or_else(|_| PathBuf::from(candidate)))
        })
        .expect("native C compiler (gcc, clang, or cc) is required")
}

fn build_input_observer_library(path: &Path) -> PathBuf {
    let source =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lang/c-abi/tests/capi_input_observer.c");
    assert!(source.is_file(), "missing CAPI input observer source");
    crate::test_support::with_process_state_lock(|| {
        let compiler = native_c_compiler();
        let status = Command::new(&compiler)
            .args(["-shared", "-o"])
            .arg(path)
            .arg(&source)
            .status()
            .expect("invoke native C compiler for CAPI input observer");
        assert!(status.success(), "CAPI input observer build failed");
        compiler
    })
}

fn capi_opts(out: &Path) -> crate::host_providers::llvm_codegen::Opts {
    boundary_default_object_opts(Some(out.to_path_buf()), None, Some("0".into()), None)
}

fn run_with_observer_env<R>(
    observer: &Path,
    record: &Path,
    mode: &str,
    runtime_dir: &Path,
    f: impl FnOnce() -> R,
) -> R {
    let observer = observer.to_string_lossy().into_owned();
    let record = record.to_string_lossy().into_owned();
    let inherited_path = std::env::var_os("PATH").unwrap_or_default();
    let mut search_path = vec![runtime_dir.to_path_buf()];
    search_path.extend(std::env::split_paths(&inherited_path));
    let search_path = std::env::join_paths(search_path)
        .expect("native C runtime directory should form a valid PATH")
        .to_string_lossy()
        .into_owned();
    crate::test_support::with_env_vars(
        &[
            ("HAKO_AOT_FFI_LIB", Some(observer.as_str())),
            ("HAKO_CAPI_RECORD_PATH", Some(record.as_str())),
            ("HAKO_CAPI_OBSERVER_MODE", Some(mode)),
            ("HAKO_LLVM_OPT_LEVEL", Some("0")),
            ("NYASH_LLVM_OPT_LEVEL", None),
            ("NYASH_NY_LLVM_OPT_TOOL", None),
            ("NYASH_NY_LLVM_LLC_TOOL", None),
            ("NYASH_NY_LLVM_LLC_FLAGS", None),
            ("PATH", Some(search_path.as_str())),
        ],
        f,
    )
}

fn assert_observer_consumed_and_dropped(record: &Path, mir_json: &str) {
    let input_path = fs::read_to_string(record.with_extension("path"))
        .expect("CAPI observer recorded input path");
    assert!(!input_path.is_empty());
    assert_eq!(
        fs::read(record.with_extension("input")).expect("CAPI observer copied input"),
        mir_json.as_bytes()
    );
    assert!(!Path::new(&input_path).exists());
}

#[test]
fn capi_windows_wrapper_observes_input_lifetime_on_success_and_failure() {
    let workspace = tempfile::Builder::new()
        .prefix("hako capi windows ")
        .tempdir()
        .expect("CAPI Windows integration tempdir");
    let observer = workspace.path().join("input observer.dll");
    let compiler = build_input_observer_library(&observer);
    let runtime_dir = compiler
        .parent()
        .expect("native C compiler should have a parent directory")
        .to_path_buf();
    let mir_json = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"),
    )
    .expect("read CAPI MIR fixture");

    let success_record = workspace.path().join("success record");
    let success_output = workspace.path().join("success output.o");
    run_with_observer_env(&observer, &success_record, "success", &runtime_dir, || {
        let result = compile_via_capi_keep(&mir_json, &capi_opts(&success_output))
            .expect("CAPI observer success should return an object");
        assert_eq!(result, success_output);
        assert_eq!(
            fs::read(&success_output).expect("read CAPI observer object"),
            b"capi-observer-object"
        );
    });
    assert_observer_consumed_and_dropped(&success_record, &mir_json);

    let failure_record = workspace.path().join("failure record");
    let failure_output = workspace.path().join("failure output.o");
    run_with_observer_env(&observer, &failure_record, "fail", &runtime_dir, || {
        let error = compile_via_capi_keep(&mir_json, &capi_opts(&failure_output))
            .expect_err("CAPI observer failure should return an error");
        assert!(error.contains("compile failed"), "{error}");
        assert!(!failure_output.exists());
    });
    assert_observer_consumed_and_dropped(&failure_record, &mir_json);
}
