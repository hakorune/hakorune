use super::*;

use crate::ast::{ASTNode, Span};
use crate::mir::builder::calls::BrandConstructorSourcePortV1;
use crate::mir::builder::calls::RawBrandCallAuthorityV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

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

#[test]
fn raw_compatibility_provenance_preserves_resolved_terminal() {
    let builder = MirBuilder::new();
    for authority in [
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
        RawBrandCallAuthorityV1::RawRootMainParkedCompatibility,
        RawBrandCallAuthorityV1::RawLegacyParkedCompatibility,
    ] {
        let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
            &builder,
            "helper".to_owned(),
            vec![integer(1)],
            authority,
        );
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::Resolved { .. }
            }
        ));
    }
}

#[test]
fn raw_compatibility_provenance_keeps_brand_precedence() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("Widget".to_owned(), "Integer".to_owned());
    for authority in [
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
        RawBrandCallAuthorityV1::RawRootMainParkedCompatibility,
        RawBrandCallAuthorityV1::RawLegacyParkedCompatibility,
    ] {
        let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
            &builder,
            "Widget".to_owned(),
            vec![integer(1)],
            authority,
        );
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Brand(_)
        ));
    }
}

#[test]
fn raw_legacy_port_issues_named_compatibility_provenance() {
    let call = ASTNode::FunctionCall {
        name: "helper".to_owned(),
        arguments: Vec::new(),
        span: Span::unknown(),
    };
    let mut port = RawLegacyChildLoweringPortV1;
    assert_eq!(
        port.brand_call_authority_v1(&call).unwrap(),
        RawBrandCallAuthorityV1::RawLegacyParkedCompatibility
    );
}
