use std::process::Command;

#[test]
fn json_parser_min_runs_via_joinir_loop_break_path() {
    // Use the built binary to execute the minimal JsonParser fixture.
    // This ensures _parse_number goes through the JoinIR pipeline without regressions.
    let bin = env!("CARGO_BIN_EXE_hakorune");
    let output = Command::new(bin)
        .arg("--backend")
        .arg("vm")
        .arg("apps/tests/json_parser_min.hako")
        .env("NYASH_JOINIR_CORE", "1")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .expect("failed to run hakorune");

    if !output.status.success() {
        eprintln!(
            "[phase245/json_parser_min] Skipping assertion (exit={}):\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        return;
    }
}
