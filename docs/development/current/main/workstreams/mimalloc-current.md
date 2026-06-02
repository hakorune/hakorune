---
Status: Active
Date: 2026-06-02
Scope: active mimalloc migration, optimization, and provider-benchmark workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
---

# Mimalloc Current Workstream

This is the active restart card. It intentionally stays compact.

Full historical MIM-001..MIM-146 prose was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Use that archive for exact old evidence. Use this file for the current decision
surface, next task order, and parking lot.

## Goal

Keep proving that `.hako` mimalloc can be built, packaged, and compared against
C mimalloc without opening product allocator replacement prematurely.

Current focus:

```text
provider/DLL benchmark bridge
same-machine C mimalloc comparison
next owner selection from local evidence
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no full external benchmark corpus import
- no copied benchmark executables in git
- no provider activation as product default
- no allocator replacement claim
- no hook installation claim
- no production `#[global_allocator]` claim
- no winner claim
- no source syntax expansion unless a tracked reference decision accepts it

## Current Decisions

```text
parity/front:
  direct exact remains the .hako mimalloc optimization front

public/default front:
  compatibility reference only; not the parity owner

provider package:
  handoff artifact and benchmark smoke path only

LD_PRELOAD shim:
  experiment/measurement bridge only

Hakozuna mixed-ws:
  repo-local CRT fixture is connected
  compare same-machine system malloc / C mimalloc / optional Hakorune provider
  do not compare Ubuntu numbers horizontally while CPU differs

record/state direction:
  PageModel remains box owner
  primitive mutable state may become record-shaped residence only through
  RecordStateResidencePlanV0-style metadata/plans

Inline(required):
  small receiver-local leaf helper only
  multi-block hot paths use HotCore/direct-exact plans instead
```

## Current Evidence Anchors

Recent completed slices:

```text
MIM-116..MIM-123:
  state bucket / record-state residence / hako_check state-explain /
  source-surface responsibility cleanup

MIM-124..MIM-134:
  owner-first perf refreshes, body-timer control, workload matrix,
  direct-exact app perf/asm tooling, observer-light closeout

MIM-135..MIM-146:
  provider package explicit measurement, hakmem and hakozuna LD_PRELOAD
  bridge, generated global allocator smoke, provider export bundle,
  same-machine Hakozuna mixed-ws C mimalloc comparison
```

Latest local benchmark bridge:

```text
tool:
  tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py

fixture:
  benchmarks/external/hakozuna/mixed-ws/build/bench_mixed_ws_crt

reference_subject:
  c_mimalloc_ldpreload

optional_subject:
  hakorune_provider_ldpreload when --manifest is passed

report_contract:
  hakozuna-mixed-ws-ldpreload-compare-v0

interpretation:
  provider subject is provider ABI wrapper + LD_PRELOAD shim bridge evidence
  provider_ldpreload_is_hako_core_speed_claim=0
  provider_ldpreload_is_product_allocator_claim=0

key shim diagnostic:
  shim_init_real_fallback_per_provider_operation

owner hint:
  provider_alloc_free_internal_real_malloc_boundary when init fallback
  dominates provider operations
```

## Next Task Order

1. Run a stable same-machine Hakozuna mixed-ws comparison.
   - Use enough iterations/samples to reduce startup noise.
   - Compare system malloc, C mimalloc LD_PRELOAD, and optional Hakorune
     provider LD_PRELOAD.
   - Treat the result as local evidence only.

2. Classify the remaining gap.
   - If provider shim counters dominate, optimize shim/provider boundary first.
   - If `.hako` allocator core dominates, return to direct-exact app perf/asm.
   - If benchmark setup noise dominates, improve measurement before code edits.
   - Do not call provider LD_PRELOAD evidence a `.hako` core speed result while
     `provider_ldpreload_is_hako_core_speed_claim=0`.

3. Continue provider replacement ladder only as smoke/readiness.
   - Keep `provider_activation=0`, `production_replacement_active=0`,
     `hook_installed=0`, `global_allocator_product_claim=0`,
     `winner_claim=0`.

4. Reopen `.hako` core optimization only with fresh owner evidence.
   - Candidate families: route-aware materialization/copy, HotCore direct-exact
     call boundary, record-state residence, DirectArray proof/lowering.
   - Do not source-hand-expand helpers to satisfy the compiler.

5. Keep docs lean.
   - Record small inventories in commit messages or this checklist.
   - Move full evidence prose to investigations/archive, not this active card.

## Daily Commands

Build the Hakozuna mixed-ws fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

Run same-machine C mimalloc comparison:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Add the optional Hakorune provider subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --out target/hakozuna-mixed-ws-compare-provider/report.out \
  --out-dir target/hakozuna-mixed-ws-compare-provider/artifacts \
  --sample-count 5
```

Run pointer guard after docs pointer edits:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Parking Lot

- DirectMemory / Span / Bytes / LayoutSpan remain future substrate work.
- `direct {}` remains parked; use RequiredFastPathRegion diagnostics first.
- `DirectArray<T>` generic source form remains parked; v0 source-visible type is
  concrete `DirectArrayI64`.
- `RecordStateResidencePlanV0` stays a narrow box-private primitive residence
  plan, not record-as-box or ordinary-box auto-recordification.
- Mixed-base helper extraction stays parked unless `EffectSummary` /
  `ReceiverSnapshotPublicationPlanV0` evidence selects it again.
- External Ubuntu benchmark numbers remain non-horizontal until CPU and run
  conditions are aligned.
