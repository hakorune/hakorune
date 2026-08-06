# Callable single-loop source-to-Recipe map D0

Status: `Decision: worker-reviewed design stop; implementation closed until mapping is sealed`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-LEDGER-S1`

## Change

Close one row-by-row, AST-free correspondence from the resolver-owned callable
ledger to the common portable Recipe/JoinSig/effect products for the selected
single-loop profile. This is a design contract only. It must not add a
Generic-specific Recipe or physical owner.

## Worker-reviewed D0 closeout (2026-08-07)

The review closes this D0 as one shallow design row; it must not be split into
deeper D0 suffixes. The four required outputs are:

1. a callable single-loop profile envelope;
2. a row-by-row source-to-portable correspondence;
3. the common owner/physical-input boundary and negative matrix; and
4. implementation-entry and `NoSafeSlice` gates.

The selected fixture is `StringHelpers.int_to_str/1`, a declared callable with
one loop at `Body(2)` and a terminal `return value` at `Body(3)`. Its prefix
`local value = me.to_i64(n)` is outside the Loop Recipe and must be materialized
by the outer callable plan. The Loop profile is exactly:

```text
loop(i < 1) {
  i = i + 1
}
```

It is intentionally not the nested two-loop `generic_g0` profile. Calls,
fields, captures, a second loop, upvars, non-`i64` values, symbolic bounds or
deltas, and non-terminal/opaque tails are outside this profile.

The logical mapping is fixed for design purposes only:

```text
L0 = the source loop; K0 = source BindingRef(i): i64
V0 = initial i; V1 = condition read; V2 = bound 1; V3 = Less result
V4 = step read; V5 = delta 1; V6 = Add result

I0 Read(K0) -> V1
I1 ConstI64(1) -> V2
I2 CompareLess(V1, V2) -> V3
I3 Read(K0) -> V4
I4 ConstI64(1) -> V5
I5 Add(V4, V5) -> V6
I6 Write(K0, V6)

