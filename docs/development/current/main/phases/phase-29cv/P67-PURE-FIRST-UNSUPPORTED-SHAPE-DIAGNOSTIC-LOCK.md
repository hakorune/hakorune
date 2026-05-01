---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: lock pure-first unsupported-shape diagnostics before any backend fix.
Related:
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
---

# P67 Pure-First Unsupported-Shape Diagnostic Lock

## Goal

Make `unsupported pure shape for current backend recipe` actionable without
turning ny-llvmc into a broader shape-policy owner.

The current failure means MIR reached the backend boundary, but the active
`pure-first + compat_replay=none` recipe did not accept that shape. That
fail-fast is correct. The next fix must improve diagnosis only; it must not add
ad-hoc C matchers, silent fallback, or compat replay as mainline proof.

## Decision

- Reuse `NYASH_LLVM_ROUTE_TRACE=1` for the diagnostic.
- Emit one stable line at the final pure-first unsupported-shape stop line:
  `[llvm-pure/unsupported-shape]`.
- Keep the returned error string unchanged:
  `unsupported pure shape for current backend recipe`.
- Report only boundary inventory fields that are already visible from MIR JSON:
  recipe, first candidate block, first candidate instruction, first candidate
  op, owner hint, and reason.
- Treat `owner_hint` as a triage hint, not policy. Canonical classification and
  route acceptance remain owned by MIR normalization / CoreOp / LoweringPlan
  work, not the C shim.

## Non-goals

- Do not expand pure-first acceptance in this card.
- Do not enable `HAKO_BACKEND_COMPAT_REPLAY=harness` for mainline proof.
- Do not add per-shape route policy to `hako_llvmc_ffi`.
- Do not make the C shim decide legality beyond emitting a trace-friendly
  inventory hint.

## Follow-up Shape Contract

The real cleanup remains a separate BoxShape lane:

1. normalize legacy MIR dialects before backend entry
2. classify CoreOp / LoweringPlan before ny-llvmc emission
3. keep recipe coverage in one SSOT table/manifest
4. let ny-llvmc consume a typed plan instead of rediscovering raw MIR shape

## Acceptance

The diagnostic fix that follows this card must prove:

```bash
NYASH_LLVM_ROUTE_TRACE=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
bash <unsupported-pure-shape-smoke>
```

Expected result:

- the command still fail-fasts or the existing smoke still treats the expected
  unsupported result as PASS
- stderr contains exactly one `[llvm-pure/unsupported-shape]` line for the
  final unsupported stop line
- no compat replay is used as mainline proof
- the original error substring remains available to existing smokes
