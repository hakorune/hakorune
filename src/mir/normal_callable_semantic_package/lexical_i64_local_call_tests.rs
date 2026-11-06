//! Focused boundary tests for `INSTANCE-CALL-SCALAR-ARG-S0` — the
//! lexical-receiver exact-i64 instance-call lane.
//!
//! Positive membership requires the sealed chain: `Lexical(Local)`
//! receiver in the method-call inventory → claim-local `new` receiver
//! binding → unique `InstanceBoxMethod` target → every callee formal
//! `ExactTrivial(I64)` → no `Object`/`NullableObject` result-class claim →
//! every verified value-return a raw Normal-only i64 (literal, exact-i64
//! formal, declared-i64 `me` field, a local of such, `+ - *` of such), plus
//! argument sealing to Integer literals or Integer-class scalar bindings.
//! A selected borrowed-I64 inner call retains its original opaque actual row.
//! Every unselected rejected row keeps the ordinary `PrefixNotCovered`
//! unavailability — the site is never silently claimed, and a callee that
//! uniformly constructs stays on the Handle lane.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, LocalCallObservationV1, LocalCallResultClassV1,
};
use crate::mir::resolved_semantics::SourceBindingSiteV1;

/// Find `(owner, call_site)` for the first method call carrying `selector`
/// anywhere in the package. Each fixture keeps selectors unique.
fn call_site_of(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    selector: &str,
) -> (
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
    crate::mir::resolved_semantics::SourceExprSiteV1,
) {
    package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == selector)
                        .map(|(site, _)| (declaration.owner(), site.clone()))
                })
                .expect("batch loan")
        })
        .unwrap_or_else(|| panic!("call site for {selector} not found"))
}

/// All local-call claim rows across every declaration owner — App Main's
/// completion lives on the root slot, selected callables' in the
/// per-owner completion index.
fn local_calls(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> Vec<LocalCallObservationV1> {
    let mut calls = Vec::new();
    for declaration in package.batch().declarations() {
        if let Some(flow) = package
            .ordinary_new_claim_ledger
            .completion_for_owner(declaration.owner())
            .and_then(|completion| completion.cleanup().root_flow())
        {
            calls.extend(flow.local_calls().iter().cloned());
        }
    }
    calls
}

/// The `new` claim's prefix record — `Err(PrefixNotCovered(_))` is the
/// named unavailability an unclaimed call leaves behind.
fn new_claim_prefix_covered(package: &super::VerifiedNormalCallableSemanticPackageV1) -> bool {
    package
        .ordinary_new_claim_ledger
        .pending_claims_for_test()
        .values()
        .all(|claim| claim.home_prefix().is_ok())
}

#[test]
fn lexical_i64_call_claims_literal_argument() {
    let package = issue(
        "box Pool {
            birth() { }
            allocate(size: i64): i64 { return size }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.allocate(8)
            return r
        } }",
    )
    .expect("literal-argument package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one lexical i64 call claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    assert_eq!(call.arguments(), &[LocalCallArgumentV1::Integer(8)]);
    let (owner, site) = call_site_of(&package, "allocate");
    assert_eq!(
        call.site(),
        &crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone())
    );
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_call_covered(owner, &site),
        "the armed disposition must corroborate the claim"
    );
    let row = package
        .ordinary_new_claim_ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("disposition row");
    assert_eq!(row.result(), Some(InvokeCallResultKind::I64));
    assert_eq!(row.target().name(), "allocate");
    assert_eq!(row.target().owner(), "Pool");
}

#[test]
fn lexical_i64_call_claims_parameter_argument() {
    // The `local r = pool.allocate(size)` shape inside a selected
    // callable — the argument is a parameter binding, not a literal.
    let package = issue(
        "box Pool {
            birth() { }
            allocate(size: i64): i64 { return size }
        }
        static box Helpers {
            lookup(size: i64): i64 {
                local pool = new Pool()
                local r = pool.allocate(size)
                return r
            }
        }
        static box Main { main() { return Helpers.lookup(9) } }",
    )
    .expect("parameter-argument package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one lexical i64 call claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    let [LocalCallArgumentV1::Scalar(binding)] = call.arguments() else {
        panic!(
            "parameter argument must seal as Scalar, got {:?}",
            call.arguments()
        )
    };
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == call.owner())
        .expect("exact owner");
    let parameter = package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            input
                .function()
                .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        })
        .expect("lowering input");
    assert_eq!(Some(*binding), parameter);
}

