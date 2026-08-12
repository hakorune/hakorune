use super::vars;
use super::{MirBuilder, ValueId};
use crate::ast::LiteralValue;
use crate::mir::exact_numeric_value_facts::ExactNumericConstFact;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    ExactNumericConversionError, NumericTarget,
};
use crate::mir::MirInstruction;

pub(in crate::mir::builder) use super::new_expression::PreparedRawNewExpressionV1;

impl MirBuilder {
    /// Build a literal value
    pub(in crate::mir::builder) fn build_literal(
        &mut self,
        literal: LiteralValue,
    ) -> Result<ValueId, String> {
        // Canonical Const emission publishes the transient type only after the
        // instruction succeeds. Literal dispatch must not duplicate that fact.
        Ok(match literal {
            LiteralValue::Integer(n) => {
                crate::mir::builder::emission::constant::emit_integer(self, n)?
            }
            LiteralValue::TypedInteger {
                value,
                declared_type_name,
            } => self.emit_typed_integer_literal(value, declared_type_name)?,
            LiteralValue::Float(f) => crate::mir::builder::emission::constant::emit_float(self, f)?,
            LiteralValue::String(s) => {
                crate::mir::builder::emission::constant::emit_string(self, s)?
            }
            LiteralValue::Bool(b) => crate::mir::builder::emission::constant::emit_bool(self, b)?,
            LiteralValue::Null => crate::mir::builder::emission::constant::emit_null(self)?,
            LiteralValue::Void => crate::mir::builder::emission::constant::emit_void(self)?,
        })
    }

    pub(in crate::mir::builder) fn emit_typed_integer_literal(
        &mut self,
        value: i64,
        declared_type_name: String,
    ) -> Result<ValueId, String> {
        let Some(ty) = exact_numeric_mir_type_from_declared_name(
            Some(declared_type_name.as_str()),
            NumericTarget::host(),
        ) else {
            return Err(format!(
                "[exact-numeric-literal/unknown-type] declared_type={}",
                declared_type_name
            ));
        };
        let checked = exact_numeric_const_from_i128(i128::from(value), &ty)
            .map_err(exact_numeric_literal_error)?;
        let dst = crate::mir::builder::emission::constant::emit_integer(self, value)?;
        if let Some(function) = self.function_state.current_function.as_mut() {
            function.metadata.exact_numeric_const_facts.insert(
                dst,
                ExactNumericConstFact {
                    declared_type_name: checked.ty.source_name,
                    value: checked.value,
                },
            );
        }
        Ok(dst)
    }

