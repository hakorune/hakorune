---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: `phase2043` Program(JSON)->MirBuilder canary から raw CLI fallback を削る。
Related:
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - tools/smokes/v2/profiles/integration/core/phase2043/program_new_array_delegate_struct_canary_vm.sh
---

# P10 Phase2043 Delegate Smoke Fallback Retire

## Goal

`tools/smokes/v2/profiles/integration/core/phase2043/program_new_array_delegate_struct_canary_vm.sh`
は `.hako MirBuilder` canary だが、失敗時に raw
`--program-json-to-mir` CLI fallback へ落ちていた。

P8 で shared helper fallback を削ったので、smoke 側の同種 fallback も
1 caller family ずつ削る。

## Decision

- `.hako MirBuilder` が MIR(JSON) を出せない current env では明示 SKIP。
- raw `--program-json-to-mir` fallback は使わない。
- `--program-json-to-mir` CLI 本体は削除しない。selfhost EXE / dev proof /
  shared test helper buckets がまだ live。

This avoids a false PASS: the old smoke could pass through raw Rust CLI fallback
even when the `.hako MirBuilder` path was not active.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/core/phase2043/program_new_array_delegate_struct_canary_vm.sh
rg -n -- '--program-json-to-mir' tools/smokes/v2/profiles/integration/core/phase2043/program_new_array_delegate_struct_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```
