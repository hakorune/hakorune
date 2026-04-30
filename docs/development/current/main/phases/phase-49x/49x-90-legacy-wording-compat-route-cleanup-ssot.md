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
| Proof-only keep | `tools/selfhost/run_stageb_compiler_vm.sh`, `tools/selfhost/bootstrap_selfhost_smoke.sh`, `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/plugins/plugin_v2_smoke.sh`, `tools/exe_first_smoke.sh`, `tools/exe_first_runner_smoke.sh`, `tools/hako_check.sh`, `tools/engineering/parity.sh`, `tools/archive/legacy-selfhost/engineering/test_stageb_using.sh`, `tools/archive/legacy-selfhost/engineering/program_analyze.sh`, `tools/archive/legacy-selfhost/engineering/gen_v1_min.sh`, `tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh`, `tools/phi_trace_bridge_try.sh` | keep explicit and non-growing |
| Compat keep | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch, `lang/src/runner/stage1_cli/core.hako`, `src/runner/modes/vm_fallback.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | keep explicit and narrow |
| Rewrite-docs | `README.md`, `docs/tools/cli-options.md`, `docs/development/selfhosting/quickstart.md`, `docs/how-to/self-hosting.md`, `docs/how-to/smokes.md`, `docs/development/runtime/cli-hakorune-stage1.md`, `docs/guides/testing-guide.md`, `docs/guides/selfhost-pilot.md`, `docs/guides/user-macros.md`, `docs/guides/exe-first-wsl.md`, `docs/guides/exception-handling.md`, `docs/guides/scope-hints.md`, `docs/guides/macro-system.md`, `docs/guides/troubleshooting/stage3-local-keyword-guide.md` | rewrite current wording so vm looks compat/proof only |
| Legacy commentary | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch comment, `src/macro/macro_box_ny.rs` compat bridge comment, `docs/development/current/selfhost/dep_tree_min_string.md` | label as legacy or compat-only, no new capability |

## Current-Docs Inventory Snapshot

The current-doc inventory found the following stale wording hotspots:

| File | Current reading | Cleanup direction |
| --- | --- | --- |
| `README.md` | `engineering/bootstrap` phrasing still appears in the backend-roles section and the phase-15 legacy ingress note | rewrite as compat/proof keep and legacy ingress only |
| `docs/tools/cli-options.md` | raw CLI default still mentions `vm` in a way that can read like the operational default | keep the wording but make the legacy-ingress nature explicit |
| `docs/development/selfhosting/quickstart.md` | still reads as `compat/proof keep` in the main lane summary, but needs the default-vm wording kept clearly historical | keep as compat/proof and historical-only |
| `docs/how-to/self-hosting.md` | compat/proof examples still show `--backend vm` | keep as explicit compat/proof examples |
| `docs/how-to/smokes.md` | already reads `rust-vm 系 = compat/proof keep` | keep |
| `docs/development/runtime/cli-hakorune-stage1.md` | Stage1 CLI / rust-vm text still carried `engineering/bootstrap` style language | rewrite to compat/proof lane language |
| `docs/guides/testing-guide.md` | VM examples are still present but marked compat/proof keep | keep examples, but keep labels explicit |
| `docs/guides/selfhost-pilot.md` | raw default `vm` and rust-vm route language still appear as live guidance | rewrite to compat/proof / legacy ingress wording |
| `docs/guides/user-macros.md` | multiple compat/proof `--backend vm` examples | keep as compat/proof examples |
| `docs/guides/exe-first-wsl.md` | compat/proof example mentions `--backend vm` | keep as compat/proof example |
| `docs/guides/exception-handling.md` | example comment still uses `--backend vm` | label as example-only, not daily route |
| `docs/guides/scope-hints.md` | MIR-building example mentions `--backend vm` generically | rewrite with compat/proof label |
| `docs/guides/macro-system.md` | compat/proof example uses `--backend vm` | keep as explicit compat/proof example |
| `docs/guides/troubleshooting/stage3-local-keyword-guide.md` | generic `--backend vm` examples still look like everyday commands | rewrite as legacy/proof examples |

This snapshot is the basis for the phase-49x rewrite order. It intentionally keeps proof/compat examples in place and only targets wording that still makes vm look like a day-to-day owner.

## Done-Enough Criteria

1. Current docs explain `rust-vm` as compat/proof keep only.
2. Current guide examples keep proof/compat routes explicit.
3. Stage-A route narration stays compat-only.
4. Hidden-default risk surfaces are labeled rather than silently re-used.
5. `cargo check --bin hakorune` stays green.
