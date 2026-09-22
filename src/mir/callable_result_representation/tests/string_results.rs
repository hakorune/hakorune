use super::super::{
    CallableResultUnavailableReasonV1, StaticCallResultPublicationTakeV1,
    VerifiedCallableResultDispositionV1 as Disposition,
    VerifiedCallableResultRepresentationV1 as Representation,
    VerifiedStaticCallResultPublicationOwnerV1,
};
use super::support::{
    declarations, disposition, extend_current_owner_targets, key, qualified_targets,
    seal_with_targets, site, CallSiteSpecV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1 as Path;

#[test]
fn string_results_preserve_operator_and_merge_boundaries() {
    let source = r#"static box Strings {
        literal() { return "猫" }
        add(x) { return "-" + x }
        local_copy() { local text = "012".substring(0, 1) return text + "x" }
        branch(x) { if x == 0 { return "0" } return "x" }
        looped(x) {
            local out = ""
            loop(x > 0) { out = "012".substring(0, 1) + out x = x - 1 }
            return out
        }
        mixed(x) { if x == 0 { return "0" } return 1 }
        loop_mixed(x) { local out = "" loop(x > 0) { out = 1 break } return out }
        minus() { return -"x" }
        subtract() { return "x" - 1 }
        right_only(x) { return x + "x" }
        receiver(x) { return x.substring(0, 1) }
    }"#;
    for (name, arity) in [
        ("literal", 0),
        ("add", 1),
        ("local_copy", 0),
        ("branch", 1),
        ("looped", 1),
    ] {
        assert_eq!(
            disposition(source, "Strings", name, arity),
            Disposition::ExactString,
            "{name}"
        );
    }
    for (name, arity) in [
        ("mixed", 1),
        ("loop_mixed", 1),
        ("minus", 0),
        ("subtract", 0),
        ("right_only", 1),
        ("receiver", 1),
    ] {
        assert!(
            matches!(
                disposition(source, "Strings", name, arity),
                Disposition::Unavailable(_)
            ),
            "{name}"
        );
    }
}

#[test]
fn natural_recursive_helpers_seal_string_and_keep_recursive_call_rows() {
    let source = format!(
        "{}\n{}",
        include_str!("../../../../lang/src/shared/common/string_helpers.hako"),
        include_str!("../../../../lang/src/compiler/parser/scan/parser_string_utils_box.hako")
    );
    let declarations = declarations(&source);
    let recursive_site = site(vec![Path::Body(2), Path::IfThen(0), Path::Value, Path::Rhs]);
    let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .unwrap();
    let inventory =
        crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
            &declarations,
            &imports,
        )
        .unwrap();
    assert!(inventory
        .first_method_observation_unavailability()
        .is_none());
    let targets = inventory.into_targets();
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .unwrap();
    assert!(owner.pending_selected_len() >= 2);
    for (box_name, method) in [
        ("ParserStringUtilsBox", "i2s"),
        ("StringHelpers", "int_to_str"),
    ] {
        let caller = key(&declarations, box_name, method, 1);
        assert_eq!(
            results.disposition(&caller),
            Some(&Disposition::ExactString)
        );
        let row = results
            .call_result(&caller, &recursive_site)
            .expect("recursive RHS survives stable pass");
        assert_eq!(row.static_target_key(), Some(&caller));
        assert_eq!(row.result_representation(), Representation::ExactString);
        let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
            .take_for_source(&declarations, &caller, &recursive_site)
            .unwrap()
        else {
            panic!("String must select general publication")
        };
        assert_eq!(handoff.representation(), &Representation::ExactString);
        assert!(owner
            .take_for_source(&declarations, &caller, &recursive_site)
            .is_err());
    }
    for ((caller, call_site), _) in targets.rows() {
        if call_site == &recursive_site
            && matches!(
                (caller.owner(), caller.name()),
                ("ParserStringUtilsBox", "i2s") | ("StringHelpers", "int_to_str")
            )
        {
            continue;
        }
        owner
            .take_for_source(&declarations, caller, call_site)
            .unwrap();
    }
    owner.finish_empty().unwrap();
}

