//! Whole-root scalar admission and rejected-root staging, through the real issuer.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::VerifiedNormalCallableSemanticPackageV1;
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, LocalCallObservationV1, LocalCallResultClassV1,
};
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, OwnedExprSiteV1, SourcePathSegmentV1};

fn static_accepts_home_calls(package: &VerifiedNormalCallableSemanticPackageV1) -> Vec<LocalCallObservationV1> {
    let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "accepts", 1);
    let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
    let owner = package.parameter_contracts.iter().find(|row| row.batch_slot == slot).unwrap().owner;
    package.ordinary_new_claim_ledger.completion_for_owner(owner)
        .and_then(|completion| completion.cleanup().root_flow())
        .expect("accepts source Home flow")
        .local_calls().to_vec()
}

#[test]
fn static_current_owner_eq_home_preserves_both_original_children_in_order() {
    let source = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    let package = issue(source).expect("unchanged size-class source");
    let calls = static_accepts_home_calls(&package);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].site().site().node().segments(), &[
        SourcePathSegmentV1::Body(0), SourcePathSegmentV1::IfCondition,
        SourcePathSegmentV1::Lhs,
    ]);
    assert_eq!(calls[1].site().site().node().segments(), &[
        SourcePathSegmentV1::Body(0), SourcePathSegmentV1::IfCondition,
        SourcePathSegmentV1::Rhs,
    ]);
    assert_eq!(calls[0].statement(), calls[1].statement());
    assert!(calls.iter().all(|call| call.is_expression_value()
        && call.result() == LocalCallResultClassV1::I64));
    assert!(matches!(calls[0].arguments(), [LocalCallArgumentV1::BorrowedActual {
        ordinal: 0, ..
    }]));
    assert!(calls[1].arguments().is_empty());
}

#[test]
fn static_current_owner_eq_home_rejects_incomplete_or_reordered_pair() {
    let source = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    let original = "if me.size_to_bin(size) == me.huge_bin()";
    for condition in [
        "if me.huge_bin() == me.size_to_bin(size)",
        "if me.size_to_bin(size) == 73",
        "if me.size_to_bin(me.huge_bin()) == me.huge_bin()",
        "if me.size_to_bin(size) && me.huge_bin()",
    ] {
        let changed = source.replacen(original, condition, 1);
        assert_ne!(changed, source);
        let package = issue(&changed).expect("changed source package");
        let calls = static_accepts_home_calls(&package);
        assert!(calls.is_empty(), "no partial Eq Home: {condition}");
    }
}

fn package(body: &str) -> VerifiedNormalCallableSemanticPackageV1 {
    issue(&format!(
        "box Page {{ id: i64 = 0 birth() {{ }} }}
        box Pool {{ size: i64 = 0 page: Page = new Page() birth() {{ }} }}
        static box Main {{ main() {{ local pool = new Pool() {body} }} }}"
    ))
    .expect("scalar expression package")
}

fn reads(package: &VerifiedNormalCallableSemanticPackageV1) -> Vec<OwnedExprSiteV1> {
    let mut reads = Vec::new();
    for declaration in package.batch().declarations() {
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                for row in input.body_shape().expect("body shape").expressions() {
                    if let BodyExpressionShapeV1::FieldAccess { site, .. } = row {
                        reads.push(OwnedExprSiteV1::new(declaration.owner(), site.clone()));
                    }
                }
            })
            .expect("exact source loan");
    }
    reads
}

fn staged_count(package: &VerifiedNormalCallableSemanticPackageV1) -> usize {
    reads(package)
        .iter()
        .filter(|site| {
            package
                .ordinary_new_claim_ledger
                .staged_local_field_read(site)
                .is_some()
        })
        .count()
}

#[test]
fn scalar_expression_claims_arithmetic_with_exact_local_and_repeated_field_sites() {
    let package = package("local one = 1 local n = pool.size + one - pool.size return n");
    let sites = reads(&package);
    assert_eq!(sites.len(), 2);
    assert_ne!(sites[0], sites[1]);
    assert_eq!(staged_count(&package), 2);
}

