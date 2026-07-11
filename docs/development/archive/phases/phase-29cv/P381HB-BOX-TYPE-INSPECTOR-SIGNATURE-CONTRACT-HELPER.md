# P381HB BoxTypeInspector Signature Contract Helper

Date: 2026-05-06
Scope: consolidate the local BoxTypeInspector describe signature gate without
changing accepted shapes.

## Context

The next cleanup slice after P381HA first inspected the preferred Stage0
same-module seam in `hako_llvmc_ffi_same_module_function_emit.inc`.

That file's remaining repeats are still entangled with per-method emit wiring,
origin publication, or helper-specific argument assembly, so there was not yet a
clear owner-local dedup seam small enough to land safely as one BoxShape slice.

The safer follow-on seam was local to
`src/mir/global_call_route_plan/box_type_inspector_describe_body.rs`:

- `box_type_inspector_describe_body_reject_reason(...)`
- `is_box_type_inspector_describe_body_candidate(...)`

Both repeated the same signature contract:

- exactly one bound parameter
- MIR param bindings match signature params
- return type remains `Unknown` or `MapBox`

## Change

- Added `box_type_inspector_describe_signature_blocker(...)` as the local
  contract helper.
- Rewired the reject path to map the helper's reason into
  `GenericPureStringReject`.
- Rewired the candidate probe to use the same helper instead of repeating the
  signature gate inline.

## Result

BoxTypeInspector describe now owns one signature contract helper for both the
candidate probe and reject path. This is behavior-preserving BoxShape cleanup:

- no new Stage0 shape variants
- no parser-private ownership changes
- no backend-specific matcher growth
- accepted BoxTypeInspector bodies stay unchanged

## Validation

```bash
cargo test -q box_type_inspector_describe
cargo test -q runner::mir_json_emit::tests::global_call_routes::box_type_inspector_describe
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
