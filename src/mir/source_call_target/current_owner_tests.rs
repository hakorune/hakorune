use crate::mir::builder::SameModuleCallableNamespaceV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::test_support::*;
use super::*;

const CURRENT_OWNER_SOURCE: &str = r#"
static box Helpers {
  call(x) { return me.target(x) }
  target(x) { return x }
  zero() { return me.zero_target() }
  zero_target() { return 0 }
  qualified(x) { return Foreign.target(x) }
}
static box Foreign { target(x) { return x } }
"#;

fn static_key(
    declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    key(
        declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        owner,
        method,
        arity,
    )
}

#[test]
fn seals_current_owner_static_target_from_exact_site() {
    let declarations = catalog(CURRENT_OWNER_SOURCE);
    let caller = static_key(&declarations, "Helpers", "call", 1);
    let call_site = return_site();
    let call = exact_call(&declarations, &caller, call_site.clone());
    let targets = empty_targets(&declarations)
        .extend_current_owner([&call])
        .unwrap();

    let row = current_owner(&targets, &caller, &call_site);
    assert_eq!(row.receiver(), CurrentOwnerStaticReceiverV1::CanonicalMe);
    assert_eq!(row.target().owner(), "Helpers");
    assert_eq!(row.target().name(), "target");
    assert_eq!(row.target().arity(), 1);
}

#[test]
fn zero_arity_and_qualified_rows_share_one_branded_catalog() {
    let declarations = catalog(CURRENT_OWNER_SOURCE);
    let imports = empty_imports(&declarations);
    let qualified_caller = static_key(&declarations, "Helpers", "qualified", 1);
    let qualified_call = exact_call(&declarations, &qualified_caller, return_site());
    let lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&qualified_call]).unwrap();
    let facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&qualified_call, &lexical, &imports).unwrap();

    let current_caller = static_key(&declarations, "Helpers", "zero", 0);
    let current_site = return_site();
    let current_call = exact_call(&declarations, &current_caller, current_site.clone());
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [facts])
        .unwrap()
        .extend_current_owner([&current_call])
        .unwrap();

    assert_eq!(targets.len(), 2);
    assert_eq!(
        current_owner(&targets, &current_caller, &current_site)
            .target()
            .name(),
        "zero_target"
    );
}

#[test]
fn actual_string_helpers_projects_exact_digit_value_site() {
    let declarations = catalog(include_str!(concat!(
        "../../../lang/src/shared/common/",
        "string_helpers.hako"
    )));
    let caller = static_key(&declarations, "StringHelpers", "to_i64", 1);
    let call_site = site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::LoopBody(2),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let call = exact_call(&declarations, &caller, call_site.clone());
    let targets = empty_targets(&declarations)
        .extend_current_owner([&call])
        .unwrap();

    let row = current_owner(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "StringHelpers");
    assert_eq!(row.target().name(), "_digit_value");
    assert_eq!(row.target().arity(), 1);
}

#[test]
fn non_me_exact_method_call_cannot_enter_current_owner_route() {
    let declarations = catalog(CURRENT_OWNER_SOURCE);
    let caller = static_key(&declarations, "Helpers", "qualified", 1);
    let call = exact_call(&declarations, &caller, return_site());

    assert_eq!(
        empty_targets(&declarations)
            .extend_current_owner([&call])
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CanonicalMeReceiverRequired {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn instance_method_caller_cannot_enter_static_current_owner_route() {
    let declarations =
        catalog("box Ordinary { call(x) { return me.target(x) } target(x) { return x } }");
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::InstanceBoxMethod,
        "Ordinary",
        "call",
        1,
    );
    let call = exact_call(&declarations, &caller, return_site());

    assert_eq!(
        empty_targets(&declarations)
            .extend_current_owner([&call])
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CallerNotStaticBoxMethod { caller }
    );
}

#[test]
fn equal_foreign_catalog_call_rejects_before_target_lookup() {
    let left = catalog(CURRENT_OWNER_SOURCE);
    let right = catalog(CURRENT_OWNER_SOURCE);
    let caller = static_key(&right, "Helpers", "call", 1);
    let call = exact_call(&right, &caller, return_site());

    assert_eq!(
        empty_targets(&left)
            .extend_current_owner([&call])
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CallCatalogMismatch {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn duplicate_exact_current_owner_site_rejects_atomically() {
    let declarations = catalog(CURRENT_OWNER_SOURCE);
    let caller = static_key(&declarations, "Helpers", "call", 1);
    let call = exact_call(&declarations, &caller, return_site());

    assert_eq!(
        empty_targets(&declarations)
            .extend_current_owner([&call, &call])
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::DuplicateCallSite {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn missing_current_owner_target_rejects() {
    let declarations = catalog("static box Helpers { call(x) { return me.absent(x) } }");
    let caller = static_key(&declarations, "Helpers", "call", 1);
    let call = exact_call(&declarations, &caller, return_site());

    assert_eq!(
        empty_targets(&declarations)
            .extend_current_owner([&call])
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
            owner: "Helpers".into(),
            method: "absent".into(),
            arity: 1,
        }
    );
}

#[test]
fn declaration_reorder_preserves_current_owner_target() {
    let reordered = r#"
static box Foreign { target(x) { return x } }
static box Helpers {
  qualified(x) { return Foreign.target(x) }
  zero_target() { return 0 }
  zero() { return me.zero_target() }
  target(x) { return x }
  call(x) { return me.target(x) }
}
"#;

    fn normalized(source: &str) -> (String, String, u32) {
        let declarations = catalog(source);
        let caller = static_key(&declarations, "Helpers", "call", 1);
        let call = exact_call(&declarations, &caller, return_site());
        let targets = empty_targets(&declarations)
            .extend_current_owner([&call])
            .unwrap();
        let row = current_owner(&targets, &caller, &return_site());
        (
            row.target().owner().to_string(),
            row.target().name().to_string(),
            row.target().arity(),
        )
    }

    assert_eq!(normalized(CURRENT_OWNER_SOURCE), normalized(reordered));
}
