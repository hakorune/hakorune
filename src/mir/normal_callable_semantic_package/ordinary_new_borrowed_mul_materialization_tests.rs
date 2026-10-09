//! Original Mul source/append observations, not finished execution permission.
use super::*;
use crate::mir::builder::MirBuilder;

fn fixture(
    body: &str,
) -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    FunctionOwnerIdV1,
    OwnedExprSiteV1,
) {
    let text = format!("box Transport {{ birth() {{}} probe(p,q): i64 {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe(5,6) return 0 }} }}");
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 2)
        .unwrap();
    let owner = contract.owner;
    ledger
        .record_borrowed_ordinary_entry_values_v1(
            owner,
            Ok(contract
                .parameters
                .iter()
                .enumerate()
                .map(|(i, p)| (i as u32, p.binding, ValueId(70 + i as u32)))
                .collect::<Vec<_>>()
                .into_boxed_slice()),
        )
        .unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let site = source.definitions[&owner]
        .uses
        .iter()
        .find_map(|row| match &row.kind {
            Use::MulOperand { binary, .. } => Some(binary.clone()),
            _ => None,
        })
        .unwrap();
    (ledger, owner, site)
}

fn completed(
    operator: crate::ast::BinaryOperator,
) -> (
    crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
    (ValueId, ValueId),
) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mul_source_completion/0".into());
    let left = crate::mir::builder::emission::constant::emit_integer(&mut builder, 5).unwrap();
    let right = crate::mir::builder::emission::constant::emit_integer(&mut builder, 6).unwrap();
    (
        builder
            .build_binary_op_from_values_recorded(operator, left, right)
            .unwrap(),
        (left, right),
    )
}

#[test]
fn borrowed_mul_retains_same_source_and_actual_append_with_complete_coverage() {
    for body in [
        "if p <= q { return p * q } return 0",
        "if p <= q { return q * p } return 0",
        "if p <= q { return p * 2 } return 0",
        "if p <= q { return -2 * p } return 0",
    ] {
        let (ledger, owner, site) = fixture(body);
        assert!(ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::empty())
            .is_err());
        let loan = ledger
            .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
            .unwrap()
            .unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        for row in &source.definitions[&owner].uses {
            if let Use::MulOperand { binary, source, .. } = &row.kind {
                if binary == &site {
                    assert!(Rc::ptr_eq(source, loan.original()));
                }
            }
        }
        let (completed, children) = completed(crate::ast::BinaryOperator::Multiply);
        let record = ledger
            .record_borrowed_mul_v1(loan, children, &completed)
            .unwrap();
        assert_eq!(record.children(), children);
        assert_eq!(record.original(), completed.arithmetic_original().unwrap());
        assert_eq!(record.source().site(), &site);
        assert!(
            ledger.borrowed_entry_values.borrow()[&owner]
                .comparisons
                .is_empty(),
            "Mul never installs a Bool record"
        );
        ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::once(&record))
            .unwrap();
        let substitute = Rc::new(BorrowedMulMaterializationV1 {
            source: ledger
                .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
                .unwrap()
                .unwrap(),
            children,
            original: record.original().clone(),
        });
        assert!(ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::once(&substitute))
            .unwrap_err()
            .contains("reuse-identity"));
        let duplicate = ledger
            .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
            .unwrap()
            .unwrap();
        assert!(ledger
            .record_borrowed_mul_v1(duplicate, children, &completed)
            .unwrap_err()
            .contains("duplicate-materialization"));
        let repeated_source = ledger
            .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
            .unwrap()
            .unwrap();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("repeated_mul_source/0".into());
        let left = crate::mir::builder::emission::constant::emit_integer(&mut builder, 5).unwrap();
        let right = crate::mir::builder::emission::constant::emit_integer(&mut builder, 6).unwrap();
        for _ in 0..8 {
            crate::mir::builder::emission::constant::emit_integer(&mut builder, 0).unwrap();
        }
        let distinct = builder
            .build_binary_op_from_values_recorded(crate::ast::BinaryOperator::Multiply, left, right)
            .unwrap();
        assert_ne!(distinct.value(), record.value());
        assert!(ledger
            .record_borrowed_mul_v1(repeated_source, (left, right), &distinct)
            .unwrap_err()
            .contains("duplicate-materialization"));
        assert!(ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::empty())
            .is_err());
        ledger
            .borrowed_entry_values
            .borrow_mut()
            .get_mut(&owner)
            .unwrap()
            .multiplications
            .clear();
        assert!(ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::once(&record))
            .is_err());
        assert!(ledger
            .verify_borrowed_mul_reuse_v1(owner, std::iter::empty())
            .is_err());
    }
}

