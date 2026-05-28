---
Status: Current
Date: 2026-05-28
Scope: implement LocalSSA same-block value reuse.
Blocker: LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-166-LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION.md
  - src/mir/builder/ssa/local.rs
---

# 296x-167 LocalSSA Same-Block Reuse Implementation

## Purpose

Apply the selected LocalSSA same-block reuse rule in `ensure_inner`, narrowed to
same-block `FieldGet` definitions. A broader all-kind same-block reuse shape
reduced more copies but broke the exact-EXE pure-first backend recipe at
`Main.runOne/2`, so this row keeps the keeper boundary field_get-only.

## Required Output

```text
implementation_contract=local-ssa-same-block-reuse-implementation-v0
instruction_count
copy_count
summary=ok
```

## Implementation

```rust
if def_block == Some(bb) && matches!(def_inst, Some(MirInstruction::FieldGet { .. })) {
    builder.local_ssa_map.insert(key, v);
    return Ok(v);
}
```

## Evidence

```text
implementation_contract=local-ssa-same-block-reuse-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
before_instruction_count=180
after_instruction_count=160
before_copy_count=88
after_copy_count=68
before_local_ssa_copy_count=38
after_local_ssa_copy_count=18
before_call_adjacent_copy_count=40
after_call_adjacent_copy_count=40
before_expression_materialization_copy_count=24
after_expression_materialization_copy_count=7
before_field_get_result_chain_copy_count=23
after_field_get_result_chain_copy_count=6
exact_exe_compile_smoke=ok
proof_allocation_count=524288
proof_free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
summary=ok
```

Interpretation:

```text
This is a structural keeper, but intentionally narrower than the first broad
trial. It removes same-block field_get LocalSSA copies without touching source
.hako shape, PHI construction, cross-block safety checks, or exact-EXE backend
recipe shape. Timing measurement remains a separate row.
```

## Next

```text
row168:
  post-local-ssa-same-block-reuse-measurement

Goal:
  measure exact-EXE timing after the large structural MIR cleanup and keep
  winner/replacement/provider claims closed.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_local_ssa_same_block_reuse_implementation_guard.sh
```
