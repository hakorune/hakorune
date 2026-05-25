---
Status: Active
Date: 2026-05-24
Scope: taskboard for phase-295x mimalloc comparison execution.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/README.md
---

# 295x-90 Mimalloc Comparison Execution Taskboard

## Rule

One row should open one comparison seam. Do not mix comparison evidence work
with provider activation, host allocator replacement, DLL packaging, atomics,
worker/TLS, or broad exact `usize` field migration.

## Current Truth

- Phase-294x closed the exact `usize` comparison-quality slice at `294x-270`.
- The `.hako` / `hako_alloc` V5 vertical-slice evidence is stable after the
  page-heap non-id exact `usize` closeout.
- The next work should resume mimalloc-facing development from explicit
  comparison evidence, not from broad allocator field drainage.
- Current comparison rows are contract/evidence rows. They do not make
  performance or memory winner claims until a later apples-to-apples repeated
  benchmark row defines workload equivalence, warmup, repetitions, and summary
  statistics.

## Current Blocker

```text
MIMALLOC-COMPARISON-PAR-STRESS-SELECTION-295X-002:
  Select the native multi-worker substrate stress as the next narrow concurrency seam after thread-safe hako_mem ABI selection.
```

## Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 194 | `MIMALLOC-COMPARISON-MALLOC-LARGE-BASELINE-BREAKDOWN-SELECTION-295X-001` | Landed | Select an empty exact-EXE footprint diagnostic for the external `malloc-large` path. |
| 195 | `MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-001` | Landed | Diagnose the fixed `malloc-large` empty exact-EXE baseline through evidence-output and no-output control pairs. |
| 196 | `MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001` | Landed | Observe the fixed `malloc-large` empty exact-EXE baseline into likely next seams. |
| 197 | `MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001` | Landed | Add env-gated NyRT self-RSS checkpoints to locate the fixed-cost step without changing runtime behavior when the env is unset. |
| 198 | `MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN-295X-002` | Landed | Run the env-gated NyRT self-RSS checkpoints on the empty no-output exact-EXE path and select the plugin-host baseline follow-on without changing runtime behavior when the env is unset. |
| 199 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002` | Landed | Close the plugin-host baseline selection row and open the narrow plugin-host substage diagnostic without changing runtime behavior when the env is unset. |
| 200 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002` | Landed | Add the narrow plugin-host substage diagnostic without changing runtime behavior when the env is unset. |
| 201 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002` | Landed | Select the load-set footprint diagnostic for the malloc-large path. |
| 202 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002` | Landed | Run the load-set footprint diagnostic for the malloc-large path. |
| 203 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002` | Landed | Close the load-set diagnostic and select a smaller default load set for exact-EXE no-output / comparison runs. |
| 204 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002` | Landed | Run repeated comparison samples with the empty default runtime config and compare them against explicit root compatibility. |
| 205 | `MIMALLOC-COMPARISON-WORKER-IDENTITY-SELECTION-295X-002` | Landed | Select the allocator-internal worker identity substrate as the next narrow concurrency seam after smaller-default load-set evidence. |
| 206 | `MIMALLOC-COMPARISON-WORKER-TLS-SELECTION-295X-002` | Landed | Select the allocator worker TLS cache-slot substrate as the next narrow concurrency seam after worker identity selection. |
| 207 | `MIMALLOC-COMPARISON-ATOMIC-ROUTE-SELECTION-295X-002` | Landed | Select the allocator-facing atomic route set as the next narrow concurrency seam after worker TLS cache-slot selection. |
| 208 | `MIMALLOC-COMPARISON-REMOTE-FREE-POLICY-SELECTION-295X-002` | Landed | Select the production-facade remote-free policy integration as the next narrow concurrency seam after allocator-facing atomic routes. |
| 209 | `MIMALLOC-COMPARISON-THREADSAFE-ABI-SELECTION-295X-002` | Landed | Select the thread-safe hako_mem ABI boundary as the next narrow concurrency seam after production-facade remote-free policy integration. |
| 210 | `MIMALLOC-COMPARISON-PAR-STRESS-SELECTION-295X-002` | Current | Select the native multi-worker substrate stress as the next narrow concurrency seam after thread-safe hako_mem ABI selection. |

