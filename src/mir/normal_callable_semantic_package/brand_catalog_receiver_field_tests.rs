/// `me.<field> = <rhs>` on the self-rooted receiver is a covered prefix
/// statement when the RHS subtree is Home-neutral: `me.<field>` reads are
/// proven by the receiver-side scalar contract (`i64` and `usize` both
/// admit — argument-position `i64` evidence stays a separate authority),
/// and the write itself mints no ledger row.
#[test]
fn ordinary_new_receiver_field_write_is_home_neutral() {
    let package = issue_with_brand_catalog(
        "box Page { left: i64 right: usize birth() { }
        touch() {
            me.left = me.left + 1
            me.right = me.right - me.left
            local item = new Page()
            return 0
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("field-write package");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim inside `touch`, got {claims:?}")
    };
    assert_eq!(
        claim.home_prefix().map(|_| ()),
        Ok(()),
        "self-rooted scalar field writes are covered prefix statements"
    );
}

/// Field-write admission stays fail-closed: an owned-binding RHS, a
/// non-`me` receiver, a `me`-field receiver call, and a `new` inside the
/// RHS each keep `PrefixNotCovered` — no owning-field or receiver-write
/// meaning is minted.
#[test]
fn ordinary_new_receiver_field_write_stays_fail_closed() {
    for (label, write) in [
        // An owned Home in the RHS is the parked owning-field family.
        ("owned-handle-rhs", "local h = new Page() me.left = h"),
        // A non-`me` receiver is not a self field write.
        ("non-self-receiver", "local p = new Page() p.left = 5"),
        // A call inside the RHS is the nested receiver-call family.
        ("rhs-receiver-call", "me.left = me.right.make()"),
        // `new` inside the RHS is a construction, not a scalar store.
        ("rhs-new", "me.left = new Page()"),
        // Compound `op=` reads and writes the same field.
        ("compound", "me.left += 1"),
    ] {
        let source = format!(
            "box Page {{ left: i64 right: Page birth() {{ }}
            make() {{ return 0 }}
            touch() {{ {write} local item = new Page() return 0 }} }}
            static box Main {{ main() {{ return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        // The `local item = new Page()` claim follows the write — its
        // prefix must name the uncovered statement; claims declared before
        // the write legitimately stay `Ok`.
        assert!(
            claims.iter().any(|claim| matches!(
                claim.home_prefix(),
                Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(_))
            )),
            "{label}: a claim past the write keeps PrefixNotCovered, got {claims:?}"
        );
    }
}

/// `me.<ArrayBox field>.m(..)` is a covered prefix statement when the
/// field's declared type is `ArrayBox`, the (selector, arity) pair
/// resolves in the generated `CORE_METHOD_CONTRACT_ROWS_V2` manifest, and
/// every argument subtree is Home-neutral. `NoValue` calls admit in bare
/// statement position; `local x = ..` installs the manifest result class
/// (`Dynamic` → `BoundValue`, scalar kinds → `Trivial`). No ledger row is
/// minted — the raw lane's `Callee::Method{RuntimeData}`/`ArrayElementWrite`
/// emission already owns the instruction.
#[test]
fn ordinary_new_receiver_field_call_is_home_neutral() {
    let package = issue_with_brand_catalog(
        "box Page { left: i64 items: ArrayBox birth() { }
        touch() {
            me.items.set(0, 1)
            me.items.push(me.left)
            local v = me.items.get(me.left)
            local n = me.items.length()
            local b = me.items.has(v)
            me.items.set(n, v)
            if n > 0 {
                me.items.push(n)
                local w = me.items.get(0)
            }
            local item = new Page()
            return 0
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("field-call package");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim inside `touch`, got {claims:?}")
    };
    assert_eq!(
        claim.home_prefix().map(|_| ()),
        Ok(()),
        "manifest-proven ArrayBox field calls are covered prefix statements"
    );
}

/// Field-call admission stays fail-closed: anything outside the declared
/// `ArrayBox`-field + manifest-(selector, arity) + Home-neutral-argument
/// contract keeps `PrefixNotCovered`.
#[test]
fn ordinary_new_receiver_field_call_stays_fail_closed() {
    for (label, prefix) in [
        // A `get` (Dynamic result) in bare statement position silently
        // discards a produced value — only `NoValue` admits there.
        ("discarded-get", "me.items.get(0)"),
        // `local x = ..` can never bind a `NoValue` result.
        ("novalue-in-local", "local x = me.items.set(0, 1)"),
        // Manifest-absent selectors keep the boundary.
        ("pop", "me.items.pop()"),
        ("insert", "me.items.insert(0, 1)"),
        ("clear", "me.items.clear()"),
        ("contains", "local x = me.items.contains(0)"),
        // A non-`me` receiver is not a self field call.
        ("non-self-receiver", "local p = new Page() p.items.set(0, 1)"),
        // A non-`ArrayBox` field has no container contract.
        ("non-arraybox-field", "me.left.set(0, 1)"),
        // `new` inside an argument is a construction, not a scalar store.
        ("new-arg", "me.items.set(0, new Page())"),
        // A nested call inside an argument is unobserved.
        ("nested-call-arg", "me.items.set(0, me.items.get(0))"),
        // A container literal argument is outside scalar admission.
        ("array-arg", "me.items.set(0, [1, 2])"),
        ("map-arg", "me.items.set(0, %{\"a\" => 1})"),
        // An owned Home argument is an untracked ownership transfer.
        ("owned-arg", "local h = new Page() me.items.set(0, h)"),
        // An `ArrayBox` field read is not a scalar argument.
        ("array-field-arg", "me.items.set(0, me.items)"),
    ] {
        let source = format!(
            "box Page {{ left: i64 items: ArrayBox birth() {{ }}
            touch() {{ {prefix} local item = new Page() return 0 }} }}
            static box Main {{ main() {{ return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        assert!(
            claims.iter().any(|claim| matches!(
                claim.home_prefix(),
                Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(_))
            )),
            "{label}: a claim past the call keeps PrefixNotCovered, got {claims:?}"
        );
    }
}
