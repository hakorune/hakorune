#[test]
fn birth_receiver_non_escape_rejects_unproven_uses_before_row_publication() {
    use super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1;
    for body in [
        "return me",
        "other.value = me",
        "local alias = me other.value = alias",
        "other.accept(me)",
        "local alias = me other.accept(alias)",
        "me.accept(value)",
        "local nested = fn() { return me }",
        "local alias = me local nested = fn() { return alias }",
        "local alias = value if value { alias = me } other.value = alias",
        "local alias = me alias = value other.value = alias",
        "local aggregate = [me]",
    ] {
        let source = format!("box Page {{ value: i64 birth(value, other) {{ {body} }} }}");
        let error = issue_with_brand_catalog(&source).expect_err("unproven Birth receiver use");
        // Direct nested `me` is already stopped by the upstream resolver.
        // Keep it as dependency evidence, not as acceptance for the new check;
        // the separate alias-capture case below must reach ReceiverNonEscape.
        if body == "local nested = fn() { return me }" {
            assert!(
                matches!(
                    &error,
                    super::NormalCallableSemanticPackageIssueV1::InstanceConstructors {
                        _error: InstanceConstructorSemanticBatchIssueV1::Resolver(_),
                    }
                ) && format!("{error:?}").contains("body Me shape lacks exact receiver authority"),
                "wrong upstream boundary: {error:?}"
            );
            continue;
        }
        assert!(
            matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::InstanceConstructors {
                    _error: InstanceConstructorSemanticBatchIssueV1::ReceiverNonEscape { .. },
                }
            ),
            "wrong boundary for {body}: {error:?}"
        );
    }
}

#[test]
fn ordinary_new_rejects_value_birth_and_unselected_effect_contract() {
    use super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1;
    for (declaration, value_return) in [
        ("birth(value) { return value }", true),
        (
            "@rune Contract(no_alloc) birth(value) { local saved = value }",
            false,
        ),
    ] {
        let source = format!(
            "box Page {{ {declaration} }}
             static box Main {{ main() {{ local page = new Page(1) return 0 }} }}"
        );
        let error = issue_with_brand_catalog(&source).expect_err("unselected birth contract");
        match error {
            super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
                _error: OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit { .. },
            } => assert!(value_return),
            super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
                _error: OrdinaryNewCoSealIssueV1::BirthEffectUnsupported { .. },
            } => assert!(!value_return),
            other => panic!("wrong boundary: {other:?}"),
        }
    }
}

#[test]
fn instance_constructor_semantics_keep_parser_identity_and_nested_brand_relations() {
    let package = issue_with_brand_catalog(
        r#"
brand Id: i64

box Holder {
    init(value) {
        local direct = Id(value)
        local nested = fn(x) { Id(x) }
    }
    pack(other) {
        local second = Id(other)
    }
}
"#,
    )
    .expect("constructor semantic package");

    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].box_name(), "Holder");
    assert_eq!(rows[0].key(), "init/1");
    assert!(rows[0].source_id().same_as(rows[0].source_id()));
    assert_eq!(rows[0].forest().owner_count(), 2);
    assert_eq!(rows[1].key(), "pack/1");
    assert!(rows.iter().all(|row| row.published_birth_key().is_none()));
    assert_eq!(
        rows.iter()
            .flat_map(|row| row.forest().owners())
            .map(|(_, owner)| owner.brand_call_relations().count())
            .sum::<usize>(),
        3
    );
}

#[test]
fn birth_semantic_row_issues_distinct_non_global_publication_key() {
    let package = issue_with_brand_catalog("box Page { birth(value) { local saved = value } }")
        .expect("birth package");
    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 1);
    let key = rows[0]
        .published_birth_key()
        .expect("Birth owns a published key");
    assert_eq!(
        key.namespace(),
        hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
    );
    assert_eq!(key.owner(), "Page");
    assert_eq!(key.arity(), rows[0].source_arity());
    assert_eq!(key.mir_symbol_projection(), "Page.birth/1");
    assert!(key.canonical_global_target_v1().is_err());
    let completion = rows[0]
        .birth_completion()
        .expect("source-owned birth completion");
    assert_eq!(completion.owner(), rows[0].forest().roots()[0]);
    assert!(!completion.returns_value());
    package
        .with_normal_program_source_loan(|loan| {
            let input = rows[0].lowering_input(loan.program()).unwrap();
            let shape = input
                .body_shape()
                .expect("constructor events survive the loan");
            assert_eq!(shape.owner(), input.owner());
            assert_eq!(
                shape.body_root(),
                &input.function().root_profile().body_root()
            );
        })
        .unwrap();
}

