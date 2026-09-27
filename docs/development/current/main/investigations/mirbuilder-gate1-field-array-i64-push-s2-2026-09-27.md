# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S2

Status: landed
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D2 (closed, decision
accepted) — implements the boundary pins from
`mirbuilder-gate1-field-array-i64-push-d2-2026-09-27.md`.
Workstream row H / Gate-1 series row 3 (caller cutover + retirement
disposition).

## Scope (bounded)

The D2 census proved the membership's armed-lane cutover is already
complete and its delete-set is empty — every candidate seam is a
shared fail-fast/reconstruction arm retained for other callers.
This slice encodes the accepted lane dispositions as durable pins;
it deletes nothing and changes no runtime behavior.

- Pin: claimed loop pushes (rows minted + consumed, markers
  recorded) on `compile_normal` stop at typed
  `[freeze:contract][named-array/retained-source-required]` — the
  plain-finishing lane deliberately does not discharge.
- Pin: a straight-line field-resident push (outside any loop body)
  mints no row (issuer loop-placement `continue`), compiles on
  `compile_normal`, emits a generic `MirInstruction::
  ArrayElementWrite`, and records zero
  `named_array_write_obligations` — the designed coverage split.
- `src/mir/source_call_target/README.md`: record the lane
  disposition table and the membership-scoped empty delete-set
  verdict.
- Focused suite + named-array regression re-run; guards.

## Acceptance

- New pins green alongside the existing 12 named-array source
  tests; construction/ordinary-new 78-pin suite green.
- No code path changes — dispositions only.
- Guard + pointer sync; every edited file < 800 lines.

## Non-goals

- No family-level retirement (needs caller-zero across
  FieldResidence + Text sites).
- No runtime-owner execution evidence (blocked upstream on outer
  `new` admission / lifecycle admission / static-result ingress —
  recorded in D2 as dependency).
- No lane discharge extension for `into_parts` (deliberate
  non-consumption per `5a2dea9b3c`).
- No straight-line row minting (loop placement is the designed
  coverage split).
- Gate 1 completion is not claimed.

## Landed evidence (2026-09-27)

- `claimed_loop_push_rejects_on_unretained_plain_lane`: the
  production four-push `holder_source` shape on `compile_normal`
  (plain `into_parts` finishing) stops at
  `[freeze:contract][named-array/retained-source-required]` —
  typed, not silent; matches the probe observed on the default
  `mir` lane and `--backend llvm`.
- `straight_line_field_resident_push_stays_on_generic_write`: a
  non-loop `a.push(1)` on `me.free_stack` compiles on
  `compile_normal`, emits one generic `ArrayElementWrite`, and
  records zero `named_array_write_obligations` — pinning the
  issuer's loop-placement coverage split (no row minted → no
  residual-row leak).
- `src/mir/source_call_target/README.md`: lane disposition table +
  empty membership delete-set verdict recorded.
- Focused suite 10/10 green (`named_array_source_tests`); wider
  `named_array` filter 31/31 green. No code path changed.
- Row 3 verdict for D22: armed-lane cutover complete, membership
  delete-set empty; runtime-owner positive execution remains
  dependency evidence on queued upstream families (outer `new`
  admission, lifecycle admission, static-result ingress foreign
  lineage — the first observed app terminal). Next selected design
  row: `MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-D0`.
