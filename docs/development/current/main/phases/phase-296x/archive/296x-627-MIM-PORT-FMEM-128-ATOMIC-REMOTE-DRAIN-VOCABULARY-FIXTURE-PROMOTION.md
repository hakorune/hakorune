---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-128.
Related:
  - docs/development/current/main/phases/phase-296x/296x-618-MIM-PORT-FMEM-119-REMAINING-SOURCE-SYNTAX-SMOKE-RETIREMENT-TASK-ORDER.md
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_terminal_ladder_smoke.sh
---

# 296x-627 MIM-PORT-FMEM-128 Atomic Remote / Drain Vocabulary Fixture Promotion

## Purpose

Promote the remaining atomic remote / drain vocabulary rows into the
source-syntax manifest so the smoke suite stops carrying those fixtures as
shell-owned one-offs.

This landed slice covers the 627 vocabulary rows:

- atomic remote head push vocabulary
- atomic remote head drain vocabulary
- drain remote list to local vocabulary

## Implementation

```text
manifest promotions:
  ATOMIC_REMOTE_HEAD_PUSH_VOCABULARY
  ATOMIC_REMOTE_HEAD_DRAIN_VOCABULARY
  DRAIN_REMOTE_LIST_TO_LOCAL_VOCABULARY

source-syntax smoke:
  now seeds these vocabulary fixtures through the manifest runner
  keeps the branch / route source split for 296x-628
```

The 627 rows keep the atomic remote and drain vocabulary evidence manifest-
backed while leaving the branch / remote routing split for the next card.

## Closed

```text
atomic remote head push vocabulary promotion
atomic remote head drain vocabulary promotion
drain remote list to local vocabulary promotion
shell-owned atomic remote / drain source-syntax assertions
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The source-syntax manifest now owns the atomic remote / drain vocabulary
fixtures, and the smoke can continue toward the remaining branch / routing
split without reintroducing shell-owned vocabulary rows.
```

## Closeout

```text
next: 296x-628 branch / remote routing source-syntax fixture split
```
