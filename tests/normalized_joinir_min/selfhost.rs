use super::*;

#[test]
fn normalized_selfhost_token_scan_p2_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_token_scan_p2_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch for n={}", n);
        assert_eq!(
            dev,
            JoinValue::Int(n),
            "unexpected result for selfhost_token_scan_p2 n={}",
            n
        );
    }
}

#[test]
fn normalized_selfhost_token_scan_p2_accum_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_token_scan_p2_accum_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(
            base, dev,
            "vm bridge mismatch for selfhost_token_scan_p2_accum n={}",
            n
        );
    }
}

#[test]
fn normalized_selfhost_if_sum_p3_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_if_sum_p3_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 4, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(
            base, dev,
            "vm bridge mismatch for selfhost_if_sum_p3 n={}",
            n
        );
        assert_eq!(dev, JoinValue::Int(expected_selfhost_if_sum_p3(n)));
    }
}

#[test]
fn normalized_selfhost_if_sum_p3_ext_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_if_sum_p3_ext_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 4, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(
            base, dev,
            "vm bridge mismatch for selfhost_if_sum_p3_ext n={}",
            n
        );
        assert_eq!(dev, JoinValue::Int(expected_selfhost_if_sum_p3_ext(n)));
    }
}

fn expected_selfhost_if_sum_p3(n: i64) -> i64 {
    if n <= 1 {
        return 0;
    }
    let sum = (n - 1) * n / 2;
    let count = n - 1;
    sum + count
}

fn expected_selfhost_if_sum_p3_ext(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    // i=0: sum += 1
    // i=1..n-1: sum += i, count += 1
    let sum = 1 + (n - 1) * n / 2;
    let count = n - 1;
    sum + count
}

/// Phase 53: selfhost args-parse P2 (practical variation with string carrier)
#[test]
fn normalized_selfhost_args_parse_p2_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_args_parse_p2_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    // Test different argc values: 0, 1, 2, 3
    let cases = [0, 1, 2, 3];

    for argc in cases {
        let input = [JoinValue::Int(argc)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(
            base, dev,
            "vm bridge mismatch for selfhost_args_parse_p2 argc={}",
            argc
        );
    }
}

/// Phase 53: selfhost stmt-count P3 (practical variation with multi-branch if-else)
#[test]
fn normalized_selfhost_stmt_count_p3_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_selfhost_stmt_count_p3_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    // Test different statement counts: 0, 5, 10, 15
    let cases = [0, 5, 10, 15];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(
            base, dev,
            "vm bridge mismatch for selfhost_stmt_count_p3 n={}",
            n
        );
    }
}
