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
//! - **return_type_strategy** - Multi-phase return type resolution
//!
//! # Execution Flow
//!
//! ```text
//! prepare_module()
//!   ↓ typed Program or responsibility-local lowering owner
//! finalize_module()
//!   ├→ TypePropagationPipeline::run()              (Copy → BinOp → PHI)
//!   ├→ type_hint_providers::annotate_*()           (Call result types)
//!   ├→ return_type_strategy::infer_return_type()   (Direct/P3-A/B/C/D/P4)
//!   └→ Module sealing (metadata, birth verification)
//! ```
//!
//! # Critical Constraints
//!
//! 1. **Execution order固定**: typed owner enforces prepare → lower → finalize
//! 2. **Type propagation BEFORE PHI inference**: TypePropagationPipeline runs first
//! 3. **Type hints BEFORE PHI inference**: Ensures value_types populated
//! 4. **Return strategy order固定**: Direct → hint → P3-D → P4 → P3-C
//!
use super::main_expansion::VerifiedMainStaticChildV1;
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
use super::recursive_child_lowering::{
    RawAstChildLoweringPortV1, RawBoxMethodChildPortV1, RawInvocationChildPortV1,
    RawLegacyChildLoweringPortV1,
};
use super::{
    CanonicalSameModuleCallableKeyV1, EffectMask, FunctionSignature, MirInstruction, MirModule,
    MirType, ValueId,
};
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::config;

use super::return_type_strategy;
// Phase 29bq+: Type hint provision extracted to dedicated module
use super::type_hint_providers;

