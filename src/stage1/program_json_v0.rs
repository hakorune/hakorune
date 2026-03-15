//! Stage1 Program(JSON v0) façade.
//!
//! Layout SSOT:
//! - `routing.rs`: source-shape and build-route policy
//! - `authority.rs`: strict source authority
//! - `bridge_shim.rs`: future-retire stage1 bridge shim
//! - `extract.rs`: source observation / helper extraction
//! - `lowering.rs`: AST subset -> Program(JSON v0) lowering
//!
//! Cross-crate surface:
//! - allowed: `emit_program_json_v0_for_strict_authority_source(...)`,
//!   `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
//! - forbidden: legacy default alias, route/source-shape internals,
//!   parse/lower orchestration

#[path = "program_json_v0/authority.rs"]
mod authority;
#[path = "program_json_v0/bridge_shim.rs"]
mod bridge_shim;
#[path = "program_json_v0/extract.rs"]
mod extract;
#[path = "program_json_v0/lowering.rs"]
mod lowering;
#[path = "program_json_v0/routing.rs"]
mod routing;

#[cfg(test)]
use routing::strict_authority_program_json_v0_source_rejection;

fn trace_enabled() -> bool {
    std::env::var("HAKO_STAGE1_PROGRAM_JSON_TRACE")
        .ok()
        .as_deref()
        == Some("1")
}

const STAGE1_PROGRAM_JSON_V0_FREEZE_TAG: &str = "[freeze:contract][stage1_program_json_v0]";

fn current_stage1_build_box_strict_authority_mode() -> bool {
    crate::config::env::stage1::emit_program_json()
}

// Public entry surface

/// Explicit compatibility keep for launcher/dev-local alias sugar.
fn source_to_program_json_v0_relaxed(source_text: &str) -> Result<String, String> {
    authority::source_to_program_json_v0_relaxed(source_text)
}

/// Explicit strict parse entry kept owner-local to this cluster.
fn source_to_program_json_v0_strict(source_text: &str) -> Result<String, String> {
    authority::source_to_program_json_v0_strict(source_text)
}

/// Explicit authority helper for current `stage1-env-mir-source`.
pub fn emit_program_json_v0_for_strict_authority_source(
    source_text: &str,
) -> Result<String, String> {
    authority::emit_program_json_v0_for_strict_authority_source(source_text)
}

/// Crate-local shim for the future-retire Rust Stage1 bridge emit-program route.
pub(crate) fn emit_program_json_v0_for_stage1_bridge_emit_program_json(
    source_text: &str,
) -> Result<String, String> {
    bridge_shim::emit_program_json_v0_for_stage1_bridge_emit_program_json(source_text)
}

fn format_stage1_program_json_v0_freeze(error_text: String) -> String {
    format!("{STAGE1_PROGRAM_JSON_V0_FREEZE_TAG} {}", error_text)
}

/// Owner-local explicit build-box helper.
fn emit_program_json_v0_for_stage1_build_box(
    source_text: &str,
    strict_authority_mode: bool,
) -> Result<String, String> {
    routing::emit_stage1_build_box_program_json(source_text, strict_authority_mode)
        .map(|emission| emission.into_program_json())
        .map_err(format_stage1_program_json_v0_freeze)
}

/// Cross-crate build-box helper that follows the current stage1 mode contract.
pub fn emit_program_json_v0_for_current_stage1_build_box_mode(
    source_text: &str,
) -> Result<String, String> {
    emit_program_json_v0_for_stage1_build_box(
        source_text,
        current_stage1_build_box_strict_authority_mode(),
    )
}

/// Legacy strict alias kept owner-local only.
#[cfg(test)]
pub(crate) fn source_to_program_json_v0(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0_strict(source_text)
}

