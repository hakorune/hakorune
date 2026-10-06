//! Focused boundary tests for `HANDLE-FIELD-READ-S0` — local-initializer
//! `receiver.field` reads.
//!
//! Positive membership requires the sealed chain: a `local x = recv.field`
//! FieldAccess initializer → a receiver binding with a proven provenance
//! (claim-local `new` Home, the entry loan's `me` root, or an earlier
//! proven field-read alias) → a unique declared field on that
//! provenance's own class authority → a declared result type that is
//! either numeric-integer (`Scalar`) or an ordinary registered box
//! (`Alias`). Anything else leaves the statement unclaimed — the `new`
//! claims that follow keep `PrefixNotCovered`, and no ledger row is
//! staged for the read site.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, OwnedExprSiteV1};

/// Exact owned site of the first `FieldAccess` reading `name` anywhere
/// in the package — fixtures keep read field names unique per test.
fn field_access_site(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    name: &str,
) -> OwnedExprSiteV1 {
    package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .body_shape()
                        .expect("body shape")
                        .expressions()
                        .iter()
                        .find_map(|row| {
                        match row {
                            BodyExpressionShapeV1::FieldAccess { site, field, .. }
                                if field.as_ref() == name =>
                            {
                                Some(OwnedExprSiteV1::new(
                                    declaration.owner(),
                                    site.clone(),
                                ))
                            }
                            _ => None,
                        }
                    })
                })
                .expect("batch loan")
        })
        .unwrap_or_else(|| panic!("field-access site for {name} not found"))
}

/// The `new` claim's prefix record — `Err(PrefixNotCovered(_))` is the
/// named unavailability an unclaimed statement leaves behind.
fn new_claim_prefix_covered(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> bool {
    package
        .ordinary_new_claim_ledger
        .pending_claims_for_test()
        .values()
        .all(|claim| claim.home_prefix().is_ok())
}

#[test]
fn local_field_read_claims_scalar_on_selected_new_home() {
    let package = issue(
        "box Pool {
            size: i64 = 0
            birth() { }
        }
        static box Main { main() {
            local pool = new Pool()
            local n = pool.size
            return n
        } }",
    )
    .expect("scalar field read package");
    let site = field_access_site(&package, "size");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged local field read");
    assert_eq!(result, LocalFieldReadResultV1::Scalar);
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn local_field_read_claims_object_alias_on_selected_new_home() {
    // `local page = pool.page` — an ordinary-box declared field installs
    // a borrowed alias carrying the declared class, never a Home.
    let package = issue(
        "box Page {
            id: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            birth() { }
        }
        static box Main { main() {
            local pool = new Pool()
            local page = pool.page
            return 0
        } }",
    )
    .expect("object alias field read package");
    let site = field_access_site(&package, "page");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged local field read");
    assert_eq!(result, LocalFieldReadResultV1::Alias("Page".into()));
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn local_field_read_claims_read_on_field_alias() {
    // The alias's retained declared class is the receiver class
    // authority for the next read — `page.alloc` resolves on Page.
    let package = issue(
        "box Page {
            alloc: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            birth() { }
        }
        static box Main { main() {
            local pool = new Pool()
            local page = pool.page
            local n = page.alloc
            return n
        } }",
    )
    .expect("read-on-alias package");
    let alias_site = field_access_site(&package, "page");
    let (_, alias_result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&alias_site)
        .expect("staged alias read");
    assert_eq!(alias_result, LocalFieldReadResultV1::Alias("Page".into()));
    let scalar_site = field_access_site(&package, "alloc");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&scalar_site)
        .expect("staged read on alias");
    assert_eq!(result, LocalFieldReadResultV1::Scalar);
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn local_field_read_claims_entry_receiver_read() {
    // `me.size` inside a selected instance method — the entry loan's
    // receiver root proves the field on the declaration's own box. The
    // method carries its own `new` claim so the verified walk (not the
    // bounded sibling) scans it.
    let package = issue(
        "box Page {
            birth() { }
        }
        box Pool {
            size: i64 = 0
            birth() { }
            get(): i64 {
                local t = new Page()
                local n = me.size
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.get()
            return r
        } }",
    )
    .expect("entry receiver field read package");
    // `pool.get()` must claim through the lexical i64 lane so `get` is
    // a selected callable carrying its entry loan into the walk.
    let (owner, call_site) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "get")
                        .map(|(site, _)| (declaration.owner(), site.clone()))
                })
                .expect("batch loan")
        })
        .expect("get call site");
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_call_covered(owner, &call_site),
        "the `pool.get()` call must claim so `get` carries an entry loan"
    );
    let site = field_access_site(&package, "size");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged entry-receiver read");
    assert_eq!(result, LocalFieldReadResultV1::Scalar);
    assert_eq!(field.declaration_ordinal(), 0);
}

