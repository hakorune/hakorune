---
Status: Landed
Date: 2026-05-25
Scope: phase-295x loadset-aware repeated measurement pack.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/reference/runtime/plugin-loadsets.md
  - docs/development/current/main/phases/phase-295x/295x-63-MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT.md
---

# 295x-64 Loadset-Aware Repeated Measurement Pack

## Blocker

```text
MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK-295X-001
```

## Decision

Run the selected comparison workload pack again with the phase-295x repeated
measurement policy and explicit `.hako` empty loadset evidence:

```text
sample_count=5
warmup_count=1
hako_runtime_config_profile=empty
hako_selected_loadset=empty
hako_plugin_load_policy=eager_selected
hako_selected_library_count=0
winner_claim=0
```

This row returns the lane to mimalloc comparison measurement after the plugin
loadset / standalone reference detour. It does not add standalone route labels.

## Contract

The row uses:

```text
tools/allocator/mimalloc_repeated_measurement_runner.py \
  --sample-count 5 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --allow-ldconfig-discovery
```

It preserves:

```text
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
canonical_rss_collector=external-time
internal_rss_evidence=preserved
```

## Follow-On

```text
MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

## Stop Line

This row does not compute RSS winners, require RSS parity, add standalone route
evidence labels, change runtime behavior, make `empty` the default, or open
provider/DLL/replacement/hook/global allocator seams.
