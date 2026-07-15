use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

use super::*;

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: vec!["x".to_string()],
        param_decls: vec![ParamDecl {
            name: "x".to_string(),
            declared_type_name: Some("i64".to_string()),
        }],
        return_type_name: Some("i64".to_string()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

#[test]
fn owns_one_function_only_program_and_exposes_located_headers() {
    let source = program(vec![
        function("first", Vec::new()),
        function("second", Vec::new()),
    ]);
    let unit = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(source).unwrap();

    assert_eq!(unit.declaration_sites().len(), 2);
    let first = unit.located_header(unit.declaration_sites()[0]).unwrap();
    let second = unit.located_header(unit.declaration_sites()[1]).unwrap();
    assert_eq!(first.site().statement_index(), 0);
    assert_eq!(first.header().name(), "first");
    assert_eq!(second.site().statement_index(), 1);
    assert_eq!(second.header().name(), "second");
}

#[test]
fn validates_only_the_top_level_surface_and_does_not_read_bodies() {
    let unsupported_body = vec![ASTNode::UsingStatement {
        namespace_name: "body_only".to_string(),
        span: Span::unknown(),
    }];
    let source = program(vec![function("f", unsupported_body)]);
    let unit = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(source).unwrap();

    assert_eq!(unit.declaration_sites().len(), 1);
    assert_eq!(
        unit.located_header(unit.declaration_sites()[0])
            .unwrap()
            .header()
            .name(),
        "f"
    );
}

#[test]
fn rejects_non_program_empty_and_mixed_top_level_surfaces() {
    let bare = function("f", Vec::new());
    assert_eq!(
        VerifiedCallableHeaderSourceUnitV1::seal_header_surface(bare).unwrap_err(),
        CallableModuleHeaderSyntaxErrorV1::RootMustBeProgram {
            actual: "FunctionDeclaration"
        }
    );

    assert_eq!(
        VerifiedCallableHeaderSourceUnitV1::seal_header_surface(program(Vec::new())).unwrap_err(),
        CallableModuleHeaderSyntaxErrorV1::EmptyCatalog
    );

    let mixed = program(vec![
        function("f", Vec::new()),
        ASTNode::UsingStatement {
            namespace_name: "other".to_string(),
            span: Span::unknown(),
        },
    ]);
    let error = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(mixed).unwrap_err();
    let CallableModuleHeaderSyntaxErrorV1::UnsupportedProgramStatement { site, actual } = error
    else {
        panic!("expected unsupported Program statement")
    };
    assert_eq!(site.statement_index(), 1);
    assert_eq!(actual, "UsingStatement");
}

#[test]
fn does_not_validate_callable_profile_or_issue_owners_in_s0() {
    let mut instance = function("main", Vec::new());
    let ASTNode::FunctionDeclaration {
        is_static,
        param_decls,
        ..
    } = &mut instance
    else {
        unreachable!()
    };
    *is_static = false;
    param_decls[0].declared_type_name = None;

    let unit =
        VerifiedCallableHeaderSourceUnitV1::seal_header_surface(program(vec![instance])).unwrap();
    let header = unit.located_header(unit.declaration_sites()[0]).unwrap();
    assert_eq!(header.header().name(), "main");
    assert!(!header.header().is_static());
}