#[test]
fn lexical_i64_call_claims_zero_argument_call() {
    let package = issue(
        "box Pool {
            birth() { }
            tell(): i64 { return 7 }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.tell()
            return r
        } }",
    )
    .expect("zero-argument package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one zero-arg lexical i64 claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    assert!(call.arguments().is_empty());
    let (owner, site) = call_site_of(&package, "tell");
    let row = package
        .ordinary_new_claim_ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("disposition row");
    assert_eq!(row.result(), Some(InvokeCallResultKind::I64));
}

#[test]
fn lexical_i64_call_claims_arithmetic_over_exact_i64_values() {
    // A computed i64 (`+ - *` over Integer literals and exact-i64 formals)
    // is a no-borrow scalar result: it joins the same lane as a literal or
    // formal return, with no instance-specific path.
    for body in ["return x + 1", "return 1 + 2", "return x * x - 3", "return (x + 1) * 2"] {
        let source = format!(
            "box Calc {{ birth() {{ }}
                add(x: i64): i64 {{ {body} }} }}
            static box Main {{ main() {{
                local c = new Calc()
                local r = c.add(3)
                return r
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{body} package: {issue:?}"));
        assert!(
            local_calls(&package).iter().any(|call| call.result() == LocalCallResultClassV1::I64),
            "{body}: claims an i64 local-call row"
        );
        assert!(new_claim_prefix_covered(&package), "{body}: prefix covered");
    }
}

#[test]
fn lexical_i64_call_claims_exact_i64_receiver_fields_in_arithmetic() {
    for body in ["return me.base + x", "return me.base * 10 + x", "return me.base"] {
        let source = format!(
            "box Calc {{ base: i64
                birth(base) {{ me.base = base }}
                f(x: i64): i64 {{ {body} }} }}
            static box Main {{ main() {{
                local c = new Calc(7)
                local r = c.f(3)
                return r
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{body} package: {issue:?}"));
        assert!(
            local_calls(&package).iter().any(|call| call.result() == LocalCallResultClassV1::I64),
            "{body}: claims an i64 local-call row"
        );
    }
    // Only a declared-i64 `me` field is exact evidence: an untyped field or a
    // missing field stays unclaimed.
    for (label, field, body) in [
        ("untyped-field", "base", "return me.base + x"),
        ("missing-field", "base: i64", "return me.nothere + x"),
    ] {
        let source = format!(
            "box Calc {{ {field}
                birth(base) {{ me.base = base }}
                f(x: i64): i64 {{ {body} }} }}
            static box Main {{ main() {{
                local c = new Calc(7)
                local r = c.f(3)
                return r
            }} }}"
        );
        if let Ok(package) = issue(&source) {
            assert!(
                local_calls(&package).iter().all(|call| call.result() != LocalCallResultClassV1::I64),
                "{label}: unclaimed"
            );
        }
    }
}

#[test]
fn lexical_i64_call_keeps_non_arithmetic_results_unclaimed() {
    // Kept apart from `lexical_i64_call_stays_fail_closed`, whose earlier rows
    // fail before reaching these.
    for (label, method, call) in [
        // Division and comparison are not part of the proved arithmetic set.
        (
            "divide-result",
            "bad(x: i64): i64 { return x / 2 }",
            "pool.bad(4)",
        ),
        (
            "compare-result",
            "bad(x: i64): i64 { return x > 2 }",
            "pool.bad(4)",
        ),
    ] {
        let source = format!(
            "box Pool {{
                size: i64 = 0
                birth() {{ }}
                {method}
            }}
            static box Main {{ main() {{
                local pool = new Pool()
                local r = {call}
                return 0
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        assert!(
            local_calls(&package).iter().all(|call| call.result() != LocalCallResultClassV1::I64),
            "{label}: unclaimed call issues no i64 local-call row"
        );
    }
}

#[test]
fn lexical_i64_call_keeps_construction_callee_on_handle_lane() {
    // `pool.make()` uniformly constructs — the Handle lane owns it; the
    // i64 lane must never claim it first.
    let package = issue(
        "box Page { birth() { } }
        box Pool {
            birth() { }
            make() { return new Page() }
        }
        static box Main { main() {
            local pool = new Pool()
            local h = pool.make()
            return 0
        } }",
    )
    .expect("construction-callee package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one handle-lane claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::Handle);
    let (owner, site) = call_site_of(&package, "make");
    let row = package
        .ordinary_new_claim_ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("disposition row");
    assert_eq!(row.result(), Some(InvokeCallResultKind::Handle));
}

#[test]
fn lexical_i64_call_stays_fail_closed() {
    for (label, method, call) in [
        (
            "float-result",
            "bad(x: i64): i64 { return 1.5 }",
            "pool.bad(1)",
        ),
        (
            "bool-result",
            "bad(x: i64): i64 { return true }",
            "pool.bad(1)",
        ),
        (
            "string-result",
            "bad(x: i64): i64 { return \"s\" }",
            "pool.bad(1)",
        ),
        (
            "null-result",
            "bad(x: i64): i64 { return null }",
            "pool.bad(1)",
        ),
        (
            "mixed-literal-results",
            "bad(x: i64): i64 { if x > 0 { return 1 } return 1.5 }",
            "pool.bad(1)",
        ),
        // A callee formal without `: i64` is `OpaqueHandle`, never
        // `ExactTrivial(I64)`.
        (
            "untyped-formal",
            "allocate(size): i64 { return size }",
            "pool.allocate(8)",
        ),
        // A handle actual is not scalar-i64 evidence.
        (
            "handle-argument",
            "give(p: i64): i64 { return p }",
            "pool.give(page)",
        ),
        // A Bool actual is not scalar-i64 evidence for an i64 formal.
        (
            "bool-argument",
            "check(x: i64): i64 { return x }",
            "pool.check(true)",
        ),
        // A compound argument is outside the scalar seal.
        (
            "non-trivial-arg",
            "bump(x: i64): i64 { return x }",
            "pool.bump(eight + 1)",
        ),
        // A `return new` exit keeps the callee off the i64 lane even
        // when the other exit returns a literal.
        (
            "mixed-exits",
            "mix(x: i64): i64 { if x > 0 { return x } return new Page() }",
            "pool.mix(1)",
        ),
        // A `me.<i64 field>` read is an exact operand now; division still is not.
        (
            "field-division",
            "look(x: i64): i64 { return me.size / 2 }",
            "pool.look(3)",
        ),
    ] {
        let source = format!(
            "box Page {{ birth() {{ }} }}
            box Pool {{
                size: i64 = 0
                birth() {{ }}
                {method}
            }}
            static box Main {{ main() {{
                local pool = new Pool()
                local page = new Page()
                local eight = 8
                local r = {call}
                local page2 = new Page()
                return 0
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        assert!(
            local_calls(&package)
                .iter()
                .all(|call| call.result() != LocalCallResultClassV1::I64),
            "{label}: unclaimed call issues no i64 local-call row"
        );
        assert!(
            !new_claim_prefix_covered(&package),
            "{label}: a `new` past the unclaimed call keeps PrefixNotCovered"
        );
    }
}

#[test]
fn lexical_i64_call_claims_call_result_argument() {
    // `local r = pool.give(pool.allocate(8))` — the inner call sits in
    // direct argument position of a claimed i64 call and seals as a
    // `CallResult` row carrying its own sealed arguments.
    let package = issue(
        "box Pool {
            birth() { }
            allocate(size: i64): i64 { return size }
            give(p: i64): i64 { return p }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.give(pool.allocate(8))
            return r
        } }",
    )
    .expect("call-result-argument package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one lexical i64 call claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    let [LocalCallArgumentV1::CallResult(inner)] = call.arguments() else {
        panic!(
            "the nested call must seal as CallResult, got {:?}",
            call.arguments()
        )
    };
    assert_eq!(inner.arguments(), &[LocalCallArgumentV1::Integer(8)]);
    // The enclosing statement's live prior-Home set is the inner call's
    // unwind evidence too.
    assert_eq!(inner.prior_homes(), call.prior_homes());
    // The inner site is the `allocate` method call — its disposition row
    // corroborates `I64` and can be taken exactly once at emission.
    let (owner, allocate_site) = call_site_of(&package, "allocate");
    assert_eq!(
        inner.site(),
        &crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, allocate_site.clone())
    );
    let row = package
        .ordinary_new_claim_ledger
        .take_lexical_instance_call(owner, &allocate_site)
        .expect("take armed inner row")
        .expect("inner disposition row");
    assert_eq!(row.result(), Some(InvokeCallResultKind::I64));
    assert_eq!(row.target().name(), "allocate");
    assert_eq!(row.target().arity(), 1);
}

#[test]
fn lexical_i64_call_rejects_unproven_call_result_argument() {
    for (label, call) in [
        // The inner callee constructs — Handle result lane, not i64.
        ("inner-construction", "pool.give(pool.make())"),
        // The selected borrowed profile proves this inner source-I64 call.
        ("inner-opaque-formal", "pool.give(pool.echo(8))"),
        // The nested call sits deeper than a direct argument site — a
        // binary-operand subtree never reaches the claim.
        ("deeper-subtree", "pool.give(pool.allocate(8) + 1)"),
        // A Bool actual is not i64 evidence for an inner i64 formal.
        ("inner-bool-arg", "pool.give(pool.allocate(true))"),
    ] {
        let source = format!(
            "box Page {{ birth() {{ }} }}
            box Pool {{
                birth() {{ }}
                allocate(size: i64): i64 {{ return size }}
                make() {{ return new Page() }}
                echo(x) {{ return 0 }}
                give(p: i64): i64 {{ return p }}
            }}
            static box Main {{ main() {{
                local pool = new Pool()
                local r = {call}
                local tail = new Page()
                return 0
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        if label == "inner-opaque-formal" {
            let calls = local_calls(&package);
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].result(), LocalCallResultClassV1::I64);
            let [LocalCallArgumentV1::CallResult(inner)] = calls[0].arguments() else {
                panic!("direct nested result retains its source call");
            };
            assert!(matches!(
                inner.arguments(),
                [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
            ));
            assert!(
                new_claim_prefix_covered(&package),
                "proved nested borrowed call admits the following Home"
            );
            continue;
        }
        assert!(
            local_calls(&package)
                .iter()
                .all(|call| call.result() != LocalCallResultClassV1::I64),
            "{label}: an unproven inner call keeps the outer site unclaimed"
        );
        assert!(
            !new_claim_prefix_covered(&package),
            "{label}: a `new` past the unclaimed call keeps PrefixNotCovered"
        );
    }
}

#[test]
fn lexical_i64_call_rejects_rebound_receiver() {
    let package = issue(
        "box Pool {
            birth() { }
            allocate(size: i64): i64 { return size }
        }
        static box Main { main() {
            local pool = new Pool()
            pool = new Pool()
            local r = pool.allocate(8)
            local page2 = new Pool()
            return 0
        } }",
    )
    .expect("rebound-receiver package");
    assert!(
        local_calls(&package)
            .iter()
            .all(|call| call.result() != LocalCallResultClassV1::I64),
        "a rebound receiver has no sole-initializer claim"
    );
    assert!(!new_claim_prefix_covered(&package));
}

#[test]
fn lexical_i64_call_follows_locals_to_their_initializer_class() {
    // Rule: a local's class is its sole initializer's class.
    for body in [
        "local y = x return y",
        "local y = x local z = y return z",
        "local y = x + 1 return y * 2",
        "local y = me.base return y + x",
    ] {
        let source = format!(
            "box Calc {{ base: i64
                birth(base) {{ me.base = base }}
                f(x: i64): i64 {{ {body} }} }}
            static box Main {{ main() {{
                local c = new Calc(7)
                local r = c.f(3)
                return r
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{body} package: {issue:?}"));
        assert!(
            local_calls(&package).iter().any(|call| call.result() == LocalCallResultClassV1::I64),
            "{body}: claims an i64 local-call row"
        );
    }
    // A local of a Fault-capable operation is an I64 class but not a Normal-only result.
    let source = "box Calc { birth() {} f(x: i64): i64 { local y = x / 2 return y } }
        static box Main { main() {
            local c = new Calc()
            local r = c.f(4)
            return 0
        } }";
    if let Ok(package) = issue(source) {
        assert!(local_calls(&package).iter().all(|call| call.result() != LocalCallResultClassV1::I64));
    }
}
