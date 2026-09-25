# MIR-CALL-V0-BOXCALL-DEPRECATION-D0 — v0 boxcall ingress retirement decision

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-COMPATIBILITY-RETIRE-R7 (caller-zero D0 closed
  NoSafeSlice; sole surviving minter = v0 boxcall parser)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Question

After S4, the v0 `boxcall` parser in `src/runner/mir_json_v0/module.rs`
is the sole remaining `LegacyCallV0` minter (receiverful `Callee::Method`
rows). R7 caller-zero deletion stays `NoSafeSlice` while it lives. This
row decides whether boxcall ingress retirement is a bounded slice or a
product-level wire deprecation that must park the R7 frontier.

This is a design_stop census row — no implementation, no fixture
changes, no production switch.

## Census surface (bounded)

- Consumers of v0 `boxcall` rows: `env.mirbuilder.emit` Phase-0 wire
  contract, release-mode v0 JSON loading, `--mir-json-file`, vm-hako
  compatibility/reference validators, `llvm_py` readers.
- Reader arms consuming `LegacyCallV0{callee: Some(Callee::Method)}`
  outside the minter (transport, published view compat lane, verifier).
- Whether a named deprecation/stop for `boxcall` input has a finite
  caller list and an observable acceptance boundary, or whether it
  requires an external product decision (record `NoSafeSlice` with
  reopen trigger and the owning gate).

## Decision options

1. Bounded retirement — replace boxcall mint with a named stop or a
   typed carrier behind an explicit compatibility gate, with finite
   callers and wire evidence.
2. `NoSafeSlice` — record the product decision owner, the reopen
   trigger, and confirm R7 frontier pause with all remaining blockers
   `ParkedSealed` outside this lane.

## Exit

- [ ] Finite consumer inventory for v0 `boxcall` with production-lane
  reachability evidence.
- [ ] One accepted Decision (bounded slice or NoSafeSlice) with a
  named next execution row or an explicit reopen trigger.
- [ ] R7 frontier status updated in the workstream row and owner card.
