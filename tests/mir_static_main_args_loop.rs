use std::fs;
/// Bug A investigation: main(args) causes loops not to execute
/// This test reproduces the issue where adding a parameter to main()
/// causes the loop body to never execute (RC=0 instead of RC=3)
use std::process::Command;

#[test]
fn mir_static_main_no_args_loop() {
    // Working case: main() → RC=3
    let source = r#"
static box Main {
  main() {
    local i = 0
    local count = 0
    loop(i < 3) {
      count = count + 1
      i = i + 1
    }
    return count
  }
}
"#;

    let temp_file = "/tmp/mir_test_no_args.hako";
    fs::write(temp_file, source).expect("Failed to write test file");

    let output = Command::new("./target/release/hakorune")
        .arg("--backend")
        .arg("vm")
        .arg(temp_file)
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .expect("Failed to execute hakorune");

    fs::remove_file(temp_file).ok();

    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 3, "Expected RC=3 for main() with loop");
}

#[test]
fn mir_static_main_with_args_loop() {
    // Broken case: main(args) → RC=0 (BUG: should be 3)
    let source = r#"
static box Main {
  main(args) {
    local i = 0
    local count = 0
    loop(i < 3) {
      count = count + 1
      i = i + 1
    }
    return count
  }
}
"#;

    let temp_file = "/tmp/mir_test_with_args.hako";
    fs::write(temp_file, source).expect("Failed to write test file");

    let output = Command::new("./target/release/hakorune")
        .arg("--backend")
        .arg("vm")
        .arg(temp_file)
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .expect("Failed to execute hakorune");

    fs::remove_file(temp_file).ok();

    let exit_code = output.status.code().unwrap_or(-1);
    // This will FAIL due to the bug - loop doesn't execute
    assert_eq!(
        exit_code, 3,
        "Expected RC=3 for main(args) with loop (BUG: currently returns 0)"
    );
}

#[test]
fn mir_static_main_args_without_loop() {
    // Sanity check: main(args) works WITHOUT loop
    let source = r#"
static box Main {
  main(args) {
    return 42
  }
}
"#;

    let temp_file = "/tmp/mir_test_args_no_loop.hako";
    fs::write(temp_file, source).expect("Failed to write test file");

    let output = Command::new("./target/release/hakorune")
        .arg("--backend")
        .arg("vm")
        .arg(temp_file)
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .expect("Failed to execute hakorune");

    fs::remove_file(temp_file).ok();

    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 42, "Expected RC=42 for main(args) without loop");
}
