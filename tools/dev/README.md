# tools/dev Active Surface

Status: Active

Decision: accepted

Scope: active developer helpers and explicit proof probes after P362A cleanup.

`tools/dev` is intentionally small. New top-level files are not allowed without
updating this inventory and `tools/checks/tools_dev_surface_inventory_guard.sh`
in the same commit.

Use this directory only for:

- interactive developer helpers that are not gate-owned yet
- explicit compat/proof probes that docs name directly
- helper implementation files paired with an active script

Do not put completed diagnostics or one-shot migration helpers here. Archive
those under `tools/archive/legacy-selfhost/engineering/`.

## Inventory

| File | Class | Owner / Removal Reading |
| --- | --- | --- |
| `README.md` | manifest | Active surface inventory. Keep in sync with `tools_dev_surface_inventory_guard.sh`. |
| `at_local_preexpand.sh` | active dev helper | Local alias pre-expander. Keep with `dev_sugar_preexpand.sh` and `docs/guides/dev-local-alias.md`. |
| `cargo_check_safe.sh` | active environment helper | EXDEV-safe cargo wrapper documented in `mir-vm-llvm-instruction-contract-fix-ssot.md`. |
| `dev_sugar_preexpand.sh` | active dev helper | Composed dev sugar pre-expander repaired in P359A. |
| `direct_loop_progression_sweep.sh` | active monitor | Direct-route loop progression monitor documented in `docs/tools/README.md` and current investigations. |
| `exdev_rename_copy_fallback.c` | paired helper | C preload implementation for `cargo_check_safe.sh`; keep/delete with that wrapper only. |
| `hako_debug_run.sh` | active debug helper | Debug runner used by trace canaries and phase132x docs. |
| `hako_preinclude.sh` | active smoke helper | Preinclude helper used by `tools/smokes/v2/lib/test_runner.sh`. |
| `phase2160_mirbuilder_module_load_probe.sh` | explicit proof keeper | Current phase2160 dehang proof; guarded by `phase216217_normalization_canary_surface_guard.sh`. |
| `phase29cg_stage2_bootstrap_phi_verify.sh` | explicit compat keeper | Program(JSON)->MIR bridge capsule proof; keep until MIR-first replacement is green. |
| `phase29ch_program_json_compat_route_probe.sh` | explicit compat keeper | Supplied Program(JSON) compat proof called by Stage1 exact emit contract smoke. |
| `phase29ck_boundary_explicit_compat_probe.sh` | explicit compat keeper | Canonical explicit compat replay proof for HAKO_CAPI_PURE boundary. |
| `phase29ck_boundary_historical_alias_probe.sh` | explicit compat keeper | Historical alias fail-fast proof; remove only after alias retirement contract changes. |
| `phase29ck_stage1_mir_dialect_probe.sh` | explicit contract keeper | Stage1 MIR dialect contract probe called by current smoke profiles. |

## Promoted Guards

These used to live under `tools/dev`, but checks own them now:

- `tools/checks/mir_builder_layer_dependency_guard.sh`
- `tools/checks/loop_pattern_context_zero_guard.sh`
- `tools/checks/phase29ca_direct_verify_dominance_block_canary.sh`

## Archived Helpers

- `tools/archive/legacy-selfhost/engineering/bug_origin_triage.sh`

## Update Rule

When a file is added, removed, archived, or promoted out of `tools/dev`:

1. Update this table in the same commit.
2. Update `tools/checks/tools_dev_surface_inventory_guard.sh`.
3. If a new `tools/checks/*.sh` file is introduced, update
   `docs/tools/check-scripts-index.md` and `tools/checks/dev_gate.sh`.
