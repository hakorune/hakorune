---
Status: Landed
Date: 2026-05-25
Scope: phase-295x minimal runtime config full repeated measurement closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-53-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK.md
---

# 295x-54 Minimal Config Repeated Measurement Closeout

## Blocker

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

## Decision

Close the explicit empty runtime config full repeated measurement pack.

The pack ran the selected comparison workloads with:

```text
hako_runtime_config_profile=empty
sample_count=5
warmup_count=1
winner_claim=0
```

The row confirms that the generated minimal runtime config profile can carry
the current phase-295x comparison pack without reopening root plugin loading
or changing the default NyRT plugin-host behavior.

## Evidence

Representative `295x-53` evidence:

```text
representative-small-block-v0:
  hako_external_rss_median_bytes=2985984
  c_external_rss_median_bytes=3928064

representative-realloc-aligned-v0:
  hako_external_rss_median_bytes=2945024
  c_external_rss_median_bytes=3903488

representative-mixed-small-v0:
  hako_external_rss_median_bytes=3010560
  c_external_rss_median_bytes=3936256

representative-huge-ish-v0:
  hako_external_rss_median_bytes=2916352
  c_external_rss_median_bytes=8077312
```

These values are comparison evidence only. They do not declare memory winners.

## Follow-On

```text
MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT-295X-001
```

The follow-on should turn the plugin/load-set lesson into an explicit linking
contract before any default runtime behavior change. It should keep root
compatibility, require selected-loadset diagnostics for comparison profiles,
and keep provider/DLL/replacement/hook/global allocator seams parked.

## Stop Line

This row does not compute RSS winners, require RSS parity, make `empty` the
default runtime profile, delete root plugin loading, teach NyRT to read
`hako.toml` directly, or open provider/DLL/replacement/hook/global allocator
seams.
