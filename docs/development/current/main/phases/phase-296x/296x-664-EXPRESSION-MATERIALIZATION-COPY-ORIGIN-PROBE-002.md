---
Status: Landed
Date: 2026-06-15
Task: EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002
Scope: Refresh expression-materialization copy-origin classification for the
  current object-lifecycle body-timing front before reopening optimization.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-663-MIMALLOC-BODY-TIMING-FRONT-SELECT-001.md
  - docs/development/current/main/phases/phase-296x/296x-179-EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py
---

# EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002

## Purpose

`MIMALLOC-BODY-TIMING-FRONT-SELECT-001` selected the product-route
object-lifecycle body timing front and narrowed the MIR body owner to
`local_ssa_copy_materialization`. The current copy-kind policy selection points
at expression materialization, but the old row 179 numbers are historical.

This row refreshes the expression-materialization copy origin on the current
MIR before implementation.

```text
row_kind=diagnostic
implementation_started=0
optimization_open=0
body_timing_front_selected=1
selected_body_timing_surface=object_lifecycle_small_block_body_timing
selected_mir_body_owner=local_ssa_copy_materialization
selected_copy_kind_policy=expression_materialization_copy_policy
```

## Input Evidence

From `MIMALLOC-BODY-TIMING-FRONT-SELECT-001`:

```text
hako_body_elapsed_ns=366000000
c_body_elapsed_ns=3239831
body_elapsed_ratio=112.969
gap_owner=compiler_lowering

copy_count=94
local_ssa_copy_count=39
dominant_copy_owner=local_ssa_copy_materialization
dominant_dynamic_owner=local_ssa_copy_materialization

expression_materialization_copy_count=11
selected_copy_kind_policy=expression_materialization_copy_policy
rejected_policy=local_ssa_same_block_field_get_reuse
```

The old landed probe had:

```text
historical_row=296x-179
historical_expression_materialization_copy_count=24
historical_dominant_expression_origin=field_get
```

Do not assume that historical origin still holds. Re-run the probe against the
current MIR and current policy report.

## Required Command Shape

```bash
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako

python3 tools/allocator/mir_callsite_copy_attribution.py \
  --mir-json "$mir_json" \
  --out "$attribution_report"

python3 tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py \
  --attribution "$attribution_report" \
  --method-invocation-count 524288 \
  --out "$dynamic_weight_report"

python3 tools/allocator/mir_local_ssa_copy_position_probe.py \
  --mir-json "$mir_json" \
  --out "$position_report"

python3 tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py \
  --dynamic-weight "$dynamic_weight_report" \
  --position "$position_report" \
  --out "$policy_report"

python3 tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py \
  --mir-json "$mir_json" \
  --selection "$policy_report" \
  --out "$origin_report"
```

## Evidence

Result:

```text
output_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
expression_materialization_copy_count=11
dominant_expression_origin=param
param_origin_copy_count=7
field_get_origin_copy_count=3
const_origin_copy_count=1
dominant_expression_sink=field_get
selected_origin_policy=param_expression_value_copy_chain
next_diagnostic=param_expression_copy_chain_policy_selection
optimization_open=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The historical row179 `field_get` owner no longer dominates the current MIR:

```text
historical_dominant_expression_origin=field_get
current_dominant_expression_origin=param
```

Therefore the next row must not reuse the old field-get expression chain
policy without a fresh param-chain policy selection.

## Selection Rules

Select the next implementation owner only if the probe finds a single narrow
origin:

```text
dominant_expression_origin_present=1
dominant_expression_origin_single_enough=1
expression_materialization_copy_count_matches_policy=1
recent_nonkeeper_not_reopened=1
```

Reject:

```text
local_ssa_same_block_field_get_reuse:
  rejected because it already regressed exact-EXE body timing

process_total_startup_delta:
  diagnostic only

resident_kernel_win_or_loss:
  not sufficient for this product-route body row
```

## Stop Line

```text
do not edit MIRBuilder before origin probe
do not reopen same-block LocalSSA reuse
do not change product NyRT startup
do not change allocator provider activation
do not install hooks or global allocator
do not claim keeper from diagnostic-only evidence
```

## Acceptance

```text
expression_materialization_copy_origin_probe_002_landed=1
body_timing_front_selected=1
selected_copy_kind_policy=expression_materialization_copy_policy
origin_probe_rerun=1
dominant_expression_origin=param
param_origin_copy_count=7
field_get_origin_copy_count=3
selected_origin_policy=param_expression_value_copy_chain
next_task=PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001
implementation_started=0
optimization_open=0
summary=ok
```
