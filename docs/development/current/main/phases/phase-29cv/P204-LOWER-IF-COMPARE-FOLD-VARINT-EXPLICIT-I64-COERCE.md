---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P204, LowerIfCompareFoldVarInt explicit i64 coercion
Related:
  - docs/development/current/main/phases/phase-29cv/P180-GENERIC-I64-BOOL-SCALAR-FLOW.md
  - docs/development/current/main/phases/phase-29cv/P181-GENERIC-STRING-RECURSIVE-ACCUMULATOR-FLOW.md
  - docs/development/current/main/phases/phase-29cv/P182A-GENERIC-STRING-CLASSIFIER-BOUNDARY.md
  - docs/development/current/main/phases/phase-29cv/P203-MIR-SCHEMA-MODULE-ROOT-CONSTRUCTOR.md
  - lang/src/mir/builder/internal/lower_if_compare_fold_varint_box.hako
  - lang/src/shared/json/utils/json_frag.hako
---

# P204: LowerIfCompareFoldVarInt Explicit I64 Coerce

## Problem

P203 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._fold_bin_varint/3
target_shape_blocker_reason=generic_string_unsupported_instruction
```

`LowerIfCompareFoldVarIntBox._fold_bin_varint/3` computes a folded integer
result after resolving one side of a binary expression from a previous local
integer. Its i64 conversion helper currently hides the parse behind the dynamic
compat idiom:

```hako
return 0 + me._coerce_text_compat(value)
```

That idiom is not an explicit route fact. The direct route planner sees the text
coercion as a string-handle flow, then `_fold_bin_varint/3` observes arithmetic
over values whose route fact has already been widened toward string handling.
Extending `generic_string_body.rs` or `generic_i64_body.rs` to reinterpret this
dynamic coercion would move body execution semantics into the classifier.

## Decision

Do not add this blocker to `generic_string_body.rs`, `generic_i64_body.rs`, or a
new `_fold_bin_varint/3` body shape.

Make the i64 coercion explicit at the source owner by using the existing JSON
fragment integer parse route:

```hako
JsonFragBox._str_to_int(me._coerce_text_compat(value))
```

`JsonFragBox._str_to_int/1` is already a compiler-visible DirectAbi i64 route
through `StringHelpers.to_i64/1`. This keeps the pipeline on:

```text
existing compiler / MirBuilder -> MIR(JSON) -> explicit route facts -> LoweringPlan -> ny-llvmc
```

instead of teaching a generic body classifier to execute dynamic language
coercion.

## Boundary

This card may change only the local i64 coercion helper in
`LowerIfCompareFoldVarIntBox`.

The implementation must not:

- extend `generic_string_body.rs`
- extend `generic_i64_body.rs`
- add a `_fold_bin_varint/3` exact-name body shape
- add C emitter support for `0 + string` numeric parsing
- change `PatternUtilBox` local-int probe semantics

The behavior remains the same intended compatibility conversion:

```text
value -> text-compatible representation -> i64 parse
```

The difference is that the i64 parse is now explicit and owned by an existing
route fact.

## Implementation

- Replace the implicit `0 + text` conversion in
  `LowerIfCompareFoldVarIntBox._coerce_i64_compat/1`.
- Route the conversion through `JsonFragBox._str_to_int/1`.
- Leave the generic body classifiers and C shim emitters unchanged.

## Probe Result

P204 removes the previous blocker:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._fold_bin_varint/3
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The i64 coercion helper now routes through the existing i64 parse body:

```text
callee_name=LowerIfCompareFoldVarIntBox._coerce_i64_compat/1
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64
value_demand=scalar_i64
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p204_lower_if_compare_fold_varint_i64_coerce.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
