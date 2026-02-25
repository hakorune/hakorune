# Integration App Smoke Lib

This directory is the shared helper layer for `tools/smokes/v2/profiles/integration/apps/*.sh`.

## Files

- `vm_hako_caps_common.sh`
  - Shared preflight/runtime checks for `vm_hako_caps_*_vm.sh`.
  - Owns fixture existence check, MIR emit check, timeout handling, and standard vm-hako tag assertions.
- `vm_hako_json_parity_common.sh`
  - Shared parity harness for vm-hako JSON-route smokes.
- `vm_hako_phase.sh`
  - Reads `VM_HAKO_PHASE` from `src/runner/modes/vm_hako.rs`.
- `phase29y_binary_only_common.sh`
  - Shared workspace/bootstrap helpers for binary-only stage1 probes
    (`phase29y_hako_emit_mir_binary_only_*`, `phase29y_hako_run_binary_only_*`,
    `phase29y_hako_binary_only_selfhost_readiness_vm.sh`).
  - Owns fixture/binary preflight, temp workdir setup/cleanup, and pinned env execution profile.

## Usage Rule

When adding a new `vm_hako_caps_*_vm.sh` smoke:

1. `source "$(dirname "$0")/lib/vm_hako_caps_common.sh"` first.
2. Use helper functions for preflight and runtime checks instead of copy-pasting local logic.
3. Keep case-specific logic local to the smoke script (fixture path, jq shape contract, expected rc only).

This keeps capability smokes consistent and prevents contract drift between C01/C02/C03 and future cases.

## vm_hako_caps Naming Contract

- Canonical script naming:
  - `vm_hako_caps_<capability>_ported_vm.sh`
  - Use `*_ported_vm.sh` as the canonical script for success contracts.
- Compatibility wrapper naming:
  - `vm_hako_caps_<capability>_block_vm.sh`
  - Keep this only as a compatibility wrapper when legacy callers exist, and delegate via `exec` to the canonical script.
- Blocked pin naming:
  - Use an explicit capability suffix for blocker pins (example: `vm_hako_caps_app1_stack_overflow_after_open_block_vm.sh`).
  - Do not add a `ported` wrapper for blocker pins; register them explicitly in `phase29y_vm_hako_caps_gate_vm.sh`.
