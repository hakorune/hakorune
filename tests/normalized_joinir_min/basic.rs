use super::*;

fn build_structured_pattern1() -> JoinModule {
    let mut module = JoinModule::new();
    let mut loop_fn = JoinFunction::new(
        JoinFuncId::new(1),
        "loop_step".to_string(),
        vec![ValueId(10)],
    );

    loop_fn.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(11),
        value: ConstValue::Integer(0),
    }));
    loop_fn.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: ValueId(12),
        op: BinOpKind::Add,
        lhs: ValueId(10),
        rhs: ValueId(11),
    }));
    loop_fn.body.push(JoinInst::Jump {
        cont: JoinContId(2),
        args: vec![ValueId(12)],
        cond: None, // 単純経路: 無条件で k_exit に渡して終了
    });

    let mut k_exit = JoinFunction::new(JoinFuncId::new(2), "k_exit".to_string(), vec![ValueId(12)]);
    k_exit.body.push(JoinInst::Ret {
        value: Some(ValueId(12)),
    });

    module.entry = Some(loop_fn.id);
    module.add_function(loop_fn);
    module.add_function(k_exit);
    module
}

#[test]
fn normalized_pattern1_minimal_smoke() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let normalized = normalize_pattern1_minimal(&structured);

    assert_eq!(normalized.phase, JoinIrPhase::Normalized);
    assert!(!normalized.env_layouts.is_empty());
    assert!(!normalized.functions.is_empty());

    let restored = normalized
        .to_structured()
        .expect("should retain structured backup");
    assert!(restored.is_structured());
    assert_eq!(restored.functions.len(), structured.functions.len());
}

#[test]
fn normalized_pattern1_roundtrip_structured_equivalent() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let normalized = normalize_pattern1_minimal(&structured);
    let reconstructed = normalized_pattern1_to_structured(&normalized);

    assert!(reconstructed.is_structured());
    assert_eq!(reconstructed.functions.len(), structured.functions.len());

    for (fid, func) in &structured.functions {
        let recon = reconstructed
            .functions
            .get(fid)
            .expect("function missing after reconstruction");
        assert_eq!(recon.params.len(), func.params.len());
    }
}

#[test]
fn normalized_pattern1_exec_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let normalized = normalize_pattern1_minimal(&structured);
    let reconstructed = normalized_pattern1_to_structured(&normalized);

    let entry = structured.entry.unwrap_or(JoinFuncId::new(1));
    let input = [JoinValue::Int(0)];

    let result_structured = run_joinir_vm_bridge(&structured, entry, &input, false);
    let result_norm = run_joinir_vm_bridge(&reconstructed, entry, &input, false);

    assert_eq!(result_structured, result_norm);
}

#[test]
fn normalized_pattern1_exec_matches_structured_roundtrip_backup() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let normalized = normalize_pattern1_minimal(&structured);
    let reconstructed = normalized_pattern1_to_structured(&normalized);
    let restored_backup = normalized
        .to_structured()
        .expect("structured backup should be present");

    let entry = structured.entry.unwrap_or(JoinFuncId::new(1));
    let input = [JoinValue::Int(0)];

    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let recon = run_joinir_vm_bridge(&reconstructed, entry, &input, false);
    let restored = run_joinir_vm_bridge(&restored_backup, entry, &input, false);

    assert_eq!(base, recon);
    assert_eq!(base, restored);
}

#[test]
fn normalized_pattern2_roundtrip_structure() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_minimal_structured();
    let normalized = normalize_pattern2_minimal(&structured);
    assert_eq!(normalized.phase, JoinIrPhase::Normalized);

    let reconstructed = normalized_pattern2_to_structured(&normalized);
    assert!(reconstructed.is_structured());
    assert_eq!(reconstructed.functions.len(), structured.functions.len());

    for name in ["main", "loop_step", "k_exit"] {
        let original_has = structured.functions.values().any(|f| f.name == name);
        let reconstructed_has = reconstructed.functions.values().any(|f| f.name == name);
        assert!(
            original_has && reconstructed_has,
            "expected function '{}' to exist in both modules",
            name
        );
    }
}

