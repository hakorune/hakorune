use crate::ast::ASTNode;
use crate::parser::NyashParser;
use std::collections::BTreeMap;

#[path = "program_json_v0/extract.rs"]
mod extract;
#[path = "program_json_v0/lowering.rs"]
mod lowering;

use extract::{
    collect_using_imports, find_static_main_box, preexpand_dev_local_aliases,
};
use lowering::{defs_json_v0_from_methods, program_json_v0_from_body};

fn trace_enabled() -> bool {
    std::env::var("HAKO_STAGE1_PROGRAM_JSON_TRACE").ok().as_deref() == Some("1")
}

pub fn source_to_program_json_v0(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0_impl(source_text, false)
}

pub fn source_to_program_json_v0_relaxed(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0_impl(source_text, true)
}

pub fn source_to_program_json_v0_strict(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0(source_text)
}

fn source_to_program_json_v0_impl(
    source_text: &str,
    allow_dev_local_alias_sugar: bool,
) -> Result<String, String> {
    let imports = collect_using_imports(source_text);
    let normalized_source = if allow_dev_local_alias_sugar {
        preexpand_dev_local_aliases(source_text)
    } else {
        source_text.to_string()
    };
    let ast = NyashParser::parse_from_string(&normalized_source)
        .map_err(|primary_error| format!("parse error (Rust parser, v0 subset): {}", primary_error))?;
    ast_to_program_json_v0_with_imports(&ast, imports)
}

pub fn ast_to_program_json_v0(ast: &ASTNode) -> Result<String, String> {
    ast_to_program_json_v0_with_imports(ast, BTreeMap::new())
}

fn ast_to_program_json_v0_with_imports(
    ast: &ASTNode,
    imports: BTreeMap<String, String>,
) -> Result<String, String> {
    let main_box = find_static_main_box(ast)
        .ok_or_else(|| "expected `static box Main { main() { ... } }`".to_string())?;
    if trace_enabled() {
        eprintln!(
            "[stage1/program_json_v0] main_body_stmts={} helper_defs={} imports={}",
            main_box.body.len(),
            main_box.helper_methods.len(),
            imports.len()
        );
    }
    let mut program = program_json_v0_from_body(main_box.body)?;
    let defs = defs_json_v0_from_methods(&main_box.helper_methods)?;
    if trace_enabled() {
        eprintln!("[stage1/program_json_v0] serialized_defs={}", defs.len());
    }
    if !defs.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert("defs".to_string(), serde_json::Value::Array(defs));
    }
    if !imports.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "imports".to_string(),
            serde_json::to_value(imports)
                .map_err(|error| format!("imports serialize error: {}", error))?,
        );
    }
    serde_json::to_string(&program).map_err(|error| format!("serialize error: {}", error))
}

#[cfg(test)]
mod tests {
    use super::{
        source_to_program_json_v0, source_to_program_json_v0_relaxed,
        source_to_program_json_v0_strict,
    };

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
        let json = source_to_program_json_v0(source).expect("program json");
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
        let json = source_to_program_json_v0(source).expect("program json");
        assert!(json.contains("\"kind\":\"Program\""));
        assert!(json.contains("\"type\":\"Call\""));
        assert!(json.contains("\"Driver.main\""));
    }

    #[test]
    fn source_to_program_json_v0_compiler_stageb_main_supported() {
        let source = include_str!("../../lang/src/compiler/entry/compiler_stageb.hako");
        let json = source_to_program_json_v0(source).expect("program json");
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
        let json = source_to_program_json_v0(source).expect("program json");
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
        let json = source_to_program_json_v0(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["kind"], "Program");
        assert_eq!(value["version"], 0);
        assert_eq!(value["imports"]["BuildBox"], "lang.compiler.build.build_box");
        assert_eq!(
            value["imports"]["Stage1UsingResolverBox"],
            "lang.compiler.entry.using_resolver_box"
        );
        assert_eq!(value["imports"]["StringHelpers"], "sh_core");
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
    fn source_to_program_json_v0_strict_rejects_dev_local_alias_sugar() {
        let source = r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#;
        let error =
            source_to_program_json_v0_strict(source).expect_err("strict path should reject @local sugar");
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
