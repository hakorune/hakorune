# 293x-297 ASTCLEAN-004 dead-code allowance inventory

Status: complete

## Decision

Decision: accepted.

`#[allow(dead_code)]` cleanup is an inventory-first cleanup lane. The first row does not bulk-delete allowances; it fixes the baseline, risk categories, and next-row split so later removals stay small and reviewable.

## Baseline

Source baseline count: 210 `#[allow(dead_code)]` attributes under `src/`.

Top source buckets:

| Bucket | Count |
| --- | ---: |
| `src/mir` | 114 |
| `src/runner` | 31 |
| `src/backend` | 23 |
| `src/boxes` | 10 |
| `src/runtime` | 7 |
| `src/bid` | 7 |
| `src/host_providers` | 5 |
| `src/tests` | 4 |
| `src/ring0` | 3 |
| `src/bid-converter-copilot` | 3 |
| `src/macro` | 1 |
| `src/core` | 1 |
| `src/benchmarks` | 1 |

Top files observed:

| File | Count | Initial handling |
| --- | ---: | --- |
| `src/mir/numeric_substrate.rs` | 18 | split into dedicated MIR cleanup row; likely substrate staging residue |
| `src/mir/builder/type_registry.rs` | 10 | inspect before removal; may be staged registry API |
| `src/mir/builder/loops.rs` | 10 | inspect before removal; loop builder surfaces may be staged |
| `src/backend/mir_interpreter/utils/error_helpers.rs` | 7 | backend utility row candidate |
| `src/mir/optimizer/diagnostics.rs` | 6 | diagnostics row candidate |
| `src/mir/builder/scope_context.rs` | 6 | inspect carefully; builder state API may be planned |
| `src/mir/builder/utils/local_ssa.rs` | 5 | MIR utility row candidate |
| `src/host_providers/llvm_codegen.rs` | 5 | host-provider row; likely optional backend staging |

## Row split

Recommended next cleanup rows:

1. `ASTCLEAN-005 MIR dead_code allowance prune pilot`
2. `ASTCLEAN-006 runner/backend dead_code allowance prune pilot`
3. `ASTCLEAN-007 intentional dead_code reason comments`
4. `ASTCLEAN-008 remaining dead_code allowance burn-down ledger`

## Stop line

Do not remove allowances in this row. Do not touch runtime semantics, parser grammar, backend behavior, or tests except dedicated row guards. Any allowance removed later must compile under that row guard.

## Guard

- `tools/checks/k2_wide_astclean_dead_code_inventory_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_dead_code_inventory_guard.sh` passed locally.
