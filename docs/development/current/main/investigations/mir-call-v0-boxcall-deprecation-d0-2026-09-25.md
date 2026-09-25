# MIR-CALL-V0-BOXCALL-DEPRECATION-D0 — v0 boxcall ingress retirement decision

Status: closed__2026-09-25 (NoSafeSlice — wire retirement is product-level)
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

- [x] Finite consumer inventory for v0 `boxcall` with production-lane
  reachability evidence.
- [x] One accepted Decision (bounded slice or NoSafeSlice) with a
  named next execution row or an explicit reopen trigger.
- [x] R7 frontier status updated in the workstream row and owner card.

## Census result (2026-09-25)

- `module.rs:439-475` is the sole production `LegacyCallV0` minter
  (`func: INVALID`, `callee: Some(Callee::Method{receiver: Some})`).
  The `"call"`/`"mir_call"` arms already stop at
  `[freeze:contract][mir-json-v0/legacy-call-stopped]`; `externcall`
  mints a typed instruction.
- Wire producers/consumers of the `"op":"boxcall"` spelling:
  `env.mirbuilder.emit` Phase-0 emit (`runtime/mirbuilder_emit.rs`,
  compat profile methodize=false), vm-hako validator
  (`subset_check/boxcalls.rs`) + reissuer (`payload_normalize.rs`),
  `LLVM_SUPPORTED_JSON_OPS`/`llvm_json_ops_for_instruction`
  allowlists, `llvm_py` readers, phase14/17 contract pins requiring
  `"op":"boxcall"` + rc=3 re-entry terminal.
- `MirCall` has no `func` slot; every boxcall wire field maps onto
  `Callee::Method` — a carrier-only typed promote is structurally
  possible for the mint itself.

## Decision (accepted 2026-09-25): NoSafeSlice — wire retirement is product-level

- Owning gate: the Phase-0 `env.mirbuilder.emit` Program(JSON v0)→MIR
  JSON wire contract, jointly with the vm-hako conformance lane and
  the LLVM JSON op allowlists. Retiring the `boxcall` spelling is a
  wire-format deprecation, not a bounded slice.
- Reopen trigger: an accepted successor-wire Decision (canonical
  `mir_call{Method}` or explicit product Stop) with emit lane, vm-hako
  validator/normalizer, phase14/17 pins, and llvm_py allowlists all
  switched; then `module.rs:439-475` replacement becomes bounded.
- Carrier-only promote declined HERE, not rejected: the mint alone is
  structurally promotable, but `callsite_canonicalize` reads
  `LegacyCallV0{Method}` exclusively to resolve user boxes. Promote
  without pass changes silently bypasses that resolution; extending
  the pass to typed `Call{Method}` widens onto production
  `Union`/`RuntimeData` Method rows (`effect_emission.rs`,
  `exprs_qmark.rs`, `rewrite/special.rs`, `boxcall_emit.rs`,
  `resolver.rs`). Both directions need their own evidence row.
- Named next row: `MIR-CALL-V0-BOXCALL-MINT-PROMOTE-D0` — resolves
  whether a carrier-only promote (typed mint + bounded canonicalize
  read-side) is safe: census production `Call{Method{Union|
  RuntimeDataBox}}` emitters, rc=3 equivalence of the canonical-call
  stop, and selected-Dynamic `Call{Method}` terminal.
- ParkedSealed (outside this lane): `LegacyCallV0` type/`func` slot
  deletion, `externcall` arm, `handle_box_call`/`execute_box_call`
  APIs, llvm_py readers, WASM legacy readers (own parked task).
