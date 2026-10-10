# Gate1 guarded outgoing transport D0

Status: selected design
Date: 2026-10-10
Scope: MIRBUILDER-GATE1-OBJECT-GUARDED-OUTGOING-TRANSPORT-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-result-new-site-provenance-d0-2026-10-06.md
  - docs/development/current/main/investigations/mirbuilder-gate1-bin-size-return-mul-home-s0-2026-10-10.md

## Production frontier and Decision

The unchanged `apps/mimalloc-lite/main.hako` pure-first EXE probe with fresh
quick CLI SHA-256 `b9079fd6ee018343cca3f7433536f6282df4100b0dfa474c69e667e018dcedde`
exits 1 before EXE at
`ordinary-new/borrowed-entry/source-only-object-actuals` on the original
`HakoAllocHeap.allocate -> HakoAllocPage.allocate` incoming call. The probe
command used `--backend mir`,
`--emit-exe`, `--emit-exe-nyrt`, `HAKO_BACKEND_COMPILE_RECIPE=pure-first`,
`HAKO_BACKEND_COMPAT_REPLAY=none`, and `NYASH_DISABLE_PLUGINS=1`; elapsed
time was 0.05 s after a 247.56 s quick CLI build. No app source changed.

Read-only audits of the physical Loop and object actual owners found that
physical `LoopBinaryI64::Mul` is a deferred separate capability, but the
production app stops before reaching it. `SourceObject` is deliberately
non-executable: `prepare_object_source_actuals_v1` retains original target
and candidates with no opaque actual proof, and `require_executable_v1`
refuses it. The typed-object input finisher can regenerate only the
zero/all-ExactI64 case; it cannot authorize the unannotated opaque formal
here. Do not promote `SourceObject` directly to executable.

The original checked guard and outgoing actual source evidence already exist:
`ordinary_new_borrowed_formal_uses.rs` records `BorrowedGuardedActualV1`
at the exact call/ordinal, and the existing source-seeds owner consumes it.
The real imported-source test retains all 15 Heap callers and confirms
candidate Integer agreement while formal executable agreement is false.
Thus another source-receipt slice would duplicate authority.

The missing part is the exact executable exclusion edge. The real-source
test proves `Heap.allocate` is absent from the final executable definition
set; it does not distinguish an initial source-draft rejection from later
fixed-point transport pruning. Static review finds all three original `size`
uses at exact method-call arguments, so `UnsupportedUse` is not established.
`LayoutBox.class_id(size)` and both `Page.allocate(size)` calls need exact
target and transport classification. Neither the app's final error nor
candidate agreement alone identifies the first exclusion. The prior
result/new-site D0's
source-only step has landed; its checked Fault/SSA/ABI transport and
executable-actual handoff remain open. Result qualification, Home success and
source argument shape are not executable proof.

## Design task and acceptance

1. Distinguish original source-draft admission from later transport pruning
   for `Heap.allocate`. Inspect the exact `call_sources` and fixed-point
   `UnresolvedArgument` eligibility with the existing 15-caller inventory.
   Identify the first exclusion edge and its missing executable authority.
2. Select one existing constructor/physical consumer and one bounded S0 for
   that edge. Preserve exact owner/site/ordinal/root and guarded Normal/Fault
   correspondence; do not add a second scan or parallel classifier.
3. Reuse the real imported-source candidate/transport test and the existing
   negative family. Add only an independently missing positive/negative.
   Require all 15 caller rows and vetoes, then probe the unchanged app for
   its actual next first-stop.

Non-claims: this D0 grants no executable actual, physical payload, result
contract, EXE, Loop Mul, old-edge retirement or full MirBuilder completion.
