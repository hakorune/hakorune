# JOINIR-IF-RECIPE-D0-D-PHYSICAL-ADOPTION

Status: D0-D3 selected-shape old-edge cutover is implemented locally; focused
tests, receipt parity, line budgets, and reusable guards are green. This is a
shape-scoped If adoption slice, not global PHI/SSA retirement. D0-C1/C2
admission-only wiring is superseded locally by the consuming pilot.
Date: 2026-08-04

## Why this is a new boundary

D0-C2 proves that the selected recipe is admitted at the exact sealed If site,
but its `VerifiedIfPhysicalInputV1` payload is currently consumed and dropped.
The canonical lowerer then continues its existing source-driven physical
branch/merge/PHI emission. That is useful parity scaffolding, not physical
adoption.

D0-D must promote the one-shot payload into the physicalizer. A claim bit alone
is not sufficient evidence of production consumption.

## Selected scope

Only the already sealed resolved-trivial explicit-else shape is selected:

```text
one IfControl row
explicit else
root-level body site matching the source claim
then/else fall through
one outer BindingRef assignment per branch
homogeneous admitted i64/Bool merge class
post-merge read
no nested control, return/throw, short-circuit, Call, Record, Match, or effect
```

All other shapes remain pre-effect `NotThisShape` or typed reject. Raw IfForm,
A+ `IfCfgSessionV1`, CorePlan/JoinIR, JoinIR converter, JSON-v0, and unrelated
PHI writers remain separate authorities and are not global cutover targets.

## Physical authority

Reuse exactly one existing sink:

```text
CanonicalSsaFunctionSessionV2
  = CanonicalCfgSessionV1 + BindingSsaBuilderV1 + one PhiTxn
```

Do not create a new SSA/PHI transaction, `IfCfgSession`, CFG writer, or route
registry. A new `CanonicalIfRecipePhysicalizerV1` belongs in its own file below
800 lines and may call the existing canonical session APIs only.

## Required production contract

The admission bridge must become a consuming demand handoff rather than a
drop:

```text
preflight selected recipe
  -> take_if(statement)
  -> VerifiedIfPhysicalInputV1 / typed demand (non-Clone)
  -> CanonicalIfRecipePhysicalizerV1
  -> CanonicalSsaFunctionSessionV2
  -> Result<CanonicalIfPhysicalSuccessV1, Freeze>
```

The physicalizer must access the JoinSig/artifact and use them to determine the
fixed logical entry/condition/then/else/continuation edge correspondence. It
must not rescan AST to select a route, repair a missing predecessor, or invent a
PHI input. After selection, failure is terminal `Freeze`; there is no `Option`,
Retry, fallback, route registry, or reselection.

The handoff API is explicitly `Result`-only after selection:

```text
take_if(statement)
  -> Result<VerifiedIfPhysicalInputV1, CanonicalIfDemandRejectV1>
```

`NotThisShape` is resolved before this API is entered. The demand owns a
consumed/unconsumed state, rejects a second take, and returns a terminal
`SelectedIfNotConsumed` error at function finish. No `Option` is allowed as a
physicalizer input or output.

The selected handoff must return a concrete non-`Clone`
`CanonicalIfPhysicalDemandV1`, not `Result<(), _>` followed by a discarded
payload. The demand owns the verified physical input, expected source If site,
and a producer-side correspondence receipt containing the owner-branded Join
`BindingRef`, condition site, then/else assignment sites, and continuation
read. These rows come from the same-pass facts (`if_site`, condition,
assignments, and continuation read); they are not a portable-schema widening
and do not authorize an AST/facts rescan. `into_parts(self)` has exactly one
production consumer: the physicalizer.

## Physical mapping proof

`IfJoinSigV1` contains recipe-local keys, not physical IDs. The physicalizer
must therefore prove this correspondence before emitting the selected shape:

```text
JoinSig logical ports/edges
  -> verified IfControl row and source site
  -> canonical profile BindingRef/representation claims
  -> actual BasicBlockId predecessors and ValueId PHI inputs
```

Its minimum state is an `&mut MirBuilder`, the existing mutable
`CanonicalSsaFunctionSessionV2`, the selected `IfControl` row, the
`TrivialProfileConsumptionV1`/BindingSSA claim ledger, and an immutable source
leaf view. It must validate the logical digest, branch predecessor count, value
class, and post-merge BindingSSA read before physical emission. Reading a
JoinSig without proving this mapping is not physical adoption.

