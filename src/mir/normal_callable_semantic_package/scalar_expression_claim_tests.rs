//! Whole-root scalar admission and rejected-root staging, through the real issuer.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::VerifiedNormalCallableSemanticPackageV1;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, OwnedExprSiteV1};

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
fn scalar_expression_scope_preserves_outside_nested_conditions_without_claims() {
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
        assert_eq!(staged_count(&package), 0, "outside profile: {condition}");
        assert!(
            package
                .ordinary_new_claim_ledger
                .pending_claims_for_test()
                .values()
                .all(|claim| claim.home_prefix().is_ok()),
            "do not newly reject old condition admission: {condition}"
        );
    }
}
