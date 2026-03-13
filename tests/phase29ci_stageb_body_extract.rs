use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn write_temp_hako(name: &str, body: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    path.push(format!("{}_{}_{}.hako", name, std::process::id(), stamp));
    fs::write(&path, body).expect("write temp hako");
    path
}

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
    assert!(
        json.contains("\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":0}}"),
        "default Stage-B route should preserve `return 0` as a Return node\njson:\n{}",
        json
    );
    assert!(
        !json.contains("\"name\":\"eturn\""),
        "default Stage-B route must not drop the leading `r` from `return`\njson:\n{}",
        json
    );
}

#[test]
fn build_box_emit_program_json_v0_uses_main_body_for_hello_simple_fixture() {
    let bin = env!("CARGO_BIN_EXE_hakorune");
    let script = r#"
using lang.compiler.build.build_box as BuildBox
static box Main {
  main() {
    local src = env.get("HAKO_SRC")
    local json = BuildBox.emit_program_json_v0(src, null)
    print(json)
    return 0
  }
}
"#;
    let script_path = write_temp_hako("phase29ci_buildbox_emit", script);
    let output = Command::new(bin)
        .arg("--backend")
        .arg("vm")
        .arg(&script_path)
        .env("HAKO_SRC", include_str!("../apps/tests/hello_simple_llvm.hako"))
        .output()
        .expect("failed to run BuildBox probe");
    let _ = fs::remove_file(&script_path);

    assert!(
        output.status.success(),
        "BuildBox probe should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout
        .find("{\"version\":0,\"kind\":\"Program\"")
        .expect("BuildBox output should contain Program(JSON v0)");
    let json = &stdout[json_start..];

    assert!(
        json.contains("\"type\":\"Extern\"") && json.contains("\"value\":42"),
        "BuildBox route should preserve print(42)\njson:\n{}",
        json
    );
    assert!(
        json.contains("\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":0}}"),
        "BuildBox route should preserve return 0 as Return(Int 0)\njson:\n{}",
        json
    );
    assert!(
        !json.contains("\"name\":\"static\"") && !json.contains("\"name\":\"eturn\""),
        "BuildBox route must not leak full-source or truncated-return artifacts\njson:\n{}",
        json
    );
}
