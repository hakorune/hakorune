//! Dynamic ownership is settled before ordinary Completion diagnostics.
use super::{issue, NormalCallableSemanticPackageIssueV1 as Issue};

fn scan(name: &str) -> String {
    format!("{name}(src, pos: i64, end: i64, pred_chars): i64 {{
        local i = pos
        loop(i < end) {{
            local ch = src.substring(i, i + 1)
            if pred_chars.indexOf(ch) < 0 {{ return i }}
            i = i + 1
        }}
        return i
    }}")
}

#[test]
fn dynamic_owner_error_precedes_ordinary_completion_error() {
    let duplicate = format!("static box Scans {{ {} {} }}", scan("first"), scan("second"));
    let unsupported_result = "static box ResultBox { run(): bool { return true } }";
    assert!(matches!(issue(&duplicate), Err(Issue::DuplicateDynamicCandidate)));
    assert!(matches!(issue(unsupported_result), Err(Issue::PhysicalHeader { .. })));
    assert!(matches!(issue(&format!("{unsupported_result}\n{duplicate}")),
        Err(Issue::DuplicateDynamicCandidate)));
}
