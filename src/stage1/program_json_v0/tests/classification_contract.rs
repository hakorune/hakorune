use super::*;

#[test]
fn classify_program_json_v0_source_shape_detects_current_compat_keep_shapes() {
    let stage1_cli_env = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let launcher = include_str!("../../../../lang/src/runner/launcher.hako");
    let dev_local = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;

    assert!(matches!(
        super::routing::classify_program_json_v0_source_shape(stage1_cli_env).label(),
        "strict-safe"
    ));
    assert!(matches!(
        super::routing::classify_program_json_v0_source_shape(launcher).label(),
        "strict-safe"
    ));
    assert!(matches!(
        super::routing::classify_program_json_v0_source_shape(dev_local).relaxed_reason(),
        Some("dev-local-alias-sugar")
    ));
}

#[test]
fn classify_program_json_v0_source_shape_reports_strict_vs_relaxed() {
    let strict_source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
    let relaxed_source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;

    assert!(matches!(
        super::routing::classify_program_json_v0_source_shape(strict_source).label(),
        "strict-safe"
    ));
    assert!(matches!(
        super::routing::classify_program_json_v0_source_shape(relaxed_source).relaxed_reason(),
        Some("dev-local-alias-sugar")
    ));
}

#[test]
fn classify_program_json_v0_source_shape_helpers_report_current_contract() {
    let strict_shape = super::routing::classify_program_json_v0_source_shape(include_str!(
        "../../../../lang/src/runner/stage1_cli_env.hako"
    ));
    assert_eq!(strict_shape.label(), "strict-safe");
    assert_eq!(strict_shape.relaxed_reason(), None);
    assert_eq!(
        strict_shape.strict_authority_rejection("source route"),
        None
    );

    let launcher_shape = super::routing::classify_program_json_v0_source_shape(include_str!(
        "../../../../lang/src/runner/launcher.hako"
    ));
    assert_eq!(launcher_shape.label(), "strict-safe");
    assert_eq!(launcher_shape.relaxed_reason(), None);
    assert_eq!(
        launcher_shape.strict_authority_rejection("source route"),
        None
    );

    let relaxed_shape = super::routing::classify_program_json_v0_source_shape(
        r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#,
    );
    assert_eq!(relaxed_shape.label(), "relaxed-compat");
    assert_eq!(
        relaxed_shape.relaxed_reason(),
        Some("dev-local-alias-sugar")
    );
    assert_eq!(
        relaxed_shape.strict_authority_rejection("source route"),
        Some(
            "source route rejects compat-only relaxed-compat source shape (dev-local-alias-sugar)"
                .to_string()
        )
    );
}

#[test]
fn strict_authority_program_json_v0_source_rejection_reports_current_contract() {
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

    assert_eq!(
        strict_authority_program_json_v0_source_rejection(strict_source, "source route"),
        None
    );
    assert_eq!(
        strict_authority_program_json_v0_source_rejection(launcher_source, "source route"),
        None
    );
    assert_eq!(
        strict_authority_program_json_v0_source_rejection(relaxed_source, "source route"),
        Some(
            "source route rejects compat-only relaxed-compat source shape (dev-local-alias-sugar)"
                .to_string()
        )
    );
}

#[test]
fn emit_program_json_v0_for_strict_authority_source_enforces_current_contract() {
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

    let strict = emit_program_json_v0_for_strict_authority_source(strict_source)
        .expect("strict authority source emission");
    assert!(strict.contains("\"kind\":\"Program\""));

    let launcher = emit_program_json_v0_for_strict_authority_source(launcher_source)
        .expect("launcher should now satisfy strict authority source contract");
    assert!(launcher.contains("\"kind\":\"Program\""));

    let error = emit_program_json_v0_for_strict_authority_source(relaxed_source)
        .expect_err("relaxed source should fail-fast on authority path");
    assert!(
        error.contains(
            "source route rejects compat-only relaxed-compat source shape (dev-local-alias-sugar)"
        ),
        "unexpected error: {error}"
    );
}

#[test]
fn emit_program_json_v0_for_stage1_build_box_wraps_freeze_contract_error() {
    let relaxed_source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;
    let error = match emit_program_json_v0_for_stage1_build_box(relaxed_source, true) {
        Ok(_) => panic!("strict authority build-box path should freeze"),
        Err(error) => error,
    };
    assert!(
        error.starts_with(super::STAGE1_PROGRAM_JSON_V0_FREEZE_TAG),
        "unexpected error: {error}"
    );
    assert!(
        error.contains("dev-local-alias-sugar"),
        "unexpected error: {error}"
    );
}

#[test]
fn emit_program_json_v0_for_stage1_bridge_emit_program_json_wraps_bridge_error() {
    let relaxed_source = r#"
static box Main {
  main() {
@x = 41
return x + 1
  }
}
"#;
    let error = emit_program_json_v0_for_stage1_bridge_emit_program_json(relaxed_source)
        .expect_err("stage1 bridge strict parse should fail on compat-only source");
    assert!(
        error.starts_with("emit-program-json-v0: "),
        "unexpected error: {error}"
    );
    assert!(
        error.contains("Unexpected character '@'"),
        "unexpected error: {error}"
    );
}
