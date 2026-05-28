---
Status: Active
Date: 2026-05-27
Scope: taskboard for phase-296x mimalloc benchmark contract lane.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
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
POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001:
  Measure exact-EXE timing after the field_get result-chain cleanup and select
  post-keeper owner refresh.
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
| 20 | `MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001` | Landed | Start provider package artifact Phase A by packaging an existing binary with a manifest. |
| 21 | `MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001` | Landed | Close existing-binary package helper and decide the next provider packaging lane. |
| 22 | `MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001` | Landed | Expose existing-binary provider package creation through the Hakorune CLI. |
| 23 | `MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001` | Landed | Close the Hakorune CLI provider package entry and decide the next package lane. |
| 24 | `MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001` | Landed | Document the stable v0 provider package command, output layout, and preflight verification path. |
| 25 | `MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001` | Landed | Close provider package v0 as functional by collecting CLI package, generated manifest preflight, docs, and gate evidence. |
| 26 | `MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001` | Landed | Select Phase B as a selected-provider-binary build/package lane, without opening activation or replacement. |
| 27 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001` | Landed | Define and pilot the smallest selected provider binary build/package contract, without opening .hako-to-shared-library generation, activation, or replacement. |
| 28 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001` | Landed | Close the selected provider binary build/package pilot evidence, without opening .hako-to-shared-library generation, activation, or replacement. |
| 29 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001` | Landed | Select the Phase C .hako-derived provider package build boundary, without opening activation, replacement, hooks, globals, or winner claims. |
| 30 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001` | Landed | Add the minimal selected .hako provider fixture package build pilot with source/MIR hashes, without opening activation, replacement, hooks, globals, or winner claims. |
| 31 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001` | Landed | Close the .hako-derived provider package pilot with descriptor/API-bind evidence, without opening semantic provider codegen, activation, replacement, hooks, globals, or winner claims. |
| 32 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001` | Landed | Select the smallest .hako semantic provider-codegen boundary, without opening activation, replacement, hooks, globals, or winner claims. |
| 33 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001` | Landed | Implement `ping-literal-v0`, mapping `.hako` `HakoProvider.ping/0` literal return into provider `hako_ping()` without opening allocator entrypoints or activation. |
| 34 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001` | Landed | Close the .hako semantic ping pilot with metadata/descriptor/API/noop evidence and select the next semantic entrypoint boundary. |
| 35 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001` | Landed | Select the smallest honest .hako semantic allocator entrypoint boundary after ping, without opening activation, replacement, hooks, globals, or winner claims. |
| 36 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001` | Landed | Implement alloc-free-owns-literal-v0 and prove explicit provider alloc/free plus .hako-owned owns policy through smoke evidence. |
| 37 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001` | Landed | Close alloc-free-owns-literal-v0 with metadata/descriptor/API/noop/alloc-free evidence and select the next semantic allocator boundary. |
| 38 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001` | Landed | Close .hako-derived provider package v0 as functional package artifact while keeping native pointer allocation mechanics and activation lanes separate. |
| 39 | `MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001` | Landed | Select the benchmark return row after .hako-derived provider package v0 functional closeout. |
| 40 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001` | Landed | Run .hako/C repeated measurement plus .hako-derived provider package explicit repeated measurement through the 3-way comparison adapter. |
| 41 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001` | Landed | Close the .hako-derived provider package explicit comparison evidence and select the next benchmark or activation decision row. |
| 42 | `HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001` | Landed | Select the `.hako` mimalloc performance-parity lane and keep hakozuna as reference-only. |
| 43 | `HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001` | Landed | Define the workload matrix and subject ids for `.hako` mimalloc, C mimalloc, hakozuna reference, and provider package evidence. |
| 44 | `HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001` | Landed | Run baseline repeated measurements for the first parity workload with winner claims closed. |
| 45 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001` | Landed | Classify each measured gap by primary owner, confidence, evidence quality, and next diagnostic before optimization work starts. |
| 46 | `HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001` | Landed | Follow row 45: measurement hygiene only for noisy harness evidence, otherwise choose the owner-specific narrow diagnostic. |
| 47 | `HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001` | Landed | Capture the selected owner diagnostic without optimizing or broadening the measurement contract unnecessarily. |
| 48 | `HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001` | Landed | Decide whether row 47 diagnostic evidence can enter optimization or needs another taxonomy pass. |
| 49 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001` | Landed | Refresh gap taxonomy over measurement hygiene evidence before any optimization work starts. |
| 50 | `HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001` | Landed | Decide the next action from refreshed gap taxonomy evidence. |
| 51 | `HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH-296X-001` | Landed | Refresh confidence for stable low-confidence hako_runtime_baseline evidence before optimization. |
| 52 | `HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC-296X-001` | Landed | Separate fixed runtime/process baseline cost from per-operation hako mimalloc cost before optimization. |
| 53 | `HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC-296X-001` | Landed | Split process-invocation scaling gap between empty runtime baseline and workload body cost before optimization. |
| 54 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT-296X-001` | Landed | Define the in-process operation-repeat measurement contract before optimization. |
| 55 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT-296X-001` | Landed | Run the first hako/C in-process operation-repeat pilot before optimization. |
| 56 | `HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION-296X-001` | Landed | Classify the first in-process hako/C gap before optimization. |
| 57 | `HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC-296X-001` | Landed | Split the first in-process hako workload gap between compiler lowering and allocator algorithm owners before optimization. |
| 58 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Landed | Apply exactly one evidence-backed optimization only when the owner is compiler_lowering or allocator_algorithm. |
| 59 | `HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed | Refresh in-process gap taxonomy after the first keeper optimization. |
| 60 | `HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION-296X-001` | Landed | Split the remaining in-process allocator-model cost into reset, alloc, and release phases before another optimization. |
| 61 | `HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION-296X-001` | Landed | Apply one acquire-side `.hako` allocator-model optimization after phase-cost ablation. |
| 62 | `HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed | Refresh in-process taxonomy after the second keeper optimization. |
| 63 | `HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH-296X-001` | Landed | Refresh phase-cost ablation after the second keeper optimization. |
| 64 | `HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION-296X-001` | Landed | Apply one known-active small-cycle `.hako` allocator-model optimization after post-second phase-cost refresh. |
| 65 | `HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed | Refresh in-process taxonomy after the third keeper optimization. |
| 66 | `HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY-296X-001` | Landed | Inventory missing mimalloc port pieces without mixing feature-port work into optimization rows. |
| 67 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001` | Landed | Select how much of real `.hako` mimalloc should be exposed through provider package explicit API. |
| 68 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001` | Landed | Pilot explicit provider-package calls through the selected real `.hako` mimalloc entrypoint. |
| 69 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001` | Landed | Select how to fuse the verified `.hako` entrypoint into the native provider-package artifact. |
| 70 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001` | Landed | Add and prove the first hako-derived provider semantic mode for the selected object-lifecycle entrypoint. |
| 71 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT-296X-001` | Landed | Measure the native-fusion provider package explicitly before LD_PRELOAD work. |
| 72 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001` | Landed | Decide whether to build a hakmem-compatible malloc/free export shim after explicit provider evidence. |
| 73 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001` | Landed | Build and smoke-test an optional LD_PRELOAD-compatible shim without enabling normal host allocator replacement. |
| 74 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001` | Landed | Pilot one hakmem benchmark compatibility check with the probe-only LD_PRELOAD shim. |
| 75 | `HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001` | Landed | Park selfhost handoff while the small-block gap remains large and select hako_check perf-surface inventory. |
| 76 | `HAKO-CHECK-PERF-SURFACE-CONTRACT-296X-001` | Landed | Define the observation-only hako_check perf-surface report contract. |
| 77 | `HAKO-CHECK-PERF-SURFACE-INVENTORY-296X-001` | Landed | Inventory objectLifecycleSmallAlloc/objectLifecycleReleaseBlock perf surfaces and select the first keeper candidate. |
| 78 | `HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH-296X-001` | Landed | Add one release known-page fast path keeper without widening replacement or winner claims. |
| 79 | `HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun the 8192-repeat in-process small-block measurement after the release keeper. |
| 80 | `HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION-296X-001` | Landed | Select the next single keeper from hako_check perf-surface evidence. |
| 81 | `HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001` | Landed | Implement the selected selectPage single-page fast path keeper. |
| 82 | `HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun the object-lifecycle facade exact-EXE measurement after the selectPage keeper. |
| 83 | `HAKO-CHECK-PERF-SURFACE-V1-MINIMAL-296X-001` | Landed | Add loop field/ArrayBox/allocation-like source counts plus confidence to hako_check source perf-surface. |
| 84 | `HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER-296X-001` | Landed | Compare keeper before/after source reports and measurement evidence without moving hako_check into optimizer responsibility. |
| 85 | `HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER-296X-001` | Landed | Add a Python MIR method shape adapter for selected MIR JSON methods outside hako_check core. |
| 86 | `HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER-296X-001` | Landed | Join hako_check source perf-surface and MIR method shape evidence for one selected method. |
| 87 | `HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION-296X-001` | Landed | Decide whether the Python MIR method shape contract is stable enough for a minimal .hako migration. |
| 88 | `HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001` | Landed | Apply source/MIR observation to multiple object-lifecycle methods and select the next keeper candidate. |
| 89 | `HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001` | Landed | Cache the page accepted by selectPage in objectLifecycleSmallAlloc instead of repeating pages.get in the caller-repeated hot method. |
| 90 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the small-alloc selected-page cache keeper. |
| 91 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the small-alloc selected-page cache keeper before selecting another keeper. |
| 92 | `HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001` | Landed | Cache the last allocated page object and reuse it in the known-page release path. |
| 93 | `HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the release known-page object cache keeper. |
| 94 | `HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after both selected-page cache keepers landed. |
| 95 | `HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER-296X-001` | Landed | Release directly through the cached page object on the known-page hot path while preserving fallback lookup. |
| 96 | `HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the release direct cached-page fast path keeper. |
| 97 | `HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the release direct cached-page fast path keeper. |
| 98 | `HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001` | Landed | Cache the first page object for the single-page select hot path. |
| 99 | `HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the select single-page first-page cache keeper. |
| 100 | `HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the select first-page cache keeper measurement. |
| 101 | `HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001` | Landed | Add an active-page field fast path inside the single-page select route. |
| 102 | `HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the active field fast path keeper. |
| 103 | `HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001` | Landed | Roll back the regressed active field fast path keeper while preserving the first-page cache keeper. |
| 104 | `HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after rolling back the active field fast path. |
| 105 | `HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the active field fast path rollback. |
| 106 | `HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001` | Landed | Bypass the selectPage wrapper from small alloc when the workload is single-page. |
| 107 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the small-alloc direct select keeper. |
| 108 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the small-alloc direct select keeper measurement. |
| 109 | `HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER-296X-001` | Landed | Inline small-alloc success result updates on the hot success path. |
| 110 | `HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after the inline success result keeper. |
| 111 | `HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001` | Landed | Roll back the regressed small-alloc inline success result keeper. |
| 112 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001` | Landed | Rerun object-lifecycle facade exact-EXE measurement after rolling back the inline success result keeper. |
| 113 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after rolling back the inline success result keeper. |
| 114 | `HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001` | Landed | Inspect lowered objectLifecycleSmallAlloc shape before selecting another keeper. |
| 115 | `HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE-296X-001` | Landed | Classify why objectLifecycleSmallAlloc lowers to high phi/copy counts. |
| 116 | `HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION-296X-001` | Landed | Select the MIR builder owner for single-incoming phi/copy elision. |
| 117 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001` | Landed | Define the guard surface for single-pred PHI elision before implementation. |
| 118 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001` | Landed | Implement guarded single-pred PHI elision and verify exact-EXE shape/measurement. |
| 119 | `HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001` | Landed | Classify the remaining multi-return/copy shape after single-pred PHI elision. |
| 120 | `HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001` | Landed | Classify local SSA copy materialization inside objectLifecycleSmallAlloc return blocks. |
| 121 | `HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001` | Landed | Classify duplicate reason global calls in objectLifecycleSmallAlloc failure return blocks. |
| 122 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE-296X-001` | Landed | Probe whether binding failure reasons once in .hako removes duplicate MIR reason calls. |
| 123 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER-296X-001` | Landed | Apply the .hako reason-local bind keeper for objectLifecycleSmallAlloc failure returns. |
| 124 | `HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT-296X-001` | Landed | Measure exact-EXE after the small-alloc .hako reason bind keeper. |
| 125 | `HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after the accepted .hako reason bind keeper. |
| 126 | `HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD-296X-001` | Landed | Add a narrow guard for duplicate facade reason-call evaluation before MIR builder changes. |
| 127 | `GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE-296X-001` | Landed | Add a generic MIR correctness fixture for nested argument single evaluation. |
| 128 | `MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX-296X-001` | Landed | Fix MIR builder nested argument single-evaluation correctness. |
| 129 | `MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-FIXTURE-296X-001` | Landed | Add a MIR correctness fixture for nested field access single evaluation. |
| 130 | `MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX-296X-001` | Landed | Fix field access lowering so inference does not re-lower nested object expressions. |
| 131 | `MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE-296X-001` | Landed | Add a MIR correctness fixture for env method fallback single evaluation. |
| 132 | `MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX-296X-001` | Landed | Fix env method lowering so unsupported env methods do not lower args before fallback. |
| 133 | `HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001` | Landed | Measure object-lifecycle facade exact-EXE after nested argument, field, and env single-eval fixes. |
| 134 | `MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP-296X-001` | Landed | Add a broader MIR builder single-evaluation surface sweep over field/index/print/typeop/constructor shapes. |
| 135 | `STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001` | Landed | Select the first verified static-scalar method fact boundary after single-eval correctness fixes. |
| 136 | `STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001` | Landed | Infer verified static-scalar facts for the selected reason getter family without lowering calls. |
| 137 | `STATIC-SCALAR-CALL-LOWERING-SELECTION-296X-001` | Landed | Select the exact call-lowering route and guard surface for verified static-scalar facts. |
| 138 | `STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION-296X-001` | Landed | Lower verified zero-arg static-scalar calls to constants through the selected route. |
| 139 | `POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001` | Landed | Measure object-lifecycle facade exact-EXE after static-scalar call lowering. |
| 140 | `POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR observation after static-scalar call lowering measurement. |
| 141 | `SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE-296X-001` | Landed | Classify remaining objectLifecycleSmallAlloc call/copy materialization after static-scalar lowering. |
| 142 | `MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION-296X-001` | Landed | Freeze the shared member-call owner behind static/env/this-me route selection before another keeper. |
| 143 | `MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT-296X-001` | Landed | Separate member-call route selection from emission without widening behavior or reopening generic CSE. |
| 144 | `MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP-296X-001` | Landed | Unify field/property receiver facts so nested receiver lowering keeps single-eval boundaries visible. |
| 145 | `MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT-296X-001` | Landed | Rerun single-eval and MIR verify surfaces after the BoxShape cleanup before returning to keeper selection. |
| 146 | `PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE-296X-001` | Landed | Measure page-local ArrayBox dynamic weight before selecting compiler helper lowering or page-model keeper work. |
| 147 | `PAGE-ARRAY-KEEPER-SELECTION-296X-001` | Landed | Select exactly one page-array keeper from dynamic weight evidence before returning to compiler helper lowering. |
| 148 | `RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION-296X-001` | Landed | Apply the direct cached-page known-live release keeper while preserving generic release fallback. |
| 149 | `POST-KNOWN-LIVE-RELEASE-MEASUREMENT-296X-001` | Landed | Measure exact-EXE after the direct cached-page known-live release keeper. |
| 150 | `POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR after known-live release measurement before choosing the next keeper. |
| 151 | `PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001` | Landed | Select one page-acquire keeper from the remaining page-local ArrayBox surface. |
| 152 | `SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION-296X-001` | Landed | Apply the selected small-alloc acquire_usize fast path keeper. |
| 153 | `POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001` | Landed | Measure exact-EXE after the small-alloc acquire_usize fast path keeper. |
| 154 | `POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001` | Landed | Refresh source/MIR after the small-alloc acquire_usize fast path measurement. |
| 155 | `MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM-296X-001` | Landed | Lower the remaining same-module helper setter calls without reopening the nested-call wrapper path. |
| 156 | `OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION-296X-001` | Landed | Attribute objectLifecycleSmallAlloc MIR copy pressure to callsite receiver/arg/result/local-SSA/phi-edge owners before another keeper row. |
| 157 | `CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS-296X-001` | Landed | Compare before/after callsite copy attribution reports before running exact-EXE measurements for another candidate. |
| 158 | `CALLSITE-COPY-OWNER-SELECTION-296X-001` | Landed | Select local SSA copy materialization as the next medium-confidence owner before opening another optimization row. |
| 159 | `LOCAL-SSA-COPY-BLOCK-POSITION-PROBE-296X-001` | Landed | Classify local-like copy positions after selecting local SSA copy materialization. |
| 160 | `EXPRESSION-MATERIALIZATION-OWNER-SELECTION-296X-001` | Landed | Select field_get result-chain cleanup as the expression materialization sub-owner. |
| 161 | `FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION-296X-001` | Landed | Select MirBuilder::build_field_access pin_to_slot cleanup as the narrow field_get result-chain owner. |
| 162 | `FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION-296X-001` | Landed | Apply field_get result-chain cleanup in MirBuilder::build_field_access and preserve semantic proof before timing measurement. |
| 163 | `POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001` | Landed | Measure exact-EXE timing after the field_get result-chain cleanup and select post-keeper owner refresh. |
| 164 | `POST-FIELD-GET-CLEANUP-OWNER-REFRESH-296X-001` | Landed | Refresh post-keeper MIR copy ownership and select the field_get result-chain follow-on probe. |
| 165 | `FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE-296X-001` | Landed | Classify remaining field_get result-chain copy consumers and select LocalSSA same-block reuse probing. |
| 166 | `LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION-296X-001` | Landed | Select LocalSSA same-block reuse in ensure_inner as the narrow compiler owner. |
| 167 | `LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION-296X-001` | Landed | Implement field_get-only LocalSSA same-block value reuse and preserve object-lifecycle semantic proof. |
| 168 | `POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT-296X-001` | Landed | Measure exact-EXE after field_get-only LocalSSA same-block reuse and select rollback after regression. |
| 169 | `ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE-296X-001` | Landed | Roll back the LocalSSA same-block field_get reuse non-keeper and restore the post-row162 baseline. |
| 170 | `POST-ROLLBACK-GAP-TAXONOMY-REFRESH-296X-001` | Landed | Stop optimization and classify the next owner as measurement contract gap: exact C object-lifecycle pair plus Hako body timing needed. |
| 171 | `OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT-296X-001` | Landed | Define the exact C object-lifecycle pair and comparable body timing contract before reopening optimization. |
| 172 | `OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN-296X-001` | Landed | Add the missing C mimalloc explicit object-lifecycle exact pair with body timing. |
| 173 | `OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN-296X-001` | Landed | Expose .hako exact-EXE object-lifecycle body timing through the existing env.now_ms seam. |
| 174 | `OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER-296X-001` | Landed | Join Hako exact-EXE and C mimalloc body timing evidence before reopening optimization. |
| 175 | `OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY-296X-001` | Landed | Classify the Hako/C body timing gap before selecting the next MIR/body owner diagnostic. |
| 176 | `OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION-296X-001` | Landed | Select the next MIR body owner from body-gap taxonomy and current attribution evidence. |
| 177 | `LOCAL-SSA-DYNAMIC-WEIGHT-PROBE-296X-001` | Landed | Estimate dynamic workload weight for the selected local-SSA MIR owner before optimization. |
| 178 | `LOCAL-SSA-COPY-KIND-POLICY-SELECTION-296X-001` | Landed | Select expression materialization copy policy as the next local-SSA diagnostic while rejecting same-block field-get reuse retry. |
| 179 | `EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-296X-001` | Landed | Classify expression-materialization copy origins before reopening optimization. |
| 180 | `FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-296X-001` | Landed | Select the field-get expression copy-chain policy before optimization. |
| 181 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-296X-001` | Landed | Count field_get direct-consumer forwarding candidates before optimization. |
| 182 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN-296X-001` | Landed | Implement narrow same-block FieldGet direct-consumer forwarding in LocalSSA. |
| 183 | `RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE-296X-001` | Landed | Attribute receiver materialization copies after row182 shifted the dominant owner. |
| 184 | `RECEIVER-PIN-CHAIN-POLICY-SELECTION-296X-001` | Landed | Select receiver pin-chain narrowing over same-receiver callsite cache. |
| 185 | `RECEIVER-PIN-CHAIN-NARROWING-KEEPER-296X-001` | Landed | Narrow receiver LocalSSA by reusing same-block Copy defs for receiver operands. |
| 186 | `OBJECT-LIFECYCLE-LARGE-OWNER-REALITY-CHECK-296X-001` | Landed | Stop copy-only optimization and select typed-object field access plus ArrayBox runtime helper cost as the next large owner. |
| 187 | `FIELD-ARRAY-RUNTIME-LOWERING-BOUNDARY-PROBE-296X-001` | Landed | Classify field/Array runtime lowering and select typed-object field helper fast lane as the next keeper family. |
| 188 | `TYPED-OBJECT-FIELD-HELPER-FAST-LANE-SELECTION-296X-001` | Landed | Select typed-object helper lock-cost probe before changing runtime or compiler behavior. |
| 189 | `TYPED-OBJECT-HELPER-LOCK-COST-PROBE-296X-001` | Landed | Quantify typed-object helper lock/global-slab cost before runtime fast-lane work. |
| 190 | `TYPED-OBJECT-STORAGE-BACKEND-SSOT-296X-001` | Landed | Define SafeMutexStore and SingleThreadExactStore boundaries before runtime fast-lane implementation. |
| 191 | `TYPED-OBJECT-RUNTIME-SINGLE-THREAD-FAST-LANE-296X-001` | Landed | Implement SingleThreadExactStore behind unchanged typed-object helper ABI. |
| 192 | `TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT-296X-001` | Landed | Measure SafeMutexStore versus SingleThreadExactStore on the object-lifecycle exact-EXE workload. |
| 193 | `MIR-TYPED-FIELD-RESIDENCE-SSOT-296X-001` | Landed | Define the MIR typed-field residence contract after the runtime fast-lane keeper. |
| 194 | `MIR-TYPED-FIELD-RESIDENCE-INVENTORY-296X-001` | Landed | Inventory MIR typed-field residence candidates before any transform. |
| 195 | `MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-PLAN-296X-001` | Landed | Build a selected-method field residence plan for HakoAllocPageModel.acquire_usize/1. |
| 196 | `MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-KEEPER-296X-001` | Landed | Reject the block-local selected-method typed-field residence implementation as a non-keeper. |
| 197 | `MIR-TYPED-FIELD-RESIDENCE-ERASURE-FEASIBILITY-296X-001` | Landed | Count net helper-call erasure before another typed-field residence implementation. |
| 198 | `CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION-296X-001` | Landed | Select CFG-aware typed-field residence design after block-local residence proves non-feasible. |
| 199 | `CFG-AWARE-TYPED-FIELD-RESIDENCE-SSOT-296X-001` | Landed | Define CFG-aware typed-field residence ownership before any transform. |
| 200 | `CFG-AWARE-TYPED-FIELD-RESIDENCE-PLAN-INVENTORY-296X-001` | Landed | Inventory CFG-aware typed-field residence net helper-call delta before implementation. |
| 201 | `LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET-296X-001` | Landed | Refresh the large owner after typed-field residence selected-method plans have zero net helper-call erasure. |
| 202 | `ARRAY-RUNTIME-SLOT-HELPER-SELECTION-296X-001` | Landed | Select the ArrayBox runtime slot helper diagnostic boundary before any keeper implementation. |
| 203 | `ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE-296X-001` | Landed | Split ArrayBox runtime slot helper cost before keeper implementation. |
| 204 | `ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-SSOT-296X-001` | Landed | Define the ArrayBox runtime single-thread store backend boundary before implementation. |
| 205 | `ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-IMPLEMENTATION-296X-001` | Landed | Implement the helper-side single-thread exact ArrayBox slot backend. |
| 206 | `ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT-296X-001` | Landed | Measure SafeRwLock versus SingleThreadExact on the object-lifecycle exact-EXE workload and decide keeper/revert. |
| 207 | `MIR-ARRAY-SLOT-RESIDENCE-SSOT-296X-001` | Current | Define ArraySlotResidencePlan / DirectSlotOp as the C-parity target after the runtime helper floor is known. |
| 208 | `MIR-ARRAY-SLOT-RESIDENCE-INVENTORY-296X-001` | Planned | Count erased ArrayBox get/set helper calls, added guards/writebacks, barriers, and net helper-call delta before any transform. |
| 209 | `MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-KEEPER-296X-001` | Planned | Apply a selected-method ArraySlotResidence keeper only if inventory shows positive net helper-call delta. |

## Hako Mimalloc Performance Parity Plan

SSOT:

```text
docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
```

The next benchmark return work must keep three decisions separate:

```text
.hako mimalloc parity:
  make the `.hako` mimalloc port approach C mimalloc under identical workload contracts