## Local implementation status

The pilot now carries a non-`Clone` `CanonicalIfPhysicalDemandV1` from the
same-pass adapter into one dedicated physicalizer caller. The demand owns the
verified physical input and correspondence receipt; `into_parts(self)` is
consumed there, and the physicalizer returns a typed `Result` without
`Option`/retry/fallback. It validates the fixed logical/source correspondence
before delegating emission to the existing canonical-session-backed lowerer.

D0-D3 now splits the old edge by shape: selected explicit-else demand enters
`lower_if_recipe_selected` with a private JoinSig-derived topology token,
while unselected/legacy shapes enter `lower_if_legacy_unselected`. The
selected core returns a typed physical receipt containing the actual branch,
merge, predecessor, and value evidence; the physicalizer validates that
receipt before success. The reusable guard proves the selected helper has one
production caller (the physicalizer), and that selected code does not call the
legacy helper or inspect source-driven topology.

The test-only late-seal failure runs after selected If lowering returns and
proves candidate drop, unchanged live fingerprint, and fresh same-compiler
reuse. This remains a bounded adoption slice: it does not claim exclusive
PHI/CFG production authority or global If retirement, and it does not broaden
to implicit-else, nested/effect shapes, raw/A+/CorePlan/JoinIR, or JSON-v0.

## Ordered tasks

### D0-D1 — demand handoff

- Replace the current claim-and-drop operation with a single-use `take_if` that
  returns the verified physical input/demand exactly once through `Result`.
- Keep `NotThisShape` pre-effect and typed; do not pass it into the physicalizer.
- Add a guard proving the selected payload is not discarded, second take is
  rejected, unconsumed demand freezes at finish, and the production
  demand/physicalizer caller count is exactly one.

### D0-D2 — physicalizer pilot

- Add `CanonicalIfRecipePhysicalizerV1` in a separate small module.
- Consume JoinSig/artifact source identity and drive the existing canonical
  session for the selected explicit-else join.
- Preserve the existing source leaf emission only as an admitted immutable view;
  route choice and branch topology come from the verified demand. The
  physicalizer must carry the logical-to-physical mapping proof above.
- Add a typed `Result` terminal and late failure injection inside the existing
  unpublished function/module candidate.

### D0-D3 — shape-scoped old-edge cutover

- Split the common `lower_if` into a named selected-shape dispatch and an
  unselected legacy helper (for example `lower_if_legacy_unselected`). Prove
  the selected explicit-else path no longer invokes the old source-driven
  branch-selection sequence. A global `lower_if` caller-zero count is not a
  valid proof because unselected Ifs still use the legacy helper.
- Retire only that old edge; do not delete global `lower_if`, raw IfForm, A+,
  CorePlan/JoinIR, or JSON-v0 writers.
- Keep all unselected writers and shapes under their own guards and design rows.

### D0-D3 execution brief (design closed)

Change: route the selected explicit-else demand through a named selected
materializer and retain the existing source-driven helper only for unselected
shapes. Construct a private explicit-else topology token only after JoinSig
verification; the physicalizer passes that token, never `Option<bool>` or a
literal topology choice.

Contract: the selected arm may use the immutable source view for condition and
branch leaf emission, but it may not inspect `else_port`, choose topology from
AST shape, call the legacy helper, retry, or create a second CFG/PHI owner.
The neutral CFG/SSA core returns a compact physical receipt (header,
condition, branch exits, merge, predecessor/value pairs) which the
physicalizer validates against the consumed demand before success.

Done: the reusable guard proves one selected-helper caller (the physicalizer),
zero selected callers of the legacy helper, no selected `else_port`/route
selection, and parity for branch/merge/predecessor/PHI/continuation output.

Stop: if the receipt cannot be produced from the existing canonical session,
or if selected emission still needs source-driven topology/reselection, return
to design. Do not broaden this row to implicit-else, nested/effect shapes,
global PHI retirement, or raw/A+/JoinIR/JSON-v0 paths.

## Acceptance gates

```text
take_if / demand producer = exactly 1 selected production seam
physicalizer caller = exactly 1
physical input payload is accessed, not dropped
selected physicalizer Option/Retry/reselection/fallback = 0
old source-driven edge caller-zero for selected shape only
recipe JoinSig digest and source identity correspondence = green
branch targets, merge topology, predecessor sets, PHI values/count = parity green
interpreter result and diagnostics = parity green
late verifier/seal failure leaves live Builder/module/ID state unchanged
same compiler succeeds on the next request after failure
all touched Rust/test files < 800 lines
```

