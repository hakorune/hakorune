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
    let package =
        super::super::brand_catalog_tests::issue_with_brand_catalog(PARAM_RECEIVER_SOURCE)
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

/// Parameter receiver proven through a field-read caller edge: `run`'s
/// `inner` is initialized by `me.inner`, and `birth`'s sole write
/// `me.inner = new Inner()` seals the field class. The universal edge
/// proof transports that class into `consume`'s parameter.
#[test]
fn lexical_instance_call_arms_parameter_receiver_with_field_read_edge() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    work(i): i64 { return i + 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    consume(inner): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local v = inner.work(i)
            out = out + v
            i = i + 1
        }
        return out
    }
    run(): i64 {
        local inner = me.inner
        return me.consume(inner)
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("field-read edge source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "work");
    let row = ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("field-read edge disposition row");
    assert_eq!(row.target().owner(), "Inner");
    assert_eq!(row.target().name(), "work");
}

/// Claim-local receiver via field read: `local inner = me.inner` inside
/// `run` carries the sealed field class, so `inner.seal()` arms without
/// any caller edge.
#[test]
fn lexical_instance_call_arms_field_read_claim_local_receiver() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("field-read claim-local source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    let row = ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("field-read claim-local disposition row");
    assert_eq!(row.target().owner(), "Inner");
    assert_eq!(row.target().name(), "seal");
}

/// Call-result receiver proven through the callee's result-class claim:
/// `node` is initialized by `maker.make(i)`, `maker` resolves to
/// `TreeMaker` through the parameter edge (`me.go(maker)` passes a
/// field-read local proven by `me.maker = new TreeMaker()`), and
/// `TreeMaker.make/1` constructs `new TreeNode` on every `return`, so
/// `node.check()` arms.
#[test]
fn lexical_instance_call_arms_call_result_receiver_with_result_class() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box TreeNode {
    init { v }
    birth(v) { me.v = v }
    check(): i64 { return 1 }
}
box TreeMaker {
    birth() { }
    make(i) { return new TreeNode(i) }
}
box Driver {
    init { maker }
    birth() { me.maker = new TreeMaker() }
    go(maker): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local node = maker.make(i)
            local v = node.check()
            out = out + v
            i = i + 1
        }
        return out
    }
    run(): i64 {
        local maker = me.maker
        return me.go(maker)
    }
}
static box Main {
    main() {
        local d = new Driver()
        return d.run()
    }
}
"#,
    )
    .expect("call-result receiver source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "check");
    let row = ledger
        .take_lexical_instance_call(owner, &site)
        .expect("take armed row")
        .expect("call-result receiver disposition row");
    assert_eq!(row.target().owner(), "TreeNode");
    assert_eq!(row.target().name(), "check");
}

/// A callee whose `return` paths disagree on the constructed class
/// carries no result-class claim, so the call-result receiver stays
/// unarmed.
#[test]
fn lexical_instance_call_vetoes_mixed_return_classes() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box TreeNode {
    init { v }
    birth(v) { me.v = v }
    check(): i64 { return 1 }
}
box OtherNode {
    init { v }
    birth(v) { me.v = v }
    check(): i64 { return 2 }
}
box TreeMaker {
    birth() { }
    make(i) {
        if i == 0 {
            return new TreeNode(i)
        }
        return new OtherNode(i)
    }
}
box Driver {
    init { maker }
    birth() { me.maker = new TreeMaker() }
    go(maker): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local node = maker.make(i)
            local v = node.check()
            out = out + v
            i = i + 1
        }
        return out
    }
    run(): i64 {
        local maker = me.maker
        return me.go(maker)
    }
}
static box Main {
    main() {
        local d = new Driver()
        return d.run()
    }
}
"#,
    )
    .expect("mixed return source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "check");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "mixed return classes must veto the result-class claim"
    );
}

/// A callee with no `return` at all (`make` only stores a marker)
/// carries no result-class claim, so the call-result receiver stays
/// unarmed.
#[test]
fn lexical_instance_call_keeps_fallthrough_result_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box TreeNode {
    init { v }
    birth(v) { me.v = v }
    check(): i64 { return 1 }
}
box TreeMaker {
    birth() { }
    make(i) {
        local marker = i
    }
}
box Driver {
    init { maker }
    birth() { me.maker = new TreeMaker() }
    go(maker): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local node = maker.make(i)
            local v = node.check()
            out = out + v
            i = i + 1
        }
        return out
    }
    run(): i64 {
        local maker = me.maker
        return me.go(maker)
    }
}
static box Main {
    main() {
        local d = new Driver()
        return d.run()
    }
}
"#,
    )
    .expect("fallthrough result source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "check");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "a fall-through ending must veto the result-class claim"
    );
}

/// A callee whose `return` value is not a `new` construction carries no
/// result-class claim, so the call-result receiver stays unarmed even
/// though the receiver parameter itself is proven.
#[test]
fn lexical_instance_call_keeps_non_new_return_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box TreeNode {
    init { v }
    birth(v) { me.v = v }
    check(): i64 { return 1 }
}
box TreeMaker {
    init { spare }
    birth() { me.spare = new TreeNode(0) }
    make(i) { return me.spare }
}
box Driver {
    init { maker }
    birth() { me.maker = new TreeMaker() }
    go(maker): i64 {
        local i = 0
        local out = 0
        loop(i < 2) {
            local node = maker.make(i)
            local v = node.check()
            out = out + v
            i = i + 1
        }
        return out
    }
    run(): i64 {
        local maker = me.maker
        return me.go(maker)
    }
}
static box Main {
    main() {
        local d = new Driver()
        return d.run()
    }
}
"#,
    )
    .expect("non-new return source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "check");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "a non-new return value must veto the result-class claim"
    );
}

/// A field written with a non-`new` value carries no class claim:
/// `me.inner = 7` vetoes `inner`, so the loop call stays unarmed.
#[test]
fn lexical_instance_call_keeps_non_new_field_write_unarmed() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = 7 }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("non-new field write source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "a non-new field write must not carry a class claim"
    );
}

/// Two writers disagreeing on the stored class veto the field:
/// `birth` stores `new Inner()` but `swap` stores `new Other()`.
#[test]
fn lexical_instance_call_vetoes_multi_class_field_writers() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Other {
    birth() { }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    swap() { me.inner = new Other() }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("multi-class field writer source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "disagreeing field writers must veto the field claim"
    );
}

/// A field name written through a receiver we cannot attribute vetoes
/// every same-named claim: `o.inner = n` in `main` is not a `me.` write,
/// so `inner` carries no provenance even though `birth` wrote `new`.
#[test]
fn lexical_instance_call_vetoes_unattributed_field_write() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        local n = new Inner()
        o.inner = n
        return o.run()
    }
}
"#,
    )
    .expect("unattributed field write source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "an unattributed write to the field name must veto the claim"
    );
}
