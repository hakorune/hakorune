use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;

fn package(
    main: &str,
    methods: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} {methods} }} static box Main {{ main() {{ local recv = new Transport() {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("source-backed borrowed discard package")
}

fn original_request(text: &str) -> crate::mir::NormalCompileRequestV1 {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text, crate::parser::ParserBuildConfig::default()).unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", ||
            crate::r#macro::transform_normal_callable_program_v1(parsed)).unwrap()
        else { panic!("original source"); };
    crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
}

fn bool_discard_source(returned: &str) -> String {
    format!("box Token {{ value: i64 birth(v) {{ me.value = v }} }}
        box Transport {{ birth() {{ }}
            make(flag) {{ if flag <= 0 {{ return null }} return new Token(1) }}
            test(token) {{ return {returned} }}
        }}
        static box Main {{ main() {{
            local recv = new Transport()
            local arr = new ArrayBox()
            arr.push(recv.make(1))
            recv.test(arr.get(0))
            return 0
        }} }}")
}

#[test]
fn borrowed_statement_discard_retains_original_actuals_without_local_destination() {
    for argument in ["true", "-7", "recv"] {
        let package = package(&format!("recv.probe({argument}) return 0"), "");
        let ledger = &package.ordinary_new_claim_ledger;
        let completion = ledger.root_completion_for_test();
        let flow = completion
            .cleanup()
            .root_flow()
            .expect("complete caller flow");
        assert_eq!(flow.local_calls().len(), 1);
        let call = &flow.local_calls()[0];
        assert!(call.local_binding().is_none(), "discard creates no local");
        assert_eq!(call.result(), LocalCallResultClassV1::I64);
        assert_eq!(call.prior_homes().len(), 1, "only the receiver Home exists");
        let pending = ledger.borrowed_formal_actuals[call.site()]
            .as_ref()
            .unwrap();
        assert_eq!(call.arguments(), pending.ordered_arguments.as_ref());
        assert!(matches!(
            call.arguments(),
            [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
        ));
        let taken = ledger
            .take_lexical_instance_call(call.owner(), call.site().site())
            .unwrap()
            .expect("same affine disposition");
        assert!(std::ptr::eq(
            ledger.borrowed_call_actuals_v1(&taken).unwrap().unwrap(),
            pending.opaque_actuals.as_ref()
        ));
        assert!(
            ledger
                .take_lexical_instance_call(call.owner(), call.site().site())
                .is_err(),
            "one-shot"
        );
    }
    for returned in ["true", "false"] {
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            &bool_discard_source(returned),
        ).expect("original object Array argument Bool source");
        let ledger = &package.ordinary_new_claim_ledger;
        let flow = ledger.root_completion_for_test().cleanup().root_flow().unwrap();
        let calls: Vec<_> = flow.local_calls().iter()
            .filter(|call| call.result() == LocalCallResultClassV1::Bool).collect();
        let [observation] = calls.as_slice() else { panic!("one original Bool discard"); };
        assert_eq!(observation.result(), LocalCallResultClassV1::Bool);
        assert!(observation.local_binding().is_none());
        let prefixes = flow.lifecycle_prefixes(&[observation.site().clone()]).unwrap();
        assert!(prefixes.iter().any(|prefix| prefix.site() == observation.site()));
        assert!(!flow.lifecycle_prefixes(&[]).unwrap().iter()
            .any(|prefix| prefix.site() == observation.site()), "source observation alone never selects a route");
        let original = observation.borrowed_scalar().expect("dedicated original Bool loan");
        let mut row = ledger.take_lexical_instance_call(observation.owner(), observation.site().site()).unwrap().unwrap();
        assert_eq!(row.scalar_result_source_v1(), Some(original));
        let (selected, signatures) = ledger.original_instance_input_issuance.as_ref().unwrap();
        let arguments = ledger.checked_completed_original_instance_target_arguments_v1(
            row.source_target(), selected, ledger.original_parameter_contracts.as_deref().unwrap(), signatures,
        ).unwrap();
        assert!(arguments.is_some(), "same original Bool input must be executable");
        assert_eq!(ledger.checked_local_scalar_call_result_v1(&row).unwrap(), crate::mir::instruction::InvokeCallResultKind::Bool);
        row.result = Some(crate::mir::instruction::InvokeCallResultKind::I64);
        assert!(ledger.checked_local_scalar_call_result_v1(&row).unwrap_err().contains("result-drift"));
        row.result = Some(crate::mir::instruction::InvokeCallResultKind::Bool);
        let retained = row.scalar_result.take();
        assert!(ledger.checked_local_scalar_call_result_v1(&row).unwrap_err().contains("source-missing-or-drift"));
        row.scalar_result = retained;
        ledger.checked_local_scalar_call_result_v1(&row).unwrap();
        let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            "static box Main { main() { return 0 } }",
        ).unwrap();
        assert!(ledger.checked_root_array_holder_caller_v1(observation.owner(), &foreign.selected,
            ledger.original_parameter_contracts.as_deref().unwrap()).unwrap_err()
            .contains("original-issuance-drift"));
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let key = ledger.app_main_catalog_key.take();
        assert!(ledger.owned_array_lifecycle_recipes_v1(&package.result_contracts)
            .unwrap_err().contains("caller-key-unavailable"));
        ledger.app_main_catalog_key = key;
        let source = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
        let owner = source.app_main_owner.take();
        assert!(ledger.owned_array_lifecycle_recipes_v1(&package.result_contracts)
            .unwrap_err().contains("root-caller-identity"));
        ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap().app_main_owner = owner;
        ledger.owned_array_lifecycle_recipes_v1(&package.result_contracts).unwrap();
    }
    for body in ["", "local scratch = 0", "return", "if true { return } local scratch = 0"] {
        let package = package("recv.touch() return 0", &format!("touch() {{ {body} }}"));
        let ledger = &package.ordinary_new_claim_ledger;
        let flow = ledger.root_completion_for_test().cleanup().root_flow().expect("Unit statement source flow");
        let [call] = flow.local_calls() else { panic!("one original Unit statement: {body}"); };
        assert_eq!(call.result(), LocalCallResultClassV1::Unit);
        assert!(call.local_binding().is_none() && !call.is_expression_value());
        assert!(call.arguments().is_empty() && call.borrowed_scalar().is_none());
        assert_eq!(call.prior_homes().len(), 1);
        let original = call.unit_source().expect("same original Unit target");
        assert_eq!(original.call_site(), call.site());
        assert!(original.is_unit_discard_source_v1());
        assert!(ledger.lexical_instance_call_covered(call.owner(), call.site().site()));
        let mut row = ledger.take_lexical_instance_call(call.owner(), call.site().site()).unwrap().unwrap();
        assert_eq!(row.result(), Some(crate::mir::instruction::InvokeCallResultKind::Unit));
        assert_eq!(row.source_target(), original);
        ledger.corroborate_unit_discard_v1(&row).unwrap();
        let completion = ledger.unit_completion_source_v1(original.callee_owner()).unwrap().expect("retained original Unit completion");
        for exit in completion.explicit_sites() {
            assert!(matches!(ledger.terminal_relation_for_owner_at(completion.owner(), exit), Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Unit(_))));
            assert!(ledger.prepare_terminal_unit_return(completion.owner(), exit.node()).unwrap());
        }
        let retained = row.unit_completion.take();
        assert!(ledger.corroborate_unit_discard_v1(&row).unwrap_err().contains("retained-completion-drift"));
        row.unit_completion = retained;
        ledger.corroborate_unit_discard_v1(&row).unwrap();
        assert!(ledger.take_lexical_instance_call(call.owner(), call.site().site()).is_err());
    }

    crate::runtime::ring0::ensure_global_ring0_initialized();
    for (case, (parameters, prefix, arguments, body)) in [
        ("n: i64", "", "7", "print(n)"),
        ("left: i64, right: i64", "local before = 1", "before, 7", "local scratch = left print(right)"),
        ("", "", "", "print(7)"), ("", "", "", "print(7) return"),
        ("", "", "", "local a = new Transport() local b = new Transport() print(7)"),
        ("", "", "", "if true { print(7) return } print(7)")].into_iter().enumerate() {
        let arity = if parameters.is_empty() { 0 } else { parameters.split(',').count() };
        let text = format!("box Transport {{ birth() {{ }} touch({parameters}) {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() {prefix} recv.touch({arguments}) return 0 }} }}");
        let request = original_request(&text);
        let compiled = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::mir::MirCompiler::with_options(false).compile_normal_with_published(request, |view, _| {
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let functions = json["functions"].as_array().unwrap();
                let callee = functions.iter().find(|row| row["name"] == format!("Transport.touch/{arity}")).expect("real Unit callee");
                assert_eq!(callee["role"], "ordinary_unit");
                let calls: Vec<_> = functions.iter().flat_map(|function| function["blocks"].as_array().unwrap())
                    .map(|block| &block["terminator"]["instruction"])
                    .filter(|row| row["operation"]["kind"] == "ordinary_call" && row["operation"]["result"] == "unit")
                    .collect();
                assert_eq!(calls.len(), 1, "one real Unit ordinary call: {wire}");
                assert!(calls[0]["operation"]["call"]["dst"].is_null());
                std::fs::write(format!("/tmp/hako-issued-borrowed-unit-{case}.json"), &wire).unwrap();
                Ok(())
            })
        });
        if let Err(error) = compiled { panic!("original ordinary Unit discard must reach final publication ({body}): {error}"); }
    }

    for returned in ["true", "false"] {
        let text = bool_discard_source(returned);
        let request = original_request(&text);
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::mir::MirCompiler::with_options(false).compile_normal_with_published(request, |view, _| {
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let functions = json["functions"].as_array().unwrap();
                let rows = view.validated_named_arrays()?;
                let holder = rows.iter().find_map(|row| match row {
                    crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1::OwnedHolder(holder) => Some(holder),
                    _ => None,
                }).expect("original root Array holder");
                assert!(view.module().canonical_callable_definition_symbol(holder.caller()).is_none());
                let symbol = holder.finished_symbol_v1()?;
                assert_eq!(holder.validate(view.module())?, symbol);
                let mut missing = view.module().clone();
                missing.functions.remove(symbol);
                assert!(holder.validate(&missing).unwrap_err().contains("function-missing"));
                let mut renamed = view.module().clone();
                renamed.functions.get_mut(symbol).unwrap().signature.name.push_str("-drift");
                assert!(holder.validate(&renamed).unwrap_err().contains("function-identity"));
                let mut foreign = view.module().clone();
                let mut function = foreign.functions[symbol].clone();
                function.signature.name = holder.caller().mir_symbol_projection();
                foreign.add_cataloged_box_method(holder.caller().clone(), function).unwrap();
                assert!(holder.validate(&foreign).unwrap_err().contains("canonical-caller-mismatch"));
                let callee = functions.iter().find(|row| row["name"] == "Transport.test/1").expect("real Bool callee");
                assert_eq!(callee["role"], "ordinary_bool");
                let calls: Vec<_> = functions.iter().flat_map(|function| function["blocks"].as_array().unwrap())
                    .map(|block| &block["terminator"]["instruction"])
                    .filter(|row| row["operation"]["kind"] == "ordinary_call" && row["operation"]["result"] == "bool")
                    .collect();
                assert_eq!(calls.len(), 1, "one real Bool call, no I64 coercion: {wire}");
                std::fs::write(format!("/tmp/hako-issued-borrowed-bool-discard-{returned}.json"), wire).unwrap();
                Ok(())
            })
        }).unwrap_or_else(|error| panic!("original Bool discard must publish ({returned}): {error}"));
    }

    let text = "box Transport { marker: i64 birth() { me.marker = 0 } touch() { print(7) } }
        box Pair { left: Transport right: Transport birth() {
            me.left = new Transport() me.right = new Transport() me.left.touch() me.right.touch()
        } } static box Main { main() { local pair = new Pair() return 0 } }";
    let request = original_request(text);
    let compiled = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::mir::MirCompiler::with_options(false).compile_normal_with_published(request, |view, _| {
            let input = view.issue_lifecycle_physical_abi_input()?;
            let program = input.program();
            let source = program.handoff().root_source().expect("same original source");
            let mut calls = 0;
            for function in program.functions() {
                let crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = function.role()
                    else { continue; };
                let caller = view.module().functions.get(function.name()).unwrap();
                for block in function.blocks() {
                    let row = block.terminator();
                    if !matches!(row.instruction(), crate::mir::MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::Call {
                            call, result: crate::mir::instruction::InvokeCallResultKind::Unit }, .. }
                        if matches!(call.callee, crate::mir::Callee::SameModuleInstance { .. })) { continue; }
                    let coordinate = (block.id(), row.index() as usize);
                    source.corroborate_constructor_unit_call_v1(view.module(), abi.target(), caller,
                        coordinate, row.instruction())?;
                    assert!(source.corroborate_constructor_unit_call_v1(view.module(), abi.target(), caller,
                        (coordinate.0, coordinate.1 + 1), row.instruction()).is_err());
                    for mutation in 0..3 {
                        let mut drift = row.instruction().clone();
                        let crate::mir::MirInstruction::Invoke { operation, normal_landing, fault_landing, .. } = &mut drift
                            else { unreachable!() };
                        match mutation {
                            0 => *normal_landing = *fault_landing,
                            1 => *fault_landing = *normal_landing,
                            _ => {
                                let crate::mir::instruction::InvokeOperation::Call { result, .. } = operation else { unreachable!() };
                                *result = crate::mir::instruction::InvokeCallResultKind::I64;
                            }
                        }
                        assert!(source.corroborate_constructor_unit_call_v1(view.module(), abi.target(), caller,
                            coordinate, &drift).is_err(), "constructor Unit mutation={mutation}");
                    }
                    calls += 1;
                }
            }
            assert_eq!(calls, 2, "both original stored-child Unit calls");
            let wire = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
            std::fs::write("/tmp/hako-issued-constructor-unit.json", wire).unwrap();
            Ok(())
        })
    });
    if let Err(error) = compiled { panic!("original constructor Unit source must reach final publication: {error}"); }

}