#[test]
fn borrowed_mul_rejects_source_entry_operator_and_append_drift() {
    use crate::ast::BinaryOperator;
    for operator in [BinaryOperator::Add, BinaryOperator::Greater] {
        let (ledger, owner, site) = fixture("if p <= q { return p * q } return 0");
        for wrong in [None, Some(BinOp::Add)] {
            assert!(ledger
                .prepare_borrowed_mul_source_v1(owner, &site, wrong)
                .is_err());
        }
        let loan = ledger
            .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
            .unwrap()
            .unwrap();
        let (completed, children) = completed(operator);
        assert!(ledger
            .record_borrowed_mul_v1(loan, children, &completed)
            .is_err());
        assert!(ledger.borrowed_entry_values.borrow()[&owner]
            .multiplications
            .is_empty());
    }
    let (ledger, owner, site) = fixture("if p <= q { return p * q } return 0");
    let mut issuer =
        crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign = OwnedExprSiteV1::new(issuer.issue().unwrap(), site.site().clone());
    assert!(ledger
        .prepare_borrowed_mul_source_v1(owner, &foreign, Some(BinOp::Mul))
        .is_err());
    let loan = ledger
        .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
        .unwrap()
        .unwrap();
    ledger.borrowed_entry_values.borrow_mut().remove(&owner);
    assert!(ledger
        .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
        .unwrap_err()
        .contains("entry-values-missing"));
    let (finished, children) = completed(BinaryOperator::Multiply);
    assert!(ledger
        .record_borrowed_mul_v1(loan, children, &finished)
        .is_err());
    let (mut ledger, owner, site) = fixture("if p <= q { return p * q } return 0");
    let state = Rc::get_mut(&mut ledger).expect("fixture owns the ledger");
    let source = state
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let definition = source.definitions.remove(&owner).unwrap();
    source.source_only_definitions.insert(owner, definition);
    assert!(ledger.has_borrowed_mul_source_v1(owner));
    assert!(ledger
        .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
        .unwrap_err()
        .contains("source-only-entry"));

    let (ledger, owner, site) = fixture("if p <= q { return p * q } return 0");
    let loan = ledger
        .prepare_borrowed_mul_source_v1(owner, &site, Some(BinOp::Mul))
        .unwrap()
        .unwrap();
    let (done, children) = completed(BinaryOperator::Multiply);
    let record = ledger
        .record_borrowed_mul_v1(loan, children, &done)
        .unwrap();
    let value = record.value();
    drop(record);
    let mut entries = ledger.borrowed_entry_values.borrow_mut();
    let record = entries
        .get_mut(&owner)
        .unwrap()
        .multiplications
        .get_mut(&value)
        .unwrap();
    let record = Rc::get_mut(record).unwrap();
    if let MirInstruction::BinOp { op, .. } = &mut record.original.1 {
        *op = BinOp::Add;
    }
    drop(entries);
    let entries = ledger.borrowed_entry_values.borrow();
    assert!(ledger
        .verify_borrowed_mul_reuse_v1(owner, entries[&owner].multiplications.values())
        .unwrap_err()
        .contains("reuse-identity"));
}
