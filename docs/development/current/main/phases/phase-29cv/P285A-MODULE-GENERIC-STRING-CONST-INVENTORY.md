---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P285a, module generic string const inventory
Related:
  - docs/development/current/main/phases/phase-29cv/P284A-STRING-CORRIDOR-PHI-RECEIVER-ROUTE.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_compiler_state.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P285a: Module Generic String Const Inventory

## Problem

After P284a, the source-execution probe advances to:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=CliRunLowerBox._emit_mir/3
```

`CliRunLowerBox._emit_mir/3` is a large owner-local MIR text emitter. Its MIR
body has more than 512 `StringBox` constants, while the C-side generic pure
lowering state stores string constants and hoist markers in fixed 512-entry
tables:

```text
str_consts[512]
owned_str_consts[512]
str_const_box_skip[512]
str_const_box_emitted[512]
```

When the inventory fills, `put_str_const` silently drops later literals. The
module generic body emitter then fails later when `get_str_const(dst)` cannot
find the literal for a valid `const` instruction.

## Decision

Keep this as a capacity/diagnostic fix only:

```text
string const inventory -> named capacity SSOT -> explicit overflow reason
```

The inventory capacity belongs to the generic pure lowering state. It must not
be encoded as repeated magic `512` literals in helper functions.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox/ArrayBox/string method widening
- no new body-specific C emitter
- no `.hako` source workaround in `CliRunLowerBox._emit_mir/3`
- no fallback or compat route change

## Acceptance

- `CliRunLowerBox._emit_mir/3` no longer fails because string constants beyond
  the old 512-entry inventory are missing.
- If a generic pure inventory fills again, the shim records an explicit capacity
  reason instead of silently dropping state.
- The source-execution probe advances to the next blocker or produces the exe.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

`CliRunLowerBox._emit_mir/3` has 584 `StringBox` constants, so it overflowed
the old 512-entry inventory. The string constant, owned string, and hoist marker
tables now use named capacity constants, and inventory overflow records an
explicit capacity reason.

The fresh source-execution probe advanced past `CliRunLowerBox._emit_mir/3` to:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

This card intentionally leaves that next owner-local source-execution blocker
for a separate acceptance card.
