use super::super::NyashRunner;
use std::fs;

// Modularized boxes for LLVM mode
mod boundary_executor;
mod compile_options;
mod error;
mod exit_reporter;
mod fallback_executor;
mod harness_executor;
mod method_id_injector;
mod mir_compiler;
mod pipeline_plan;
mod pipeline_report;
mod plugin_init;
mod pyvm_executor;
mod report;

// Re-export error types for convenience
use self::error::LlvmRunError;
use self::pipeline_plan::LlvmPipelinePlan;
use self::pipeline_report::{LlvmPipelineReport, PipelineReportBox};

impl NyashRunner {
    /// Execute LLVM mode (split)
    pub(crate) fn execute_llvm_mode(&self, filename: &str) {
        // Step 1: Plugin initialization
        if let Err(e) = plugin_init::PluginInitBox::init() {
            report::emit_error_and_exit(LlvmRunError::fatal(format!("Plugin init error: {}", e)));
        }

        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!(
                    "Error reading file {}: {}",
                    filename, e
                )));
            }
        };

        // Step 3: use the same source preparation contract as MIR emit mode.
        let prepared =
            match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
                self, filename, &code,
            ) {
                Ok(prepared) => prepared,
                Err(e) => {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
                }
            };

        let materialized = match crate::runner::modes::common_util::normal_callable::
            materialize_normal_callable_program_with_identity_v1(
                &prepared.code,
                self.parser_build_config(),
                filename,
            )
        {
            Ok(materialized) => materialized,
            Err(
                crate::runner::modes::common_util::normal_callable::
                    NormalCallableMaterializationErrorV1::Parse(e),
            ) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename,
                    &prepared.code,
                    &e,
                );
                report::emit_error_and_exit(LlvmRunError::fatal(format!("Parse error: {}", e)));
            }
            Err(
                crate::runner::modes::common_util::normal_callable::
                    NormalCallableMaterializationErrorV1::Transform(rejected),
            ) => report::emit_error_and_exit(LlvmRunError::fatal(format!(
                "Normal callable source transform error: {:?}",
                rejected
            ))),
            Err(
                crate::runner::modes::common_util::normal_callable::
                    NormalCallableMaterializationErrorV1::SourceLineage(rejected),
            ) => report::emit_error_and_exit(LlvmRunError::fatal(format!(
                "Normal callable source lineage error: {:?}",
                rejected
            ))),
        };

        let pipeline_plan = LlvmPipelinePlan::current_default();
        let mut pipeline_report = LlvmPipelineReport::new(&pipeline_plan);

        // Compile to MIR
        let compile_result = match compile_options::CompileOptionsBox::compile_normal_callable(
            materialized,
            Some(filename),
            prepared.imports,
            pipeline_plan.compile_options,
        ) {
            Ok(result) => result,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
            }
        };

        let selected_dynamic =
            match crate::runner::modes::common_util::exec::selected_dynamic_aot_metadata_present(
                &compile_result.module,
            ) {
                Ok(selected) => selected,
                Err(error) => report::emit_error_and_exit(LlvmRunError::fatal(error)),
            };

        if selected_dynamic {
            if let Err(error) = crate::runner::modes::common_util::selected_dynamic_identity::
                validate_selected_dynamic_launch_helper_identity(&compile_result.module)
            {
                report::emit_error_and_exit(LlvmRunError::fatal(error));
            }
        }

        let mut module = if selected_dynamic {
            compile_result
                .into_verified_module()
                .unwrap_or_else(|error| {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!(
                        "selected MIR verification failed: {error}"
                    )))
                })
        } else {
            // Ordinary compatibility retains its historical result handling;
            // the selected lane is the only route that consumes the strict
            // verification fence here.
            compile_result.module
        };

        if selected_dynamic {
            if let Err(error) = reject_selected_dynamic_legacy_callsites(&module) {
                report::emit_error_and_exit(LlvmRunError::fatal(error));
            }
        }

        // Inject method_id for BoxCall where resolvable (by-id path)
        #[allow(unused_mut)]
        let _injected = if !selected_dynamic && pipeline_plan.method_id_injector_enabled {
            method_id_injector::MethodIdInjectorBox::inject(&mut module)
        } else {
            0
        };
        pipeline_report.method_id_injector_mutation_count = _injected;

        // PyVM remains an explicit compatibility helper.  A selected Dynamic
        // module bypasses this stage so the Boundary owner is reachable; an
        // explicit PyVM request is still a typed pre-effect rejection.
        match decide_pyvm_stage(
            selected_dynamic,
            crate::config::env::env_bool("SMOKES_USE_PYVM"),
        ) {
            Err(message) => report::emit_error_and_exit(LlvmRunError::fatal(message)),
            Ok(PyVmStageDecision::SkipSelected) => {}
            Ok(PyVmStageDecision::RunCompatibility) => {
                match pyvm_executor::PyVmExecutorBox::try_execute(&module) {
                    Ok(code) => {
                        pipeline_report.execution_backend = "pyvm";
                        PipelineReportBox::emit_if_requested(&pipeline_report);
                        exit_reporter::ExitReporterBox::emit_and_exit(code);
                    }
                    Err(e) if e.code == 0 && e.msg == "PyVM not requested" => {
                        // Continue to next executor
                    }
                    Err(e) => {
                        pipeline_report.execution_backend = "pyvm_error";
                        PipelineReportBox::emit_if_requested(&pipeline_report);
                        report::emit_error_and_exit(e);
                    }
                }
            }
        }

        if let Some(out_path) = requested_object_output_path() {
            pipeline_report.execution_backend = "obj_out";
            PipelineReportBox::emit_if_requested(&pipeline_report);
            emit_requested_object_or_exit(&module, &out_path, selected_dynamic);
            return;
        }

        match execute_via_harness_or_fallback(&module, selected_dynamic) {
            Ok(outcome) => {
                pipeline_report.execution_backend = outcome.backend;
                pipeline_report.llvm_fallback_used = outcome.fallback_used;
                pipeline_report.llvm_fallback_reason = outcome.fallback_reason;
                pipeline_report.mock_fallback_used = outcome.backend == "mock";
                PipelineReportBox::emit_if_requested(&pipeline_report);
                exit_reporter::ExitReporterBox::emit_and_exit(outcome.code);
            }
            Err(e) => report::emit_error_and_exit(e),
        }

        // Execute via LLVM backend (mock or real)
        #[cfg(feature = "llvm-inkwell-legacy")]
        {
            use nyash_rust::backend::llvm_compile_and_execute;
            let temp_path = "nyash_llvm_temp";
            match llvm_compile_and_execute(&module, temp_path) {
                Ok(result) => {
                    if let Some(int_result) = result.as_any().downcast_ref::<IntegerBox>() {
                        let exit_code = int_result.value;
                        crate::console_println!("✅ LLVM execution completed!");
                        crate::console_println!("📊 Exit code: {}", exit_code);
                        exit_reporter::ExitReporterBox::emit_and_exit(exit_code as i32);
                    } else {
                        crate::console_println!(
                            "✅ LLVM execution completed (non-integer result)!"
                        );
                        crate::console_println!("📊 Result: {}", result.to_string_box().value);
                    }
                }
                Err(e) => {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!(
                        "LLVM execution error: {}",
                        e
                    )));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PyVmStageDecision {
    SkipSelected,
    RunCompatibility,
}

fn decide_pyvm_stage(
    selected_dynamic: bool,
    pyvm_requested: bool,
) -> Result<PyVmStageDecision, &'static str> {
    if selected_dynamic {
        if pyvm_requested {
            return Err("selected Dynamic Boundary route rejects an explicit PyVM request");
        }
        return Ok(PyVmStageDecision::SkipSelected);
    }
    Ok(PyVmStageDecision::RunCompatibility)
}

