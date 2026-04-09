//! Stage1 Program(JSON v0) façade.
//!
//! Layout SSOT:
//! - `routing.rs`: source-shape and build-route policy
//! - `authority.rs`: strict source authority
//! - `extract.rs`: source observation / helper extraction
//! - `record_payload.rs`: shared enum record payload boxification helpers
//! - `lowering.rs`: AST subset -> Program(JSON v0) lowering
//!
//! Cross-crate surface:
//! - allowed: `emit_program_json_v0_for_strict_authority_source(...)`,
//!   `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
//! - forbidden: route/source-shape internals, parse/lower orchestration

#[path = "program_json_v0/authority.rs"]
mod authority;
#[path = "program_json_v0/extract.rs"]
mod extract;
#[path = "program_json_v0/lowering.rs"]
mod lowering;
#[path = "program_json_v0/record_payload.rs"]
mod record_payload;
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

/// Crate-local helper for the future-retire Rust Stage1 bridge emit-program route.
pub(crate) fn emit_program_json_v0_for_stage1_bridge_emit_program_json(
    source_text: &str,
) -> Result<String, String> {
    authority::source_to_program_json_v0_strict(source_text)
        .map_err(|error_text| format!("emit-program-json-v0: {}", error_text))
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

#[cfg(test)]
mod tests {
    use super::{
        emit_program_json_v0_for_stage1_bridge_emit_program_json,
        emit_program_json_v0_for_stage1_build_box,
        emit_program_json_v0_for_strict_authority_source, source_to_program_json_v0_relaxed,
        source_to_program_json_v0_strict, strict_authority_program_json_v0_source_rejection,
    };
    use std::collections::BTreeSet;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    struct FeatureOverrideGuard {
        prev: Option<String>,
        _lock: MutexGuard<'static, ()>,
    }

    impl FeatureOverrideGuard {
        fn new(features: Option<&str>) -> Self {
            let lock = match env_guard().lock() {
                Ok(lock) => lock,
                Err(poisoned) => poisoned.into_inner(),
            };
            let prev = std::env::var("NYASH_FEATURES").ok();
            match features {
                Some(v) => std::env::set_var("NYASH_FEATURES", v),
                None => std::env::remove_var("NYASH_FEATURES"),
            }
            Self { prev, _lock: lock }
        }
    }

    impl Drop for FeatureOverrideGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var("NYASH_FEATURES", v),
                None => std::env::remove_var("NYASH_FEATURES"),
            }
        }
    }

    fn with_features<R>(features: Option<&str>, f: impl FnOnce() -> R) -> R {
        let _guard = FeatureOverrideGuard::new(features);
        f()
    }

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
    fn source_to_program_json_v0_emits_enum_inventory_and_ctor() {
        let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local x = Option::Some("hello")
    return 0
  }
}
"#;

        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let enum_decls = value["enum_decls"].as_array().expect("enum decls");
        assert_eq!(enum_decls.len(), 1);
        assert_eq!(enum_decls[0]["name"], "Option");
        assert_eq!(enum_decls[0]["type_parameters"], serde_json::json!(["T"]));
        assert_eq!(enum_decls[0]["variants"][1]["name"], "Some");
        assert_eq!(enum_decls[0]["variants"][1]["payload_type"], "T");

        let body = value["body"].as_array().expect("body");
        assert_eq!(body[0]["type"], "Local");
        assert_eq!(body[0]["expr"]["type"], "EnumCtor");
        assert_eq!(body[0]["expr"]["enum"], "Option");
        assert_eq!(body[0]["expr"]["variant"], "Some");
    }

    #[test]
    fn source_to_program_json_v0_emits_known_enum_match() {
        let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = Option::Some(1)
    return match value {
      Some(v) => v
      None => 0
    }
  }
}
"#;

        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let body = value["body"].as_array().expect("body");
        assert_eq!(body[1]["type"], "Return");
        assert_eq!(body[1]["expr"]["type"], "EnumMatch");
        assert_eq!(body[1]["expr"]["enum"], "Option");
        assert_eq!(body[1]["expr"]["arms"][0]["variant"], "Some");
        assert_eq!(body[1]["expr"]["arms"][0]["bind"], "v");
        assert_eq!(body[1]["expr"]["arms"][1]["variant"], "None");
        assert!(body[1]["expr"]["else"].is_null());
    }

    #[test]
    fn source_to_program_json_v0_rejects_non_exhaustive_enum_match() {
        let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = Option::Some(1)
    return match value {
      Some(v) => v
      _ => 0
    }
  }
}
"#;

        let error = source_to_program_json_v0_strict(source)
            .expect_err("non-exhaustive enum match should fail");
        assert!(error.contains("non-exhaustive enum match"));
        assert!(error.contains("None"));
    }

    #[test]
    fn source_to_program_json_v0_emits_record_enum_payload_box_contract() {
        let source = r#"
enum Token {
  Ident { name: String }
  Eof
}

static box Main {
  main() {
    local tok = Token::Ident { name: "hello" }
    return match tok {
      Ident { name } => name
      Eof => "eof"
    }
  }
}
"#;

        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let payload_box = "__NyEnumPayload_Token_Ident";

        let enum_decls = value["enum_decls"].as_array().expect("enum decls");
        assert_eq!(enum_decls[0]["variants"][0]["payload_type"], payload_box);
        assert_eq!(
            enum_decls[0]["variants"][0]["record_fields"][0]["name"],
            "name"
        );

        let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
        let payload_decl = user_box_decls
            .iter()
            .find(|decl| decl["name"] == payload_box)
            .expect("hidden payload box decl");
        assert_eq!(payload_decl["field_decls"][0]["name"], "name");
        assert_eq!(payload_decl["field_decls"][0]["declared_type"], "String");

        let body = value["body"].as_array().expect("body");
        assert_eq!(body[0]["expr"]["type"], "EnumCtor");
        assert_eq!(body[0]["expr"]["payload_type"], payload_box);
        assert_eq!(body[0]["expr"]["args"][0]["type"], "New");
        assert_eq!(body[0]["expr"]["args"][0]["class"], payload_box);

        assert_eq!(body[1]["expr"]["type"], "EnumMatch");
        assert_eq!(body[1]["expr"]["arms"][0]["payload_type"], payload_box);
        assert_eq!(body[1]["expr"]["arms"][0]["expr"]["type"], "BlockExpr");
        assert_eq!(
            body[1]["expr"]["arms"][0]["expr"]["prelude"][0]["expr"]["type"],
            "Field"
        );
        assert_eq!(
            body[1]["expr"]["arms"][0]["expr"]["prelude"][0]["expr"]["field"],
            "name"
        );
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
            value["imports"]["Stage1UsingResolverBox"],
            "lang.compiler.entry.using_resolver_box"
        );
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
                    && def["name"] == "emit_mir_from_program_json_checked"
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
            value["imports"]["HostFacadeBox"],
            "lang.runtime.host.host_facade_box"
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
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
            "../../lang/src/runner/stage1_cli_env.hako"
        ));
        assert_eq!(strict_shape.label(), "strict-safe");
        assert_eq!(strict_shape.relaxed_reason(), None);
        assert_eq!(
            strict_shape.strict_authority_rejection("source route"),
            None
        );

        let launcher_shape = super::routing::classify_program_json_v0_source_shape(include_str!(
            "../../lang/src/runner/launcher.hako"
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let launcher_source = include_str!("../../lang/src/runner/launcher.hako");
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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let launcher_source = include_str!("../../lang/src/runner/launcher.hako");
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

    #[test]
    fn build_route_accessors_report_current_contract() {
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let launcher_source = include_str!("../../lang/src/runner/launcher.hako");
        let relaxed_source = r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#;

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
        let strict_source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let launcher_source = include_str!("../../lang/src/runner/launcher.hako");
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
        let error =
            source_to_program_json_v0_strict(source).expect_err("script body should fail-fast");
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
    fn emit_program_json_v0_for_current_stage1_build_box_mode_emits_stage1_cli_env_program_json() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let json = super::emit_program_json_v0_for_current_stage1_build_box_mode(source)
            .expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let defs = value["defs"].as_array().expect("defs array");

        assert_eq!(value["kind"], "Program");
        assert_eq!(value["version"], 0);
        assert_eq!(
            value["imports"]["Stage1UsingResolverBox"],
            "lang.compiler.entry.using_resolver_box"
        );
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
        let source = include_str!("../../lang/src/runner/launcher.hako");
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
}
