# Rewriter Module - JoinIR→MIR Rewriting Pipeline

**Role**: Implements the 2-stage rewriting pipeline (Plan→Apply) for JoinIR→MIR merge.

**Status**: Phase 287 complete - Plan→Apply SSOT established

---

## SSOT: Entry Point

**Primary entry**: `rewriter/stages/mod.rs`

### Pipeline Stages

1. **Plan Stage** (`stages/plan/`)
   - **Input**: JoinIR MirModule, remapper, boundary, loop_header_phi_info
   - **Output**: `RewrittenBlocks` (pure data structure)
   - **Responsibility**: Pure transformation - generates new blocks WITHOUT touching builder
   - **Modules**:
     - `entry_resolver.rs` - Entry function resolution
     - `instruction_rewrite.rs` - Instruction filtering & remapping
     - `tail_call_rewrite.rs` - Tail call parameter binding
     - `terminator_rewrite.rs` - Terminator conversion & routing

2. **Apply Stage** (`stages/apply.rs`)
   - **Input**: `RewrittenBlocks`, builder, boundary
   - **Output**: Mutated builder
   - **Responsibility**: Builder mutation - adds blocks, injects boundary copies, updates context

---

## Module Structure

### Data Structures
- `plan_box.rs` - `RewrittenBlocks` struct (Plan stage output)
- `rewrite_context.rs` - Shared state consolidation (carrier_inputs, exit_phi_inputs)

### Box Helpers (Single-Responsibility)
- `carrier_inputs_collector.rs` - Carrier inputs collection (DRY)
- `exit_collection.rs` - Exit value collection (Return→Jump + ExitArgsCollectorBox)
- `instruction_filter_box.rs` - Instruction skip judgment
- `return_converter_box.rs` - Return→Jump conversion
- `latch_incoming_recorder.rs` - Latch recording SSOT
- `tail_call_policy.rs` - Entry-like判定 + latch record policy SSOT

### Utility Modules
- `helpers.rs` - Pure functions (`is_skippable_continuation`)
- `plan_helpers.rs` - Helper functions for `plan_rewrites()`
- `terminator.rs` - Terminator remapping (Branch/Jump/Return conversion)
- `type_propagation.rs` - Type propagation (JoinIR→HOST value types)
- `logging.rs` - Debug logging (DEBUG-177 style)

---

## What This Module Does NOT Do

### ❌ Contract Checks
- Contract validation is handled by `merge/contract_checks/*`
- Do NOT add contract checks here - delegate to the dedicated module

### ❌ Boundary Design Changes
- `JoinInlineBoundary` contract is defined elsewhere
- This module consumes boundaries, does NOT design them

### ❌ Silent Fallback
- **Fail-Fast principle**: Errors must fail explicitly
- Do NOT add silent fallback paths - use `Result<T, String>` and propagate errors

### ❌ Box Proliferation
- Phase 287 P7 removed unused Box scaffolding
- Only create new Box helpers if they serve a single, clear responsibility

---

## Phase 287 Evolution

- **P0-P2**: Modularized contract_checks, ast_feature_extractor, boundary_logging
- **P3**: Extracted pipeline stages to `stages/` directory
- **P4**: Modularized `plan.rs` using facade pattern (84% reduction)
- **P5**: Unified stages API through `stages/mod.rs` facade
- **P6**: Removed Scan stage (2-stage Plan→Apply pipeline)
- **P7**: Removed unused Box scaffolding (apply_box, tail_call_detector_box, parameter_binding_box)
- **P8**: Added README.md (this file) - responsibility boundaries and SSOT guard

---

## Usage Example

```rust
use super::rewriter::stages::{plan_rewrites, apply_rewrites};

// Stage 1: Plan (pure transformation)
let blocks = plan_rewrites(
    mir_module,
    remapper,
    function_params,
    boundary,
    loop_header_phi_info,
    ctx,
    value_to_func_name,
    debug,
)?;

// Stage 2: Apply (builder mutation)
apply_rewrites(
    builder,
    blocks,
    boundary,
    remapper,
    loop_header_phi_info,
    mir_module,
    ctx,
    debug,
)?;
```

---

## Architecture Principles

1. **Single Source of Truth**: `stages/` is the entry point - do NOT create parallel entry paths
2. **Pure Transformation**: Plan stage must NOT mutate builder
3. **Builder Mutation**: Apply stage is the ONLY place that mutates builder
4. **Fail-Fast**: Errors fail explicitly - no silent fallback
5. **Single Responsibility**: Each Box/module serves one clear purpose

---

**Last Updated**: Phase 287 P8 (2025-12-27)
