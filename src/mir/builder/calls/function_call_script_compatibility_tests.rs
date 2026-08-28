use super::*;

use crate::mir::builder::calls::RawBrandCallAuthorityV1;

#[test]
fn script_root_parked_compatibility_preserves_existing_resolved_terminal() {
    let builder = MirBuilder::new();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::ScriptRootParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Resolved { .. }
        }
    ));
}

#[test]
fn script_root_parked_compatibility_keeps_brand_precedence() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("Widget".to_owned(), "Integer".to_owned());
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "Widget".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::ScriptRootParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::Brand(_)
    ));
}
