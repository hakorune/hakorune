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

type IndexOfRow = (
    crate::mir::core_method_op::CoreMethodOp,
    crate::mir::resolved_semantics::ResolvedLoopPlacementV1,
    Vec<crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1>,
    crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1,
);

/// The `StringIndexOf/1` arm runs inside the generic StringBox issuer — the
/// package loan exercises the exact production path with no brand catalog.
fn index_of_rows(param_decl: &str, body: &str, arity: usize) -> Result<Vec<IndexOfRow>, String> {
    let source = format!(
        r#"
static box Scan {{
  run({param_decl}) {{
    local alphabet = "abc"
{body}
  }}
}}
"#
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(991).unwrap();
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        super::resolved_selected_handoff_tests::final_source(&source),
    )
    .expect("source-backed package");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Scan", "run", arity),
    );
    let mut context = CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    let mut port = installed.begin_lowering(&context).unwrap();
    let mut rows = Vec::new();
    port.with_selected_lowering_input_and_core_methods(&key, |_, source_rows, _| {
        for (_, row) in source_rows {
            let contract = row.contract();
            rows.push((
                contract.target().row().row().op,
                contract.placement(),
                contract.target().parameters().to_vec(),
                contract.target().result(),
            ));
        }
        Ok(())
    })
    .map_err(|error| format!("{error:?}"))?;
    Ok(rows)
}

#[test]
fn literal_receiver_index_of_arms_body_with_text_parameter() {
    let rows = index_of_rows(
        "",
        r#"
    local ch = "x"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.indexOf(ch)
      i = i + 1
    }
    return i
"#,
        0,
    )
    .expect("index-of issue");
    assert_eq!(
        rows,
        vec![(
            crate::mir::core_method_op::CoreMethodOp::StringIndexOf,
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
            vec![crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::TextParameter],
            crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller,
        )],
        "exactly one StringIndexOf Body contract with a borrowed text needle"
    );
}

#[test]
fn substring_contract_supplies_index_of_needle_text() {
    let rows = index_of_rows(
        "",
        r#"
    local text = "abc"
    local i = 0
    loop(i < 1) {
      local ch = text.substring(i, i + 1)
      local idx = alphabet.indexOf(ch)
      i = i + 1
    }
    return i
"#,
        0,
    )
    .expect("index-of issue");
    let ops: Vec<_> = rows.iter().map(|row| row.0).collect();
    assert_eq!(
        ops,
        vec![
            crate::mir::core_method_op::CoreMethodOp::StringSubstring,
            crate::mir::core_method_op::CoreMethodOp::StringIndexOf,
        ],
        "substring mints the TextToCaller contract the needle proof follows"
    );
}

#[test]
fn find_alias_follows_the_same_text_evidence_gate() {
    for body in [
        r#"
    local ch = "x"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.find(ch)
      i = i + 1
    }
    return i
"#,
    ] {
        let rows = index_of_rows("", body, 0).expect("find issue");
        assert_eq!(
            rows.iter().map(|row| row.0).collect::<Vec<_>>(),
            vec![crate::mir::core_method_op::CoreMethodOp::StringIndexOf],
            "the manifest alias mints the same StringIndexOf contract"
        );
    }
    // The alias never relaxes evidence — a non-text receiver stays unarmed.
    let rows = index_of_rows(
        "",
        r#"
    local arr = new ArrayBox()
    local i = 0
    loop(i < 1) {
      local idx = arr.find("x")
      i = i + 1
    }
    return i
"#,
        0,
    )
    .expect("find issue");
    assert_eq!(rows.len(), 0);
}

#[test]
fn index_of_stays_unarmed_without_text_evidence() {
    for body in [
        // ArrayBox receiver — the runtime router also routes
        // `ArrayBox.indexOf/1`; it must not mint StringIndexOf.
        r#"
    local arr = new ArrayBox()
    local i = 0
    loop(i < 1) {
      local idx = arr.indexOf("x")
      i = i + 1
    }
    return i
"#,
        // Untyped receiver — no initializer text proof.
        r#"
    local arr = 7
    local i = 0
    loop(i < 1) {
      local idx = arr.indexOf("x")
      i = i + 1
    }
    return i
"#,
        // Non-text needle — `1` carries no text evidence.
        r#"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.indexOf(1)
      i = i + 1
    }
    return i
"#,
        // Rebound receiver — the literal proof covers only the
        // initializer, not later assignments.
        r#"
    alphabet = "xy"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.indexOf("x")
      i = i + 1
    }
    return i
"#,
        // Arity 2 — the bounded arm covers `indexOf/1` only.
        r#"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.indexOf("x", 0)
      i = i + 1
    }
    return i
"#,
        // `lastIndexOf` stays outside the armed family.
        r#"
    local i = 0
    loop(i < 1) {
      local idx = alphabet.lastIndexOf("x")
      i = i + 1
    }
    return i
"#,
    ] {
        let rows = index_of_rows("", body, 0).expect("index-of issue");
        assert_eq!(rows.len(), 0, "unarmed indexOf must mint no contract");
    }
    // Parameter receiver — no initializer relation exists at all.
    let rows = index_of_rows(
        "pred_chars",
        r#"
    local i = 0
    loop(i < 1) {
      local idx = pred_chars.indexOf("x")
      i = i + 1
    }
    return i
"#,
        1,
    )
    .expect("index-of issue");
    assert_eq!(rows.len(), 0);
}

#[test]
fn index_of_condition_placement_stays_rejected() {
    // Body-only arm: a condition site mints no contract and stays unarmed
    // rather than erroring — the placement filter declines before issue.
    let rows = index_of_rows(
        "",
        r#"
    local ch = "x"
    local i = 0
    loop(alphabet.indexOf(ch) >= 0) {
      i = i + 1
    }
    return i
"#,
        0,
    )
    .expect("index-of issue");
    assert_eq!(rows.len(), 0);
}