hakozuna reference:
  preserve hakozuna evidence as a comparison subject only

allocator product selection:
  parked until a separate decision row opens it
```

Do not turn parity rows into C ABI `malloc` replacement, provider activation,
global allocator integration, or hakozuna selection.

### Row 42 - Performance Parity Roadmap Selection

Purpose: close the provider-package comparison return and select the `.hako`
mimalloc performance-parity lane.

Required stop line:

```text
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
hakozuna_selection=0
```

### Row 43 - Workload Matrix

Purpose: define the exact workload matrix before running more benchmarks.

Required subjects:

```text
hako_mimalloc_exact_exe
c_mimalloc_explicit_runner
hakozuna_reference
provider_package_hako_mimalloc_explicit
```

Required workload ladder:

```text
small_block_alloc_free
realloc_aligned
remote_free_publish_collect
mixed_small
large_huge_backing
osvm_page_source
hakmem_selected_family
```

### Row 44 - Baseline Pack

Purpose: run the first same-workload baseline pack with the accepted repeated
measurement policy.

Required fields:

```text
same_workload=1
same_operation_count=1
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

### Row 45 - Gap Taxonomy Adapter

Purpose: classify benchmark gaps before any optimization.

Allowed primary owners:

```text
allocator_algorithm
compiler_lowering
hako_runtime_baseline
c_abi_memory_bridge
osvm_page_source
provider_wrapper
benchmark_harness
```

