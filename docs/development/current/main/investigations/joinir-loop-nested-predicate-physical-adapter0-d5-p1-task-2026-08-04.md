# JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-P1

Status: implementation checkpoint — caller-zero resolver-issued prefix/effect
claims and canonical identity/scope harness are green; production wiring remains
out of scope.
Date: 2026-08-04
Design authority:
`joinir-loop-nested-predicate-d4-physical-emission-design-2026-08-04.md`

## Checkpoint

The following caller-zero slices are now landed and pushed:

```text
P1-A  resolver-issued prefix/effect products + mismatch guard
P1-B  declaration-only identity seam + initialized-readiness gate
P1-C  ordered effect consumer + duplicate/order rejection
P1-D  exact root/child LoopBody pair enter/close; Root.After retires j once
```

Focused evidence is green: nested effect-plan (4), ordered/scope consumer (3),
resolved-lowering regression (123), and the shared inplace-replacement guard.
P1-E still owns the final contract guard tightening. No production route,
physicalizer, Retry removal, or live-builder caller has been added.

## Objective

Issue the exact source claims needed by the nested-predicate physical adapter
without creating a second identity, SSA, PHI, or scope owner. P1 extends the
caller-zero P0 input seam only; it does not add a production route or mutate a
live builder.

```text
resolved function owner/frame
  -> VerifiedNestedPrefixInputV1
  -> VerifiedNestedBindingEffectPlanV1
  -> VerifiedNestedPhysicalEmissionInputV1 + P0 block projection
  -> later canonical-session adapter (D5-P2/I0)
```

The prefix and effect products are non-`Clone`, owner/frame-branded, and
resolver-issued. They may carry `BindingRefV1` and exact source sites, but may
not infer identity from a name, ordinal, AST reread, or fabricated `ValueId`.

## PHI/SSA ownership decision

The canonical ownership is already established and must not be duplicated:

```text
CanonicalSsaFunctionSessionV2
  = ResolvedSsaIdentityStateV2
  + BindingSsaBuilderV1
  + CanonicalCfgSessionV1
  + one caller-owned PhiTxn
```

P1 may add one narrow declaration-only method to
`ResolvedSsaIdentityStateV2`, tentatively:

```rust
activate_declaration_without_value(
    site: &SourceBindingSiteV1,
    expected_kind: BindingKindV1,
    expected_name: &str,
) -> Result<BindingRefV1, String>
```

Its contract is exactly: ledger adoption, declaration coverage, and active
scope activation; **no** `BindingSsaBuilderV1::define`, `ValueId`, PHI, or
physical effect. The first assignment must still use the existing
`define_assignment_exact` path. A narrow ledger helper may remain private to
the canonical identity module.

`BindingSsaBuilderV1` remains value-definition/reaching-value authority. The
canonical identity state may keep one initialized-binding set as its lifecycle
gate: `read_entry`, variable-use claims, and assignment reads must reject an
active-but-uninitialized binding before delegating to SSA (otherwise an open
read could create a provisional PHI for `j`). This is not a second ledger or
SSA owner; it is the identity owner's readiness invariant. The effect adapter
still supplies the ordered `DeclaredUninitialized -> Defined` transition and
must not keep a competing readiness truth.

## Exact prefix contract

`VerifiedNestedPrefixInputV1` must contain exact resolver evidence for:

| binding | declaration | entry/first definition |
| --- | --- | --- |
| `i` | root local declaration site, `Local`, exact binding | root `i` initializer value site and value evidence |
| `sum` | root local declaration site, `Local`, exact binding | root `sum` initializer value site and value evidence |
| `j` | root-body local declaration site, `Local`, exact binding | explicitly absent; declaration-only activation |

The prefix is a source/effect input, not a physical plan. Root `i` and `sum`
use the existing declaration publication contract with resolver-issued initial
values. `j` must be adopted without a reaching value and cannot be read until
its first assignment claim is consumed.

## Exact effect-plan matrix

`VerifiedNestedBindingEffectPlanV1` covers every claim once and in order:

1. root `i` declaration and initializer;
2. root `sum` declaration and initializer;
3. root predicate `i` read;
4. child `j` declaration-only activation;
5. child first assignment `j = 0` (the first SSA definition);
6. child predicate `j` read;
7. child `j` update read and assignment target;
8. child ancestor `sum` read and assignment target;
9. root `i` update read and assignment target;
10. static root-loop-body lexical retirement of `j` after the root loop reaches
    its outer `Root.After` boundary (exactly once, not on each backedge).

Each item carries its exact `SourceBindingSiteV1` or `SourceExprSiteV1`,
`BindingRefV1`, and owner/frame brand. Duplicate, foreign, out-of-order, or
missing claims reject before the first physical effect. The `j = 0` row is a
first-assignment target/value claim; it is not a variable-read claim because
the recipe emits a literal followed by `WriteBinding`. Predicate/update rows
carry their separate identity-read and assignment-target sites.

