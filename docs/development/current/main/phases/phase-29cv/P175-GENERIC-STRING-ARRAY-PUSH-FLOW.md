---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P175, generic string ArrayBox push flow
Related:
  - docs/development/current/main/phases/phase-29cv/P174-VOID-LOGGING-CHILD-WRAPPER.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P175: Generic String Array Push Flow

## Problem

After P174, the source-execution probe advanced to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The first unsupported method in the normalizer was `RuntimeDataBox.push` on an
`ArrayBox` builder carrier. `ArrayBox` length already used MIR-owned
`NewBox`/copy/all-array-PHI flow evidence, but `push` only recognized direct
receiver origin evidence. That duplicated the collection flow boundary and left
`generic_pure_string_body` unable to accept string pushes into local builder
arrays.

## Decision

Accept one additional method surface inside `generic_pure_string_body`:

- `ArrayBox.push(string)` / `RuntimeDataBox.push(string)`
- receiver proven as `ArrayBox` through existing array flow evidence
- pushed value already proven string
- matching `generic_method.push` / `ArrayPush` LoweringPlan entry present

The C module generic string emitter validates the same LoweringPlan contract
before lowering to `nyash.array.slot_append_hh`. This does not accept array
`get`, array `set`, map methods, non-string pushes, or mixed collection PHIs.

## Acceptance

```bash
cargo test -q records_runtime_data_arraybox_push_through_phi_flow_as_cold_core_method_route --lib
cargo test -q refresh_module_semantic_metadata_accepts_array_string_push_in_generic_pure_string_body --lib
cargo test -q generic_method_route_plan --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p175_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonFragNormalizerBox._normalize_instructions_array/1` now passes its
`ArrayBox` string push surface through `generic_pure_string_body` and the
LoweringPlan-backed C emitter.

The probe advances to:

```text
target_shape_blocker_symbol=JsonNumberCanonicalBox.read_num_token/2
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

Treat the numeric token sentinel handling as the next card. Do not fold it into
ArrayBox push acceptance.
