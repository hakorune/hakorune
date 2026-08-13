//! Selected Dynamic launch/helper identity fence.
//!
//! The zero-argument runtime launch function and the metadata-bearing selected
//! callable are distinct physical functions. This module only validates the
//! already-sealed MIR shape before a Boundary process is spawned; it does not
//! choose a provider, copy metadata, or invent a launch-to-helper call.

use crate::mir::{function::DynamicV2MetadataPairObservation, MirModule};

const LAUNCH_NAMES: [&str; 2] = ["main", "ny_main"];
const SELECTED_HELPER_ARITY: usize = 4;

pub(crate) fn validate_selected_dynamic_launch_helper_identity(
    module: &MirModule,
) -> Result<(), String> {
    let mut launch_name = None;
    let mut selected_helper_name = None;

    for (name, function) in &module.functions {
        if LAUNCH_NAMES.contains(&name.as_str()) {
            if !function.signature.params.is_empty() {
                return Err(format!(
                    "selected Dynamic launch function {name} must have zero parameters"
                ));
            }
            if launch_name.replace(name.as_str()).is_some() {
                return Err("selected Dynamic launch entry must be unique".to_owned());
            }
        }

        match function.metadata.selected_dynamic_metadata_observation() {
            DynamicV2MetadataPairObservation::Ordinary => {}
            DynamicV2MetadataPairObservation::Selected { .. } => {
                if selected_helper_name.replace(name.as_str()).is_some() {
                    return Err("selected Dynamic helper metadata pair must be unique".to_owned());
                }
                if function.signature.params.len() != SELECTED_HELPER_ARITY {
                    return Err(format!(
                        "selected Dynamic helper {name} must have exactly {SELECTED_HELPER_ARITY} parameters"
                    ));
                }
            }
            DynamicV2MetadataPairObservation::Scrubbed => {
                return Err(format!(
                    "selected Dynamic helper metadata pair is scrubbed for function {name}"
                ));
            }
            DynamicV2MetadataPairObservation::Partial => {
                return Err(format!(
                    "selected Dynamic helper metadata pair is partial for function {name}"
                ));
            }
        }
    }

    let launch_name = launch_name.ok_or_else(|| {
        "selected Dynamic module must contain one zero-argument main or ny_main launch".to_owned()
    })?;
    let selected_helper_name = selected_helper_name
        .ok_or_else(|| "selected Dynamic helper metadata pair is missing".to_owned())?;
    if launch_name == selected_helper_name {
        return Err("selected Dynamic launch and helper identities must be distinct".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_selected_dynamic_launch_helper_identity;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

    fn function(name: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_owned(),
                params: vec![MirType::Unknown; arity],
                return_type: MirType::Integer,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        )
    }

    fn selected_helper(arity: usize) -> MirFunction {
        let mut helper = function("ParserScanLoopBox.skip_while/4", arity);
        helper
            .metadata
            .install_a_prime_i64_physical_receipt_for_test(
                crate::mir::test_support::a_prime_receipt(),
            )
            .expect("receipt install");
        helper
            .metadata
            .install_dynamic_v2_aot_metadata_for_test(
                crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(),
            )
            .expect("admission install");
        helper
    }

    fn valid_module() -> MirModule {
        let mut module = MirModule::new("selected".to_owned());
        module.add_function(function("main", 0));
        module.add_function(selected_helper(4));
        module
    }

    #[test]
    fn accepts_distinct_zero_arg_launch_and_four_arg_helper() {
        assert!(validate_selected_dynamic_launch_helper_identity(&valid_module()).is_ok());
    }

    #[test]
    fn rejects_missing_launch() {
        let mut module = MirModule::new("missing-launch".to_owned());
        module.add_function(selected_helper(4));
        let error = validate_selected_dynamic_launch_helper_identity(&module).unwrap_err();
        assert!(error.contains("launch"));
    }

    #[test]
    fn rejects_nonzero_launch() {
        let mut module = MirModule::new("nonzero-launch".to_owned());
        module.add_function(function("main", 1));
        module.add_function(selected_helper(4));
        let error = validate_selected_dynamic_launch_helper_identity(&module).unwrap_err();
        assert!(error.contains("zero parameters"));
    }

    #[test]
    fn rejects_duplicate_launch_names() {
        let mut module = valid_module();
        module.add_function(function("ny_main", 0));
        let error = validate_selected_dynamic_launch_helper_identity(&module).unwrap_err();
        assert!(error.contains("unique"));
    }

    #[test]
    fn rejects_helper_arity_drift() {
        let mut module = MirModule::new("helper-arity".to_owned());
        module.add_function(function("main", 0));
        module.add_function(selected_helper(3));
        let error = validate_selected_dynamic_launch_helper_identity(&module).unwrap_err();
        assert!(error.contains("exactly 4"));
    }
}
