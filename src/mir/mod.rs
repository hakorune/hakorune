/*!
 * Nyash MIR (Mid-level Intermediate Representation) - Stage 1 Implementation
 *
 * ChatGPT5-designed MIR infrastructure for native compilation support
 * Based on SSA form with effect tracking and Box-aware optimizations
 */

pub mod analysis; // analysis-only views (no AST rewrite)
#[cfg(feature = "aot-plan-import")]
pub mod aot_plan_import;
pub mod basic_block;
pub mod builder;
pub mod contracts; // backend-core instruction contracts (SSOT)
pub mod definitions; // Unified MIR definitions (MirCall, Callee, etc.)
pub mod diagnostics; // freeze diagnostics helpers (SSOT)
pub mod effect;
pub mod escape_barrier; // escape operand-role vocabulary (SSOT)
pub mod function;
pub mod if_in_loop_phi; // Phase 187-2: Minimal if-in-loop PHI emitter (extracted from loop_builder)
pub mod instruction;
pub mod instruction_introspection; // Introspection helpers for tests (instruction names)
pub mod instruction_kinds; // small kind-specific metadata (Const/BinOp)
pub mod loop_api; // Minimal LoopBuilder facade (adapter-ready)
pub mod loop_canonicalizer; // Phase 1: Loop skeleton canonicalization (AST preprocessing)
pub mod naming; // Static box / entry naming rules（NamingBox）
pub mod optimizer;
pub mod policies; // shared routing policies (SSOT)
pub mod ssot; // Shared helpers (SSOT) for instruction lowering
pub mod types; // core MIR enums (ConstValue, Ops, MirType)
pub mod utils; // Phase 15 control flow utilities for root treatment
               // pub mod lowerers; // reserved: Stage-3 loop lowering (while/for-range)
pub mod cfg_extractor; // Phase 154: CFG extraction for hako_check
pub mod control_form;
pub mod control_tree; // Phase 110: Structure-only SSOT (StepTree)
pub mod function_emission; // FunctionEmissionBox（MirFunction直編集の発行ヘルパ）
pub mod hints; // scaffold: zero-cost guidance (no-op)
pub mod join_ir; // Phase 26-H: 関数正規化IR（JoinIR）
pub mod join_ir_ops; // Phase 27.8: JoinIR 命令意味箱（ops box）
pub mod join_ir_runner; // Phase 27.2: JoinIR 実行器（実験用）
pub mod join_ir_vm_bridge; // Phase 27-shortterm S-4: JoinIR → Rust VM ブリッジ
pub mod join_ir_vm_bridge_dispatch; // Phase 30 F-4.4: JoinIR VM ブリッジ dispatch helper
pub mod loop_form; // ControlForm::LoopShape の薄いエイリアス
pub mod loop_route_detection; // Active module surface for loop route-shape detection
pub mod optimizer_passes; // optimizer passes (normalize/diagnostics)
pub mod optimizer_stats; // extracted stats struct
pub mod passes;
pub mod phi_core; // Phase 1 scaffold: unified PHI entry (re-exports only)
pub(crate) mod phi_query; // generic PHI base-relation seam for later relation consumers
pub mod printer;
mod printer_helpers; // internal helpers extracted from printer.rs
pub mod query; // Phase 26-G: MIR read/write/CFGビュー (MirQuery)
pub mod region; // Phase 25.1l: Region/GC観測レイヤ（LoopForm v2 × RefKind）
pub mod semantic_refresh; // MIR semantic metadata refresh owner (SSOT)
pub mod slot_registry; // Phase 9.79b.1: method slot resolution (IDs)
mod spanned_instruction;
pub mod storage_class; // primitive / user-box storage-class inventory + refresh helper
pub mod string_corridor; // string canonical corridor facts + refresh helper
pub(crate) mod string_corridor_compat; // compat semantic recovery quarantined from canonical facts
pub mod string_corridor_placement; // placement/effect scaffold over canonical string facts
pub(crate) mod string_corridor_recognizer; // shared pure shape recognizers for string corridor
pub mod string_corridor_relation; // string-corridor relation layer over generic PHI queries
pub mod string_kernel_plan; // backend-consumable string plan seam derived from corridor candidates
pub mod sum_placement; // sum-local proving slice for later generic placement/effect pass
pub mod sum_placement_layout; // LLVM-side payload-lane choices for selected local sums
pub mod sum_placement_selection; // selection pilot over sum-local placement facts
pub mod thin_entry; // thin-entry inventory for known local routes
pub mod thin_entry_selection; // manifest-driven thin-entry selection pilot
pub mod type_propagation; // Phase 279 P0: SSOT type propagation pipeline
pub mod value_id;
pub mod value_kind; // Phase 26-A: ValueId型安全化
pub mod value_origin; // generic copy-root / alias-root owner (SSOT)
pub mod verification;
pub mod verification_types; // extracted error types // Optimization subpasses (e.g., type_hints) // Phase 25.1f: Loop/If 共通ビュー（ControlForm）