#[cfg(test)]
mod tests {
    use super::{
        emit_program_json_v0_for_stage1_bridge_emit_program_json,
        emit_program_json_v0_for_stage1_build_box,
        emit_program_json_v0_for_strict_authority_source, source_to_program_json_v0,
        source_to_program_json_v0_relaxed, source_to_program_json_v0_strict,
        strict_authority_program_json_v0_source_rejection,
    };
    use std::collections::BTreeSet;

    #[test]
    fn source_to_program_json_v0_minimal_main() {
        let source = r#"
static box Main {
  main() {
    print(42)
    return 0
  }
}
"#;
        let json = source_to_program_json_v0_strict(source).expect("program json");
        assert!(json.contains("\"kind\":\"Program\""));
        assert!(json.contains("\"version\":0"));
        assert!(json.contains("\"env.console.log\""));
    }

    #[test]
    fn source_to_program_json_v0_supports_static_method_call() {
        let source = r#"
static box Driver {
  main(args) {
    return 0
  }
}
static box Main {
  main(args) {
    return Driver.main(args)
  }
}
"#;
        let json = source_to_program_json_v0_strict(source).expect("program json");
        assert!(json.contains("\"kind\":\"Program\""));
        assert!(json.contains("\"type\":\"Call\""));
        assert!(json.contains("\"Driver.main\""));
    }

