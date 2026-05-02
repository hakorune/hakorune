---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P194, builder registry dispatcher body shape and child blocker propagation
Related:
  - docs/development/current/main/phases/phase-29cv/P193-STRING-TYPED-STRING-OR-VOID-PASSTHROUGH.md
  - lang/src/mir/builder/internal/registry_authority_box.hako
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/builder_registry_dispatch_body.rs
---

# P194: Builder Registry Dispatch Body Shape

## Problem

P193 moved the source-execution probe to:

```text
target_shape_blocker_symbol=BuilderRegistryAuthorityBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`BuilderRegistryAuthorityBox.try_lower/1` is not a pure string body. It is a
registry dispatcher:

```text
PatternRegistryBox.candidates()
ArrayBox/RuntimeDataBox length/get/push
registry-name string comparisons
Lower*Box.try_lower/1 calls
debug logging
return child result or null
```

Adding `ArrayBox.get` and registry dispatch semantics to
`generic_string_body.rs` would make the generic string classifier a second
compiler. The registry dispatcher must be a dedicated body shape.

## Decision

Introduce a dedicated Rust-side body classifier:

```text
builder_registry_dispatch_body
```

P194 only records the structural shape and propagates child lowerer blockers. A
registry dispatcher is DirectAbi only when its returned child lowerers are also
DirectAbi-compatible. If any returned child lowerer is still unsupported, the
registry body remains unsupported and reports that child as the blocker.

## Boundary

The classifier may observe:

- array candidate list traversal (`length`/`get`, optional `push`)
- registry-name string constants and equality checks
- `*.try_lower/1` global child calls
- string-or-void return shape (`return child_result` or `return null`)

The classifier must not:

- match `BuilderRegistryAuthorityBox` by exact name
- add registry semantics to `generic_string_body`
- silently skip unsupported child lowerers
- emit C code for the dispatcher before the child lowerers and method ops are
  lowerable

## Acceptance

```bash
cargo test -q builder_registry_dispatch --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p194_builder_registry_dispatch_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe. P194 is expected to
replace the generic registry `unsupported_method_call` with the first returned
child lowerer blocker if child lowerers are still incomplete.

## Probe Result

Observed on 2026-05-02:

```text
BuilderRegistryAuthorityBox.try_lower/1
  target_shape_reason=generic_string_global_target_shape_unknown
  target_shape_blocker_symbol=LowerIfCompareFoldBinIntsBox._fold_bin_ints/2
  target_shape_blocker_reason=generic_string_unsupported_instruction

active source-execution blocker:
  target_shape_blocker_symbol=LowerNewboxConstructorBox.try_lower/1
  target_shape_blocker_reason=generic_string_unsupported_instruction
```

This confirms that the registry body no longer stops at generic
`unsupported_method_call`. The source-execution probe now exposes a lowerer body
blocker outside the registry classifier itself.
