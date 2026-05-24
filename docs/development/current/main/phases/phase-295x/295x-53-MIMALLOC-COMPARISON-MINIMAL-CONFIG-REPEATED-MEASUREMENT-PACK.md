---
Status: Landed
Date: 2026-05-25
Scope: phase-295x full repeated measurement pack with minimal runtime config.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-52-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT.md
---

# 295x-53 Minimal Config Repeated Measurement Pack

## Blocker

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK-295X-001
```

## Decision

Run the selected comparison workload pack with the phase-295x repeated
measurement policy and explicit `.hako` minimal runtime config:

```text
sample_count=5
warmup_count=1
hako_runtime_config_profile=empty
winner_claim=0
```

This row produces stable repeated evidence for the minimal runtime profile
before presentation or winner-claim rows.

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
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

## Stop Line

This row does not compute RSS winners, require RSS parity, make `empty` the
default runtime profile, delete root plugin loading, teach NyRT to read
`hako.toml` directly, or open provider/DLL/replacement/hook/global allocator
seams.
