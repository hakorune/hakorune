// Focused tests for `me.<ArrayBox field>.get(..)` element-integer results
// — the whole-Box integer-store census arm.
//
// `ArrayBox` storage accepts any tagged value, so the declared type proves
// container identity only: the manifest `get` result is `Dynamic`. The
// sealed census — a sole `new ArrayBox()` provider inside `birth`, every
// `set`/`push` write value integer-source, and no foreign selector or
// field escape anywhere in the Box — upgrades a `get` result to
// `I64Value`, which the existing local-result lane installs as `Trivial`.
// `set`/`push` statement coverage is unchanged by the census: the
// manifest already admits them, so a vetoed field keeps its `Dynamic`
// result instead of gaining or losing call coverage.
//
// Bounded: the get index must observe an Integer leaf (literal,
// Integer-class binding, or proven `me.<numeric field>` read), receivers
// stay `me.<field>` rooted at the entry loan, `local a = me.<field>`
// alias receivers are outside the call arm, and a return-position
// `me.<field>.get(..)` on a non-borrowed owner mints no terminal
// relation until that arm lands.
use crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

/// Issue the Page fixture with `body` inside `probe`, then report whether
/// every claim member kept a covered prefix. A body may introduce extra
/// `new` claim members — the probe requires every member's prefix to be
/// covered.
fn probe_covered(body: &str) -> bool {
    let package = issue_with_brand_catalog(&format!(
        "box Page {{ block_used: ArrayBox = new ArrayBox() free_top: i64 = 0
        birth() {{ }}
        probe() {{
            {body}
            local item = new Page()
            return 0
        }}
        take(a) {{ }}
        }}
        static box Main {{ main() {{ return 0 }} }}"
    ))
    .expect("package issues");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    assert!(!claims.is_empty(), "at least one claim member");
    claims.iter().all(|claim| {
        !matches!(
            claim.home_prefix(),
            Err(HomePrefixUnavailableV1::PrefixNotCovered(_))
        )
    })
}

/// The owner of the sole claim member inside `probe` — the function's
/// canonical id, so borrowed-result pins can address it directly.
fn probe_owner(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> FunctionOwnerIdV1 {
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim inside `probe`, got {claims:?}")
    };
    claim.site().owner()
}

/// Issue a borrowed-formal `probe(h)` fixture whose sealed borrowed
/// return source the tests inspect — `return x` of a Dynamic get can
/// only seal when the i64 arm fired. `main` calls `a.probe(a)` so
/// `probe` stays inside the armed call graph the result lane retains.
fn issue_borrowed(
    body: &str,
    tail: &str,
) -> Result<
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    crate::mir::normal_callable_semantic_package::NormalCallableSemanticPackageIssueV1,
> {
    issue_with_brand_catalog(&format!(
        "box Page {{ block_used: ArrayBox = new ArrayBox() free_top: i64 = 0
        birth() {{ }}
        probe(h) {{
            {body}
            local item = new Page()
            {tail}
        }}
        take(a) {{ }}
        }}
        static box Main {{ main() {{
            local a = new Page()
            local out = a.probe(null)
            return 0
        }} }}"
    ))
}

/// Issue a non-borrowed `probe()` fixture — the package issues even when
/// the field is vetoed; the sealed census records whether `block_used`
/// was proven element-integer.
fn issue_plain(
    body: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    issue_with_brand_catalog(&format!(
        "box Page {{ block_used: ArrayBox = new ArrayBox() free_top: i64 = 0
        birth() {{ }}
        probe() {{
            {body}
            local item = new Page()
            return 0
        }}
        take(a) {{ }}
        }}
        static box Main {{ main() {{ return 0 }} }}"
    ))
    .expect("non-borrowed package issues")
}

