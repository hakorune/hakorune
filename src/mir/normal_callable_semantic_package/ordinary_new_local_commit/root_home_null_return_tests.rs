//! Independent source-loan and append boundaries for the original Null exit.
use super::*;
use crate::mir::builder::emission::constant::{emit_integer_recorded, emit_null_recorded};
use crate::mir::MirBuilder;

fn source() -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    OwnedExprSiteV1,
    SourceStmtSiteV1,
) {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size: i64) { if size == 0 { return null } return new Token() } } static box Main { main() { local maker = new Maker() return maker.make(7) } }"
    ).unwrap();
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let (site, exit) = ledger
        .terminal_relation_index
        .values()
        .flat_map(|table| table.values())
        .find_map(|row| match row {
            TerminalRelationV1::Value(value)
                if matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral) =>
            {
                Some((
                    OwnedExprSiteV1::new(value.owner(), value.value_site().clone()),
                    value.return_site().clone(),
                ))
            }
            _ => None,
        })
        .unwrap();
    (ledger, site, exit)
}
fn builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("null_append_test/0".into());
    builder
}
fn snapshot(ledger: &OrdinaryNewClaimLedgerV1) -> String {
    format!("{:?}", ledger.root_exits.borrow())
}

#[test]
fn original_null_loan_append_and_unselected_boundaries() {
    for state in 0..3 {
        let (ledger, site, exit) = source();
        if state != 0 {
            ledger.root_exits.borrow_mut().insert(
                (site.owner(), exit),
                if state == 1 {
                    RootHomeExitProgress::Unprepared
                } else {
                    RootHomeExitProgress::Unavailable
                },
            );
        }
        let before = snapshot(&ledger);
        assert!(
            ledger
                .prepare_terminal_null_literal_v1(site)
                .unwrap()
                .is_none(),
            "unselected state {state}"
        );
        assert!(emit_null_recorded(&mut builder()).is_ok());
        assert_eq!(snapshot(&ledger), before);
    }
    for failure in 0..5 {
        let (ledger, site, exit) = source();
        assert!(ledger
            .prepare_root_home_exit(site.owner(), exit.node())
            .unwrap());
        let loan = ledger
            .prepare_terminal_null_literal_v1(site.clone())
            .unwrap()
            .unwrap();
        let mut builder = builder();
        let completed = if failure == 3 {
            emit_integer_recorded(&mut builder, 7).unwrap()
        } else {
            emit_null_recorded(&mut builder).unwrap()
        };
        let actual = match failure {
            1 => ledger.terminal_relation_index[&site.owner()]
                .values()
                .find_map(|row| match row {
                    TerminalRelationV1::Value(value) if value.value_site() != site.site() => Some(
                        OwnedExprSiteV1::new(value.owner(), value.value_site().clone()),
                    ),
                    _ => None,
                })
                .unwrap(),
            2 => {
                let owner = ledger.root_owner().unwrap();
                let value = ledger.terminal_relation_index[&owner]
                    .values()
                    .find_map(|row| match row {
                        TerminalRelationV1::Value(value) => Some(value),
                        _ => None,
                    })
                    .unwrap();
                OwnedExprSiteV1::new(owner, value.value_site().clone())
            }
            _ => site.clone(),
        };
        let before = snapshot(&ledger);
        let result = ledger.complete_terminal_null_literal_v1(loan, actual, &completed);
        if matches!(failure, 1 | 2 | 3) {
            assert!(
                result.unwrap_err().contains(if failure == 3 {
                    "append-kind"
                } else {
                    "loan-identity"
                }),
                "failure {failure}"
            );
            assert_eq!(snapshot(&ledger), before);
            continue;
        }
        result.unwrap();
        if failure == 4 {
            let loan = ledger
                .prepare_terminal_null_literal_v1(site.clone())
                .unwrap()
                .unwrap();
            let before = snapshot(&ledger);
            let another = emit_null_recorded(&mut builder).unwrap();
            assert!(ledger
                .complete_terminal_null_literal_v1(loan, site, &another)
                .unwrap_err()
                .contains("duplicate-append"));
            assert_eq!(snapshot(&ledger), before);
        } else {
            let exits = ledger.root_exits.borrow();
            let RootHomeExitProgress::Prepared(order) = &exits[&(site.owner(), exit.clone())]
            else {
                panic!("prepared original")
            };
            assert_eq!(order.null_return_binding(), Some(completed.original()));
            drop(exits);
            let other = emit_null_recorded(&mut builder).unwrap();
            let returned = (
                other.original().0,
                MirInstruction::Return {
                    value: Some(other.value()),
                },
            );
            builder.emit_for_test(returned.1.clone()).unwrap();
            assert!(ledger
                .begin_root_home_exit(site.owner(), &exit)
                .unwrap()
                .is_empty());
            let before = snapshot(&ledger);
            assert!(ledger
                .record_root_home_exit(site.owner(), &exit, Vec::new(), vec![returned])
                .unwrap_err()
                .contains("return-value"));
            assert_eq!(
                snapshot(&ledger),
                before,
                "joint physical and recorded drift refuses before move"
            );
        }
    }
    let (ledger, site, exit) = source();
    assert!(ledger
        .prepare_root_home_exit(site.owner(), exit.node())
        .unwrap());
    let completed = emit_null_recorded(&mut builder()).unwrap();
    assert!(ledger
        .begin_root_home_exit(site.owner(), &exit)
        .unwrap()
        .is_empty());
    let before = snapshot(&ledger);
    assert!(ledger
        .record_root_home_exit(
            site.owner(),
            &exit,
            Vec::new(),
            vec![(
                completed.original().0,
                MirInstruction::Return {
                    value: Some(completed.value())
                }
            )]
        )
        .unwrap_err()
        .contains("producer-missing"));
    assert_eq!(snapshot(&ledger), before);
}
