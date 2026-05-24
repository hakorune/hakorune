---
Status: Landed
Date: 2026-05-24
Scope: close the phase-294x mimalloc comparison-quality vertical slice detour.
Blocker: PHASE-294X-MIMALLOC-COMPARISON-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-228-MIMALLOC-COMPARISON-VSLICE-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-243-MIMALLOC-COMPARISON-PHASE-CLOSEOUT-SELECTION.md
---

# 294x-244 Mimalloc Comparison Closeout

## Decision

Close `PHASE-294X-MIMALLOC-COMPARISON-CLOSEOUT-001`.

The phase-294x mimalloc-facing detour has reached its intended comparison
quality target. It produced and refreshed a stable `.hako` / `hako_alloc`
vertical-slice report chain and aligned it with the explicit C mimalloc runner
evidence without opening provider activation, host replacement, hooks, DLL
packaging, worker/TLS, atomics, remote-free stress, or native allocator
replacement.

## Validated Evidence

The refreshed chain includes:

- V5 `.hako` / C mimalloc schema alignment:
  `k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh`
- explicit C mimalloc runner pilot/closeout:
  `k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2`
  and
  `k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_closeout_guard.sh`
- result ledger, diagnostics, summary, reporting, and closeout guards through
  MIMAP-461A
- first-conclusion preflight, pilot, and closeout guards through MIMAP-470A
- presentation-only and presentation follow-on/extension guards through
  MIMAP-500A

The latest stable V5 evidence shape remains:

```text
schema=vertical-slice-v1
hako_slices=1,1,1
hako_requested=48,216,4194321,4194585
hako_evidence=4194433,7,4,6,6,0
c_mimalloc=1,1,1,1,64,64,33254,4096,4096,0,1
summary=ok
```

## Parked Work

Keep parked until a later row explicitly reopens them:

- deeper presentation-only extension rows after MIMAP-500A;
- broad report mirror / bool flag / status reason / signed sentinel / identity
  payload migration;
- provider package / DLL generation;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- true worker/TLS, atomics, remote-free stress, abandoned heap stress, and
  native allocator replacement.

## Next Row

Select `STAGEB-PARSER-LITERAL-SUFFIX-ALIGNMENT-001` as the next blocker.

This returns phase-294x to the usize semantic foundation backlog. The next
small implementation row should align the Stage-B `.hako` parser with the Rust
parser for numeric literal suffixes such as `0usize` before broader parameter
or return type annotation parsing.

## Stop Line

The next row should not migrate additional hako_alloc field groups or reopen
mimalloc comparison/presentation rows. It should be a parser-front alignment
row only.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
