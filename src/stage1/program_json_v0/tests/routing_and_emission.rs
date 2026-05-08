use super::*;

#[test]
fn build_route_accessors_report_current_contract() {
    let strict_source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let launcher_source = include_str!("../../../../lang/src/runner/launcher.hako");
    let relaxed_source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;

    let strict_authority = super::routing::emit_stage1_build_box_program_json(strict_source, true)
        .expect("strict authority build emission");
    assert_eq!(
        strict_authority.trace_summary(),
        "route=strict-authority relaxed_reason=none"
    );

    let strict_default = super::routing::emit_stage1_build_box_program_json(strict_source, false)
        .expect("strict default build emission");
    assert_eq!(
        strict_default.trace_summary(),
        "route=strict-default relaxed_reason=none"
    );

    let launcher = super::routing::emit_stage1_build_box_program_json(launcher_source, false)
        .expect("launcher build emission");
    assert_eq!(
        launcher.trace_summary(),
        "route=strict-default relaxed_reason=none"
    );

    let relaxed = super::routing::emit_stage1_build_box_program_json(relaxed_source, false)
        .expect("relaxed build emission");
    assert_eq!(
        relaxed.trace_summary(),
        "route=relaxed-compat relaxed_reason=dev-local-alias-sugar"
    );
}

#[test]
fn routing_build_box_emission_returns_route_and_payload() {
    let strict_source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let launcher_source = include_str!("../../../../lang/src/runner/launcher.hako");
    let relaxed_source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;

    let strict = super::routing::emit_stage1_build_box_program_json(strict_source, false)
        .expect("strict-safe build emission");
    assert_eq!(
        strict.trace_summary(),
        "route=strict-default relaxed_reason=none"
    );
    assert!(strict.into_program_json().contains("\"kind\":\"Program\""));

    let launcher = super::routing::emit_stage1_build_box_program_json(launcher_source, false)
        .expect("launcher build emission");
    assert_eq!(
        launcher.trace_summary(),
        "route=strict-default relaxed_reason=none"
    );
    assert!(launcher
        .into_program_json()
        .contains("\"kind\":\"Program\""));

    let relaxed = super::routing::emit_stage1_build_box_program_json(relaxed_source, false)
        .expect("relaxed build emission");
    assert_eq!(
        relaxed.trace_summary(),
        "route=relaxed-compat relaxed_reason=dev-local-alias-sugar"
    );
    assert!(relaxed.into_program_json().contains("\"kind\":\"Program\""));
}

#[test]
fn source_to_program_json_v0_strict_rejects_script_body_without_static_main() {
    let source = r#"
print(42)
return 0
"#;
    let error = source_to_program_json_v0_strict(source).expect_err("script body should fail-fast");
    assert!(
        error.contains("expected `static box Main { main() { ... } }`")
            || error.contains("parse error (Rust parser, v0 subset):"),
        "unexpected error: {error}"
    );
}

#[test]
fn source_to_program_json_v0_strict_accepts_stage1_cli_env_source() {
    let source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["kind"], "Program");
    assert_eq!(value["version"], 0);
}

#[test]
fn source_to_program_json_v0_emits_static_data_plans_for_static_const_table() {
    let source = r#"
static const SIZE_CLASS: u16[] = [8, 16, 24, 32]
static box Main {
  main() {
    return 0
  }
}
"#;
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let root: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let plans = root["static_data_plans"]
        .as_array()
        .expect("static_data_plans array");

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["source_name"], "SIZE_CLASS");
    assert_eq!(plans[0]["symbol"], ".hako.static.SIZE_CLASS");
    assert_eq!(plans[0]["element"], "u16");
    assert_eq!(plans[0]["align"], 2);
    assert_eq!(plans[0]["linkage"], "private");
    assert_eq!(plans[0]["unnamed_addr"], true);
    assert_eq!(plans[0]["values"], serde_json::json!([8, 16, 24, 32]));
}

#[test]
fn emit_program_json_v0_for_current_stage1_build_box_mode_returns_payload_only() {
    let source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let program_json = super::emit_program_json_v0_for_current_stage1_build_box_mode(source)
        .expect("program json");
    assert!(program_json.contains("\"kind\":\"Program\""));
    assert!(program_json.contains("\"version\":0"));
}

#[test]
fn emit_program_json_v0_for_current_stage1_build_box_mode_emits_stage1_cli_env_program_json() {
    let source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let json = super::emit_program_json_v0_for_current_stage1_build_box_mode(source)
        .expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("defs array");

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
    assert!(
        defs.iter()
            .any(|def| def["box"] == "Stage1InputContractBox"),
        "build-box mode should keep input-contract helpers"
    );
    assert!(
        defs.iter()
            .any(|def| def["box"] == "Stage1ProgramJsonMirCallerBox"),
        "build-box mode should keep program-json caller helpers"
    );
    assert!(
        defs.iter()
            .any(|def| def["box"] == "Stage1SourceMirAuthorityBox"),
        "build-box mode should keep source-authority helpers"
    );
    assert!(
        defs.iter()
            .any(|def| def["box"] == "Stage1SourceProgramAuthorityBox"),
        "build-box mode should keep source-program authority helpers"
    );
}

#[test]
fn emit_program_json_v0_for_current_stage1_build_box_mode_emits_launcher_program_json() {
    let source = include_str!("../../../../lang/src/runner/launcher.hako");
    let json = super::emit_program_json_v0_for_current_stage1_build_box_mode(source)
        .expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("defs array");

    assert_eq!(value["kind"], "Program");
    assert_eq!(value["version"], 0);
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
    assert!(
        defs.iter()
            .any(|def| def["box"] == "HakoCli" && def["name"] == "run"),
        "build-box mode should keep launcher helper defs"
    );
}

#[test]
fn source_to_program_json_v0_strict_rejects_dev_local_alias_sugar() {
    let source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;
    let error = source_to_program_json_v0_strict(source)
        .expect_err("strict path should reject @local sugar");
    assert!(
        error.contains("parse error (Rust parser, v0 subset):"),
        "unexpected error: {error}"
    );
}