#[test]
fn birth_fixed_source_retains_definition_through_normal_publication() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default(),
        )
        .expect("birth source with explicitly migrated i64 field contracts");
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed).expect("birth transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("birth fixture must retain source authority")
        };
        let result = crate::mir::MirCompiler::with_options(false)
            .compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    Some("apps/typed-object-birth-min/main.hako"),
                    Default::default(),
                ),
            )
            .expect("normal constructor publication");
        let object = result
            .module
            .canonical_object_definition(
                hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
            )
            .expect("source object definition survives the production collector");
        assert_eq!(object.diagnostic_name(), "Pair");
        assert_eq!(object.fields().len(), 2);
        assert_eq!(object.local_fields_for_layout().unwrap(), object.fields());
        assert!(object
            .fields()
            .iter()
            .all(|field| field.declared_type_name.as_deref() == Some("i64") && !field.is_weak));
        assert!(
            result.verification_result.is_ok(),
            "published source must retain valid field contracts: {:?}",
            result.verification_result
        );
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2);
        assert_eq!(
            result.module.canonical_callable_definition_symbol(&key),
            Some("Pair.birth/2")
        );
        let function = result.module.get_function("Pair.birth/2").unwrap();
        assert_eq!(
            function.root_ordinary_new_observation(),
            crate::mir::function::RootOrdinaryNewObservation::NotIssued
        );
        assert_eq!(function.signature.params.len(), 3);
        let contracts = &function.metadata.exact_numeric_runtime_check_contracts;
        assert_eq!(
            contracts.len(),
            2,
            "unannotated birth values require checks"
        );
        for contract in contracts {
            assert_eq!(contract.declared_type_name, "i64");
            assert_eq!(
                contract.kind,
                crate::mir::function::ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            );
            let block = &function.blocks[&contract.block];
            let (_, spanned) = block
                .all_spanned_instructions_enumerated()
                .find(|(index, _)| *index == contract.instruction_index)
                .unwrap();
            let crate::mir::MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::FieldSet { field, base, value },
                ..
            } = spanned.inst
            else {
                panic!("each check must remain bound to its actual fallible FieldSet")
            };
            let declaration = result.module.canonical_field_definition(*field).unwrap();
            assert_eq!(&declaration.name, &contract.field);
            assert_eq!(declaration.declared_type_name.as_deref(), Some("i64"));
            assert_eq!(*base, function.params[0]);
            assert_eq!(
                *value,
                function.params[field.declaration_ordinal() as usize + 1]
            );
            assert_eq!(value, &contract.value);
        }
        let mut fields: Vec<_> = contracts.iter().map(|row| row.field.as_str()).collect();
        fields.sort_unstable();
        assert_eq!(fields, ["left", "right"]);

        // The compatibility name map is not an authority for canonical stores.
        let mut without_name_map = result.module.clone();
        without_name_map.metadata.user_box_field_decls.clear();
        crate::mir::exact_numeric_field_contracts::refresh_module_exact_numeric_runtime_check_contracts(
            &mut without_name_map,
        );
        assert_eq!(
            without_name_map
                .get_function("Pair.birth/2")
                .unwrap()
                .metadata
                .exact_numeric_runtime_check_contracts
                .len(),
            2
        );
        let mut missing_check = result.module.clone();
        missing_check
            .get_function_mut("Pair.birth/2")
            .unwrap()
            .metadata
            .exact_numeric_runtime_check_contracts
            .clear();
        let findings = crate::mir::exact_numeric_field_contracts::collect_exact_numeric_field_assignment_findings(&missing_check);
        assert_eq!(findings.iter().filter(|finding| matches!(finding,
            crate::mir::exact_numeric_field_contracts::ExactNumericFieldAssignmentFinding::DynamicCheckRequired(_)
        )).count(), 2, "missing checks must not silently become admitted writes");

        let main = result
            .module
            .get_function("main")
            .expect("published Main entry");
        assert_eq!(
            main.root_ordinary_new_observation(),
            crate::mir::function::RootOrdinaryNewObservation::SourceCompleteAtFinalization
        );
        assert_eq!(main.signature.return_type, crate::mir::MirType::Integer);
        let mut read_count = 0;
        let mut add_count = 0;
        let mut birth_count = 0;
        for instruction in main
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
        {
            if let crate::mir::MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Call(call),
                ..
            } = instruction
            {
                if let crate::mir::Callee::BirthConstructor {
                    key: target,
                    receiver,
                } = &call.callee
                {
                    assert_eq!(target, &key);
                    assert_eq!(call.dst, None);
                    assert_eq!(call.args.len(), 2, "receiver is not a source argument");
                    assert!(main.metadata.value_types.contains_key(receiver));
                    for effect in [
                        crate::mir::Effect::WriteHeap,
                        crate::mir::Effect::ReadHeap,
                        crate::mir::Effect::Panic,
                        crate::mir::Effect::Barrier,
                    ] {
                        assert!(call.effects.contains(effect));
                    }
                    assert!(!call.effects.contains(crate::mir::Effect::Pure));
                    assert!(!call.effects.is_moveable());
                    birth_count += 1;
                }
            }
            let dst = match instruction {
                crate::mir::MirInstruction::ObjectFieldGet { dst, field, .. } => {
                    assert_eq!(field.object().declaration_index(), 0);
                    assert_eq!(field.declaration_ordinal() as usize, read_count);
                    let definition = result.module.canonical_field_definition(*field).unwrap();
                    assert_eq!(definition.declared_type_name.as_deref(), Some("i64"));
                    assert!(!definition.is_weak);
                    read_count += 1;
                    dst
                }
                crate::mir::MirInstruction::FieldGet { .. } => {
                    panic!("selected terminal must not recover a field by name")
                }
                crate::mir::MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    ..
                } => {
                    add_count += 1;
                    dst
                }
                _ => continue,
            };
            assert_eq!(
                main.metadata.value_types.get(dst),
                Some(&crate::mir::MirType::Integer)
            );
        }
        assert_eq!((read_count, add_count), (2, 1));
        assert_eq!(
            birth_count, 1,
            "selected birth uses the exact canonical carrier"
        );
        let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module)
            .expect("birth definition relation remains valid");
        assert_eq!(
            view.route(),
            crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject
        );
        let output_dir = tempfile::tempdir().expect("temporary artifact directory");
        let exe = output_dir.path().join("birth");
        let error = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
            &result.module,
            exe.to_str().unwrap(),
            None,
            None,
        )
        .expect_err("unimplemented Birth consumer must not retry compatibility");
        assert!(error.contains("UnsupportedBeforeObject"));
        assert_eq!(std::fs::read_dir(output_dir.path()).unwrap().count(), 0);
        let static_key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "Pair", "birth", 2,
        );
        assert!(result
            .module
            .canonical_callable_definition_symbol(&static_key)
            .is_none());
    });
}
