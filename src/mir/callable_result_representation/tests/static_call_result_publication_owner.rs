use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::VerifiedStaticCallResultPublicationOwnerV1;
use super::support::{
    declarations, extend_current_owner_targets, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

const SOURCE: &str = r#"
static box StringHelpers {
  int_to_str(n) { local value = me.to_i64("x") return value }
  to_i64(x) { return x + 1 }
}
"#;

fn call_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

#[test]
fn issuer_keeps_exact_source_row_and_consumes_it_once() {
    let declarations = declarations(SOURCE);
    let targets = qualified_targets(&declarations, &[], &[]);
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "int_to_str",
            caller_arity: 1,
            site: call_site(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("source-bound owner must issue exact rows");
    assert_eq!(owner.len(), 1);

    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);
    let handoff = owner
        .take(&declarations, &caller, &call_site(), &target)
        .expect("branded take must verify")
        .expect("exact row must be available");
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.target(), &target);
    assert!(owner
        .take(&declarations, &caller, &call_site(), &target)
        .expect("second lookup remains well-formed")
        .is_none());
}
