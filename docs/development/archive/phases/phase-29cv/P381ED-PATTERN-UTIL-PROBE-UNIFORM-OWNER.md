# P381ED PatternUtil Probe Uniform Owner

Date: 2026-05-06
Scope: move PatternUtil local-value probe direct-call definitions to the uniform MIR owner.

## Context

P381EC moved BoxTypeInspector describe to `definition_owner=uniform_mir` after
adding runner MIR JSON owner coverage. The next isolated source-owner/body
cleanup slice is `PatternUtilLocalValueProbeBody`: recursive child-probe
recognition already consumes proof/return facts, and the direct-call body is
selected by MIR-owned LoweringPlan metadata.

`GenericStringOrVoidSentinelBody` and parser body handling are intentionally not
part of this card because they still have broader source-owner plumbing.

## Change

Moved `GlobalCallProof::PatternUtilLocalValueProbe` to
`definition_owner=uniform_mir`.

Added/updated tests to pin:

- route-plan owner and trace consumer
- runner MIR JSON route and lowering-plan `definition_owner`
- runner MIR JSON route and lowering-plan `emit_trace_consumer`

Preserved proof, return shape, value demand, and recursive child-probe
recognition.

## Verification

Commands:

```bash
cargo test -q pattern_util_local_value_probe
cargo test -q runner::mir_json_emit::tests::global_call_routes::pattern_util_local_value_probe
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

PatternUtil local-value probe direct-call definitions now use the uniform MIR
definition owner while keeping proof/return facts as the recursive recognition
contract.
