---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P320a, StringOps.index_of_from explicit text contract
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P319A-STAGE1-MIR-DEBUG-STATE-SHRINK.md
  - lang/src/shared/common/string_ops.hako
---

# P320a: StringOps Index Of From Text Contract

## Problem

P319a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=StringOps.index_of_from/3
target_shape_blocker_reason=-
```

`StringOps.index_of_from/3` is a common string utility, but its `text` and
`needle` parameters are untyped at the MIR boundary.  The body performs
`length()` and final `indexOf()` directly on those parameters, so the final
`RuntimeDataBox.indexOf` call can lose the string receiver/argument facts.

## Decision

Make the helper contract explicit after the existing null checks:

```hako
local s = "" + text
local p = "" + needle
```

Then all length and index operations use `s` / `p`.  This keeps the string
contract in the owner utility and avoids widening generic `RuntimeDataBox`
method acceptance for arbitrary unknown receivers.

## Non-Goals

- no generic unknown-receiver `indexOf` widening
- no new `GlobalCallTargetShape`
- no C body-specific emitter for `StringOps`
- no change to null handling or return sentinel policy

## Acceptance

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p320_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p320.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
StringOps.index_of_from/3 no longer blocks module_generic_prepass_failed.
```

## Result

Accepted. `StringOps.index_of_from/3` now makes the string contract explicit for
both `text` and `needle` after preserving the existing null sentinels.  Direct
MIR now emits the exact `generic_method.indexOf` route for the final
`s.indexOf(p, pos)` call.

Validation:

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p320_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p320.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `StringOps.index_of_from/3` to a later lowering variant
failure:

```text
stage=mem2reg result=fail reason=opt
reason=no_lowering_variant
target_shape_blocker_symbol=-
```
