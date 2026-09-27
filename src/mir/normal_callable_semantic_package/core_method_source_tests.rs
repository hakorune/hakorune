//! Canonical/source correspondence must survive the real package install loan.
use super::*;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use std::collections::BTreeMap;

fn package() -> VerifiedNormalCallableSemanticPackageV1 {
    let mut resolver = FunctionSemanticResolverSessionV1::new(977).unwrap();
    issue_normal_callable_semantic_package_v1(
        &mut resolver,
        super::resolved_selected_handoff_tests::final_source(
            r#"
static box Scan {
  run(s) {
    local i = 0
    loop(i < s.length()) {
      local ch = s.substring(i, i + 1)
      i = i + 1
    }
    return i
  }
}
"#,
        ),
    )
    .expect("source-backed package")
}

fn key() -> SelectedNormalCallableKeyV1 {
    SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Scan", "run", 1),
    )
}

#[test]
fn selected_loan_retains_canonical_caller_and_exact_source_owner() {
    let mut context = CompilationContext::new();
    let installed = package().prepare_install(&mut context).unwrap().commit();
    let mut port = installed.begin_lowering(&context).unwrap();
    let mut count = 0;
    port.with_selected_lowering_input_and_core_methods(&key(), |input, rows, _| {
        count = rows.len();
        for (site, row) in rows {
            row.require_selected(input.selected_key(), input.source().owner())?;
            assert_eq!(&site, row.contract().call_site());
            let contract = row.into_unconditional_contract()?;
            assert_eq!(contract.owner(), input.source().owner());
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(
        count, 2,
        "length and substring must both cross the package loan"
    );
    assert!(port
        .with_selected_lowering_input_and_core_methods(&key(), |_, _, _| Ok(()))
        .is_err());
}

#[test]
fn canonical_caller_substitution_is_rejected() {
    let mut context = CompilationContext::new();
    let installed = package().prepare_install(&mut context).unwrap().commit();
    let mut port = installed.begin_lowering(&context).unwrap();
    port.with_selected_lowering_input_and_core_methods(&key(), |input, rows, _| {
        assert_eq!(rows.len(), 2);
        let foreign = SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::test_static_box_method("Other", "run", 1),
        );
        for row in rows.values() {
            assert_eq!(
                row.require_selected(&foreign, input.source().owner())
                    .unwrap_err(),
                "[freeze:contract][named-array/selected-source-mismatch]"
            );
        }
        Ok(())
    })
    .unwrap();
}

#[test]
fn same_key_from_another_package_cannot_supply_source_owner() {
    let mut first_context = CompilationContext::new();
    let first = package()
        .prepare_install(&mut first_context)
        .unwrap()
        .commit();
    let mut second_context = CompilationContext::new();
    let second = package()
        .prepare_install(&mut second_context)
        .unwrap()
        .commit();
    let mut first_port = first.begin_lowering(&first_context).unwrap();
    let mut second_port = second.begin_lowering(&second_context).unwrap();
    first_port
        .with_selected_lowering_input_and_core_methods(&key(), |_, rows, _| {
            assert_eq!(rows.len(), 2);
            second_port
                .with_selected_lowering_input_and_core_methods(&key(), |input, _, _| {
                    for row in rows.values() {
                        assert_eq!(
                            row.require_selected(input.selected_key(), input.source().owner())
                                .unwrap_err(),
                            "[freeze:contract][named-array/selected-source-mismatch]"
                        );
                    }
                    Ok(())
                })
                .unwrap();
            Ok(())
        })
        .unwrap();
}

type GetRows = BTreeMap<
    SelectedNormalCallableKeyV1,
    BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
>;

/// The package loan omits named-array issuance without the brand catalog, so
/// these pins exercise the same issuer the production `Some(catalog)` arm runs.
fn field_resident_get_rows(body: &str) -> Result<GetRows, String> {
    let source = format!(
        r#"box Store {{
  init {{ ids }}
  birth() {{ me.ids = new ArrayBox() }}
  collect() {{ {body} }}
}}"#
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(984).unwrap();
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        super::resolved_selected_handoff_tests::final_source(&source),
    )
    .expect("package issue");
    let ast = crate::parser::NyashParser::parse_from_string(&source).unwrap();
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&ast).unwrap();
    super::core_method_source::issue_source_core_method_calls_with_named_arrays_v1(
        &package.catalog,
        &package.batch,
        &package.selected,
        &brands,
        package.instance_constructors(),
    )
    .map(|(rows, _)| rows)
}

fn collect_key() -> SelectedNormalCallableKeyV1 {
    SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_instance_box_method("Store", "collect", 0),
    )
}

#[test]
fn field_resident_array_get_issues_plain_core_method_contract() {
    let rows = field_resident_get_rows(
        r#"
    local ids = me.ids
    local i = 0
    local n = ids.length()
    loop(i < n) {
      local x = ids.get(i)
      i = i + 1
    }
    return i
"#,
    )
    .expect("named-array issue");
    let rows = rows.get(&collect_key()).expect("collect rows");
    assert_eq!(rows.len(), 1, "only the loop-body get is armed");
    let (_, row) = rows.iter().next().unwrap();
    let contract = row.contract();
    assert_eq!(
        contract.target().row().row().op,
        crate::mir::core_method_op::CoreMethodOp::ArrayGet
    );
    assert_eq!(
        contract.target().result(),
        crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::DynamicToCaller
    );
    assert_eq!(
        contract.placement(),
        crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body
    );
    assert!(contract.named_array_requirement().is_none());
}

#[test]
fn field_resident_get_rejects_receiver_rebind_and_non_integer_index() {
    for (body, token) in [
        (
            r#"
    local ids = me.ids
    local i = 0
    loop(i < 1) {
      local x = ids.get(i)
      ids = me.ids
      i = i + 1
    }
    return i
"#,
            "ReassignedReceiver",
        ),
        (
            r#"
    local ids = me.ids
    local i = 0
    loop(i < 1) {
      local x = ids.get("s")
      i = i + 1
    }
    return i
"#,
            "IntegerSourceMissing",
        ),
    ] {
        let error = field_resident_get_rows(body).unwrap_err();
        assert!(error.contains(token), "{token} absent from {error}");
    }
}

#[test]
fn non_resident_get_stays_unarmed() {
    for body in [
        // Construction-alias receiver — no field-residence claim.
        r#"
    local ids = new ArrayBox()
    local i = 0
    loop(i < 1) {
      local x = ids.get(i)
      i = i + 1
    }
    return i
"#,
        // Unaliased `me` field access — receiver is not lexical local.
        r#"
    local i = 0
    loop(i < 1) {
      local x = me.ids.get(i)
      i = i + 1
    }
    return i
"#,
        // Condition placement — the read arm is Body-only.
        r#"
    local ids = me.ids
    local i = 0
    loop(ids.get(i) == i) {
      i = i + 1
    }
    return i
"#,
    ] {
        let rows = field_resident_get_rows(body).expect("named-array issue");
        let empty = rows.get(&collect_key()).map_or(0, |rows| rows.len());
        assert_eq!(empty, 0, "unarmed get must not mint a contract");
    }
}
