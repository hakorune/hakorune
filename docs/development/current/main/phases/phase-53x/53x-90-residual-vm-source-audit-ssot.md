---
Status: Active
Date: 2026-04-04
---

# 53x-90 Residual VM Source Audit SSOT

## One-line Read

Inventory residual rust-vm / vm-hako source surfaces, classify keep-now / archive-later / delete-ready, and peel only drained rust-vm leftovers while keeping vm-hako reference/conformance live.

## Why this phase exists

- `rust-vm` is no longer a mainline owner, but several proof-only / compat keep surfaces still exist.
- `vm-hako` is still a live reference/conformance lane and must not be archived wholesale.
- The repo still has active source and smoke edges that mention `--backend vm`, `vm-hako`, or proof-only compat callers.
- The task is to finish the residual source audit without re-widening live ownership.

## Inventory Targets

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/vm_hako.rs`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/plugins/plugin_v2_smoke.sh`
- `tools/smokes/v2/profiles/integration/vm_hako_caps/**`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/suites/integration/vm-hako-core.txt`

## Classification Rules

- `keep-now`
  - proof-only / compat keep surfaces that still have a live caller or a live reference lane
- `archive-later`
  - stale docs, wrappers, or commentary that no longer carry current ownership but still document history
- `delete-ready`
  - drained source / helper / wrapper surfaces with no remaining caller and no reference lane

## Guard Rails

- do not archive or delete `vm-hako` wholesale
- do not restore `--backend vm` as a day-to-day default caller path
- do not widen compat proof gates while classifying the residue
- leave canonical historical traces in archive docs, not in active source wording

