---
Status: Landed
Date: 2026-05-25
Scope: phase-295x minimal runtime config repeated pack closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-51-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN.md
---

# 295x-52 Minimal Config Repeated Pack Closeout

## Blocker

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001
```

## Decision

Close the guard-fast minimal runtime config repeated pack run.

The selected `.hako` workload pack runs successfully with:

```text
hako_runtime_config_profile=empty
sample_count=1
warmup_count=0
winner_claim=0
```

## Evidence

Representative guard output from `295x-51`:

```text
representative-small-block-v0:
  hako_external_rss_median_bytes=2920448
  c_external_rss_median_bytes=3919872

representative-realloc-aligned-v0:
  hako_external_rss_median_bytes=2883584
  c_external_rss_median_bytes=3928064

representative-mixed-small-v0:
  hako_external_rss_median_bytes=3014656
  c_external_rss_median_bytes=3932160

representative-huge-ish-v0:
  hako_external_rss_median_bytes=2875392
  c_external_rss_median_bytes=8122368
```

These are evidence values only. They are not winner claims because the row uses
one sample and no warmup.

## Follow-On

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK-295X-001
```

The follow-on should run the same explicit empty profile with the phase-295x
repeated measurement policy (`sample_count=5`, `warmup_count=1`) before any
presentation or winner-claim row.

## Stop Line

This row does not compute RSS winners, require RSS parity, make `empty` the
default runtime profile, change root plugin loading, teach NyRT to read
`hako.toml` directly, or open provider/DLL/replacement/hook/global allocator
seams.
