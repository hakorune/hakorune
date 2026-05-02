---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P195, generic string Bool UnaryOp::Not flow
Related:
  - docs/development/current/main/phases/phase-29cv/P194-BUILDER-REGISTRY-DISPATCH-BODY-SHAPE.md
  - lang/src/mir/builder/internal/lower_newbox_constructor_box.hako
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P195: Generic String Bool Not Flow

## Problem

P194 moved the active source-execution blocker to:

```text
target_shape_blocker_symbol=LowerNewboxConstructorBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The MIR body contains boolean control flow that lowers `not` as:

```text
UnaryOp::Not(bool)
```

This is a scalar Bool fact gap inside an otherwise string-or-void lowerer body.
It is not a new registry/normalizer shape.

## Decision

Teach `generic_string_body` to accept `UnaryOp::Not` when the operand is Bool or
I64 scalar and to classify the result as Bool.

## Forbidden

- accepting arbitrary unary operators in string bodies
- inferring string/collection semantics from `not`
- adding by-name handling for `LowerNewboxConstructorBox`

## Acceptance

```bash
cargo test -q bool_not_in_string_or_void --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p195_generic_string_bool_not_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Probe Result

Observed on 2026-05-02:

```text
LowerNewboxConstructorBox.try_lower/1
  target_shape=generic_string_or_void_sentinel_body
  proof=typed_global_call_generic_string_or_void_sentinel

target_shape_blocker_symbol=MirSchemaBox.inst_const/2
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

This confirms that the bool `not` instruction was the active blocker inside
`LowerNewboxConstructorBox.try_lower/1`. The next blocker is a schema constructor
object boundary and should not be folded into generic string flow without a
separate route decision.
