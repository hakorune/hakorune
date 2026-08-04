---
Status: Paused at design boundary; successor M10a D2-S5 task active
Date: 2026-08-04
Decision: accepted — `JOINIR-LOOP-ACCUM-MIR-PHYSICAL-SNAPSHOT0-M5-P4-S1`
Scope: compare the legacy Accum oracle with the already-connected resolved
       DirectAccum physicalizer through one immutable test-only alpha snapshot
Related:
  - joinir-loop-accum-mir-physical-snapshot-design0-m5-p4-task-2026-08-03.md
  - joinir-loop-accum-mir-physical-snapshot0-m5-p4-s0-task-2026-08-03.md
  - joinir-loop-accum-production-bridge-m10a-n2-design-stop-2026-08-03.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../../../../reference/mir/phi_invariants.md
  - ../../../../reference/mir/phi_policy.md
  - joinir-loop-accum-final-carrier-projection-m10a-d2-s5-task-2026-08-04.md

# DirectAccum physical-parity snapshot S1

## Why this task is now executable

The M10a bridge is no longer caller-zero: `compile_resolved` reaches one
resolved DirectAccum lowerer, and that lowerer reaches the shared physicalizer
through the caller-owned canonical CFG, Binding SSA, and `PhiTxn` services.
The legacy observer already exists as a test-only oracle. The next bounded
slice is therefore an observer adapter, not a new physicalizer or a route
cutover.

The predecessor P4 design card and the M10a bridge card contain historical
caller-zero wording. Before implementation, reconcile those claims in the
active cards and `CURRENT_STATE.toml`; do not use a stale zero-caller claim as
an acceptance criterion.

## Current blocker — production final-carrier contract

The first actual resolved candidate has the expected five-block topology and
Unit/After return, but its After block currently lacks the final `i`/`sum`
carrier evidence required by the accepted P1/D1 binding-publication contract.
This is not an observer mismatch. Do not project final values from header
PHIs or synthesize snapshot rows. P4-S1 is paused until
`JOINIR-LOOP-ACCUM-FINAL-CARRIER-PROJECTION-M10A-D2-S5` adds the caller-owned
sealed-After read and typed final-carrier receipt. This card resumes only
after the successor task's focused gates are green.

## Authority boundary

Source/admission authority is the resolved DirectAccum profile:

```text
resolved source/frame
  -> policy handoff + StructuralFacts
  -> verified Recipe/JoinSig + physical input
  -> CanonicalSsaFunctionSessionV2
  -> one ResolvedSsaIdentityStateV2 / BindingSsaBuilderV1
  -> one CanonicalCfgSessionV1 + one PhiTxn
  -> DirectAccum physicalizer
```

The snapshot is comparison-only. It must not own AST/source policy, route
selection, `CorePlan` lowering, CFG construction, operation emission, PHI/SSA
writing, candidate publication, or retry. `route_loop`, the legacy registry,
Generic policy, `LoopPhiMaterializerV1`, and raw test-only binding adapters are
not authorities for the resolved candidate.

## Implementation slice

1. Add a separate `#[cfg(test)]` child or equivalent private test module; keep
   every touched Rust source below 800 lines.
2. Adapt the actual resolved DirectAccum candidate MIR into the existing
   immutable alpha snapshot DTO. Use typed `preheader/header/body/step/after`
   role grammar and reject missing, duplicate, or unknown roles.
3. Derive successor/predecessor edges from terminators and reject any cached
   block-edge mismatch. Do not rely on adjacent-only duplicate detection or
   arbitrary role strings.
4. Compare legacy and candidate semantic core: CFG/terminators,
   operations/dataflow, carrier PHIs, final `i`/`sum`, and Unit/Void result
   disposition. Keep legacy-only PHI/copy/step/after rows behind the existing
   explicit auxiliary allow-list; unknown auxiliary rows fail fast.
5. Add late candidate failure evidence showing zero external commit and a
   fresh candidate/session reuse success. The live Builder must remain
   unchanged after the failed unpublished candidate.
6. Run the exact caller census: one non-test DirectAccum physicalizer caller
   in the resolved lowerer; `route_loop`, Retry/Option/fallback, and legacy
   physicalizer callers for this branch remain zero.

## Required gates

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
focused DirectAccum/physicalizer/resolved-lowering tests
candidate abort + fresh-session reuse test
RUSTFLAGS='-Awarnings' cargo check -q
```

The focused test names and counts must be recorded in this card at closeout.

The late-failure/reuse evidence must be rerun after D2-S5; it may not claim
final-carrier parity while the production receipt is absent.

## Explicit non-claims

This task does not claim raw MIR identity, allocation-ID equality, equal
instruction/PHI counts, printer-text parity, all Loop families, Generic V0/V1,
Retry deletion, old Accum-edge retirement, global PHI-writer retirement,
default `compile_with_source` activation, selfhost authority, M10b scheduler
cutover, grammar changes, or IR changes.

## Mandatory reference-document closeout

Implementation is not complete until the evidence and the design contract are
synchronized. The implementing commit or its immediately-following closeout
commit must update, as applicable:

```text
this task card and the M10a bridge card
P4 design/S0 observer card
joinir-loop-selfhost-recipe-pipeline-ssot.md
phi-lifecycle-ssot.md
binding-ssa-first-control-lowering-ssot.md
docs/reference/mir/phi_invariants.md
docs/reference/mir/phi_policy.md
src/mir/builder/README.md
CURRENT_STATE.toml and 10-Now.md when the active pointer/status changes
```

The closeout must record commands, expected/actual test counts, caller census,
line-budget evidence, and explicitly state that no grammar, IR, route,
Generic, Retry, or fallback behavior changed unless a separately accepted
cutover task authorizes it. Reference-document synchronization is an
acceptance condition, not optional cleanup.
