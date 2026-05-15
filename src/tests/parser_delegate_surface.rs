use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::tests::helpers::parser::{find_box, parse_ok};

#[test]
fn parser_delegate_surface_parses_explicit_exposes_list() {
    let ast = parse_ok(
        r#"
box P2PBox {
    connect() {
        return 1
    }

    send(value) {
        return value
    }
}

box MeshNode {
    p2p: P2PBox = new P2PBox()

    delegate p2p exposes {
        connect
        send as p2pSend
    }
}
"#,
    );
    let decl = find_box(&ast, "MeshNode");

    let ASTNode::BoxDeclaration {
        fields,
        methods,
        delegates,
        ..
    } = decl
    else {
        panic!("expected box declaration");
    };

    assert_eq!(fields, &vec!["p2p".to_string()]);
    assert_eq!(delegates.len(), 1);
    assert_eq!(delegates[0].field_name, "p2p");
    assert_eq!(delegates[0].exposes.len(), 2);
    assert_eq!(delegates[0].exposes[0].source_name, "connect");
    assert_eq!(delegates[0].exposes[0].exposed_name, "connect");
    assert_eq!(delegates[0].exposes[1].source_name, "send");
    assert_eq!(delegates[0].exposes[1].exposed_name, "p2pSend");
    assert!(methods.contains_key("connect"));
    assert!(methods.contains_key("p2pSend"));
    let ASTNode::FunctionDeclaration { params, body, .. } = &methods["p2pSend"] else {
        panic!("generated forwarding method");
    };
    assert_eq!(params.as_slice(), ["value".to_string()].as_slice());
    let [ASTNode::Return {
        value: Some(value), ..
    }] = body.as_slice()
    else {
        panic!("forwarding method should return delegated call");
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = value.as_ref()
    else {
        panic!("forwarding return should call delegate target");
    };
    assert_eq!(method, "send");
    assert_eq!(arguments.len(), 1);
    assert!(matches!(&arguments[0], ASTNode::Variable { name, .. } if name == "value"));
    assert!(matches!(
        object.as_ref(),
        ASTNode::FieldAccess { object, field, .. }
            if field == "p2p" && matches!(object.as_ref(), ASTNode::Me { .. })
    ));
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

#[test]
fn parser_delegate_surface_rejects_local_method_collision() {
    NyashParser::parse_from_string(
        r#"
box P2PBox {
    connect() {
        return 1
    }
}

box MeshNode {
    p2p: P2PBox
    delegate p2p exposes {
        connect
    }
    connect() {
        return 0
    }
}
"#,
    )
    .expect_err("delegate exposed method must not collide with local method");
}
