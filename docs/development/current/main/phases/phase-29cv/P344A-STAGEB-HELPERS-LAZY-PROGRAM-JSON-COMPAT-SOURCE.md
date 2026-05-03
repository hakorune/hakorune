---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Stage-B fixture helper raw Program(JSON) source narrowing
Related:
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/lib/program_json_v0_compat.sh
  - docs/development/current/main/phases/phase-29cv/P65-FIXTURE-CALLER-OWNERSHIP-SPLIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P344A: Stage-B Helpers Lazy Program(JSON) Compat Source

## Problem

`tools/smokes/v2/lib/stageb_helpers.sh` sourced
`tools/lib/program_json_v0_compat.sh` at load time even for callers that only
needed Stage-B stdout capture, environment setup, or compile helpers.

The raw `--emit-program-json-v0` shell spelling should stay behind the exact
fixture emit function that still needs it.

## Boundary

Do not delete `tools/lib/program_json_v0_compat.sh`.

Do not change `stageb_emit_program_json_v0_fixture()` behavior.

Do not change Stage-B stdout capture, fixture callers, or Rust/public
delete-last surfaces.

This is a BoxShape cleanup only: the helper source point moves from load time
to the fixture emit function.

## Implementation

- add `stageb_source_program_json_v0_compat()`
- source `program_json_v0_compat.sh` only when
  `stageb_emit_program_json_v0_fixture()` calls the raw emit helper
- keep the function idempotent through `declare -F`

## Acceptance

```text
bash -n tools/smokes/v2/lib/stageb_helpers.sh
-> ok
```

```text
bash -c 'source tools/smokes/v2/lib/stageb_helpers.sh; ! declare -F program_json_v0_compat_emit_to_file >/dev/null'
-> ok
```

```text
bash -c 'source tools/smokes/v2/lib/stageb_helpers.sh; stageb_source_program_json_v0_compat; declare -F program_json_v0_compat_emit_to_file >/dev/null'
-> ok
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
