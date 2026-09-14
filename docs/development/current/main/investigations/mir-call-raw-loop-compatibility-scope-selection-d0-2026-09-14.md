---
Status: accepted__recovery_dependency__2026-09-14
Task: MIR-CALL-RAW-LOOP-COMPATIBILITY-SCOPE-SELECTION-D0
Date: 2026-09-14
Priority: unblock owner-level Hako Program(JSON v0) acceptance without widening loop migration
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: MIR-CALL-RAW-LOOP-COMPATIBILITY-SCOPE-SELECTION-I0
Implementation permission: false until this D0 is selected independently
---

# Raw Loop compatibility scope selection D0

## Six-line brief

Decision: keep the compatibility Program(JSON v0) root on the existing
non-callable child path; pass callable-loop scope and ledger only to the
source-backed selected route that issued both capabilities.

Source authority + canonical issuer: the existing compatibility/source-root
dispatch in `normal_default_program_root.rs` and `program_root_lowering.rs`.
The route owner decides whether a callable scope is present before raw child
lowering.

Non-authority: Hako lowerer membership, environment toggles, planner strictness,
fallback/retry, JSON/MIR scans, and a fabricated empty callable ledger.

Fail-fast boundary: a compatibility root must not enter the invocation port
with a scope but no ledger. A source-backed selected root keeps the current
scope+ledger route; if its ledger is actually absent, preserve the named
`[freeze:contract][raw-loop-child-entry/callable-ledger-missing]` stop.

Smallest next slice: make the compatibility branch omit
`callable_loop_root_scope` when constructing the raw child port, then add the
three route-level tests that prove compatibility, source-backed, and missing
ledger behavior.

Non-claims: no Generic G0 migration, loop recipe change, Hako owner change,
fallback deletion, backend parity, or whole-R7 completion.

## Boundary and finite census

Boundary: `module_invocation_session` → `normal_default_program_root` →
`program_root_lowering` → `raw_loop_child_port`. It includes only the choice of
raw child port and its existing scope/ledger capabilities; it excludes loop
semantic lowering after the port is selected.

| Root disposition | Scope | Ledger | Required result |
| --- | --- | --- | --- |
| Compatibility Program(JSON v0) root | absent | absent | existing non-callable/legacy child route |
| Source-backed selected root | present | present | existing invocation-aware route |
| Source-backed selected root with a missing ledger | present | absent | named `callable-ledger-missing` stop before effects |
| Non-loop root | either | either | unchanged existing route |

The worker audit located the current mismatch at
`src/mir/builder/program_root_lowering.rs:229` (scope is always supplied) and
`src/mir/builder/normal_default_program_root.rs:65` (the compatibility route
does not issue a ledger). The error is therefore a known baseline debt from
`25a264e89f`, not a StringBox lowerer current-change failure.

## I0 task contract

1. Confirm the compatibility/source-backed branch at the existing root
   dispatch; do not reclassify Hako Program(JSON v0) semantics.
2. Omit only the compatibility branch's callable scope when creating the raw
   child port. Keep source-backed scope+ledger construction unchanged.
3. Add focused positive/negative/guard evidence:
   compatibility Loop reaches its current legacy terminal, source-backed Loop
   still reaches the invocation route, and an explicitly missing ledger keeps
   the named fail-fast error with no emitted effects.
4. Update the owning module README/reference and this card together. Keep all
   Generic G0, fallback, and Hako StringBox work outside the slice.

## Acceptance

- Compatibility Program(JSON v0) Hako entry no longer stops at
  `callable-ledger-missing` solely because a scope was attached without a
  ledger.
- Source-backed selected entry still carries both scope and ledger into the
  invocation port.
- Explicit missing-ledger tests retain the named fail-fast terminal and emit
  no partial MIR.
- The existing StringBox phase14/16/17 probes can reach their selected owner;
  their own positive/negative membership evidence is recorded by the parent
  I0 card, not claimed here.

No Hako StringBox membership, source-artifact transport, shared-schema
retirement, backend parity, or whole-R7 completion is claimed by this D0.

## Recovery I0 evidence (2026-09-14)

The selected I0 implemented the bounded constructor branch and recorded its
route-level evidence in the child card. Compatibility no longer inherits an
unissued callable scope; source-backed selection still carries scope and
ledger, while an explicitly missing ledger remains a named pre-effect stop.
The downstream source-backed recipe boundary and Hako StringBox ownership are
outside this recovery decision.
