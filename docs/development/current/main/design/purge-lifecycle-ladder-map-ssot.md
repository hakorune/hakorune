---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Navigation map for the M192-M213 hako_alloc purge, recommit, lifecycle, and reclaim ladder.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/phases/phase-293x/README.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Purge Lifecycle Ladder Map SSOT

## Purpose

This document is a navigation map for the purge/lifecycle ladder.
It prevents later rows from bypassing the existing seams or reopening closed
behavior.

The active blocker after M212 is:

```text
M213 abandoned/reclaim inventory
```

This map does not implement M213.

## Core flow

```text
raw page facts
  -> read-only purge policy
  -> dry-run observation
  -> bounded decommit seam
  -> marker / duplicate guard
  -> reuse precondition
  -> bounded recommit seam
  -> marker transition / page reactivation
  -> lifecycle vocabulary
  -> reuse priority
  -> stats observer
  -> lifecycle EXE hardening
  -> purge candidate inventory
  -> bounded scheduler
  -> abandoned/reclaim inventory
```

## Row map

| Row | Owner | Responsibility | Must not own |
| --- | --- | --- | --- |
| M192 | `purge_policy_box.hako` | classify raw local page facts into read-only purge/decommit decisions | page-source calls, heap/page mutation, OS release |
| M193 | `purge_dry_run_box.hako` | observe heap page/backing state and delegate to M192 | execution, mutation, OS release |
| M194 | `purge_execution_box.hako` | expose blocked/report-only purge execution attempt | actual decommit, source calls |
| M195 | `purge_bounded_decommit_box.hako` | call caller-provided decommit executor once for eligible bounded decisions | page-source APIs directly, heap mutation, unreserve |
| M196 | `purge_page_source_decommit_adapter_box.hako` | adapt bounded decommit to `HakoAllocPageSourcePolicy.decommitPage` only | reserve, commit, unreserve, OS release |
| M197 | `purge_heap_decommit_box.hako` | compose M193/M195/M196 for existing heap page/backing | heap/page mutation, marker state, direct scheduler |
| M198 | `purge_decommit_state_marker_box.hako` | record successful decommit reports as separate marker state | page-source calls, heap/page mutation, physical marker deletion |
| M199 | `purge_state_aware_decommit_box.hako` | prevent duplicate decommit by consulting M198 before M197 | direct page-source calls, unreserve, OS release |
| M200 | `purge_decommitted_page_reuse_precondition_box.hako` | classify decommitted pages as requiring recommit before reuse | recommit execution |
| M201 | `purge_recommit_failfast_box.hako` | report explicit recommit attempts while blocked/no-op | source execution |
| M202 | `purge_bounded_recommit_box.hako` | call caller-provided commit executor once after M200 requires recommit | marker clear, heap mutation, page-source direct calls |
| M203 | `purge_page_source_recommit_adapter_box.hako` | adapt bounded recommit to `HakoAllocPageSourcePolicy.commitPage` only | decommit, reserve, unreserve, OS release |
| M204 | `purge_decommit_state_marker_box.hako` | record recommitted generations so marker state becomes generation-counted | physical marker deletion, heap/page mutation |
| M205 | `purge_recommit_heap_integration_box.hako` | compose M200/M202/M203/M204 and page-local reactivation | page sourcing, unreserve, OS release |
| M206 | `hako-alloc-reuse-proof-closeout-proof` | prove two-generation decommit/recommit/reuse loop | new allocator owner |
| M207 | `page_lifecycle_invariant_box.hako` | freeze active/retired/decommitted/recommitted-active observer vocabulary | verifier enforcement, mutation, execution |
| C194b | MIR verifier checker | verify selected M207 lifecycle report/function invariants | allocator behavior |
| M208 | `heap_reuse_priority_box.hako` | rank active, recommitted-active, retired-reactivate, fresh fallback | acquire/release/reactivate execution, sourcing |
| M209 | `lifecycle_stats_observer_box.hako` | snapshot M207/M208 counters read-only | trigger observation/selection, mutate state |
| M210 | proof-only EXE hardening | prove M195-M209 lifecycle path under pure-first EXE | new allocator owner |
| M211 | `purge_candidate_policy_box.hako` | classify supplied M207 lifecycle reports as future purge candidates | observe heap pages, schedule, execute |
| M212 | `purge_bounded_scheduler_box.hako` | bounded scan, M207 observe, M211 classify, M199 attempt one candidate | direct M197/M195/M196/page-source calls, unbounded loops |
| M213 | planned | abandoned/reclaim vocabulary inventory | threads, atomics expansion, reclaim execution, OS release |

## Current seam contract

### Candidate classification

```text
M207 observeHeapPage(...)
  -> M211 classifyLifecycleReport(report)
```

M211 must stay read-only.
It consumes a lifecycle report; it must not observe the heap itself.

### Bounded scheduler

```text
M212 run(heap, guard, max_scan_pages)
  -> M207 observeHeapPage(...)
  -> M211 classifyLifecycleReport(...)
  -> M199 attemptHeapPage(...) for at most one eligible candidate
```

M212 must not call M197, M195, M196, or page-source APIs directly.
M199 remains the duplicate-guarded execution seam.

### Recommit reuse loop

```text
M199 successful decommit
  -> M198 marked generation
  -> M200 requires recommit
  -> M202/M203 bounded commit
  -> M204 recommitted generation
  -> M205 page-local reactivate
```

M205 is the owner that may call `HakoAllocPageModel.reactivate()`.

## M213 entry contract

M213 should be an inventory row.

Recommended scope:

```text
abandoned page vocabulary
reclaim candidate vocabulary
thread-owner vocabulary as metadata only
read-only report/counters
no execution
```

M213 stop lines:

```text
no thread scheduling
no atomics expansion
no reclaim execution
no page-source calls
no unreserve
no OS release
no provider activation
no hooks
no process allocator replacement
no changes to allocation/release/realloc/reuse priority
```

If M213 starts needing execution, split the row.

## Backlog after M213

After M213 lands, create a closeout/map refresh row before opening new reclaim
execution work.

Suggested docs-only row:

```text
D204 purge lifecycle ladder closeout
```

Purpose:

```text
freeze M192-M213 owners
name remaining inactive surfaces
select the next blocker explicitly
```

