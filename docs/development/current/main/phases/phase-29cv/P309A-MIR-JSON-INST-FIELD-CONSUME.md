---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P309a, MIR JSON instruction field fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P219A-MIR-JSON-INST-FIELD-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P219B-MIR-JSON-INST-NULL-GUARD-CLEANUP.md
  - docs/development/current/main/phases/phase-29cv/P308A-LOWER-LOOP-COUNT-PARAM-FINISH-SPLIT.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P309a: MIR JSON Instruction Field Consume

## Problem

P308a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_inst/1
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_inst/1` already classifies as:

```text
tier=DirectAbi
proof=typed_global_call_generic_pure_string
target_shape=generic_pure_string_body
```

Its `inst.get(<static key>)` reads also already have exact MIR-owned
`generic_method.get` rows:

```text
route_proof=mir_json_inst_field
route_kind=runtime_data_load_any
symbol=nyash.runtime_data.get_hh
```

The C module generic string emitter does not yet consume that exact route row.

## Decision

Consume only the exact `mir_json_inst_field` MapGet row in the module generic
string emitter.

Classify static instruction field keys narrowly for C prepass:

```text
op / operation / op_kind / cmp / value -> string-like handle
args / effects -> array handle
other instruction fields -> runtime data map get handle/scalar
```

This mirrors the existing MIR-owned proof and does not accept arbitrary
`RuntimeDataBox.get/1`.

## Non-Goals

- no generic `RuntimeDataBox.get/1` widening
- no new `GlobalCallTargetShape`
- no `_emit_inst/1` body-specific emitter
- no MIR instruction schema change
- no new `.hako` workaround

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p309a_inst_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._emit_inst/1 no longer fails module_generic_prepass_failed on
mir_json_inst_field reads.
```

## Result

Accepted.

The C module generic string emitter now consumes the exact
`mir_json_inst_field` MapGet rows. `_emit_inst/1` no longer fails module generic
prepass on static instruction field reads.

The probe advances to the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_module_rec/3
target_shape_blocker_reason=-
```
