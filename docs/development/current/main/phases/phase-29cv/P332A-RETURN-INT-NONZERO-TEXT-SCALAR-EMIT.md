---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Int) nonzero payload lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P327A-RETURN-INT-FIELD-ORDER-TOLERANCE.md
  - docs/development/current/main/phases/phase-29cv/P331A-RETURN-BINOP-INTINT-TEXT-SCALAR-EMIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P332A: Return Int Nonzero Text Scalar Emit

## Problem

P327A fixed field-order tolerance for `Return(Int)` and preserved literal zero,
but the generated source-execution EXE still emits nonzero integer payloads as
zero:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 7 } }' \
  /tmp/hakorune_p331a.exe
```

Observed:

```text
rc=0
"value":0
```

This is a silent correctness bug. The route reaches the existing
`LowerReturnIntBox` owner, but the payload crosses the Stage0 source-execution
path through `JsonFragBox.read_int_after(...)` and `StringHelpers.int_to_str(...)`
instead of staying as owner-local scalar text.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

Do not widen Stage0 with a new body-specific shape or emitter.

This card stays inside the existing `LowerReturnIntBox` owner.

## Implementation

Mirror the P331A scalar-text contract for the single integer payload:

```text
find "value": -> skip whitespace -> read optional sign -> read digits -> emit text
```

`0` remains valid, but nonzero values no longer round-trip through
`StringHelpers.int_to_str(...)`.

The emitted MIR JSON shape is unchanged.

## Acceptance

Run the generated source-execution EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 7 } }' \
  /tmp/hakorune_p332a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "value":7
stdout is not a MirBuilder unsupported message
```

## Result

Implemented inside `LowerReturnIntBox` only:

- removed the `StringHelpers.int_to_str(...)` round-trip from Return(Int) payload lowering
- added owner-local whitespace / optional sign / digit-text scanning for the existing `"value":` field
- preserved the emitted MIR JSON shape

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p332a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 7 } }' \
  /tmp/hakorune_p332a.exe
-> rc=0, stdout contains "value":7
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 0 } }' \
  /tmp/hakorune_p332a.exe
-> rc=0, stdout contains "value":0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return -7 } }' \
  /tmp/hakorune_p332a.exe
-> rc=0, stdout contains "value":-7
```

```text
SMOKES_ENABLE_SELFHOST=1 \
  bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
-> PASS
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
