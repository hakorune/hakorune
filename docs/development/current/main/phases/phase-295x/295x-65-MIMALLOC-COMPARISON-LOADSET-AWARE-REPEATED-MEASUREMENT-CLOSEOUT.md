---
Status: Landed
Date: 2026-05-25
Scope: phase-295x loadset-aware repeated measurement closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/reference/runtime/plugin-loadsets.md
  - docs/development/current/main/phases/phase-295x/295x-64-MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK.md
---

# 295x-65 Loadset-Aware Repeated Measurement Closeout

## Blocker

```text
MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT-295X-001
```

## Decision

Close the loadset-aware repeated measurement pack.

The pack returned phase-295x to mimalloc comparison measurement with explicit
`.hako` runtime/loadset evidence:

```text
hako_runtime_config_profile=empty
hako_selected_loadset=empty
hako_plugin_load_policy=eager_selected
hako_selected_library_count=0
hako_missing_library_count=0
hako_loadset_preflight_ok=1
sample_count=5
warmup_count=1
winner_claim=0
```

Representative median RSS evidence from `295x-64`:

```text
representative-small-block-v0:
  hako_external_rss_median_bytes=2998272
  c_external_rss_median_bytes=3895296

representative-realloc-aligned-v0:
  hako_external_rss_median_bytes=2904064
  c_external_rss_median_bytes=3973120

representative-mixed-small-v0:
  hako_external_rss_median_bytes=3010560
  c_external_rss_median_bytes=3870720

representative-huge-ish-v0:
  hako_external_rss_median_bytes=2936832
  c_external_rss_median_bytes=8105984
```

These are repeated comparison evidence values only. They do not declare memory
winners.

## Follow-On

```text
MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION-295X-001
```

The follow-on should choose whether to prepare a presentation-only summary,
add another workload family, or open a narrow diagnostic for the next observed
gap. It should keep standalone packaging and winner claims closed unless a
separate row explicitly opens them.

## Stop Line

This row does not compute RSS winners, require RSS parity, add standalone route
evidence labels, change runtime behavior, make `empty` the default, or open
provider/DLL/replacement/hook/global allocator seams.
