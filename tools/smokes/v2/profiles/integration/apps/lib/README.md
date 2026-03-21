# Integration App Smoke Lib

This directory is the shared helper layer for the remaining `tools/smokes/v2/profiles/integration/apps/*.sh` families that have not yet been split into semantic subdirectories.

## Files

- `vm_hako_json_parity_common.sh`
  - Shared parity harness for vm-hako JSON-route smokes.
- `vm_hako_phase.sh`
  - Reads `VM_HAKO_PHASE` from `src/runner/modes/vm_hako.rs`.
- `phase29y_binary_only_common.sh`
  - Shared workspace/bootstrap helpers for binary-only stage1 probes
    (`phase29y_hako_emit_mir_binary_only_*`, `phase29y_hako_run_binary_only_*`,
    `phase29y_hako_binary_only_selfhost_readiness_vm.sh`).
  - Owns fixture/binary preflight, temp workdir setup/cleanup, and pinned env execution profile.

## Migration Note

- `vm_hako_caps_common.sh` moved to `tools/smokes/v2/profiles/integration/vm_hako_caps/lib/vm_hako_caps_common.sh`.
- The vm-hako capability family now lives under `tools/smokes/v2/profiles/integration/vm_hako_caps/`.
- `phase29cc_wsm_g3_canvas_contract_common_vm.sh` moved to `tools/smokes/v2/profiles/integration/phase29cc_wsm/g3_canvas/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh`.
- The first `phase29cc_wsm` split now lives under `tools/smokes/v2/profiles/integration/phase29cc_wsm/g3_canvas/`.
- `phase29cc_wsm_g2_*` browser bridge cases moved to `tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/`.
- `phase29cc_wsm_g4_*` playground/browser progression cases moved to `tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/`.
- `phase21_5_perf_apps_*` singleton bundle cases moved to:
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/case_breakdown/`
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/compile_run_split/`
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/crosslang_bundle/`
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/emit_mir_jsonfile_route/`
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/startup_subtract/`

## Usage Rule

When adding a new smoke under the remaining `apps`-based families:

1. `source` the relevant shared helper first.
2. Use helper functions for preflight and runtime checks instead of copy-pasting local logic.
3. Keep case-specific logic local to the smoke script (fixture path, jq shape contract, expected rc only).

This keeps the remaining app-based smokes consistent and prevents contract drift.
