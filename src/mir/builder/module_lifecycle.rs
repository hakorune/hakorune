//! Module Lifecycle Orchestrator - MIR module construction pipeline
//!
//! Phase 29bq+ cleanliness: lifecycle.rs modularization (623 → ~200 lines)
//!
//! # Purpose
//!
//! Owns the shared setup and finalization kernels used by typed module owners:
//! 1. prepare_module() - Module setup and entry point creation
//! 2. finalize_module() - Type propagation, PHI inference, module sealing
//!
//! # Architecture
//!
//! This orchestrator delegates to specialized modules:
//!
//! - **type_hint_providers** - Annotates Call/BoxCall/Await result types
//! - **phi_type_inference** - Multi-phase PHI return type resolution
//!
//! # Execution Flow
//!
//! ```text
//! prepare_module()
//!   ↓ typed Program or responsibility-local lowering owner
//! finalize_module()
//!   ├→ TypePropagationPipeline::run()              (Copy → BinOp → PHI)
//!   ├→ type_hint_providers::annotate_*()           (Call result types)
//!   ├→ phi_type_inference::infer_return_type()     (P3-A/B/C/D/P4)
//!   └→ Module sealing (metadata, birth verification)
//! ```
//!
//! # Critical Constraints
//!
//! 1. **Execution order固定**: typed owner enforces prepare → lower → finalize
//! 2. **Type propagation BEFORE PHI inference**: TypePropagationPipeline runs first
//! 3. **Type hints BEFORE PHI inference**: Ensures value_types populated
//! 4. **PHI resolver order固定**: A → B → P3-D → P4 → P3-C
//!
use super::recursive_child_lowering::{
    RawAstChildLoweringPortV1, RawBoxMethodChildPortV1, RawInvocationChildPortV1,
    RawLegacyChildLoweringPortV1,
};
use super::{
    BasicBlockId, CanonicalSameModuleCallableKeyV1, EffectMask, FunctionSignature, MirInstruction,
    MirModule, MirType, ValueId,
};
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::config;

// Phase 29bq+: PHI type inference extracted to dedicated module
use super::phi_type_inference;
// Phase 29bq+: Type hint provision extracted to dedicated module
use super::type_hint_providers;

