---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Str.length()) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P336A-RETURN-STR-DIRECT-CHILD-GUARD.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P337A: Return Str Length Method Emit

## Problem

P336A stopped `return.string` from silently accepting nested receiver strings:

```text
static box Main { main() { return "hello".length() } }
```

The shape now fail-fasts with `no_match`, which is correct until the proper
owner handles it.

The generated Program(JSON v0) is:

```text
{"expr":{"args":[],"method":"length","recv":{"type":"Str","value":"hello"},"type":"Method"},"type":"Return"}
```

`LowerReturnMethodStringLengthBox` already owns string length returns and
`return.method.string.length` is registered in `PatternRegistryBox`, but the
owner expects the older `New(StringBox, String)` receiver shape and the
methodize pass treats every `"op":"call"` as a legacy `func`-register call.

## Boundary

Do not add a new MirBuilder pattern.

Do not route nested method calls through `return.string`.

Do not add fallback behavior.

Do not widen Stage0 with arbitrary method-call semantics.

This card reuses the existing `LowerReturnMethodStringLengthBox` owner for the
current parser's direct `Str` receiver shape.

## Implementation

- make `LowerReturnMethodStringLengthBox` accept
  `Return(Method(recv=Str|String, method=length|size, args=[]))`
- preserve the receiver string as a raw JSON string literal slice
- keep `CallMethodizeBox` as an identity pass for already-canonical call
  instructions that have no legacy `"func":` register field

## Acceptance

Regenerate the source-execution EXE:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p337a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Run the generated EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' \
  /tmp/hakorune_p337a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"kind":"MIR" or {"functions"
stdout contains StringBox length/Method call metadata
stdout is not a const-string-only Return(Str) MIR
stdout is not a MirBuilder unsupported message
```

## Result

Implemented on the existing owner path:

- `LowerReturnMethodStringLengthBox` now accepts the current parser shape
  `Return(Method(recv=Str|String, method=length|size, args=[]))`
- receiver strings are preserved as raw JSON string literal slices
- emitted MIR uses the canonical `{"functions":[...]}` root
- `CallMethodizeBox.methodize_calls_in_mir/1` now returns input unchanged when
  calls are already canonical and no legacy `"func":` register field exists
- no `MirBuilderMinBox` dispatch was added, keeping the source-execution
  minimal entry from widening its function set

Validation:

```text
target/release/hakorune --emit-exe /tmp/hakorune_p337a.exe \
  lang/src/runner/stage1_cli_env.hako
-> rc=0
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' \
  /tmp/hakorune_p337a.exe
-> rc=0, stdout contains StringBox length Method call metadata
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".size() } }' \
  /tmp/hakorune_p337a.exe
-> rc=0, stdout contains StringBox size Method call metadata
```

Regression checks:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/hakorune_p337a.exe
-> rc=0, stdout contains "type":"string" and "value":"hello"
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/hakorune_p337a.exe
-> rc=0, stdout contains "value":1
```

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/hakorune_p337a.exe
-> rc=0, stdout contains "value":5
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
