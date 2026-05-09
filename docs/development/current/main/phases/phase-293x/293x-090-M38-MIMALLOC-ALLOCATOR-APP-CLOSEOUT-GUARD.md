---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M38 mimalloc allocator app closeout guard
---

# 293x-090 M38 Mimalloc Allocator App Closeout Guard

## Decision

`M38 mimalloc allocator app closeout guard` is live-narrow.

M38 closes the current mimalloc allocator app proof path as a regression
coverage contract. It adds no new substrate capability, source syntax, MIR route
row, NyRT export, allocator policy, or `.inc` semantic matcher.

The closeout truth is:

```text
M20-M37 app proofs stay individually owned.
M38 owns only the inventory/coverage guard.
```

The guard checks that every active mimalloc allocator app proof from M20 through
M37 still has:

- an app directory with `main.hako`, `README.md`, and `test.sh`;
- a matching `tools/checks/k2_wide_mimalloc_*_guard.sh` entry;
- a docs index entry;
- a `dev_gate.sh quick` entry;
- no app-specific matcher in `lang/c-abi/shims`.

## Owned

- `tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh`
- docs/taskboard status for M38.
- quick-gate coverage assertion for the existing mimalloc app guards.

## Not Owned

- New `.hako` app fixture.
- New MIR route row.
- New runtime export.
- Pointer load/CAS/fetch_add activation.
- Native pointer attrs or noalias/nonnull widening.
- Re-running every mimalloc EXE guard inside the closeout guard.

## Why It Does Not Re-run Every EXE Guard

`dev_gate.sh quick` already runs the individual M20-M37 guards. Re-running all
of them from M38 would make daily checks duplicate expensive EXE work. Instead,
M38 makes the coverage itself fail-fast: if a future edit removes an allocator
app proof from quick, docs index, or the app directory, the closeout guard fails.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_allocator_closeout_guard.sh` passes.
