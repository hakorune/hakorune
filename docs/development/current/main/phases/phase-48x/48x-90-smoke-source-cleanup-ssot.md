# Phase 48x SSOT: Smoke/Source Cleanup

## Intent

Clean the remaining rust-vm-facing smoke/source surfaces after phase-47x landed direct/core finalization. The purpose of this lane is not to reintroduce vm as a default owner, but to inventory the remaining live `--backend vm` references, classify them, and then clean stale smoke/source/docs entries that still make vm look like a day-to-day route.

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
