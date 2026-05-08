---
Status: done
Date: 2026-05-09
Scope: M12 mimalloc raw-page proof
---

# 293x-062 M12 Mimalloc Raw Page Proof

## Decision

M12 raw-page proof is live-narrow.

The accepted shape is a fixed raw page/free-list fixture that uses existing
capability rows and existing rune contract facts:

```text
RawBufCoreBox
  -> raw page byte allocation/free

RawArrayCoreBox
  -> explicit free-list slot append/load/store

Contract(no_alloc/no_safepoint)
  -> EffectPlan
  -> MIR verifier
```

This row proves the substrate consumer seam. It does not add allocator
fast-path backend lowering.

## Owned

- `apps/mimalloc-raw-page-proof/`
- `tools/checks/k2_wide_mimalloc_raw_page_proof_guard.sh`
- M12 docs status in the mimalloc capability taskboard and runtime substrate
  reference.

## Not Owned

- Regular VM execution of `hako_mem_alloc`.
- `@rune Profile(...)` parser acceptance.
- `@rune Capability(...)` parser acceptance.
- restricted `unsafe(...)` blocks.
- pointer arithmetic or native layout syntax.
- TLS / atomics.
- backend or `.inc` allocator fast-path special cases.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_raw_page_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard runs the app-local proof script, which:

- runs MIR verification with `NYASH_FEATURES=rune`
- emits MIR JSON
- checks `MiRawPageProof.acquireBlock/1` and
  `MiRawPageProof.releaseBlock/1` carry `effect_plans`
- checks the fixture routes through `RawBufCoreBox` and `RawArrayCoreBox`
