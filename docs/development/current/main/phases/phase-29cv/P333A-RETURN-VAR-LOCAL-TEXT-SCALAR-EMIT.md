---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Local(Int) + Return(Var) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P332A-RETURN-INT-NONZERO-TEXT-SCALAR-EMIT.md
  - docs/development/current/main/phases/phase-29cv/P331A-RETURN-BINOP-INTINT-TEXT-SCALAR-EMIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P333A: Return Var Local Text Scalar Emit

## Problem

After P332A, generated source-execution `emit-mir` handles `Return(Int)` and
`Return(Binary(Int,Int))`. The next small existing MirBuilder owner bucket is:

```text
static box Main { main() { local x = 5 return x } }
```

Observed:

```text
rc=96
[mirbuilder/internal/unsupported] supported: Return(Int|Binary(Int,Int)|Call), Expr(Call env.console.log Int)+Return(Int), If-nested, and probe nested-ternary
[builder/selfhost-first:unsupported:no_match]
```

The generated Program(JSON v0) is:

```text
{"body":[{"expr":{"type":"Int","value":5},"name":"x","type":"Local"},{"expr":{"name":"x","type":"Var"},"type":"Return"}],"kind":"Program",...}
```

`LowerReturnVarLocalBox` already owns `Local(Int) -> Return(Var)`, and
`return.var.local` is already present in the registry authority. However the
source-execution minimal entry does not call that existing owner, and the owner
itself assumes the older type-before-name/expr field order plus integer
round-tripping through `JsonFragBox.read_int_after(...)`.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

Do not widen Stage0 with local/variable semantics or a body-specific emitter.

This card reuses the existing `LowerReturnVarLocalBox` owner and keeps the MIR
JSON shape unchanged.

## Implementation

- make `LowerReturnVarLocalBox` tolerate the current Local object
  `expr/name/type` field order
- read the local integer payload as owner-local scalar digit text
- add the existing `return.var.local` owner to the source-execution minimal
  `MirBuilderMinBox` dispatch surface

## Acceptance

Regenerate the source-execution EXE:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p333a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Run the generated EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p333a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "value":5
stdout is not a MirBuilder unsupported message
```

## Result

Implemented inside the existing owner path:

- `LowerReturnVarLocalBox` now accepts the current Local object
  `expr/name/type` field order while preserving the older type-first shape
- the Local(Int) payload is emitted as owner-local scalar digit text
- `MirBuilderMinBox` now dispatches to the existing `return.var.local` owner
  instead of treating this already-registered bucket as unsupported

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p333a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p333a.exe
-> rc=0, stdout contains "value":5
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = -5 return x } }' \
  /tmp/hakorune_p333a.exe
-> rc=0, stdout contains "value":-5
```

Regression checks:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 7 } }' \
  /tmp/hakorune_p333a.exe
-> rc=0, stdout contains "value":7
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p333a.exe
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
