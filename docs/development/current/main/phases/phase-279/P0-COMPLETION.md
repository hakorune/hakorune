# Phase 279 P0: Type propagation pipeline SSOT unification — completion

Status: ✅ completed (2025-12-22)

Goal:
- Eliminate "2本のコンパイラ" (two compilers) problem
- Root fix for order drift where identical fixtures pass in one route but fail in another
- Unify type propagation into single SSOT entry with fixed order

SSOT implementation:
- `src/mir/type_propagation/pipeline.rs` - SSOT type propagation pipeline
- `TypePropagationPipeline::run()` - Single entry point for all routes

Pipeline steps (fixed order):
1. Copy propagation (initial)
2. BinOp re-propagation (numeric promotion: Int+Float→Float)
3. Copy propagation (propagate promoted types)
4. PHI type inference (private step - cannot bypass)

Callers (both routes now use SSOT):
- `src/mir/builder/lifecycle.rs::finalize_module()` - Builder lifecycle route
- `src/mir/join_ir_vm_bridge/joinir_function_converter.rs::propagate_types()` - JoinIR bridge route

Fail-fast guard (structural guarantee):
- PHI type inference is **private step** inside TypePropagationPipeline
- lifecycle.rs and joinir_function_converter.rs **cannot call PhiTypeResolver directly**
- Only public API: `TypePropagationPipeline::run()`
- Order drift is **structurally impossible** (private encapsulation)

Environment variables (reused existing flags):
- `NYASH_PHI_GLOBAL_DEBUG=1` - PHI type inference debug output
- `NYASH_BINOP_REPROP_DEBUG=1` - BinOp re-propagation debug output
- **No new environment variables** (Phase 279 P0 policy)

Files changed (6 total):
- New: `src/mir/type_propagation/mod.rs` - Module definition
- New: `src/mir/type_propagation/pipeline.rs` - SSOT pipeline (~300 lines)
- Modified: `src/mir/mod.rs` - Add type_propagation module export
- Modified: `src/mir/builder/lifecycle.rs` - Replace with SSOT call (~400 lines removed)
- Modified: `src/mir/join_ir_vm_bridge/joinir_function_converter.rs` - Replace with SSOT call (~100 lines removed)
- Modified: `docs/development/current/main/phases/phase-279/README.md` - Document SSOT entry

Code reduction:
- ~500 lines of duplicate BinOp re-propagation logic removed
- 2 implementations consolidated into 1 SSOT

Regression testing:
- ✅ Lifecycle route (VM backend): Phase 275 fixture - exit=3
- ✅ JoinIR route (VM backend): loop_min_while.hako - exit=0
- ✅ LLVM harness: Phase 275 fixture - compiled and executed successfully

Acceptance criteria met:
- ✅ Single SSOT entry for type propagation (`TypePropagationPipeline::run()`)
- ✅ Both routes call SSOT entry (lifecycle.rs + joinir_function_converter.rs)
- ✅ No duplicate BinOp re-propagation logic
- ✅ Fixed order: Copy → BinOp → Copy → PHI
- ✅ Fail-fast guard prevents PHI-before-BinOp (structural guarantee)
- ✅ All regression tests pass
- ✅ Documentation reflects SSOT entry point

Next phase:
- Phase 280 (planned): Further compiler unification work
