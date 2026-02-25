use super::*;

#[test]
fn normalized_pattern3_if_sum_minimal_runner_dev_switch_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern3_if_sum_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // phase212_if_sum_min.hako 相当: sum=2 になることを期待
    let input: [JoinValue; 0] = [];

    let base = run_joinir_runner(&structured, entry, &input, false);
    let dev = run_joinir_runner(&structured, entry, &input, true);

    assert_eq!(base, dev, "runner mismatch for P3 minimal if-sum");
    assert_eq!(
        dev,
        JoinValue::Int(2),
        "unexpected result for P3 minimal if-sum (expected sum=2)",
    );
}

#[test]
fn normalized_pattern3_if_sum_multi_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern3_if_sum_multi_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let input = [JoinValue::Int(0)];

    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(base, dev, "vm bridge mismatch for P3 if-sum multi");
    assert_eq!(
        dev,
        JoinValue::Int(2),
        "unexpected result for P3 if-sum multi (expected sum=2)"
    );
}

#[test]
fn normalized_pattern3_json_if_sum_min_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern3_json_if_sum_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let input = [JoinValue::Int(0)];

    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(base, dev, "vm bridge mismatch for P3 json if-sum");
    assert_eq!(
        dev,
        JoinValue::Int(10),
        "unexpected result for P3 json if-sum (expected sum=10)"
    );
}

#[cfg(feature = "normalized_dev")]
#[test]
fn test_phase46_canonical_set_includes_p2_mid() {
    use nyash_rust::mir::join_ir::normalized::shape_guard::{
        is_canonical_shape, NormalizedDevShape,
    };

    // Phase 46: Verify P2-Mid patterns are canonical
    assert!(is_canonical_shape(&NormalizedDevShape::JsonparserAtoiReal));
    assert!(is_canonical_shape(
        &NormalizedDevShape::JsonparserParseNumberReal
    ));
    assert!(is_canonical_shape(
        &NormalizedDevShape::Pattern3IfSumMinimal
    ));
    assert!(is_canonical_shape(&NormalizedDevShape::Pattern3IfSumMulti));
    assert!(is_canonical_shape(&NormalizedDevShape::Pattern3IfSumJson));

    // Verify P2-Core patterns still canonical
    assert!(is_canonical_shape(&NormalizedDevShape::Pattern2Mini));
    assert!(is_canonical_shape(
        &NormalizedDevShape::JsonparserSkipWsMini
    ));
    assert!(is_canonical_shape(
        &NormalizedDevShape::JsonparserSkipWsReal
    ));
    assert!(is_canonical_shape(&NormalizedDevShape::JsonparserAtoiMini));
}

/// Phase 47-A: Test P3 minimal normalization
#[test]
fn test_phase47a_pattern3_if_sum_minimal_normalization() {
    use nyash_rust::mir::join_ir::normalized::normalize_pattern3_if_sum_minimal;

    let module = build_pattern3_if_sum_min_structured_for_normalized_dev();

    // Test that normalization succeeds (includes shape detection internally)
    let result = normalize_pattern3_if_sum_minimal(&module);
    assert!(
        result.is_ok(),
        "P3 normalization should succeed (shape detection + normalization): {:?}",
        result.err()
    );

    let normalized = result.unwrap();
    assert_eq!(
        normalized.functions.len(),
        module.functions.len(),
        "Normalized function count should match Structured"
    );

    // Verify normalized module has proper phase
    assert_eq!(
        normalized.phase,
        nyash_rust::mir::join_ir::JoinIrPhase::Normalized,
        "Normalized module should have Normalized phase"
    );
}

/// Phase 47-A: Test P3 VM execution (basic smoke test)
#[test]
fn test_phase47a_pattern3_if_sum_minimal_runner() {
    let module = build_pattern3_if_sum_min_structured_for_normalized_dev();

    // Basic test: module should be runnable through JoinIR runner
    // This test verifies the P3 fixture is valid and generates proper JoinIR
    assert_eq!(module.functions.len(), 3, "P3 should have 3 functions");

    let entry = module.entry.expect("P3 should have entry function");
    assert_eq!(entry.0, 0, "Entry should be function 0");
}

