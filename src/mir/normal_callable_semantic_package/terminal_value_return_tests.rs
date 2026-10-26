//! Focused tests for the non-i64 terminal `return <value>` relation: the
//! source Facts record which exact value the site hands to the caller. No
//! physical ABI or ownership transfer is issued by the relation itself.
//!
//! The relation issues inside the New-home prefix walk, which runs for
//! AppMain roots and for child owners carrying a sealed MapLiteral. Child
//! owners outside that walk keep no relation and stay fail-closed.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{
    HomePrefixUnavailableV1, TerminalRelationV1, TerminalReturnedSourceV1,
};

/// Covered obligations admit install; the physical execution boundary
/// stays downstream at lowering.
fn assert_install_admits(package: super::VerifiedNormalCallableSemanticPackageV1) {
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

fn work_contract<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
) -> super::result_contract::CallableResultContractRefV1<'a> {
    let owner = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Work::make declaration")
        .owner();
    package
        .result_contracts
        .rows()
        .find(|row| row.borrow().owner() == owner)
        .expect("Work::make contract row")
        .borrow()
}

// The map literal keeps the child owner inside the prefix walk; the return
// value under test is a sibling of that map, never part of it.
fn work_program(body: &str) -> String {
    format!(
        "box Page {{}} static box Work {{ make(args) {{ {body} }} }}
         static box Main {{ main() {{ return 30 }} }}"
    )
}

#[test]
fn map_local_return_issues_value_relation_and_leaves_cleanup() {
    let package = issue(&work_program("local m = %{\"a\" => args} return m"))
        .expect("map-local return package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.sole_terminal_relation() else {
        panic!("map-local return issues a Value relation");
    };
    assert_eq!(relation.owner(), contract.owner());
    let flow = contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow");
    let local = flow
        .maps()
        .iter()
        .filter_map(|row| row.complete())
        .find(|map| map.local_binding().is_some())
        .expect("map local flow row");
    assert!(matches!(
        relation.returned(),
        TerminalReturnedSourceV1::MapLocal(binding) if *binding == local.local_binding().unwrap()
    ));
    // The returned local leaves with the caller; it is not terminal cleanup.
    assert!(flow.terminal_homes().unwrap().is_empty());
}

#[test]
fn home_local_return_issues_value_relation_and_leaves_cleanup() {
    // Source class discovery must retain the local acquisition separately
    // from its Return read. Rebinding must not reuse that construction.
    for (body, accepted) in [
        ("local p = new Page() return p", true),
        ("local p = new Page() p = new Page() return p", false),
    ] {
        let facts = super::ordinary_new_coseal::source_result_facts_for_test(&work_program(body));
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Work", "make", 1);
        let outcomes = facts.outcomes(&key);
        assert_eq!(outcomes.is_some(), accepted, "local return source: {body}");
        if let Some(outcomes) = outcomes {
            let root = &outcomes[0].witnesses()[0];
            let acquisition = root.construction_site_v1().expect("local New source");
            assert_eq!(acquisition.owner(), root.site().owner());
            assert_ne!(acquisition.site(), root.site().site());
        }
    }
    let package = issue(&work_program(
        "local p = new Page() local m = %{\"k\" => 1} return p",
    ))
    .expect("home-local return package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.sole_terminal_relation() else {
        panic!("home-local return issues a Value relation");
    };
    let flow = contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow");
    let TerminalReturnedSourceV1::Home {
        binding,
        acquisition,
    } = relation.returned()
    else {
        panic!("home return class");
    };
    // `p` left with the caller; only the sibling map stays for cleanup.
    let terminal = flow.terminal_homes().unwrap();
    assert_eq!(terminal.len(), 1);
    assert!(!terminal.iter().any(|home| home.binding() == *binding));
    assert!(matches!(
        flow.maps()[0].complete().unwrap().destination(),
        crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1::LocalBinding(
            map_binding
        ) if terminal.iter().any(|home| home.binding() == *map_binding)
    ));
    let _ = acquisition;
}