#[test]
fn unproved_recursive_result_still_rejects() {
    let declarations = declarations("static box Cycle { run(x) { return me.run(x) } }");
    let call_site = site(vec![Path::Body(0), Path::Value]);
    let targets = extend_current_owner_targets(
        qualified_targets(&declarations, &[], &[]),
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "Cycle",
            caller_name: "run",
            caller_arity: 1,
            site: call_site,
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    assert_eq!(
        results.disposition(&key(&declarations, "Cycle", "run", 1)),
        Some(&Disposition::Unavailable(
            CallableResultUnavailableReasonV1::RecursiveDependency
        ))
    );
}

#[test]
fn pending_local_string_receiver_waits_for_callee_before_sealing() {
    let declarations = declarations(
        r#"static box ReceiverChain {
        a_consumer() { local text = me.z_producer() return text.substring(0,1) }
        z_producer() { return "猫" }
    }"#,
    );
    let call_site = site(vec![Path::Body(0), Path::Initializer(0)]);
    let targets = extend_current_owner_targets(
        qualified_targets(&declarations, &[], &[]),
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "ReceiverChain",
            caller_name: "a_consumer",
            caller_arity: 0,
            site: call_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "ReceiverChain", "a_consumer", 0);
    assert_eq!(
        results.disposition(&caller),
        Some(&Disposition::ExactString)
    );
    assert_eq!(
        results
            .call_result(&caller, &call_site)
            .unwrap()
            .result_representation(),
        Representation::ExactString
    );
    let substring_site = site(vec![Path::Body(1), Path::Value]);
    assert_eq!(
        results
            .call_result(&caller, &substring_site)
            .unwrap()
            .result_representation(),
        Representation::ExactString
    );
}

#[test]
fn string_add_is_stable_across_pending_to_mixed_local_transitions() {
    let methods = [
        r#"a_right(flag) { local v = 1 if flag > 0 { v = me.z_text() } return "-" + v }"#,
        r#"b_left(flag) { local v = 1 if flag > 0 { v = "mixed" } local text = me.z_text() return text + v }"#,
        r#"z_text() { return "x" }"#,
    ];
    for reverse in [false, true] {
        let ordered = if reverse {
            methods.iter().rev().copied().collect::<Vec<_>>()
        } else {
            methods.to_vec()
        };
        let declarations = declarations(&format!("static box Stable {{ {} }}", ordered.join("\n")));
        let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            std::iter::empty::<(String, String)>(),
        )
        .unwrap();
        let targets =
            crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
                &declarations,
                &imports,
            )
            .unwrap()
            .into_targets();
        let results = seal_with_targets(&declarations, &targets);
        for name in ["a_right", "b_left"] {
            let caller = key(&declarations, "Stable", name, 1);
            assert_eq!(
                results.disposition(&caller),
                Some(&Disposition::ExactString)
            );
            assert_eq!(
                results
                    .call_rows()
                    .filter(|((k, _), _)| k == &caller)
                    .count(),
                1
            );
        }
    }
}

#[test]
fn string_result_cannot_swallow_incompatible_call_row() {
    use super::super::expression_proof::{ExpressionProofContextV1, I64ExpressionFactV1};
    use super::super::CallableResultCatalogErrorV1;
    let declarations = declarations(
        r#"static box Coverage {
        caller(x,y) { return "-" + me.identity(x) }
        identity(value) { return value }
    }"#,
    );
    let recursive_site = site(vec![Path::Body(0), Path::Value, Path::Rhs]);
    let targets = extend_current_owner_targets(
        qualified_targets(&declarations, &[], &[]),
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "Coverage",
            caller_name: "caller",
            caller_arity: 2,
            site: recursive_site,
        }],
    );
    let caller = key(&declarations, "Coverage", "caller", 2);
    let callee = key(&declarations, "Coverage", "identity", 1);
    let result_rows = [(
        callee,
        Disposition::ExactI64 {
            required_i64_arguments: Box::new([0]),
        },
    )]
    .into_iter()
    .collect();
    let declaration = declarations
        .declaration_for(
            crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Coverage",
            "caller",
            2,
        )
        .unwrap();
    let crate::ast::ASTNode::Return {
        value: Some(expression),
        ..
    } = &declaration.body()[0]
    else {
        panic!("return expression")
    };
    let path = crate::mir::resolved_semantics::SourcePathV1::root_body(0).child(Path::Value);
    let mut context =
        ExpressionProofContextV1::new(&caller, declaration.params(), &targets, &result_rows)
            .unwrap();
    assert_eq!(
        context.prove_expression(expression, &path).unwrap(),
        I64ExpressionFactV1::ExactString
    );
    // Reusing this exact site with a different caller requirement is a
    // structural ledger error, independent of its enclosing String result.
    context.publish_binding("x", I64ExpressionFactV1::Exact([1].into()));
    assert!(matches!(
        context.prove_expression(expression, &path),
        Err(CallableResultCatalogErrorV1::DuplicateCallResultSite { .. })
    ));
}