#[test]
fn scalar_expression_claims_bool_initializer_and_field_condition() {
    let package = package("local b = pool.size == 0 if b && pool.size != 1 { return 1 } return 0");
    assert_eq!(staged_count(&package), 2);
}

#[test]
fn scalar_expression_rejects_whole_initializer_without_partial_rows() {
    for root in [
        "pool.size + true",
        "pool.size + pool.missing",
        "pool.size + pool.page",
        "pool.size * 2",
        "pool.size + pool.unknown()",
        "pool.size + null",
    ] {
        let package = package(&format!(
            "local n = {root} local later = new Page() return 0"
        ));
        assert_eq!(staged_count(&package), 0, "rejected root {root}");
        assert!(
            package
                .ordinary_new_claim_ledger
                .pending_claims_for_test()
                .values()
                .any(|claim| claim.home_prefix().is_err()),
            "root must remain uncovered: {root}"
        );
    }
}

#[test]
fn scalar_expression_rejects_non_bool_field_condition_before_staging() {
    let package = package("if pool.size + 1 { local later = new Page() return 1 } return 0");
    assert_eq!(staged_count(&package), 0);
}

#[test]
fn scalar_expression_claims_read_through_prior_field_alias() {
    let package = package("local page = pool.page local n = page.id + 1 return n");
    assert_eq!(staged_count(&package), 2);
}

#[test]
fn scalar_expression_rejects_field_reads_after_selected_root_transfer() {
    for later in [
        "local n = page.id + 1",
        "local n = direct.size + 1",
        "if page.id == 0 { return 1 }",
        "if direct.size == 0 { return 1 }",
    ] {
        let source = format!(
            "box Page {{ id: i64 = 0 birth() {{ }} }}
            box Pool {{ size: i64 = 0 page: Page = new Page() birth() {{ }} }}
            box Shell {{ birth(x) {{ }} }}
            static box Main {{ main() {{ local pool = new Pool()
                local direct = pool local page = pool.page
                local shell = new Shell(pool) {later} return 0 }} }}"
        );
        let package = issue(&source).expect("selected root transfer fixture");
        let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let shell = claims
            .values()
            .find(|claim| claim.class() == "Shell")
            .expect("Shell claim");
        assert!(
            shell.home_prefix().is_ok(),
            "the transfer must have been admitted"
        );
        assert_eq!(
            staged_count(&package),
            1,
            "only pre-transfer alias read may be staged: {later}"
        );
    }
}

#[test]
fn scalar_expression_claims_order_compare_field_operands() {
    // FIELDOPERAND-S0: a guarded formal's `i64` field is admitted as an
    // order-compare operand through the same staged local-read authority —
    // `<` and `>=` both claim their exact FieldAccess sites, and the `me.`
    // receiver operand rides the same request inside the selected root.
    let formal = issue(
        "box Handle { page_id: i64 block_id: i64 birth(pid, bid) { me.page_id = pid me.block_id = bid } } \
        box Store { limit: i64 birth() { me.limit = 10 } \
        check(p) { if p > me.limit { return null } return new Handle(p, 3) } \
        release(handle) { if handle == null { return 0 } if handle.page_id < 0 { return 0 } \
        if handle.page_id >= me.limit { return 0 } return 1 } } \
        static box Main { main() { local s = new Store() local h = s.check(5) return s.release(h) } }",
    )
    .expect("guarded formal order compare package");
    assert_eq!(staged_count(&formal), 3);
    // A whole Bool frame now owns both exact field uses and comparison laws.
    // Direct order comparisons still keep their existing admission owner.
    let owned = package("if (pool.size > 0) && (pool.size == 1) { return 1 } return 0");
    assert_eq!(staged_count(&owned), 2);
    let frames: Vec<_> = owned.parameter_contracts.iter().filter_map(|row|
        owned.ordinary_new_claim_ledger.completion_for_owner(row.owner))
        .filter_map(|completion| completion.cleanup().root_flow())
        .flat_map(|flow| flow.expression_plans().iter())
        .filter_map(|plan| plan.condition_frame()).collect();
    assert_eq!(frames.len(), 1);
    let frame = frames[0];
    assert_eq!(frame.logical_nodes().len(), 1);
    assert!(frame.calls().is_empty());
    assert_eq!(frame.operations().iter().map(|loan| loan.single_envelope().unwrap().domain().family()).collect::<Vec<_>>(),
        vec![crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1::Greater,
            crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1::Equal]);
    assert!(frame.operations().iter().all(|loan| std::rc::Rc::ptr_eq(loan.source(), frame.source())));
}

