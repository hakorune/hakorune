//! Dev/strict StepTree pipeline (capability guard + shadow lowering)

use crate::ast::ASTNode;
use crate::mir::control_tree::normalized_shadow::available_inputs_collector::AvailableInputsCollectorBox;
use crate::mir::control_tree::normalized_shadow::normalized_verifier;
use crate::mir::control_tree::normalized_shadow::StepTreeNormalizedShadowLowererBox;
use crate::mir::control_tree::StepTreeBuilderBox;
use crate::runtime::get_global_ring0;

pub trait DevTrace {
    fn dev(&self, tag: &str, msg: &str);
}

pub struct StepTreeDevPipelineBox;

impl StepTreeDevPipelineBox {
    pub fn run(
        builder: &mut crate::mir::builder::MirBuilder,
        body: &[ASTNode],
        func_name: &str,
        strict: bool,
        dev: bool,
        trace: &dyn DevTrace,
    ) -> Result<(), String> {
        if !strict && !dev {
            return Ok(());
        }

        let tree = StepTreeBuilderBox::build_from_block(body);

        crate::mir::control_tree::normalized_shadow::log_step_tree_gate_root(func_name);
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "{} StepTree root for '{}': {:?}",
                crate::mir::control_tree::normalized_shadow::STEP_TREE_DEBUG_TAG,
                func_name,
                tree.root
            ));
        }

        if dev {
            trace.dev("control_tree/step_tree", &tree.to_compact_string());
        }

        crate::mir::builder::check_step_tree_capabilities(&tree, func_name, strict, dev)?;

        if !dev {
            return Ok(());
        }

        // Phase 126: Collect available_inputs from SSOT sources
        // Note: CapturedEnv is None for now (if-only patterns don't use CapturedEnv yet)
        // Phase 141 P1.5: No prefix_variables in dev_pipeline context (function-level only)
        let available_inputs = AvailableInputsCollectorBox::collect(builder, None, None);

        // Try shadow lowering (if-only scope)
        let shadow_result =
            StepTreeNormalizedShadowLowererBox::try_lower_if_only(&tree, &available_inputs);

        match shadow_result {
            Ok(Some((module, _meta))) => {
                // Phase 122: Verify Normalized JoinModule structure
                let expected_env_fields =
                    StepTreeNormalizedShadowLowererBox::expected_env_field_count(
                        &tree,
                        &available_inputs,
                    );

                if let Err(err) =
                    normalized_verifier::verify_normalized_structure(&module, expected_env_fields)
                {
                    if strict {
                        return Err(err);
                    }
                    trace.dev("phase122/emit/error", &err);
                } else {
                    // Shadow lowering succeeded + structure verified
                    let status = format!(
                        "module_emitted=true funcs={} env_fields={} step_tree_sig={}",
                        module.functions.len(),
                        expected_env_fields,
                        tree.signature_basis_string()
                    );
                    trace.dev("phase122/emit", &status);
                }
            }
            Ok(None) => {
                // Out of scope (e.g., contains loops)
                let status = StepTreeNormalizedShadowLowererBox::get_status_string(&tree);
                trace.dev("phase121/shadow", &status);
            }
            Err(err) => {
                // Should be supported but failed (internal error)
                let msg = format!("phase121/shadow: internal error for {}: {}", func_name, err);
                // NOTE: Shadow lowering is a dev-only diagnostic path. It must not block
                // compilation, even under strict, because strict is about fail-fast
                // capability boundaries, not requiring the dev shadow emitter to succeed.
                trace.dev("phase121/shadow/error", &msg);
            }
        }

        Ok(())
    }
}
