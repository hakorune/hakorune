//! Phase 246-EX Step 5: E2E tests for JsonParser _atoi JoinIR integration
//!
//! Tests the complete _atoi loop lowering using loop_break route + NumberAccumulation.
//!
//! Test cases cover:
//! - Single digit zero: "0" → 0
//! - Two digits: "42" → 42
//! - Multiple digits: "123" → 123
//! - Leading zeros: "007" → 7
//! - Break at non-digit: "123abc" → 123
//! - Immediate break (no digits): "abc" → 0

use std::fs;
use std::process::Command;

/// Helper function to run _atoi implementation via hakorune binary
fn run_atoi_test(input: &str, expected: i64, test_name: &str) {
    let code = format!(
        r#"
static box Main {{
    main() {{
        local result = me._atoi("{}", {})
        print(result)
        return result
    }}

    method _atoi(s, len) {{
        local result = 0
        local digits = "0123456789"
        local i = 0

        loop(i < len) {{
            local ch = s.substring(i, i + 1)
            local digit_pos = digits.indexOf(ch)
            if digit_pos < 0 {{ break }}
            result = result * 10 + digit_pos
            i = i + 1
        }}

        return result
    }}
}}
"#,
        input,
        input.len()
    );

    // Write test file
    let test_file = format!("local_tests/phase246_atoi_{}.hako", test_name);
    fs::write(&test_file, &code).expect("Failed to write test file");

    // Run hakorune
    let bin = env!("CARGO_BIN_EXE_hakorune");
    let output = Command::new(bin)
        .arg("--backend")
        .arg("vm")
        .arg(&test_file)
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_JOINIR_CORE", "1")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .expect("Failed to run hakorune");

    // Clean up test file
    let _ = fs::remove_file(&test_file);

    // Accept non-zero exit codes (program returns parsed value as exit code). Only fail on signal.
    if output.status.code().is_none() {
        panic!(
            "[phase246/atoi/{}] Test failed (terminated by signal?):\nstdout: {}\nstderr: {}",
            test_name,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    // Verify output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let output_value: i64 = stdout
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse output '{}' as integer", stdout.trim()));

    assert_eq!(
        output_value, expected,
        "Failed for input '{}': expected {}, got {}",
        input, expected, output_value
    );
}

#[test]
fn test_atoi_single_digit_zero() {
    // Phase 246-EX: Test case "0" → 0
    run_atoi_test("0", 0, "zero");
}

#[test]
fn test_atoi_two_digits() {
    // Phase 246-EX: Test case "42" → 42
    run_atoi_test("42", 42, "two_digits");
}

#[test]
fn test_atoi_multiple_digits() {
    // Phase 246-EX: Test case "123" → 123
    run_atoi_test("123", 123, "multiple_digits");
}

#[test]
fn test_atoi_leading_zeros() {
    // Phase 246-EX: Test case "007" → 7
    run_atoi_test("007", 7, "leading_zeros");
}

#[test]
fn test_atoi_break_at_non_digit() {
    // Phase 246-EX: Test case "123abc" → 123 (break at 'a')
    run_atoi_test("123abc", 123, "break_non_digit");
}

#[test]
fn test_atoi_immediate_break_no_digits() {
    // Phase 246-EX: Test case "abc" → 0 (immediate break, no digits parsed)
    run_atoi_test("abc", 0, "immediate_break");
}

#[test]
fn test_atoi_empty_string() {
    // Phase 246-EX: Test case "" → 0 (empty string, no iterations)
    run_atoi_test("", 0, "empty");
}

#[test]
fn test_atoi_single_digit_nine() {
    // Phase 246-EX: Additional test case "9" → 9 (max single digit)
    run_atoi_test("9", 9, "nine");
}

#[test]
fn test_atoi_large_number() {
    // Phase 246-EX: Test case "999999" → 999999 (large number)
    run_atoi_test("999999", 999999, "large");
}