#[test]
fn scalar_expression_rejects_order_compare_on_object_typed_field() {
    // An ordinary-box declared field read still cannot feed an order
    // compare: `scalar_only` keeps the Alias result out of Integer
    // operand position, so the root stays uncovered and unstaged.
    for root in [
        "if pool.page > 0 { return 1 } return 0",
        "if pool.page <= pool.size { return 1 } return 0",
    ] {
        let package = package(&format!("{root} local later = new Page() return 0"));
        assert_eq!(staged_count(&package), 0, "rejected root {root}");
        assert!(
            package
                .ordinary_new_claim_ledger
                .pending_claims_for_test()
                .values()
                .any(|claim| claim.home_prefix().is_err()),
            "root must remain uncovered: {root}"
        );
    }
}

#[test]
fn field_alias_null_equality_claims_both_predicates_and_orders() {
    // OBJECT-FIELD-READ-S0: a proven field-read alias is a borrowed
    // handle — `== null`/`!= null` claim Bool through the scalar
    // preflight in either operand order, and the statement leaves the
    // following `new` claim covered.
    for compare in ["l == null", "l != null", "null == l", "null != l"] {
        let package = package(&format!(
            "local l = pool.page local c = {compare} local later = new Page() return 0"
        ));
        assert_eq!(staged_count(&package), 1, "compare {compare}");
        assert!(
            package
                .ordinary_new_claim_ledger
                .pending_claims_for_test()
                .values()
                .all(|claim| claim.home_prefix().is_ok()),
            "compare leaves the following claim covered: {compare}"
        );
    }
}

#[test]
fn field_alias_compare_rejects_non_null_operands() {
    // Only the exact `null` literal names a borrowed handle's emptiness:
    // alias-vs-integer and alias-vs-alias equality stay uncovered, and
    // the following `new` claim keeps `PrefixNotCovered`.
    for compare in ["l == 5", "l != 0", "l == l", "l != l"] {
        let package = package(&format!(
            "local l = pool.page local c = {compare} local later = new Page() return 0"
        ));
        assert_eq!(staged_count(&package), 1, "compare {compare}");
        assert!(
            package
                .ordinary_new_claim_ledger
                .pending_claims_for_test()
                .values()
                .any(|claim| claim.home_prefix().is_err()),
            "compare must stay uncovered: {compare}"
        );
    }
}

#[test]
fn scalar_expression_scope_keeps_whole_bool_and_outside_nested_conditions_distinct() {
    for condition in [
        "(pool.size > 0) && (pool.size == 1)",
        "(pool.size == 1) && pool.ready()",
        "(pool.size == 1) && pool.page.id == 1",
    ] {
        let source = format!("box Page {{ id: i64 = 0 birth() {{ }} }}
            box Pool {{ size: i64 = 0 page: Page = new Page() birth() {{ }} ready(): i64 {{ return 1 }} }}
            static box Main {{ main() {{ local pool = new Pool()
                if {condition} {{ local later = new Page() return 1 }} return 0 }} }}");
        let package = issue(&source).expect("outside condition source package");
        let whole_integer_compare = condition == "(pool.size > 0) && (pool.size == 1)";
        let invalid_logical_integer = condition == "(pool.size == 1) && pool.ready()";
        assert_eq!(staged_count(&package), if whole_integer_compare { 2 } else { 0 }, "{condition}");
        let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
        if invalid_logical_integer {
            let mut refused = 0;
            package.batch().declarations().for_each(|declaration| {
                package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                    for site in input.function().if_region_sites() {
                        let flow = package.ordinary_new_claim_ledger.completion_for_owner(input.owner())
                            .and_then(|completion| completion.cleanup().root_flow()).unwrap();
                        assert!(flow.statement_prefix_unavailable(site));
                        assert!(flow.expression_plans().iter().all(|plan| plan.condition_frame().is_none()));
                        refused += 1;
                    }
                }).unwrap();
            });
            assert_eq!(refused, 1, "selected I64 logical operand refuses the exact Home condition");
        } else {
            assert!(claims.values().all(|claim| claim.home_prefix().is_ok()),
                "whole-frame success and unselected nested-field admission: {condition}");
        }
    }
}

