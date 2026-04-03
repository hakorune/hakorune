---
Status: Active
Date: 2026-04-04
Scope: current-doc and helper-comment cleanup for lingering wording that still makes `rust-vm` look like a day-to-day owner.
Related:
  - docs/development/current/main/phases/phase-49x/README.md
  - docs/development/current/main/phases/phase-49x/49x-91-task-board.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
---

# 49x-90 Legacy Wording / Compat Route Cleanup SSOT

## Purpose

phase-48x already cleaned the obvious smoke/source defaults. phase-49x is the follow-up lane for the wording, examples, and helper comments that still over-read `--backend vm` as a default owner.

## Inventory Buckets

| Bucket | Current reading | Action |
| --- | --- | --- |
| Proof-only keep | `tools/selfhost/run_stageb_compiler_vm.sh`, `tools/selfhost/bootstrap_selfhost_smoke.sh`, `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/plugins/plugin_v2_smoke.sh`, `tools/exe_first_smoke.sh`, `tools/exe_first_runner_smoke.sh`, `tools/hako_check.sh`, `tools/engineering/parity.sh`, `tools/test_stageb_using.sh`, `tools/selfhost/program_analyze.sh`, `tools/selfhost/gen_v1_min.sh`, `tools/phi_trace_bridge_try.sh`, `tools/ny_selfhost_inline.sh` | keep explicit and non-growing |
| Compat keep | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch, `lang/src/runner/stage1_cli/core.hako`, `src/runner/modes/vm_fallback.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | keep explicit and narrow |
| Rewrite-docs | `README.md`, `docs/tools/cli-options.md`, `docs/development/selfhosting/quickstart.md`, `docs/how-to/self-hosting.md`, `docs/how-to/smokes.md`, `docs/development/runtime/cli-hakorune-stage1.md`, `docs/guides/testing-guide.md`, `docs/guides/selfhost-pilot.md`, `docs/guides/user-macros.md`, `docs/guides/exe-first-wsl.md`, `docs/guides/exception-handling.md`, `docs/guides/scope-hints.md`, `docs/guides/macro-system.md`, `docs/guides/troubleshooting/stage3-local-keyword-guide.md` | rewrite current wording so vm looks compat/proof only |
| Legacy commentary | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch comment, `src/macro/macro_box_ny.rs` compat bridge comment, `docs/development/current/selfhost/dep_tree_min_string.md` | label as legacy or compat-only, no new capability |

## Done-Enough Criteria

1. Current docs explain `rust-vm` as compat/proof keep only.
2. Current guide examples keep proof/compat routes explicit.
3. Stage-A route narration stays compat-only.
4. Hidden-default risk surfaces are labeled rather than silently re-used.
5. `cargo check --bin hakorune` stays green.
