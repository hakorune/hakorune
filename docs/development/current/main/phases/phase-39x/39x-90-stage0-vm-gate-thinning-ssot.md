---
Status: Active
Date: 2026-04-03
Scope: stage0/bootstrap lane の `--backend vm` 残面 inventory
---

# 39x-90 Stage0 VM Gate Thinning SSOT

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `39xA stage0 gate route inventory` | active | remaining vm-gated bootstrap surfaces を exact に inventory する |
| `39xB direct route selection` | queued | direct bootstrap mainline と explicit vm keep を分ける |
| `39xC caller drain / keep freeze` | queued | mixed routes から callers を drain し、keep set を freeze する |
| `39xD closeout` | queued | next source lane に handoff する |

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

## Active Front

- active macro wave: `39xA stage0 gate route inventory`
- active micro task: `39xA2 classify route ownership`
- next queued micro task: `39xB1 bootstrap mainline direct route`
