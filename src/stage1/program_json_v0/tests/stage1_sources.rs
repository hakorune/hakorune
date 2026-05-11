use super::*;

#[test]
fn source_to_program_json_v0_compiler_stageb_main_supported() {
    let source = include_str!("../../../../lang/src/compiler/entry/compiler_stageb.hako");
    let json = source_to_program_json_v0_strict(source).expect("program json");
    assert!(json.contains("\"kind\":\"Program\""));
    assert!(json.contains("\"body\""));
}

#[test]
fn source_to_program_json_v0_emits_helper_defs_for_main_box_methods() {
    let source = r#"
static box Main {
  main() {
return me.helper(41)
  }

  method helper(x: usize): i64 {
return x + 1
  }
}
"#;
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("defs array");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0]["name"], "helper");
    assert_eq!(defs[0]["box"], "Main");
    assert_eq!(defs[0]["params"], serde_json::json!(["x"]));
    assert_eq!(
        defs[0]["param_decls"],
        serde_json::json!([{"name": "x", "declared_type": "usize"}])
    );
    assert_eq!(defs[0]["return_type"], "i64");
    assert_eq!(defs[0]["body"]["kind"], "Program");
    assert!(defs[0]["body"]["body"].is_array());
}

#[test]
fn source_to_program_json_v0_accepts_stage1_cli_env_source() {
    let source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["kind"], "Program");
    assert_eq!(value["version"], 0);
    assert_eq!(
        value["imports"]["BuildBox"],
        "lang.compiler.build.build_box"
    );
    assert_eq!(
        value["imports"]["MirBuilderBox"],
        "lang.mir.builder.MirBuilderBox"
    );
}

#[test]
fn source_to_program_json_v0_does_not_widen_with_rune_attrs() {
    with_features(Some("rune"), || {
        let source = r#"
@rune Public
static box Main {
  @rune Ownership(owned)
  main() {
return 0
  }
}
"#;
        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert!(
            value.get("attrs").is_none(),
            "Program(JSON v0) must not widen with Rune attrs"
        );
    });
}

#[test]
fn source_to_program_json_v0_stage1_cli_env_materializes_same_file_stage1_boxes() {
    let source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("defs array");
    let boxes = defs
        .iter()
        .filter_map(|def| def["box"].as_str())
        .collect::<BTreeSet<_>>();

    assert!(
        boxes.contains("Stage1InputContractBox"),
        "stage1_cli_env should carry input-contract helper defs"
    );
    assert!(
        boxes.contains("Stage1SourceMirAuthorityBox"),
        "stage1_cli_env should carry source-authority helper defs"
    );
    assert!(
        boxes.contains("Stage1SourceProgramAuthorityBox"),
        "stage1_cli_env should carry source-program authority helper defs"
    );
    assert!(
        boxes.contains("Stage1ProgramResultValidationBox"),
        "stage1_cli_env should carry program validation helper defs"
    );
    assert!(
        boxes.contains("Stage1ProgramJsonMirCallerBox"),
        "stage1_cli_env should carry program-json caller helper defs"
    );
    assert!(
        boxes.contains("Stage1MirResultValidationBox"),
        "stage1_cli_env should carry mir validation helper defs"
    );
    assert!(
        boxes.contains("Stage1ProgramJsonCompatBox"),
        "stage1_cli_env should keep explicit compat helper defs quarantined"
    );
    assert!(
        defs.iter().any(|def| {
            def["box"] == "Stage1ProgramJsonMirCallerBox"
                && def["name"] == "_emit_mir_from_program_json_text_checked"
        }),
        "stage1 program-json caller entry must be materialized"
    );
    assert!(
        defs.iter().any(|def| {
            def["box"] == "Stage1SourceMirAuthorityBox" && def["name"] == "emit_mir_from_source"
        }),
        "stage1 source-route authority entry must be materialized"
    );
    assert!(
        defs.iter().any(|def| {
            def["box"] == "Stage1SourceProgramAuthorityBox"
                && def["name"] == "emit_program_from_source"
        }),
        "stage1 source-program authority entry must be materialized"
    );
}

#[test]
fn source_to_program_json_v0_accepts_launcher_source_with_multibox_defs() {
    let source = include_str!("../../../../lang/src/runner/launcher.hako");
    let json = source_to_program_json_v0_relaxed(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("defs array");
    assert!(
        defs.iter()
            .any(|def| def["box"] == "HakoCli" && def["name"] == "run"),
        "launcher surrogate should materialize HakoCli.run in defs"
    );
    assert_eq!(
        value["imports"]["LauncherCommandBox"],
        "lang.runner.launcher.command_dispatch"
    );
    assert_eq!(
        value["imports"]["LauncherDispatchBox"],
        "lang.runner.launcher.dispatch"
    );
    assert_eq!(
        value["imports"]["LauncherBootstrapBox"],
        "lang.runner.launcher.bootstrap"
    );
}

#[test]
fn source_to_program_json_v0_accepts_dev_local_alias_sugar() {
    let source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;
    let json = source_to_program_json_v0_relaxed(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["kind"], "Program");
    assert_eq!(value["version"], 0);
    assert!(value["body"].is_array());
}
