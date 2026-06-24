---
Status: Landed
Date: 2026-06-15
Task: CURRENT-ENTRY-MANUAL-SYNC-001
Scope: Align root/current restart pointers and docs navigation after manual
  sync rows 739 and 740.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/README.md
  - docs/reference/language/README.md
---

# CURRENT-ENTRY-MANUAL-SYNC-001

## Result

```text
output_contract=hako-current-entry-manual-sync-v0
source_evidence=296x-739,296x-740
current_task_read_next_latest_card=296x-741
current_now_read_next_latest_card=296x-741
current_state_latest_card=296x-741
docs_layout_manual_sync_bucket_visible=1
docs_readme_concurrency_nav_linked=1
language_reference_record_box_nav_linked=1
language_reference_concurrency_nav_linked=1
manual_sync_guard_indexed=1
summary=ok
```

## Decision

Restart/manual entry points now lead to the current manual sync closeout before
older object-storage implementation rows.

`CURRENT_STATE.toml` remains the compact current-state SSOT. Manual/readme
pages point to reference/SSOT documents rather than duplicating long phase
history.

## Stop Line

```text
do not copy landed chronology into CURRENT_TASK.md
do not make README a phase ledger
do not leave Read Next pointing behind latest_card_path
```
