Status: Done
Date: 2026-06-18
Scope: preflight MIR JSON emitter as backend-side crate split candidate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1098-BUILD-BACKEND-CRATE-PREFLIGHT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/runner/mir_json_emit

# BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001

## Purpose

Decide whether `src/runner/mir_json_emit/**` can be moved directly into a new
backend-side crate.

## Audit

```text
src_runner_mir_json_emit_rs_total_lines=10033
crate_mir_reference_count=372
crate_object_storage_plan_reference_count=4
crate_ast_reference_count=1
largest_file_line_count=786
large_file_count=0
```

## Decision

Do not move `runner/mir_json_emit` directly into a crate yet.

```text
direct_crate_extraction_selected=0
reason=emitter_reads_main_crate_mir_shape_directly
reason=route_json_and_metadata_emitters_depend_on_many_active_plan_modules
reason=root_emitter_reaches_cfg_extractor_and_main_mir_module
```

Select a boundary-design row first.

```text
selected_next_task=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
purpose=define the emitter input/view boundary before extraction
```

## Candidate Table

| Candidate | Decision | Reason |
|---|---|---|
| Move all `runner/mir_json_emit` now | reject | Too many direct `crate::mir` dependencies; likely creates a cyclic crate. |
| Move only `io.rs` / JSON writer helpers | reject | Too small to affect build time; adds ceremony without boundary value. |
| Move route metadata emitters first | reject for now | They depend on many active plan modules and route producers. |
| Define MIR JSON emitter view boundary | select | Allows future extraction without moving producer logic or main MIR ownership. |

## Boundary Direction

The future extraction should not require `hakorune-mir-json-emit` to depend on
the main crate. The emitter needs a stable input view:

```text
MirJsonModuleView
MirJsonFunctionView
MirJsonBlockView
MirJsonInstructionView
MirJsonMetadataView
```

The main crate remains the owner of MIR producers and plan refresh logic. The
future emitter crate should consume only the view/model, not builder/backend
internals.

## Stop Lines

```text
do_not_create_crate_cycle=1
do_not_move_mir_producers=1
do_not_move_route_refresh_logic=1
do_not_change_mir_json_schema=1
do_not_change_ny_llvmc_route=1
do_not_move_runner_or_parser_or_using_resolution=1
behavior_change_allowed=0
```

## Next

```text
next_task=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
purpose=define the view/model seam that lets MIR JSON emission become extractable
```