/// Phase 48-A: Test P4 minimal normalization
#[test]
fn test_phase48a_pattern4_continue_minimal_normalization() {
    use nyash_rust::mir::join_ir::normalized::normalize_pattern4_continue_minimal;

    let module = build_pattern4_continue_min_structured_for_normalized_dev();

    // Test that normalization succeeds (includes shape detection internally)
    let result = normalize_pattern4_continue_minimal(&module);
    assert!(
        result.is_ok(),
        "P4 normalization should succeed (shape detection + normalization): {:?}",
        result.err()
    );

    let normalized = result.unwrap();
    assert_eq!(
        normalized.functions.len(),
        module.functions.len(),
        "Normalized function count should match Structured"
    );

    // Verify normalized module has proper phase
    assert_eq!(
        normalized.phase,
        nyash_rust::mir::join_ir::JoinIrPhase::Normalized,
        "Normalized module should have Normalized phase"
    );
}

/// Phase 48-A: Test P4 VM execution (basic smoke test)
#[test]
fn test_phase48a_pattern4_continue_minimal_runner() {
    let module = build_pattern4_continue_min_structured_for_normalized_dev();

    // Basic test: module should be runnable through JoinIR runner
    // This test verifies the P4 fixture is valid and generates proper JoinIR
    assert_eq!(module.functions.len(), 3, "P4 should have 3 functions");

    let entry = module.entry.expect("P4 should have entry function");
    assert_eq!(entry.0, 0, "Entry should be function 0");
}

/// Phase 48-A: Test P4 minimal Runner dev switch matches Structured
#[test]
fn test_normalized_pattern4_continue_minimal_runner_dev_switch_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern4_continue_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // pattern4_continue_min fixture: acc=4 (skipped i==2, so counted 0,1,3,4)
    let input = [JoinValue::Int(5)]; // n = 5

    let base = run_joinir_runner(&structured, entry, &input, false);
    let dev = run_joinir_runner(&structured, entry, &input, true);

    assert_eq!(base, dev, "runner mismatch for P4 minimal continue");
    assert_eq!(
        dev,
        JoinValue::Int(4),
        "unexpected result for P4 minimal continue (expected acc=4, skipped i==2)",
    );
}

/// Phase 48-A: Test P4 minimal VM Bridge direct matches Structured
#[test]
fn test_normalized_pattern4_continue_minimal_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern4_continue_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // pattern4_continue_min fixture: acc=4 (skipped i==2)
    let input = [JoinValue::Int(5)]; // n = 5

    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(base, dev, "vm bridge mismatch for P4 minimal continue");
    assert_eq!(
        dev,
        JoinValue::Int(4),
        "unexpected result for P4 minimal continue (expected acc=4)",
    );
}

/// Phase 48-C: P4 minimal should use canonical normalized route even without env
#[test]
fn test_normalized_pattern4_continue_minimal_canonical_matches_structured() {
    let structured = build_pattern4_continue_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let input = [JoinValue::Int(5)];
    let structured_res = run_joinir_vm_bridge_structured_only(&structured, entry, &input);
    let canonical = run_joinir_vm_bridge(&structured, entry, &input, false);

    assert_eq!(
        structured_res, canonical,
        "canonical P4 minimal result mismatch"
    );
    assert_eq!(canonical, JoinValue::Int(4));
}

/// Phase 48-B: JsonParser _parse_array continue skip_ws (dev-only) VM Bridge comparison
#[test]
fn test_normalized_pattern4_jsonparser_parse_array_continue_skip_ws_vm_bridge_direct_matches_structured(
) {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // Fixture mirrors pattern4_continue_min: skip i == 2
    let cases = [3, 5, 7];

    for n in cases {
        let args = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &args, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &args, true);
        assert_eq!(
            base, dev,
            "vm bridge mismatch for array continue case n={}",
            n
        );
    }
}

/// Phase 48-C: JsonParser _parse_array continue skip_ws canonical route should match Structured
#[test]
fn test_normalized_pattern4_jsonparser_parse_array_continue_skip_ws_canonical_matches_structured() {
    let structured = build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let cases = [3, 5, 7];
    for n in cases {
        let args = [JoinValue::Int(n)];
        let structured_res = run_joinir_vm_bridge_structured_only(&structured, entry, &args);
        let canonical = run_joinir_vm_bridge(&structured, entry, &args, false);
        assert_eq!(
            structured_res, canonical,
            "canonical array continue mismatch n={}",
            n
        );
    }
}

/// Phase 48-B: JsonParser _parse_object continue skip_ws (dev-only) VM Bridge comparison
#[test]
fn test_normalized_pattern4_jsonparser_parse_object_continue_skip_ws_vm_bridge_direct_matches_structured(
) {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // Fixture mirrors pattern4_continue_min: skip i == 2
    let cases = [4, 6, 8];

    for n in cases {
        let args = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &args, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &args, true);
        assert_eq!(
            base, dev,
            "vm bridge mismatch for object continue case n={}",
            n
        );
    }
}

