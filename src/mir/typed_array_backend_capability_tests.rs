//! Source-issued native input coverage; mutation cases do not grant admission.
use super::*;
use crate::mir::function::{
    PublishedMirBackendView, TypedArrayBoundaryValue, TypedArrayContractBoundary,
};
use crate::mir::{MirCompiler, NormalCompileRequestV1, ValueId};

#[test]
fn native_input_requires_retained_identity_and_exact_carrier_coverage() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
        materialize_normal_callable_program_v1(
            "local a: Array<i8> = [7]\nlocal b: Array<u8> = []\nreturn 30",
            Default::default(),
        )
        .unwrap()
    else {
        panic!("retained source");
    };
    MirCompiler::with_options(true)
        .compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok());
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(
                    view, &input,
                )?;
                let generic = PublishedMirBackendView::try_new(view.module()).unwrap();
                assert!(enforce_lifecycle_input(&generic, &input).is_err());
                let foreign = view.module().clone();
                assert!(
                    crate::mir::exact_numeric_backend_capability::enforce_lifecycle_input(
                        &foreign, &input
                    )
                    .is_err()
                );
                let root = input.program().functions()[0].name();
                for mutation in 0..8 {
                    let mut module = view.module().clone();
                    let rows = &mut module
                        .functions
                        .get_mut(root)
                        .unwrap()
                        .metadata
                        .typed_array_element_contracts;
                    match mutation {
                        0 => {
                            rows.pop();
                        }
                        1 => rows.push(rows[0].clone()),
                        2 => rows[0].contract_id.push_str("-drift"),
                        3 => {
                            rows[0].boundary_value =
                                TypedArrayBoundaryValue::Value(ValueId::new(u32::MAX))
                        }
                        4 => rows[0].element_spec = rows[1].element_spec,
                        5 => rows[0].boundary = TypedArrayContractBoundary::ParameterEntry,
                        6 => rows[0].boundary = TypedArrayContractBoundary::ReturnExit,
                        7 => {
                            let mut extra = module.functions[root].clone();
                            extra.signature.name = "unselected".into();
                            module.add_function(extra);
                        }
                        _ => unreachable!(),
                    }
                    assert!(
                        enforce_claim_coverage(&module, &input).is_err(),
                        "mutation {mutation}"
                    );
                }
                Ok(())
            },
        )
        .unwrap();
}