// Re-export main types for easy access
pub use basic_block::{BasicBlock, EdgeArgs, OutEdge};
pub use builder::MirBuilder;
pub use hakorune_mir_core::{BasicBlockId, BasicBlockIdGenerator, BindingId};

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use builder::detect_skip_whitespace_shape;
// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use builder::detect_read_digits_loop_true_shape;
// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use builder::detect_continue_shape;
// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use builder::detect_parse_number_shape;
// Phase 143-P1:
pub(crate) use builder::detect_parse_string_shape;
// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub(crate) use builder::detect_escape_skip_shape;
pub use cfg_extractor::extract_cfg_info; // Phase 154: CFG extraction
pub use definitions::{CallFlags, Callee, MirCall}; // Unified call definitions
pub use effect::{Effect, EffectMask};
pub use escape_barrier::{classify_escape_uses, EscapeBarrier, EscapeUse};
pub use function::{
    ClosureBodyId, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, MirFunction, MirModule,
    UserBoxFieldDecl,
};
pub use instruction::MirInstruction;
pub use join_ir_runner::{run_joinir_function, JoinRuntimeError, JoinValue};
pub use optimizer::MirOptimizer;
pub use printer::MirPrinter;
pub use query::{MirQuery, MirQueryBox};
pub use semantic_refresh::{
    refresh_function_semantic_metadata, refresh_function_string_corridor_metadata,
    refresh_module_semantic_metadata,
};
pub use slot_registry::{BoxTypeId, MethodSlot};
pub use spanned_instruction::{SpannedInstRef, SpannedInstruction};
pub use storage_class::{
    refresh_function_storage_class_facts, refresh_module_storage_class_facts, StorageClass,
};
pub use string_corridor::{
    refresh_function_string_corridor_facts, refresh_module_string_corridor_facts,
    StringCorridorCarrier, StringCorridorFact, StringCorridorOp, StringCorridorRole,
    StringOutcomeFact, StringPlacementFact,
};
pub use string_corridor_placement::{
    refresh_function_string_corridor_candidates, refresh_module_string_corridor_candidates,
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorCandidateState,
};
pub use string_corridor_relation::{
    refresh_function_string_corridor_relations, refresh_module_string_corridor_relations,
    StringCorridorRelation, StringCorridorRelationKind, StringCorridorWindowContract,
};
pub use string_kernel_plan::{
    derive_string_kernel_plan, StringKernelPlan, StringKernelPlanConsumer, StringKernelPlanFamily,
    StringKernelPlanLegality, StringKernelPlanPart, StringKernelPlanRetainedForm,
};
pub use sum_placement::{
    refresh_function_sum_placement_facts, refresh_module_sum_placement_facts,
    SumObjectizationBarrier, SumPlacementFact, SumPlacementState,
};
pub use sum_placement_layout::{
    refresh_function_sum_placement_layouts, refresh_module_sum_placement_layouts,
    SumLocalAggregateLayout, SumPlacementLayout,
};
pub use sum_placement_selection::{
    refresh_function_sum_placement_selections, refresh_module_sum_placement_selections,
    SumPlacementPath, SumPlacementSelection,
};
pub use thin_entry::{
    refresh_function_thin_entry_candidates, refresh_module_thin_entry_candidates,
    ThinEntryCandidate, ThinEntryCurrentCarrier, ThinEntryPreferredEntry, ThinEntrySurface,
    ThinEntryValueClass,
};
pub use thin_entry_selection::{
    refresh_function_thin_entry_selections, refresh_module_thin_entry_selections,
    ThinEntrySelection, ThinEntrySelectionState,
};
pub use types::{
    BarrierOp, BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, WeakRefOp,
};
pub use value_id::{LocalId, ValueId, ValueIdGenerator};
pub use value_kind::{MirValueKind, TypedValueId}; // Phase 26-A: ValueId型安全化
pub use value_origin::{
    build_value_def_map, resolve_value_origin, resolve_value_origin_from_copy_parents,
    resolve_value_origin_from_parent_map, CopyParentMap, ParentMap, ValueDefMap,
};
pub use verification::MirVerifier;
pub use verification_types::VerificationError;
// Phase 29y.1: RC insertion pass (skeleton)
pub use passes::rc_insertion::{insert_rc_instructions, RcInsertionStats};
// Phase 15 control flow utilities (段階的根治戦略)
pub use utils::{
    capture_actual_predecessor_and_jump, collect_phi_incoming_if_reachable,
    execute_statement_with_termination_check, is_current_block_terminated,
};

