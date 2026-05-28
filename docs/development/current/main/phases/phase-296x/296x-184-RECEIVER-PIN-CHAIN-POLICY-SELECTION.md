---
Status: Current
Date: 2026-05-28
Scope: select the receiver pin-chain policy before another MIR optimization.
Blocker: RECEIVER-PIN-CHAIN-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-183-RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE.md
  - tools/allocator/hako_mimalloc_receiver_pin_chain_policy_selection.py
---

# 296x-184 Receiver Pin Chain Policy Selection

## Purpose

Select the next receiver-materialization policy from row183 attribution. This
row stays observe/select only and prevents guessing between same-receiver cache
and pin-chain narrowing.

## Required Output

```text
output_contract=hako-mimalloc-receiver-pin-chain-policy-selection-v0
receiver_attributed_copy_count=27
unique_receiver_copy_count=24
duplicate_receiver_attribution_count=3
page_hotpath_receiver_copy_count=13
selected_receiver_policy=receiver_pin_chain_narrowing
rejected_receiver_policy=same_receiver_callsite_cache
rejected_reason=duplicate_receiver_attribution_too_small
next_diagnostic=receiver_pin_chain_keeper_design
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next optimization should target the receiver pin-to-slot plus LocalSSA recv
chain, not a shared callsite receiver cache. Any keeper must stay narrow and
must not remove cross-block receiver safety.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_receiver_pin_chain_policy_selection_guard.sh
```