Required output contract:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
outlier_observed=0|1
evidence_quality=stable|noisy
gap_owner=<one primary owner>
gap_confidence=low|medium|high
next_diagnostic
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

Rows with low-confidence or noisy evidence cannot select an optimization.

### Row 46 - Conditional Diagnostic Selection

Purpose: branch from row 45 without spending time on unnecessary measurement
machinery.

Rules:

```text
if gap_owner=benchmark_harness:
  select measurement_hygiene_refresh

if gap_owner=hako_runtime_baseline:
  select empty_workload_or_repeat_scaling_runtime_diagnostic

if gap_owner=compiler_lowering:
  select mir_or_body_shape_diagnostic

if gap_owner=allocator_algorithm:
  select operation_repeat_scaling_or_allocator_counter_diagnostic

if gap_owner=c_abi_memory_bridge:
  select c_runner_api_or_load_boundary_diagnostic

if gap_owner=provider_wrapper:
  select provider_explicit_call_overhead_diagnostic
```

Measurement hygiene is optional and must be selected only when row 45 evidence
is noisy or harness-owned.

### Row 47 - Owner Narrow Diagnostic

Purpose: collect the narrow diagnostic selected by row 46.

Required evidence:

```text
front=<exact workload/front>
gap_owner=<one primary owner>
diagnostic_kind=<selected diagnostic>
body_elapsed_ns_secondary=0|1
build_compile_excluded=1 when measurement_hygiene_refresh
sample_count=5|7 only when measurement_hygiene_refresh
next_optimization_allowed=0|1
```

