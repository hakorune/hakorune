//! Field receiver views keep their original transport boundary.
use super::*;

fn field_receiver_package(
    check_body: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!(
        "box Probe {{ sizes: ArrayBox = new ArrayBox() birth() {{ }} \
         check(p) {{ {check_body} }} }} \
         box Outer {{ probe: Probe birth() {{ me.probe = new Probe() }} \
         run() {{ return me.probe.check(9) }} }} \
         static box Main {{ main() {{ local outer = new Outer() return outer.run() }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("field-receiver package")
}

fn formal_owner_named(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    name: &str,
) -> super::FunctionOwnerIdV1 {
    package
        .parameter_contracts
        .iter()
        .find(|row| {
            package
                .batch()
                .with_lowering_input(row.batch_slot, |input| {
                    row.parameters.iter().any(|formal| {
                        input
                            .function()
                            .binding(formal.binding)
                            .is_some_and(|binding| binding.diagnostic_name() == name)
                    })
                })
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("{name} formal contract"))
        .owner
}

fn view_sites_of(
    prepared: &PreparedBorrowedFormalIngressV1,
    owner: super::FunctionOwnerIdV1,
) -> Vec<&crate::mir::resolved_semantics::SourceNodeSiteV1> {
    prepared
        .dominated_view_sites
        .iter()
        .filter(|site| site.owner() == owner)
        .map(|site| site.site().node())
        .collect()
}

/// `me.<field>` receiver calls never enter the lexical incoming map, so a
/// field-receiver callee stays outside the borrowed transport profile —
/// yet its sealed draft still admits dominated-view value uses. The
/// prefix scanner's consult reads classification output, not transport
/// membership: the `.set` element value and the `+` operand both carry
/// dominated-view rows while `definitions` holds no row for the owner.
#[test]
fn field_receiver_callee_keeps_dominated_view_sites_outside_transport() {
    let package = field_receiver_package(
        "if p > 5 { return 1 } local i = me.sizes.get(0) \
         me.sizes.set(i, p) return p + 1",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    let check_owner = formal_owner_named(&package, "p");
    assert!(
        !prepared.definitions.contains_key(&check_owner),
        "`me.probe.check` is a field-receiver edge — no lexical incoming call"
    );
    let sites = view_sites_of(prepared, check_owner);
    assert_eq!(
        sites.len(),
        2,
        "`.set` value and `+` operand rows: {sites:?}"
    );
    assert!(
        sites.iter().any(|site| site.segments().last()
            == Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(1))),
        "`.set` element-value leaf admitted: {sites:?}"
    );
    assert!(
        sites.iter().any(|site| site.segments().ends_with(&[
            crate::mir::resolved_semantics::SourcePathSegmentV1::Value,
            crate::mir::resolved_semantics::SourcePathSegmentV1::Lhs,
        ])),
        "dominated `+` operand leaf admitted: {sites:?}"
    );
}

/// The `.get`-result index arm names only the exact sole initializer —
/// it never propagates through a plain local copy. `j = i` is not a
/// `.get` initializer, so the `.set` element value stays an unresolved
/// argument and carries no dominated-view row; the `+` operand is still
/// admitted under the same guard.
#[test]
fn get_result_index_rejects_plain_copy_of_get_result() {
    let package = field_receiver_package(
        "if p > 5 { return 1 } local i = me.sizes.get(0) local j = i \
         me.sizes.set(j, p) return p + 1",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    let check_owner = formal_owner_named(&package, "p");
    assert!(!prepared.definitions.contains_key(&check_owner));
    let sites = view_sites_of(prepared, check_owner);
    assert_eq!(
        sites.len(),
        1,
        "only the `+` operand keeps a dominated-view row: {sites:?}"
    );
    assert!(
        sites.iter().any(|site| site.segments().ends_with(&[
            crate::mir::resolved_semantics::SourcePathSegmentV1::Value,
            crate::mir::resolved_semantics::SourcePathSegmentV1::Lhs,
        ])),
        "dominated `+` operand leaf admitted: {sites:?}"
    );
    assert!(
        !sites.iter().any(|site| site.segments().last()
            == Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(1))),
        "no `.set` element-value row through a plain copy: {sites:?}"
    );
}
