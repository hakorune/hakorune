---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Return(Str) direct-child guard after P335A
Related:
  - docs/development/current/main/phases/phase-29cv/P335A-RETURN-STR-TEXT-EMIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P336A: Return Str Direct Child Guard

## Problem

P335A correctly routes direct string literal returns:

```text
static box Main { main() { return "hello" } }
```

but the first follow-up probe exposed a silent wrong acceptance:

```text
static box Main { main() { return "hello".length() } }
```

Observed after P335A:

```text
rc=0
stdout emits const string "hello" + ret
```

The generated Program(JSON v0) is:

```text
{"expr":{"args":[],"method":"length","recv":{"type":"Str","value":"hello"},"type":"Method"},"type":"Return"}
```

`LowerReturnStringBox` accepted the nested receiver `Str` under
`Return(Method(...))` instead of requiring the return expression itself to be a
direct `Str` / `String` literal.

## Boundary

Do not add a new MirBuilder pattern.

Do not implement string method lowering here.

Do not change Program(JSON) field ordering.

Do not add fallback behavior.

This card only tightens `LowerReturnStringBox` to direct-child string returns.

## Implementation

Require the Return expression root to be exactly one of:

```text
"expr":{"type":"Str"
"expr":{"type":"String"
```

for both type-before-expr and expr-before-type Program(JSON) order.

Nested receiver strings under `Return(Method(...))` must be rejected so the
proper `return.method.string.length` owner can handle or reject the shape.

## Acceptance

Regenerate the source-execution EXE:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p336a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Positive:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/hakorune_p336a.exe
-> rc=0, stdout contains "type":"string" and "value":"hello"
```

Negative guard:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' \
  /tmp/hakorune_p336a.exe
-> must not emit const string "hello" as a successful Return(Str) MIR
```

## Result

Implemented in `LowerReturnStringBox`:

- direct expr-before-type acceptance now checks the exact root prefix
  `"expr":{"type":"Str"` or `"expr":{"type":"String"`
- nested receiver strings under `Return(Method(...))` are rejected by
  `return.string`
- fixed the guard implementation to use fixed token lengths so the
  source-execution generic body does not need extra local-string `.length()`
  semantics

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p336a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/hakorune_p336a.exe
-> rc=0, stdout contains "type":"string" and "value":"hello"
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' \
  /tmp/hakorune_p336a.exe
-> rc=96, no const-string success MIR emitted by return.string
```

Regression checks:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/hakorune_p336a.exe
-> rc=0, stdout contains "value":1
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p336a.exe
-> rc=0, stdout contains "value":5
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p336a.exe
-> rc=0, stdout contains "op":"binop" and "operation":"+"
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