### Row 48 - Post Diagnostic Decision

Purpose: decide whether row 47 diagnostic evidence can enter the first keeper
optimization.

Rules:

```text
if next_optimization_allowed=1 and gap_owner in compiler_lowering,allocator_algorithm:
  select HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001

otherwise:
  select another diagnostic or taxonomy refresh row
```

Do not optimize in this row.

### Row 49 - Gap Taxonomy Refresh

Purpose: run the gap taxonomy adapter again over measurement hygiene evidence.

Required decision:

```text
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### Row 50 - Refreshed Taxonomy Decision

Purpose: decide the next action from refreshed gap taxonomy evidence.

Do not optimize in this row.

### Row 51 - Owner Confidence Refresh

Purpose: refresh confidence for stable low-confidence `hako_runtime_baseline`
evidence.

Do not optimize in this row.

### Row 52 - Runtime Baseline Scaling Diagnostic

Purpose: separate fixed runtime/process baseline cost from per-operation cost
after the empty workload confidence refresh.

Required ladder:

```text
workload_id=representative-small-block-v0
operation_repeat=128|1024|8192
sample_count=3
warmup_count=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

Do not optimize in this row.

### Row 53 - Runtime vs Workload Repeat Split Diagnostic

Purpose: split process-invocation scaling gap between empty runtime baseline
and workload body cost.

