# Generic legacy corpus universe P0

Status: `next implementation row; S4 caller-zero Recipe producer landed 2026-08-07`

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

## Non-goals

This row does not observe runtime routes, classify dispositions, perform
cross-family dependency analysis, open the common physical preflight, switch
M10b, or delete Generic V0/V1/legacy files. Those remain later ordered rows.
