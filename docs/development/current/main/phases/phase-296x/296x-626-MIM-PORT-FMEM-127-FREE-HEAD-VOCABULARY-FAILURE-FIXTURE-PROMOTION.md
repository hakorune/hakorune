---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-127.
Related:
  - docs/development/current/main/phases/phase-296x/296x-618-MIM-PORT-FMEM-119-REMAINING-SOURCE-SYNTAX-SMOKE-RETIREMENT-TASK-ORDER.md
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_terminal_ladder_smoke.sh
---

# 296x-626 MIM-PORT-FMEM-127 Free-Head Vocabulary Failure Fixture Promotion

## Purpose

Close the remaining source-syntax smoke retirement slice by splitting the
terminal ladder into a dedicated smoke script and promoting the remaining
precondition / vocabulary rows into the source-syntax manifest.

This landed slice covers 296x-623..626:

- 296x-623 terminal ladder shared-input split
- 296x-624 local-free precondition manifest promotion
- 296x-625 free-head precondition manifest promotion
- 296x-626 free-head vocabulary failure fixture promotion

## Implementation

```text
source smoke:
  retains only source-syntax fixtures and the manifest runner seed
  no longer carries the terminal ladder route / product ladder blocks

terminal ladder smoke:
  dedicated script now owns the route / terminal ladder checks

manifest promotions:
  LOCAL_FREE_PUSH_PRECONDITION
  LOCAL_FREE_POP_PRECONDITION
  FREE_HEAD_PUSH_PRECONDITION
  FREE_HEAD_POP_PRECONDITION
  FREE_HEAD_PUSH_VOCABULARY
  FREE_HEAD_POP_VOCABULARY
```

The new terminal ladder smoke keeps the shared route / ladder checks together
without reintroducing them into the source-syntax smoke.

## Closed

```text
source-body assertion changes
terminal ladder shared-input ownership split
precondition fixture promotion for local-free / free-head rows
free-head vocabulary failure fixture promotion
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_terminal_ladder_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The source-syntax smoke now seeds only manifest-backed source fixtures, while
the dedicated terminal ladder smoke owns the shared route / ladder evidence.
```

## Closeout

```text
next: 296x-627 atomic remote / drain vocabulary fixture promotion
```
