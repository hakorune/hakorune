use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[path = "program_json_v0/extract.rs"]
mod extract;
#[path = "program_json_v0/lowering.rs"]
mod lowering;

use extract::{extract_static_main_body_text, find_static_main_body};
use lowering::program_json_v0_from_body;

pub fn source_to_program_json_v0(source_text: &str) -> Result<String, String> {
    match NyashParser::parse_from_string(source_text) {
        Ok(ast) => ast_to_program_json_v0(&ast),
        Err(primary_error) => {
            let body = extract_static_main_body_text(source_text)
                .ok_or_else(|| format!("parse error (Rust parser, v0 subset): {}", primary_error))?;
            let wrapped = format!("static box Main {{ main(args) {{\n{}\n}} }}", body);
            let ast = NyashParser::parse_from_string(&wrapped).map_err(|fallback_error| {
                format!(
                    "parse error (Rust parser, v0 subset): {}; fallback parse error: {}",
                    primary_error, fallback_error
                )
            })?;
            ast_to_program_json_v0(&ast)
        }
    }
}

pub fn ast_to_program_json_v0(ast: &ASTNode) -> Result<String, String> {
    let body = find_static_main_body(ast)
        .ok_or_else(|| "expected `static box Main { main() { ... } }`".to_string())?;
    let program = program_json_v0_from_body(body)?;
    serde_json::to_string(&program).map_err(|error| format!("serialize error: {}", error))
}

#[cfg(test)]
mod tests {
    use super::source_to_program_json_v0;

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
}