## Full Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 0 | `295x-00` | Landed | Lock phase-295x and switch current pointers from phase-294x closeout to mimalloc comparison execution. |
| 1 | `295x-01` | Landed | Selected explicit C mimalloc evidence contract refresh as the first execution/evidence row. |
| 2 | `295x-02` | Landed | Validated stable output / memory-use evidence contract without benchmark repetition expansion. |
| 3 | `295x-03` | Landed | Selected the `.hako` vs C comparison ledger refresh. |
| 4 | `295x-04` | Landed | Consumed existing `.hako` vertical-slice and C runner evidence through the result ledger and diagnostics. |
| 5 | `295x-05` | Landed | Consolidated the comparison method and selected result ledger closeout. |
| 6 | `295x-06` | Landed | Closed the refreshed result ledger pack without winner claims. |
| 7 | `295x-07` | Landed | Executed the same-workload memory report path for representative-small-block-v0. |
| 8 | `295x-08` | Landed | Closed the same-workload execution refresh without benchmark/winner expansion. |
| 9 | `295x-09` | Landed | Selected repeated-run evidence refresh before adding a wider `.hako` port seam. |
| 10 | `295x-10` | Landed | Refreshed repeated same-workload RSS evidence without winner claims. |
| 11 | `295x-11` | Landed | Closed repeated-run evidence and selected count-evidence seam selection. |
| 12 | `295x-12` | Landed | Selected `.hako` allocation/free count evidence refresh. |
| 13 | `295x-13` | Landed | Surfaced `.hako` allocation/free counts in hako memory evidence. |
| 14 | `295x-14` | Landed | Closed matching `.hako`/C allocation-free count evidence. |
| 15 | `295x-15` | Landed | Selected representative-realloc-aligned-v0 as the next same-workload family. |
| 16 | `295x-16` | Landed | Added realloc/aligned workload contract and optional evidence fields. |
| 17 | `295x-17` | Landed | Added a narrow exact-EXE `.hako` realloc/aligned evidence app. |
| 18 | `295x-18` | Landed | Ran C mimalloc and `.hako` realloc/aligned same-workload evidence through the normalizer. |
| 19 | `295x-19` | Landed | Closed the realloc/aligned workload family. |
| 20 | `295x-20` | Landed | Selected representative-mixed-small-v0 as the next mixed-size workload family. |
| 21 | `295x-21` | Landed | Added C runner and `.hako` evidence contract support for representative-mixed-small-v0. |
| 22 | `295x-22` | Landed | Ran mixed-size same-workload evidence through the normalizer. |
| 23 | `295x-23` | Landed | Closed the mixed-size workload family. |
| 24 | `295x-24` | Landed | Selected representative-huge-ish-v0 as the next huge-ish workload family. |
| 25 | `295x-25` | Landed | Added C runner and `.hako` evidence contract support for representative-huge-ish-v0. |
| 26 | `295x-26` | Landed | Ran huge-ish same-workload evidence through the normalizer. |
| 27 | `295x-27` | Landed | Closed the huge-ish workload family. |
| 28 | `295x-28` | Landed | Defined repeated measurement policy before winner claims. |
| 29 | `295x-29` | Landed | Implemented repeated evidence runner without winner claims. |
| 30 | `295x-30` | Landed | Ran selected workload pack without winner claims. |
| 31 | `295x-31` | Landed | Closed repeated measurement pack. |
| 32 | `295x-32` | Landed | Added presentation-only repeated measurement report. |
| 33 | `295x-33` | Landed | Selected RSS gap attribution plan. |
| 34 | `295x-34` | Landed | Added empty/baseline repeated evidence. |
| 35 | `295x-35` | Landed | Computed baseline-subtracted RSS gap evidence. |
| 36 | `295x-36` | Landed | Closed RSS gap attribution pack. |
| 37 | `295x-37` | Landed | Selected empty exact-EXE footprint diagnostic. |
| 38 | `295x-38` | Landed | Observed empty exact-EXE RSS and static/loadable footprint. |
| 39 | `295x-39` | Landed | Closed empty exact-EXE footprint diagnostic. |
| 40 | `295x-40` | Landed | Added env-gated NyRT self-RSS checkpoints. |
| 41 | `295x-41` | Landed | Ran empty no-output exact-EXE checkpoint diagnostic. |
| 42 | `295x-42` | Landed | Selected plugin-host substage RSS diagnostics. |
| 43 | `295x-43` | Landed | Added and ran plugin-host substage RSS checkpoints. |
| 44 | `295x-44` | Landed | Selected generated-config plugin load-set footprint diagnostic. |
| 45 | `295x-45` | Landed | Ran generated-config plugin load-set RSS diagnostic. |
| 46 | `295x-46` | Landed | Closed plugin load-set diagnostic and selected exact-EXE minimal config pilot. |
| 47 | `295x-47` | Landed | Added comparison-runner-only exact-EXE minimal runtime config pilot. |
| 48 | `295x-48` | Landed | Compared root versus generated-empty runtime config exact-EXE evidence. |
| 49 | `295x-49` | Landed | Closed `MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001` and selected runtime config profile contract. |
| 50 | `295x-50` | Landed | Documented `MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001` for comparison-runner runtime config profiles. |
| 51 | `295x-51` | Landed | Ran `MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001` with explicit empty runtime config. |
| 52 | `295x-52` | Landed | Closed `MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001` and selected full repeated measurement pack. |
| 53 | `295x-53` | Landed | Ran `MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK-295X-001` with full repeated measurement policy. |
| 54 | `295x-54` | Landed | Closed the explicit empty runtime config full repeated measurement pack. |
| 55 | `295x-55` | Landed | Defined `MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT-295X-001` before changing default plugin loading behavior. |
| 56 | `295x-56` | Landed | Added `MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001` no-dlopen plugin loadset preflight plan artifact. |
| 57 | `295x-57` | Landed | Closed `MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT-295X-001` and selected runner loadset evidence. |
| 58 | `295x-58` | Landed | Added `MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-295X-001` selected-loadset fields to repeated comparison evidence. |
| 59 | `295x-59` | Landed | Closed `MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001` selected-loadset fields in repeated comparison evidence. |
| 60 | `295x-60` | Landed | Defined `MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001` standalone EXE route contract. |
| 61 | `295x-61` | Landed | Selected `MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-295X-001` reference-doc alignment before standalone implementation work. |
| 62 | `295x-62` | Landed | Added runtime reference docs for plugin loadsets and standalone EXE routes. |
| 63 | `295x-63` | Landed | Closed `MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001` and returned to comparison measurement. |
| 64 | `295x-64` | Landed | Ran `MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK-295X-001` with explicit loadset evidence. |
| 65 | `295x-65` | Landed | Closed `MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT-295X-001` repeated measurement evidence. |
| 66 | `295x-66` | Landed | Selected `MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001` before more mimalloc porting. |
| 67 | `295x-67` | Landed | Observed elapsed-time stability on selected repeated comparison workloads. |
| 68 | `295x-68` | Landed | Closed speed/stability observation and selected a high-resolution timing seam. |
| 69 | `295x-69` | Landed | Selected long process-repeat timing to escape the 1ms elapsed-time floor. |
| 70 | `295x-70` | Landed | Observed long process-repeat timing across selected comparison workloads. |
| 71 | `295x-71` | Landed | Closed long process-repeat timing evidence and selected a post-long-timing decision row. |
| 72 | `295x-72` | Landed | Selected presentation-only process timing before allocator-body timing or more `.hako` porting. |
| 73 | `295x-73` | Landed | Added process-repeat timing presentation evidence with allocator-body timing and winner claims closed. |
| 74 | `295x-74` | Landed | Closed process timing presentation and selected allocator-body timing contract work. |
| 75 | `295x-75` | Landed | Defined body timing vocabulary before C/.hako implementation. |
| 76 | `295x-76` | Landed | Added C-runner body timing for one workload while preserving process timing. |
| 77 | `295x-77` | Landed | Parked `.hako` body timing until a clock seam exists and returned to port seam selection. |
| 78 | `295x-78` | Landed | Selected reuse-cycle small-block as the next narrow `.hako` mimalloc port seam. |
| 79 | `295x-79` | Landed | Defined C/.hako reuse-cycle small-block workload contract. |
| 80 | `295x-80` | Landed | Implemented C/.hako evidence for reuse-cycle small-block workload. |
| 81 | `295x-81` | Landed | Closed reuse-cycle small-block workload evidence and selected the hakmem external benchmark bridge. |
| 82 | `295x-82` | Landed | Added a target-local bridge for the extracted hakmem mimalloc-bench corpus. |
| 83 | `295x-83` | Landed | Selected a narrow benchres.csv adapter for hakmem schema alignment. |
| 84 | `295x-84` | Landed | Added a narrow benchres.csv adapter for the extracted hakmem benchmark corpus. |
| 85 | `295x-85` | Landed | Closed the benchres adapter and selected the hakozuna_compare log adapter. |
| 86 | `295x-86` | Landed | Added a hakozuna_compare log adapter for repeated throughput/RSS evidence. |
| 87 | `295x-87` | Landed | Cataloged selected hakmem external artifacts and selected malloc-large workload alignment. |
| 88 | `295x-88` | Landed | Applied accepted record defaults / empty literal / `with` ergonomics to one guarded allocator-comparison owner. |
| 89 | `295x-89` | Landed | Batch-applied accepted record defaults / empty literal / `with` ergonomics to allocator-comparison owners that still used direct ReportFields literals. |
| 90 | `295x-187` | Landed | Defined the first `.hako` workload alignment contract against external `malloc-large`. |
| 188 | `MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001` | Landed | Run the external `mimalloc-bench` `malloc-large` corpus and the selected huge-ish `.hako` evidence through the comparison normalizer. |
| 189 | `MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001` | Landed | Close the external `malloc-large` evidence family and choose the next comparison seam. |
| 190 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001` | Landed | Select baseline attribution for the external `malloc-large` evidence family before any winner claim. |
| 191 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001` | Landed | Add empty-baseline evidence for the external `malloc-large` attribution family under the repeated measurement policy. |
| 192 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001` | Landed | Compute baseline-subtracted RSS evidence for the external `malloc-large` attribution family. |
| 193 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001` | Current | Close the external `malloc-large` memory-gap attribution pack and choose the next comparison seam. |
| 194 | `MIMALLOC-COMPARISON-MALLOC-LARGE-BASELINE-BREAKDOWN-SELECTION-295X-001` | Current | Select an empty exact-EXE footprint diagnostic for the external `malloc-large` path. |
| 91 | `295x-91` | Landed | Thin-wrapped allocator-provider guard families into root wrappers and `tools/checks/impl/` entries without changing the current mimalloc comparison blocker. |
| 92 | `295x-92` | Landed | Thin-wrapped the heaviest remaining mimalloc / hako_alloc guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 93 | `295x-93` | Landed | Thin-wrapped the remaining mimalloc facade huge guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 94 | `295x-94` | Landed | Thin-wrapped the remaining mimalloc facade huge page-model and decommit guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 95 | `295x-95` | Landed | Thin-wrapped the remaining mimalloc remote-free guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 96 | `295x-96` | Landed | Thin-wrapped the remaining mimalloc facade huge fail-fast guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 97 | `295x-97` | Landed | Thin-wrapped the remaining hako_alloc local-free reuse ledger release-apply and release-applied-recycle guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 98 | `295x-98` | Landed | Thin-wrapped the segment-map accepted-readiness consume-ledger guard root and aligned the consume-ledger closeout guard to the included proof / guard manifests without changing the current mimalloc comparison blocker. |
| 99 | `295x-99` | Landed | Thin-wrapped the remaining mimalloc facade huge page-source and decommit fail-fast guard roots into impl-backed wrappers while aligning the owner-contract checks to the arena-reclaim family contract. |
| 100 | `295x-100` | Landed | Thin-wrapped the MIMAP-161A release guard root and aligned the release closeout guard to the included proof and guard manifests without changing the current mimalloc comparison blocker. |
| 101 | `295x-101` | Landed | Thin-wrapped the huge/OSVM comparison slice guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker. |
| 102 | `295x-102` | Landed | Thin-wrapped the M173 pre-realloc release invariant guard root into an impl-backed wrapper and added the missing memory README owner note without changing the current mimalloc comparison blocker. |
| 103 | `295x-103` | Landed | Thin-wrapped the MIMAP-033A page-source unreserve adapter guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 104 | `295x-104` | Landed | Thin-wrapped the M48 remote-free policy guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker. |
| 105 | `295x-105` | Landed | Thin-wrapped the MIMAP-451A explicit C mimalloc runner execution pilot guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker. |
| 106 | `295x-106` | Landed | Thin-wrapped the MIMAP-560A result presentation-only extension pilot guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker. |
| 107 | `295x-107` | Landed | Thin-wrapped the M213 abandoned/reclaim inventory guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 108 | `295x-108` | Landed | Thin-wrapped the MIMAP-021B facade page-source fresh-page guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 109 | `295x-109` | Landed | Thin-wrapped the MIMAP-265A segment arena backing modeled source accounting diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 110 | `295x-110` | Landed | Thin-wrapped the C205b allocator record construction/read lowering guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker. |
| 111 | `295x-111` | Landed | Thin-wrapped the MIMAP-240A segment arena backing scalar requirement matrix guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 113 | `295x-113` | Landed | Thin-wrapped the MIMAP-284A segment arena backing modeled allocation ledger release intent guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 114 | `295x-114` | Landed | Thin-wrapped the M172 page-map-backed release seam guard root into an impl-backed wrapper and kept the memory owner note in sync without changing the current mimalloc comparison blocker. |
| 115 | `295x-115` | Landed | Thin-wrapped the M296A second-release diagnostic guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 116 | `295x-116` | Landed | Thin-wrapped the M293A diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 117 | `295x-117` | Landed | Thin-wrapped the M285A release-intent diagnostics guard root into an impl-backed wrapper and kept the memory owner note in sync without changing the current mimalloc comparison blocker. |
| 118 | `295x-118` | Landed | Thin-wrapped the M277A diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 119 | `295x-119` | Landed | Thin-wrapped the M281A release-candidate diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 120 | `295x-120` | Landed | Thin-wrapped the M289A release-apply diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 121 | `295x-121` | Landed | Thin-wrapped the M288A release-apply guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 122 | `295x-122` | Landed | Thin-wrapped the M301A lifecycle-continuation bridge diagnostics guard root into an impl-backed wrapper and kept the memory README owner note plus proof manifest entry in sync without changing the current mimalloc comparison blocker. |
| 124 | `295x-124` | Landed | Thin-wrapped the M174 no-move realloc guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 125 | `295x-125` | Landed | Thin-wrapped the M176 realloc failure-contract guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 126 | `295x-126` | Landed | Thin-wrapped the MIMAP-034A facade huge unreserve guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 127 | `295x-127` | Landed | Thin-wrapped the MIMAP-130A local-free reuse ledger guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 128 | `295x-128` | Landed | Thin-wrapped the MIMAP-292A release-applied recycle guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 129 | `295x-129` | Landed | Thin-wrapped the MIMAP-094A modeled ledger guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 130 | `295x-130` | Landed | Thin-wrapped the MIMAP-196A release bridge guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 131 | `295x-131` | Landed | Thin-wrapped the M212 bounded purge/decommit scheduler guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 132 | `295x-132` | Landed | Thin-wrapped the M195 bounded decommit execution policy guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 133 | `295x-133` | Landed | Thin-wrapped the M191 hako alloc stats surface guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 134 | `295x-134` | Landed | Thin-wrapped the M119A segment allocation modeled local-free integration guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 135 | `295x-135` | Landed | Thin-wrapped the M097A segment allocation modeled ledger release guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 136 | `295x-136` | Landed | Thin-wrapped the M115A segment allocation modeled local-free page-model apply guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 137 | `295x-137` | Landed | Thin-wrapped the M100A segment allocation modeled ledger released-token recycle guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 138 | `295x-138` | Landed | Thin-wrapped the M188A, M200A, and M204A segment-map local-free reuse bridge guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 139 | `295x-139` | Landed | Thin-wrapped the M224A, M228A, and M229A segment-map local-free reuse ledger lifecycle-keyed release guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 140 | `295x-140` | Landed | Thin-wrapped the M212A, M220A, and M216A segment-map local-free reuse ledger lifecycle-token guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 141 | `295x-141` | Landed | Thin-wrapped the M304A and M305A segment arena backing modeled allocation-ledger release/recycle continuation application bridge guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 142 | `295x-142` | Landed | Thin-wrapped the M164A and M168A segment-map modeled consume-ledger released-token recycle and released-span observation guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 143 | `295x-143` | Landed | Thin-wrapped the M42 and M43 remote-free policy and retry-loop guard roots into impl-backed wrappers without changing the current mimalloc comparison blocker. |
| 144 | `295x-144` | Landed | Thin-wrapped the M175 realloc alloc-copy-release guard root into an impl-backed wrapper and kept the current mimalloc comparison blocker unchanged. |
| 145 | `295x-145` | Landed | Thin-wrapped the MIMAP-TLS-001 worker TLS cache guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 146 | `295x-146` | Landed | Thin-wrapped the M211 purge candidate policy inventory guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 147 | `295x-147` | Landed | Thin-wrapped the M208 heap reuse priority policy guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 148 | `295x-148` | Landed | Thin-wrapped the MIMAP-149A segment allocation blocked-substrate matrix guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 149 | `295x-149` | Landed | Thin-wrapped the M192A segment-map local-free reuse ledger bridge guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 150 | `295x-150` | Landed | Thin-wrapped the MIMAP-018A facade stats snapshot guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 151 | `295x-151` | Landed | Thin-wrapped the MIMAP-153A segment-map lookup guarded readiness composition guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 171 | `295x-171` | Landed | Thin-wrapped the MIMAP-256A arena-slot inventory and MIMAP-257A arena-slot diagnostics guard roots into impl-backed wrappers and kept the memory README owner notes in sync without changing the current mimalloc comparison blocker. |
| 172 | `295x-172` | Landed | Thin-wrapped the MIMAP-068A reclaim scheduler request ledger guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 173 | `295x-173` | Landed | Thin-wrapped the MIMAP-151A segment-map scalar lookup boundary inventory guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 174 | `295x-174` | Landed | Thin-wrapped the MIMAP-088A segment allocation readiness scalar guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |
| 175 | `295x-175` | Landed | Thin-wrapped the MIMAP-012 object lifecycle queue guard root into an impl-backed wrapper and kept the memory README owner note in sync without changing the current mimalloc comparison blocker. |

## Parked

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration outside the comparison workload.

### Thread / remote-free migration ladder

If phase-295x reopens native allocator threading work, keep the seam order
small and explicit:

1. `MIMAP-WORKER-001` internal worker identity substrate.
2. `MIMAP-TLS-001` allocator-local TLS / worker-local cache-slot substrate.
3. `MIMAP-ATOMIC-001` allocator-facing atomic load/store/CAS/fetch_add routes.
   This row should extend the existing `hako_atomic_slot_*` and
   `hako_atomic_ptr_*` route vocabulary and guards first; add a new primitive
   only if a missing semantic cannot be expressed by the existing narrow leaf
   routes.
4. `MIMAP-REMOTE-001` remote-free / abandoned-owner / page ownership policy.
5. `MIMAP-THREADSAFE-ABI-001` thread-safe `hako_mem` ABI requirements and smoke boundary.
6. `MIMAP-PAR-STRESS-001` native multi-worker substrate stress for per-worker heaps and remote-free pressure.

Stop line for the ladder:

- no provider activation;
- no provider API execution;
- no process allocator replacement;
- no hooks or backend matcher shortcuts;
- no `#[global_allocator]`;
- no broad source-level thread semantics.
