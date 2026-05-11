use crate::ast::{ASTNode, ParamDecl};
use crate::parser::NyashParser;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};

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

fn find_method_decl<'a>(ast: &'a ASTNode, box_name: &str, method_name: &str) -> &'a ASTNode {
    let box_decl = find_box(ast, box_name);
    let ASTNode::BoxDeclaration { methods, .. } = box_decl else {
        panic!("expected BoxDeclaration");
    };
    methods.get(method_name).expect("method not found")
}

fn param(name: &str, ty: Option<&str>) -> ParamDecl {
    ParamDecl {
        name: name.to_string(),
        declared_type_name: ty.map(str::to_string),
    }
}

#[test]
fn parser_accepts_typed_params_and_keeps_param_names_in_ast_v0() {
    let src = r#"
box Worker {
  run(input: StringBox, flags: util.Flag[]): i64 {
    return input
  }
}
"#;
    let ast = parse(src);
    let params = find_method_params(&ast, "Worker", "run");
    assert_eq!(params, vec!["input".to_string(), "flags".to_string()]);

    let ASTNode::FunctionDeclaration {
        param_decls,
        return_type_name,
        ..
    } = find_method_decl(&ast, "Worker", "run")
    else {
        panic!("expected FunctionDeclaration");
    };
    assert_eq!(
        param_decls,
        &vec![
            param("input", Some("StringBox")),
            param("flags", Some("util.Flag[]")),
        ]
    );
    assert_eq!(return_type_name.as_deref(), Some("i64"));
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
    assert_eq!(
        implements,
        &vec!["Runnable".to_string(), "Loggable".to_string()]
    );

    let ASTNode::BoxDeclaration { implements, .. } = find_box(&ast, "B") else {
        panic!("expected BoxDeclaration B");
    };
    assert_eq!(implements, &vec!["Runnable".to_string()]);
}

#[test]
fn parser_accepts_interface_generics_and_typed_signature_params() {
    let src = r#"
interface box Mapper<T, U> {
  map(input: T, output: U): U
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

    let ASTNode::FunctionDeclaration {
        param_decls,
        return_type_name,
        ..
    } = find_method_decl(&ast, "Mapper", "map")
    else {
        panic!("expected FunctionDeclaration");
    };
    assert_eq!(
        param_decls,
        &vec![param("input", Some("T")), param("output", Some("U"))]
    );
    assert_eq!(return_type_name.as_deref(), Some("U"));
}

#[test]
fn parser_preserves_constructor_and_static_method_type_metadata() {
    let src = r#"
box Page {
  birth(capacity: usize, page_id: i64) {
  }
}

static box Main {
  main(argc: usize): i64 {
    return 0
  }
}
"#;
    let ast = parse(src);

    let ASTNode::BoxDeclaration { constructors, .. } = find_box(&ast, "Page") else {
        panic!("expected BoxDeclaration Page");
    };
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        ..
    } = constructors
        .get("birth/2")
        .expect("birth constructor not found")
    else {
        panic!("expected FunctionDeclaration birth");
    };
    assert_eq!(params, &vec!["capacity".to_string(), "page_id".to_string()]);
    assert_eq!(
        param_decls,
        &vec![
            param("capacity", Some("usize")),
            param("page_id", Some("i64")),
        ]
    );
    assert_eq!(return_type_name, &None);

    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        ..
    } = find_method_decl(&ast, "Main", "main")
    else {
        panic!("expected FunctionDeclaration main");
    };
    assert_eq!(params, &vec!["argc".to_string()]);
    assert_eq!(param_decls, &vec![param("argc", Some("usize"))]);
    assert_eq!(return_type_name.as_deref(), Some("i64"));
}

#[test]
fn ast_json_roundtrip_preserves_param_and_return_type_metadata() {
    let src = r#"
box Worker {
  run(size: usize, tag: i64): usize {
    return size
  }
}
"#;
    let ast = parse(src);
    let json = ast_to_json_roundtrip(&ast);
    let roundtrip = json_to_ast(&json).expect("ast json roundtrip");

    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        ..
    } = find_method_decl(&roundtrip, "Worker", "run")
    else {
        panic!("expected FunctionDeclaration");
    };
    assert_eq!(params, &vec!["size".to_string(), "tag".to_string()]);
    assert_eq!(
        param_decls,
        &vec![param("size", Some("usize")), param("tag", Some("i64"))]
    );
    assert_eq!(return_type_name.as_deref(), Some("usize"));
}
