# Checked GreaterEqual NormalInteger contract S0

Status: closed bounded contract slice
Date: 2026-10-10
Scope: MIRBUILDER-GE-NORMAL-INTEGER-CONTRACT-S0
Related:
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md
  - docs/reference/language/dynamic-operators.md
  - src/mir/dynamic_operator_contract/README.md

## Decision

Source authority + canonical issuer: original `Op::GreaterEqual` at
`LayoutBox.accepts` is the production trigger. The existing
`dynamic_operator_contract` issuer alone owns the profile-neutral checked
integer comparison envelope. `Greater` and `LessEqual` define the adjacent
NormalInteger laws. `CompareOp::Ge` and physical `sge` exist but do not grant
semantic permission.

Non-authority: raw MIR, JSON, C emitter, Home preflight, candidate Integer
agreement, and `SourceStatic` argument retention cannot issue the comparison
contract or execute the CurrentOwner condition call.

Fail-fast boundary: issue only
`GreaterEqual(NormalInteger, NormalInteger)`. Both operands are borrowed in
source order, Normal is TrivialBool without lifecycle, and a non-Integer
operand Faults before result or operand mutation/rebind. Dynamic, I64, Null,
and mixed domains remain rejected. Existing opaque-formal `>=` source
rejection stays until a separately selected source/physical consumer exists.

Selected responsibility: extend the single canonical issuer, its existing
comparison contract test family, and the language/owner contract text. This
is a prerequisite contract slice, not a production caller switch. It replaces
no executable edge and authorizes no transport.

## Acceptance

- Existing complete comparison-axes test includes GreaterEqual and proves
  stable singleton issuance.
- Focused negative covers every other operand-class pair for GreaterEqual.
- Existing opaque-formal `>=` source rejection remains green.
- Existing physical `Ge -> sge` projection and C V4 guarded compare are
  reviewed or checked with the nearest focused existing test; no new physical
  path is introduced.
- After passing, return to guarded outgoing transport D0 for the exact
  CurrentOwner condition-call source and packet. Do not infer that the
  unchanged mimalloc-lite EXE has advanced from this contract-only slice.

Parked: Page outgoing edges, Loop Mul, result contract, full EXE and old-edge
retirement.

## Closeout evidence

The canonical issuer now issues only the NormalInteger/NormalInteger
GreaterEqual envelope, with the same complete axes as Greater/LessEqual.
No borrowed-formal source classifier or physical emitter changed.

- `cargo test --profile quick -p nyash-rust --lib dynamic_operator_contract::tests::`
  with `CARGO_BUILD_JOBS=4`: 8/8 PASS on the selected test binary; 5m18s
  compile, test execution under 0.01s. Existing warnings only.
- Same binary, `checked_compare_source_retention_does_not_expand_acceptance`:
  1/1 PASS, preserving rejection of opaque-formal `>= 0`. An initial
  over-specific `--exact` filter selected 0 tests and was not counted.
- Same binary, `serializer_preserves_signed_compare_predicates`: 1/1 PASS,
  including `Ge -> sge`.
- Read-only C review: the published physical verifier accepts the `sge`
  predicate and checks row shape and operand availability; V4 emission checks
  tagged operand kind 1 before the `icmp` or branches to Fault. No C code
  changed in this contract slice.
- `rustfmt --check` for the three modified Rust files and `git diff --check`
  passed. No production EXE rerun is claimed: this contract alone cannot
  execute `LayoutBox.accepts`'s CurrentOwner condition call.