#[test]
fn normalized_pattern2_jsonparser_parse_number_real_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_parse_number_real_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [
        ("42", 0, "42"),
        ("123abc", 0, "123"),
        ("9", 0, "9"),
        ("abc", 0, ""),
        ("xx7yy", 2, "7"),
        ("007", 0, "007"),
    ];

    for (s, pos, expected) in cases {
        let input = [JoinValue::Str(s.to_string()), JoinValue::Int(pos)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch for input '{}'", s);
        assert_eq!(
            dev,
            JoinValue::Str(expected.to_string()),
            "unexpected result for input '{}' (pos={}) (expected num_str)",
            s,
            pos
        );
    }
}

#[test]
fn normalized_pattern2_exec_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_minimal_structured();
    let normalized = normalize_pattern2_minimal(&structured);
    let reconstructed = normalized_pattern2_to_structured(&normalized);

    let entry = structured.entry.unwrap_or(JoinFuncId::new(0));
    let input = [JoinValue::Int(0)];

    let base = run_joinir_vm_bridge(&structured, entry, &input, false);
    let recon = run_joinir_vm_bridge(&reconstructed, entry, &input, false);

    assert_eq!(base, recon);
}

#[test]
#[should_panic(expected = "normalize_pattern2_minimal")]
fn normalized_pattern2_rejects_non_pattern2_structured() {
    let _ctx = normalized_dev_test_ctx();
    // Pattern1 Structured module should be rejected by Pattern2 normalizer.
    let structured = build_structured_pattern1();
    let _ = normalize_pattern2_minimal(&structured);
}

#[test]
fn normalized_pattern2_real_loop_roundtrip_structure() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_break_fixture_structured();
    let normalized = normalize_pattern2_minimal(&structured);
    let reconstructed = normalized_pattern2_to_structured(&normalized);

    assert!(reconstructed.is_structured());
    assert_eq!(structured.functions.len(), reconstructed.functions.len());
    assert_eq!(structured.entry, reconstructed.entry);

    let original_names: Vec<_> = structured
        .functions
        .values()
        .map(|f| f.name.clone())
        .collect();
    for name in original_names {
        let reconstructed_has = reconstructed.functions.values().any(|f| f.name == name);
        assert!(
            reconstructed_has,
            "function '{}' missing after roundtrip",
            name
        );
    }
}

#[test]
fn normalized_pattern2_real_loop_exec_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_break_fixture_structured();
    let normalized = normalize_pattern2_minimal(&structured);
    let reconstructed = normalized_pattern2_to_structured(&normalized);

    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let recon = run_joinir_vm_bridge(&reconstructed, entry, &input, false);

        assert_eq!(base, recon, "mismatch at n={}", n);
        let expected_sum = n * (n.saturating_sub(1)) / 2;
        assert_eq!(
            base,
            JoinValue::Int(expected_sum),
            "unexpected loop result at n={}",
            n
        );
    }
}

#[test]
fn normalized_pattern1_runner_dev_switch_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let entry = structured.entry.expect("structured entry required");
    let input = [JoinValue::Int(7)];

    let base = run_joinir_runner(&structured, entry, &input, false);
    let dev = run_joinir_runner(&structured, entry, &input, true);

    assert_eq!(base, dev);
    assert_eq!(base, JoinValue::Int(7));
}

#[test]
fn normalized_pattern2_runner_dev_switch_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_break_fixture_structured();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_runner(&structured, entry, &input, false);
        let dev = run_joinir_runner(&structured, entry, &input, true);

        assert_eq!(base, dev, "runner mismatch at n={}", n);
        let expected_sum = n * (n.saturating_sub(1)) / 2;
        assert_eq!(
            dev,
            JoinValue::Int(expected_sum),
            "runner result mismatch at n={}",
            n
        );
    }
}