fn reject_selected_dynamic_legacy_callsites(module: &crate::mir::MirModule) -> Result<(), String> {
    for (function_name, function) in &module.functions {
        for (block_id, block) in &function.blocks {
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if let Some(reason) =
                    crate::mir::contracts::backend_core_ops::legacy_callsite_reject_code(
                        instruction,
                    )
                {
                    return Err(format!(
                        "selected Dynamic legacy callsite rejected: function={function_name} block={block_id:?} instruction={instruction_index} reason={reason}"
                    ));
                }
            }
            if let Some(terminator) = block.terminator.as_ref() {
                if let Some(reason) =
                    crate::mir::contracts::backend_core_ops::legacy_callsite_reject_code(terminator)
                {
                    return Err(format!(
                        "selected Dynamic legacy callsite rejected: function={function_name} block={block_id:?} terminator reason={reason}"
                    ));
                }
            }
        }
    }
    Ok(())
}

struct LlvmExecutionOutcome {
    code: i32,
    backend: &'static str,
    fallback_used: bool,
    fallback_reason: &'static str,
}

fn execute_via_harness_or_fallback(
    module: &nyash_rust::mir::MirModule,
    selected_dynamic: bool,
) -> Result<LlvmExecutionOutcome, LlvmRunError> {
    if selected_dynamic {
        let code = boundary_executor::BoundaryExecutorBox::try_execute_selected_dynamic(module)?;
        return Ok(LlvmExecutionOutcome {
            code,
            backend: "ny_llvmc_selected_dynamic_exe",
            fallback_used: false,
            fallback_reason: "none",
        });
    }
    match harness_executor::HarnessExecutorBox::try_execute(module) {
        Ok(code) => Ok(LlvmExecutionOutcome {
            code,
            backend: "ny_llvmc_exe",
            fallback_used: false,
            fallback_reason: "none",
        }),
        Err(e) if crate::config::env::env_bool("NYASH_LLVM_USE_HARNESS") => Err(e),
        Err(_e) => {
            let code = fallback_executor::FallbackExecutorBox::execute(module)?;
            Ok(LlvmExecutionOutcome {
                code,
                backend: "mock",
                fallback_used: true,
                fallback_reason: "harness_unavailable_or_not_requested",
            })
        }
    }
}