/// MIR compilation result
#[derive(Debug, Clone)]
pub struct MirCompileResult {
    pub module: MirModule,
    pub verification_result: Result<(), Vec<VerificationError>>,
}

/// MIR compiler - converts AST to MIR/SSA form
pub struct MirCompiler {
    builder: MirBuilder,
    verifier: MirVerifier,
    optimize: bool,
}

impl MirCompiler {
    /// Create a new MIR compiler
    pub fn new() -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize: true,
        }
    }
    /// Create with options
    pub fn with_options(optimize: bool) -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize,
        }
    }

    /// Phase 288 P2: Set REPL mode flag
    pub fn set_repl_mode(&mut self, repl_mode: bool) {
        self.builder.repl_mode = repl_mode;
    }

    /// Phase 288: REPL mode での内部ログ抑制フラグを設定
    pub fn set_quiet_internal_logs(&mut self, quiet: bool) {
        self.builder.comp_ctx.quiet_internal_logs = quiet;
    }

    /// Compile AST to MIR module with verification
    pub fn compile_with_source(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        self.builder.comp_ctx.clear_using_import_boxes();
        self.compile_with_source_internal(ast, source_file)
    }

    /// Compile AST to MIR with an explicit imported static-box alias table.
    pub fn compile_with_source_and_imports(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
        imports: std::collections::HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        self.builder.comp_ctx.set_using_import_boxes(imports);
        self.compile_with_source_internal(ast, source_file)
    }

    fn compile_with_source_internal(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        if let Some(src) = source_file {
            self.builder.set_source_file_hint(src.to_string());
        } else {
            self.builder.clear_source_file_hint();
        }
        // Convert AST to MIR using builder
        let mut module = self.builder.build_module(ast)?;

        if self.optimize {
            let mut optimizer = MirOptimizer::new();
            let stats = optimizer.optimize_module(&mut module);
            if (crate::config::env::opt_diag_fail() || crate::config::env::opt_diag_forbid_legacy())
                && stats.diagnostics_reported > 0
            {
                return Err(format!(
                    "Diagnostic failure: {} issues detected (unlowered/legacy)",
                    stats.diagnostics_reported
                ));
            }
        }

        // Verify the generated MIR
        let verification_result = self.verifier.verify_module(&module);

        // Phase 29y.1: RC insertion pass (skeleton - no-op for now)
        // Runs after optimization and verification, before backend codegen
        let _rc_stats = insert_rc_instructions(&mut module);
        refresh_module_semantic_metadata(&mut module);

        Ok(MirCompileResult {
            module,
            verification_result,
        })
    }

    /// Compile AST to MIR module with verification (no source hint).
    pub fn compile(&mut self, ast: crate::ast::ASTNode) -> Result<MirCompileResult, String> {
        self.compile_with_source(ast, None)
    }

    /// Dump MIR to string for debugging
    pub fn dump_mir(&self, module: &MirModule) -> String {
        MirPrinter::new().print_module(module)
    }
}

