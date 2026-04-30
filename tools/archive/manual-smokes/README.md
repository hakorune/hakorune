# Manual Smoke Archive

Archived in `phase-30x / 30xG1` on `2026-04-02`.

These scripts were moved out of `tools/` root because they are no longer
current product or engineering-mainline smoke entrypoints. They remain as
historical/manual evidence only.

Archived scripts:

- `async_smokes.sh`
- `apps_tri_backend_smoke.sh`
- `cross_backend_smoke.sh`
- `ny_stage1_asi_smoke.sh`
- `ny_stage3_bridge_accept_smoke.sh`
- `smoke_aot_vs_vm.sh`
- `selfhost_stage2_smoke.sh`

Current reading:

- use role-first smoke lanes under `tools/smokes/v2/` for active checks
- keep `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh` as an explicit
  test-local manual parity harness
- `smoke_aot_vs_vm.sh` was archived in `phase-30x / 30xG3`
