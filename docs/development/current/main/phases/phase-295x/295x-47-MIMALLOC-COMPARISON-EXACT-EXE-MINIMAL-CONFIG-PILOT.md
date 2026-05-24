---
Status: Landed
Date: 2026-05-25
Scope: phase-295x exact-EXE minimal runtime config pilot.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-46-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT.md
---

# 295x-47 Exact-EXE Minimal Config Pilot

## Blocker

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001
```

## Decision

Add a comparison-runner-only runtime config profile:

```text
tools/allocator/hako_exe_memory_runner.sh --runtime-config empty
```

The default profile remains `root` and keeps current NyRT behavior. The new
`empty` profile runs the exact EXE from a generated temporary working directory
containing only:

```toml
[libraries]
```

This is the current safe bridge:

```text
hako.toml:
  remains the repo/root SSOT for configured plugin libraries

generated nyash.toml:
  remains the runtime artifact consumed by current NyRT exact-EXE startup
```

## Result

The runner now reports:

```text
runtime_config_profile=root|empty
```

Representative guard evidence for the empty comparison app:

```text
root_external_peak_rss_bytes  = 9,560,064
empty_external_peak_rss_bytes = 2,936,832
rss_reduction_bytes           = 6,623,232
```

This row does not claim a winner. It only enables the next row to compare root
config versus empty generated runtime config on selected phase-295x workloads.

## Follow-On

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001
```

## Stop Line

This row does not change default NyRT behavior, disable plugins by default,
alter provider selection, compute memory winners, open provider/DLL/replacement
/hook/global allocator seams, or require RSS parity.