#[test]
fn exact_string_projection_covers_callers_without_general_rows() {
    use super::super::{
        StaticExactI64RequirementErrorV1 as Error,
        VerifiedStaticCallResultPublicationHandoffV1 as Handoff,
    };
    let source = r#"static box Strings {
        caller(): i64 { local text = me.text() return 1 }
        text() { return "猫" }
    }"#;
    let declarations = declarations(source);
    let call_site = site(vec![Path::Body(0), Path::Initializer(0)]);
    let targets = extend_current_owner_targets(
        qualified_targets(&declarations, &[], &[]),
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "Strings",
            caller_name: "caller",
            caller_arity: 0,
            site: call_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "Strings", "caller", 0);
    assert!(results.call_result(&caller, &call_site).is_none());
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .unwrap();
    assert!(owner.finish_empty().is_err());
    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(&declarations, &caller, &call_site)
        .unwrap()
    else {
        panic!("String projection must select")
    };
    assert_eq!(handoff.representation(), &Representation::ExactString);
    assert_eq!(handoff.catalog_identity(), declarations.brand().identity());
    assert_eq!(handoff.target(), &key(&declarations, "Strings", "text", 0));
    assert!(handoff.required_callee_i64_arguments().is_empty());
    assert!(owner
        .take_for_source(&declarations, &caller, &call_site)
        .is_err());
    assert!(owner.finish_empty().is_ok());
    let missing_site = site(vec![Path::Body(99), Path::Value]);
    assert_eq!(
        Handoff::project_unconditional_result(
            &declarations,
            &caller,
            &missing_site,
            &targets,
            &results
        ),
        Err(Error::SourceTargetUnavailable)
    );
    let missing_caller =
        crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Absent", "caller", 0,
        );
    assert_eq!(
        Handoff::project_unconditional_result(
            &declarations,
            &missing_caller,
            &call_site,
            &targets,
            &results
        ),
        Err(Error::CallerOutsideCatalog)
    );
    let other_targets = qualified_targets(&declarations, &[], &[]);
    let other_results = seal_with_targets(&declarations, &other_targets);
    assert_eq!(
        Handoff::project_unconditional_result(
            &declarations,
            &caller,
            &call_site,
            &targets,
            &other_results
        ),
        Err(Error::ResultCatalogBrandMismatch)
    );
    let foreign = super::support::declarations(source);
    assert_eq!(
        Handoff::project_unconditional_result(&foreign, &caller, &call_site, &targets, &results),
        Err(Error::TargetCatalogBrandMismatch)
    );
}

#[test]
fn string_projection_does_not_replace_general_rows_or_unproved_nested_targets() {
    use super::super::{
        StaticExactI64RequirementErrorV1 as Error,
        VerifiedStaticCallResultPublicationHandoffV1 as Handoff,
    };
    let source = r#"static box Strings {
        caller(x) { return me.text(me.unknown(x)) }
        text(x) { return "猫" }
        unknown(x) { return me.unknown(x) }
        direct() { return me.text(1) }
    }"#;
    let declarations = declarations(source);
    let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .unwrap();
    let inventory =
        crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
            &declarations,
            &imports,
        )
        .unwrap();
    let targets = inventory.into_targets();
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .unwrap();
    let direct = key(&declarations, "Strings", "direct", 0);
    let call_site = site(vec![Path::Body(0), Path::Value]);
    assert!(results.call_result(&direct, &call_site).is_some());
    assert_eq!(
        Handoff::project_unconditional_result(
            &declarations,
            &direct,
            &call_site,
            &targets,
            &results
        ),
        Err(Error::GeneralCallResultAlreadyAvailable)
    );
    let unknown = key(&declarations, "Strings", "unknown", 1);
    assert_eq!(
        Handoff::project_unconditional_result(
            &declarations,
            &unknown,
            &call_site,
            &targets,
            &results
        ),
        Err(Error::TargetResultUnavailable)
    );
    let mut rejected_nested = false;
    for ((caller, site), target) in targets.rows() {
        let result = owner.take_for_source(&declarations, caller, site).unwrap();
        if caller.name() == "caller" && target.target() == &unknown {
            assert!(matches!(
                result,
                StaticCallResultPublicationTakeV1::TargetOnly(_)
            ));
            rejected_nested = true;
        }
    }
    assert!(rejected_nested);
    owner.finish_empty().unwrap();
}