#[test]
fn local_field_read_alias_is_not_a_movable_argument() {
    // `page` is a borrowed FieldAlias — it must not enter a selected
    // `new` argument slot (moving the receiver root through the alias
    // would double-own the storage).
    let package = issue(
        "box Page {
            id: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            birth() { }
        }
        box Shell {
            inner: Page = new Page()
            birth(x) { }
        }
        static box Main { main() {
            local pool = new Pool()
            local page = pool.page
            local shell = new Shell(page)
            return 0
        } }",
    )
    .expect("alias-argument package");
    let site = field_access_site(&package, "page");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&site)
            .map(|(_, result)| result),
        Some(LocalFieldReadResultV1::Alias("Page".into())),
        "the alias-producing read itself is claimed"
    );
    assert!(
        !new_claim_prefix_covered(&package),
        "the `new Shell(page)` claim stays uncovered — the alias is not a movable value"
    );
}

#[test]
fn local_field_read_stays_fail_closed() {
    for (label, member) in [
        // No declared field by that name — declaration authority fails.
        ("missing-field", "pool.missing"),
        // A `MapBox` field is neither a numeric integer nor an ordinary
        // registered box — the read is out of this lane's scope.
        ("container-field", "pool.map"),
    ] {
        let source = format!(
            "box Pool {{
                size: i64 = 0
                map: MapBox = new MapBox()
                birth() {{ }}
            }}
            box Page {{
                birth() {{ }}
            }}
            static box Main {{ main() {{
                local pool = new Pool()
                local n = {member}
                local page = new Page()
                return 0
            }} }}"
        );
        let package = issue(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        assert!(
            !new_claim_prefix_covered(&package),
            "{label}: an unclaimed read keeps the following `new` uncovered"
        );
    }
}

#[test]
fn local_field_read_claims_object_alias_on_entry_receiver() {
    // OBJECT-FIELD-READ-S0: `me.page` inside a selected instance method —
    // the entry loan's borrowed `me` root proves the object-typed field
    // on the declaration's own box source through the same staged
    // `local_read_field` issuer arm, and the result binds as a borrowed
    // `Page` alias. The method holds no `new` claim — the object-field
    // read itself selects the verified walk, and the alias-vs-null
    // compare claims beside it.
    let package = issue(
        "box Page {
            id: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            size: i64 = 0
            birth() { }
            probe(): i64 {
                local l = me.page
                local c = l == null
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.probe()
            return r
        } }",
    )
    .expect("entry-receiver object field read package");
    // `pool.probe()` must claim through the lexical i64 lane so `probe`
    // carries its entry loan into the walk.
    let (owner, call_site) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "probe")
                        .map(|(site, _)| (declaration.owner(), site.clone()))
                })
                .expect("batch loan")
        })
        .expect("probe call site");
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_call_covered(owner, &call_site),
        "the `pool.probe()` call must claim so `probe` carries an entry loan"
    );
    let site = field_access_site(&package, "page");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged `me.` object field read");
    assert_eq!(result, LocalFieldReadResultV1::Alias("Page".into()));
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn me_object_field_read_stays_fail_closed_without_declared_field() {
    // `me.missing` names no declared field — declaration authority
    // declines and no read row is staged, so the statement keeps the
    // uncovered boundary the lane always had.
    let package = issue(
        "box Pool {
            size: i64 = 0
            birth() { }
            probe(): i64 {
                local l = me.missing
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.probe()
            return r
        } }",
    )
    .expect("undeclared `me.` field package");
    assert!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&field_access_site(&package, "missing"))
            .is_none(),
        "an undeclared `me.` field stages no read row"
    );
}

#[test]
fn local_field_read_rejects_parameter_receiver() {
    // A parameter-rooted read — `p.size` where `p` is an opaque formal —
    // is outside the entry loan's `me` provenance for this slice and
    // stages no row.
    let package = issue(
        "box Pool {
            size: i64 = 0
            birth() { }
        }
        static box Helpers {
            probe(p): i64 {
                local n = p.size
                return n
            }
        }
        static box Main { main() {
            local pool = new Pool()
            return Helpers.probe(pool)
        } }",
    )
    .expect("parameter-receiver package");
    let site = field_access_site(&package, "size");
    assert!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&site)
            .is_none(),
        "a parameter receiver is not the entry loan's `me` root"
    );
}