Required evidence:

```text
per_invocation_growth_observed=1
empty_workload_id=representative-empty-v0
small_workload_id=representative-small-block-v0
selected_gap_owner=compiler_lowering|allocator_algorithm|hako_runtime_baseline
selected_gap_confidence=low|medium|high
next_optimization_allowed=0|1
winner_claim=0
```

Do not optimize in this row.

### Row 54 - In-Process Operation Repeat Contract

Purpose: define a measurement contract where the workload loop repeats inside
one process rather than repeating process startup.

Required contract:

```text
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
process_repeat=<sample process count>
operation_repeat=<inner workload repeat>
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

Do not optimize in this row.

### Row 55 - In-Process Operation Repeat Pilot

Purpose: run the first measurement that repeats the allocator workload inside
one process.

Required output:

```text
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
timing_repeat_kind=in-process-operation-loop-v0
process_invocation_repeat=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

Do not optimize in this row.

### Row 56 - In-Process Gap Taxonomy Decision

Purpose: classify the first in-process hako/C gap before optimization.

Required output:

```text
output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0
gap_owner=compiler_lowering|allocator_algorithm|hako_runtime_baseline|benchmark_harness
gap_confidence=low|medium|high
next_optimization_allowed=0|1
winner_claim=0
```

Do not optimize in this row.

