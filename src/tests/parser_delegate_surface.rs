use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn first_decl(source: &str) -> ASTNode {
    let ast = NyashParser::parse_from_string(source).expect("parse delegate source");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements.into_iter().next().expect("first statement")
}

#[test]
fn parser_delegate_surface_parses_explicit_exposes_list() {
    let decl = first_decl(
        r#"
box MeshNode {
    p2p: P2PBox = new P2PBox()

    delegate p2p exposes {
        connect
        send as p2pSend
    }
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields, delegates, ..
    } = decl
    else {
        panic!("expected box declaration");
    };

    assert_eq!(fields, vec!["p2p"]);
    assert_eq!(delegates.len(), 1);
    assert_eq!(delegates[0].field_name, "p2p");
    assert_eq!(delegates[0].exposes.len(), 2);
    assert_eq!(delegates[0].exposes[0].source_name, "connect");
    assert_eq!(delegates[0].exposes[0].exposed_name, "connect");
    assert_eq!(delegates[0].exposes[1].source_name, "send");
    assert_eq!(delegates[0].exposes[1].exposed_name, "p2pSend");
}

#[test]
fn parser_delegate_surface_rejects_empty_exposes_list() {
    NyashParser::parse_from_string(
        r#"
box MeshNode {
    p2p: P2PBox
    delegate p2p exposes {}
}
"#,
    )
    .expect_err("empty delegate exposes list must reject");
}