fn requested_object_output_path() -> Option<String> {
    std::env::var("NYASH_LLVM_OBJ_OUT").ok()
}

fn emit_requested_object_or_exit(
    _module: &nyash_rust::mir::MirModule,
    _out_path: &str,
    selected_dynamic: bool,
) {
    if selected_dynamic {
        report::emit_error_and_exit(LlvmRunError::fatal(
            "selected Dynamic object emission is not a live Boundary artifact route; request --emit-exe",
        ));
    }
    #[cfg(feature = "llvm-boundary")]
    {
        if let Err(e) =
            crate::runner::modes::common_util::exec::ny_llvmc_emit_obj_lib(_module, _out_path)
        {
            report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
        }
        return;
    }
    #[cfg(all(not(feature = "llvm-boundary"), feature = "llvm-inkwell-legacy"))]
    {
        emit_requested_legacy_object_or_exit(_module, _out_path);
        return;
    }
    #[cfg(all(not(feature = "llvm-boundary"), not(feature = "llvm-inkwell-legacy")))]
    {
        report::emit_error_and_exit(LlvmRunError::fatal(
            "LLVM backend not available (object emit)",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{decide_pyvm_stage, reject_selected_dynamic_legacy_callsites, PyVmStageDecision};

    #[test]
    fn selected_without_pyvm_reaches_boundary_stage() {
        assert_eq!(
            decide_pyvm_stage(true, false),
            Ok(PyVmStageDecision::SkipSelected)
        );
    }

    #[test]
    fn selected_explicit_pyvm_is_rejected_before_execution() {
        assert!(decide_pyvm_stage(true, true).is_err());
    }

    #[test]
    fn ordinary_route_keeps_compatibility_pyvm_stage() {
        assert_eq!(
            decide_pyvm_stage(false, false),
            Ok(PyVmStageDecision::RunCompatibility)
        );
        assert_eq!(
            decide_pyvm_stage(false, true),
            Ok(PyVmStageDecision::RunCompatibility)
        );
    }

    #[test]
    fn selected_legacy_callsite_scan_rejects_missing_callee() {
        let mut module = crate::mir::MirModule::new("selected".to_owned());
        let entry = crate::mir::BasicBlockId::new(0);
        let mut function = crate::mir::MirFunction::new(
            crate::mir::FunctionSignature {
                name: "selected".to_owned(),
                params: vec![],
                return_type: crate::mir::MirType::Void,
                effects: crate::mir::EffectMask::PURE,
            },
            entry,
        );
        function.blocks.get_mut(&entry).unwrap().instructions.push(
            crate::mir::MirInstruction::Call {
                dst: None,
                func: crate::mir::ValueId::INVALID,
                callee: None,
                args: vec![],
                effects: crate::mir::EffectMask::PURE,
            },
        );
        module.add_function(function);

        let error = reject_selected_dynamic_legacy_callsites(&module).unwrap_err();
        assert!(error.contains("call-missing-callee"));
    }

    #[test]
    fn selected_legacy_callsite_scan_accepts_canonical_method() {
        let module = crate::mir::MirModule::new("selected".to_owned());
        assert!(reject_selected_dynamic_legacy_callsites(&module).is_ok());
    }
}

#[cfg(all(not(feature = "llvm-boundary"), feature = "llvm-inkwell-legacy"))]
fn emit_requested_legacy_object_or_exit(module: &nyash_rust::mir::MirModule, out_path: &str) {
    use nyash_rust::backend::llvm_compile_to_object;
    if let Err(e) =
        crate::mir::backend_capability::enforce_mir_backend_supported(module, "llvm-legacy-obj")
    {
        report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
    }
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    crate::cli_v!(
        "[Runner/LLVM] emitting object to {} (cwd={})",
        out_path,
        std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default()
    );
    if let Err(e) = llvm_compile_to_object(module, out_path) {
        report::emit_error_and_exit(LlvmRunError::fatal(format!(
            "LLVM object emit error: {}",
            e
        )));
    }
    match std::fs::metadata(out_path) {
        Ok(meta) if meta.len() > 0 => {
            crate::cli_v!("[LLVM] object emitted: {} ({} bytes)", out_path, meta.len());
        }
        _ => {
            report::emit_error_and_exit(LlvmRunError::fatal(format!(
                "LLVM object not found or empty: {}",
                out_path
            )));
        }
    }
}

// emit_mir_json_for_harness moved to crate::runner::mir_json_emit
