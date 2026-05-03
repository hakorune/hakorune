---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Str) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P334A-RETURN-BOOL-TEXT-SCALAR-EMIT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P335A: Return Str Text Emit

## Problem

After P334A, generated source-execution `emit-mir` handles the existing
`Return(Bool)` bucket. The next small existing owner bucket is:

```text
static box Main { main() { return "hello" } }
```

Observed:

```text
rc=96
[builder/selfhost-first:unsupported:no_match]
```

The generated Program(JSON v0) is:

```text
{"body":[{"expr":{"type":"Str","value":"hello"},"type":"Return"}],...}
```

`LowerReturnStringBox` already owns string literal returns and `return.string`
is registered in `PatternRegistryBox`, but the owner expects the older
`"type":"String"` spelling and type-before-expr field order. The
source-execution minimal entry also does not dispatch to this existing owner.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering or rename `Str`.

Do not add a delegate fallback.

Do not widen Stage0 with string-literal body semantics or a body-specific
emitter.

This card reuses the existing `LowerReturnStringBox` owner and keeps the MIR
JSON shape unchanged.

## Implementation

- make `LowerReturnStringBox` accept `Str` and `String`
- make the owner tolerate expr-before-type `Return(Str)`
- add the existing `return.string` owner to `MirBuilderMinBox`

## Acceptance

Regenerate the source-execution EXE:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p335a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Run the generated EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/hakorune_p335a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "type":"string"
stdout contains "value":"hello"
stdout is not a MirBuilder unsupported message
```

## Result

Implemented inside the existing owner path:

- `LowerReturnStringBox` now accepts `Str` and `String`
- the owner tolerates expr-before-type `Return(Str)`
- string payloads are preserved as raw JSON string literal slices from
  Program(JSON), avoiding a second escape/concat reconstruction path
- `MirBuilderMinBox` now dispatches to the existing `return.string` owner

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p335a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/hakorune_p335a.exe
-> rc=0, stdout contains "type":"string" and "value":"hello"
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "he\\\"llo" } }' \
  /tmp/hakorune_p335a.exe
-> rc=0, stdout preserves the escaped JSON literal
```

Regression checks:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/hakorune_p335a.exe
-> rc=0, stdout contains "value":1
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p335a.exe
-> rc=0, stdout contains "value":5
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p335a.exe
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
