# MIRBUILDER-LOOP-COND-AFTER-PHI-S0

Status: CLOSED; selected PHI boundary verified
Date: 2026-10-11
Decision: `mirbuilder-loop-cond-after-phi-d0-2026-10-11.md`
Scope: one verified LoopCond after-PHI role into existing ordinary-new physical owner
Related: `mirbuilder-loop-cond-after-phi-d0-2026-10-11.md`

Implement the D0 with one sealed role from the selected LoopCond physical
path. The role must include exact source owner/site, after block, and PHI
prefix. Record it once in the existing ordinary-new ledger after successful
PlanLowerer source-publication completion. Physical capture compares the
sealed role against the actual block and its leading PHIs. Keep generic
single-predecessor PHI refusal and all existing physical checks.

The VariableAccum issuer experiment was disproved by route diagnostics and
removed from the production diff. Its pre-rework diff was preserved under
`/tmp/hako-loop-cond-before-rework-20261011.patch`; route-neutral exact
PHI-prefix checks were retained. No production source rewrite.

Close this S0 only after a focused exact-role positive/negative family and
the unchanged SizeClassBox physical probe cross block 32 to a later named
frontier. `cargo check` alone and a synthetic fixture are insufficient.

## Evidence

The selected LoopCond branch now calls a PlanLowerer entry that checks the
selected plan's after PHI destinations against the completed physical block
after source publication, then seals owner/site/block/exact PHI rows. The
ordinary-new ledger transports it to its existing physical boundary. The
VariableAccum record path was removed. `cargo check --profile quick -p
nyash-rust --lib` is green (`/tmp/hako-loop-cond-after-phi-s0-check-20261011.log`).
The original SizeClassBox physical probe moved from
`physical-boundary/phi-in-recorded-block` to
`IncompleteOrdinaryNewCoverage`
(`/tmp/hako-loop-cond-after-phi-s0-physical-20261011.log`); this is the next
aggregate stop, not whole-source success.

Focused exact-role test: 1/1, including wrong block, predecessor,
input and destination refusal
(`/tmp/hako-loop-cond-after-phi-s0-final-focused-20261011.log`).
Existing physical-boundary family: 8/8. Existing signed-return Home family:
3/3. Raw loop child entry family: 11/11. The latter three reused the same
quick-profile test binary after an interrupted redundant Cargo rebuild;
the interrupted run was not counted as a pass. Final `cargo check` passed
(`/tmp/hako-loop-cond-after-phi-s0-final-check-20261011.log`), and
`current_state_pointer_guard.sh` passed. No new guard was added.

Disposable coverage diagnostics identify only `size_to_bin` owner 5 as
`all_ready=Some(false)` and `local_calls=false`; `bin_size` owner 4 and
`good_size` owner 7 are ready with emitted local-call sites
(`/tmp/hako-loop-cond-after-phi-s0-coverage-20261011.log`). The physical
red is the known next Home frontier, not a failure introduced by this role.
The S0 closes on this bounded first-stop movement; whole SizeClassBox and
mimalloc-lite acceptance remain open.
