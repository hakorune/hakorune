---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Bool) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P333A-RETURN-VAR-LOCAL-TEXT-SCALAR-EMIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P334A: Return Bool Text Scalar Emit

## Problem

After P333A, generated source-execution `emit-mir` handles the existing
`return.var.local` bucket. The next small existing owner bucket is:

```text
static box Main { main() { return true } }
```

Observed:

```text
rc=96
[mirbuilder/internal/unsupported] supported: Return(Int|Binary(Int,Int)|Call), Expr(Call env.console.log Int)+Return(Int), If-nested, and probe nested-ternary
[builder/selfhost-first:unsupported:no_match]
```

The generated Program(JSON v0) is:

```text
{"body":[{"expr":{"type":"Bool","value":true},"type":"Return"}],...}
```

`LowerReturnBoolBox` already owns `Return(Bool)` and `return.bool` is already
registered in `PatternRegistryBox`, but the source-execution minimal entry does
not dispatch to it. The owner also expects the older type-before-expr order.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

Do not widen Stage0 with bool-specific body semantics or a body-specific
emitter.

This card reuses the existing `LowerReturnBoolBox` owner and keeps the MIR JSON
shape unchanged.

## Implementation

- make `LowerReturnBoolBox` tolerate expr-before-type `Return(Bool)`
- read the `true` / `false` payload as owner-local literal text
- add the existing `return.bool` owner to `MirBuilderMinBox`

## Acceptance

Regenerate the source-execution EXE:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p334a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Run the generated EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/hakorune_p334a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "value":1
stdout is not a MirBuilder unsupported message
```

## Result

Implemented inside the existing owner path:

- `LowerReturnBoolBox` now accepts expr-before-type `Return(Bool)`
- `true` / `false` are read as owner-local literal text and emitted as i64
  `1` / `0`
- `MirBuilderMinBox` now dispatches to the existing `return.bool` owner

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p334a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/hakorune_p334a.exe
-> rc=0, stdout contains "value":1
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return false } }' \
  /tmp/hakorune_p334a.exe
-> rc=0, stdout contains "value":0
```

Regression checks:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p334a.exe
-> rc=0, stdout contains "value":5
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 7 } }' \
  /tmp/hakorune_p334a.exe
-> rc=0, stdout contains "value":7
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p334a.exe
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
