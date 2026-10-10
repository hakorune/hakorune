# CurrentOwner If condition call Home S0

Status: closed bounded source slice
Date: 2026-10-10
Scope: MIRBUILDER-CURRENTOWNER-IF-CALL-HOME-S0
Related:
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md
  - src/mir/resolved_semantics/home_new_prefix.rs
  - docs/reference/language/dynamic-operators.md

## Decision

Source authority + canonical issuer: the original resolved If condition
binary and its original Static CurrentOwner call from the selected source
ledger. The existing Home branch walk and scalar-expression owner issue one
`ExpressionValue` `LocalCallObservationV1`; the direct-value helper already
corroborates original call site, receiver, arity, ordered SourceStatic actuals,
result class and prior Homes. The `GreaterEqual(NormalInteger,
NormalInteger)` envelope landed at `c480ae8c70`.

Non-authority: a physical `sge`, retained SourceStatic arguments, candidate
Integer agreement, and the Loop-only CurrentOwner packet do not authorize
execution. No `LayoutBox.accepts` caller is present in the selected source;
this slice cannot infer its formal Integer class from absent callers.

Fail-fast boundary: select only a whole original If condition with exact
`GreaterEqual` root, left CurrentOwner ExactI64 call, and right integer-zero
literal. Check the whole root and sibling before any selected call-argument
demand. A mismatched operator, operand order, sibling, call identity, result
claim or returned arguments stays unavailable. Keep the existing
opaque-formal `>=` rejection and both Static transport seed vetoes.

Selected responsibility: extend the existing Home source observation for
this bounded condition shape. This replaces the uncovered condition-call
observation in that Home walk; it does not select a physical call packet,
change transport owners or switch the production EXE.

## Acceptance

- Focused positive proves one original `ExpressionValue` call at the left
  condition child with exact ordered actual and prior Homes, after whole-root
  validation.
- Focused negative rejects changed operator, reversed operands, nonzero or
  non-Integer sibling, source/target/argument drift and failed selected
  argument demand without publishing a partial call.
- Existing direct-value call, scalar-expression and borrowed-formal compare
  tests remain green. In particular, opaque-formal `>= 0` remains rejected.
- No physical packet or EXE result is claimed. Return to guarded outgoing
  transport D0 for the selected condition-call packet and all-caller closure.

Parked: physical packet, Static transport admission, Page edges, Loop Mul,
result contract and full EXE.

## Closeout evidence

The existing Home scalar-expression owner now selects the resolver-sealed If
condition only after the exact whole-root `>=`/right-zero/left-CurrentOwner
shape and canonical GreaterEqual envelope agree. It reuses the direct-value
issuer for original ordered arguments and prior Homes; the observation
retains `ExpressionValue`. A rejected selected demand leaves the condition
unavailable and publishes no call. No physical or transport owner changed.

- `cargo test --profile quick -p nyash-rust --lib
  current_owner_if_ge_zero_keeps_one_source_call_without_opening_transport`
  with `CARGO_BUILD_JOBS=4`: 1/1 PASS on the final source (5m12s build,
  0.01s test). It covers the original call, exact source argument, result,
  destination, prior Homes, operator/operand/literal negatives and the
  unchanged static-context transport veto. A Bool actual to an opaque formal
  remains a valid source-only call; it grants no Integer payload proof.
- A draft negative expected the opaque Bool actual to be rejected. That
  expectation contradicted `SourceStatic`'s candidate/Integer-evidence split;
  it failed 1/1, was corrected in this slice, and the final test passed.
  No changed-code or unclassified red remains.
- Same binary: direct CurrentOwner original arguments, corrupted static
  source arguments, scalar binary site identity, and opaque-formal `>=`
  rejection tests each passed 1/1. No 0-test filter counted as PASS.
- Rust format check for the modified Home source, `git diff --check`, and
  `current_state_pointer_guard.sh` passed. Existing warnings only.
- The selected real mimalloc-lite EXE first-stop was not rerun: this source
  slice cannot alter the executable packet or transport fixed point.
