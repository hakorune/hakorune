use crate::ast::{
    ASTNode, BoxMethodCompatibilityOriginV1, BoxMethodGeneratedProvenanceV1, BoxMethodProvenanceV1,
    BoxMethodSourceSelectionV1, DelegateDeclarationProvenanceV1,
};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};
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
    assert!(delegates[0].source_member_ordinal().is_some());
    assert!(methods.contains_name("connect"));
    assert!(methods.contains_name("p2pSend"));
    let generated = methods.get("p2pSend").expect("generated delegate row");
    assert!(matches!(
        generated.provenance(),
        BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate {
            field_name,
            exposed_name,
            selection: BoxMethodSourceSelectionV1::Direct,
        }) if field_name.as_ref() == "p2p" && exposed_name.as_ref() == "p2pSend"
    ));
    assert!(methods
        .iter_selected_declaration_order()
        .all(|entry| !matches!(
            entry.provenance(),
            BoxMethodProvenanceV1::CompatibilityOnly { .. }
        )));
    let ASTNode::FunctionDeclaration { params, body, .. } =
        methods.get_declaration("p2pSend").unwrap()
    else {
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

#[test]
fn parser_delegate_batch_rejects_second_row_collision_without_compatibility_fallback() {
    let error = NyashParser::parse_from_string(
        r#"
box Target {
    first() { return 1 }
    second() { return 2 }
}
box Host {
    target: Target
    secondAlias() { return 0 }
    delegate target exposes {
        first as firstAlias
        second as secondAlias
    }
}
"#,
    )
    .expect_err("the complete delegate batch must reject before publication");

    let message = error.to_string();
    assert!(message.contains("delegate method batch conflicts"));
    assert!(message.contains("secondAlias"));
}

#[test]
fn legacy_json_delegate_remains_compatibility_only() {
    let ast = parse_ok(
        r#"
box Target {
    run() { return 1 }
}
box Host {
    target: Target
    delegate target exposes { run }
}
"#,
    );
    let json = crate::r#macro::ast_json::ast_to_json_roundtrip(&ast);
    let decoded = crate::r#macro::ast_json::json_to_ast(&json).expect("legacy JSON decode");
    let ASTNode::BoxDeclaration { delegates, .. } = find_box(&decoded, "Host") else {
        panic!("expected Host BoxDeclaration")
    };
    assert!(matches!(
        delegates[0].provenance(),
        DelegateDeclarationProvenanceV1::CompatibilityOnly {
            origin: BoxMethodCompatibilityOriginV1::LegacyJsonV1,
        }
    ));
    assert!(delegates[0].explicit_source_selection().is_none());
    assert!(delegates[0].source_member_ordinal().is_none());
}

#[test]
fn selected_delegate_retains_exact_gate_and_branch_member_ordinals() {
    let ast = NyashParser::parse_from_string_with_build_config(
        r#"
box Target {
    run() { return 1 }
}
box Host {
    target: Target
    gate Build.test {
        branch_field: i64
        delegate target exposes { run }
    } else {
        branch_field: i64
        delegate target exposes { run }
    }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("selected delegate fixture parses");
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Host") else {
        panic!("expected Host BoxDeclaration")
    };
    let entry = methods.get("run").expect("generated delegate method");
    let BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate {
        selection: BoxMethodSourceSelectionV1::SelectedBuildGate { path },
        ..
    }) = entry.provenance()
    else {
        panic!("delegate method must retain selected provenance")
    };
    assert_eq!(path.len(), 1);
    assert_eq!(path[0].gate_site().box_member_ordinal(), 1);
    assert_eq!(path[0].branch_member_ordinal(), 1);
}
