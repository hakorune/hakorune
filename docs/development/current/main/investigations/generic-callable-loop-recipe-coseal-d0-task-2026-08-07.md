# Callable single-loop Recipe co-seal D0

Status: `Decision: open design stop; implementation and production selection are not authorized`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-MAP-S1`

## Purpose

MAP-S1 is now a closed caller-zero source map. It co-seals the resolver-owned
callable ledger with the neutral `VerifiedSourceSyntaxFactsV1` product, but it
does not issue a portable Recipe, JoinSig, ValueId, CFG, PHI, After, or
physical input. This one shallow design stop fixes the common owner boundary
before any Recipe implementation begins.

The selected callable profile remains separate from the nested Generic G0
profile:

```text
StringHelpers.int_to_str/1
  prefix: local value = helper.to_i64(n)
  loop:   i < 1 { i = i + 1 }
  tail:   return value
```

No shape similarity may relabel this profile as Generic G0.

## Decision candidates to close

The design must choose one common, not callable-specific, product for:

```text
Recipe / JoinSig
LoopOperationSourceRelation
BindingSSA effect relation
After / Tail / completion envelope
resolver Loop source + frame + Scope/Region
```

The source map remains the only source-to-role input. It owns neutral source
roles and exact resolver evidence; the Recipe mapper owns numeric/operator
policy already admitted by MAP; the common Recipe owner issues logical keys;
the canonical SSA/CFG session remains the sole ValueId/PHI/physical owner;
completion/DraftSeal remains the sole terminal owner. No layer may mint a
second identity or re-pair rows by AST, name, path suffix, ordinal, or
`variable_map`.

The prefix and tail are whole-callable envelope obligations, not additional
Loop operations. The selected MethodCall prefix has no canonical callable
target in the resolver product, so its call boundary must remain an outer
callable-plan obligation until a separate callable-target authority is
accepted. `After` and tail must not be inferred from the Loop-only rows.

## Required design output

One compact table must close each source role to its common logical product,
effect/BindingSSA relation, and completion owner:

```text
InitialCarrier       -> carrier/entry relation
ConditionRead        -> condition read
ConditionBound       -> constant operand
ConditionOperator    -> compare operator
StepRead             -> recurrence read
StepDelta            -> recurrence constant
StepOperator         -> recurrence arithmetic
StepWrite            -> one exact rebind
PrefixBoundary       -> outer callable prelude
TailReturnRead       -> outer callable After/Tail
Loop source/frame    -> Scope/Region + physical session brand
```

The table must also define one sealed `JoinSig`/completion shape, one source
coverage key, and one exact-consumption rule for the later mapper. If the
common products cannot represent the profile without a second owner, the
outcome is `NoSafeSlice`; do not widen the mapper during implementation.

## Explicit non-claims at this stop

```text
Recipe producer             = 0
ValueId / PHI / CFG         = 0
canonical physicalizer      = 0
production caller/selection = 0
retry/fallback/reselection  = 0
legacy retirement/deletion  = 0
```

## Acceptance

- worker-reviewed source-role → Recipe/JoinSig/effect/After/Tail table is
  stored in this task and the companion Recipe SSOT;
- one authority is named for every logical key, BindingSSA effect, scope/frame,
  completion, and physical session;
- prefix target absence and tail's separate prefix binding are explicit, not
  silently inferred;
- negative matrix covers missing/duplicate/foreign source or frame, binding
  mismatch, unsupported policy, absent After/Tail, cross-owner pairing, and
  any second Recipe/SSA/PHI owner;
- implementation entry is one bounded Recipe co-seal row, with focused tests
  and `docs/reference/**` updated in the same implementation commit;
- physicalization and production selection remain closed until the design's
  fresh-session and exact-consumption gates are separately accepted.

## Stop rule

Do not implement Recipe, touch Builder/MIR, open a physicalizer, or delete
legacy callers while this design stop is open. Do not create deeper D4-style
suffixes. After acceptance, open one bounded Recipe implementation row and
stop again before physicalization.
