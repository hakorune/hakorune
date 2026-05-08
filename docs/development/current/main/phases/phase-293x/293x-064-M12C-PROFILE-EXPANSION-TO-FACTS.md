---
Status: done
Date: 2026-05-09
Scope: M12c Profile expansion to facts
---

# 293x-064 M12c Profile Expansion To Facts

## Decision

M12c is live-narrow.

`@rune Profile(...)` is accepted only for the registry names in:

```text
docs/reference/mir/rune-profile-registry.md
```

The profile string is not backend semantics. It is parser-validated authoring
sugar that expands into existing MIR-owned facts:

- `InlinePlan` from `Hint(...)` / `Lowering(inline_required)` expansion.
- `EffectPlan` from `Contract(no_alloc/no_safepoint)` expansion.
- `CapabilityPlan` allow-list metadata from the profile registry.

## Accepted Profiles

- `allocator.fast`
- `allocator.slow`
- `substrate.leaf`
- `intrinsic.leaf`
- `raw.layout`

## Owned Changes

- Rust parser accepts dotted profile names such as
  `@rune Profile(allocator.fast)`.
- `.hako` parser rune contract registry validates the same reserved names.
- MIR plan refresh expands profile names through `src/rune_profile_registry.rs`.
- Existing required-inline verifier sees profile-provided contracts as
  satisfying required inline obligations.
- MIR JSON exposes only expanded plan facts.

## Not Owned

- `@rune Capability(...)` syntax.
- Capability verifier acceptance.
- Backend or `.inc` profile-name consumption.
- `no_panic` / `no_io` / `no_trace` effect requirements.
- Allocator fast-path EXE proof.
- Native pointer strong attrs.

## Acceptance

```bash
bash tools/checks/k2_wide_profile_expansion_to_facts_guard.sh
bash tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
