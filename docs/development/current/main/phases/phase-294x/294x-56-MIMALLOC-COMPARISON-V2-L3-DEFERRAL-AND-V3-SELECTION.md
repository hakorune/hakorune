---
Status: Landed
Date: 2026-05-23
Scope: V2 small-path L3 deferral and V3 realloc/aligned slice selection.
Blocker: MIMALLOC-COMPARISON-VSLICE-004
Related:
  - docs/development/current/main/phases/phase-294x/294x-55-MIMALLOC-COMPARISON-SMALL-PATH-SLICE-PILOT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-56 Mimalloc Comparison V2 L3 Deferral And V3 Selection

## Decision

Do not run a dedicated V2 exact-MIR EXE closeout immediately after the
small-path schema pilot.

The V2 pilot added no new backend route, no new C shim matcher, no new OSVM /
TLS / atomic substrate, and no provider or host allocator activation. Its guard
already fixes the VM output schema and MIR route contract for the existing
small-path owners.

Therefore, continue to V3 and batch representative L3 evidence at the vertical
slice closeout.

## Selected Next Row

```text
MIMALLOC-COMPARISON-VSLICE-005:
  V3 realloc/aligned comparison schema pilot
```

The V3 pilot should reuse M174-M178 behavior and publish stable comparison
fields for:

```text
same-class realloc
grow fallback alloc-copy-release
aligned small allocation
requested bytes
copied bytes
live handles
failure/reject reason counters
alignment metadata
```

## Stop Line

The V3 pilot must not open:

- byte-copy execution;
- remote free;
- TLS / worker-local behavior;
- atomics;
- OSVM/page-source behavior;
- provider activation;
- host allocator replacement;
- hooks;
- `#[global_allocator]`;
- C mimalloc execution;
- backend owner-name matchers.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
