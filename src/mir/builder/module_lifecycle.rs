//! Module Lifecycle Orchestrator - MIR module construction pipeline
//!
//! Phase 29bq+ cleanliness: lifecycle.rs modularization (623 → ~200 lines)
//!
//! # Purpose
//!
//! Orchestrates the complete MIR module construction pipeline:
//! 1. prepare_module() - Module setup and entry point creation
//! 2. lower_root() - AST lowering with declaration indexing
//! 3. finalize_module() - Type propagation, PHI inference, module sealing
//!
//! # Architecture
//!
//! This orchestrator delegates to specialized modules:
//!
//! - **declaration_indexer** - Pre-indexes user boxes and static methods
//! - **type_hint_providers** - Annotates Call/BoxCall/Await result types
//! - **phi_type_inference** - Multi-phase PHI return type resolution
//!
//! # Execution Flow
//!
//! ```text
//! prepare_module()
//!   ↓
//! lower_root()
//!   ├→ declaration_indexer::index_declarations()  (Phase A: symbol indexing)
//!   ├→ declaration_indexer::has_main_static()      (App vs Script mode)
//!   └→ AST lowering (build_expression, etc.)
//!   ↓
//! finalize_module()
//!   ├→ TypePropagationPipeline::run()              (Copy → BinOp → PHI)
//!   ├→ type_hint_providers::annotate_*()           (Call result types)
//!   ├→ phi_type_inference::infer_return_type()     (P3-A/B/C/D/P4)
//!   └→ Module sealing (metadata, birth verification)
//! ```
//!
//! # Critical Constraints
//!
//! 1. **Execution order固定**: prepare → lower → finalize
//! 2. **Type propagation BEFORE PHI inference**: TypePropagationPipeline runs first
//! 3. **Type hints BEFORE PHI inference**: Ensures value_types populated
//! 4. **PHI resolver order固定**: A → B → P3-D → P4 → P3-C
//!
//! # Called By
//!
//! - `builder_build.rs::build_module()` - Main entry point

use super::{
    BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirModule, MirType, ValueId,
};
use crate::ast::ASTNode;
use crate::config;

// Phase 29bq+: Declaration indexing extracted to dedicated module
use super::declaration_indexer;
// Phase 29bq+: PHI type inference extracted to dedicated module
use super::phi_type_inference;
// Phase 29bq+: Type hint provision extracted to dedicated module
use super::type_hint_providers;