### Row 57 - Compiler/Allocator Owner Split Diagnostic

Purpose: split the first in-process hako workload gap between compiler lowering
and allocator algorithm owners.

Required output:

```text
selected_gap_owner=compiler_lowering|allocator_algorithm
selected_gap_confidence=low|medium|high
next_optimization_allowed=0|1
winner_claim=0
```

Do not optimize in this row.

### Row 58 - First Keeper Optimization

Purpose: make one optimization for one owner family and prove it with the same
parity gate.

Required owner:

```text
selected_gap_owner=allocator_algorithm
selected_gap_confidence=high
next_optimization_allowed=1
winner_claim=0
```

Only `compiler_lowering` and `allocator_algorithm` may enter this row directly.
Do not combine algorithm porting, compiler lowering, harness cleanup, and
provider wrapper cleanup in one row.

### Row 59 - Post Keeper Taxonomy Refresh

Purpose: refresh in-process gap taxonomy after the first keeper optimization.

Required output:

```text
previous_hako_external_elapsed_median_ms=330
current_hako_external_elapsed_median_ms=280
improvement_ms=50
remaining_gap_ms=276
winner_claim=0
```

### Row 60 - Phase Cost Ablation

Purpose: split the remaining in-process allocator-model gap into reset,
allocation, and release phases before choosing another optimization.

