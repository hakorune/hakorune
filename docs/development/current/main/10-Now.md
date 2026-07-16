Status: SSOT mirror
Date: 2026-07-14
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- priority: read the exact row from `current_blocker_token`. CAT0, MP0, P0c-F,
  P0c-MR, and Program-only R0 retirement through G0 are closed. The exact
  HMI-P0-D0 selected serialized Rust-emitted MIR JSON V1 plus a future strict
  whole-document profile as the sole `.hako` semantic-reference carrier. The
  HMI-P0-I0 is closed with one checked-in 43-instruction/9-caller/6-fixture-
  family/9-transport/9-VMValue-class inventory and one source-derived checker.
  HMI-P0-G0 is closed: one manifest-backed public guard validates source drift
  and a checked-in normalized coverage/lossiness report. HMI-P0 is therefore
  closed with no execution-owner or opcode activation. The current blocker is
  HMI-S0-D0 is now an explicit design consultation stop. Current-source audit
  found that the tolerant `.hako` JSON parser loses duplicate keys, Rust i64
  overflow is build-mode-dependent, and V1 drops CFG edge arguments. Lock the
  direct strict-reader boundary, portable overflow law, and exact empty-edge
  witness in the active consultation card before any interpreter code begins.
  C-prime is
  taskized as `G0 -> S0 -> V0 -> C0 -> I1 -> R0`; G0 through I1 are closed. The
  deterministic SCC partition is non-Clone, uses a host-stack-safe traversal,
  and keeps production callers at zero. V0 seals recursive-module admission and
  exact module/partition/typed-plan/call-row correspondence. C0 adds one passive
  module-level capability schema. I1 adds exactly one explicit VM-only ingress
  using unpublished drafts and atomic publication without changing other routes
  or adding fallback. The MR-specific MAX_CALL_DEPTH and inner parameter/return
  contract failure fixtures now prove caller restoration and interpreter
  reuse; the reference VM depth guard is a resource boundary, not a language
  recursion limit. R0 selected Program-only authority with
  `S0 -> P0 -> CUT0 -> G0`; all rows are closed:
  singleton self recursion now uses the Program/catalog/SCC/atomic-publication
  authority, while RootCallable, one-entry facades, and exact-one call policy
  are deleted in the same atomic change. G0 fixes the old-symbol/caller zero,
  one explicit Program ingress, and no-retry guards. The next implementation
  carrier must remain the same V1 JSON tree without v0/compact conversion or a
  second schema. The 29bq parser/MirBuilder lane remains failure-driven with no
  current blocker.
  Production Ownership SSA,
  Loop production,
  SCC-aware optimization, legacy
  fallback, default source, Lambda/capture, ProgramV0 authority, and durable
  RegionId materialization remain inactive
- parked language work: LANGV1 conformance closeout remains parked; no
  language behavior is changed by the reprioritization

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
