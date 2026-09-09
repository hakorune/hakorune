//! Dynamic ownership is settled before ordinary Completion diagnostics.
use super::{issue, NormalCallableSemanticPackageIssueV1 as Issue};

fn scan(name: &str) -> String {
    format!(
        "{name}(src, pos: i64, end: i64, pred_chars): i64 {{
        local i = pos
        loop(i < end) {{
            local ch = src.substring(i, i + 1)
            if pred_chars.indexOf(ch) < 0 {{ return i }}
            i = i + 1
        }}
        return i
    }}"
    )
}

#[test]
fn dynamic_owner_error_precedes_ordinary_completion_error() {
    let duplicate = format!(
        "static box Scans {{ {} {} }}",
        scan("first"),
        scan("second")
    );
    let unsupported_result = "static box ResultBox { run(): bool { return true } }";
    assert!(matches!(
        issue(&duplicate),
        Err(Issue::DuplicateDynamicCandidate)
    ));
    assert!(matches!(
        issue(unsupported_result),
        Err(Issue::PhysicalHeader { .. })
    ));
    assert!(matches!(
        issue(&format!("{unsupported_result}\n{duplicate}")),
        Err(Issue::DuplicateDynamicCandidate)
    ));
}

#[test]
fn dynamic_lends_its_original_completion_without_owned_result_row() {
    use super::{
        CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedCallableSemanticRefV1,
        SelectedNormalCallableKeyV1,
    };
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let super::super::model::NormalCallableDynamicProjectionV1::Selected { batch_slot, .. } =
        &package.dynamic
    else {
        panic!("selected Dynamic");
    };
    assert!(package.result_contracts.row(*batch_slot).is_none());
    assert_eq!(
        package.result_contracts.rows().count(),
        3,
        "ordinary siblings retained"
    );
    let mut context = CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed.begin_lowering(&context).unwrap();
    port.with_selected_lowering_input(&key, |input| {
        let SelectedCallableSemanticRefV1::Dynamic { program, .. } = input.semantic() else {
            panic!("same Dynamic owner");
        };
        program.with_canonical_session_authority(|authority| {
            assert!(std::ptr::eq(
                input.result_contract().unwrap().completion_for_test(),
                authority.completion()
            ));
        });
        assert!(
            input.physical_header().is_some(),
            "APrime header remains available"
        );
    })
    .unwrap();
}
