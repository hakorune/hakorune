//! LLVM runner pipeline plan.
//!
//! The plan names the current runner stages. It does not decide optimization
//! routes or change executor behavior.

use super::compile_options::LlvmCompileOptions;

#[derive(Clone, Copy, Debug)]
pub struct LlvmPipelinePlan {
    pub compile_options: LlvmCompileOptions,
    pub method_id_injector_enabled: bool,
    pub joinir_experiment_hook_enabled: bool,
}

impl LlvmPipelinePlan {
    pub fn current_default() -> Self {
        Self {
            compile_options: LlvmCompileOptions::current_default(),
            method_id_injector_enabled: true,
            joinir_experiment_hook_enabled: true,
        }
    }
}
