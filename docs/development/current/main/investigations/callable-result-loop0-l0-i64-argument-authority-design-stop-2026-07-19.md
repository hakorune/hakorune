# LOOP0-L0 selected i64 argument authority — design stop

Status: **design stop**. `LOOP0-L0` must not add a local type inference or
representation backfill.

## Observed boundary

The first located GenericLoop fixture reaches the selected call:

```text
ParserStringUtilsBox.skip_ws(text, me.static_const_eval_pos(rhs))
```

`skip_ws/2` is selected with required exact-i64 argument ordinal `1`. The
nested `me.static_const_eval_pos(rhs)` CorePlan call result is `%27` with
`MirType::Unknown`, so the selected emission terminal correctly fails with:

```text
[freeze:contract][callable_result/selected_call_required_i64]
```

The session is poisoned and no selected Call/result is published. This is the
required fail-fast behavior; it is not an L0 acceptance result.

## Why LOOP0-L0 cannot repair it

The selected activation claim owns the canonical target and required argument
ordinals. It deliberately owns neither caller-side `ValueId` identity nor a
caller argument representation fact. Writing `Integer` into `type_ctx` from
that claim would therefore create a second source-to-ValueId/type authority.

The raw route accepts this nested method result dynamically. Inferring i64 from
`static_const_eval_pos`, source spelling, runtime class, finalized metadata,
or the selected callee contract is forbidden. The existing selected terminal
must remain a read-only exact-Integer check.

## Fixed L0 boundary

```text
LOOP0-L0 may:
  preserve the selected terminal fail-fast and its poisoned-session fixture

LOOP0-L0 may not:
  publish a type for the nested result
  widen the exact-i64 ABI to dynamic arguments
  use a name/runtime/final-metadata fallback
  retry raw or alternate lowering
```

The actual 15-row success fixture remains required for L0 closeout. A
failure-only fixture cannot close L0.

## Next decision required

Choose one durable owner before resuming L0:

1. Tighten activation selection so a source call lacking a proven required-i64
   argument is `Unselected`.
2. Introduce a separately sealed nested-call result representation proof and a
   source-site-to-final-lowered-ValueId witness, then make that proof the sole
   producer of the exact argument type.

Dynamic ABI admission is rejected: it contradicts the selected exact-i64 first
profile. The decision must also preserve the existing rule that activation
claims store no `ValueId`, `MirType`, Builder state, or retry authority.

## Evidence and WIP

The disconnected L0 experiment is retained only as evidence:

```text
stash@{0}: wip/loop0-l0-selected-i64-argument-authority (design stop)
```

Do not restore it as an authority. A later selected design must reimplement the
minimal approved slice from a clean tree.