/// Root-only extension of the existing raw child-lowering capability.
///
/// Catalog-addressable Box methods, top-level functions, and selected instance
/// constructors use source-keyed extensions. Raw/reference ports retain the
/// raw child terminal and must never consume those normal-only receipts.
pub(in crate::mir::builder) trait RootCallableCapturePortV1:
    RawBoxMethodChildPortV1 + RawAstChildLoweringPortV1
{
    /// Lower one source-backed App Main static child.  The package adapter
    /// overrides this with its typed same-cohort admission; raw ports retain
    /// their compatibility-only direct child terminal.
    fn lower_app_main_static_child(
        &mut self,
        builder: &mut super::MirBuilder,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        let (symbol, params, param_decls, return_type_name, body, uses, attrs) =
            child.to_owned_lowering().into_parts();
        self.lower_static_box_method(
            builder,
            symbol,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

    /// Selected normal top-level functions carry a source-order occurrence
    /// receipt.  Raw/reference ports must never consume that receipt.
    #[allow(clippy::too_many_arguments)]
    fn lower_normal_top_level_function(
        &mut self,
        _builder: &mut super::MirBuilder,
        _admission: NormalTopLevelFunctionDraftAdmissionV1,
        _params: Vec<String>,
        _param_decls: Vec<ParamDecl>,
        _return_type_name: Option<String>,
        _body: Vec<ASTNode>,
        _uses: Vec<String>,
        _attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        Err("[freeze:contract][mir/top-level-function-admission/raw-port]".to_owned())
    }

    /// Selected normal instance constructors carry a parser-map-keyed source
    /// occurrence. Raw/reference ports must never consume that receipt.
    #[allow(clippy::too_many_arguments)]
    fn lower_normal_instance_constructor(
        &mut self,
        _builder: &mut super::MirBuilder,
        _source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        _params: Vec<String>,
        _param_decls: Vec<ParamDecl>,
        _return_type_name: Option<String>,
        _body: Vec<ASTNode>,
        _uses: Vec<String>,
        _attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        Err("[freeze:contract][mir/instance-constructor-admission/raw-port]".to_owned())
    }

    /// Selected-normal constructors must carry the work-plan-issued linear
    /// demand ticket into the installed semantic-package adapter. Raw and
    /// compatibility ports intentionally reject this typed surface.
    #[allow(clippy::too_many_arguments)]
    fn lower_normal_instance_constructor_with_demand(
        &mut self,
        _builder: &mut super::MirBuilder,
        _source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        _ticket: super::normal_instance_constructor_admission::InstanceConstructorDemandTicketV1,
        _params: Vec<String>,
        _param_decls: Vec<ParamDecl>,
        _return_type_name: Option<String>,
        _body: Vec<ASTNode>,
        _uses: Vec<String>,
        _attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        Err("[freeze:contract][mir/instance-constructor-demand/raw-port]".to_owned())
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_static_box_method(
        &mut self,
        builder: &mut super::MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.lower_static_box_method(
            builder,
            admission.physical_symbol().to_owned(),
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_instance_box_method(
        &mut self,
        builder: &mut super::MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        let function_name = admission.physical_symbol().to_owned();
        let owner = admission.source_key().owner().to_owned();
        let method = admission.source_key().name().to_owned();
        let canonical_key = admission.source_key().clone();
        self.lower_root_instance_method(
            builder,
            canonical_key,
            owner,
            method,
            function_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

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

impl RootCallableCapturePortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_normal_top_level_function(
        &mut self,
        builder: &mut super::MirBuilder,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.lower_normal_top_level_function_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
        .map_err(|error| error.to_string())
    }

    fn lower_normal_instance_constructor(
        &mut self,
        builder: &mut super::MirBuilder,
        source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.lower_normal_instance_constructor_v1(
            builder,
            source_key,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
        .map_err(|error| error.to_string())
    }

    fn lower_cataloged_static_box_method(
        &mut self,
        builder: &mut super::MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.lower_normal_cataloged_static_box_method_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
        .map_err(|error| error.to_string())
    }

    fn lower_cataloged_instance_box_method(
        &mut self,
        builder: &mut super::MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.lower_normal_cataloged_instance_box_method_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
        .map_err(|error| error.to_string())
    }
}
impl RootCallableCapturePortV1 for RawLegacyChildLoweringPortV1 {}

impl super::MirBuilder {
    pub(super) fn prepare_module(&mut self) -> Result<(), String> {
        self.prepare_module_with_callable_main_policy(
            super::module_compat_policy::CallableMainCompatibilityPolicyV1::snapshot_from_legacy_ingress(),
            crate::config::env::builder_safepoint_entry(),
        )
    }

    pub(super) fn prepare_normal_default_module(
        &mut self,
        entry_safepoint: bool,
    ) -> Result<(), String> {
        self.prepare_module_with_callable_main_policy(
            super::module_compat_policy::CallableMainCompatibilityPolicyV1::Omitted,
            entry_safepoint,
        )
    }

    fn prepare_module_with_callable_main_policy(
        &mut self,
        callable_main_policy: super::module_compat_policy::CallableMainCompatibilityPolicyV1,
        entry_safepoint: bool,
    ) -> Result<(), String> {
        self.comp_ctx.clear_callable_declaration_catalog();
        self.comp_ctx.callable_main_compatibility_policy = callable_main_policy;
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

        if entry_safepoint {
            self.emit_instruction(MirInstruction::Safepoint)?;
        }

        Ok(())
    }

    /// Finalize MIR module after a typed or responsibility-local lowering owner.
    ///
    /// Execution flow:
    /// 1. Type propagation (TypePropagationPipeline)
    /// 2. Type hint provision (delegation to type_hint_providers)
    /// 3. Return type inference (delegation to return_type_strategy)
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

        super::module_finalization_function_metadata::
            PreparedModuleFinalizationFunctionMetadataV1::prepare(
                &function,
                &self.function_state.type_ctx.value_types,
                self.value_origin_caller_rows(),
            )
            .commit_into(&mut function);

        // ===== Step 3: return type strategy =====
        // Multi-phase resolver chain (P3-A/B/C/D/P4) for return type resolution
        if let Some(inferred_type) =
            return_type_strategy::infer_return_type_from_phi(self, &mut function)
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

        // main 関数スコープの Region スタックをポップするよ。
        crate::mir::region::observer::pop_function_region(self);

        // main 関数スコープの SlotRegistry を解放するよ。
        self.comp_ctx.current_slot_registry = None;

        super::module_finalization_declaration_metadata::
            PreparedModuleFinalizationDeclarationMetadataV1::prepare(&self.comp_ctx)
            .commit_into(&mut module);
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