#[test]
fn normalized_pattern2_jsonparser_runner_dev_switch_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 2, 5];

    for len in cases {
        let input = [JoinValue::Int(len)];
        let base = run_joinir_runner(&structured, entry, &input, false);
        let dev = run_joinir_runner(&structured, entry, &input, true);

        assert_eq!(base, dev, "runner mismatch at len={}", len);
        assert_eq!(dev, JoinValue::Int(len), "unexpected result at len={}", len);
    }
}

#[test]
fn normalized_pattern2_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_pattern2_break_fixture_structured();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 3, 5];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch at n={}", n);
        let expected_sum = n * (n.saturating_sub(1)) / 2;
        assert_eq!(
            dev,
            JoinValue::Int(expected_sum),
            "vm bridge result mismatch at n={}",
            n
        );
    }
}

#[test]
fn normalized_pattern1_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_structured_pattern1();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 5, 7];

    for n in cases {
        let input = [JoinValue::Int(n)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch at n={}", n);
        assert_eq!(dev, JoinValue::Int(n), "unexpected result at n={}", n);
    }
}

#[test]
fn normalized_pattern2_jsonparser_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_skip_ws_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [0, 1, 2, 5];

    for len in cases {
        let input = [JoinValue::Int(len)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch at len={}", len);
        assert_eq!(dev, JoinValue::Int(len), "unexpected result at len={}", len);
    }
}

#[test]
fn normalized_pattern2_jsonparser_skip_ws_real_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_skip_ws_real_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [
        ("   abc", 0, 3),
        ("abc", 0, 0),
        (" \t\nx", 0, 3),
        (" \t\nx", 2, 3),
    ];

    for (s, pos, expected) in cases {
        let input = [JoinValue::Str(s.to_string()), JoinValue::Int(pos)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch for input '{}'", s);
        assert_eq!(
            dev,
            JoinValue::Int(expected),
            "unexpected result for input '{}' (pos={})",
            s,
            pos
        );
    }
}

#[test]
fn normalized_pattern2_jsonparser_atoi_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_atoi_structured_for_normalized_dev();
    let entry = structured.entry.expect("structured entry required");
    let cases = [("42", 2, 42), ("123abc", 6, 123), ("007", 3, 7), ("", 0, 0)];

    for (s, len, expected) in cases {
        let input = [JoinValue::Str(s.to_string()), JoinValue::Int(len)];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch for input '{}'", s);
        assert_eq!(
            dev,
            JoinValue::Int(expected),
            "unexpected result for input '{}'",
            s
        );
    }
}

#[test]
fn normalized_pattern2_jsonparser_atoi_real_vm_bridge_direct_matches_structured() {
    let _ctx = normalized_dev_test_ctx();
    let structured = build_jsonparser_atoi_real_structured_for_normalized_dev();
    if nyash_rust::config::env::joinir_test_debug_enabled() {
        eprintln!(
            "[joinir/normalized-dev/test] structured jsonparser_atoi_real: {:#?}",
            structured
        );
        let normalized = normalize_pattern2_minimal(&structured);
        eprintln!(
            "[joinir/normalized-dev/test] normalized jsonparser_atoi_real: {:#?}",
            normalized
        );
    }
    let entry = structured.entry.expect("structured entry required");
    let cases = [
        ("42", 42),
        ("123abc", 123),
        ("007", 7),
        ("", 0),
        ("abc", 0),
        ("-42", -42),
        ("+7", 7),
        ("-0", 0),
        ("-12x", -12),
        ("+", 0),
        ("-", 0),
    ];

    for (s, expected) in cases {
        let input = [JoinValue::Str(s.to_string())];
        let base = run_joinir_vm_bridge(&structured, entry, &input, false);
        let dev = run_joinir_vm_bridge(&structured, entry, &input, true);

        assert_eq!(base, dev, "vm bridge mismatch for input '{}'", s);
        assert_eq!(
            dev,
            JoinValue::Int(expected),
            "unexpected result for input '{}'",
            s
        );
    }
}
