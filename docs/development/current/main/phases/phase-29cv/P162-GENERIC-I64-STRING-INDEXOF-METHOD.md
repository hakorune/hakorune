---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P162, generic i64 string indexOf method acceptance
Related:
  - docs/development/current/main/phases/phase-29cv/P161-STAGE1-EMIT-PROGRAM-USING-MERGE-PRUNE.md
  - docs/development/current/main/phases/phase-29cv/P144-GLOBAL-CALL-STRING-SCAN-I64-BODY.md
  - docs/development/current/main/phases/phase-29cv/P93-LOWERING-PLAN-STRINGINDEXOF-DIRECTABI-CONSUME.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P162: Generic I64 String IndexOf Method

## Problem

P161 advanced source execution to the Program(JSON v0) output validator:

```text
target_shape_blocker_symbol=Stage1ProgramResultValidationBox.finalize_emit_result/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The unsupported operation was the validator's marker check:

```text
RuntimeDataBox.indexOf("\"version\":0")
RuntimeDataBox.indexOf("\"kind\":\"Program\"")
```

The MIR route vocabulary already owns `generic_method.indexOf` /
`StringIndexOf`. The remaining gap was that same-module `generic_i64_body`
classification and the module generic function emitter did not consume that
route in the source-execution capsule.

## Decision

Accept the one-argument string `indexOf` surface in `generic_i64_body` only:

- `RuntimeDataBox.indexOf(string)` on a receiver already classified as string
- `StringBox.indexOf(string)` on a receiver already classified as string
- return shape is scalar i64

Backend emission remains metadata-owned. The module generic emitter may emit
`nyash.string.indexOf_hh` only when the LoweringPlan entry proves:

```text
source_route_id=generic_method.indexOf
core_op=StringIndexOf
route_kind=string_indexof
symbol=nyash.string.indexOf_hh
route_proof=indexof_surface_policy
receiver_origin_box=StringBox
return_shape=scalar_i64
value_demand=scalar_i64
publication_policy=no_publication
tier=DirectAbi
```

## Non-Goals

- no `ArrayBox.indexOf` change
- no two-argument `indexOf(search, start)` change
- no backend raw method-name fallback
- no `.hako` workaround in the Stage1 validator

## Evidence

The route plan now also proves `RuntimeDataBox.indexOf` when the receiver is a
copy/PHI of a same-module generic pure string global call. This mirrors the
existing `substring` receiver proof and keeps the C emitter as a consumer of
MIR-owned metadata.

For `stage1_cli_env.hako`, the emit-program dispatcher now classifies the
validator call as:

```text
Stage1ProgramResultValidationBox.finalize_emit_result/1
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64
```

Inside the validator, both Program(JSON v0) marker checks carry:

```text
generic_method.indexOf
core_op=StringIndexOf
route_kind=string_indexof
symbol=nyash.string.indexOf_hh
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_indexof_generic_i64_body --lib
cargo test -q records_runtime_data_indexof_from_generic_global_call_phi_origin --lib
cargo test -q generic_i64 --lib
cargo test -q generic_method_route_plan --lib
cargo fmt --check
cargo build -q --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p162_string_indexof_method.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p162_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is accepted as an advance-to-next-backend
diagnostic probe. It no longer reports the P161 validator blocker; it now stops
later in the backend with `unsupported_pure_shape/no_lowering_variant`.
