---
Status: Landed
Date: 2026-05-25
Scope: phase-295x comparison-runner runtime config profile contract.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-49-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT.md
---

# 295x-50 Runtime Config Profile Contract

## Blocker

```text
MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001
```

## Decision

Use explicit comparison-runner runtime config profiles.

```text
root:
  default exact-EXE runtime profile.
  Runs from the repository/root environment and preserves existing NyRT
  plugin discovery behavior.

empty:
  comparison-runner-only minimal runtime profile.
  Runs from a generated temporary working directory with a generated
  nyash.toml containing an empty [libraries] set.
```

hako.toml remains the package-facing configuration intent. Current NyRT
plugin host execution still consumes `nyash.toml` load-set files, so the
runner may lower a selected profile to generated runtime `nyash.toml`. This
keeps the comparison lane from changing default NyRT behavior while still
measuring the exact-EXE workload without unrelated root plugin load cost.

## Contract

The runner contract is:

```text
tools/allocator/hako_exe_memory_runner.sh
  --runtime-config root
  --runtime-config empty
```

Rules:

```text
root is the default.
empty is opt-in.
unsupported profile names fail-fast.
runtime_config_profile is written to hako-exe-memory-evidence-v0.
empty profile must preserve workload identity and count evidence for selected
workloads that do not require plugin libraries.
```

## Follow-On

```text
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001
```

The follow-on may run selected comparison workloads with `--runtime-config
empty` as an explicit profile and keep winner claims closed.

## Stop Line

This row does not make `empty` the default, teach NyRT to read `hako.toml`
directly, delete root plugin loading, remove plugin packages, compute RSS
winners, require RSS parity, or open provider/DLL/replacement/hook/global
allocator seams.
