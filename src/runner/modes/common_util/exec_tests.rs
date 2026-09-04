use super::{
    append_ny_llvmc_extra_libs_arg, build_ny_llvmc_emit_exe_command,
    ny_llvmc_driver_arg_from_backend, selected_dynamic_aot_metadata_present,
    validate_selected_dynamic_boundary_route_values, with_retained_mir_path,
};

use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

#[test]
fn rejects_native_backend_selector_for_runner_route() {
    let err = ny_llvmc_driver_arg_from_backend(Some("native")).unwrap_err();
    assert!(err.contains("canary-only"));
    let err = ny_llvmc_driver_arg_from_backend(Some(" native ")).unwrap_err();
    assert!(err.contains("ny-llvmc --driver native"));
}

#[test]
fn ignores_empty_or_non_native_backend_values() {
    assert_eq!(ny_llvmc_driver_arg_from_backend(None).unwrap(), None);
    assert_eq!(ny_llvmc_driver_arg_from_backend(Some("")).unwrap(), None);
    assert_eq!(ny_llvmc_driver_arg_from_backend(Some("crate")).unwrap(), None);
    assert_eq!(ny_llvmc_driver_arg_from_backend(Some("llvmlite")).unwrap(), None);
}

#[test]
fn selected_dynamic_census_keeps_ordinary_module_on_generic_route() {
    let module = MirModule::new("ordinary".to_owned());
    assert!(!selected_dynamic_aot_metadata_present(&module).unwrap());
}

#[test]
fn selected_dynamic_census_rejects_scrubbed_clone() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ParserScanLoopBox.skip_while/4".to_owned(),
            params: vec![MirType::Unknown; 4],
            return_type: MirType::Integer,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    function
        .metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(),
        )
        .expect("admission install");

    let scrubbed_function = function.clone();
    let mut module = MirModule::new("selected".to_owned());
    module.add_function(function);
    assert!(selected_dynamic_aot_metadata_present(&module).unwrap());

    let mut cloned_module = MirModule::new("scrubbed-clone".to_owned());
    cloned_module.add_function(scrubbed_function);
    let error = selected_dynamic_aot_metadata_present(&cloned_module).unwrap_err();
    assert!(error.contains("scrubbed"));
}

#[test]
fn selected_dynamic_census_rejects_partial_pair() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "partial/4".to_owned(),
            params: vec![MirType::Unknown; 4],
            return_type: MirType::Integer,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    let mut module = MirModule::new("partial".to_owned());
    module.add_function(function);
    let error = selected_dynamic_aot_metadata_present(&module).unwrap_err();
    assert!(error.contains("partial"));
}

#[test]
fn selected_dynamic_boundary_accepts_only_fixed_route_values() {
    assert!(validate_selected_dynamic_boundary_route_values(None, None, None, None).is_ok());
    assert!(validate_selected_dynamic_boundary_route_values(
        Some("pure-first"),
        Some("none"),
        None,
        Some("0"),
    )
    .is_ok());
}

#[test]
fn selected_dynamic_boundary_rejects_compat_route_inheritance() {
    for (recipe, replay, provider, legacy) in [
        (Some("harness"), None, None, None),
        (None, Some("harness"), None, None),
        (None, None, Some("llvmlite"), None),
        (None, None, None, Some("1")),
    ] {
        assert!(validate_selected_dynamic_boundary_route_values(recipe, replay, provider, legacy)
            .is_err());
    }
}

#[test]
fn appends_non_empty_extra_libs_as_single_arg() {
    let mut cmd = std::process::Command::new("ny-llvmc");
    append_ny_llvmc_extra_libs_arg(&mut cmd, Some("-ldl -lpthread"));
    let args: Vec<_> = cmd
        .get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    assert_eq!(args, vec!["--libs".to_string(), "-ldl -lpthread".to_string()]);
}

#[test]
fn ignores_blank_extra_libs() {
    let mut cmd = std::process::Command::new("ny-llvmc");
    append_ny_llvmc_extra_libs_arg(&mut cmd, Some("   "));
    assert!(cmd.get_args().next().is_none());
}

#[test]
fn retained_mir_path_is_reported_on_emit_failure() {
    let err = with_retained_mir_path(
        "ny-llvmc failed".to_string(),
        std::path::Path::new("tmp/nyash_cli_emit_123.json"),
    );
    assert!(err.contains("ny-llvmc failed"));
    assert!(err.contains("retained_mir=tmp/nyash_cli_emit_123.json"));
}

#[test]
fn selected_receipt_flag_is_forwarded_to_boundary_command() {
    let cmd = build_ny_llvmc_emit_exe_command(
        std::path::Path::new("ny-llvmc"),
        std::path::Path::new("candidate.json"),
        "candidate.exe",
        Some("target/release"),
        None,
        Some(std::path::Path::new("receipt.json")),
        None,
    )
    .expect("command");
    let args: Vec<_> = cmd
        .get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    assert!(args
        .windows(2)
        .any(|pair| pair == ["--receipt-json", "receipt.json"]));
}
