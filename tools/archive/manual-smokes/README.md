# Manual Smoke Archive

Archived in `phase-30x / 30xG1` on `2026-04-02`.

These scripts were moved out of `tools/` root because they are no longer
current product or engineering-mainline smoke entrypoints. They remain as
historical/manual evidence only.

Archived scripts:

- `async_smokes.sh`
- `aot_counter_smoke.sh`
- `apps_tri_backend_smoke.sh`
- `cross_backend_smoke.sh`
- `hako_check_deadcode_smoke.sh`
- `llvm_smoke.sh`
- `mir_builder_exe_smoke.sh`
- `modules_smoke.sh`
- `ny_me_dummy_smoke.sh`
- `nyfmt_smoke.sh`
- `ny_parser_bridge_smoke.ps1`
- `ny_parser_run.sh`
- `ny_parser_run.ps1`
- `ny_parser_stage2_phi_smoke.sh`
- `ny_stage1_asi_smoke.sh`
- `ny_stage2_new_method_smoke.sh`
- `ny_stage2_bridge_smoke.sh`
- `ny_parser_mvp_roundtrip.sh`
- `ny_roundtrip_smoke.ps1`
- `ny_stage3_bridge_accept_smoke.sh`
- `phase24_comprehensive_smoke.sh`
- `selfhost_json_guard_smoke.sh`
- `selfhost_parser_json_smoke.sh`
- `selfhost_emitter_usings_gate_smoke.sh`
- `selfhost_progress_guard_smoke.sh`
- `smoke_aot_vs_vm.sh`
- `smoke_provider_modes.sh`
- `selfhost_stage2_smoke.sh`
- `test_filebox_fallback_smoke.sh`
- `test_joinir_freeze_inventory.sh`
- `test_loopssa_breakfinder_min.sh`
- `test_loopssa_breakfinder_slot.sh`
- `test_phase132_phi_ordering.sh`
- `test_phase133_console_llvm.sh`
- `tlv_roundtrip_smoke.sh`
- `using_prefix_strict_smoke.sh`
- `vm_filebox_smoke.sh`

Current reading:

- use role-first smoke lanes under `tools/smokes/v2/` for active checks
- keep `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh` as an explicit
  test-local manual parity harness
- `smoke_aot_vs_vm.sh` was archived in `phase-30x / 30xG3`

## Delete Policy

This folder is an archive bucket, not a permanent keeper list.

New archive entries should record these fields in the card that moves them, and
may mirror them here when the entry needs a long-lived restore note:

- `original_path`
- `archived_on`
- `archived_by_card`
- `last_known_owner`
- `delete_after`
- `restore_command`
- `delete_blocker`

An archived smoke becomes a delete candidate after 30-60 days or two cleanup
batches when all of these remain true:

- no active refs from current docs, tools, src, lang, Makefile, or root README
- no current PASS gate owns it
- no compat capsule README owns it with a reproduction command
- `docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md`
  still classifies it as unprotected

If a script needs to be restored, move it back to its original path from git
history and add an owner pointer before using it as a current gate again.