impl super::MirBuilder {
    pub(super) fn prepare_module(&mut self) -> Result<(), String> {
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
        self.scope_ctx.current_function = Some(main_function);
        self.current_block = Some(entry_block);

        // Phase 29bq+: reset sealing session for new function
        self.frag_emit_session.reset();

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

    /// Lower root AST to MIR (Orchestrator Step 2)
    ///
    /// Execution flow:
    /// 1. Declaration indexing (delegation to declaration_indexer)
    /// 2. App vs Script mode detection
    /// 3. AST lowering (build_expression, etc.)
    pub(super) fn lower_root(&mut self, ast: ASTNode) -> Result<ValueId, String> {
        // ===== Step 1: Declaration Indexing (delegation to declaration_indexer) =====
        // Pre-index static methods to enable safe fallback for bare calls in using-prepended code
        let snapshot = ast.clone();
        // Phase A: collect declarations in one pass (symbols available to lowering)
        declaration_indexer::index_declarations(self, &snapshot);

        // Decide root mode (App vs Script) once per module based on presence of static box Main.main
        // true  => App mode (Main.main is entry)
        // false => Script/Test mode (top-level Program runs sequentially)
        let is_app_mode = self
            .root_is_app_mode
            .unwrap_or_else(|| declaration_indexer::has_main_static(&snapshot));
        self.root_is_app_mode = Some(is_app_mode);

        // Phase B: top-level program lowering with declaration-first pass
        match ast {
            ASTNode::Program { statements, .. } => {
                use crate::ast::ASTNode as N;
                // First pass: lower declarations (static boxes except Main, and instance boxes)
                let mut main_static: Option<(String, std::collections::HashMap<String, ASTNode>)> =
                    None;
                for st in &statements {
                    if let N::BoxDeclaration {
                        name,
                        methods,
                        is_static,
                        fields,
                        constructors,
                        weak_fields,
                        ..
                    } = st
                    {
                        if *is_static {
                            if name == "Main" {
                                main_static = Some((name.clone(), methods.clone()));
                            } else {
                                // Script/Test モードでは static box の lowering は exprs.rs 側に任せる
                                if is_app_mode {
                                    // Dev: trace which static box is being lowered (env-gated)
                                    self.trace_compile(format!("lower static box {}", name));
                                    // 🎯 箱理論: 各static boxに専用のコンパイルコンテキストを作成
                                    // これにより、using文や前のboxからのメタデータ汚染を構造的に防止
                                    // スコープを抜けると自動的にコンテキストが破棄される
                                    {
                                        let ctx = super::context::BoxCompilationContext::new();
                                        self.comp_ctx.compilation_context = Some(ctx);

                                        // Lower all static methods into standalone functions: BoxName.method/Arity
                                        for (mname, mast) in methods.iter() {
                                            if let N::FunctionDeclaration { params, body, .. } =
                                                mast
                                            {
                                                let func_name = format!(
                                                    "{}.{}{}",
                                                    name,
                                                    mname,
                                                    format!("/{}", params.len())
                                                );
                                                self.lower_static_method_as_function(
                                                    func_name,
                                                    params.clone(),
                                                    body.clone(),
                                                )?;
                                                self.comp_ctx
                                                    .static_method_index
                                                    .entry(mname.clone())
                                                    .or_insert_with(Vec::new)
                                                    .push((name.clone(), params.len()));
                                            }
                                        }
                                    }

                                    // 🎯 箱理論: コンテキストをクリア（スコープ終了で自動破棄）
                                    // これにより、次のstatic boxは汚染されていない状態から開始される
                                    self.comp_ctx.compilation_context = None;
                                }
                            }
                        } else {
                            // Instance box: register type and lower instance methods/ctors as functions
                            // Phase 285LLVM-1.1: Register with field information for LLVM harness
                            self.comp_ctx
                                .register_user_box_with_fields(name.clone(), fields.clone());
                            self.build_box_declaration(
                                name.clone(),
                                methods.clone(),
                                fields.clone(),
                                weak_fields.clone(),
                            )?;
                            for (ctor_key, ctor_ast) in constructors.iter() {
                                if let N::FunctionDeclaration { params, body, .. } = ctor_ast {
                                    // Keep constructor function name as "Box.birth/N" where ctor_key already encodes arity.
                                    // ctor_key format comes from parser as "birth/<arity>".
                                    let func_name = format!("{}.{}", name, ctor_key);
                                    self.lower_method_as_function(
                                        func_name,
                                        name.clone(),
                                        params.clone(),
                                        body.clone(),
                                    )?;
                                }
                            }
                            for (mname, mast) in methods.iter() {
                                if let N::FunctionDeclaration {
                                    params,
                                    body,
                                    is_static,
                                    ..
                                } = mast
                                {
                                    if !*is_static {
                                        let func_name = format!(
                                            "{}.{}{}",
                                            name,
                                            mname,
                                            format!("/{}", params.len())
                                        );
                                        self.lower_method_as_function(
                                            func_name,
                                            name.clone(),
                                            params.clone(),
                                            body.clone(),
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }

                // Second pass: mode-dependent entry lowering
                if is_app_mode {
                    // App モード: Main.main をエントリとして扱う
                    if let Some((box_name, methods)) = main_static {
                        self.build_static_main_box(box_name, methods)
                    } else {
                        // 理論上は起こりにくいが、安全のため Script モードと同じフォールバックにする
                        self.cf_block(statements)
                    }
                } else {
                    // Script/Test モード: トップレベル Program をそのまま順次実行
                    self.cf_block(statements)
                }
            }
            other => self.build_expression(other),
        }
    }

    /// Finalize MIR module (Orchestrator Step 3)
    ///
    /// Execution flow:
    /// 1. Type propagation (TypePropagationPipeline)
    /// 2. Type hint provision (delegation to type_hint_providers)
    /// 3. PHI type inference (delegation to phi_type_inference)
    /// 4. Module sealing (metadata, birth verification)
    pub(super) fn finalize_module(&mut self, result_value: ValueId) -> Result<MirModule, String> {
        // Hint: scope leave at function end (id=0 for main)
        self.hint_scope_leave(0);
        if let Some(block_id) = self.current_block {
            if let Some(ref mut function) = self.scope_ctx.current_function {
                if let Some(block) = function.get_block_mut(block_id) {
                    if !block.is_terminated() {
                        block.add_instruction(MirInstruction::Return {
                            value: Some(result_value),
                        });
                    }
                    if let Some(mt) = self.type_ctx.value_types.get(&result_value).cloned() {
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
        let mut function = self.scope_ctx.current_function.take().unwrap();

        // ===== Step 1: Type Propagation (TypePropagationPipeline SSOT) =====
        // Phase 279 P0: SSOT type propagation pipeline
        //
        // 全ての型伝播処理を1つの入口（SSOT）に統一。
        // 順序固定: Copy → BinOp → Copy → PHI
        // lifecycle.rs と joinir_function_converter.rs の両方がこのパイプラインを呼ぶ。
        use crate::mir::type_propagation::TypePropagationPipeline;
        TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)?;

        // ===== Step 2: Type Hint Provision (delegation to type_hint_providers) =====
        // Phase 84-5 guard hardening: ensure call/await results are registered in `value_types`
        // before return type inference. This avoids "impossible" debug panics when the builder
        // emitted a value-producing instruction without annotating its dst type.
        type_hint_providers::annotate_missing_result_types_from_calls_and_await(
            self, &function, &module,
        );

        // Phase 131-9: Update function metadata with corrected types
        // MUST happen after PHI type correction above AND BinOp re-propagation
        function.metadata.value_types = self.type_ctx.value_types.clone();
        let mut origin_callers = function.metadata.value_origin_callers.clone();
        for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
            origin_callers.insert(*k, v.clone());
        }
        function.metadata.value_origin_callers = origin_callers;

        // ===== Step 3: PHI Type Inference (delegation to phi_type_inference) =====
        // Phase 29bq+: PHI type inference delegated to phi_type_inference module
        // Multi-phase fallback chain (P3-A/B/C/D/P4) for return type resolution
        if let Some(inferred_type) =
            phi_type_inference::infer_return_type_from_phi(self, &mut function)
        {
            function.signature.return_type = inferred_type;
        }
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

        Ok(module)
    }

    // Phase 131-11-E: Re-propagate BinOp result types after PHI resolution
    // This fixes cases where BinOp instructions were created before PHI types were known
    // Phase 279 P0: repropagate_binop_types() method removed
    // Moved to TypePropagationPipeline (SSOT)
}

// Phase 279 P0: OperandTypeClass enum removed
// Moved to TypePropagationPipeline (SSOT)
