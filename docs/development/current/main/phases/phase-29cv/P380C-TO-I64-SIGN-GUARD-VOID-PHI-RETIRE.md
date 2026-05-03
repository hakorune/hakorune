---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380C, remove StringHelpers.to_i64 sign-guard void PHI source
Related:
  - docs/development/current/main/phases/phase-29cv/P380B-TO-I64-FORCE-ZERO-ENV-RETIRE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/src/shared/common/string_helpers.hako
---

# P380C: to_i64 Sign-Guard Void PHI Retire

## Problem

After P380B, `StringHelpers.to_i64/1` has no active debug/env residue, but the
pure-first route still reports:

```text
target_shape_blocker_symbol=StringHelpers.to_i64/1
target_shape_blocker_reason=generic_string_return_not_string
```

Inventory of the generated MIR shows that the sign guard:

```hako
if s.substring(0,1) == "-" { neg = 1  i = 1 }
```

creates a dead hidden PHI whose destination is typed `i64` but one incoming edge
is `void`:

```text
b246 dst=60 dst_ty=i64 inputs=%54=i64@b244,%59=void@b245
```

The value is unused, but teaching Stage0/generic_i64 to ignore mixed scalar/void
dead PHIs would move a source-owner artifact into the Stage0 classifier.

## Decision

Rewrite the sign guard so both branches assign scalar values explicitly:

```hako
if first == "-" { neg = 1 } else { neg = 0 }
if neg == 1 { i = 1 } else { i = 0 }
```

This keeps the parser semantics unchanged while making the source contract
explicit: the sign path produces scalar state only, not an implicit `void`
sentinel that Stage0 must understand.

## Non-Goals

- Do not add a new `GlobalCallTargetShape`.
- Do not broaden generic string/i64 body acceptance.
- Do not add ny-llvmc body-specific emitter logic.
- Do not change numeric parsing behavior.

## Acceptance

```bash
cargo test --release generic_i64 --lib
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Advance-to-next-blocker probe:

```bash
target/release/hakorune --emit-mir-json /tmp/p380c_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako

jq -r '.functions[] | select(.name=="StringHelpers.to_i64/1") as $f |
  ($f.metadata.value_types // {}) as $types |
  $f.blocks[] | . as $b |
  ($b.instructions[]?, $b.terminator?) |
  select(.op=="phi") |
  select((.dst_type // ($types[(.dst|tostring)] // null)) == "i64") |
  select(((.incoming // []) | map($types[(.[0]|tostring)] // "unknown") |
    any(. == "void"))) |
  "b\($b.id) dst=\(.dst)"' /tmp/p380c_stage1_cli_env.mir.json
```

Expected reading: the query prints no `StringHelpers.to_i64/1` scalar PHI with a
`void` incoming edge. Any remaining pure-first stop is the next route blocker,
not the sign-guard hidden PHI.

## Result

The sign guard now emits scalar-only branch state:

```hako
local first = s.substring(0,1)
if first == "-" { neg = 1 } else { neg = 0 }
if neg == 1 { i = 1 } else { i = 0 }
```

The MIR probe prints no `StringHelpers.to_i64/1` scalar PHI with a `void`
incoming edge.

The pure-first EXE route advances past `StringHelpers.to_i64/1`. The next
blocker is now:

```text
[llvm-pure/unsupported-shape] recipe=pure-first first_block=14165 first_inst=2 first_op=mir_call owner_hint=backend_lowering reason=missing_multi_function_emitter target_return_type=i64 target_shape_reason=generic_string_global_target_shape_unknown target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5 target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