`j` is owned by the outer root-body lexical scope; it is not recurrence-visible
through `ParentBodyResume`. Its retirement is a static lexical-close claim at
the root loop's `Root.After` boundary, once after the loop has exited. It must
not be called after each root backedge or each outer iteration, because the
same `BindingRefV1` is reused by the next iteration.

## Required implementation order

### P1-A — resolver-issued products (caller-zero)

Add the named prefix/effect products and their consuming constructor from the
existing resolved source projection. Preserve P0's Recipe/JoinSig/topology
pair and block projection; do not reread AST or rebuild a recipe. Add positive
and mismatch tests.

### P1-B — declaration-only canonical identity seam

Add the one narrow identity/ledger seam described above. Prove that it adopts
and activates `j` without touching `BindingSsaBuilderV1`; then prove the first
`define_assignment_exact` supplies the first reaching value. Keep all touched
source/test files below 800 lines.

### P1-C — ordered effect witness

Add a caller-zero adapter harness that consumes the effect plan in the matrix
order. It must reject `j` reads before its first assignment, accept the first
assignment exactly once, and drive the canonical identity owner's
`DeclaredUninitialized -> Defined` transition. Do not add a local readiness
map or a second identity/SSA owner.

### P1-D — exact LoopBody scope retirement

`ResolvedSemanticStackV1::enter_scope_region` already verifies an arbitrary
sealed scope/region pair, so the preferred seam is to consume the resolver's
`loop_pair()` directly with `ScopeKindV1::LoopBody` and
`RegionKindV1::Loop`. Enter root and child pairs once; close the child pair
without retiring the outer `j`, then close the root pair exactly once at the
root predicate-false `Root.After` boundary so the existing
`ResolvedScopeRetirementV1` owner retires `j`. Never call generic
`retire_scope_success` with a raw declaration list while skipping pair
verification, and never retire on an iteration-time backedge.

### P1-E — guards and evidence

Add grep/contract guards proving that the P1 products contain no AST reread,
name/ordinal lookup, fabricated physical IDs, direct MIR/PHI writes, route
retry, or live-builder caller. Run focused tests and update the current-state
pointer only after the gates below are green.

## Acceptance gates

```text
prefix/effect production callers = 0
all claims are resolver-issued exact site+BindingRef pairs
Recipe/JoinSig/topology pair remains preserved exactly once
j declaration activates without SSA define or ValueId
j first assignment defines exactly once through BindingSsaBuilderV1
j read before first assignment = typed reject
j read after first assignment = accepted exact claim
j retirement after root-body last use = accepted; later use = reject
all claims consumed once; missing/duplicate/order mismatch = reject
no second identity/SSA/PHI/scope owner
all touched Rust/test files < 800 lines
```

Focused tests must cover positive matrix consumption, foreign owner/frame,
site/binding mismatch, duplicate claim, pre-assignment read, first assignment,
post-assignment read, retirement, and fresh candidate reuse. P0 remains
caller-zero and preseeded; production wiring, Retry removal, legacy PHI writer
retirement, external commit, and selfhost claims remain outside P1.

## Explicit non-claims

P1 does not claim that the Nested physicalizer is production-ready, that all
Loop routes have one winner, that Generic retry debt is classified, or that
route-specific PHI materializers have been retired. Those are later
winner-equivalence and D5-I0 gates.

## Remembered post-cutover convergence task

Keep this as a named follow-up; do not let the temporary family adapters become
the final design:

```text
JOINIR-LOOP-RECURSIVE-FRAME-CONVERGENCE0-M12
```

After D5-I0 has one production physicalizer caller:

1. extend the shared JoinSig owner with explicit branch-transfer/merge
   obligations for conditional `break`/`continue`/`return` versus fallthrough;
2. prove `LoopV0`, `LoopTrue`, and `LoopCond` are the same verified recursive
   frame (`Always | Predicate` plus typed exit rows), with no route/family tag in
   semantic input;
3. replace their family switch with one recursive physicalizer consuming only
   `VerifiedRecipe + VerifiedJoinSig + VerifiedPhysicalFramePlan + effect plan`;
4. make Nested consume the same frame recursively, preserve carrier/latch
   ordering, and keep `ParentBodyResume` distinct from `Root.After`;
5. classify Generic V0/V1 post-effect debt separately, then remove its retry
   projection rather than mixing it into the unified admission;
6. require family production callers = 0, physicalizer caller = 1,
   `Option`/`Retry` = 0, parity green, and late-failure candidate isolation.

This is a cleanup/cutover follow-up, not part of caller-zero D5-P1. The
portable `LoopRecipeV1` already supplies the single recursive semantic shape;
the remaining work is JoinSig closure and physical adapter convergence.