C0 = (L0, K0, V0)
```

The source relation is not one-to-one: `i < 1` expands to a read, a literal,
and a compare. Coverage is therefore measured by
`(typed_source_site, source_role, target_kind)`, while every non-synthetic
operation/value also carries one exact source anchor. Synthetic carrier and
JoinSig glue must be named as derived products, never presented as source
rows.

The upper-level review corrected the row count before implementation: the
syntax observer has **9 syntax rows plus one separate prefix boundary**. The
9 are initial carrier (1), condition Lhs/Rhs/operator (3), step
Lhs/Rhs/operator/assignment target (4), and terminal tail shape (1). The
prefix call-to-binding boundary is the separate tenth whole-callable envelope;
whole-callable declaration/reference/assignment/exit coverage is proven by the
MAP-S1 join and outer plan, not by the syntax observer alone.

| source row | logical target | co-sealed evidence / disposition |
| --- | --- | --- |
| callable owner/origin/source-kind | common owner brand | every Recipe/Core/After row must match; foreign rejects |
| `Body(2)` loop | `L0`, condition/body nodes | resolver loop source, frame, and Scope/Region pair; missing/duplicate/nested rejects |
| `Body(1).Initializer(0)` (`i = 0`) | `K0`, `V0`, `C0.entry_value` | separate prefix/input projection; ordinal/name inference is forbidden |
| `LoopCondition.Lhs` | `I0 Read(K0) -> V1` | `ConditionRead`; exact BindingRef/site |
| `LoopCondition.Rhs` literal `1` | `I1 ConstI64(1) -> V2` | `ConditionBoundLiteral`; literal anchor required |
| `LoopCondition` operator `Less` | `I2 CompareLess` | `ConditionCompare`; exact operator/value relation |
| `LoopBody(0).Value.Lhs` | `I3 Read(K0) -> V4` | `StepRead`; must match target binding |
| `LoopBody(0).Value.Rhs` literal `1` | `I4 ConstI64(1) -> V5` | `StepDeltaLiteral`; literal anchor required |
| `LoopBody(0).Value` operator `Add` | `I5 Add(V4,V5) -> V6` | `StepArithmetic`; overflow/profile policy must be sealed |
| `LoopBody(0).Target` | `I6 Write(K0,V6)` | `StepWrite`; exact target/value pairing and one rebind |
| `Body(0).Initializer(0)` (`value` call) | prefix boundary | outer callable plan owns it; Loop physicalizer never re-lowers it |
| `Body(3).Value` (`return value`) | common After/Tail envelope | existing completion/DraftSeal remains sole Return writer |

The common effect relation must not overload the existing
`LoopBindingEffectRelationV1` with callable-specific fields. The design selects
one common `LoopOperationSourceRelationV1` family for the later Recipe row;
each row will carry an operation/item key, typed input/output value keys,
source role/site, optional `(LoopBindingKey, BindingRef)`, and literal/operator
payload where applicable. The existing binding-effect relation remains the
BindingSSA claim. A callable-specific effect or SSA owner is forbidden.

The source `L0` brand is resolver-issued and must be co-sealed with its
`LoopExecutionFrameKeyV1` and a resolver-issued scope/region pair. Portable
source claims alone do not prove source existence or frame identity. The
outer callable plan owns the whole source ledger, prefix, tail, completion,
and loop brand; the portable core owns Recipe/JoinSig/logical relations; the
physical input later carries core + brand/frame/scope + After disposition; the
physicalizer borrows the canonical session and returns an open After. The
callable lowerer consumes prefix/tail and whole-function coverage. DraftSeal
alone extracts and publishes the function.

`ResolvedSsaIdentityStateV2::finish()` requires whole-callable coverage. The
S0 fixture therefore cannot be completed by a Loop-only physicalizer: its
`value` declaration/call, `i` declaration, Loop rows, and terminal Return must
all be covered by one outer canonical plan. If that coverage cannot be sealed,
the result is `NoSafeSlice`, not a partial function.

### Explicit negative matrix

`NoSafeSlice`/typed rejection is required for missing or duplicate rows,
foreign owner/frame/scope, a second loop or nested profile, unsupported
operator/type/literal, mismatched condition/step/target bindings, absent
initial carrier or prefix boundary, missing operation/value/source relation,
unavailable After/tail/completion, body-local escape, unconsumed whole-callable
coverage, a second resolver/SSA/PHI owner, or any post-effect retry/fallback.

### Implementation ladder after D0

```text
SYNTAX-S1 caller-zero VerifiedSourceSyntaxFactsV1 (AST-free shape only)
MAP-S1   caller-zero resolver/source map (no Recipe/ValueId/CFG effects)
RECIPE-S2 common Recipe + JoinSig + operation/effect + After/Tail co-seal
PHYS-S3  common physical input + canonical session/scope + completion
CUT-S4   named caller, strict/fresh/atomic/backend parity
RETIRE-R1 selected profile old caller/route/composer = 0
CORPUS-R2 all accepted profile dispositions complete, then legacy deletion
```

The syntax-facts design/implementation dependency is
`resolver-syntax-facts-d0-task-2026-08-07.md`; after its caller-zero product
lands, go directly to the MAP-S1 task below. Do not create row-specific D0
suffixes.

The MAP-S1 task is
`generic-callable-single-loop-source-map-s1-task-2026-08-07.md`; its immutable
design fixture is
`docs/development/current/main/design/fixtures/generic-callable-single-loop-source-map-d0-v1.json`.

No deeper D0 suffix is authorized. Each later implementation row updates
`docs/reference/**`, the immutable fixture/receipt, current pointer, and
workstream in the same commit.

## Contract

- Every admitted source site has one typed disposition: mapped, unsupported,
  opaque, or missing; duplicate and foreign sites reject before lowering.
- The mapping names condition read/operator/bound, body read, assignment
  target/value/step, initial carrier, effect relation, Loop membership/scope,
  After, and function tail/continuation. Derived Recipe glue is not a source
  site or AST rewrite.
- Resolver owns source identity and `LoopExecutionFrameKeyV1`; the compiler
  projector owns profile policy. `CanonicalSsaFunctionSessionV2` remains the
  sole physical owner, and DraftSeal remains the sole completion boundary.
- The selected callable single-loop profile remains distinct from nested
  Generic G0. No direct shape projection, retry, fallback, route selection,
  or production caller is allowed.

## Done

- A compact mapping table proves source site, Recipe item/key, effect row,
  carrier/merge, scope, After, and tail correspondence for the positive
  fixture.
- The common Recipe schema is shown to represent the exact recurrence and
  function completion, or the row is explicitly `NoSafeSlice`.
- Positive, missing, duplicate, foreign, opaque, and tail/scope counterexamples
  are named before implementation; one verifier input and one physical input
  owner are identified.
- The design and reference pointers are synchronized. Only after this row is
  accepted may a separate source-to-Recipe implementation row be opened.

## Stop

Return to a design review if any source operation, carrier, effect, scope,
After, or tail is inferred from a path suffix or an existing AST-bearing
Recipe. Do not add another transport adapter, deepen the task suffix, open a
physicalizer, switch production selection, or delete legacy callers while the
mapping is incomplete.
