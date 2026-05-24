---
Status: Landed
Date: 2026-05-25
Scope: phase-295x exact-EXE minimal runtime config closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-48-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE.md
---

# 295x-49 Exact-EXE Minimal Config Closeout

## Blocker

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001
```

## Decision

Close the root versus generated-empty runtime config evidence pack.

The selected comparison path is:

```text
hako.toml / package intent
  -> comparison runner runtime_config_profile
  -> generated minimal runtime nyash.toml for exact-EXE execution
```

`hako.toml` should remain the package-facing configuration SSOT. The NyRT
plugin host currently consumes `nyash.toml` load sets, so the comparison runner
is allowed to generate a narrow runtime `nyash.toml` for measurement. This is a
runner profile, not a default NyRT behavior change.

## Evidence

`295x-48` compared root versus generated-empty runtime config on selected
exact-EXE workloads and preserved workload identity, operation family,
operation sequence, free order, count evidence, requested bytes, and closed
provider/replacement/hook/global allocator seams.

Representative guard evidence:

```text
representative-empty-v0:
  root_external_peak_rss_bytes=9404416
  empty_external_peak_rss_bytes=2854912
  rss_reduction_bytes=6549504

representative-small-block-v0:
  root_external_peak_rss_bytes=9605120
  empty_external_peak_rss_bytes=2940928
  rss_reduction_bytes=6664192

representative-realloc-aligned-v0:
  root_external_peak_rss_bytes=9625600
  empty_external_peak_rss_bytes=2883584
  rss_reduction_bytes=6742016
```

## Follow-On

```text
MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001
```

The follow-on should document the comparison-runner runtime config profile
contract and keep the default `root` profile for normal execution.

## Stop Line

This row does not change default NyRT behavior, make `empty` config the default,
teach NyRT to read `hako.toml` directly, disable plugins globally, compute RSS
winners, require RSS parity, or open provider/DLL/replacement/hook/global
allocator seams.
