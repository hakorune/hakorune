# JOINIR Loop Canonical Family-Plan Envelope F0

Date: 2026-08-03
Status: accepted; clean-worktree gate satisfied; F1 implementation in progress.
Task: `JOINIR-LOOP-CANONICAL-FAMILY-PLAN-ENVELOPE0-F0`

## Decision

The resolved canonical compiler currently has two different notions mixed in
one plan sum:

```text
semantic body profile: DirectAccum
module/header/token lifecycle: BindingSsaTrivial
```

Keep those notions separate before adding another Loop family. Introduce one
compiler-layer semantic envelope:

```text
CanonicalFirstFamilyPlanV1
  -> Loop(CanonicalLoopFamilyPlanV1)
       -> DirectAccum(CanonicalDirectAccumPlanV1)
  -> TrivialBindingSsa
  -> CurrentCanonicalAPlus
```

The source-bound package mirrors that boundary:

```text
ExactCanonicalPreflightPlanV1
  -> Loop(CanonicalLoopFamilyPlanV1)
  -> BindingSsaTrivial
  -> APlus
  -> callable plans
```

`CanonicalLoopFamilyPlanV1::DirectAccum` is the only Loop variant admitted in
this slice. Nested, LoopTrue, LoopCond, and Generic variants are not
placeholders: each may be added only after its own sealed structural/source
product and JoinSig closure exists.

The external lifecycle remains `BindingSsaTrivial`. Do not add a Loop variant
to `ModuleInvocationFamilyV1` or `ResolvedOwnerHeaderFamilyV1`; the semantic
family and lifecycle family are intentionally different axes.

## Work order

### F0 — contract and line-budget freeze

This card fixes the enum ownership, mapping, and non-claims. No route scheduler,
`route_loop`, Generic policy, Retry, PHI writer, SSA owner, or physicalizer
changes are allowed.

### F1 — source-bound plan extraction (behavior-neutral)

Move `ExactCanonicalPreflightPlanV1` and its route mapping into a small
`source_bound_plan` sibling. Re-export the type through the old module during
the series so existing callers remain stable. The extraction must reduce
`source_bound_package.rs` below the 800-line ceiling before the enum envelope
is introduced.

### F2 — Loop envelope migration

Change both plan sums to carry `Loop(CanonicalLoopFamilyPlanV1)`. Update every
exhaustive match once. The DirectAccum source-bound caller consumes the nested
variant and invokes the same candidate/session/commit path. Trivial/A+ and
callable plans must reject or remain unreachable for Loop without fallback.

### F3 — evidence and closeout

Add focused tests and shared guards for:

- preflight emits `Loop(DirectAccum)`;
- source-bound route remains `BindingSsaTrivial`;
- DirectAccum production caller and prepare/commit edge remain exactly one;
- old top-level `DirectAccum` variants are absent;
- no lifecycle-family, route/retry, second PHI/SSA, or production `route_loop`
  edge is introduced;
- all touched Rust/check files remain below 800 lines.

## Clean-worktree boundary

This is a BoxShape refactor series. Unrelated worker WIP in builder/resolver/
test files was verified as rustfmt-only and moved to the named recoverable
stash `wip/formatting-only-before-loop-family-plan-f1`. F1/F2 must not absorb
that formatting diff; the series starts from the clean HEAD covered by this
card.

Each series commit must build. The final F2/F3 commit is the only commit that
changes the enum shape; intermediate F1 commits are mechanical extraction only.

## Non-claims

This task does not add a second Builder candidate, symbolic MIR, undo journal,
new Loop recipe vocabulary, Nested source forest, LoopTrue/LoopCond branch
closure, Generic debt classification, `route_loop` production wiring, or
legacy PHI/SSA retirement. The existing
`CanonicalCfgSessionV1 -> BindingSsaBuilderV1 -> PhiTxn` chain remains the only
physical PHI/SSA authority.

## Acceptance

```text
DirectAccum successful MIR/result/fresh-reuse parity = unchanged and green
source_bound_package.rs < 800 lines                 = true
new source_bound_plan.rs < 800 lines                = true
Loop envelope production caller                   = exactly one
old top-level DirectAccum plan variants            = zero
new route/retry/fallback/PHI/SSA owner              = zero
```

Stop and return to design if moving the enum requires changing lifecycle
identity, source re-observation, route selection, or PHI/SSA ownership.
