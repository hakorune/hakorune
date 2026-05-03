---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Stage1 contract raw Program(JSON) helper source narrowing
Related:
  - tools/selfhost/lib/stage1_contract.sh
  - tools/lib/program_json_v0_compat.sh
  - docs/development/current/main/phases/phase-29cv/P64-STAGE1-CONTRACT-CALLER-OWNERSHIP-SPLIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P343A: Stage1 Contract Lazy Program(JSON) Compat Source

## Problem

`tools/selfhost/lib/stage1_contract.sh` sourced
`tools/lib/program_json_v0_compat.sh` at load time even for callers that only
needed `emit-mir`, `run`, artifact metadata, or route validation helpers.

That kept the raw `--emit-program-json-v0` spelling helper present on
non-Program(JSON) paths and made the remaining keeper surface look wider than
the actual live need.

## Boundary

Do not delete `tools/lib/program_json_v0_compat.sh`.

Do not change the `stage1_contract_exec_direct_emit_mode ... emit-program`
behavior.

Do not change Stage1 wrapper behavior, fixture helper behavior, or Rust/public
delete-last surfaces.

This is a BoxShape cleanup only: the helper source point moves from contract
load time to the direct `emit-program` branch that needs it.

## Implementation

- add `stage1_contract_source_program_json_v0_compat()`
- source `program_json_v0_compat.sh` only when the direct `emit-program` branch
  calls `program_json_v0_compat_emit_to_file()`
- keep the function idempotent through `declare -F`

## Acceptance

```text
bash -n tools/selfhost/lib/stage1_contract.sh
-> ok
```

```text
bash -c 'source tools/selfhost/lib/stage1_contract.sh; ! declare -F program_json_v0_compat_emit_to_file >/dev/null'
-> ok
```

```text
bash -c 'source tools/selfhost/lib/stage1_contract.sh; stage1_contract_source_program_json_v0_compat; declare -F program_json_v0_compat_emit_to_file >/dev/null'
-> ok
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
