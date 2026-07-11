# P381GA PatternUtil Probe Contract Helper

Date: 2026-05-06
Scope: make the PatternUtil local-value probe body module own its proof/return contract pair.

## Problem

`PatternUtilLocalValueProbeBody` has already been retired as a public target
shape and moved to `definition_owner=uniform_mir`.

The remaining body-handling cleanup was a small SSOT split:

- the PatternUtil body recognizer used the PatternUtil proof/return pair to
  recognize child probes
- the top-level global-call classifier repeated the same proof/return pair when
  publishing the accepted route contract

That means the body module recognized the contract but did not own the route
classification entry for the same contract.

## Change

The PatternUtil body module now exposes one helper:

```text
pattern_util_probe_body_classification()
```

The helper returns the direct contract:

```text
proof=typed_global_call_pattern_util_local_value_probe
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
```

The child-probe recognizer and the top-level classifier now consume the same
module-local proof/return constants.

## Boundary

Allowed:

- move only the proof/return pair ownership
- preserve the exact body recognizer
- preserve `definition_owner=uniform_mir`

Not allowed:

- widen PatternUtil body acceptance
- add name-based C fallback
- change mixed scalar/handle ABI behavior

## Verification

```bash
cargo test -q pattern_util_local_value_probe
cargo test -q runner::mir_json_emit::tests::global_call_routes::pattern_util_local_value_probe
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```
