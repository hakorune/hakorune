---
Status: Landed
Date: 2026-04-03
Scope: stage0/bootstrap lane の `--backend vm` 残面 inventory
---

# 39x-90 Stage0 VM Gate Thinning SSOT

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `39xA stage0 gate route inventory` | landed | remaining vm-gated bootstrap surfaces を exact に inventory する |
| `39xB direct route selection` | landed | direct bootstrap mainline と explicit vm keep を分ける |
| `39xC caller drain / keep freeze` | landed | mixed routes から callers を drain し、keep set を freeze する |
| `39xD closeout` | landed | focused proof を戻して successor lane に handoff する |

## Candidate Reading

| Path | State | Reading |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | mixed | Stage-B producer / direct MIR / EXE artifact / dispatcher が同居する bootstrap owner surface |
| `tools/selfhost/run_stageb_compiler_vm.sh` | vm gate | explicit Stage-B VM gate; direct route candidate とは別に扱う |
| `tools/selfhost/run.sh` | outer facade | `stage-a|exe` facade だが runtime route はまだ vm-dependent |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | keep for now | explicit bootstrap smoke gate; caller drain が進むまで freeze 対象 |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper | shell capture helper; callers を減らして thin owner に寄せる |
| `src/runner/core_executor.rs` | direct owner | already-materialized MIR(JSON) execution owner |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | raw compat no-widen lane; widening target ではない |

## Caller Inventory (39xA1 landed)

| Surface | Live caller families |
| --- | --- |
| `tools/selfhost/selfhost_build.sh` | `tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh`, `tools/smokes/v2/profiles/quick/selfhost/selfhost_build_{binop,return}_vm.sh`, `tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/selfhost/README.md`, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `tools/selfhost/run.sh`, `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`, `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_*_vm.sh`, `tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_*_vm.sh`, `tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` |
| `tools/selfhost/run.sh` | `tools/selfhost/README.md`, `docs/tools/script-index.md`, `README.md`, `README.ja.md`, `docs/development/current/main/05-Restart-Quick-Resume.md`, `tools/compat/phase29x_rust_lane_gate.sh`, `tools/selfhost_stage2_bridge_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/smokes/v2/profiles/integration/selfhost/phase29bq_*`, `tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`, `tools/smokes/v2/profiles/integration/parser/parser_*` |
| `tools/selfhost/run_stage1_cli.sh` | `tools/selfhost/stage1_mainline_smoke.sh`, `docs/development/current/main/phases/phase-39x/README.md`, `docs/development/current/main/05-Restart-Quick-Resume.md` |
| `tools/selfhost/stage1_mainline_smoke.sh` | `docs/development/current/main/phases/phase-39x/README.md`, `docs/development/current/main/10-Now.md`, `docs/development/current/main/15-Workstream-Map.md` |

## Route Classification (39xA2 landed)

| Route | Class | Note |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh --in` / Stage-B producer | `vm 必須` | Stage-B program-json production still goes through `--backend vm` today |
| `tools/selfhost/selfhost_build.sh --mir` | `direct` | direct MIR emission can bypass vm-gated producer ownership |
| `tools/selfhost/selfhost_build.sh --run` | `core_executor` | in-proc run is already handed to direct core execution owner |
| `tools/selfhost/selfhost_build.sh --exe` | `direct` | Program(JSON v0) → MIR(JSON) → ny-llvmc artifact lane |
| `tools/selfhost/run_stage1_cli.sh` | `direct` | direct Stage1 shell contract for `emit mir-json` |
| `tools/selfhost/stage1_mainline_smoke.sh` | `direct` | current Stage1 mainline proof smoke |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `vm 必須` | explicit Stage-B VM gate; keep frozen until caller drain lands |
| `tools/selfhost/run.sh --gate` / `--runtime` / `--direct` / `--steady-state` | `vm 必須` | outer facade; all current modes remain vm-dependent |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | `vm 必須` | explicit bootstrap smoke gate |
| `src/runner/modes/common_util/selfhost/child.rs` | `thin helper` | shell capture helper; route-neutral but caller-sensitive |
| `src/runner/core_executor.rs` | `direct owner` | already-materialized MIR(JSON) execution owner |
| `lang/src/runner/stage1_cli/core.hako` | `compat keep` | raw compat no-widen lane; widening target ではない |

## Explicit VM Keep Set (39xB2 landed)

| Surface | Keep reason |
| --- | --- |
| `tools/selfhost/selfhost_build.sh --in` / Stage-B producer | stage0/bootstrap producer still emits Program(JSON v0) via `--backend vm` |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit Stage-B VM gate remains the canonical keep gate |
| `tools/selfhost/run.sh --gate` / `--runtime` / `--direct` / `--steady-state` | outer facade modes remain vm-dependent until caller drain lands |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | bootstrap smoke gate stays frozen as explicit engineering keep |
| `tools/selfhost/selfhost_vm_smoke.sh` | explicit VM smoke gate stays frozen until direct bootstrap coverage replaces it |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | stage3 accept keeps the vm route visible for current bootstrap proof |

## Caller Drain Map (39xC1 landed)

| Frozen gate | Current live callers | Drain target |
| --- | --- | --- |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | `Makefile`, `README.md`, `README.ja.md`, `docs/guides/selfhost-pilot.md`, `tools/selfhost/README.md`, current phase docs (`phase-31x`/`37x`/`38x`/`39x`) | move caller docs to `run_stage1_cli.sh` / `stage1_mainline_smoke.sh` and then archive the legacy bootstrap smoke when coverage is broad enough |
| `tools/selfhost/selfhost_vm_smoke.sh` | `Makefile`, `README.md`, `README.ja.md`, `tools/selfhost/README.md`, current phase docs (`phase-31x`/`37x`/`39x`) | drain toward direct Stage1 proof or archive when VM-path parity is no longer the proof owner |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | `docs/guides/selfhost-pilot.md`, `docs/guides/exceptions-stage3.md`, `tools/selfhost/README.md`, `tools/selfhost/stage3_same_result_check.sh`, current phase docs (`phase-31x`/`37x`/`39x`) | keep as the explicit stage3 acceptance proof until direct coverage replaces the VM route |

## Active Front

- active macro wave: `39xD closeout`
- active micro task: `none`
- next queued micro task: `40xA1 archive candidate inventory`

## Closeout Summary (39xD1 landed)

- focused proof commands passed:
  - `cargo check --bin hakorune`
  - `git diff --check`
  - `bash tools/selfhost/stage1_mainline_smoke.sh`
  - `bash tools/selfhost/stage1_mainline_smoke.sh --bin target/selfhost/hakorune.stage1_cli.stage2 apps/tests/hello_simple_llvm.hako`
- `phase-39x` is landed; successor lane is `phase-40x stage0 vm archive candidate selection`.