/// Root-only extension of the existing raw child-lowering capability.
///
/// Static/free callables, constructors, and root-body descent already have
/// exact owners on the raw port. Only source-order instance methods need this
/// extra seam because the parked Stage-B adapter observes their canonical key.
pub(in crate::mir::builder) trait RootCallableCapturePortV1:
    RawBoxMethodChildPortV1 + RawAstChildLoweringPortV1
{
    #[allow(clippy::too_many_arguments)]
    fn lower_root_instance_method(
        &mut self,
        builder: &mut super::MirBuilder,
        canonical_key: CanonicalSameModuleCallableKeyV1,
        owner: String,
        method: String,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        let _ = (canonical_key, method);
        self.lower_instance_box_method(
            builder,
            function_name,
            owner,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }
}

impl RootCallableCapturePortV1 for RawInvocationChildPortV1<'_, '_> {}
impl RootCallableCapturePortV1 for RawLegacyChildLoweringPortV1 {}

impl super::MirBuilder {
    pub(super) fn prepare_module(&mut self) -> Result<(), String> {
        self.comp_ctx.clear_callable_declaration_catalog();
        self.comp_ctx.callable_main_compatibility_policy =
            super::module_compat_policy::CallableMainCompatibilityPolicyV1::snapshot_from_legacy_ingress();
        // A new module is a new legacy compatibility snapshot. Clearing the
        // candidate cache also resets its freshness witness so same-size module
        // replacement cannot reuse the previous module's tail candidates.
        self.comp_ctx.clear_method_tail_index();

        let mut module = MirModule::new("main".to_string());
        module.metadata.source_file = self.current_source_file();
        let main_signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };

        let entry_block = self.next_block_id();
        let mut main_function = self.new_function_with_metadata(main_signature, entry_block);
        main_function.metadata.is_entry_point = true;

        self.current_module = Some(module);
        // Phase 136 Step 3/7: Use scope_ctx as SSOT
        self.function_state.current_function = Some(main_function);
        self.function_state.current_block = Some(entry_block);

        // Phase 29bq+: reset sealing session for new function
        self.function_state.frag_emit_session.reset();

        // 関数スコープの SlotRegistry を初期化するよ（観測専用）。
        // main 関数用のスロット登録箱として使う想定だよ。
        self.comp_ctx.current_slot_registry =
            Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());

        // Region 観測レイヤ: main 関数の FunctionRegion を 1 つ作っておくよ。
        crate::mir::region::observer::observe_function_region(self);

        // Hint: scope enter at function entry (id=0 for main)
        self.hint_scope_enter(0);

        if crate::config::env::builder_safepoint_entry() {
            self.emit_instruction(MirInstruction::Safepoint)?;
        }

        Ok(())
    }

    /// Finalize MIR module after a typed or responsibility-local lowering owner.
    ///
    /// Execution flow:
    /// 1. Type propagation (TypePropagationPipeline)
    /// 2. Type hint provision (delegation to type_hint_providers)
    /// 3. PHI type inference (delegation to phi_type_inference)
    /// 4. Module sealing (metadata, birth verification)
    pub(super) fn finalize_module(&mut self, result_value: ValueId) -> Result<MirModule, String> {
        // Hint: scope leave at function end (id=0 for main)
        self.hint_scope_leave(0);
        if let Some(block_id) = self.function_state.current_block {
            if let Some(ref mut function) = self.function_state.current_function {
                if let Some(block) = function.get_block_mut(block_id) {
                    if !block.is_terminated() {
                        block.add_instruction(MirInstruction::Return {
                            value: Some(result_value),
                        });
                    }
                    if let Some(mt) = self
                        .function_state
                        .type_ctx
                        .value_types
                        .get(&result_value)
                        .cloned()
                    {
                        function.signature.return_type = mt;
                    }
                }
            }
        }

        let mut module = self.current_module.take().unwrap();
        // Phase 136 Step 3/7: Take from scope_ctx (SSOT)
        crate::mir::builder::emission::value_lifecycle::verify_typed_values_are_defined(
            self,
            "finalize_module",
        )?;
        let mut function = self.function_state.current_function.take().unwrap();

        // ===== Step 1: Type Propagation (TypePropagationPipeline SSOT) =====
        // Phase 279 P0: SSOT type propagation pipeline
        //
        // 全ての型伝播処理を1つの入口（SSOT）に統一。
        // 順序固定: Copy → BinOp → Copy → PHI
        // lifecycle.rs と joinir_function_converter.rs の両方がこのパイプラインを呼ぶ。
        use crate::mir::type_propagation::TypePropagationPipeline;
        TypePropagationPipeline::run(&mut function, &mut self.function_state.type_ctx.value_types)?;

        // ===== Step 2: Type Hint Provision (delegation to type_hint_providers) =====
        // Phase 84-5 guard hardening: ensure call/await results are registered in `value_types`
        // before return type inference. This avoids "impossible" debug panics when the builder
        // emitted a value-producing instruction without annotating its dst type.
        type_hint_providers::annotate_missing_result_types_from_calls_and_await(
            &mut self.function_state.type_ctx,
            &function,
            &module,
        );

        // Phase 131-9: Update function metadata with corrected types
        // MUST happen after PHI type correction above AND BinOp re-propagation
        function.metadata.value_types = self.function_state.type_ctx.value_types.clone();
        let mut origin_callers = function.metadata.value_origin_callers.clone();
        for (k, v) in self.value_origin_caller_rows().iter() {
            origin_callers.insert(*k, v.clone());
        }
        function.metadata.value_origin_callers = origin_callers;

        // ===== Step 3: PHI Type Inference (delegation to phi_type_inference) =====
        // Phase 29bq+: PHI type inference delegated to phi_type_inference module
        // Multi-phase resolver chain (P3-A/B/C/D/P4) for return type resolution
        if let Some(inferred_type) =
            phi_type_inference::infer_return_type_from_phi(self, &mut function)
        {
            function.signature.return_type = inferred_type;
        }
        // Final builder seal: PHI inputs are edge values, so every incoming value
        // must be valid in the predecessor block recorded on that edge.
        crate::mir::builder::ssa::phi_input_materializer::materialize_all_phi_inputs(
            &mut function,
            "finalize_module",
        )?;
        // ===== Step 4: Module Sealing (metadata, birth verification) =====
        // Dev-only verify: NewBox → birth() invariant (warn if missing)
        //
        // Policy:
        // - Keep stderr clean by default (gates compare output).
        // - Enable emission only when explicitly requested (CLI verbose).
        if crate::config::env::using_is_dev()
            && config::env::stageb_dev_verify_enabled()
            && crate::config::env::cli_verbose_enabled()
        {
            let mut warn_count = 0usize;
            for (_bid, bb) in function.blocks.iter() {
                let insns = &bb.instructions;
                let mut idx = 0usize;
                while idx < insns.len() {
                    if let MirInstruction::NewBox {
                        dst,
                        box_type,
                        args,
                    } = &insns[idx]
                    {
                        // Phase 71-SSA 71-11.2: StageBDriverBox is a static box → skip birth warning unconditionally
                        // Static boxes don't follow NewBox→birth pattern by design
                        if box_type == "StageBDriverBox" {
                            idx += 1;
                            continue;
                        }
                        // Skip StringBox (literal optimization path)
                        if box_type != "StringBox" {
                            let expect_tail = format!("{}.birth/{}", box_type, args.len());
                            // Look ahead up to 3 instructions for:
                            // - Call(Method birth) on dst (canonical),
                            // - or Global(expect_tail) compatibility path.
                            let mut ok = false;
                            let mut j = idx + 1;
                            let mut last_const_name: Option<String> = None;
                            while j < insns.len() && j <= idx + 3 {
                                match &insns[j] {
                                    MirInstruction::Call {
                                        callee:
                                            Some(
                                                crate::mir::definitions::call_unified::Callee::Method {
                                                    method,
                                                    receiver: Some(recv),
                                                    ..
                                                },
                                            ),
                                        ..
                                    } => {
                                        if method == "birth" && recv == dst {
                                            ok = true;
                                            break;
                                        }
                                    }
                                    MirInstruction::Const { value, .. } => {
                                        if let super::ConstValue::String(s) = value {
                                            last_const_name = Some(s.clone());
                                        }
                                    }
                                    MirInstruction::Call { func: _, .. } => {
                                        // If immediately preceded by matching Const String, accept
                                        if let Some(prev) = last_const_name.as_ref() {
                                            if prev == &expect_tail {
                                                ok = true;
                                                break;
                                            }
                                        }
                                        // Heuristic: in some forms, builder may reuse a shared const; best-effort only
                                    }
                                    _ => {}
                                }
                                j += 1;
                            }
                            if !ok {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.warn(&format!("[warn] dev verify: NewBox {} at v{} not followed by birth() call (expect {})", box_type, dst, expect_tail));
                                warn_count += 1;
                            }
                        }
                    }
                    idx += 1;
                }
            }
            if warn_count > 0 {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.warn(&format!(
                    "[warn] dev verify: NewBox→birth invariant warnings: {}",
                    warn_count
                ));
            }
        }

        module.add_function(function);

        // Dev stub: provide condition_fn when missing to satisfy predicate calls in JSON lexers
        // Returns integer 1 (truthy) and accepts one argument (unused).
        //
        // NOTE:
        // - MirFunction::new() はシグネチャの params に応じて
        //   [ValueId(0)..ValueId(param_count-1)] を事前に予約する。
        // - ここでは追加の next_value_id()/params.push() は行わず、
        //   予約済みのパラメータ集合をそのまま使う。
        if module.functions.get("condition_fn").is_none() {
            let sig = FunctionSignature {
                name: "condition_fn".to_string(),
                params: vec![MirType::Integer], // accept one i64-like arg
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            };
            let entry = BasicBlockId::new(0);
            let mut f = self.new_function_with_metadata(sig, entry);
            // body: const 1; return it（FunctionEmissionBox を使用）
            let one = crate::mir::function_emission::emit_const_integer(&mut f, entry, 1);
            crate::mir::function_emission::emit_return_value(&mut f, entry, one);
            module.add_function(f);
        }

        // main 関数スコープの Region スタックをポップするよ。
        crate::mir::region::observer::pop_function_region(self);

        // main 関数スコープの SlotRegistry を解放するよ。
        self.comp_ctx.current_slot_registry = None;

        // Phase 285LLVM-1.1: Copy user box declarations to module metadata for LLVM harness
        module.metadata.user_box_decls = self.comp_ctx.user_defined_boxes.clone();
        module.metadata.user_box_field_decls = self
            .comp_ctx
            .user_box_field_decls
            .clone()
            .into_iter()
            .map(|(name, decls)| {
                (
                    name,
                    decls
                        .into_iter()
                        .map(|decl| crate::mir::UserBoxFieldDecl {
                            name: decl.name,
                            declared_type_name: decl.declared_type_name,
                            is_weak: decl.is_weak,
                        })
                        .collect(),
                )
            })
            .collect();
        module.metadata.record_decls = self.comp_ctx.record_decls.clone().into_iter().collect();
        module.metadata.enum_decls = self.comp_ctx.enum_decls_for_module_metadata();
        crate::mir::semantic_refresh::refresh_module_record_and_packed_layout_plans(&mut module);
        crate::mir::typed_object_plan::refresh_module_typed_object_plans(&mut module);
        crate::mir::direct_state_plan::refresh_module_direct_state_plans(&mut module);
        for function in module.functions.values_mut() {
            crate::mir::builder::ssa::phi_input_materializer::materialize_all_phi_inputs(
                function,
                "finalize_module_all_functions",
            )?;
        }

        self.function_state = Default::default();
        Ok(module)
    }

    // Phase 131-11-E: Re-propagate BinOp result types after PHI resolution
    // This fixes cases where BinOp instructions were created before PHI types were known
    // Phase 279 P0: repropagate_binop_types() method removed
    // Moved to TypePropagationPipeline (SSOT)
}

// Phase 279 P0: OperandTypeClass enum removed
// Moved to TypePropagationPipeline (SSOT)

#[cfg(test)]
#[path = "module_lifecycle_capture_tests.rs"]
mod capture_tests;
