# Generic legacy corpus universe P0

Status: `landed 2026-08-07; next row is GENERIC-LEGACY-OBSERVATION-FRONT-G0`

Parent SSOT: `../design/generic-loop-source-to-portable-recipe-ssot.md`.

## Scope

Build one normalized, machine-readable case universe for the Generic legacy
disposition work. The universe is an observation inventory only: it must not
select a runtime route, open a production caller, delete legacy code, or infer
that a fixture name is a semantic route.

The input set is the active phase29bq cases, the selfhost subset, the four
Generic smoke cases, and Generic-named fixtures discovered by the checked
manifest. Compatibility stems may alias one canonical case, but every
`(case, mode, release/profile)` key must be unique.

## Required product

```text
source inventory
  -> canonical case identity
  -> mode/profile identity
  -> compatibility alias relation
  -> normalized Generic legacy corpus universe
```

The product must retain provenance for each case and distinguish canonical
fixtures from aliases. It must not call route selection, Recipe, Builder/MIR,
physical lowering, retry/fallback, or deletion machinery.

## Acceptance

- every selected case has one canonical identity and stable mode/profile key;
- duplicate keys, ambiguous aliases, and name-only route claims reject;
- the checked legacy manifest and this product enumerate the same universe;
- no runtime selection or production caller is introduced;
- the implementation updates the exact reference page and active workstream
  receipt in the same commit;
- focused corpus tests, the relevant manifest guard, pointer guard, and
  `git diff --check` are green before commit/push.

## Landed receipt

The checked manifest now contains 389 case records: 179 normalized
phase29bq rows, 198 planner-required selfhost rows, four Generic fixture-
inventory records, four canonical Generic smoke scripts, and four explicit
compatibility aliases. The exact 25-column union header, source line/profile
parity, canonical fixture existence, alias target contract, sentinel policy,
and inventory-only no-route boundary are validated by
`tools/checks/lib/generic_legacy_corpus_universe_guard.py`, focused tests, and
the shared `mirbuilder_inplace_replacement_guard.sh`. All cases remain
`unobserved`/`unknown`/`nonproduction-future-evidence`; edge records are zero
until the later dependency row. The next task is
`generic-legacy-observation-front-g0-task-2026-08-07.md`.

## Non-goals

This row does not observe runtime routes, classify dispositions, perform
cross-family dependency analysis, open the common physical preflight, switch
M10b, or delete Generic V0/V1/legacy files. Those remain later ordered rows.