#[test]
fn parameter_handle_return_issues_borrowed_value_relation() {
    let package = issue(&work_program("local m = %{\"k\" => 1} return args"))
        .expect("parameter handle return package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.sole_terminal_relation() else {
        panic!("parameter handle return issues a Value relation");
    };
    assert!(matches!(
        relation.returned(),
        TerminalReturnedSourceV1::Handle(_)
    ));
    // The borrowed parameter stays owned by the caller; the map local still
    // owes its own terminal cleanup.
    assert_eq!(
        contract
            .completion()
            .cleanup()
            .root_flow()
            .expect("root flow")
            .terminal_homes()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn non_i64_literal_returns_issue_value_relations() {
    for (body, expected) in [
        ("return \"s\"", Some(TerminalReturnedSourceV1::StringLiteral)),
        ("return null", Some(TerminalReturnedSourceV1::NullLiteral)),
        ("return 1.5", Some(TerminalReturnedSourceV1::FloatLiteral)),
        ("local older = \"o\" + \"p\" local text = \"a\" + \"b\" text = text + \"c\" local younger = \"y\" + \"z\" return text", None),
    ] {
        let package = issue(&work_program(&format!("local m = %{{\"k\" => 1}} {body}")))
            .expect("literal return package");
        let contract = work_contract(&package);
        let Some(TerminalRelationV1::Value(relation)) = contract.sole_terminal_relation() else {
            panic!("literal return issues a Value relation: {body}");
        };
        if let Some(expected) = expected {
            if expected == TerminalReturnedSourceV1::StringLiteral {
                let TerminalReturnedSourceV1::TextExpression(evaluation) = relation.returned() else {
                    panic!("literal Return retains its original evaluation");
                };
                assert_eq!(evaluation.source().owner(), relation.owner());
                assert_eq!(evaluation.source().root().statement(), relation.return_site());
                assert_eq!(evaluation.source().nodes().len(), 1);
                assert_eq!(evaluation.source().nodes()[0].site(), relation.value_site());
                assert!(matches!(evaluation.source().nodes()[0].source(),
                    crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Literal(
                        crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(value)) if value.as_ref() == "s"));
                assert!(evaluation.owned_result_nodes().is_empty(), "literal needs explicit Materialize");
            } else {
                assert_eq!(relation.returned(), &expected, "{body}");
            }
        } else {
            let TerminalReturnedSourceV1::TextResidence { binding, acquisition } = relation.returned() else {
                panic!("owned Text return retains its current acquisition");
            };
            assert_eq!(*binding, acquisition.binding());
            assert!(acquisition.rebind_origin_v1().is_some(), "return reads the latest residence");
            let homes = contract.completion().cleanup().root_flow().unwrap().terminal_homes().unwrap();
            assert!(homes.contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone())),
                "returned Text remains available if sibling cleanup faults");
            let projection = package.ordinary_new_claim_ledger
                .normal_exit_projection_v1(relation.owner(), relation.return_site()).unwrap().unwrap();
            assert_eq!(projection.fault_homes(), homes);
            assert_eq!(projection.homes().len() + 1, homes.len());
            assert!(!projection.homes().iter().any(|home| home.binding() == acquisition.binding()),
                "only successful Normal exit transfers the returned residence");
            let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(
                &format!("static box Main {{ main() {{ {body} }} }}"));
            let (key, module, same, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
            assert!(std::rc::Rc::ptr_eq(&same, &ledger));
            let handoff = ledger.seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort, Vec::new()).unwrap();
            let root = handoff.root_source().unwrap();
            root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(())).unwrap();
            let events = root.finished_initializer_installs_v1(&module, &module.functions[&key]).unwrap();
            assert_eq!(events.iter().filter(|(_, row)| matches!(row,
                crate::mir::MirInstruction::StringResidenceInstall { returning: true, .. })).count(), 1,
                "publication lends the same source Return hold exactly once");
            let mut missing_hold = module.clone();
            for block in missing_hold.functions.get_mut(&key).unwrap().blocks.values_mut() {
                block.instructions.retain(|row| !matches!(row,
                    crate::mir::MirInstruction::StringResidenceInstall { returning: true, .. }));
            }
            assert!(root.finished_initializer_installs_v1(&missing_hold, &missing_hold.functions[&key]).is_err());

            let previous = module.functions[&key].blocks.values().flat_map(|block| &block.instructions)
                .find_map(|row| match row { crate::mir::MirInstruction::StringResidenceInstall { value, replaces: None, returning: false } => Some(*value), _ => None })
                .expect("the pre-rebind residence remains an original producer");
            let mut changed = module.clone();
            for block in changed.functions.get_mut(&key).unwrap().blocks.values_mut() {
                if let Some(crate::mir::MirInstruction::Return { value }) = &mut block.terminator {
                    assert_ne!(*value, Some(previous));
                    *value = Some(previous);
                }
            }
            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err());
            assert_eq!(super::direct_call_loan::lifecycle::call_result_kind(Some(&TerminalRelationV1::Value(relation.clone()))), None,
                "source relation alone cannot grant an I64 or ordinary owned-pair ABI");
        }
    }
}