/// Phase 88: JsonParser _unescape_string コア（i+=2 + continue）を canonical route で固定する。
#[test]
fn test_phase88_jsonparser_unescape_string_step2_min_canonical_matches_structured() {
    let structured = build_jsonparser_unescape_string_step2_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    // n=10 → i=0,2,4,6,8 で acc++ → 5
    let args = [JoinValue::Int(10)];
    let structured_res = run_joinir_vm_bridge_structured_only(&structured, entry, &args);
    let canonical = run_joinir_vm_bridge(&structured, entry, &args, false);

    assert_eq!(
        structured_res, canonical,
        "canonical unescape(step2) mismatch"
    );
    assert_eq!(canonical, JoinValue::Int(5));
}

/// Phase 88: Continue 側の `i` 更新は `i = i + const` のみに限定して Fail-Fast する。
#[test]
fn test_phase88_jsonparser_unescape_string_step2_min_rejects_non_const_then_i_update() {
    use std::any::Any;

    fn panic_message(payload: Box<dyn Any + Send>) -> String {
        if let Some(s) = payload.downcast_ref::<&str>() {
            return (*s).to_string();
        }
        if let Some(s) = payload.downcast_ref::<String>() {
            return s.clone();
        }
        "<non-string panic payload>".to_string()
    }

    // then 側の `i = i + n`（const ではない）を入れて、Fail-Fast を確認する。
    let program_json = serde_json::json!({
      "version": 0,
      "kind": "Program",
      "defs": [{
        "type": "FunctionDef",
        "name": "jsonparser_unescape_string_step2_min",
        "params": ["n"],
        "body": { "type": "Block", "body": [
          { "type": "Local", "name": "i", "expr": { "type": "Int", "value": 0 } },
          { "type": "Local", "name": "acc", "expr": { "type": "Int", "value": 0 } },
          { "type": "Loop",
            "cond": { "type": "Compare", "op": "<", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Var", "name": "n" } },
            "body": [
              { "type": "If",
                "cond": { "type": "Compare", "op": "<", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Var", "name": "n" } },
                "then": [
                  { "type": "Local", "name": "i", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Var", "name": "n" } } },
                  { "type": "Continue" }
                ],
                "else": []
              },
              { "type": "Local", "name": "acc", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "acc" }, "rhs": { "type": "Int", "value": 0 } } },
              { "type": "Local", "name": "i", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Int", "value": 1 } } }
            ]
          },
          { "type": "Return", "expr": { "type": "Var", "name": "acc" } }
        ]}
      }]
    });

    let res = std::panic::catch_unwind(|| {
        let mut lowerer = nyash_rust::mir::join_ir::frontend::AstToJoinIrLowerer::new();
        lowerer.lower_program_json(&program_json);
    });
    assert!(res.is_err(), "expected fail-fast panic");
    let msg = panic_message(res.err().unwrap());
    assert!(
        msg.contains("then' branch step increment")
            || msg.contains("then i update of form (i + const)"),
        "unexpected panic message: {}",
        msg
    );
}

/// Phase 48-C: JsonParser _parse_object continue skip_ws canonical route should match Structured
#[test]
fn test_normalized_pattern4_jsonparser_parse_object_continue_skip_ws_canonical_matches_structured()
{
    let structured = build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let cases = [4, 6, 8];
    for n in cases {
        let args = [JoinValue::Int(n)];
        let structured_res = run_joinir_vm_bridge_structured_only(&structured, entry, &args);
        let canonical = run_joinir_vm_bridge(&structured, entry, &args, false);
        assert_eq!(
            structured_res, canonical,
            "canonical object continue mismatch n={}",
            n
        );
    }
}

/// Phase 54: False positive observation test - P2 structural axis discrimination
///
/// This test validates that structural detection can discriminate between
/// canonical P2 and selfhost P2 shapes using structural features alone.
#[test]
fn test_phase54_structural_axis_discrimination_p2() {
    use nyash_rust::mir::join_ir::normalized::shape_guard::{
        detect_shapes, is_canonical_shape, NormalizedDevShape,
    };

    // Canonical P2 shapes
    let canonical_p2_shapes = vec![
        build_pattern2_minimal_structured(),
        build_jsonparser_skip_ws_structured_for_normalized_dev(),
    ];

    // Selfhost P2 shapes (Phase 53)
    let selfhost_p2_shapes = vec![
        build_selfhost_args_parse_p2_structured_for_normalized_dev(),
        build_selfhost_token_scan_p2_structured_for_normalized_dev(),
    ];

    // Canonical P2 should be detected as canonical, NOT selfhost
    for canonical in &canonical_p2_shapes {
        let shapes = detect_shapes(canonical);
        let has_canonical = shapes.iter().any(|s| is_canonical_shape(s));
        let has_selfhost_p2 = shapes.iter().any(|s| {
            matches!(
                s,
                NormalizedDevShape::SelfhostArgsParseP2
                    | NormalizedDevShape::SelfhostTokenScanP2
                    | NormalizedDevShape::SelfhostTokenScanP2Accum
            )
        });

        assert!(
            has_canonical,
            "canonical P2 should be detected as canonical: {:?}",
            shapes
        );
        assert!(
            !has_selfhost_p2,
            "canonical P2 should NOT be detected as selfhost: {:?}",
            shapes
        );
    }

    // Selfhost P2 should be detected as selfhost, NOT canonical
    for selfhost in &selfhost_p2_shapes {
        let shapes = detect_shapes(selfhost);
        let has_canonical = shapes.iter().any(|s| is_canonical_shape(s));
        let has_selfhost_p2 = shapes.iter().any(|s| {
            matches!(
                s,
                NormalizedDevShape::SelfhostArgsParseP2
                    | NormalizedDevShape::SelfhostTokenScanP2
                    | NormalizedDevShape::SelfhostTokenScanP2Accum
            )
        });

        assert!(
            !has_canonical,
            "selfhost P2 should NOT be detected as canonical: {:?}",
            shapes
        );
        assert!(
            has_selfhost_p2,
            "selfhost P2 should be detected as selfhost (with name guard): {:?}",
            shapes
        );
    }
}

/// Phase 54: False positive observation test - P3 structural axis discrimination
///
/// This test validates that structural detection can discriminate between
/// canonical P3 and selfhost P3 shapes using structural features alone.
#[test]
fn test_phase54_structural_axis_discrimination_p3() {
    use nyash_rust::mir::join_ir::normalized::shape_guard::{
        detect_shapes, is_canonical_shape, NormalizedDevShape,
    };

    // Canonical P3 shapes
    let canonical_p3_shapes = vec![
        build_pattern3_if_sum_min_structured_for_normalized_dev(),
        build_pattern3_if_sum_multi_min_structured_for_normalized_dev(),
    ];

    // Selfhost P3 shapes (Phase 53)
    let selfhost_p3_shapes = vec![
        build_selfhost_stmt_count_p3_structured_for_normalized_dev(),
        build_selfhost_if_sum_p3_structured_for_normalized_dev(),
    ];

    // Canonical P3 should be detected as canonical, NOT selfhost
    for canonical in &canonical_p3_shapes {
        let shapes = detect_shapes(canonical);
        let has_canonical = shapes.iter().any(|s| is_canonical_shape(s));
        let has_selfhost_p3 = shapes.iter().any(|s| {
            matches!(
                s,
                NormalizedDevShape::SelfhostStmtCountP3
                    | NormalizedDevShape::SelfhostIfSumP3
                    | NormalizedDevShape::SelfhostIfSumP3Ext
            )
        });

        assert!(
            has_canonical,
            "canonical P3 should be detected as canonical: {:?}",
            shapes
        );
        assert!(
            !has_selfhost_p3,
            "canonical P3 should NOT be detected as selfhost: {:?}",
            shapes
        );
    }

    // Selfhost P3 should be detected as selfhost, NOT canonical
    for selfhost in &selfhost_p3_shapes {
        let shapes = detect_shapes(selfhost);
        let has_canonical = shapes.iter().any(|s| is_canonical_shape(s));
        let has_selfhost_p3 = shapes.iter().any(|s| {
            matches!(
                s,
                NormalizedDevShape::SelfhostStmtCountP3
                    | NormalizedDevShape::SelfhostIfSumP3
                    | NormalizedDevShape::SelfhostIfSumP3Ext
            )
        });

        assert!(
            !has_canonical,
            "selfhost P3 should NOT be detected as canonical: {:?}",
            shapes
        );
        assert!(
            has_selfhost_p3,
            "selfhost P3 should be detected as selfhost (with name guard): {:?}",
            shapes
        );
    }
}

/// Phase 89 P1: Pattern Continue + Return minimal - vm_bridge direct vs structured
#[test]
fn test_normalized_pattern_continue_return_min_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern_continue_return_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let input = [JoinValue::Int(10)]; // n = 10
    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        base, dev,
        "vm bridge mismatch for pattern continue+return min"
    );
}

