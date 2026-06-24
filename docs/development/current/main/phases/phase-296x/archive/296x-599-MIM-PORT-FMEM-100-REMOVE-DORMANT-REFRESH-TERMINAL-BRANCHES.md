---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-100.
Related:
  - docs/development/current/main/phases/phase-296x/296x-598-MIM-PORT-FMEM-099-REPORT-CHECK-REFRESH-PROFILE-SSOT-CLEANUP.md
---

# 296x-599 MIM-PORT-FMEM-100 Remove Dormant Refresh Terminal Branches

## Purpose

Remove the now-dormant per-refresh-profile terminal rule branches from
`fastmem_check_terminal_rules.py` after 296x-598 moved refreshed profile checks
behind `RefreshProfileSpec`.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not change emitted KV rows
do not change fastmem-check semantics
do not touch non-refresh pilot terminal branches
do not add product behavior
```

## Acceptance Sketch

```text
only refresh-profile terminal rule duplication is deleted
refresh profiles still validate through RefreshProfileSpec
fastmem_check_smoke stays green
fastmem_source_syntax_smoke stays green
```

## Landed Evidence

```text
fastmem_check_terminal_rules.py:
  before this row: 1389 lines
  after this row: 953 lines

refresh terminal branches now validate only through:
  refresh_profile_spec_for_rows(rows)
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
296x-600 MIM-PORT-FMEM-101 producer refresh flag-row cleanup.
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
