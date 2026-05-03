---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir methodize-on no-call MIR identity
Related:
  - docs/development/current/main/phases/phase-29cv/P327A-RETURN-INT-FIELD-ORDER-TOLERANCE.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P328A: Methodize No-Call Identity

## Problem

P327A makes the existing `Return(Int)` lowerer accept the current
Program(JSON) body shape and preserve literal zero. With methodization disabled,
the generated source-execution EXE now emits the expected MIR JSON for:

```text
static box Main { main() { return 0 } }
```

The normal source-execution contract keeps `HAKO_MIR_BUILDER_METHODIZE=1`.
Under that route, the same no-call MIR fails after the `Return(Int)` lowerer:

```text
[builder/selfhost-first:unsupported:promote_missing_functions]
```

`CallMethodizeBox.methodize_calls_in_mir/1` is a call-rewrite pass. A MIR body
with no `"op":"call"` instructions should be an identity input. The current
DirectAbi route rebuilds the string through the scanner even when there is
nothing to rewrite, and the promoted MIR validation sees a result without the
canonical `"functions":` key.

## Boundary

Do not add a new MirBuilder pattern.

Do not change `Return(Int)` lowering again.

Do not widen methodization semantics.

This is a no-call identity fix inside the existing methodize pass.

## Implementation

Add an early no-call scan in `CallMethodizeBox.methodize_calls_in_mir/1`:

```text
if input has no "op":"call" marker:
  return input text unchanged
```

The existing rewrite loop remains the owner for actual call methodization.

## Acceptance

Run the generated source-execution EXE in normal methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 0 } }' \
  /tmp/hakorune_p328a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "value":0
stdout is not a promote_missing_functions diagnostic
```

## Result

Observed on `/tmp/hakorune_p328a.exe`:

```text
rc=0
stdout={"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"ret","value":1}]}]}]}
```

`promote_missing_functions` is not emitted for the no-call MIR path.
