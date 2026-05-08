---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-045-INLINE-PLAN-DOCS-LOCK
Scope: InlinePlan docs-first boundary and allocator-substrate task order
---

# 293x-045 InlinePlan Docs Lock

## Decision

Inline support is needed for allocator-grade fast paths, but it must be added
as a compiler-owned plan, not as backend-local special casing.

Accepted boundary:

```text
@rune Hint(inline/noinline/hot/cold)
-> MIR InlinePlan / CallsiteInlinePlan
-> verifier
-> MIR transform or intrinsic route
-> backend emits the result
```

`@rune Lowering(inline_required)` is reserved for substrate/allocator lanes
only. It is not live syntax yet.

## Ordering

`M11b` is already the static const table lane. Inline work is named `M11c`.

Recommended next order:

```text
M11c-docs:
  this card

M11b-eval:
  const expression / const fn table generation

M11c-preserve:
  preserve existing Hint(inline/noinline/hot/cold) into MIR InlinePlan metadata

M11c-soft-leaf:
  best-effort same-module leaf MIR inline

M10c-pre / M10c:
  pointer/native-ptr proof and strong attrs widening

M11c-required-vocab / M11c-required-verify:
  substrate-only inline_required vocabulary and verifier-backed fail-fast

M12 / M13:
  mimalloc raw-page proof and allocator fast-path EXE proof
```

This keeps static table completeness, inline planning, pointer proof, and
allocator proof as separate rows.

## Responsibility

- Source annotations stay in `@rune`.
- MIR owns InlinePlan truth.
- Verifier owns required-inline acceptance.
- MIR optimizer / route selection owns the transform.
- Backend emitters are readers only.

## Non-Goals

- No code behavior change in this card.
- No `inline` keyword.
- No public `always_inline` guarantee.
- No `.inc` / ll_emit inliner.
- No app-specific method-name matching.

## Manual Updates

- `docs/development/current/main/design/inline-plan-ssot.md`
- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`
- `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
- `docs/development/current/main/design/optimization-tag-flow-ssot.md`
- `docs/development/current/main/design/current-optimization-mechanisms-ssot.md`
- `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- `docs/development/current/main/design/README.md`
- `docs/reference/mir/hints.md`
- `docs/reference/runtime/substrate-capabilities.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Gates

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
