# 293x-024: binary-trees EXE parity

- Status: Landed
- Date: 2026-05-08
- Lane: `phase-293x real-app bringup`

## Summary

`apps/binary-trees/main.hako` now builds and runs through the pure-first direct
EXE path without compat replay.

The compiler-side blocker was not app code. It was the same-module route seam:
recursive user-box method bodies, typed-object handle returns through global
callee surfaces, and instruction-form branch/return bodies were not all visible
to MIR-owned body-shape facts.

## Changes

- Expanded user-box method body support to accept instruction-form
  branch/jump/return bodies used by the same-module emitter.
- Allowed self-recursive user-box method bodies to prove through their own
  route symbol instead of waiting for an impossible non-recursive first pass.
- Added a MIR-owned global-call return contract for same-module typed-object
  handle returns.
- Published `target_result_box_name` for global call routes so the EXE shim can
  consume typed-object result metadata instead of inferring box semantics from
  raw names.
- Promoted binary-trees from the generic EXE boundary probe to
  `tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh`.

## Boundary

- TypedObjectPlan remains the layout truth.
- The C shim remains a reader/emitter: it reads `global_call_routes`,
  `user_box_method_routes`, `target_result_box_name`, and value metadata.
- No app-specific `BinaryTreeBuilder` / `TreeNode` matcher was added.
- No compat replay is accepted as parity proof.

## Gates

```bash
cargo fmt --check
cargo test -q refresh_module_global_call_routes_accepts_typed_object_handle_return --lib
cargo test -q refresh_module_user_box_method_routes_accepts_object_handle_method_target --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build --release -p nyash-llvm-compiler
bash tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```
