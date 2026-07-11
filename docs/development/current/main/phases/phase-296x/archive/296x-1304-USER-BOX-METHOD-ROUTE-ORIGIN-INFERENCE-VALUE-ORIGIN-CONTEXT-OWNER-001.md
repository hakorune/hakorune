---
Status: Closed
Decision: accepted
Date: 2026-06-19
Scope: Continue the scan-methods focused timeout reduction after typed-object
  BoxOriginQueryContext stopped owning the hot copy-origin lookup.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1303-COREPLAN-SCAN-METHODS-MULTI-CANDIDATE-LOOP-OWNER-SELECTION-001.md
---

# USER-BOX-METHOD-ROUTE-ORIGIN-INFERENCE-VALUE-ORIGIN-CONTEXT-OWNER

## Current Task Snapshot

Current status:

```text
previous_owner=typed_object_box_origin_value_origin_lookup_cost
previous_owner_closed_by=BoxOriginQueryContext_function_local_copy_origin_index
current_failure_mode=compile_time_timeout
focused_direct_timeout=0
focused_gate_green=1
current_owner=user_box_method_route_origin_inference_value_origin_lookup_cost
current_blocker_token=USER-BOX-METHOD-ROUTE-ORIGIN-INFERENCE-VALUE-ORIGIN-CONTEXT-OWNER-001
```

Focused command:

```bash
timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

## Evidence

Latest sampled stack:

```text
current_owner=user_box_method_route_origin_inference_value_origin_lookup_cost
path=user_box_method_route_plan::origin_inference::route_flow_value_box_name
path=value_origin::resolve_value_origin
path=value_origin::ValueOriginQueryContext::origin
path=function.blocks HashMap lookup
symptom=route-flow box-name inference repeatedly resolves copy origins through
the generic ValueOriginQueryContext while walking PHI/route-flow inputs.
```

This is still compile-time owner movement, not a loop route semantics failure.

## Next Task

```text
USER-BOX-METHOD-ROUTE-ORIGIN-INFERENCE-VALUE-ORIGIN-CONTEXT-OWNER-001
```

Purpose:

```text
Make user_box_method_route_plan::origin_inference share copy-origin resolution
inside one inference scan, preferably by reusing ValueOriginQueryContext or a
function-local copy-origin context. Preserve route semantics.
```

Required shape:

```text
1. Keep user-box method route inference as the owner.
2. Do not add function-name/source-name bypasses.
3. Do not change route certainty semantics.
4. Do not merge typed-object storage/box-origin ownership into this row.
5. Keep touched code files below 800 lines.
```

Acceptance:

```text
cargo check -q --lib
cargo test -q user_box_method_route_plan
cargo build -q --bin hakorune --features vm-reference

timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

If focused direct still times out:

```text
sample_new_owner=1
update_current_state=1
do_not_claim_green=1
```

## Implementation Result

Implemented:

```text
origin_inference_shared_value_origin_context_enabled=1
route_flow_box_name_memo_enabled=1
route_flow_helper_split_enabled=1
generic_resolve_value_origin_hot_path_enabled=0
route_certainty_semantics_changed=0
typed_object_ownership_changed=0
```

Code boundary:

```text
src/mir/user_box_method_route_plan/origin_inference.rs:
  keeps user-box method route inference ownership
  shares ValueOriginQueryContext within function scans
  keeps compatibility wrappers for external callers

src/mir/user_box_method_route_plan/origin_route_flow.rs:
  owns route-flow PHI box-name inference
  owns scan-local route-flow memoization
```

Verified:

```text
cargo check -q --lib
cargo test -q storage_inference
cargo test -q user_box_method_route_plan
cargo build -q --bin hakorune --features vm-reference

timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
result=0
```

Focused gate:

```text
focused_direct_timeout=0
focused_gate_green=1
summary=ok
```

## Stop Lines

```text
do not commit this row as closed while focused_direct_timeout=1
do not reopen legacy loop route perfection from compile-time-owner evidence
do not add source/function-name workarounds
do not change user-box route certainty semantics while reducing lookup cost
```

## Report Vocabulary

```text
output_contract=user-box-method-route-origin-inference-value-origin-context-owner-v0
origin_inference_shared_value_origin_context_enabled=<0|1>
generic_resolve_value_origin_hot_path_enabled=<0|1>
route_certainty_semantics_changed=0
typed_object_ownership_changed=0
focused_direct_timeout=<0|1>
summary=<ok|blocked>
```
