# RAW-SOURCE0 LOWER ROOT0 — FINAL0 DRAIN HANDOFF 設計相談

Status: **Open design stop — DRAIN0 closed; FINAL0 not selected**
Date: 2026-07-24
Inventory source: worker audit after `c470dc12d6`

## Current boundary

`RAW-SOURCE0-LOWER0-ROOT0-DRAIN0-S0` is closed. The new Raw route now ends
at:

```text
RawRootBatchCompleteInvocationV1
  -> prepare_drain(self)
  -> PreparedRawDrainInvocationV1::drain(self)
  -> RawDrainedInvocationV1::{Script, App}
```

The drained product retains the opaque unfinalized module, sealed ledger,
ledger-derived manifest, root witness, continuation/runtime snapshot, and
route-specific helper/callable evidence. There are no production consumers
and no finalization/postprocess/external-commit wiring.

## Worker inventory

The new DRAIN output has zero non-test consumers. The old Raw finalization
chain is disconnected but still compiled:

```text
src/mir/builder/raw_physical_finalization.rs
  RawPhysicalCompleteInvocationV1::prepare_finalization
  hard-coded ["condition_fn", "main"] inventory
  collector.into_draft_functions()
  bare MirModule construction

src/mir/compiler/raw_finalization.rs
  RawFinalizationInputV1 over bare MirModule

src/mir/compiler/module_postprocess.rs
  run_raw over the old RawFinalizationInputV1
```

Their current call sites are test/disconnected fixtures only. They must not
become adapters for the new route. The existing generic `DrainedModuleCandidateV1`,
`finalize_drained_module_once`, caller-supplied symbol vectors,
`require_main`, and `ConditionFnPolicyV1` are also non-authorities for FINAL0.

The existing old finalization guard has a separate scope defect: it can count
`prepare_finalization` inside `#[cfg(test)]` fixtures as a production caller.
That guard repair is a separate cleanup and must not be hidden inside the new
Raw finalization owner.

## Questions to close

### Q1 — sole FINAL0 entry

Should the only compiler-visible entry be:

```rust
RawDrainedInvocationV1::{Script, App}::prepare_finalization(self)
```

with the old `RawPhysicalCompleteInvocationV1` bridge left test-only and
unconnected? A second finalization entry, generic candidate adapter, or bare
`MirModule` ingress would create a competing publication owner.

### Q2 — finalization owner and module handoff

Should a compiler-private Raw finalizer consume the complete drained owner,
while a Builder sibling terminal privately consumes
`RawUnfinalizedModuleV1` and `RawDrainWitnessV1`? The compiler must not receive
the shell/collector/ledger tuple or a bare mutable `MirModule`.

### Q3 — inventory and source authority

Should finalization derive all expected module facts only from
`RawDrainWitnessV1.manifest` and sealed ledger final events, proving the
opaque candidate module matches them? The following must remain zero:

```text
AST/source/catalog re-resolution
current_module inventory reads
collector-derived expectation
module-derived expectation
synthetic main/condition generation
MirBuilder::finalize_module fallback
```

### Q4 — preparation and commit boundary

Should FINAL0 use mutation-free preflight for token/family/brand, Builder
readiness, manifest↔module function parity, root witness parity, and source
evidence lifetime, followed by a private infallible commit? No ledger reserve,
collector mutation, shell publication retry, or external commit should occur
during preparation.

### Q5 — failure owner

Should every preflight failure retain the exact `RawDrainedInvocationV1` in a
discard-only rejected owner with typed nested cause and stage? Retry, resume,
fallback, replacement manifest, and partial bare-module publication remain
forbidden.

### Q6 — route-specific success product

Should success remain typed as:

```text
RawFinalizedInvocationV1::{Script, App}
```

retaining continuation, runtime snapshot, module name, route evidence,
manifest, sealed ledger, root witness, and helper/callable evidence until the
later POST0 handoff? The product must not infer Script/App from physical
symbols because an App with no helpers can look like a Script.

### Q7 — retirement and guard scope

Should FINAL0 close with these measured conditions?

```text
RawDrainedInvocationV1 non-test consumer = 1
old RawPhysicalCompleteInvocationV1 finalization caller = 0
old RawFinalizationInputV1 ingress caller = 0
hard-coded ["condition_fn", "main"] in Raw production finalizer = 0
raw finalizer into_draft_functions caller = 0
bare MirModule between DRAIN and FINAL0 = 0
source/catalog/current_module re-observation = 0
production postprocess/commit/ingress consumer = 0
all new/modified source/check files < 800 lines
```

The `#[cfg(test)]` exclusion defect in the old guard must be fixed in a
separate cleanup row or explicitly scoped as a guard-only prerequisite.

## Recommended candidate

```text
Candidate FINAL-DRAIN-prime-r1

Q1 = direct RawDrainedInvocationV1 consumer only
Q2 = one compiler finalizer + one Builder opaque-module terminal
Q3 = RawDrainWitness/ledger manifest only
Q4 = mutation-free prepare -> private infallible commit
Q5 = exact drained-owner discard-only rejection
Q6 = typed Script/App finalized product with evidence retention
Q7 = old bare-module/hard-coded-inventory bridge caller zero
```

This is a design question only. Do not implement FINAL0, modify the old Raw
finalizer, connect POST0, or activate production ingress until Q1–Q7 are
selected and recorded.

## Next row after selection

```text
RAW-SOURCE0-LOWER0-ROOT0-FINAL0-S0
  FINAL-MANIFEST0 -> FINAL-PHYSICAL0 -> FINAL-I0/G0
```

Non-claims remain: finalization, postprocess, external commit, public ingress,
JSON bridges, legacy retirement, and CUT0 activation.
