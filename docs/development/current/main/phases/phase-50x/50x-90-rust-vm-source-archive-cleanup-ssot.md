# Phase 50x SSOT: Rust VM Source / Archive Cleanup

## Intent

Clean the remaining rust-vm-facing source and helper surfaces after the current docs wording cleanup is complete. The purpose of this lane is not to restore vm as a default owner, but to inventory the remaining live `--backend vm` references, classify them, and then archive or delete only the drained surfaces.

## Inventory Snapshot

The current inventory is already split into practical buckets:

| Bucket | Representative surfaces | Read as |
| --- | --- | --- |
| Proof-only keep | `tools/selfhost/run_stageb_compiler_vm.sh`, `tools/selfhost/bootstrap_selfhost_smoke.sh`, `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/plugins/plugin_v2_smoke.sh`, `tools/exe_first_smoke.sh` | explicit VM / bridge / bootstrap proof gates that must stay non-growing |
| Compat keep | `tools/selfhost/lib/selfhost_run_routes.sh` stage-a branch, `lang/src/runner/stage1_cli/core.hako`, `src/runner/modes/vm_fallback.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | narrow fallback or legacy contract lanes that should stay explicit and non-widening |
| Archive-later | `docs/how-to/self-hosting.md`, `docs/tools/cli-options.md`, `docs/guides/testing-guide.md`, `docs/guides/selfhost-pilot.md`, `docs/guides/user-macros.md`, `docs/guides/exe-first-wsl.md`, `docs/development/current/selfhost/dep_tree_min_string.md` | wording / example surfaces that should stay current-only and be archived or rewritten once replacements are stable |
| Delete-ready only after drain | current top-level shims or wrappers that still have a caller path today | source deletion comes only after caller count goes to zero |

Important context:

- `rust-vm` is already mainline-frozen in the current docs.
- `PyVM` is historical/direct-only and is not part of the day-to-day blocker set.
- `phase-49x` finished wording cleanup and handed off to this lane.

## Task Order

1. `50xA1 residual rust-vm surface inventory lock`
   - inventory the remaining live `--backend vm` / rust-vm references
   - separate day-to-day callers from proof-only and compat keeps
2. `50xA2 proof-only / compat keep classification`
   - freeze proof-only gates as explicit keeps
   - freeze compat lanes as no-widen keeps
3. `50xB1 smoke/helper stale-route cleanup`
   - clean smoke scripts that still read like day-to-day VM callers
4. `50xB2 route-comment stale wording cleanup`
   - clean helper comments and docs that still suggest vm is a day-to-day owner
5. `50xC1 archive-ready docs/examples move`
   - move archive-ready docs/examples out of the live current surface
6. `50xC2 historical PyVM / legacy wrapper archival sweep`
   - keep PyVM tooling historical/direct-only and archive drained legacy wrappers
7. `50xD1 proof / closeout`
   - verify the cleanup remains green and hand off the next lane

## Acceptance

- day-to-day smoke/source paths no longer imply vm as the default owner
- proof-only gates and compat keeps are explicit
- docs/examples/comments match the current direct/core split
- `cargo check --bin hakorune` remains green
