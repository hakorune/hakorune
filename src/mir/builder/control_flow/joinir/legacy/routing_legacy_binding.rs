//! Legacy LoopFrontendBinding path (Phase 49-3)
//!
//! This module contains the legacy JSON v0 construction logic for specific
//! whitelisted functions (print_tokens, array_filter) that use the old
//! LoopFrontendBinding system.
//!
//! Phase 194+ uses the pattern-based router instead. This legacy path is
//! kept for backward compatibility with existing whitelist entries.

// Phase 132-R0 Task 4: Fixed imports for legacy/ subdirectory
use super::super::trace;
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

impl MirBuilder {
    /// Phase 49-3: Legacy JoinIR Frontend integration via LoopFrontendBinding
    ///
    /// This implements the old JSON v0 construction path for whitelisted functions.
    /// New patterns should use the pattern router instead (route_loop_pattern).
    ///
    /// # Pipeline
    /// 1. Build Loop AST → JSON v0 format (with "defs" array)
    /// 2. AstToJoinIrLowerer::lower_program_json() → JoinModule
    /// 3. bridge_joinir_to_mir_with_meta() → MirModule
    /// 4. Merge MIR blocks into current_function
    pub(in crate::mir::builder) fn cf_loop_joinir_legacy_binding(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        func_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::loop_frontend_binding::LoopFrontendBinding;
        use crate::mir::join_ir::frontend::{AstToJoinIrLowerer, JoinFuncMetaMap};
        use crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir_with_meta;
        use crate::mir::types::ConstValue;
        use crate::mir::MirInstruction;
        use crate::r#macro::ast_json::ast_to_json;

        // Phase 50: Create appropriate binding based on function name
        let binding = match func_name {
            "JsonTokenizer.print_tokens/0" => LoopFrontendBinding::for_print_tokens(),
            "ArrayExtBox.filter/2" => LoopFrontendBinding::for_array_filter(),
            _ => {
                trace::trace().routing(
                    "router",
                    func_name,
                    "No legacy binding defined, falling back",
                );
                return Ok(None);
            }
        };

        trace::trace().debug(
            "router",
            &format!(
                "Using legacy binding for '{}': counter={}, acc={:?}, pattern={:?}",
                func_name, binding.counter_var, binding.accumulator_var, binding.pattern
            ),
        );

        // Step 1: Convert condition and body to JSON
        let condition_json = ast_to_json(condition);
        let mut body_json: Vec<serde_json::Value> = body.iter().map(|s| ast_to_json(s)).collect();

        // Phase 50: Rename variables in body (e.g., "out" → "acc" for filter)
        binding.rename_body_variables(&mut body_json);

        // Phase 50: Generate Local declarations from binding
        let (i_local, acc_local, n_local) = binding.generate_local_declarations();

        // Phase 52/56: Build params from external_refs
        let mut params: Vec<serde_json::Value> = Vec::new();

        // Phase 52: Add 'me' for instance methods
        if binding.needs_me_receiver() {
            trace::trace().debug("router", "Adding 'me' to params (instance method)");
            params.push(serde_json::json!("me"));
        }

        // Phase 56: Add external_refs as parameters (arr, pred for filter)
        for ext_ref in &binding.external_refs {
            if ext_ref == "me" || ext_ref.starts_with("me.") {
                continue;
            }
            trace::trace().debug(
                "router",
                &format!("Adding '{}' to params (external_ref)", ext_ref),
            );
            params.push(serde_json::json!(ext_ref));
        }

        // Step 2: Construct JSON v0 format with "defs" array
        let program_json = serde_json::json!({
            "defs": [
                {
                    "name": "simple",
                    "params": params,
                    "body": {
                        "type": "Block",
                        "body": [
                            // Phase 50: Inject i/acc/n Local declarations
                            i_local,
                            acc_local,
                            n_local,
                            {
                                "type": "Loop",
                                "cond": condition_json,
                                "body": body_json
                            },
                            // Return the accumulator
                            {
                                "type": "Return",
                                "value": { "kind": "Variable", "name": "acc" }
                            }
                        ]
                    }
                }
            ]
        });

        trace::trace().debug(
            "router",
            &format!(
                "Generated JSON v0 for {}: {}",
                func_name,
                serde_json::to_string_pretty(&program_json).unwrap_or_default()
            ),
        );

        // Step 3: Lower to JoinIR (Fail-Fast: no silent error swallowing)
        // Phase 132-Post: Removed catch_unwind - panics should be fixed at the source
        let join_module = {
            let json_clone = program_json.clone();
            let mut lowerer = AstToJoinIrLowerer::new();

            // If lower_program_json panics, the panic will propagate up
            // This is intentional - we want to catch and fix panics at their source
            // rather than silently swallowing them with Ok(None)
            lowerer.lower_program_json(&json_clone)
        };

        let join_meta = JoinFuncMetaMap::new();

        trace::trace().joinir_stats(
            "router",
            join_module.functions.len(),
            join_module.functions.values().map(|f| f.body.len()).sum(),
        );

        // Step 4: Convert JoinModule to MIR
        // Phase 256 P1.5: Pass None for boundary (legacy path doesn't use boundary)
        let mir_module = bridge_joinir_to_mir_with_meta(&join_module, &join_meta, None)
            .map_err(|e| format!("JoinIR→MIR conversion failed: {}", e.message))?;

        // Debug MIR module if trace enabled
        if trace::trace().is_joinir_enabled() {
            trace::trace().debug(
                "router",
                &format!("MirModule has {} functions", mir_module.functions.len()),
            );
            for (name, func) in &mir_module.functions {
                trace::trace().debug(
                    "router",
                    &format!(
                        "  - {}: {} blocks, entry={:?}",
                        name,
                        func.blocks.len(),
                        func.entry_block
                    ),
                );
                for (block_id, block) in &func.blocks {
                    trace::trace().blocks(
                        "router",
                        &format!(
                            "Block {:?}: {} instructions",
                            block_id,
                            block.instructions.len()
                        ),
                    );
                    for (i, inst) in block.instructions.iter().enumerate() {
                        trace::trace().instructions("router", &format!("[{}] {:?}", i, inst));
                    }
                    if let Some(ref term) = block.terminator {
                        trace::trace().instructions("router", &format!("terminator: {:?}", term));
                    }
                }
            }
        }

        // Step 5: Merge MIR blocks into current_function
        // Phase 132 P1: Always pass boundary with continuation contract (no by-name guessing in merge)
        use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
        let boundary = JoinInlineBoundary::new_inputs_only(vec![], vec![]);
        let _ = self.merge_joinir_mir_blocks(&mir_module, Some(&boundary), debug)?;

        // Return void
        let void_val = self.next_value_id();
        self.emit_instruction(MirInstruction::Const {
            dst: void_val,
            value: ConstValue::Void,
        })?;

        Ok(Some(void_val))
    }
}
