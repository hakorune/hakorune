# CURRENT-STATE-CONSUMER-CENSUS0-P0 — key x reader census

Parent: `repo-physical-structure-cleanup-ssot.md` parked order
(`CURRENT-STATE-CONSUMER-CENSUS0-P0` -> `CURRENT-STATE-LIVE-SCHEMA0-D0`
-> `CURRENT-STATE-LIVE-CUTOVER0-I0-R0`), opened because
`REPO-FINAL-CONVERGENCE-AUDIT0-G0` lists the cutover row as a
dependency.

## Scope boundary

```text
起点: docs/development/current/main/CURRENT_STATE.toml at HEAD
終点: per-key reader census (classification only — no key removed)
includes: all 37 top-level keys; executable readers under tools/,
          src/, CURRENT_TASK.md that name CURRENT_STATE.toml
excludes: docs prose mentions (guidance text, not consumers);
          archive copies under docs/development/archive/**
```

## Manifest

`docs/development/current/main/design/fixtures/current-state-consumer-census-p0-v1.tsv`

37 keys classified (parser skips multi-line string bodies —
`reference_delta=` inside `latest_card_summary` is prose, not a key):

- 23 `live-pointer` — read by guards/tools (the pointer guard alone
  reads 21; `current_blocker_token` is name-checked by ~353 guards,
  `latest_card` by ~279, `current_execution_row` by ~57).
- 6 `parked-pointer` — `parked_*` task keys; part of the allowed
  floor ("explicitly parked-family pointers").
- 8 `informational` — zero executable readers detected by string
  census (`development_branch_ssot`, `finite_product_goal`,
  `guard_followup`, `current_execution_cohort`,
  `next_design_card_path`, `next_execution_card_path`,
  `final_convergence_cleanup_ssot`, `current_update_policy`,
  `repo_hygiene_punchlist`,
  `active_mirbuilder_compile_time_performance_task`).

## Key finding — file already at floor

The SSOT text still describes "830 lines with 266 top-level
assignments". At HEAD the file is 79 lines / 37 keys — the pointer
guard already enforces `MAX_CURRENT_STATE_LINES=80` and
`MAX_CURRENT_STATE_TOP_LEVEL_FIELDS=40`, i.e. the live floor. The
`I0-R0` cutover's destructive half (removing 200+ historical keys)
therefore has no remaining bulk; what remains is the schema decision
on the 8 informational keys and the two `next_*_card_path` keys.

## Rules recorded

- Census method: string-level reader detection; a key counts as read
  when an executable file names `CURRENT_STATE.toml` and the key.
  Zero-reader does not equal dead — `LIVE-SCHEMA0-D0` must re-verify
  before any removal.
- No key was removed or renamed; `reference_delta = 0`.

## Next

`CURRENT-STATE-LIVE-SCHEMA0-D0` — fix the minimal live schema over
the 24 live + 6 parked keys, and decide the 8 informational keys
(retain as documented pointers vs remove), then
`CURRENT-STATE-LIVE-CUTOVER0-I0-R0` migrates/removes atomically.