#[test]
fn strict_statement_call_does_not_acquire_borrowed_discard_membership() {
    let package = package(
        "recv.strict(7) return 0",
        "strict(p: i64): i64 { return p }",
    );
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    assert!(ledger.borrowed_formal_actuals.is_empty());
    assert!(completion
        .cleanup()
        .root_flow()
        .map_or(true, |flow| flow.local_calls().is_empty()));
}

#[test]
fn selected_discard_failure_is_terminal_without_strict_retry() {
    for argument in ["%{\"key\"=>1}", "1.5"] {
        let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() recv.probe({argument}) return 0 }} }}");
        let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source)
            .err().expect("selected unsupported discard must reject");
        assert!(
            format!("{error:?}").contains("borrowed-actual/unsupported-or-unavailable"),
            "{error:?}"
        );
    }
    for use_site in ["local value = recv.touch() return 0", "return recv.touch()"] {
        let source = format!("box Transport {{ birth() {{ }} touch() {{ print(7) }} }} static box Main {{ main() {{ local recv = new Transport() {use_site} }} }}");
        let request = original_request(&source);
        let compiled = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::mir::MirCompiler::with_options(false).compile_normal_with_published(request, |view, _|
                view.issue_lifecycle_physical_abi_input().map(|_| ()))
        });
        let error = compiled.err().expect("original Unit source cannot acquire a value result");
        assert!(error.contains("unit-value-use"), "{use_site}: {error}");
    }

}

