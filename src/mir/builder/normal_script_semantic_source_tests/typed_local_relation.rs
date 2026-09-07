#[test]
fn typed_local_input_rejects_annotation_site_and_initializer_drift() {
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    };
    use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
    use crate::mir::builder::stmts::RawLegacyLocalInputV1;
    use crate::mir::resolved_semantics::ExprChildRoleV1;
    let ast = NyashParser::parse_from_string("local a: Array<i64> = [10, 20]").unwrap();
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(ScriptSyntaxViewV1::from_program(&ast).unwrap(), &window)
        .unwrap()
    else {
        panic!("typed literal source must Complete")
    };
    let relation = owner
        .expression_source()
        .initializers()
        .next()
        .unwrap()
        .clone();
    let (program, root) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::script_root(ast),
    );
    let ASTNode::Program { mut statements, .. } = program else {
        panic!("Program")
    };
    let (statement, context) =
        RawInvocationSourceContextV1::from_transport(root.body_statement(statements.remove(0), 0));
    let site = context.site().unwrap();
    let initializer = PreparedRawChildSourceV1::Exact(
        context
            .child_expression(&statement, ExprChildRoleV1::LocalInitializer(0))
            .unwrap(),
    );
    assert!(RawLegacyLocalInputV1::from_script_relation(
        statement.clone(),
        relation.clone(),
        site,
        Some(&initializer),
    )
    .is_ok());
    assert!(RawLegacyLocalInputV1::from_script_relation(
        statement.clone(),
        relation.clone(),
        site,
        None,
    )
    .is_err());
    assert!(RawLegacyLocalInputV1::from_script_relation(
        statement.clone(),
        relation.clone(),
        root.site().unwrap(),
        Some(&initializer),
    )
    .is_err());
    assert!(RawLegacyLocalInputV1::from_script_relation(
        statement.clone(),
        relation.clone(),
        site,
        Some(&PreparedRawChildSourceV1::Preserve),
    )
    .is_err());
    let mut drifted = statement.clone();
    let ASTNode::Local {
        declared_type_names,
        ..
    } = &mut drifted
    else {
        unreachable!()
    };
    declared_type_names[0] = Some("Array<u8>".into());
    assert!(RawLegacyLocalInputV1::from_script_relation(
        drifted,
        relation.clone(),
        site,
        Some(&initializer),
    )
    .is_err());
    let mut drifted = statement;
    let ASTNode::Local { initial_values, .. } = &mut drifted else {
        unreachable!()
    };
    initial_values[0] = None;
    assert!(RawLegacyLocalInputV1::from_script_relation(
        drifted,
        relation,
        site,
        Some(&initializer),
    )
    .is_err());
}