    /// Build variable access
    pub(in crate::mir::builder) fn build_variable_access(
        &mut self,
        name: String,
    ) -> Result<ValueId, String> {
        // Step 5-5-G: __pin$ variables should NEVER be accessed from variable_map
        // They are transient temporaries created during expression building and
        // should not persist across blocks. If we see one here, it's a compiler bug.
        if name.starts_with("__pin$") {
            return Err(format!(
                "COMPILER BUG: Attempt to access __pin$ temporary '{}' from variable_map. \
                 __pin$ variables should only exist as direct SSA values, not as named variables.",
                name
            ));
        }

        if let Some(&value_id) = self.function_state.variable_ctx.variable_map.get(&name) {
            self.fail_if_record_value_escape_by_name(&name, value_id)?;
            // Removed: [build_variable_access:GHOST_v36] observation (PHI issue resolved)
            // Removed: [build_variable_access:index_of_trace] observation (PHI issue resolved)
            // Removed: [build_variable_access:VAR_j] observation (PHI issue resolved)
            // Debug-only observation: check if variable_map value is defined
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(func) = self.function_state.current_function.as_ref() {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                    if !def_blocks.contains_key(&value_id) {
                        // Found undefined ValueId returned from variable_map
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[call/arg_build:undefined_value] fn={} bb={:?} var_name={} v=%{} ast=Variable span=n/a next={}",
                            func.signature.name,
                            self.function_state.current_block,
                            name,
                            value_id.0,
                            func.next_value_id
                        ));
                    }
                }
            }
            Ok(value_id)
        } else {
            Err(self.undefined_variable_message(&name))
        }
    }

    pub(in crate::mir::builder) fn undefined_variable_message(&self, name: &str) -> String {
        // Enhance diagnostics using Using simple registry (Phase 1)
        let mut msg = format!("Undefined variable: {}", name);

        // syntax-3 keyword diagnostic (local/flow/try/catch/throw)
        if name == "local" && !crate::config::env::parser_stage3_enabled() {
            msg.push_str("\nHint: 'local' is a syntax-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for mode-B compatibility routes).");
            msg.push_str("\nFor AotPrep verification, use tools/hakorune_emit_mir.sh which sets these automatically.");
        } else if (name == "flow" || name == "try" || name == "catch" || name == "throw")
            && !crate::config::env::parser_stage3_enabled()
        {
            msg.push_str(&format!("\nHint: '{}' is a syntax-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for mode-B compatibility routes).", name));
        }

        let suggest = crate::using::simple_registry::suggest_using_for_symbol(name);
        if !suggest.is_empty() {
            msg.push_str("\nHint: symbol appears in using module(s): ");
            msg.push_str(&suggest.join(", "));
            msg.push_str(
                "\nConsider adding 'using <module> [as Alias]' or check nyash.toml [using].",
            );
        }

        msg
    }

    /// Build assignment from an already-evaluated value.
    ///
    /// This is the shared shell used by ordinary lowering and fastmem lowering.
    pub(in crate::mir::builder) fn build_assignment_from_value(
        &mut self,
        var_name: String,
        value_id: ValueId,
    ) -> Result<ValueId, String> {
        vars::assignment_resolver::AssignmentResolverBox::ensure_declared(self, &var_name)?;
        // Removed: [build_expression:GHOST_v36_result] observation (PHI issue resolved)

        // Step 5-5-E: FIX variable map corruption bug
        // REMOVED pin_to_slot() call - it was causing __pin$ temporaries to overwrite
        // real variable names in the variable map.
        //
        // Root cause: pin_to_slot(raw_value_id, "@assign") would sometimes return
        // a ValueId from a previous __pin$ temporary (e.g., __pin$767$@binop_lhs),
        // causing variable_map["m"] to point to the wrong ValueId.
        //
        // SSA + PHI merges work correctly without explicit pinning here.
        // The expression building already creates necessary temporaries.

        // Step 5-5-F: NEVER insert __pin$ temporaries into variable_map
        // __pin$ variables are transient compiler-generated temporaries that should
        // never be tracked as real variables. They are used only within expression
        // building and should not persist across blocks or loops.
        //
        // BUG FIX: Previously, __pin$ variables would be inserted into variable_map,
        // causing stale references after LoopForm transformation renumbers blocks.
        // Result: VM would try to read undefined ValueIds (e.g., ValueId(270) at bb303).
        if !var_name.starts_with("__pin$") {
            let local_slot_id = self
                .function_state
                .binding_ctx
                .lookup(&var_name)
                .map(crate::mir::LocalSlotId::from);
            let local_contract = local_slot_id.and_then(|slot| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        crate::mir::type_contracts::local_slot::local_slot_contract(function, slot)
                            .cloned()
                    })
            });
            let typed_array_spec = local_slot_id.and_then(|slot| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        crate::mir::type_contracts::typed_array::local_slot_spec(function, slot)
                    })
            });
            if let (Some(local_slot_id), Some(spec)) = (local_slot_id, typed_array_spec) {
                let contract_id = format!(
                    "typed-array:local:{}:reassign:{}",
                    local_slot_id.binding_id().raw(),
                    value_id.as_u32()
                );
                let function = self
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| {
                        "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                    })?;
                function.metadata.typed_array_contract_sources.push(
                    crate::mir::function::TypedArrayContractSource {
                        contract_id: contract_id.clone(),
                        boundary: crate::mir::function::TypedArrayContractBoundary::LocalReassign,
                        source_identity:
                            crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(
                                local_slot_id,
                            ),
                        boundary_value: crate::mir::function::TypedArrayBoundaryValue::Value(
                            value_id,
                        ),
                        element_spec: spec,
                    },
                );
                self.emit_instruction(MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: value_id,
                })?;
            }
            let published_value =
                if let (Some(local_slot_id), Some(_contract)) = (local_slot_id, local_contract) {
                    let dst = self.next_value_id();
                    self.emit_instruction(MirInstruction::LocalContractWrite {
                        dst,
                        src: value_id,
                        local_slot_id,
                        write_kind: crate::mir::function::LocalContractWriteKind::Reassign,
                    })?;
                    crate::mir::builder::metadata::propagate::propagate(self, value_id, dst);
                    dst
                } else {
                    value_id
                };

            // Phase 287: Release strong references for previous value BEFORE updating variable_map
            // This ensures "alive until overwrite, then dropped" semantics
            // ⚠️ Termination guard: don't emit after return/throw
            if !self.is_current_block_terminated() {
                if let Some(prev) = self
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(&var_name)
                    .copied()
                {
                    let _ =
                        self.emit_instruction(MirInstruction::ReleaseStrong { values: vec![prev] });
                }
            }

            // In SSA form, each assignment creates a new value
            self.function_state
                .variable_ctx
                .variable_map
                .insert(var_name.clone(), published_value);

            // Removed: [build_assignment:GHOST_v36_assigned] observation (PHI issue resolved)
            // Removed: [build_assignment:index_of_trace] observation (PHI issue resolved)
            return Ok(published_value);
        }

        Ok(value_id)
    }

    /// Check if the current basic block is terminated
    pub(in crate::mir::builder) fn is_current_block_terminated(&self) -> bool {
        if let (Some(block_id), Some(ref function)) = (
            self.function_state.current_block,
            &self.function_state.current_function,
        ) {
            if let Some(block) = function.get_block(block_id) {
                return block.is_terminated();
            }
        }
        false
    }
}
fn exact_numeric_literal_error(error: ExactNumericConversionError) -> String {
    match error {
        ExactNumericConversionError::NegativeToUnsigned { source_name, value } => format!(
            "[exact-numeric-literal/negative-unsigned] declared_type={} value={}",
            source_name, value
        ),
        ExactNumericConversionError::OutOfRange {
            source_name,
            value,
            min,
            max,
        } => format!(
            "[exact-numeric-literal/out-of-range] declared_type={} value={} range={}..{}",
            source_name, value, min, max
        ),
    }
}
