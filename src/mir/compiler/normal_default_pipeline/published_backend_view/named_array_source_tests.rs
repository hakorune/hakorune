//! Natural source through the production package, sole write, final handoff and
//! C-frame boundary. Query replies here are controlled observations, not C execution.
use super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;
use crate::mir::{ArrayElementWriteKind, MirInstruction};

fn source(body: &str) -> String {
    format!("static box Scan {{ run(s) {{ local arr = new ArrayBox() local i = 0 loop(i < 1) {{ {body} i = i + 1 }} return i }} }}")
}

#[test]
fn named_array_source_reaches_retained_typed_write_and_c_frame() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        for body in [
            "arr.push(\"text\")",
            "arr.push(s.substring(0, 1))",
            "arr.push(\"one\") arr.push(\"two\")",
        ] {
            for optimize in [false, true] {
                let expected = if body.contains("two") { 2 } else { 1 };
                let mut calls = 0;
                MirCompiler::with_options(optimize).compile_normal_with_published(
                    published_request(&source(body)), |view, verification| -> Result<(), String> {
                        calls += 1;
                        assert!(verification.is_ok(), "{verification:?}");
                        assert_eq!(view.validated_named_arrays()?.len(), expected);
                        let writes = view.module().functions.values().flat_map(|f| f.blocks.values())
                            .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                                MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Push, dst: None, index: None, .. })).count();
                        assert_eq!(writes, expected);
                        assert!(!view.has_lifecycle_instructions());
                        crate::mir::backend_capability::enforce_published_backend_supported(view, "ny-llvmc-obj")?;
                        crate::runner::mir_json_emit::emit_published_view_body(view)?;
                        let mut queries = 0;
                        c_transport_v2::PublishedStaticMethodCFrameV2::from_view_with_query(view, |_, _, _| {
                            queries += 1;
                            Ok(Some(map_named_allocations::NamedAllocationConsumer::Array))
                        })?;
                        assert_eq!(queries, 1);
                        assert!(c_transport_v2::PublishedStaticMethodCFrameV2::from_view_with_query(view, |_, _, _| {
                            Ok(Some(map_named_allocations::NamedAllocationConsumer::DirectArray))
                        }).unwrap_err().contains("array-capability-unsupported"));
                        let generic = PublishedMirBackendView::try_new(view.module()).unwrap();
                        assert!(generic.validated_named_arrays().is_err());
                        Ok(())
                    }
                ).unwrap_or_else(|error| panic!("{body}, optimize={optimize}: {error:?}"));
                assert_eq!(calls, 1);
            }
        }
    });
}

#[test]
fn named_array_value_demand_rejects_before_published_consumer() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        let mut reached = false;
        let result = MirCompiler::with_options(false).compile_normal_with_published(
            published_request(&source("local result = arr.push(\"text\")")),
            |_, _| -> Result<(), String> {
                reached = true;
                Ok(())
            },
        );
        assert!(!reached);
        let error = match result {
            Err(error) => format!("{error:?}"),
            Ok(_) => panic!("value-demand must reject"),
        };
        assert!(error.contains("ValueDemand"), "{error}");
    });
}
