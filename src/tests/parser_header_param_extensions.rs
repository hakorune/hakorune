use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn parse(src: &str) -> ASTNode {
    NyashParser::parse_from_string(src).expect("parse ok")
}

fn find_box<'a>(ast: &'a ASTNode, box_name: &str) -> &'a ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .expect("box declaration not found")
}

fn find_method_params(ast: &ASTNode, box_name: &str, method_name: &str) -> Vec<String> {
    let box_decl = find_box(ast, box_name);
    let ASTNode::BoxDeclaration { methods, .. } = box_decl else {
        panic!("expected BoxDeclaration");
    };
    let method = methods.get(method_name).expect("method not found");
    let ASTNode::FunctionDeclaration { params, .. } = method else {
        panic!("expected FunctionDeclaration");
    };
    params.clone()
}

#[test]
fn parser_accepts_typed_params_and_keeps_param_names_in_ast_v0() {
    let src = r#"
box Worker {
  run(input: StringBox, flags: util.Flag[]) {
    return input
  }
}
"#;
    let ast = parse(src);
    let params = find_method_params(&ast, "Worker", "run");
    assert_eq!(params, vec!["input".to_string(), "flags".to_string()]);
}

#[test]
fn parser_accepts_interface_clause_keyword_and_implements_alias() {
    let src = r#"
box A from Base interface Runnable, Loggable {
  run() { return 0 }
}
box B implements Runnable {
  run() { return 0 }
}
"#;
    let ast = parse(src);

    let ASTNode::BoxDeclaration {
        extends,
        implements,
        ..
    } = find_box(&ast, "A")
    else {
        panic!("expected BoxDeclaration A");
    };
    assert_eq!(extends, &vec!["Base".to_string()]);
    assert_eq!(implements, &vec!["Runnable".to_string(), "Loggable".to_string()]);

    let ASTNode::BoxDeclaration { implements, .. } = find_box(&ast, "B") else {
        panic!("expected BoxDeclaration B");
    };
    assert_eq!(implements, &vec!["Runnable".to_string()]);
}

#[test]
fn parser_accepts_interface_generics_and_typed_signature_params() {
    let src = r#"
interface box Mapper<T, U> {
  map(input: T, output: U)
}
"#;
    let ast = parse(src);
    let ASTNode::BoxDeclaration {
        is_interface,
        type_parameters,
        ..
    } = find_box(&ast, "Mapper")
    else {
        panic!("expected BoxDeclaration Mapper");
    };
    assert!(*is_interface);
    assert_eq!(type_parameters, &vec!["T".to_string(), "U".to_string()]);

    let params = find_method_params(&ast, "Mapper", "map");
    assert_eq!(params, vec!["input".to_string(), "output".to_string()]);
}