Required output:

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
reset_only_elapsed_median_ms
alloc_release_elapsed_median_ms
release_only_elapsed_median_ms
dominant_phase=reset|alloc|release|mixed
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not optimize in this row.

### Row 61 - Second Keeper Optimization

Purpose: optimize only the acquire-side allocator-model phase selected by row
60.

Required input:

```text
dominant_phase=alloc
next_optimization_target=acquire_usize_fast_path_and_invariant_hoist
next_optimization_allowed=1
winner_claim=0
```

Required output:

```text
optimization_kind
target_phase=alloc
before_full_elapsed_median_ms=280
after_full_elapsed_median_ms
improvement_ms
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not mix reset/release/provider/replacement work into this row.

### Row 62 - Post Second Keeper Taxonomy Refresh

Purpose: refresh in-process taxonomy after the second keeper optimization.

Required output:

```text
output_contract=hako-mimalloc-post-second-keeper-taxonomy-refresh-v0
current_hako_external_elapsed_median_ms=260
remaining_gap_ms
gap_owner
gap_confidence
next_diagnostic
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not optimize in this row.

### Row 63 - Post Second Phase Cost Refresh

Purpose: refresh phase costs after the second keeper optimization.

Required output:

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
full_elapsed_median_ms=260
dominant_phase=reset|alloc|release|mixed
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not optimize in this row.

### Row 64 - Third Keeper Optimization

Purpose: optimize only the known-active small-cycle allocator-model phase
selected by row 63.

Required output:

```text
optimization_kind
target_phase=known_active_small_cycle
before_full_elapsed_median_ms=250
after_full_elapsed_median_ms
improvement_ms
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not mix reset/release/provider/replacement work into this row.

### Row 65 - Post Third Keeper Taxonomy Refresh

Purpose: refresh in-process taxonomy after the third keeper optimization.

Required output:

```text
output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0
current_hako_external_elapsed_median_ms=240
remaining_gap_ms
gap_owner
gap_confidence
next_diagnostic
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not optimize in this row.

### Row 66 - Port Feature Gap Inventory

Purpose: list missing `.hako` mimalloc features separately from speed
optimization.

Required input:

```text
optimization_checkpoint=small_model_fast_path_plateau
current_hako_external_elapsed_median_ms=240
remaining_gap_ms=236
next_diagnostic=port_feature_gap_inventory
```

Required output:

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
small_model_checkpoint_elapsed_median_ms=240
missing_feature_count=7
primary_gap_kind=integration_surface_gap
next_port_feature=real_provider_explicit_entrypoint_selection
ld_preload_shim_ready=0
provider_entrypoint_selection_ready=1
winner_claim=0
```

Initial buckets:

```text
size_classes
local_free_list
remote_free_delayed_collect
realloc_aligned
page_segment_lifecycle
huge_backing
abandoned_or_cross_thread
osvm_purge_decommit
stats_or_diagnostics
```

Do not optimize in this row.

### Row 67 - Real Provider Entrypoint Selection

Purpose: decide which real `.hako` mimalloc API surface should be exposed
through explicit provider package calls.

Required input:

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
primary_gap_kind=integration_surface_gap
next_port_feature=real_provider_explicit_entrypoint_selection
```

