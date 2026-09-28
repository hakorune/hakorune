//! Lexical instance-call disposition pins: parameter receivers proven by
//! caller-side ordinary-new claim edges and claim-local receivers proven by
//! their sole initializer. Unproven shapes — call-result arguments, rebound
//! parameters, ambiguous caller classes, call-result receivers — must stay
//! unarmed so the loop route coverage names them.

/// Find `(owner, call_site)` for the first method call carrying `selector`
/// anywhere in the package. Each fixture keeps selectors unique.
fn call_site_of(
    package: &super::super::VerifiedNormalCallableSemanticPackageV1,
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

const PARAM_RECEIVER_SOURCE: &str = r#"
box ParamStore {
    birth() { }
    readData(cid): i64 { return cid + 1 }
    release(cid) { }
}
box ParamManifest {
    birth() { }
    materialize(store): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local data = store.readData(i)
            out = out + data
            i = i + 1
        }
        return out
    }
    releaseAll(store) {
        local i = 0
        loop(i < 2) {
            store.release(i)
            i = i + 1
        }
    }
}
static box Main {
    main() {
        local store = new ParamStore()
        local manifest = new ParamManifest()
        local a = manifest.materialize(store)
        local b = manifest.releaseAll(store)
        return 0
    }
}
"#;

/// Parameter receiver co-sealed: `store` is `Parameter{0}` of the callee,
/// every caller edge passes a `new ParamStore()` claim local, and the
/// `InstanceBoxMethod` target resolves uniquely. Both the value-position
/// (`readData`) and statement-position (`release`) rows must be armed.
#[test]
fn lexical_instance_call_arms_parameter_receiver_with_claim_edge() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        PARAM_RECEIVER_SOURCE,
    )
    .expect("parameter receiver source package");
    let ledger = &package.ordinary_new_claim_ledger;
    for (selector, target_owner, arity) in [
        ("readData", "ParamStore", 1u32),
        ("release", "ParamStore", 1u32),
    ] {
        let (owner, site) = call_site_of(&package, selector);
        assert!(
            ledger.lexical_instance_call_covered(owner, &site),
            "{selector} must be armed"
        );
        let row = ledger
            .take_lexical_instance_call(owner, &site)
            .expect("take armed row")
            .expect("disposition row");
        assert_eq!(row.target().name(), selector);
        assert_eq!(row.target().owner(), target_owner);
        assert_eq!(row.target().arity(), arity);
        assert_eq!(
            row.target().namespace(),
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
        );
        assert_eq!(row.receiver_binding().owner(), owner);
        // A second take of the same site is a named one-shot violation.
        let error = ledger
            .take_lexical_instance_call(owner, &site)
            .expect_err("armed site is one-shot");
        assert!(error.contains("already-taken"), "{error}");
    }
}

/// Claim-local receiver: `local manifest = new ParamManifest()` inside the
/// callee proves the class without any caller edge.
#[test]
fn lexical_instance_call_arms_claim_local_receiver() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box ParamManifest {
    birth() { }
    seal(): i64 { return 1 }
}
box ParamDriver {
    birth() { }
    run(): i64 {
        local manifest = new ParamManifest()
        local i = 0
        loop(i < 2) {
            local r = manifest.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local d = new ParamDriver()
        return d.run()
    }
}
"#,
    )
    .expect("claim-local receiver source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    let row = ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("claim-local disposition row");
    assert_eq!(row.target().name(), "seal");
    assert_eq!(row.target().owner(), "ParamManifest");
}

/// A call-result argument carries no ordinary-new claim, so the callee's
/// parameter stays unarmed — issuance still succeeds; the loop route names
/// the uncovered site at lowering time.
#[test]
fn lexical_instance_call_keeps_call_result_argument_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box ParamStore {
    birth() { }
    readData(cid): i64 { return cid + 1 }
}
box ParamManifest {
    birth() { }
    make() { local s = new ParamStore() return s }
    materialize(store): i64 {
        local i = 0
        loop(i < 2) {
            local data = store.readData(i)
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local m = new ParamManifest()
        local s = m.make()
        return m.materialize(s)
    }
}
"#,
    )
    .expect("call-result argument source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "readData");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "call-result argument must not arm the parameter receiver"
    );
    assert!(
        ledger
            .take_lexical_instance_call(owner, &site)
            .expect("take")
            .is_none(),
        "unarmed site must not carry a disposition"
    );
}

/// A callee-side call-result receiver (`local m2 = m.make()`) has no
/// initializer claim, so `m2.seal()` stays unarmed.
#[test]
fn lexical_instance_call_keeps_call_result_receiver_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box ParamManifest {
    birth() { }
    make() { local m = new ParamManifest() return m }
    seal(): i64 { return 1 }
}
box ParamDriver {
    birth() { }
    run(): i64 {
        local m = new ParamManifest()
        local m2 = m.make()
        local i = 0
        loop(i < 2) {
            local r = m2.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local d = new ParamDriver()
        return d.run()
    }
}
"#,
    )
    .expect("call-result receiver source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "call-result receiver must not arm"
    );
}

/// Caller edges disagreeing on the argument's claim class veto the
/// parameter: `materialize` is entered with `ParamStore` and `ParamOther`
/// arguments, so no single class can be proven.
#[test]
fn lexical_instance_call_vetoes_ambiguous_argument_classes() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box ParamStore {
    birth() { }
    readData(cid): i64 { return cid + 1 }
}
box ParamOther {
    birth() { }
}
box ParamManifest {
    birth() { }
    materialize(store): i64 {
        local i = 0
        loop(i < 2) {
            local data = store.readData(i)
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local s = new ParamStore()
        local o = new ParamOther()
        local m = new ParamManifest()
        local a = m.materialize(s)
        local b = m.materialize(o)
        return 0
    }
}
"#,
    )
    .expect("ambiguous argument source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "readData");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "disagreeing caller classes must veto the parameter"
    );
}

/// A rebound parameter can hold a different object at call time; the
/// receiver call must stay unarmed.
#[test]
fn lexical_instance_call_keeps_rebound_parameter_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box ParamStore {
    birth() { }
    readData(cid): i64 { return cid + 1 }
}
box ParamManifest {
    birth() { }
    materialize(store): i64 {
        local s2 = new ParamStore()
        store = s2
        local i = 0
        loop(i < 2) {
            local data = store.readData(i)
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local store = new ParamStore()
        local m = new ParamManifest()
        return m.materialize(store)
    }
}
"#,
    )
    .expect("rebound parameter source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "readData");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "rebound parameter must not arm"
    );
}
