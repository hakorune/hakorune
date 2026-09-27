# MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-S0

Status: landed
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-D0 (closed,
decision accepted) — implements the `Main` lineage arm from
`mirbuilder-gate1-static-result-ingress-lineage-d0-2026-09-27.md`.
Workstream row H / Gate-1 internal owner series.

## Scope (bounded)

One classification arm: a located `Main(locator)` root with a
callable ledger verifies its caller through the sealed declaration
catalog probe and takes the existing publication handoff — same
probe discipline the `Cataloged` arm already applies. No other
lineage changes.

- `src/mir/builder/static_result_publication_ingress.rs`
  `classify_source_context_v1`: `Located{root: Main(locator), site}`
  + `source_backed` ->
  - `declarations == None` -> `DeclarationCatalogUnavailable`;
  - `declaration_for(StaticBoxMethod, locator.box_name(),
    locator.method_name(), locator.arity())` -> `Cataloged{caller:
    decl.key().clone(), site}` — the sealed key, never rebuilt;
  - probe `None` -> `ForeignLineage`.
  All other non-`Cataloged` roots keep `ForeignLineage`;
  `!source_backed` stays `Unavailable`.
- Pins in the same test module:
  - located `Main` + matching `Main.main` / `Main.<helper>`
    declaration -> `Cataloged` carrying the sealed key;
  - `Main` + `declarations == None` -> `DeclarationCatalogUnavailable`;
  - `Main` + undeclared method probe -> `ForeignLineage`;
  - `Main` + `!source_backed` -> `Unavailable` (ledger-less
    raw-root producer unchanged).
- `src/mir/builder/README.md`: admitted-lineage table + recorded
  coverage dependency for `InstanceConstructor`/`TopLevel`.

## Acceptance

- New pins green; existing ingress tests green.
- `member_route` / me-policy consumers unchanged.
- Guards green; every edited file < 800 lines.

## Non-goals

- No `InstanceConstructor`/`TopLevel` admission — blocked on the
  caller-coverage family (`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-
  COVERAGE-D0`), which is the observed boxtorrent terminal owner.
- No `NestedBoxMethod`/`ScriptRoot` admission — permanently outside.
- No inventory/catalog/result-catalog widening.
- No producer changes; `Main`+ledger stays production-unreachable —
  the arm is the rule-complete classification contract only.
- No fallback: `Unavailable` is never manufactured for a
  source-backed located root.
- Gate-1 or overall MirBuilder completion is not claimed.

## Landed evidence (2026-09-27)

- `classify_source_context_v1` gained the `Main(locator)` arm ahead
  of the foreign arm: `!source_backed` -> `Unavailable`; absent
  catalog -> `DeclarationCatalogUnavailable`; unresolved probe ->
  `ForeignLineage`; resolved probe -> `Cataloged{caller:
  decl.key().clone(), site}` — the sealed key only.
- Pins green (11/11 ingress tests): sealed-declaration resolution for
  `Main.main/1` and `Main.sample/0` (call target passed as
  `LayoutBox.class_size/0` to prove the caller key comes from the
  root declaration, never the call); absent catalog ->
  `DeclarationCatalogUnavailable`; undeclared `Main.ghost/0` ->
  `ForeignLineage`; ledger-less `Main` -> `Unavailable`.
- `member_route`/`me`-policy consumers unchanged; `static_result`
  26/26, `publication` filter 169+2 baseline-debt reds
  (`reserves_four_mechanical_lanes_without_builder_publication`,
  `birth_receiver_non_escape_rejects_unproven_uses_before_row_
  publication` — both in `cargo_lib_red_baseline.tests.txt`),
  `member_route` 16/16, `callable_result` 95/95.
- Guard repair (baseline debt): the reusable
  `script_static_result_publication_ingress_guard.sh` still pinned
  the retired `member_route` TargetOnly *rejection* arm; landed
  `734fdc0c91` (TARGET-ONLY-EMISSION-S0) routes it through the
  physical bridge. Guard updated to pin the bridge consumption for
  `member_route` and the named rejection for `me-policy` — PASS.
- `src/mir/builder/README.md`: admitted-lineage table +
  `InstanceConstructor`/`TopLevel` coverage dependency recorded.
- Edited files: ingress 568 lines, guard 95, README — all < 800.
- Pointer + ingress guards green.

## Next selected row

`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-D0` — the caller-
inventory coverage design for `BirthConstructor`/`FreeFunction`
callers; it owns the observed boxtorrent terminal
(`InstanceConstructor`-rooted `LayoutBox.class_*` field-initializer
calls inside `HakoAllocHeap` construction).
