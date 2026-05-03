---
Status: Blocked
Decision: deferred
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Binary Int,Int) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P327A-RETURN-INT-FIELD-ORDER-TOLERANCE.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P330A: Return Binary IntInt Field Order Stop-Line

## Problem

After P328A, methodize-on source-execution emits MIR for `return 0`.
The next simple bucket fails:

```text
static box Main { main() { return 1 + 2 } }
```

Observed `emit-program` Program(JSON v0):

```text
{"body":[{"expr":{"lhs":{"type":"Int","value":1},"op":"+","rhs":{"type":"Int","value":2},"type":"Binary"},"type":"Return"}],"kind":"Program","user_box_decls":[{"field_decls":[],"fields":[],"name":"Main","type_parameters":[]}],"version":0}
```

`LowerReturnBinOpBox.try_lower` already owns `Return(Binary(Int,Int))`, but it
searches `"type":"Binary"` only after the `"type":"Return"` marker. The current
Program(JSON) writer emits `expr` and the nested Binary object before the Return
type marker.

The first blocker is therefore field-order tolerance. A direct implementation
attempt exposed a deeper owner-side text assembly seam: once the Binary object is
found, the returned MIR text is truncated around dynamic string concatenation
points before `CallMethodizeBox` sees valid JSON.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

Do not widen `generic_string_body` / Stage0 collection or dynamic-string
semantics to rescue this body.

## Investigation

Attempted source-local field-order tolerance in `LowerReturnBinOpBox`:

```text
k_bin = nearest preceding "expr":{... "type":"Binary" ...} for the Return object
```

That made the lowerer hit, but the returned MIR text became truncated:

```text
","lhs":1,"rhs":2,"dst":3},{"op":"ret","value":3}]}]}]}
```

Changing the dynamic `op` insertion to static operator branches moved, but did
not remove, the truncation:

```text
}},{"op":"binop","operation":"+","lhs":1,"rhs":2,"dst":3},{"op":"ret","value":3}]}]}]}
```

Trying to append raw scalar operands widened the required source-execution shape
and failed earlier during EXE generation:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

## Decision

Do not commit the failing `LowerReturnBinOpBox` code path.

The next safe seam is not a Stage0/classifier widening. The next implementation
should split Return(Binary) MIR emission so integer-to-text and JSON assembly are
owned by a small source-side text/scalar contract that existing source-execution
already accepts, or reuse an existing owner-local text builder with the same
contract.

The emitted MIR JSON shape should remain unchanged.

## Acceptance

Run the generated source-execution EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p330a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "op":"binop"
stdout is not a MirBuilder unsupported message
```

Current status: blocked before code commit. This card records the stop-line so
the next card can address the text assembly seam without adding Stage0 fallback
semantics.
