# P381GV JoinIR VM Bridge Old Lane Gating

Date: 2026-05-06
Scope: gate the legacy top-level JoinIR VM bridge handler lane out of normal builds and document the remaining loop-scope hold vocabulary.

## Context

`cargo build --release` was still reporting 24 `nyash-rust (lib)` warnings.
Most of them came from the old top-level JoinIR VM bridge handler lane under
`src/mir/join_ir_vm_bridge/**`, while the active normal-build owner had already
moved to `joinir_block_converter/handlers.rs`.

The remaining non-bridge warnings were not dead code to delete immediately:
`loop_scope_shape` still carries shape vocabulary that current release routing
does not read directly, but future shape-aware lowering and docs still name.

## Change

- Marked the old top-level JoinIR VM bridge handler lane as test-only in
  `src/mir/join_ir_vm_bridge/mod.rs`.
- Recorded that ownership split in `src/mir/join_ir_vm_bridge/README.md`.
- Gated legacy `CopyEmitReason` variants to tests because they are only used by
  the old top-level handler lane.
- Gated test-only bridge conversion entrypoints
  (`JoinIrFunctionConverter::convert_function`,
  `JoinIrBlockConverter::new`) to match their actual usage.
- Added local hold annotations plus README notes for
  `loop_scope_shape` structural vocabulary instead of deleting it.

## Result

Observed `cargo build --release` warning count:

```text
before: 24 warnings
after:   0 warnings
```

This is BoxShape-only cleanup. It does not change JoinIR lowering behavior,
bridge semantics, or VM execution.

## Validation

```bash
cargo build --release
cargo test -q join_ir_vm_bridge --lib
```