fn source_operations(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> Vec<
    std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1,
    >,
> {
    package
        .batch()
        .declarations()
        .flat_map(|declaration| {
            package
                .ordinary_new_claim_ledger
                .completion_for_owner(declaration.owner())
                .and_then(|completion| completion.cleanup().root_flow())
                .map(|flow| flow.scalar_operations().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect()
}

#[test]
fn scalar_source_retention_preserves_original_size_class_initializer_rows() {
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    let package = issue(include_str!(
        "../../../lang/src/hako_alloc/memory/size_class_box.hako"
    ))
    .unwrap();
    let rows = source_operations(&package);
    assert_eq!(rows.len(), 3);
    assert_eq!(
        rows.iter()
            .filter(|row| row.binary().operator() == Op::Subtract)
            .count(),
        2
    );
    let div = rows
        .iter()
        .find(|row| row.binary().operator() == Op::Divide)
        .unwrap();
    assert_eq!(div.positive_divisor(), Some(4));
    for row in &rows {
        assert_eq!(row.initializer().unwrap().binding().owner(), row.owner());
        assert_eq!(
            row.initializer().unwrap().initializer_site(),
            Some(row.binary().site())
        );
        assert_eq!(row.plan().owner(), row.owner());
        assert_eq!(row.plan().nodes()[0].binary(), Some(row.binary()));
        assert!(!row.plan().has_opaque_obligations());
        assert!(row.plan().nodes().iter().any(|node| node.site() == row.binary().lhs()));
        assert!(row.plan().nodes().iter().any(|node| node.site() == row.binary().rhs()));
        assert_ne!(row.binary().lhs(), row.binary().rhs());
        assert!(!row.statement().node().segments().is_empty());
    }
    let again = source_operations(&package);
    assert!(rows
        .iter()
        .zip(again)
        .all(|(original, retained)| std::rc::Rc::ptr_eq(original, &retained)));
}

#[test]
fn scalar_source_retention_is_published_only_for_complete_integer_roots() {
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    for (root, expected) in [
        ("pool.size - 1", Some(Op::Subtract)),
        ("pool.size / 4", Some(Op::Divide)),
        ("pool.size / 0", None),
        ("pool.size / -1", None),
        ("pool.size / true", None),
        ("pool.size - true", None),
        ("pool.missing / 4", None),
        ("pool.page / 4", None),
        ("pool.size + 1", None),
    ] {
        let package = package(&format!("local value = {root} return 0"));
        let rows = source_operations(&package);
        assert_eq!(rows.len(), usize::from(expected.is_some()), "{root}");
        if let Some(expected) = expected {
            assert_eq!(rows[0].binary().operator(), expected);
        }
    }
}

#[test]
fn scalar_source_retention_keeps_both_branch_inventories() {
    let package = package("if pool.size == 0 { local left = pool.size / 4 return 1 } else { local right = pool.size - 1 return 2 }");
    let rows = source_operations(&package);
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].binary().site(), rows[1].binary().site());
    for row in &rows {
        assert!(matches!(row.initializer().unwrap().declaration_site(),
            crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, .. }
                if statement == row.statement()));
    }
    let again = source_operations(&package);
    assert!(rows
        .iter()
        .zip(again)
        .all(|(left, right)| std::rc::Rc::ptr_eq(left, &right)));
}