#[test]
fn return_void_issues_the_unit_terminal_relation() {
    let package =
        issue(&work_program("local m = %{\"k\" => 1} return void")).expect("return void package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Unit(relation)) = contract.sole_terminal_relation() else {
        panic!("return void issues the Unit relation");
    };
    assert_eq!(relation.owner(), contract.owner());
    assert!(contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow")
        .terminal_homes()
        .is_ok());
}

/// `return <qualified call>` — the sealed method-call row proves a
/// qualified receiver outside the lexical environment. The OpaqueCall
/// relation records owner and sites only: it owns no callee identity,
/// result class, argument handoff, or Invoke authority.
#[test]
fn qualified_call_return_issues_opaque_call_relation() {
    for body in [
        "local m = %{\"k\" => 1} return Work.helper(0)",
        "local m = %{\"k\" => 1} return Main.main()",
    ] {
        let package = issue(&format!(
            "static box Work {{ make(args) {{ {body} }}
                     helper(x) {{ return x }} }}
                 static box Main {{ main() {{ return 30 }} }}"
        ))
        .expect("qualified-call return package");
        let contract = work_contract(&package);
        let Some(TerminalRelationV1::OpaqueCall(relation)) = contract.sole_terminal_relation()
        else {
            panic!("{body} issues an OpaqueCall relation");
        };
        assert_eq!(relation.owner(), contract.owner());
        assert_eq!(
            Some(relation.return_site()),
            contract.completion().explicit_site()
        );
        let flow = contract
            .completion()
            .cleanup()
            .root_flow()
            .expect("root flow");
        // The call result is opaque to this owner; the map local still
        // owes its own terminal cleanup.
        assert_eq!(flow.terminal_homes().unwrap().len(), 1, "{body}");
        assert_install_admits(package);
    }
}

/// A qualified call carrying a map argument keeps the OpaqueCall relation
/// but the argument's lifecycle obligations fail closed at the named
/// capability boundary — never flattened into unavailable terminal homes.
#[test]
fn qualified_call_map_argument_reaches_the_named_capability_boundary() {
    let package = issue(
        "static box Work { make(args) { local m = %{\"k\" => 1} return Work.helper(%{\"a\" => m}) }
             helper(x) { return x } }
         static box Main { main() { return 30 } }",
    )
    .expect("qualified-call map-argument package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::OpaqueCall(relation)) = contract.sole_terminal_relation() else {
        panic!("map-argument qualified call issues an OpaqueCall relation");
    };
    assert_eq!(relation.owner(), contract.owner());
    let flow = contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow");
    assert!(flow.terminal_homes().is_ok());
    let mut context = crate::mir::builder::CompilationContext::new();
    let error = match package.prepare_install(&mut context) {
        Err((_, error)) => error,
        Ok(_) => panic!("map argument handoff is not an admitted capability"),
    };
    assert!(
        matches!(
            error,
            super::install::NormalCallableSemanticPackageInstallIssueV1::MapLifecycleUndertaking(
                super::map_lifecycle_undertaking::MapLifecycleUndertakingIssueV1::UncoveredOperation { .. }
            )
        ),
        "{error:?}"
    );
}

#[test]
fn consumed_or_uninitialized_returns_stay_uncovered() {
    for body in [
        "local p = new Page() local m = %{\"h\" => p} return p",
        "local u local m = %{\"k\" => 1} return u",
    ] {
        let package = issue(&work_program(body)).expect("package");
        let contract = work_contract(&package);
        assert!(
            contract.terminal_relations().is_empty(),
            "{body} issues no relation"
        );
        assert!(
            matches!(
                contract
                    .completion()
                    .cleanup()
                    .root_flow()
                    .expect("root flow")
                    .terminal_homes(),
                Err(HomePrefixUnavailableV1::ReturnValueNotCovered(_))
            ),
            "{body} stays uncovered"
        );
    }
}
