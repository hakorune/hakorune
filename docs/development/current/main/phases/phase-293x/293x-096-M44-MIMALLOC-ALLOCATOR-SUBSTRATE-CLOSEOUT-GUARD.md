---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M44 mimalloc allocator substrate closeout guard
---

# 293x-096 M44 Mimalloc Allocator Substrate Closeout Guard

## Decision

`M44 mimalloc allocator substrate closeout guard` is live-narrow.

M44 closes the M20-M43 mimalloc substrate proof ladder before production
allocator port work. It adds no new source syntax, MIR route row, NyRT export,
allocator policy, pointer `fetch_add`, native pointer attrs, or app-specific
backend matcher.

The closeout truth is:

```text
M20-M43 app proofs stay individually owned.
M44 owns only the substrate proof coverage guard.
Production allocator port work starts after this inventory is locked.
```

The guard checks that every active mimalloc substrate app proof from M20 through
M43 still has:

- an app directory with `main.hako`, `README.md`, and `test.sh`;
- a matching `tools/checks/k2_wide_mimalloc_*_guard.sh` entry;
- a docs index entry;
- a `dev_gate.sh quick` entry;
- no mimalloc app/policy-specific matcher in `lang/c-abi/shims`;
- no native pointer `fetch_add` implementation row.

## Owned

- `tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh`
- docs/taskboard status for M44.
- quick-gate coverage assertion for the existing M20-M43 mimalloc substrate
  proof guards.

## Not Owned

- New `.hako` app fixture.
- New MIR route row.
- New runtime export.
- New `.inc` route emitter.
- Production allocator policy.
- Pointer `fetch_add` activation.
- Native pointer attrs or noalias/nonnull widening.
- Re-running every mimalloc EXE guard inside the closeout guard.

## Why It Does Not Re-run Every EXE Guard

`dev_gate.sh quick` already runs the individual M20-M43 guards. Re-running all
of them from M44 would duplicate expensive EXE work. Instead, M44 makes the
coverage itself fail-fast: if a future edit removes a substrate app proof from
quick, docs index, or the app directory, the closeout guard fails.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_allocator_substrate_closeout_guard.sh` passes.