The late-failure harness must inject failure after selected branch/merge/PHI
work but before the inner function draft seal and outer module commit. It must
compare the pre/post fingerprints of the live Builder, module/function list,
ID cursors, current function, catalog/metadata, BindingSSA, and pending PHI
state, then compile a fresh request on the same compiler. Parity must compare
the existing explicit-else oracle against JoinSig digest, branch/merge edges,
two predecessor/value pairs, PHI count and inputs, post-merge BindingSSA read,
interpreter result, and diagnostics.

The 800-line budget includes every touched Rust owner: the new physicalizer
module, adapter, lowerer, test harness, and guards. `capability.rs` is already
800 lines and `compiler/mod.rs`/`source_bound_package.rs` are near the limit;
do not add physicalization logic there. Extract a small module instead.

## Fixed logical-to-physical topology

The selected fixed shell uses this table; the physicalizer may verify it but
must not infer or repair it:

| Logical port/edge | Physical correspondence |
| --- | --- |
| Entry / Condition | existing current block alias and condition value |
| Condition → Then/Else | one branch with two distinct fresh targets |
| Then / Else | fresh branch blocks |
| Then/Else transfer | each actual branch exit jumps to the one fresh merge block |
| Continuation | fresh merge block; exactly two branch predecessors |
| Join values | exactly `then_exit -> then_value` and `else_exit -> else_value` |

`CanonicalCfgSessionV1` owns the sealed predecessor witness and
`BindingSsaBuilderV1`/`PhiTxn` owns final value publication. An implicit header
predecessor, guessed `ValueId`, or repaired missing edge is a typed Freeze.

## Old-edge dispatch boundary

The common lowerer must be split into a shape-scoped dispatch:

```text
lower_if
  -> lower_if_recipe(selected demand)
  -> lower_if_legacy(unselected/implicit/unsupported)
```

The selected arm may not call the legacy helper. The guard proves this
selected-edge property; a repository-wide `lower_if` caller-zero count is not
required and would be misleading. `ResolvedIfElsePortV1`/AST `else_body`
topology selection remains only in the legacy arm for unselected shapes.

## Explicit non-claims

This row does not unify every PHI/SSA writer, does not cover implicit-else,
nested/Loop/Call/Record/Match/short-circuit/effect shapes, and does not retire
raw/A+/CorePlan/JoinIR/JSON-v0 paths. Repository-wide sole-writer claims require
later independent caller-zero rows.

## Design closeout

The worker audit confirmed this boundary is complete: the payload drop is
explicitly the D0-C limitation, the Result-only `take_if` contract and
unconsumed/second-take failures are fixed, logical-to-physical mapping is a
required proof, the selected old edge is shape-scoped rather than global
`lower_if` caller-zero, candidate/PHI parity and late-failure fingerprints are
named, and near-limit owners are excluded from physicalizer growth. D0-D1,
D0-D2, and the D0-D3 selected-shape cutover are implemented with focused
gates green. The next work must be a separately scoped adoption/design row;
do not silently widen this slice into global PHI/SSA or all-If retirement.

## Next selected row — `JOINIR-IF-RECIPE-BOOL-MERGE-P0`

Worker review selected the existing explicit-else shell's `InlineBool` merge
as the next bounded production proof. This reuses the already admitted profile,
facts mapper, portable value class, JoinSig, selected helper, canonical CFG,
Binding SSA, and `PhiTxn`; it does not add a lowerer branch or a new PHI owner.

Change: add one resolved-trivial fixture with a Bool local, explicit-else
Bool assignments, and a continuation Bool read; prove the selected demand and
physical receipt consume the existing Bool class.

Contract: retain the current explicit-else/fallthrough topology and one outer
BindingRef per branch. No implicit-else, return-ABI widening, route retry,
schema/JoinSig variant, or global PHI/SSA caller-zero claim.

Done: profile/mapper admission, selected-helper use, Bool receipt class, two
actual PHI predecessors, interpreter/diagnostic parity, and same-candidate
late-failure reuse are green; no lowerer owner grows past 800 lines.

Stop: if the fixture selects A+, `NotThisShape`, or a different
return/continuation contract, stop and open an implicit-else/value-shape design
row instead of widening this shell.