#[test]
fn branch_discards_keep_distinct_sites_and_only_their_own_exit_groups() {
    let package = package(
        "if true { recv.probe(true) return 0 } else { recv.probe(false) return 0 }",
        "",
    );
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion
        .cleanup()
        .root_flow()
        .expect("complete branch flow");
    assert!(flow.all_exits_ready());
    assert_eq!(completion.explicit_sites().len(), 2);
    assert_eq!(flow.local_calls().len(), 2);
    assert_ne!(flow.local_calls()[0].site(), flow.local_calls()[1].site());
    assert!(flow
        .local_calls()
        .iter()
        .all(|call| call.local_binding().is_none()));
    let mut covered = std::collections::BTreeSet::new();
    for exit in completion.explicit_sites() {
        let row = flow.exit_row(exit).unwrap().unwrap();
        assert_eq!(row.homes().len(), 1, "only the original receiver Home");
        assert_eq!(row.covered_calls().len(), 1, "branch-local call group");
        assert!(covered.insert(row.covered_calls()[0].clone()));
        let call = flow
            .local_calls()
            .iter()
            .find(|call| call.site() == &row.covered_calls()[0])
            .unwrap();
        assert_eq!(call.prior_homes(), row.homes());
    }
    assert_eq!(covered.len(), 2);
}
