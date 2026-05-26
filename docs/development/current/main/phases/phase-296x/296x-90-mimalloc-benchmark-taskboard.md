---
Status: Active
Date: 2026-05-27
Scope: taskboard for phase-296x mimalloc benchmark contract lane.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/README.md
---

# 296x-90 Mimalloc Benchmark Taskboard

## Rule

Benchmark contract work comes before DLL/provider work. Do not use this phase
to activate a provider, replace the process allocator, install hooks, or make
winner claims.

## Current Truth

- Phase-295x landed the first `.hako` mimalloc comparison/remote-free pass.
- The external `hakmem` corpus exists at:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

- The corpus includes benchmark binaries, source, historical `benchres.csv`,
  `hakozuna_compare` logs, perf data, and strace data.
- Phase-296x should make those assets usable through stable Hakorune-side
  contracts before DLL/provider work begins.

## Current Blocker

```text
MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001:
  Start provider package artifact Phase A by packaging an existing binary with a manifest.
```

## Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 0 | `MIMALLOC-BENCHMARK-LANE-LOCK-296X-001` | Landed | Open the benchmark contract lane and keep DLL/provider/replacement seams closed. |
| 1 | `MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001` | Landed | Inventory the external hakmem corpus and select first adapter rows. |
| 2 | `MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001` | Landed | Define the stable benchmark result vocabulary before parsing external logs. |
| 3 | `MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001` | Landed | Convert selected `benchres.csv` rows to Hakorune benchmark evidence with winner claims closed. |
| 4 | `MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001` | Landed | Convert selected `hakozuna_compare` logs to Hakorune benchmark evidence. |
| 5 | `MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001` | Landed | Run one already-landed `.hako` workload through a benchmark harness using the accepted result contract. |
| 6 | `MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001` | Landed | Close corpus adapter bring-up and select the first real repeated measurement row. |
| 7 | `MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001` | Landed | Run the selected same workload with process-repeat timing and repeated samples. |
| 8 | `MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001` | Landed | Select load-only DLL metadata smoke after benchmark contracts are stable. |
| 9 | `MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001` | Landed | Validate provider-package manifest/descriptor/hash metadata before shared-library loading. |
| 10 | `MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001` | Landed | Load a manifest-selected shared library and stop before export resolution, descriptor reads, provider calls, or allocator entrypoints. |
| 11 | `MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001` | Landed | Resolve and call only the descriptor export, leaving provider activation and allocator entrypoints closed. |
| 12 | `MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001` | Landed | Bind the provider API table while leaving explicit allocator calls and activation closed. |
| 13 | `MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001` | Landed | Call only a safe provider no-op while leaving allocator entrypoints and activation closed. |
| 14 | `MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE-296X-001` | Landed | Call explicit provider alloc/free while leaving process replacement and activation closed. |
| 15 | `MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT-296X-001` | Landed | Run repeated measurement through explicit provider alloc/free while leaving process replacement closed. |
| 16 | `MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001` | Landed | Close explicit provider measurement evidence and park activation work. |
| 17 | `MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001` | Landed | Define the 3-way hako/C/provider explicit comparison contract with winner claims closed. |
| 18 | `MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001` | Landed | Adapt landed hako/C/provider explicit measurement evidence into the 3-way comparison contract. |
| 19 | `MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001` | Landed | Close the 3-way comparison adapter and decide the next provider packaging lane. |
| 20 | `MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001` | Current | Start provider package artifact Phase A by packaging an existing binary with a manifest. |

## Mini-Agent Restart Queue

Pick exactly one slice.

### Slice 1 - Hakmem Asset Inventory

Purpose: make the external corpus readable from Hakorune docs without parsing
everything yet.

Allowed files:

- `docs/development/current/main/phases/phase-296x/296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY.md`
- optional tiny helper under `tools/allocator/` if the inventory row chooses to
  generate a report
- the row guard for the inventory row

Read-only external path:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Done when the row lists selected artifacts, rejects unsafe artifacts, and
chooses exactly one next adapter.

### Slice 2 - Result Contract

Purpose: define the stable output vocabulary before any parser is written.

Allowed files:

- a new `296x-02-*` card
- `docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md`
- a row guard

Required stop line:

```text
winner_claim=0
provider_active=0
replacement_active=0
global_allocator=0
```

### Slice 3 - Benchres Adapter

Purpose: parse one selected `benchres.csv` corpus family into the accepted
result contract.

Allowed files:

- `tools/allocator/*benchres*`
- a focused test/guard
- the active row card

Do not edit external corpus files.

### Slice 4 - Hakozuna Compare Adapter

Purpose: parse one selected `hakozuna_compare_*.log` family into the accepted
result contract.

Allowed files:

- `tools/allocator/*hakozuna_compare*`
- a focused test/guard
- the active row card

Do not open DLL/provider work.

### Slice 5 - Exact-EXE Repeated Measurement

Purpose: run the selected same workload with real repeated measurement policy.

Required policy:

```text
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

### Slice 6 - DLL Load-Only Selection

Purpose: only after adapter closeout, select a load-only DLL metadata smoke.

This slice is not eligible until rows 1-7 are landed.

### Slice 7 - DLL Metadata Preflight

Purpose: validate provider-package manifest/descriptor/hash metadata before
any shared-library load.

Required stop line:

```text
dll_mode=metadata-preflight
shared_library_load_executed=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

### Slice 8 - Shared-Library Load-Only Smoke

Purpose: load a manifest-selected shared library after metadata preflight.

Required stop line:

```text
dll_mode=load-only
shared_library_load_executed=1
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

Do not resolve exports, read descriptors, call provider APIs, or call allocator
entrypoints.

### Slice 9 - Descriptor-Read Smoke

Purpose: resolve and call only the descriptor export after load-only smoke.

Required stop line:

```text
dll_mode=descriptor-smoke
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

Do not bind the provider API or call allocator entrypoints.

### Slice 10 - Provider API Bind Smoke

Purpose: bind the provider API table after descriptor-read smoke.

Required stop line:

```text
dll_mode=provider-api-bind
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_api_bound=1
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

Do not call allocator entrypoints or activate the provider.

### Slice 12 - Provider Explicit Measurement Closeout

Purpose: close the explicit provider measurement ladder without opening
activation.

Required stop line:

```text
provider_activation_lane=parked
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

Selected next:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001
```

### Slice 13 - Provider Explicit Comparison Contract

Purpose: define a 3-way comparison contract for `.hako` exact-EXE, C mimalloc
explicit runner, and provider package explicit alloc/free evidence.

Required stop line:

```text
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### Slice 11 - Provider No-op Call Smoke

Purpose: call only a safe no-op provider function after API bind smoke.

Required stop line:

```text
dll_mode=provider-noop-call
provider_api_bound=1
provider_call_executed=1
provider_noop_call_executed=1
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

Do not call allocator entrypoints or activate the provider.
