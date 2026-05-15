# 293x-433 DOCS-SLIM-026 Phase Card Resolver Leak Helper Extraction

Status: landed
Date: 2026-05-16

## Decision

Extract the repeated phase-card resolver leak checks into a shared helper in
`tools/checks/lib/guard_common.sh`. Keep the docs-slim guard semantics
unchanged.

This row only thins gate-leak boilerplate. It does not change the docs-slim
card / taskboard / history-pin guard bands.

## TODO

- [x] Add a shared helper for phase-card resolver leak checks.
- [x] Convert the DOCS-SLIM-004/005/006/007/008/009/010/013/014/015/016/
  017/018/019/020/021/022/023/024 guards to use the helper.
- [x] Keep the per-row landed-history pin assertions and card metadata checks
  in place.

## Scope

- Guard helper extraction only.
- DOCS-SLIM-004..024 leak-check assertions only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change docs-slim guard semantics in this row.
- Do not remove the per-script landed-history pin assertions or card metadata
  checks.
- Do not wire the helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_026_phase_card_resolver_leak_helper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Added `guard_require_no_phase_card_resolver_leak` to
  `tools/checks/lib/guard_common.sh`.
- Converted the DOCS-SLIM-004/005/006/007/008/009/010/013/014/015/016/
  017/018/019/020/021/022/023/024 guards to use the shared helper for gate-
  leak assertions.

## Evidence

```text
bash tools/checks/docs_slim_026_phase_card_resolver_leak_helper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
