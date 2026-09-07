#[test]
fn array_child_failure_keeps_allocation_and_only_prior_write() {
    let root = crate::parser::NyashParser::parse_from_string("[1, missing, 3]").unwrap();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("array_failure/0".into());
    let error = {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut builder,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            port.with_source_transport_v1(
                RawInvocationSourceTransportV1::script_root(root),
                |port, program| {
                    let ASTNode::Program { statements, .. } = program else {
                        panic!("Program")
                    };
                    port.lower_body(builder, statements)
                },
            )
        })
    }
    .unwrap_err();
    assert!(error.contains("Undefined variable: missing"), "{error}");
    let instructions: Vec<_> = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|b| &b.instructions)
        .collect();
    assert!(matches!(
        instructions.first(),
        Some(MirInstruction::NewBox {
            target: crate::mir::ConstructionTarget::IntrinsicArray,
            ..
        })
    ));
    assert_eq!(array_write_count(&builder), 1);
    assert!(!instructions.iter().any(|i| matches!(
        i,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(3),
            ..
        }
    )));
    assert!(!instructions.iter().any(|i| matches!(
        i,
        MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
    )));
}
