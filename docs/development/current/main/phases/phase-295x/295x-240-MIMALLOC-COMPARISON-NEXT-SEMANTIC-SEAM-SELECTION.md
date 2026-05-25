---
Status: Landed
Date: 2026-05-25
Scope: select the next allocator-facing semantic seam after closing process-repeat evidence.
Related:
  - docs/development/current/main/phases/phase-295x/295x-239-MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-240 Next Semantic Seam Selection

## Blocker

```text
MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002
```

## Decision

Return phase-295x to allocator-facing semantic work after closing the selected
process-repeat evidence pack. The next row must not add another benchmark-only
median row under the same runner, schema, measurement policy, stop-line, and
interpretation.

New rows are justified only when at least one of these changes:

```text
allocator semantic capability
workload family contract
runner/output contract
measurement policy
diagnostic interpretation
phase or carryover boundary
```

If only `workload_id` changes while the runner/schema/policy/stop-line stay the
same, the work belongs in an evidence pack or generated report, not a new row.

## Semantic Backlog

The remaining mimalloc-facing work is grouped by allocator meaning, not by
benchmark shape:

```text
A. remote-free production facade
   Use the existing worker/TLS/atomic/par-stress substrate evidence to move the
   remote-free policy toward a production-facing allocator facade.

B. abandoned-heap reclaim behavior
   Turn the abandoned-owner / abandoned-reclaim evidence into a clearer allocator
   behavior contract.

C. huge / OSVM / page-source / purge carryover
   Keep this as a heavier future lane unless a narrow blocker requires it now.

D. phase-295x closeout / carryover boundary
   Close the comparison-execution phase if no narrow semantic row is selected.
```

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002
```

This is the preferred next semantic seam because worker identity, TLS cache
slots, atomic routes, remote-free policy, thread-safe ABI, and native
multi-worker stress evidence already exist. The follow-on should decide the
smallest production-facing facade step without opening provider activation,
DLL/replacement/hook/global allocator seams, or broad source-level threading.

## Stop Line

This row does not add samples, add a benchmark workload, compute speed winners,
compute RSS winners, require timing parity, change runtime behavior, open
provider/DLL/replacement/hook/global allocator seams, or start huge/OSVM/purge
implementation.
