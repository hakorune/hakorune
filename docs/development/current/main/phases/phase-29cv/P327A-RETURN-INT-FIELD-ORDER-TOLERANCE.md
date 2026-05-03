---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Int) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P326A-GLOBAL-PRINT-STRING-HANDLE-MARSHAL.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P327A: Return Int Field Order Tolerance

## Problem

After P326A, the generated `stage1_cli_env.hako` EXE prints Program(JSON v0)
for `emit-program`. The next `emit-mir` run reaches the `.hako` MirBuilder and
fails on a simple source:

```text
static box Main { main() { return 0 } }
```

Observed runtime output:

```text
[mirbuilder/internal/unsupported] supported: Return(Int|Binary(Int,Int)|Call), Expr(Call env.console.log Int)+Return(Int), If-nested, and probe nested-ternary
[builder/selfhost-first:unsupported:no_match]
[freeze:contract][stage1-cli/emit-mir] MirBuilderBox.emit_from_source_v0 returned null
```

The emitted Program(JSON v0) body is:

```text
[{"expr":{"type":"Int","value":0},"type":"Return"}]
```

`LowerReturnIntBox.try_lower` already owns the `Return(Int)` shape, but it
assumes `"expr":{...}` appears after `"type":"Return"`. The current Program(JSON)
writer emits `expr` before `type`, so the existing accepted shape is missed.

After fixing the field order, the same DirectAbi path exposed a second
owner-local issue: `read_int_after(...)` can legitimately return `0`, and the
existing `val == null` guard treated that zero as null on the Stage0 route.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

This is a source-side field-order tolerance fix inside the existing
`Return(Int)` lowerer.

## Implementation

Teach `LowerReturnIntBox` to find the `expr` object either after the
`"type":"Return"` marker or as the nearest preceding `"expr":{` in the same
small return object window.

Keep `0` as a valid integer payload by converting it to text explicitly before
MIR string assembly.

The emitted MIR JSON is unchanged.

## Acceptance

Run the generated source-execution EXE in `emit-mir` mode:

```text
NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return 0 } }' \
  /tmp/hakorune_p326a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout is not a MirBuilder unsupported message
```

## Result

`Return(Int)` now lowers the expr-before-type Program(JSON) body and preserves
literal zero:

```text
HAKO_MIR_BUILDER_METHODIZE=0 ... NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 0 } }' /tmp/hakorune_p327a.exe
```

Observed:

```text
rc=0
stdout={"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"ret","value":1}]}]}]}
```

The normal methodize-on source-execution contract now advances to the next
blocker:

```text
target: CallMethodizeBox.methodize_calls_in_mir/1
reason: no-call MIR should be identity, but DirectAbi output fails promoted MIR
        validation with promote_missing_functions
```
