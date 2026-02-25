# Phase 279 P0: Type propagation pipeline SSOT unification

Status: ✅ completed (2025-12-22)

Goal: eliminate “two compiler pipelines” by making type propagation run through **one SSOT entry** with a fixed order across routes, so the same fixture cannot pass in one route and fail in another purely due to ordering drift.

Background trigger:
- A real incident occurred where PHI type resolution ran before BinOp re-propagation in one route, but after it in another, producing LLVM parity breakage. This is effectively “two compilers”.

Scope:
- Define a single type propagation pipeline entry (SSOT).
- Make every route that emits MIR/LLVM metadata call that SSOT entry.
- Add fail-fast guards that make order drift impossible to miss.

SSOT references:
- Current status log: `docs/development/current/main/10-Now.md`
- Backlog entry: `docs/development/current/main/30-Backlog.md`

Implementation guide:
- `docs/development/current/main/phases/phase-279/P0-INSTRUCTIONS.md`
- Claude Code instructions:
  - `docs/development/current/main/phases/phase-279/P0-CLAUDE.md`

Non-goals:
- new language features (no Union/Any)
- broad optimizer rewrite
- adding new environment variables

---

## SSOT Entry Point

**File**: `src/mir/type_propagation/pipeline.rs`

This is the **single source of truth** for type propagation. All routes MUST call this pipeline.

### Main Entry

- `TypePropagationPipeline::run(function, value_types)` - SSOT entry point

### Pipeline Steps (fixed order)

1. **Copy propagation** (initial) - Propagate types through Copy chains
2. **BinOp re-propagation** - Numeric promotion (Int+Float→Float, String+String→StringBox)
3. **Copy propagation** (after promotion) - Propagate promoted types
4. **PHI type inference** (private step) - Resolve PHI node types

### Callers (exhaustive list)

- `src/mir/builder/lifecycle.rs::finalize_module()` - Builder lifecycle route (AST → MIR)
- `src/mir/join_ir_vm_bridge/joinir_function_converter.rs::propagate_types()` - JoinIR bridge route (JoinIR → MIR)

### Consumer (read-only)

- LLVM harness (Python `llvm_py/`) - Consumer only, no type propagation logic (best-effort forbidden)

### Fail-Fast Guards

**Structural guarantee** (Phase 279 P0):
- PHI type inference is a **private step** inside TypePropagationPipeline
- lifecycle.rs and joinir_function_converter.rs **cannot call PhiTypeResolver directly**
- Only public API: `TypePropagationPipeline::run()`
- Order drift is **structurally impossible** (private encapsulation)

**Environment variables** (existing, reused):
- `NYASH_PHI_GLOBAL_DEBUG=1` - PHI type inference debug output
- `NYASH_BINOP_REPROP_DEBUG=1` - BinOp re-propagation debug output

**No new environment variables** (Phase 279 P0 policy: prevent env var sprawl)

---

## Implementation Summary

**Files changed** (6 total):
- New: `src/mir/type_propagation/mod.rs` - Module definition
- New: `src/mir/type_propagation/pipeline.rs` - SSOT pipeline implementation (~300 lines)
- Modified: `src/mir/mod.rs` - Add type_propagation module export
- Modified: `src/mir/builder/lifecycle.rs` - Replace with SSOT call (~400 lines removed)
- Modified: `src/mir/join_ir_vm_bridge/joinir_function_converter.rs` - Replace with SSOT call (~100 lines removed)
- Modified: `docs/development/current/main/phases/phase-279/README.md` - Document SSOT entry

**Code reduction**:
- Removed ~500 lines of duplicate BinOp re-propagation logic
- Consolidated 2 implementations into 1 SSOT

**Completion**:
- `docs/development/current/main/phases/phase-279/P0-COMPLETION.md` - Completion record
