Task: MIR-CALL-MAP-RETURN-SITE-FLOW-I0
Parent: mir-call-map-lifecycle-consumer-i0-2026-09-15.md
NextCard: map entry-value source-class coverage (MIR-CALL-MAP-ENTRY-VALUE-SOURCE-I0)
Implementation permission: landed
---

# Return-position map-literal flow observation I0

## Six-line brief

```text
Decision: extend map-flow observation so a `%{...}` MapLiteral at a
Return-value site produces an explicit row whose destination is the
function-return boundary — never silently skipped, never faked as a
Local binding.
Source authority + canonical issuer: the existing caller-ownership walk
in resolved_semantics/home_new_prefix.rs owns the terminal-Return arm;
resolved_control_flow/map_control.rs owns the outward verification;
home_map_flow.rs owns the sealed row shape.
Non-authority: the row must not mint a BindingRef for a return, claim a
physical consumer, or imply a Map return ABI exists.
Fail-fast boundary: return-position maps that fail exact membership /
path / owner / target checks stay Unavailable or named rejects; foreign
or duplicate sites stay rejected; `[Body, ReturnValue]` path is exact —
no nested shapes admitted.
Smallest next slice: add a ReturnBoundary destination kind to
MapHomeFlow, a `map_return_outward` verification for the
`[Body, ReturnValue]` path, and the terminal-Return arm hookup that
emits the row; focused positive/negative tests.
Non-claims: Map return physical ABI, per-owner install-contract
generalization (parent C5), admission success on the merged route —
this row only makes the site visible and explicitly unconsumed.
```

`Census boundary: merged entry program -> `%{` MapLiteral in
return position; 31 live sites (MirJsonEmitBox make_*/to_json family),
plus 27 `new MapBox()`-then-return functions downstream of the same
missing contract.`

## Entry contract

Parent card pinned the failing arm: `preflight_map_install`'s first loop
requires every MapLiteral site to have a `Complete` map-flow row, but
`observe_map` runs only for `local x = %{...}` initializers —
`return %{...}` sites produce no row and fail `map-source-unavailable`.
`map_source_outward` today pins `[Body(_), Initializer(_)]` paths and a
Local destination binding.

## Acceptance

Focused tests: `return %{...}` produces a `Complete` map-flow row with a
ReturnBoundary destination carrying the exact return statement site;
wrong-owner / wrong-path / non-literal return-value sites stay rejected
or Unavailable; existing initializer-position map tests stay green.
Route evidence: the merged entry advances past the first-loop
`map-source-unavailable` arm to the next named arm (expected:
per-owner/loan gate or terminal-relation coverage).

## Receipt (landed)

**Implementation**: `MapDestinationV1::{LocalBinding, ReturnBoundary}` in
`home_map_flow.rs`; `map_return_outward` in `resolved_control_flow/
map_control.rs` verifies exact `[Body(i), Value]` map path, `[Body(i)]`
return statement path, `ExplicitReturn` exit origin, and
`Return { target_function }` transfer to the current owner; the
terminal-Return arm in `home_new_prefix.rs` observes the map and records
`Complete`/`Unavailable`, deliberately leaving scalar terminal coverage
unissued (fail-closed `ReturnValueNotCovered`). Terminal-relation types
were extracted to `home_terminal_relation.rs` to keep both sources under
the 800-line boundary (539 + 340).

**Focused evidence**: `return_boundary_map_carries_exact_exit_membership`
and `return_boundary_outward_rejects_foreign_and_non_return_membership`
added; the map/resolved-control-flow focused suite passes 70/70
including all pre-existing initializer-position tests. Baseline reds
unrelated to this change were already manifest-listed.

**Merged input identity** (reproducibility): `/tmp/merged_entry.hako`
sha256 `23b6cf894b0619a41ade0f7ff77480d2228530727a0c883540f06d37b78ba1ed`,
19042 lines, 723247 bytes; the ordered input list is recoverable from the
89 embedded `// <file>.hako` section headers. Generation recipe is still
/tmp-local — tracked as punchlist P1-3.

**Route evidence (merged `/tmp/merged_entry.hako`, quick-profile
binary)**: the entry still stops at
`[callable-semantic-package/install] MapLifecycleConsumerMissing`, but
the boundary moved *inside* the same arm — all 12 `return %{...}` sites
are now observed and produce `Unavailable` rows (previously no row
existed). The first failing site (slot 673, `[Body(0), Value]`) rejects
with `MapCandidateNotCovered(EntryValue(0))`: merged map entry values are
string literals, nested `%{...}`, array literals (`[]`), and
non-scalar locals — none covered by the existing
`OrdinaryObservation::{Integer,Bool,TrivialLocal}`/`TransferHome`
admission. Entry-value source-class coverage is a different
responsibility and becomes the next bounded card.

**Non-claims held**: no `BindingRef` minted for returns, no physical
consumer, no Map return ABI, no per-owner install-contract change (parent
C5 still open). `map_flow()` folds missing and incomplete rows into the
same `map-source-unavailable` freeze, so the outer terminal label is
unchanged — the movement is observable only at row level.
