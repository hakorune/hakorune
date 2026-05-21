# 293x-1070 MIMAP-448A Allocator Comparison C Mimalloc Execution Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the explicit inputs required before any C mimalloc comparison
execution row can run.

## Scope

- Track the presence of an explicit C mimalloc runner/tool.
- Track the representative workload contract inherited from MIMAP-444A.
- Track output and memory-use evidence contracts.
- Track evidence storage and run-count inputs.
- Keep the row inventory-only unless the card explicitly opens execution.

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

Evidence:

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_inventory_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-448A --level L2`

L3/L4 evidence is deferred to a later C mimalloc execution closeout.
