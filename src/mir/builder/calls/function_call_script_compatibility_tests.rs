use super::super::RawCompatibilityOrdinaryCallTerminalV1;
use super::*;

use crate::ast::{ASTNode, Span};
use crate::mir::builder::calls::BrandConstructorSourcePortV1;
use crate::mir::builder::calls::RawBrandCallAuthorityV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

#[test]
fn script_root_parked_compatibility_retires_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("script_root/0".to_owned());
    let before_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1), integer(2)],
        RawBrandCallAuthorityV1::ScriptRootParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::ScriptRootRetired
        )
    ));
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("ScriptRoot ordinary calls must retire before argument descent");
    assert_eq!(
        error,
        "[freeze:contract][raw-compat/script-root-ordinary-retired]"
    );
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
    let after_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    assert_eq!(after_instructions, before_instructions);
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
fn raw_legacy_parked_compatibility_retires_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_legacy/0".to_owned());
    let before_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1), integer(2)],
        RawBrandCallAuthorityV1::RawLegacyParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawLegacyRetired
        )
    ));
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("RawLegacy ordinary calls must retire before argument descent");
    assert_eq!(
        error,
        "[freeze:contract][raw-compat/raw-legacy-ordinary-retired]"
    );
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
    let after_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    assert_eq!(after_instructions, before_instructions);
}

#[test]
fn raw_root_main_ordinary_call_retires_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_root_main/0".to_owned());
    let before_instructions = builder.current_function_instructions().len();
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1), integer(2)],
        RawBrandCallAuthorityV1::RawRootMainParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("RawRootMain ordinary calls must retire before argument descent");
    assert_eq!(
        error,
        "[freeze:contract][raw-compat/raw-root-main-ordinary-retired]"
    );
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
    assert_eq!(
        builder.current_function_instructions().len(),
        before_instructions
    );
}

#[test]
fn raw_script_root_ordinary_call_retires_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_script_root/0".to_owned());
    let before_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1), integer(2)],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );

    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawScriptRootRetired
        )
    ));
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("RawScriptRoot ordinary calls must retire before argument descent");
    assert_eq!(
        error,
        "[freeze:contract][raw-compat/raw-script-root-ordinary-retired]"
    );
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
    let after_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .count();
    assert_eq!(after_instructions, before_instructions);
}

#[test]
fn raw_script_root_keeps_brand_and_special_precedence() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("Widget".to_owned(), "Integer".to_owned());
    let brand = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "Widget".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );
    assert!(matches!(
        brand.route,
        PreparedRawFunctionPreflightRouteV1::Brand(_)
    ));

    let typeop = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "isType".to_owned(),
        vec![
            integer(1),
            ASTNode::Literal {
                value: crate::ast::LiteralValue::String("Integer".to_owned()),
                span: Span::unknown(),
            },
        ],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );
    assert!(matches!(
        typeop.route,
        PreparedRawFunctionPreflightRouteV1::TypeOp { .. }
    ));

    let math = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "sqrt".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );
    assert!(matches!(
        math.route,
        PreparedRawFunctionPreflightRouteV1::Math { .. }
    ));

    let mut fastmem_builder = MirBuilder::new();
    fastmem_builder.push_fastmem_region(crate::mir::instruction::FastMemRegionId::new(1));
    let fastmem = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &fastmem_builder,
        "mem.addr".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );
    assert!(matches!(
        fastmem.route,
        PreparedRawFunctionPreflightRouteV1::FastMem { .. }
    ));

    let string = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "str".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::RawScriptRootParkedCompatibility,
    );
    assert!(matches!(
        string.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { .. }
        }
    ));
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

#[test]
fn unclassified_source_rejects_ordinary_call_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("unclassified_source/0".to_owned());
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::UnclassifiedSource,
    );
    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::SourceRejected { ref error }
            if error.contains("unclassified-source")
    ));
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("unclassified ordinary calls must fail before argument descent");
    assert!(error.contains("unclassified-source"));
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
}

#[test]
fn unclassified_source_keeps_special_route_precedence() {
    let builder = MirBuilder::new();
    let math = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "sqrt".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::UnclassifiedSource,
    );
    assert!(matches!(
        math.route,
        PreparedRawFunctionPreflightRouteV1::Math { .. }
    ));
    let string = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "str".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::UnclassifiedSource,
    );
    assert!(matches!(
        string.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { .. }
        }
    ));
}
