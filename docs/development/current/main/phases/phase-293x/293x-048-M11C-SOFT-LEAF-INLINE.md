---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-048-M11C-SOFT-LEAF-INLINE
Scope: M11c-soft-leaf best-effort same-module MIR inline for advisory Hint(inline)
---

# 293x-048 M11c Soft Leaf Inline

## Decision

`M11c-soft-leaf` is live for narrow best-effort MIR inline.

Accepted shape:

```text
caller:
  Call { callee: Global(name), ... }

callee:
  has InlinePlan request=prefer from Hint(inline)
  one entry block
  no PHI/control-flow split
  no nested Call
  no return_env
  body size <= 8 supported pure instructions
```

Unsupported shapes keep the original call. This card does not introduce
required inline or fail-fast verifier semantics.

## Supported Body Vocabulary

```text
Const
UnaryOp
BinOp
Compare
StaticDataLoad
Copy
Select
TypeOp
Return
```

`Hint(noinline)` / `request=avoid` wins over soft inline.

## Responsibility

- MIR `InlinePlan` remains the truth for advisory inline requests.
- The MIR optimizer owns this best-effort transform.
- Backends emit the already-transformed MIR and must not inspect source rune
  strings or function names to decide inline behavior.
- `.inc` remains a non-owner for inline planning.

## Non-Goals

- No `Lowering(inline_required)` syntax.
- No verifier-backed required inline.
- No cross-module inline.
- No method/virtual/dynamic dispatch inline.
- No backend or `.inc` inline consumer.
- No app-specific allocator symbol switch.

## Gates

```bash
bash tools/checks/k2_wide_inline_plan_soft_leaf_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