    #[test]
    fn source_to_program_json_v0_compiler_stageb_main_supported() {
        let source = include_str!("../../lang/src/compiler/entry/compiler_stageb.hako");
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

  method helper(x) {
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
        assert_eq!(defs[0]["body"]["kind"], "Program");
        assert!(defs[0]["body"]["body"].is_array());
    }

    #[test]
    fn source_to_program_json_v0_accepts_stage1_cli_env_source() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["kind"], "Program");
        assert_eq!(value["version"], 0);
        assert_eq!(
            value["imports"]["BuildBox"],
            "lang.compiler.build.build_box"
        );
        assert_eq!(
            value["imports"]["Stage1UsingResolverBox"],
            "lang.compiler.entry.using_resolver_box"
        );
        assert_eq!(value["imports"]["StringHelpers"], "sh_core");
    }

    #[test]
    fn source_to_program_json_v0_stage1_cli_env_materializes_same_file_stage1_boxes() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
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
            boxes.contains("Stage1ProgramAuthorityBox"),
            "stage1_cli_env should carry program-authority helper defs"
        );
        assert!(
            boxes.contains("Stage1ProgramResultValidationBox"),
            "stage1_cli_env should carry program validation helper defs"
        );
        assert!(
            boxes.contains("Stage1SourceMirAuthorityBox"),
            "stage1_cli_env should carry source-route helper defs"
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
                def["box"] == "Stage1ProgramAuthorityBox"
                    && def["name"] == "build_program_json_from_source"
            }),
            "stage1 program authority entry must be materialized"
        );
        assert!(
            defs.iter().any(|def| {
                def["box"] == "Stage1SourceMirAuthorityBox" && def["name"] == "emit_mir_from_source"
            }),
            "stage1 source-route authority entry must be materialized"
        );
    }

    #[test]
    fn source_to_program_json_v0_accepts_launcher_source_with_multibox_defs() {
        let source = include_str!("../../lang/src/runner/launcher.hako");
        let json = source_to_program_json_v0_relaxed(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let defs = value["defs"].as_array().expect("defs array");
        assert!(
            defs.iter()
                .any(|def| def["box"] == "HakoCli" && def["name"] == "run"),
            "launcher surrogate should materialize HakoCli.run in defs"
        );
        assert_eq!(
            value["imports"]["MirBuilderBox"],
            "lang.mir.builder.MirBuilderBox"
        );
        assert_eq!(
            value["imports"]["BuildBox"],
            "lang.compiler.build.build_box"
        );
        assert_eq!(
            value["imports"]["CodegenBridgeBox"],
            "selfhost.shared.host_bridge.codegen_bridge"
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

    #[test]
    fn classify_program_json_v0_source_shape_detects_current_compat_keep_shapes() {
        let stage1_cli_env = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let launcher = include_str!("../../lang/src/runner/launcher.hako");
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
            super::routing::classify_program_json_v0_source_shape(launcher).relaxed_reason(),
            Some("dev-local-alias-sugar")
        ));
        assert!(matches!(
            super::routing::classify_program_json_v0_source_shape(dev_local).relaxed_reason(),
            Some("dev-local-alias-sugar")
        ));
    }

    #[test]
    fn classify_program_json_v0_source_shape_reports_strict_vs_relaxed() {
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");

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
            "../../lang/src/runner/stage1_cli_env.hako"
        ));
        assert_eq!(strict_shape.label(), "strict-safe");
        assert_eq!(strict_shape.relaxed_reason(), None);
        assert_eq!(
            strict_shape.strict_authority_rejection("source route"),
            None
        );

        let relaxed_shape = super::routing::classify_program_json_v0_source_shape(include_str!(
            "../../lang/src/runner/launcher.hako"
        ));
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");

        assert_eq!(
            strict_authority_program_json_v0_source_rejection(strict_source, "source route"),
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");

        let strict = emit_program_json_v0_for_strict_authority_source(strict_source)
            .expect("strict authority source emission");
        assert!(strict.contains("\"kind\":\"Program\""));

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
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");
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
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");
        let error = emit_program_json_v0_for_stage1_bridge_emit_program_json(relaxed_source)
            .expect_err("stage1 bridge strict parse should fail on launcher source");
        assert!(
            error.starts_with("emit-program-json-v0: "),
            "unexpected error: {error}"
        );
        assert!(
            error.contains("Unexpected character '@'"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn build_route_accessors_report_current_contract() {
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");

        let strict_authority =
            super::routing::emit_stage1_build_box_program_json(strict_source, true)
                .expect("strict authority build emission");
        assert_eq!(
            strict_authority.trace_summary(),
            "route=strict-authority relaxed_reason=none"
        );

        let strict_default =
            super::routing::emit_stage1_build_box_program_json(strict_source, false)
                .expect("strict default build emission");
        assert_eq!(
            strict_default.trace_summary(),
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let relaxed_source = include_str!("../../lang/src/runner/launcher.hako");

        let strict = super::routing::emit_stage1_build_box_program_json(strict_source, false)
            .expect("strict-safe build emission");
        assert_eq!(
            strict.trace_summary(),
            "route=strict-default relaxed_reason=none"
        );
        assert!(strict.into_program_json().contains("\"kind\":\"Program\""));

        let relaxed = super::routing::emit_stage1_build_box_program_json(relaxed_source, false)
            .expect("relaxed build emission");
        assert_eq!(
            relaxed.trace_summary(),
            "route=relaxed-compat relaxed_reason=dev-local-alias-sugar"
        );
        assert!(relaxed.into_program_json().contains("\"kind\":\"Program\""));
    }

    #[test]
    fn source_to_program_json_v0_rejects_script_body_without_static_main() {
        let source = r#"
print(42)
return 0
"#;
        let error = source_to_program_json_v0(source).expect_err("script body should fail-fast");
        assert!(
            error.contains("expected `static box Main { main() { ... } }`")
                || error.contains("parse error (Rust parser, v0 subset):"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn source_to_program_json_v0_strict_accepts_stage1_cli_env_source() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["kind"], "Program");
        assert_eq!(value["version"], 0);
    }

    #[test]
    fn emit_program_json_v0_for_current_stage1_build_box_mode_returns_payload_only() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let program_json = super::emit_program_json_v0_for_current_stage1_build_box_mode(source)
            .expect("program json");
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("\"version\":0"));
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

    #[test]
    fn source_to_program_json_v0_default_is_now_strict() {
        let source = r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#;
        let error = source_to_program_json_v0(source).expect_err("default path should be strict");
        assert!(
            error.contains("parse error (Rust parser, v0 subset):"),
            "unexpected error: {error}"
        );
    }
}
