---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: root-level tool entrypoint inventory.
Related:
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
  - docs/development/current/main/phases/phase-29cv/P53-ROOT-TOOL-SURFACE-MANIFEST.md
---

# Tools Root Surface

This file classifies root-level shell, PowerShell, and Python entrypoints under
`tools/`. It is an inventory, not a proof gate.

Use this table with
`docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md` before
moving or deleting a root helper.

## Categories

- `protected/build`: build or packaging helper
- `protected/ci`: CI or golden-check helper
- `protected/current-smoke`: current smoke or gate wrapper
- `protected/platform`: platform-specific user entrypoint
- `protected/generator`: manifest/codegen helper
- `compat-capsule`: bounded compatibility owner
- `debug-probe`: diagnostic helper, not mainline proof
- `manual-tool`: developer convenience helper
- `manual-smoke`: manual or historical smoke wrapper
- `delete-candidate`: ready for archive/delete review

## Inventory

Owner evidence is intentionally qualitative. Exact `rg` counts drift when docs
cards are added, so decisions should follow owner/gate/capsule evidence.

| Path | Category | Owner evidence | Decision | Next action |
| --- | --- | --- | --- | --- |
| `tools/abi_manifest_codegen.py` | protected/generator | ABI manifest SSOT and generated Hako defaults | keep | maintain with ABI manifest |
| `tools/backend_runtime_decl_manifest_codegen.py` | protected/generator | runtime-decl manifest SSOT and generated Hako defaults | keep | maintain with runtime-decl manifest |
| `tools/build_aot.ps1` | protected/platform | current README and Cranelift AOT guide refs | keep | keep with Windows AOT docs owner |
| `tools/build_aot.sh` | protected/build | current README and Cranelift AOT guide refs | keep | keep with current AOT docs owner |
| `tools/build_compiler_exe.sh` | protected/build | EXE-first parser bundle builder | keep | keep while EXE-first smokes call it |
| `tools/build_hako_llvmc_ffi.sh` | protected/build | LLVM harness FFI build helper | keep | keep with LLVM harness owner |
| `tools/build_llvm.ps1` | protected/platform | Windows LLVM wrappers | keep | retire only with Windows LLVM owner |
| `tools/build_llvm.sh` | protected/build | LLVM native executable builder | keep | keep with ny-llvmc route |
| `tools/build_plugins_all.sh` | protected/build | plugin workspace build helper, P51 non-goal | keep | keep until plugin-build owner retires it |
| `tools/ci_check_golden.sh` | protected/ci | called by `tools/core_ci.sh` | keep | keep with golden MIR chain |
| `tools/codex-async-notify.sh` | manual-tool | manual AI/tmux helper | hold | archive only after workflow owner says unused |
| `tools/compare_mir.sh` | protected/ci | called by `tools/ci_check_golden.sh` | keep | keep with golden MIR chain |
| `tools/core_ci.sh` | protected/ci | CLI testing guide | keep | keep as local core CI entrypoint |
| `tools/core_method_contract_manifest_codegen.py` | protected/generator | CoreMethodContract owner box | keep | maintain with contract manifest |
| `tools/crate_exe_smoke.sh` | protected/current-smoke | crate/ny-llvmc EXE proof | keep | keep while EXE route is current |
| `tools/dev_env.sh` | manual-tool | developer profile source helper | keep | document owner if expanded |
| `tools/dev_selfhost_loop.sh` | manual-tool | selfhost iteration helper | hold | archive only after selfhost loop owner retires it |
| `tools/dev_stagea.sh` | compat-capsule | Stage-A Program(JSON v0) dev route | hold | classify with Program(JSON v0) delete-last work |
| `tools/dev_stageb.sh` | compat-capsule | Stage-B Program(JSON v0) dev route | hold | classify with Program(JSON v0) delete-last work |
| `tools/exe_first_runner_smoke.sh` | protected/current-smoke | EXE-first runner smoke | keep | keep with EXE-first proof lane |
| `tools/exe_first_smoke.sh` | protected/current-smoke | EXE-first parser bundle smoke | keep | keep with EXE-first proof lane |
| `tools/hako_check.sh` | protected/current-smoke | hako-check entrypoint | keep | keep as hako-check facade |
| `tools/hako_check_loopless_gate.sh` | protected/current-smoke | hako-check gate wrapper | keep | keep with hako-check gate owner |
| `tools/hakorune_emit_mir.sh` | compat-capsule | Hako-first Program(JSON)->MIR helper | hold | replace or archive via P33 keeper order |
| `tools/hakorune_emit_mir_compat.sh` | compat-capsule | Program(JSON)->MIR compat preset | hold | replace or archive via P33 keeper order |
| `tools/hakorune_emit_mir_mainline.sh` | compat-capsule | Program(JSON)->MIR mainline preset | hold | replace or archive via P33 keeper order |
| `tools/llvm_smoke.sh` | manual-smoke | llvmlite harness compatibility smoke | hold | move under archive/manual-smokes only after harness owner agrees |
| `tools/llvmlite_harness.py` | compat-capsule | explicit llvmlite backend keep | keep | keep while `ny_mir_builder.sh` exposes llvmlite |
| `tools/modules_smoke.sh` | protected/current-smoke | modules JSON VM smoke | hold | route into smoke v2 or archive after owner check |
| `tools/native_llvm_builder.py` | compat-capsule | `ny_mir_builder.sh` native backend canary | hold | capsule-classify before any move |
| `tools/ny_mir_builder.sh` | protected/build | MIR JSON to obj/exe wrapper | keep | keep as ny-llvmc route facade |
| `tools/ny_parser_bridge_smoke.sh` | protected/current-smoke | current parser bridge smoke | keep | keep current |
| `tools/ny_parser_mvp.py` | compat-capsule | Python MVP parser used by bridge smokes | keep | keep while Stage-2 bridge smokes use it |
| `tools/ny_roundtrip_smoke.sh` | protected/current-smoke | current Ny roundtrip smoke | keep | keep current |
| `tools/ny_stage2_shortcircuit_smoke.sh` | protected/current-smoke | current Stage-2 parser smoke | keep | keep current |
| `tools/opbox-json.sh` | manual-smoke | OperatorBox JSON smoke shortcut | hold | route into smoke v2 or archive after opbox owner check |
| `tools/opbox-quick.sh` | manual-smoke | OperatorBox quick smoke shortcut | hold | route into smoke v2 or archive after opbox owner check |
| `tools/phi_trace_bridge_try.sh` | debug-probe | PHI trace bridge experiment | hold | move to debug/archive with PHI owner decision |
| `tools/phi_trace_check.py` | debug-probe | PHI trace validator used by PHI probes | keep | keep while PHI probes exist |
| `tools/phi_trace_run.sh` | debug-probe | PHI troubleshooting guide | hold | move to debug/archive with PHI owner decision |
| `tools/run_llvm_harness.sh` | compat-capsule | explicit LLVM harness compat/probe lane | keep | keep with LLVM harness owner |
| `tools/selfhost_exe_stageb.sh` | compat-capsule | Program(JSON)->MIR bridge capsule plus direct probe | keep | split or strengthen capsule owner before delete-last |
| `tools/selfhost_identity_check.sh` | compat-capsule | Stage1/Stage2 identity comparison | hold | align with Stage1 contract keeper order |
| `tools/selfhost_read_tmp_dev_smoke.sh` | manual-smoke | retired tmp-only selfhost dev smoke | hold | archive after selfhost tmp owner check |
| `tools/selfhost_stage2_bridge_smoke.sh` | protected/current-smoke | Stage-2 bridge smoke | keep | keep while bridge proof is current |
| `tools/smoke_plugins.sh` | protected/current-smoke | plugin smoke wrapper | keep | keep or route into smoke v2 index |
| `tools/snapshot_mir.sh` | protected/ci | called by `tools/compare_mir.sh` | keep | keep with golden MIR chain |
| `tools/using_e2e_smoke.sh` | protected/current-smoke | using E2E smoke | keep | keep while using gate is current |
| `tools/using_resolve_smoke.sh` | protected/current-smoke | using resolve smoke | keep | keep while using gate is current |
| `tools/using_strict_path_fail_smoke.sh` | protected/current-smoke | using strict failure smoke | keep | keep while using gate is current |
| `tools/using_unresolved_smoke.sh` | protected/current-smoke | using unresolved smoke | keep | keep while using gate is current |
| `tools/validate_mir_json.py` | protected/build | MIR JSON schema validator | keep | keep with JSON schema docs |
| `tools/vm_plugin_smoke.sh` | manual-smoke | plugin smoke wrapper over archived smoke profiles | hold | route into smoke v2 or archive after plugin owner check |

## Current Hold Queue

Do not move these without a focused card:

- backend canary: `tools/native_llvm_builder.py`
- PHI probes: `tools/phi_trace_bridge_try.sh`, `tools/phi_trace_run.sh`
- Program(JSON v0) capsules: `tools/dev_stagea.sh`, `tools/dev_stageb.sh`,
  `tools/hakorune_emit_mir.sh`, `tools/hakorune_emit_mir_compat.sh`,
  `tools/hakorune_emit_mir_mainline.sh`, `tools/selfhost_exe_stageb.sh`
- smoke shortcuts that need smoke-v2 owner decisions:
  `tools/llvm_smoke.sh`, `tools/modules_smoke.sh`, `tools/opbox-json.sh`,
  `tools/opbox-quick.sh`, `tools/vm_plugin_smoke.sh`
