# Phase 48x SSOT: Smoke/Source Cleanup

## Intent

Clean the remaining rust-vm-facing smoke/source surfaces after phase-47x landed direct/core finalization. The purpose of this lane is not to reintroduce vm as a default owner, but to inventory the remaining live `--backend vm` references, classify them, and then clean stale smoke/source/docs entries that still make vm look like a day-to-day route.

## Inventory Snapshot

The current inventory is already split into three practical buckets:

| Bucket | Representative surfaces | Read as |
| --- | --- | --- |
| Proof-only keep | `tools/selfhost/run_stageb_compiler_vm.sh`, `tools/selfhost/bootstrap_selfhost_smoke.sh`, `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/plugins/plugin_v2_smoke.sh`, `tools/exe_first_smoke.sh` | explicit VM / bridge / bootstrap proof gates that must stay non-growing |
| Compat keep | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch, `lang/src/runner/stage1_cli/core.hako`, `src/runner/modes/vm_fallback.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | narrow fallback or legacy contract lanes that should stay explicit and non-widening |
| Stale cleanup candidates | `tools/archive/legacy-selfhost/engineering/program_analyze.sh`, `tools/archive/legacy-selfhost/engineering/gen_v1_min.sh`, `tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh`, `tools/test_stageb_using.sh`, `tools/selfhost_compiler_smoke.sh`, `tools/ny_selfhost_using_smoke.sh`, `tools/apps_tri_backend_smoke.sh`, `src/macro/macro_box_ny.rs`, `src/runner/modes/common.rs`, `docs/how-to/self-hosting.md`, `docs/tools/cli-options.md`, `docs/guides/testing-guide.md`, `docs/guides/selfhost-pilot.md`, `docs/guides/user-macros.md`, `docs/guides/exe-first-wsl.md`, `docs/development/current/selfhost/dep_tree_min_string.md` | stale day-to-day `--backend vm` wording or helper routes that should be cleaned or rewritten next |

The important non-targets for this lane are already drained away from the day-to-day mainline:

- `tools/selfhost/lib/selfhost_build_stageb.sh` is no longer a default VM caller.
- `tools/selfhost/run.sh` defaults to the temp-MIR -> `--mir-json-file` route for `exe`.
- `src/runner/modes/common_util/selfhost/stage0_capture.rs` is route-neutral.
- `src/runner/modes/common_util/selfhost/stage_a_route.rs` and `stage_a_compat_bridge.rs` are already split away from the old VM-fixed builder ownership.

The inventory/classification cut is now locked:

- `48xA1` inventory lock is landed.
- `48xA2` proof-only / compat keep classification is landed.
- `48xB1` smoke script stale-route cleanup is landed; `48xB2` proof-only smoke gate lock is landed; `48xC1` source helper stale-route cleanup is landed; `48xC2` vm.rs / vm_fallback thin keep trim is the active cleanup move, with the current proof/compat scripts and helpers tagged accordingly.

## Task Order

1. `48xA1 residual vm surface inventory lock`
   - inventory the remaining live `--backend vm` / rust-vm references
   - separate day-to-day callers from proof-only and compat keeps
2. `48xA2 proof-only / compat keep classification`
   - freeze proof-only gates as explicit keeps
   - freeze compat lanes as no-widen keeps
3. `48xB1 smoke script stale-route cleanup`
   - clean smoke scripts that still read like day-to-day VM callers
4. `48xB2 proof-only smoke gate lock`
   - keep proof-only gates explicit and non-growing
5. `48xC1 source helper stale-route cleanup`
   - trim source helpers and shell helpers that still leak vm as a default owner
6. `48xC2 vm.rs / vm_fallback thin keep trim`
   - keep the VM core/fallback as thin proof/oracle rails only
7. `48xD1 README/example command cleanup`
   - remove stale `--backend vm` examples from docs and readmes
8. `48xD2 stale `--backend vm` commentary cleanup`
   - update comments and helper docs that still suggest vm is the day-to-day owner
9. `48xE1 proof / closeout`
   - verify the cleanup remains green and hand off the next lane

## Acceptance

- day-to-day smoke/source paths no longer imply vm as the default owner
- proof-only gates and compat keeps are explicit
- docs/examples/comments match the current direct/core split
- `cargo check --bin hakorune` remains green