impl Default for MirCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue};

    #[test]
    fn test_basic_mir_compilation() {
        let mut compiler = MirCompiler::new();

        // Create a simple literal AST node
        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: crate::ast::Span::unknown(),
        };

        // Compile to MIR
        let result = compiler.compile(ast);
        assert!(result.is_ok(), "Basic MIR compilation should succeed");

        let compile_result = result.unwrap();
        assert!(
            !compile_result.module.functions.is_empty(),
            "Module should contain at least one function"
        );
    }

    #[test]
    fn test_mir_dump() {
        let mut compiler = MirCompiler::new();

        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: crate::ast::Span::unknown(),
        };

        let result = compiler.compile(ast).unwrap();
        let mir_dump = compiler.dump_mir(&result.module);

        assert!(!mir_dump.is_empty(), "MIR dump should not be empty");
        assert!(
            mir_dump.contains("define"),
            "MIR dump should contain function definition"
        );
    }

    #[test]
    fn test_lowering_is_type_function_call_in_print() {
        // Build AST: print(isType(42, "Integer"))
        let ast = ASTNode::Print {
            expression: Box::new(ASTNode::FunctionCall {
                name: "isType".to_string(),
                arguments: vec![
                    ASTNode::Literal {
                        value: LiteralValue::Integer(42),
                        span: crate::ast::Span::unknown(),
                    },
                    ASTNode::Literal {
                        value: LiteralValue::String("Integer".to_string()),
                        span: crate::ast::Span::unknown(),
                    },
                ],
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");

        // Ensure TypeOp exists in the resulting MIR
        let has_typeop = result.module.functions.values().any(|f| {
            f.blocks.values().any(|b| {
                b.all_spanned_instructions()
                    .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
            })
        });
        assert!(
            has_typeop,
            "Expected TypeOp lowering for print(isType(...))"
        );
    }

    #[test]
    fn test_lowering_is_method_call_in_print() {
        // Build AST: print( (42).is("Integer") )
        let ast = ASTNode::Print {
            expression: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(42),
                    span: crate::ast::Span::unknown(),
                }),
                method: "is".to_string(),
                arguments: vec![ASTNode::Literal {
                    value: LiteralValue::String("Integer".to_string()),
                    span: crate::ast::Span::unknown(),
                }],
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");

        // Ensure TypeOp exists in the resulting MIR
        let has_typeop = result.module.functions.values().any(|f| {
            f.blocks.values().any(|b| {
                b.all_spanned_instructions()
                    .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
            })
        });
        assert!(
            has_typeop,
            "Expected TypeOp lowering for print(obj.is(...))"
        );
    }

    #[test]
    #[ignore = "MIR13 migration: extern console.log expectation pending"]
    fn test_lowering_extern_console_log() {
        // Build AST: console.log("hi") → ExternCall env.console.log
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "console".to_string(),
                span: crate::ast::Span::unknown(),
            }),
            method: "log".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::String("hi".to_string()),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");
        let dump = MirPrinter::verbose().print_module(&result.module);

        assert!(
            dump.contains("extern_call env.console.log"),
            "Expected extern_call env.console.log in MIR dump. Got:\n{}",
            dump
        );
    }

    #[test]
    fn test_lowering_boxcall_array_push() {
        // Build AST: (new ArrayBox()).push(1)
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::New {
                class: "ArrayBox".to_string(),
                arguments: vec![],
                type_arguments: vec![],
                span: crate::ast::Span::unknown(),
            }),
            method: "push".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");
        let dump = MirPrinter::new().print_module(&result.module);
        // Expect a BoxCall to push (printer formats as `call <box>.<method>(...)`)
        assert!(
            dump.contains(".push("),
            "Expected BoxCall to .push(...). Got:\n{}",
            dump
        );
    }

    #[test]
    fn test_compile_attaches_string_corridor_fact_for_string_length() {
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::Literal {
                value: LiteralValue::String("hello".to_string()),
                span: crate::ast::Span::unknown(),
            }),
            method: "length".to_string(),
            arguments: vec![],
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");

        let len_fact_count = result
            .module
            .functions
            .values()
            .flat_map(|function| function.metadata.string_corridor_facts.values())
            .filter(|fact| fact.op == StringCorridorOp::StrLen)
            .count();

        assert!(
            len_fact_count >= 1,
            "expected at least one str.len fact in compiled MIR"
        );
    }

    #[test]
    fn test_compile_attaches_string_corridor_candidate_for_string_length() {
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::Literal {
                value: LiteralValue::String("hello".to_string()),
                span: crate::ast::Span::unknown(),
            }),
            method: "length".to_string(),
            arguments: vec![],
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");

        let direct_kernel_candidate_count = result
            .module
            .functions
            .values()
            .flat_map(|function| function.metadata.string_corridor_candidates.values())
            .flatten()
            .filter(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry)
            .count();

        assert!(
            direct_kernel_candidate_count >= 1,
            "expected at least one direct-kernel-entry candidate in compiled MIR"
        );
    }

    #[test]
    #[ignore = "MIR13 migration: method id naming in printer pending"]
    fn test_boxcall_method_id_on_universal_slot() {
        // Build AST: (new ArrayBox()).toString()
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::New {
                class: "ArrayBox".to_string(),
                arguments: vec![],
                type_arguments: vec![],
                span: crate::ast::Span::unknown(),
            }),
            method: "toString".to_string(),
            arguments: vec![],
            span: crate::ast::Span::unknown(),
        };

        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");
        let dump = MirPrinter::new().print_module(&result.module);
        // Expect a BoxCall with numeric method id [#0] for toString universal slot
        assert!(
            dump.contains("toString[#0]"),
            "Expected method_id #0 for toString. Dump:\n{}",
            dump
        );
    }

    #[test]
    fn test_lowering_await_expression() {
        if crate::config::env::mir_core13_pure() {
            if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug("[TEST] skip await under Core-13 pure mode");
            }
            return;
        }
        // Build AST: await 1  (semantic is nonsensical but should emit Await)
        let ast = ASTNode::AwaitExpression {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        };
        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile should succeed");
        let dump = MirPrinter::new().print_module(&result.module);
        assert!(
            dump.contains("await"),
            "Expected await in MIR dump. Got:\n{}",
            dump
        );
    }

    // Legacy await / safepoint モデルのテスト（Core-13/Pure 以降とは挙動差あり）.
    #[test]
    #[ignore]
    fn test_await_has_checkpoints() {
        if crate::config::env::mir_core13_pure() {
            if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug("[TEST] skip await under Core-13 pure mode");
            }
            return;
        }
        use crate::ast::{LiteralValue, Span};
        // Build: await 1
        let ast = ASTNode::AwaitExpression {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");
        // Verifier should pass (await flanked by safepoints)
        assert!(
            result.verification_result.is_ok(),
            "Verifier failed for await checkpoints: {:?}",
            result.verification_result
        );
        let dump = compiler.dump_mir(&result.module);
        // Expect at least two safepoints in the function (before/after await)
        let sp_count = dump.matches("safepoint").count();
        assert!(
            sp_count >= 2,
            "Expected >=2 safepoints around await, got {}. Dump:\n{}",
            sp_count,
            dump
        );
    }

    // Legacy await rewrite テスト（現行の Future 統合とは独立にアーカイブ扱い）.
    #[test]
    #[ignore]
    fn test_rewritten_await_still_checkpoints() {
        if crate::config::env::mir_core13_pure() {
            if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug("[TEST] skip await under Core-13 pure mode");
            }
            return;
        }
        use crate::ast::{LiteralValue, Span};
        // Enable rewrite so Await → ExternCall(env.future.await)
        std::env::set_var("NYASH_REWRITE_FUTURE", "1");
        let ast = ASTNode::AwaitExpression {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let mut compiler = MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");
        // Verifier should still pass (checkpoint verification includes ExternCall await)
        assert!(
            result.verification_result.is_ok(),
            "Verifier failed for rewritten await checkpoints: {:?}",
            result.verification_result
        );
        let dump = compiler.dump_mir(&result.module);
        assert!(
            dump.contains("env.future.await"),
            "Expected rewritten await extern call. Dump:\n{}",
            dump
        );
        let sp_count = dump.matches("safepoint").count();
        assert!(
            sp_count >= 2,
            "Expected >=2 safepoints around rewritten await, got {}. Dump:\n{}",
            sp_count,
            dump
        );
        // Cleanup env
        std::env::remove_var("NYASH_REWRITE_FUTURE");
    }

    #[test]
    #[ignore = "MIR13 migration: throw/safepoint expectations pending"]
    fn test_throw_compilation() {
        let mut compiler = MirCompiler::new();

        let throw_ast = ASTNode::Throw {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::String("Test exception".to_string()),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        };

        let result = compiler.compile(throw_ast);
        assert!(result.is_ok(), "Throw compilation should succeed");

        let compile_result = result.unwrap();
        let mir_dump = compiler.dump_mir(&compile_result.module);
        assert!(
            mir_dump.contains("throw"),
            "MIR should contain throw instruction"
        );
        assert!(
            mir_dump.contains("safepoint"),
            "MIR should contain safepoint instruction"
        );
    }

    #[test]
    #[ignore = "MIR13 migration: loop safepoint expectation pending"]
    fn test_loop_compilation() {
        let mut compiler = MirCompiler::new();

        let loop_ast = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: crate::ast::Span::unknown(),
            }),
            body: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::String("Loop body".to_string()),
                    span: crate::ast::Span::unknown(),
                }),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        };

        let result = compiler.compile(loop_ast);
        assert!(result.is_ok(), "Loop compilation should succeed");

        let compile_result = result.unwrap();
        let mir_dump = compiler.dump_mir(&compile_result.module);
        assert!(
            mir_dump.contains("br"),
            "MIR should contain branch instructions"
        );
        assert!(
            mir_dump.contains("safepoint"),
            "MIR should contain safepoint instructions"
        );
    }

    #[test]
    fn test_try_catch_compilation() {
        // Core-13 pure モードでは Try/Catch 命令は許容集合外のためスキップ
        if crate::config::env::mir_core13_pure() {
            if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug("[TEST] skip try/catch under Core-13 pure mode");
            }
            return;
        }
        let mut compiler = MirCompiler::new();

        let try_catch_ast = ASTNode::TryCatch {
            try_body: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::String("Try block".to_string()),
                    span: crate::ast::Span::unknown(),
                }),
                span: crate::ast::Span::unknown(),
            }],
            catch_clauses: vec![crate::ast::CatchClause {
                exception_type: Some("Exception".to_string()),
                variable_name: Some("e".to_string()),
                body: vec![ASTNode::Print {
                    expression: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("Catch block".to_string()),
                        span: crate::ast::Span::unknown(),
                    }),
                    span: crate::ast::Span::unknown(),
                }],
                span: crate::ast::Span::unknown(),
            }],
            finally_body: None,
            span: crate::ast::Span::unknown(),
        };

        let result = compiler.compile(try_catch_ast);
        assert!(result.is_ok(), "TryCatch compilation should succeed");

        let compile_result = result.unwrap();
        let mir_dump = compiler.dump_mir(&compile_result.module);
        assert!(
            mir_dump.contains("catch"),
            "MIR should contain catch instruction"
        );
    }
}
