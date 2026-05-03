---
Status: Done
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P288a, JsonFrag decode-escapes length route
Related:
  - docs/development/current/main/phases/phase-29cv/P287A-GENERIC-TYPE-INVENTORY-CAPACITY.md
  - lang/src/shared/json/utils/json_frag.hako
---

# P288a: JsonFrag Decode-Escapes Length Route

## Problem

After P287a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
```

`JsonFragBox._decode_escapes/1` is already classified as a module-generic
string body. Its module prepass fails at one method call that has no
`LoweringPlan` row:

```text
site=b4587.i4
callee=StringBox.length
args=[self]
```

The same function already has the existing text-surface length route for
`RuntimeDataBox.length`:

```text
site=b4582.i7
route_kind=string_len
core_op=StringLen
```

This is not a reason to widen the C shim to rediscover method semantics. The
owner source should keep the text materialization explicit so MIR-owned
LoweringPlan facts remain the single route source.

## Decision

Materialize the decoded temporary through the existing text surface before
taking its length:

```hako
local n = ("" + s1).length()
```

This keeps Stage0 small:

- C shim reads `LoweringPlan`; it does not infer a missing `StringBox.length`.
- No new body shape is added.
- No generic collection or method semantics are widened.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C shim fallback for missing `StringBox.length` plan rows
- no body-specific emitter
- no JSON escape semantic change
- no fallback or compat route behavior change

## Acceptance

- `JsonFragBox._decode_escapes/1` no longer fails module-generic prepass on the
  missing `StringBox.length(self)` route.
- The source-execution probe advances to the next blocker or produces the exe.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done. The source-execution probe no longer fails module-generic prepass on
`JsonFragBox._decode_escapes/1`; that function now reaches DirectAbi use sites.

The probe advances to the next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonNumberCanonicalBox.canonicalize_f64/1
target_shape_blocker_reason=-
```