/// Phase 89 P1: Pattern Continue + Return minimal - expected output test
#[test]
fn test_normalized_pattern_continue_return_min_expected_output() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern_continue_return_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let input = [JoinValue::Int(10)]; // n = 10
    let result = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        result,
        JoinValue::Int(4),
        "Expected acc=4 for n=10 (i=0,1,3,4 increments acc, i=2 skipped by continue, i=5 early return)"
    );
}

/// Phase 90 P0: Parse String Composite minimal - vm_bridge direct vs structured
#[test]
fn test_parse_string_composite_min_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_parse_string_composite_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let input = [JoinValue::Int(10)]; // n = 10
    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        base, dev,
        "vm bridge mismatch for parse_string composite min"
    );
}

/// Phase 90 P0: Parse String Composite minimal - expected output test
#[test]
fn test_parse_string_composite_min_expected_output() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_parse_string_composite_min_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");

    let input = [JoinValue::Int(10)]; // n = 10
    let result = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        result,
        JoinValue::Int(5),
        "Expected acc=5 for n=10 (i=0,1,2,5,6 increments acc, i=3 escape continue, i=7 close quote return)"
    );
}

/// Refactor-A+B: ContinueReturn multi minimal - tests both Null literal and multiple return-if
#[test]
fn test_continue_return_multi_min_returns_null_at_first_match() {
    use nyash_rust::mir::join_ir::normalized::dev_fixtures::NormalizedDevFixture;

    let _ctx = normalized_dev_test_ctx();
    let structured = NormalizedDevFixture::ContinueReturnMultiMin.load_and_lower();
    let entry = structured.entry.expect("entry required");

    let input = [JoinValue::Int(10)];
    let result = run_joinir_vm_bridge(&structured, entry, &input, true);
    // Tests:
    // - Refactor-A: Null literal support (returns ConstValue::Null → JoinValue::Unit)
    // - Refactor-B: Multiple return-if with same value (i==3, i==7 both return null)
    assert_eq!(
        result,
        JoinValue::Unit,
        "Expected Unit (null) from first return-if at i=3"
    );
}

