---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: record that the remaining Program(JSON v0) cleanup work is now blocked on explicit keeper replacement, not additional thin seam deletion.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh
  - tools/lib/program_json_v0_compat.sh
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost/lib/stage1_contract.sh
  - src/runner/stage1_bridge/README.md
---

# P33 Delete-Last Blockers Only

## Goal

Keep the lane honest after the thin wrapper/seam cleanup work.

By P32, the small wrapper-local and probe-local dead seams in the shell/test
layers have been exhausted. The remaining `Program(JSON v0)` work is no longer
about deleting one more local helper. It is about replacing or retiring
explicit keepers in the right order.

## Decision

- record that shell/test thin-seam cleanup is effectively complete through P32
- treat the remaining work as explicit keeper replacement or final delete-last
  work only
- keep the public/Rust compat surface frozen as delete-last until every shell
  or probe keeper has a replacement/archive owner

## Active Blockers

1. Explicit Stage-B artifact diagnostic keeper
   - archived: `tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh`
   - archived: `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh`
   - archived: `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh`
   - `tools/lib/program_json_v0_compat.sh`
   - live debt is now the shared raw emit helper, not the manual artifact/dev
     probe entries
   - the empty active `tools/dev/program_json_v0/` marker is archived as
     `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_capsule_README.md`
2. Explicit Program(JSON)->MIR bridge keepers
   - `tools/selfhost/lib/program_json_mir_bridge.sh`
   - `tools/selfhost_exe_stageb.sh`
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
3. Stage1 contract keepers
   - `tools/selfhost/lib/stage1_contract.sh`
   - `tools/selfhost/compat/run_stage1_cli.sh`
   - `tools/dev/phase29ch_program_json_compat_route_probe.sh`
   - archived diagnostics-only probes:
     `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh`,
     `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh`,
     `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh`,
     `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh`,
     `tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh`
4. Fixture contract keepers
   - `tools/smokes/v2/lib/stageb_helpers.sh`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*`
   - `tools/smokes/v2/profiles/integration/stageb/*`
   - `tools/smokes/v2/profiles/integration/core_direct/*`
   - related budget/quick core fixture gates
5. Rust/public delete-last surface
   - `src/runtime/deprecations.rs`
   - `src/stage1/program_json_v0*`
   - `src/runner/stage1_bridge/**`

## Non-goals

- do not claim further thin wrapper cleanup remains unless a new dead seam is
  proven with repo references
- do not delete the Rust/public compat surface while the explicit shell/test
  keepers above still exist
- do not weaken proof probes just to make delete-last look closer

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
