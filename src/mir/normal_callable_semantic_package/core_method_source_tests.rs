//! Canonical/source correspondence must survive the real package install loan.
use super::*;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;

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
