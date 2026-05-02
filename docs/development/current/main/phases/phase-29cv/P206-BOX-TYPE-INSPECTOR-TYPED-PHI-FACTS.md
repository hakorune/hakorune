---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P206, BoxTypeInspector typed value facts
Related:
  - docs/development/current/main/phases/phase-29cv/P201-BOX-TYPE-INSPECTOR-DESCRIBE-BODY.md
  - docs/development/current/main/phases/phase-29cv/P205-BOX-TYPE-INSPECTOR-SHAPE-PRIORITY.md
  - src/mir/global_call_route_plan/box_type_inspector_describe_body.rs
---

# P206: BoxTypeInspector Typed Value Facts

## Problem

P205 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The dedicated BoxTypeInspector shape is now selected, but the body classifier is
still trying to recover PHI classes only from all PHI inputs. The real
`_describe/1` MIR already carries owner-produced value type facts that serialize
as `dst_type` in MIR JSON, including `box<MapBox>`, `string`, `i64`, and `i1`.
Ignoring those facts makes cyclic/forward PHI regions look unknown at `indexOf`
/ `MapBox.set` sites and produces a method-call reject even though the owner
compiler has already emitted the value class.

## Decision

Teach only `box_type_inspector_describe_body` to consume MIR-owned value type
facts as route facts. PHI-local `type_hint` remains accepted when present, but
the primary owner fact for the active selfhost MIR is `function.metadata.value_types`.

This is not a new body shape and not a generic string expansion. The classifier
remains scoped to the BoxTypeInspector metadata-map shape and only uses type
hints that the MIR owner already produced.

## Boundary

The type mapping is limited to this classifier:

```text
box<MapBox> / MapBox -> Map
string / box<StringBox> -> String
i64 -> Scalar
i1 -> Bool
void -> Void
```

The implementation must not:

- change generic PHI handling in `generic_string_body.rs`
- change `mir_schema_map_constructor_body`
- infer types from callee names
- add C emitter behavior
- accept arbitrary MapBox-return helpers as BoxTypeInspector bodies

## Implementation

- Add a local `InspectorValueClass` mapping from `MirType`.
- Seed the classifier from `function.metadata.value_types`.
- Prefer the PHI-local type hint when present, then fall back to the existing
  all-inputs-known merge.
- Add a regression test with a typed MapBox PHI whose inputs are not fully known
  structurally, proving the classifier uses the MIR-owned type facts.

## Probe Result

P206 removes the previous blocker:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=LoopScanBox.find_loop_var_name/2
target_shape_blocker_reason=generic_string_return_not_string
```

## Acceptance

```bash
cargo test -q box_type_inspector_describe --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p206_box_type_inspector_typed_phi_facts.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
