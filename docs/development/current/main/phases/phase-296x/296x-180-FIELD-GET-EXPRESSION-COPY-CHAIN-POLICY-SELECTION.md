---
Status: Current
Date: 2026-05-28
Scope: select the field-get expression copy-chain policy before optimization.
Blocker: FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-179-EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE.md
  - tools/allocator/hako_mimalloc_field_get_expression_copy_chain_policy_selection.py
---

# 296x-180 Field Get Expression Copy Chain Policy Selection

## Purpose

Select the next compiler-side policy owner after row179 showed that expression
materialization copies are overwhelmingly `field_get`-origin value chains. This
row is still observe/select only.

## Required Output

```text
output_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0
field_get_origin_copy_count=23
expression_materialization_copy_count=24
field_get_origin_ratio_bp=9583
compare_sink_copy_count=12
selected_chain_policy=field_get_direct_consumer_value_forwarding
selected_chain_policy_confidence=high
next_diagnostic=field_get_direct_consumer_forwarding_candidate_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next row should inspect whether MIR builder can forward field_get values to
direct consumers without emitting intermediate local-SSA copies. It should not
retry broad LocalSSA coalescing or source-level .hako rewrites.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_expression_copy_chain_policy_selection_guard.sh
```
