# JOINIR-IF-RECIPE-D0-D-PHYSICAL-ADOPTION

Status: design stop active; no D0-D code is authorized until this card is
closed. D0-C1/C2 admission-only wiring is landed in `38f6a751d2`.
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

## Ordered tasks

### D0-D1 — demand handoff

- Replace the current claim-and-drop operation with a single-use `take_if` that
  returns the verified physical input/demand exactly once.
- Keep `NotThisShape` pre-effect and typed; do not pass it into the physicalizer.
- Add a guard proving the selected payload is not discarded and the production
  demand/physicalizer caller count is exactly one.

### D0-D2 — physicalizer pilot

- Add `CanonicalIfRecipePhysicalizerV1` in a separate small module.
- Consume JoinSig/artifact source identity and drive the existing canonical
  session for the selected explicit-else join.
- Preserve the existing source leaf emission only as an admitted immutable view;
  route choice and branch topology come from the verified demand.
- Add a typed `Result` terminal and late failure injection inside the existing
  unpublished function/module candidate.

### D0-D3 — shape-scoped old-edge cutover

- Prove the selected explicit-else path no longer invokes the old
  source-driven branch-selection sequence.
- Retire only that old edge; do not delete global `lower_if`, raw IfForm, A+,
  CorePlan/JoinIR, or JSON-v0 writers.
- Keep all unselected writers and shapes under their own guards and design rows.

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

## Explicit non-claims

This row does not unify every PHI/SSA writer, does not cover implicit-else,
nested/Loop/Call/Record/Match/short-circuit/effect shapes, and does not retire
raw/A+/CorePlan/JoinIR/JSON-v0 paths. Repository-wide sole-writer claims require
later independent caller-zero rows.
