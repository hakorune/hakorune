---
Status: Landed
Date: 2026-05-25
Scope: phase-295x minimal runtime config repeated comparison pack run.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-50-MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT.md
---

# 295x-51 Minimal Config Repeated Pack Run

## Blocker

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001
```

## Decision

Run the selected repeated comparison workload pack with explicit `.hako`
runtime config profile:

```text
--hako-runtime-config empty
```

This measures the same `.hako` exact-EXE workload families without unrelated
root plugin load cost while preserving the existing C mimalloc evidence route.

## Contract

`tools/allocator/mimalloc_repeated_measurement_runner.py` accepts:

```text
--hako-runtime-config root
--hako-runtime-config empty
```

The default remains `root`. The runner emits:

```text
hako_runtime_config_profile=<root|empty>
```

and validates that each `.hako` sample writes the same
`runtime_config_profile` to `hako-exe-memory-evidence-v0`.

## Evidence

The guard runs the selected pack with:

```text
sample_count=1
warmup_count=0
hako_runtime_config_profile=empty
winner_claim=0
```

The row is evidence-only. It does not compare winners or require RSS parity.

## Follow-On

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001
```

## Stop Line

This row does not make `empty` the default, change C mimalloc evidence, compute
RSS winners, require RSS parity, delete root plugin loading, teach NyRT to read
`hako.toml` directly, or open provider/DLL/replacement/hook/global allocator
seams.
