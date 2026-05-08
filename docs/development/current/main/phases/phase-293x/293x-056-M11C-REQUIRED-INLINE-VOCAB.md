---
Status: done
Date: 2026-05-09
Scope: M11c-required-vocab parser parity + MIR InlinePlan preservation
---

# 293x-056 M11c Required Inline Vocabulary

## Decision

M11c-required-vocab is live as a vocabulary/preservation row.

`@rune Lowering(inline_required)` is accepted by the Rust parser and the
`.hako` parser, then preserved as MIR-owned InlinePlan metadata:

```text
request = required
requires = ["no_alloc", "no_safepoint"]
verified = false
fallback = fail_fast
source = rune_lowering
```

## Why

Allocator fast paths need a future strict inline acceptance row, but `.inc` and
backends must not rediscover inline policy from function names. The first clean
step is to make the source vocabulary flow into MIR metadata without activating
required inline lowering.

`M10c LLVM export attrs widening` remains blocked because the active hako.mem
runtime-decl rows are nullable native pointers or void and have no eligible
native-pointer proof row for strong LLVM pointer attrs. This card does not
export pointer attrs, infer pointer proof, or lower allocator fast paths.

## Owned

- `Lowering(inline_required)` value contract in the Rust parser
- `Lowering(inline_required)` value contract in the `.hako` parser
- MIR `InlineRequest::Required`
- `rune_lowering` InlinePlan preservation
- guard that keeps `.inc` free of `inline_required` / `inline_plans` ownership

## Not Owned

- required inline verifier acceptance
- no_alloc/no_safepoint proof consumption for required inline
- backend-active required inline
- LLVM pointer attrs
- allocator fast-path lowering
- legacy `@lowering(...)` alias

## Acceptance

```bash
bash tools/checks/k2_wide_inline_required_vocab_guard.sh
bash tools/checks/k2_wide_inline_plan_preserve_guard.sh
bash tools/checks/k2_wide_inline_plan_soft_leaf_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Files

```text
src/ast/attrs.rs
src/parser/runes.rs
lang/src/compiler/parser/rune/rune_contract_box.hako
src/mir/inline_plan.rs
src/mir/passes/inline_soft_leaf.rs
tools/checks/k2_wide_inline_required_vocab_guard.sh
```