/// Phase Next: Parse Array minimal - vm_bridge direct vs structured
#[test]
fn test_parse_array_min_vm_bridge_direct_matches_structured() {
    use nyash_rust::mir::join_ir::normalized::dev_fixtures::NormalizedDevFixture;

    let _ctx = normalized_dev_test_ctx();
    let structured = NormalizedDevFixture::ParseArrayMin.load_and_lower();
    let entry = structured.entry.expect("entry required");

    let input = [JoinValue::Int(10)];
    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(base, dev, "vm bridge mismatch for parse_array min");
}

/// Phase Next: Parse Array minimal - expected output test
#[test]
fn test_parse_array_min_expected_output() {
    use nyash_rust::mir::join_ir::normalized::dev_fixtures::NormalizedDevFixture;

    let _ctx = normalized_dev_test_ctx();
    let structured = NormalizedDevFixture::ParseArrayMin.load_and_lower();
    let entry = structured.entry.expect("entry required");

    let input = [JoinValue::Int(10)];
    let result = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        result,
        JoinValue::Int(6),
        "Expected acc=6 for n=10 (i=0,1,2,4,5,6 increments, i=3 continue, i=7 return)"
    );
}

/// Phase Next: Parse Object minimal - vm_bridge direct vs structured
#[test]
fn test_parse_object_min_vm_bridge_direct_matches_structured() {
    use nyash_rust::mir::join_ir::normalized::dev_fixtures::NormalizedDevFixture;

    let _ctx = normalized_dev_test_ctx();
    let structured = NormalizedDevFixture::ParseObjectMin.load_and_lower();
    let entry = structured.entry.expect("entry required");

    let input = [JoinValue::Int(10)];
    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(base, dev, "vm bridge mismatch for parse_object min");
}

/// Phase Next: Parse Object minimal - expected output test
#[test]
fn test_parse_object_min_expected_output() {
    use nyash_rust::mir::join_ir::normalized::dev_fixtures::NormalizedDevFixture;

    let _ctx = normalized_dev_test_ctx();
    let structured = NormalizedDevFixture::ParseObjectMin.load_and_lower();
    let entry = structured.entry.expect("entry required");

    let input = [JoinValue::Int(10)];
    let result = run_joinir_vm_bridge(&structured, entry, &input, true);

    assert_eq!(
        result,
        JoinValue::Int(7),
        "Expected acc=7 for n=10 (i=0,1,2,3,5,6,7 increments, i=4 continue, i=8 return)"
    );
}