Keep replacement parked:

```text
output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
selected_surface_owner=HakoAllocObjectLifecycleFacade
provider_call_allowed=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 68 - Real Provider Entrypoint Pilot

Purpose: call the selected real `.hako` mimalloc surface through the explicit
provider package lane before any LD_PRELOAD or replacement work.

Required input:

```text
selected_entrypoint=object_lifecycle_small_alloc_release_v0
selected_surface_owner=HakoAllocObjectLifecycleFacade
provider_call_allowed=1
```

Required stop line:

```text
provider_call_executed=1
hako_selected_entrypoint_executed=1
provider_package_native_fused_to_hako_entrypoint=0
provider_package_native_fusion_required=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 69 - Native Provider Package Fusion Selection

Purpose: select how to make the native provider-package artifact consume the
verified `.hako` object-lifecycle entrypoint instead of jumping directly to
LD_PRELOAD.

Required input:

```text
selected_entrypoint=object_lifecycle_small_alloc_release_v0
hako_selected_entrypoint_executed=1
provider_package_native_fused_to_hako_entrypoint=0
provider_package_native_fusion_required=1
```

Required stop line:

```text
native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0
provider_package_native_fusion_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 70 - Native Provider Package Fusion Pilot

Purpose: add the selected hako-derived semantic codegen mode and prove
provider alloc/free smoke with the object-lifecycle entrypoint call-chain
checked from MIR.

Required input:

```text
native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0
required_codegen_mode=object-lifecycle-small-alloc-release-v0
required_fixture=apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako
```

Required stop line:

```text
hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0
hako_entrypoint_mir_call_chain_verified=1
provider_alloc_executed=1
provider_free_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 71 - Native Fusion Explicit Provider Measurement

Purpose: measure the object-lifecycle native-fusion provider package through
explicit provider calls before deciding on LD_PRELOAD.

Required stop line:

```text
output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0
provider_explicit_measurement_ready=1
ld_preload_decision_ready=1
provider_call_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 72 - Hakmem LD_PRELOAD Shim Decision

Purpose: decide whether to open an LD_PRELOAD-compatible bridge for hakmem's
existing benchmark scripts.

The bridge is useful only after explicit provider API evidence is stable.

Required decision stop line:

```text
ld_preload_shim_decision=accepted
decision_scope=hakmem_compat_probe_only
provider_call_evidence_ready=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 73 - Hakmem LD_PRELOAD Shim Smoke

Purpose: build a shim that exposes malloc/free-family symbols for hakmem
compatibility and smoke-test it separately from normal Hakorune execution.

Required stop line:

```text
ld_preload_compatible=1
shared_library_load_executed=1
malloc_family_symbols_exported=1
hakmem_script_compatible=probe-only
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

Do not use this row to make Hakorune's own runtime use the shim by default.

### Row 74 - Hakmem LD_PRELOAD Bench Pilot

Purpose: run one probe-only hakmem compatibility sample with `LD_PRELOAD`
applied to a selected benchmark process.

Required stop line:

```text
hakmem_script_compatible=probe-only
ld_preload_env_applied=1
benchmark_sample_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

### Row 75 - Selfhost Handoff Gate

Purpose: decide whether allocator performance evidence is good enough to move
attention back toward selfhosting. Current plan is to park handoff and open a
hako_check perf-surface diagnostic because the small-block gap remains large.

Required closeout:

```text
selfhost_handoff_decision=parked
park_reason=hako_mimalloc_small_block_gap_still_large
remaining_allocator_gap_classified=1
next_diagnostic=hako_check_perf_surface_inventory
winner_claim=0
replacement_active=0
```

### Row 76 - hako_check Perf Surface Contract

Purpose: define an observation-only `hako_check perf-surface` report contract
before adding another allocator keeper.

Required output:

```text
output_contract=hako-check-perf-surface-contract-v0
target_file
target_box
target_method
method_call_count
loop_method_call_count
array_access_count
linear_search_candidate=0|1
result_capsule_churn=0|1
observer_call_count
hot_path_risk=low|medium|high
suggested_next
summary=ok
```

### Row 77 - hako_check Perf Surface Inventory

Purpose: apply the perf-surface contract to
`lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`.

Initial target methods:

```text
objectLifecycleSmallAlloc
objectLifecycleReleaseBlock
```

Expected first keeper selection:

```text
target_method=objectLifecycleReleaseBlock
linear_search_candidate=1
suggested_next=release_known_page_fast_path
```

### Row 78 - Release Known-Page Fast Path Keeper

Purpose: add exactly one `.hako` allocator-model keeper that avoids the hot
`objectLifecycleKnownPageIndexById` linear lookup when releasing the page just
allocated.

### Row 79 - Post Release-Keeper Measurement

Purpose: rerun the 8192-repeat in-process small-block measurement and compare
against the current small-block checkpoint. Winner claims remain closed.

### Row 80 - Next Keeper Selection

Purpose: select one next keeper from hako_check evidence.

Candidate queue:

```text
selectPage single-page fast path
result capsule hot-loop update reduction
observer getter reduction
ArrayBox get/length call reduction
```

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