/// The single sealed borrowed-return proof inside `probe` — the fixture
/// arms exactly one borrowed-formal callee, so the map is its row:
/// `Ok(())` sealed i64, `Err` the named unavailability.
fn borrowed_result(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> Result<(), String> {
    let results = package
        .ordinary_new_claim_ledger
        .borrowed_i64_results_for_test();
    let [(_, row)] = results.as_slice() else {
        panic!("one armed borrowed callee, got {results:?}")
    };
    row.clone()
}

/// The proven arm: integer `set`/`push` writes seal the field, an
/// Integer-observed index admits the `get`, and the i64 result rides the
/// existing local-result lane all the way to `return x`.
#[test]
fn array_i64_get_installs_i64_result() {
    let package = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox() free_top: i64 = 0
        birth() { }
        probe() {
            me.block_used.set(0, 7)
            me.block_used.push(9)
            local i = 0
            local x = me.block_used.get(i)
            local item = new Page()
            return x
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("proven ArrayBox package");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .array_i64_fields_for_test("Page"),
        1,
        "the census proves the sole ArrayBox field"
    );
    {
        let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        let [claim] = claims.as_slice() else {
            panic!("one claim inside `probe`, got {claims:?}")
        };
        assert_eq!(
            claim.home_prefix().map(|_| ()),
            Ok(()),
            "proven ArrayBox field calls are covered prefix statements"
        );
    }
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

/// A `me.<numeric field>` read indexes the get — the same Integer leaf
/// the `integer_source` contract credits (`me.free_top` is declared i64
/// and initialized).
#[test]
fn array_i64_get_admits_numeric_field_index() {
    let package = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox() free_top: i64 = 0
        birth() { }
        probe() {
            me.block_used.set(0, 7)
            local x = me.block_used.get(me.free_top)
            local item = new Page()
            return x
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("numeric-field index admits the get result");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

/// A `get` on a proven field still chains through the fixpoint: a local
/// fed by a `get` result is Integer-observed for the next index.
#[test]
fn array_i64_get_result_feeds_next_index() {
    let package = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox()
        birth() { }
        probe() {
            me.block_used.push(1)
            local i = me.block_used.get(0)
            local x = me.block_used.get(i)
            local item = new Page()
            return x
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("get results chain as Integer indices");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

/// A borrowed callee returning the proven `get` — directly or through a
/// single-binding local — seals its i64 result source through the
/// ordinary borrowed-result pending lane. This is the `isLiveHandle`
/// shape minus the formal-field index leaf.
#[test]
fn array_i64_get_seals_borrowed_return_source() {
    for tail in [
        "local x = me.block_used.get(0) return x",
        "return me.block_used.get(0)",
    ] {
        let package = issue_borrowed(
            "me.block_used.set(0, 7)",
            tail,
        )
        .unwrap_or_else(|error| panic!("borrowed `{tail}` issues: {error:?}"));
        assert!(
            borrowed_result(&package).is_ok(),
            "borrowed `{tail}`: the proven get seals an i64 return source"
        );
    }
}

/// A non-integer `set`/`push` value vetoes the field: the calls stay
/// covered by the manifest, the `get` keeps `Dynamic`, and a borrowed
/// `return x` keeps the named `source-not-i64` unavailability.
#[test]
fn array_i64_non_integer_write_keeps_dynamic_result() {
    for body in [
        "me.block_used.set(0, true)",
        "me.block_used.push(true)",
        "local flag = true me.block_used.set(0, flag)",
    ] {
        // The Dynamic path is preserved — calls cover and `return 0`
        // still completes.
        assert!(
            probe_covered(&format!("{body} local x = me.block_used.get(0)")),
            "{body}: the Dynamic get path stays covered"
        );
        // The census vetoes the field and a borrowed `return x` of the
        // Dynamic result fails closed — the i64 proof names the
        // unavailability and the issue propagates it.
        let package = issue_plain(&format!("{body} local x = me.block_used.get(0)"));
        assert_eq!(
            package
                .ordinary_new_claim_ledger
                .array_i64_fields_for_test("Page"),
            0,
            "{body}: the census must veto the field"
        );
        let error = issue_borrowed(
            &format!("{body} local x = me.block_used.get(0)"),
            "return x",
        )
        .expect_err("{body}: a Dynamic get result must fail the borrowed i64 proof");
        assert!(
            format!("{error:?}").contains("source-not-i64"),
            "{body}: {error:?}"
        );
    }
}

/// A foreign selector on the field vetoes it *and* is outside the
/// manifest — the call itself stays uncovered.
#[test]
fn array_i64_foreign_selector_stays_uncovered() {
    assert!(
        !probe_covered("local n = me.block_used.frobnicate()"),
        "a selector outside the manifest keeps PrefixNotCovered"
    );
}

/// A non-Integer index leaf keeps the `Dynamic` result: a `null` index
/// is neutral (the call covers) but the i64 arm does not fire; a
/// `new`-bound local is a live `homes` member and is never neutral, so
/// its call stays uncovered — same for a string literal.
#[test]
fn array_i64_get_requires_i64_index() {
    // `null` is Home-neutral but not Integer — covered, Dynamic result.
    assert!(
        probe_covered("me.block_used.set(0, 1) local x = me.block_used.get(null)"),
        "a neutral non-Integer index covers the call without the i64 upgrade"
    );
    // The field itself stays proven — only the index leaf withholds the
    // i64 claim — so the census keeps the field while a borrowed
    // `return x` fails closed with `source-not-i64`.
    let package = issue_plain("me.block_used.set(0, 1) local x = me.block_used.get(null)");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .array_i64_fields_for_test("Page"),
        1,
        "a non-Integer index does not veto the field"
    );
    let error = issue_borrowed(
        "me.block_used.set(0, 1) local x = me.block_used.get(null)",
        "return x",
    )
    .expect_err("a non-Integer index must fail the borrowed i64 proof");
    assert!(
        format!("{error:?}").contains("source-not-i64"),
        "{error:?}"
    );
    // A live `new` binding is a `homes` member — the get is never neutral.
    assert!(
        !probe_covered(
            "local h = new Page() me.block_used.set(0, 1) local x = me.block_used.get(h)"
        ),
        "a homes-member index keeps the call uncovered"
    );
    // A string index is not even neutral — the call stays uncovered.
    assert!(
        !probe_covered("me.block_used.set(0, 1) local x = me.block_used.get(\"k\")"),
        "a non-neutral index keeps PrefixNotCovered"
    );
}

/// A second `me.<field>` store — even to another `new ArrayBox()` —
/// defeats the sole-provider leg: the field is vetoed and `get` keeps
/// `Dynamic`.
#[test]
fn array_i64_second_provider_keeps_dynamic_result() {
    // `birth` carrying a second `me.<field>` store defeats the
    // sole-provider leg: the field is vetoed even though every store
    // builds a bare `new ArrayBox()`.
    // `probe()` stays outside the borrowed lane: the package issues and
    // the census records the veto.
    let package = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox()
        birth() { me.block_used = new ArrayBox() }
        probe() {
            local x = me.block_used.get(0)
            local item = new Page()
            return 0
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("a second provider still issues outside borrowed returns");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .array_i64_fields_for_test("Page"),
        0,
        "a second field store keeps the census unproven"
    );
    // The Dynamic path itself still covers.
    assert!(
        probe_covered("local x = me.block_used.get(0)"),
        "a vetoed field keeps the manifest get coverage"
    );
    // A borrowed `return x` of the Dynamic get fails closed.
    let error = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox()
        birth() { me.block_used = new ArrayBox() }
        probe(h) {
            local x = me.block_used.get(0)
            local item = new Page()
            return x
        } }
        static box Main { main() {
            local a = new Page()
            local out = a.probe(null)
            return 0
        } }",
    )
    .expect_err("a second provider must fail the borrowed i64 proof");
    assert!(
        format!("{error:?}").contains("source-not-i64"),
        "{error:?}"
    );
}

/// A `me.<field>` escape — passing the array as a call argument — is
/// outside neutrality in the first place, and an alias whose uses are
/// not receivers vetoes the census as well.
#[test]
fn array_i64_field_escape_and_alias_rebind_keep_dynamic() {
    assert!(
        !probe_covered(
            "me.take(me.block_used) me.block_used.set(0, 1) local x = me.block_used.get(0)"
        ),
        "a `me.<field>` argument occurrence is not a neutral receiver"
    );
    // `local a = me.block_used` then rebinding `a` severs the proven
    // alias — the field is vetoed and the i64 arm does not fire.
    let package = issue_plain(
        "me.block_used.set(0, 1) local a = me.block_used a = me.block_used local x = me.block_used.get(0)",
    );
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .array_i64_fields_for_test("Page"),
        0,
        "a rebound field alias must veto the census"
    );
    let error = issue_borrowed(
        "me.block_used.set(0, 1) local a = me.block_used a = me.block_used local x = me.block_used.get(0)",
        "return x",
    )
    .expect_err("a rebound alias must fail the borrowed i64 proof");
    assert!(
        format!("{error:?}").contains("source-not-i64"),
        "{error:?}"
    );
}

/// Bounded arm: `return me.<field>.get(..)` on a non-borrowed owner
/// mints no terminal relation — the terminal arm is a separate slice.
#[test]
fn array_i64_return_position_mints_no_terminal_relation() {
    let package = issue_with_brand_catalog(
        "box Page { block_used: ArrayBox = new ArrayBox()
        birth() { }
        probe() {
            me.block_used.set(0, 1)
            local item = new Page()
            return me.block_used.get(0)
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("return-position package");
    let owner = probe_owner(&package);
    // No terminal relation may be minted for `return me.<field>.get(..)` —
    // the bounded arm admits the local-result form only.
    assert!(
        package
            .ordinary_new_claim_ledger
            .terminal_relations_for_owner(owner)
            .is_empty(),
        "a return-position get keeps its named unavailability"
    );
}

/// Bounded arm: `if me.<field>.get(i) == 0` rides the pre-existing `If`
/// walk — a condition rooted at a method call never selects the
/// field-read gate, so coverage here is unchanged by the census and the
/// arm mints no row for the condition get. This pin records that the
/// boundary did not move.
#[test]
fn array_i64_condition_compare_keeps_existing_boundary() {
    assert!(
        probe_covered("me.block_used.set(0, 1) if me.block_used.get(0) == 0 { me.block_used.set(0, 2) }"),
        "a condition-position get rides the existing un-gated if walk"
    );
}
