//! Real source actuals are lent again; retained physical fields cannot issue them.
use super::*;

fn fixture(
    argument: &str,
    prefix: &str,
) -> (
    PreparedLexicalCallProjectionV1,
    LexicalInstanceCallDispositionRowV1,
    Box<[LocalCallArgumentV1]>,
    std::rc::Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    Vec<Binding>,
) {
    let text = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() {prefix} return recv.probe({argument}) }} }}");
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
    let exit = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .explicit_sites()[0]
        .clone();
    crate::mir::builder::lexical_call_projection_borrowed_fixture(package, &exit)
}

#[test]
fn borrowed_projection_demands_original_integer_bool_scalar_and_home_actuals() {
    for (argument, prefix) in [("-7", ""), ("true", ""), ("recv", ""), ("n", "local n = 8")] {
        let (prepared, row, source, ledger, bindings) = fixture(argument, prefix);
        let owner = row.call_site().owner();
        assert!(prepared
            .materialize(owner, &row, &source)
            .unwrap_err()
            .contains("borrowed-lender-missing"));
        let call = prepared
            .materialize_with_ledger(owner, &row, &source, &ledger)
            .unwrap();
        assert_eq!(call.args.len(), 1);
        prepared.validate_recorded(&bindings).unwrap();
        match argument {
            "recv" => assert_eq!(call.args, vec![ValueId(77)]),
            "n" => assert_eq!(call.args, vec![ValueId(79)]),
            _ => assert!(
                matches!(bindings.as_slice(), [(_, MirInstruction::Const { dst, .. })] if call.args == vec![*dst])
            ),
        }
    }
}

#[test]
fn borrowed_projection_rejects_reassigned_ordinal_site_formal_and_literal_domain() {
    for mutation in 0..4 {
        let (mut prepared, row, source, ledger, _) = fixture("true", "");
        let LexicalCallArgumentProjectionV1::BorrowedLiteral {
            ordinal,
            site,
            formal,
            binding,
        } = &mut prepared.arguments[0]
        else {
            panic!("literal")
        };
        match mutation {
            0 => *ordinal = 1,
            1 => *site = row.receiver_site().clone(),
            2 => *formal = row.receiver_binding(),
            _ => {
                let MirInstruction::Const { value, .. } = &mut binding.1 else {
                    panic!("Const")
                };
                *value = crate::mir::ConstValue::Integer(1);
            }
        }
        assert!(prepared
            .materialize_with_ledger(row.call_site().owner(), &row, &source, &ledger)
            .is_err());
    }
}

#[test]
fn borrowed_projection_rejects_foreign_lender_and_foreign_exact_read() {
    let (mut prepared, row, source, ledger, _) = fixture("recv", "");
    let (_, _, _, foreign, _) = fixture("recv", "");
    assert!(prepared
        .materialize_with_ledger(row.call_site().owner(), &row, &source, &foreign)
        .is_err());
    let LexicalCallArgumentProjectionV1::BorrowedRead { read, .. } = &mut prepared.arguments[0]
    else {
        panic!("read")
    };
    std::mem::swap(read, &mut prepared.receiver);
    assert!(prepared
        .materialize_with_ledger(row.call_site().owner(), &row, &source, &ledger)
        .is_err());
}

#[test]
fn borrowed_literal_producer_keeps_domain_and_exact_record_membership() {
    let (integer, integer_row, integer_source, integer_ledger, integer_bindings) = fixture("1", "");
    let (boolean, boolean_row, boolean_source, boolean_ledger, boolean_bindings) =
        fixture("true", "");
    assert!(matches!(
        integer_bindings[0].1,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(1),
            ..
        }
    ));
    assert!(matches!(
        boolean_bindings[0].1,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Bool(true),
            ..
        }
    ));
    integer
        .materialize_with_ledger(
            integer_row.call_site().owner(),
            &integer_row,
            &integer_source,
            &integer_ledger,
        )
        .unwrap();
    boolean
        .materialize_with_ledger(
            boolean_row.call_site().owner(),
            &boolean_row,
            &boolean_source,
            &boolean_ledger,
        )
        .unwrap();
    assert!(boolean.validate_recorded(&[]).is_err());
    let mut duplicate = boolean_bindings.clone();
    duplicate.push(boolean_bindings[0].clone());
    assert!(boolean.validate_recorded(&duplicate).is_err());
}
