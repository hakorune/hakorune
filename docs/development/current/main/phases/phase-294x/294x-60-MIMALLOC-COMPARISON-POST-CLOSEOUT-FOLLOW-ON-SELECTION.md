---
Status: Landed
Date: 2026-05-23
Scope: post V5 comparison closeout follow-on selection.
Blocker: MIMALLOC-COMPARISON-VSLICE-008
Related:
  - docs/development/current/main/phases/phase-294x/294x-59-MIMALLOC-COMPARISON-VERTICAL-SLICE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-60 Mimalloc Comparison Post-Closeout Follow-On Selection

## Decision

Select hako-side memory-use evidence as the next narrow row.

The V5 closeout aligned the `.hako` hako_alloc schema with the existing C
mimalloc explicit runner planning surface, but it intentionally kept hako-side
RSS / memory-use evidence pending:

```text
hako_has_memory_use_evidence = 0
c_has_memory_use_evidence = 1
```

The next useful row is therefore a small external hako EXE memory-evidence
runner over the already validated comparison app shape. This is still not
provider activation, host allocator replacement, hook installation, or a native
allocator replacement claim.

## Selected Next Row

```text
MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001:
  add a hako pure-first EXE memory-use evidence runner that builds a selected
  comparison `.hako` app to an exact-MIR EXE, runs that EXE, records peak RSS /
  exit status / output-summary evidence, and keeps provider activation, host
  replacement, hooks, TLS, atomics, and allocator replacement parked.
```

## Stop Line

The next row may:

- build an already selected comparison `.hako` app into a pure-first EXE;
- run that EXE as an external process;
- record peak RSS / output-summary evidence.

The next row must not:

- load or replace the process allocator;
- install hooks;
- generate provider packages or DLLs;
- use `#[global_allocator]`;
- open TLS / worker-local behavior;
- open remote-free stress or atomic bitmap execution;
- claim performance or memory-use winner status.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
