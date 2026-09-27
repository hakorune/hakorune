# MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-S0

Status: emitted
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
