# P381EC BoxTypeInspector Uniform Owner

Date: 2026-05-06
Scope: move BoxTypeInspector describe direct-call definitions to the uniform MIR owner.

## Context

P381DW moved the Phase 1 direct-only retired capsules to
`definition_owner=uniform_mir`. The next safe source-owner cleanup slice is
`BoxTypeInspectorDescribeBody`: active source-owner callers already use scalar
predicates, and the direct-call body contract is represented by MIR proof,
return shape, and result-origin metadata.

`GenericStringOrVoidSentinelBody` and parser body cleanup are intentionally not
part of this card because their source-owner/body plumbing is broader.

## Change

Moved `GlobalCallProof::BoxTypeInspectorDescribe` to
`definition_owner=uniform_mir`.

Added/updated tests to pin:

- route-plan owner and trace consumer
- runner MIR JSON route and lowering-plan `definition_owner`
- runner MIR JSON route and lowering-plan `emit_trace_consumer`

Preserved proof, return shape, value demand, and `result_origin=map_birth`.

## Verification

Commands:

```bash
cargo test -q box_type_inspector_describe
cargo test -q runner::mir_json_emit::tests::global_call_routes::box_type_inspector_describe
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

BoxTypeInspector describe direct-call definitions now use the same uniform MIR
definition owner as other retired same-module body contracts whose backend body
is selected by MIR-owned LoweringPlan facts.
