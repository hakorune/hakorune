---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-047-M11C-INLINE-PLAN-PRESERVE
Scope: M11c-preserve advisory Hint rune metadata into MIR InlinePlan rows
---

# 293x-047 M11c InlinePlan Preserve

## Decision

`M11c-preserve` is live for advisory inline hint preservation.

Accepted source hints:

```hako
@rune Hint(inline)
@rune Hint(noinline)
@rune Hint(hot)
@rune Hint(cold)
```

These hints now derive MIR-owned `metadata.inline_plans` rows. They do not
trigger MIR transforms, backend inline, or `.inc` behavior.

## Responsibility

- Parser keeps owning rune syntax and validation.
- MIR owns `InlinePlan` metadata derived from declaration-local `Hint(...)`
  runes.
- MIR JSON emits `functions[].metadata.inline_plans`.
- Backends remain readers of already-decided MIR shapes and must not infer
  inline behavior from source rune strings or function names.

## Live Mapping

```text
Hint(inline)   -> request=prefer
Hint(noinline) -> request=avoid
Hint(hot)      -> request=none, hotness=hot
Hint(cold)     -> request=none, hotness=cold
```

All M11c-preserve rows use:

```text
verified=false
fallback=keep_call
source=rune_hint
requires=[]
max_ir=null
```

## Non-Goals

- No MIR inline transform.
- No callsite inline plan.
- No `Lowering(inline_required)` syntax.
- No required-inline verifier.
- No backend or `.inc` inline consumer.
- No app-specific or allocator-specific inline switch.

## Gates

```bash
bash tools/checks/k2_wide_inline_plan_preserve_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
