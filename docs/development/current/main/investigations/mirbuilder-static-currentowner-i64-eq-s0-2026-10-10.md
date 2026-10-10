# Static CurrentOwner integer Eq execution S0

Status: selected implementation
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-I64-EQ-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-i64-eq-d0-2026-10-10.md
  - src/mir/dynamic_operator_contract/README.md
  - src/mir/builder/ops/README.md

## Selected replacement

Make the unchanged `SizeClassBox.accepts(size)` condition
`me.size_to_bin(size) == me.huge_bin()` executable through one verified
Static integer Eq path. Replace the source-only Eq condition responsibility
of this caller. The existing Static packet issuer, ordered Binary descent,
Compare emitter and IfForm remain the physical owners. Do not reuse the
formal-origin Instance borrowed-compare receipt or create a second emitter.

Source authority: the exact resolved If Eq source site, the two original
Static call claims/Home observations in Lhs/Rhs order, selected I64 result
and completed packet for each child. Canonical semantic issuer:
`dynamic_operator_contract` for `Equal(NormalInteger, NormalInteger)`,
with `TrivialBool`, `MaySuspend`, read-only operands, Normal/Fault and no
lifecycle. Canonical physical owners: `binary_expression_descent` ordered
child demand, `direct_call_disposition_port` plus the sole Static packet
issuer, `lexical_i64` Invoke/NormalResult/Fault, shared recorded Compare,
and IfForm Branch. A narrow source loan must bind both children and the
recorded comparison/condition use to the same original Eq site; it must
not infer identity from AST spelling or ValueIds alone. Keep final MIR and
backend verification independent.

Fail-fast: a missing/reordered child claim, source site, result class,
actual, Completion or packet, `!=`/`&&` replacement, wrong Compare op,
swapped children, or missing condition Branch refuses the whole relation.
No partial receipt, fallback, or generic Eq retry after selected failure.

## Acceptance

1. Confirm the unchanged imported `accepts` selects the raw/default path
   identified by D0. Its original Lhs packet emits Invoke Normal, then the
   RHS packet emits Invoke Normal, then exactly one Eq Compare feeds Branch.
   Lhs Fault bypasses RHS and Compare; RHS Fault bypasses Compare. The
   published JSON uses ordinary `compare/eq`, not `borrowed_null_compare`.
2. Corrupting either source claim/site, packet, I64 result, actual,
   Completion, compare operand/order or Branch destination rejects before
   publication. `!=` and `&&` are not admitted. No one-child partial
   execution proof is retained.
3. Existing Static one-caller and two-caller packet/entry tests, integer
   compare family, final MIR verifier and pointer guard pass. Classify any
   broader red against the baseline. No new guard solely for this row.

The app's recorded first stop is Heap-to-Page `source-only-object-actuals`;
do not count a standalone `accepts` result as whole-app EXE advance.
Heap/Page support and unrelated condition forms remain outside this slice.
