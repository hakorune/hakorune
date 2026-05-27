---
Status: Current
Date: 2026-05-28
Scope: refresh source/MIR observation after the accepted .hako reason bind keeper.
Blocker: HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-124-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT.md
---

# 296x-125 Hako Mimalloc Post Hako Reason Bind Source MIR Refresh

## Purpose

Row124 accepted the `.hako` reason-local bind keeper:

```text
after_hako_elapsed_median_ms=610
previous_checkpoint_hako_elapsed_median_ms=620
keeper_effect=accepted
```

Refresh source/MIR observation before selecting another keeper or MIR-builder
probe.

## Required Output

```text
output_contract=hako-mimalloc-post-hako-reason-bind-source-mir-refresh-v0
input_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0
selected_owner
selected_next
selected_next_kind=box_count|box_shape|mir_diagnostic|measurement
winner_claim=0
summary=ok
```

## Stop Line

Do not apply another optimization in this row.
