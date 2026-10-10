# MIRBUILDER-LOOP-COND-AFTER-PHI-D0

Status: decided; select `MIRBUILDER-LOOP-COND-AFTER-PHI-S0`
Date: 2026-10-11
Parent: `mirbuilder-negative-integer-return-home-s0-2026-10-11.md`
Scope: original `SizeClassBox.bin_size/1` loop-after PHI at physical capture
Related: `mirbuilder-loop-cond-after-phi-s0-2026-10-11.md`

## Evidence

The unchanged source probe reaches `physical-boundary/phi-in-recorded-block`
for `bin_size` block 32. Block 32 has three leading one-input PHIs from
header block 29. Disposable route instrumentation shows
`raw_loop_child_entry` with source input and callable handoff, then
`CallableGenericLoopSourceFactsDispositionV1::LoopCondReady`
(`Discriminant(5)` in its declared variant order). No VariableAccum role is
issued (`/tmp/hako-bin-size-route-instrumented-20261011.log`). The earlier
VariableAccum issuer Decision is disproved for this caller; its WIP remains
uncommitted and must not land as `bin_size` coverage.

## Decision

The verified `SourceLoopCondPhysicalInputV1` owns source owner and parent
site. The selected `CorePlan::Loop` owns `after_bb` and planned after PHIs;
PlanLowerer patches and validates their physical instructions before
successful source-publication completion. Issue one sealed loop-after role
only after that completion, binding owner, source site, exact after block,
and exact canonical leading PHI rows (`dst` and every predecessor/value).
Require the physical after block to match the selected plan and PHI rows.
Transport this role once through the existing callable state's
`OrdinaryNewClaimLedgerV1`. At recorded physical capture, grant the
single-predecessor exception only to that exact block with that exact PHI
prefix. Preserve the existing generic refusal and finished physical
projection checks. Plan failure, absent/duplicate role, wrong owner/site,
block, PHI input value, or edge must fail closed. No MIR-shape inference
or role from an unselected route.

Keep source authority and representation distinct: the LoopCond source
product selects the route; the verified plan and physicalizer issue the
block/PHI receipt. `PhysicalBoundary` consumes the receipt but does not
infer it from the CFG. The selected caller is the original `bin_size`
LoopCond branch in `raw_loop_child_entry.rs`; its former rejection is the
replacement target. No additional authority or retry route is needed.

Positive acceptance: original `bin_size` crosses this physical boundary
and reaches the next named unchanged-source frontier; focused PHI and
source-publication tests pass. Negative acceptance: same-predecessor wrong
defined `ValueId`, wrong destination, wrong block/site/owner, non-leading
PHI, and unselected route refuse. Do not claim full SizeClassBox or EXE
acceptance; `size_to_bin` search-loop Home is a separate queued D0.
