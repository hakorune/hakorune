---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-046-M11B-STATIC-CONST-TABLE-EVAL
Scope: M11b-eval integer const expressions for static const u16 tables
---

# 293x-046 M11b Static Const Table Eval

## Decision

`M11b-eval` is live for the first narrow static table generation row.

Accepted initializer elements:

```hako
static const SIZE_CLASS: u16[] = [
  8 + 8,
  3 * 8,
  1 << 5,
  (40 - 8) | 1,
]
```

The result is still a MIR-owned `static_data_plans` row with concrete `u16`
values. No runtime table object is constructed.

## Responsibility

- Rust parser evaluates the narrow integer const-expression subset while
  parsing `static const NAME: u16[]`.
- `.hako` parser evaluates the same subset before emitting Program(JSON v0)
  `static_data_plans`.
- MIR remains the owner of the concrete `StaticDataPlan`.
- VM and ll_emit keep reading `static_data_plans`; they do not re-evaluate
  source expressions.

## Accepted Limits

- Element type remains `u16` only.
- Expressions are integer-only and side-effect-free.
- Supported operators:
  `+`, `-`, `*`, `/`, `%`, `<<`, `>>`, `&`, `|`, `^`, unary `-`, and
  parentheses.
- Final value must fit `0..65535`.
- Bitwise and shift operands must be non-negative in this narrow row.
- Divide/modulo by zero fail fast.

## Non-Goals

- No const fn.
- No references to other consts.
- No function/method calls.
- No allocation or runtime object construction during const evaluation.
- No user-code execution at compile time.
- No backend-side expression evaluation.

## Gates

```bash
bash tools/checks/k2_wide_static_const_table_eval_guard.sh
bash tools/checks/k2_wide_static_const_table_load_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
