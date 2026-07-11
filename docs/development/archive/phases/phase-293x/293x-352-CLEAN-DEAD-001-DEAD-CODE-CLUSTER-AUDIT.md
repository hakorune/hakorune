# 293x-352 CLEAN-DEAD-001 dead_code cluster audit

Status: landed
Date: 2026-05-15

## Decision

`#[allow(dead_code)]` cleanup must proceed cluster-by-cluster. The first audit
classifies the largest current clusters rather than blindly deleting attributes.

## Audit result

| Cluster | Count | Decision | Reason |
| --- | ---: | --- | --- |
| `src/mir/numeric_substrate.rs` | 18 | keep for now | Exact numeric substrate models/policies are staged for later MIR fact, verifier, VM, and backend rows. Existing attributes already carry row-specific reason comments. |
| `src/mir/builder/type_registry.rs` | 6 | keep for now | Trace/debug and scoped reset hooks are staged for builder metadata and diagnostics. Existing attributes already carry ASTCLEAN reason comments. |

## Non-goals

- Do not remove dead-code allows without a build/test gate in the same row.
- Do not replace precise per-item reasons with a broad module-level allow.
- Do not audit unrelated clusters in the same commit.

## Follow-up

`CLEAN-DEAD-002` may audit the next cluster (`host_providers/llvm_codegen.rs` or
plugin loader error reporting) if cleanup remains selected. It is not a blocker
for `MIMAP-012`.
