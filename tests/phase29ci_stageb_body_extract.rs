use std::process::Command;

#[test]
fn stageb_compiler_no_longer_falls_back_to_full_source_for_hello_simple_fixture() {
    let bin = env!("CARGO_BIN_EXE_hakorune");
    let output = Command::new(bin)
        .arg("--backend")
        .arg("vm")
        .arg("lang/src/compiler/entry/compiler.hako")
        .arg("--")
        .arg("--stage-b")
        .arg("--stage3")
        .env("HAKO_SRC", include_str!("../apps/tests/hello_simple_llvm.hako"))
        .output()
        .expect("failed to run compiler.hako stage-b route");

    assert!(
        output.status.success(),
        "stage-b route should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout
        .find("{\"version\":0,\"kind\":\"Program\"")
        .expect("stage-b output should contain Program(JSON v0)");
    let json = &stdout[json_start..];

    assert!(
        json.contains("\"type\":\"Extern\"") && json.contains("\"value\":42"),
        "expected stage-b route to preserve the extracted print(42) body\njson:\n{}",
        json
    );
    assert!(
        !json.contains("\"name\":\"static\""),
        "full-source fallback should not leak `static box Main` into Program(JSON)\njson:\n{}",
        json
    );
}
