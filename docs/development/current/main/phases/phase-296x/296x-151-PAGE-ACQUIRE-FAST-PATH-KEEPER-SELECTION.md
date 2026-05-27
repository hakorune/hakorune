---
Status: Current
Date: 2026-05-28
Scope: select one page-acquire keeper after known-live release source/MIR refresh.
Blocker: PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-150-POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH.md
---

# 296x-151 Page Acquire Fast Path Keeper Selection

## Purpose

Select exactly one page-acquire keeper from the remaining page-local ArrayBox
surface. Keep compiler helper-copy lowering parked as secondary.

## Required Output

```text
output_contract=page-acquire-fast-path-keeper-selection-v0
input_contract=post-known-live-release-source-mir-refresh-v0
selected_keeper
keeper_owner
fallback_preservation
summary=ok
```
