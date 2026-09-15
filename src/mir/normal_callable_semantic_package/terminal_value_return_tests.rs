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
    let Some(TerminalRelationV1::Value(relation)) = contract.terminal_relation() else {
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
    let package = issue(&work_program(
        "local p = new Page() local m = %{\"k\" => 1} return p",
    ))
    .expect("home-local return package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.terminal_relation() else {
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
    assert!(!terminal.contains(binding));
    assert!(matches!(
        flow.maps()[0].complete().unwrap().destination(),
        crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1::LocalBinding(
            map_binding
        ) if terminal.contains(map_binding)
    ));
    let _ = acquisition;
}

#[test]
fn parameter_handle_return_issues_borrowed_value_relation() {
    let package = issue(&work_program("local m = %{\"k\" => 1} return args"))
        .expect("parameter handle return package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.terminal_relation() else {
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
        ("return \"s\"", TerminalReturnedSourceV1::StringLiteral),
        ("return null", TerminalReturnedSourceV1::NullLiteral),
        ("return 1.5", TerminalReturnedSourceV1::FloatLiteral),
    ] {
        let package = issue(&work_program(&format!("local m = %{{\"k\" => 1}} {body}")))
            .expect("literal return package");
        let contract = work_contract(&package);
        let Some(TerminalRelationV1::Value(relation)) = contract.terminal_relation() else {
            panic!("literal return issues a Value relation: {body}");
        };
        assert_eq!(relation.returned(), &expected, "{body}");
    }
}

#[test]
fn return_void_issues_the_unit_terminal_relation() {
    let package =
        issue(&work_program("local m = %{\"k\" => 1} return void")).expect("return void package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Unit(relation)) = contract.terminal_relation() else {
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

#[test]
fn consumed_or_uninitialized_returns_stay_uncovered() {
    for body in [
        "local p = new Page() local m = %{\"h\" => p} return p",
        "local u local m = %{\"k\" => 1} return u",
    ] {
        let package = issue(&work_program(body)).expect("package");
        let contract = work_contract(&package);
        assert!(
            contract.terminal_relation().is_none(),
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
