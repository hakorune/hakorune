---
Status: Current
Date: 2026-05-28
Scope: attribute receiver materialization copies after row182 shifted the dominant owner.
Blocker: RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN.md
  - tools/allocator/hako_mimalloc_receiver_materialization_attribution_probe.py
---

# 296x-183 Receiver Materialization Attribution Probe

## Purpose

After row182 reduced local-SSA expression copies, `receiver_materialization`
became the dominant MIR copy owner. This row classifies receiver copy chains
before any receiver/pinning optimization.

## Required Output

```text
output_contract=hako-mimalloc-receiver-materialization-attribution-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
receiver_attributed_copy_count=27
unique_receiver_copy_count=24
duplicate_receiver_attribution_count=3
page_hotpath_receiver_copy_count=13
other_receiver_copy_count=12
facade_result_receiver_copy_count=2
dominant_receiver_family=page_hotpath_helpers
dominant_receiver_chain_len=2
selected_receiver_policy=receiver_pin_chain_policy_selection
next_diagnostic=receiver_pin_chain_policy_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next row should not assume receiver sharing is the main issue:
duplicate attribution is only 3 copies. The likely owner is the receiver
pin-to-slot plus LocalSSA recv chain itself, especially on page-hotpath helper
calls.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_receiver_materialization_attribution_probe_guard.sh
```
