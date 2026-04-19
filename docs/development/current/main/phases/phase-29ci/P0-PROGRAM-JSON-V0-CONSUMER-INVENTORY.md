---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `Program(JSON v0)` bootstrap boundary の remaining consumer table。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md
  - docs/development/current/main/phases/phase-29ci/P5-STAGEB-MALFORMED-PROGRAM-JSON.md
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
---

# P0 Program JSON v0 Consumer Inventory

## Goal

`Program(JSON v0)` の残存 consumer と current caller surface を exact owner つきで短く固定する。

Boundary class:

- `public/deprecate-now`
- `internal-compat-keep`
- `delete-ready-later`

Caller action bucket:

- closed in this slice

## Consumer Matrix

| Bucket | Boundary class | Owner / caller | Surface | Note |
| --- | --- | --- | --- | --- |
| `current authority` | `internal-compat-keep` | [`src/host_providers/mir_builder.rs`](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs), [`src/host_providers/mir_builder/handoff.rs`](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/handoff.rs), [`src/host_providers/mir_builder/decls.rs`](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/decls.rs), [`src/host_providers/mir_builder/lowering.rs`](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/lowering.rs) | `emit_program_json_v0_for_strict_authority_source(...)`, `program_json_to_mir_json(...)` | current source-route authority; `lowering.rs` stays test-only evidence |
| `compat loader keep` | `internal-compat-keep` | [`src/runner/json_artifact/mod.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/json_artifact/mod.rs), [`src/runner/json_artifact/program_json_v0_loader.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/json_artifact/program_json_v0_loader.rs), [`tools/smokes/v2/lib/test_runner_builder_helpers.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/lib/test_runner_builder_helpers.sh) | `load_json_artifact_to_module(...)`, `load_program_json_v0_to_module(...)`, direct-MIR detection + compat fallback helpers | `--json-file` compat umbrella intake; owns import-bundle alias collect / merge / trace; mixed probe split is now explicit helper routing |
| `legacy AST JSON compat keep` | `internal-compat-keep` | [`src/host_providers/mir_builder/lowering/ast_json.rs`](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/lowering/ast_json.rs) | `program_json_to_mir_json(...)` legacy AST fallback branch | phase-0 compat fallback |
| `build surrogate keep` | `internal-compat-keep` | [`crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | `emit_program_json_v0_for_current_stage1_build_box_mode(...)` | compiled-stage1 `BuildBox.emit_program_json_v0` dispatch shim |
| `build surrogate test keep` | `internal-compat-keep` | [`crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | same as above | route-match / arg-decode / encode regression coverage |
| `future-retire bridge` | `public/deprecate-now` | [`src/runner/stage1_bridge/program_json/mod.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/mod.rs), [`src/runner/stage1_bridge/program_json/orchestrator.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/orchestrator.rs), [`src/runner/stage1_bridge/program_json/read_input.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/read_input.rs), [`src/runner/stage1_bridge/program_json/payload.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/payload.rs), [`src/runner/stage1_bridge/program_json/writeback.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/writeback.rs) | `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)` | bridge-local read→emit→write orchestration |
| `future-retire bridge entry` | `public/deprecate-now` | [`src/runner/stage1_bridge/program_json_entry/mod.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/mod.rs), [`src/runner/stage1_bridge/program_json_entry/execute.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/execute.rs), [`src/runner/stage1_bridge/program_json_entry/exit.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/exit.rs), [`src/runner/stage1_bridge/program_json_entry/request.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/request.rs), [`src/runner/emit.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/emit.rs), [`src/runner/mod.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/mod.rs) | `program_json_entry::{emit_program_json_v0_requested, emit_program_json_v0_and_exit}` | delegate-only entry |
| `.hako` live/bootstrap callers | `internal-compat-keep` | [`lang/src/runner/stage1_cli_env.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli_env.hako), [`lang/src/runner/stage1_cli.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako), [`lang/src/runner/launcher.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/launcher.hako), [`lang/src/mir/builder/MirBuilderBox.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/mir/builder/MirBuilderBox.hako) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | live/bootstrap callers on the `.hako` side |
| `shell helper keep` | `public/deprecate-now` | [`tools/hakorune_emit_mir.sh`](/home/tomoaki/git/hakorune-selfhost/tools/hakorune_emit_mir.sh), [`tools/selfhost/selfhost_build.sh`](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/selfhost_build.sh), [`tools/smokes/v2/lib/test_runner.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/lib/test_runner.sh) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | helper/canary route |
| `diagnostics/probe keep` | `delete-ready-later` | [`tools/dev/phase29ch_program_json_helper_exec_probe.sh`](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_program_json_helper_exec_probe.sh), [`tools/dev/phase29ch_selfhost_program_json_helper_probe.sh`](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_selfhost_program_json_helper_probe.sh) | `MirBuilderBox.emit_from_program_json_v0(...)` | diagnostics-only keep |

## Caller Surface Matrix

| Caller bucket | Representative caller | Actual artifact kind | Action | Replacement / note |
| --- | --- | --- | --- | --- |
| `direct-mir rewrite now` | [`tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh), [`tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh) | direct `MIR(JSON)` file | rewrite to `--mir-json-file` now | these callers are not testing compat loader behavior |
| `compat loader keep` | [`tools/smokes/v2/lib/stageb_helpers.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/lib/stageb_helpers.sh), `tools/smokes/v2/profiles/quick/core/bridge/canonicalize_*`, `tools/smokes/v2/profiles/integration/core/phase2034/program_v0_*`, `tools/smokes/v2/profiles/integration/core_direct/*` | `Program(JSON v0)` or mixed compat-loader probe | keep on `--json-file` | purpose is Stage-B / bridge / import-bundle / downconvert / Gate-C coverage |
| `explicit compat convert/emit keep` | [`tools/selfhost/selfhost_build.sh`](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/selfhost_build.sh), [`tools/selfhost_exe_stageb.sh`](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_exe_stageb.sh), [`tools/selfhost/lib/identity_routes.sh`](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/identity_routes.sh), `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*` | explicit Program(JSON v0) emit/convert | keep | this is the compat-only bridge/delegate lane |
| `delete-last internal alias` | [`src/runner/core_executor.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/core_executor.rs) `run_json_v0(...)` (deleted), [`src/runner/pipe_io.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/pipe_io.rs) `--json-file` handling comment seam | internal compat alias / route wording | closed | runner alias deleted; keep pipe_io wording aligned with loader split |

## Archived Monitor Evidence

- [`tools/ny_parser_bridge_smoke.sh`](/home/tomoaki/git/hakorune-selfhost/tools/ny_parser_bridge_smoke.sh)
- [`tools/ny_parser_bridge_smoke.ps1`](/home/tomoaki/git/hakorune-selfhost/tools/ny_parser_bridge_smoke.ps1)
- [`tools/smokes/selfhost_local.sh`](/home/tomoaki/git/hakorune-selfhost/tools/smokes/selfhost_local.sh)
- [`tools/selfhost_parser_json_smoke.sh`](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_parser_json_smoke.sh)
- status: archive-only evidence; no longer a current cleanup bucket

## Closed In This Slice

- `delete-last internal alias` bucket is now closed; the runner-side compat alias layer has been deleted

## Closed in This Slice

- `archive-ready monitor/probe/docs` bucket is now archive-only evidence
- `mixed route probe keep` bucket is now explicit helper routing inside `compat loader keep`

## Delete Order Guard

1. do not touch `current authority` until a non-JSON authority path exists
2. keep `compat loader keep` explicit and separate from `future-retire bridge`
3. thin `build surrogate keep` and `future-retire bridge` as separate owner buckets
4. keep `.hako` live/bootstrap callers and diagnostics/probes out of the same patch as Rust host caller deletion

## Detail Links

- exact caller / owner matrix detail: `P0`
- future-retire bridge delete order: `P1`
- live/bootstrap + shell caller delete order: `P2`
- shared shell helper audit: `P3`
- malformed producer closeout: `P5`
