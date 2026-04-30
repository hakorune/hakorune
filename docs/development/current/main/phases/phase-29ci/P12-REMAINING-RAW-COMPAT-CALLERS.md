---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: P8-P11 後に残っている raw Program(JSON v0) compat caller を棚卸し、次に削る順番を固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/P6-STAGE1-MIR-ROUTE-VOCABULARY.md
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - tools/smokes/v2/lib/test_runner_builder_helpers.sh
  - tools/selfhost/lib/selfhost_build_exe.sh
  - tools/selfhost_exe_stageb.sh
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
---

# P12 Remaining Raw Compat Callers

## Goal

P8-P11 で helper-local / smoke-local fallback を削った後、残りの
`--emit-program-json-v0` / `--program-json-to-mir` caller を current owner
単位で読み直す。

Conclusion:
- P12 checkpoint では `--program-json-to-mir` live caller が 4 ファイルまで
  減っていた。
- 次の安全な削除対象は shared smoke helper fallback だった。
- P13 で shared smoke helper fallback は retired。現在の live caller は
  selfhost EXE / Stage-B delegate / phase29cg proof の 3 ファイル。
- P14 で selfhost EXE / Stage-B delegate は retired。現在の live caller は
  phase29cg proof の 1 ファイル。
- P15 で phase29cg proof は retired。current tools/src shell caller は 0。
- P16 で raw CLI implementation/config も retired。
- follow-up cleanup slices then retired the G1 identity `program-json`
  stage0/auto wrapper surface and rerouted stale helper callers
  (`tools/selfhost/stage3_same_result_check.sh`,
  `tools/dev/phase29ch_selfhost_program_json_helper_probe.sh`) onto the
  stage1 env contract instead of the retired `run_stage1_cli.sh emit
  program-json` wrapper surface.
- remaining live raw emit callers are now the two true shell keepers plus the
  active `phase29bq` fixture/contract-pin family.
- selfhost EXE / Stage-B delegate / phase29cg proof は、EXE または
  compiled-stage1 生成路の置換 proof が必要なので、同時に削らない。
- `--emit-program-json-v0` は mirbuilder fixture producer と stage0/stageB
  compat producer がまだ live なので、CLI 本体削除はまだしない。

## Remaining `--program-json-to-mir`

| Owner | Caller | Next action |
| --- | --- | --- |
| shared smoke helper fallback | `tools/smokes/v2/lib/test_runner_builder_helpers.sh` | retired in P13; non-raw builder fallback now owns this path |
| selfhost EXE helper | `tools/selfhost/lib/selfhost_build_exe.sh` | retired in P14; uses `tools/selfhost/lib/program_json_mir_bridge.sh` |
| Stage-B delegate CLI helper | `tools/selfhost_exe_stageb.sh` | retired in P14; uses `tools/selfhost/lib/program_json_mir_bridge.sh` |
| stage2 bootstrap proof | `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | retired in P15; still requires stage1 binary for full execution |

Guardrail:
- do not delete `src/runner/pipe_io.rs` `program_json_to_mir` until these
  four callers are migrated and `user_box_decls` preservation is pinned
  outside the raw CLI route.

## Remaining `--emit-program-json-v0`

| Owner | Caller family | Next action |
| --- | --- | --- |
| explicit stage1 compat/direct emit keeper | `tools/selfhost/lib/stage1_contract.sh` | keep until the explicit compat probe lane is migrated |
| Stage-B Program producer | `tools/selfhost/lib/selfhost_build_stageb.sh` | keep until selfhost build route can produce MIR directly |
| hako mirbuilder fixture producer | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*`, `tools/smokes/v2/lib/stageb_helpers.sh` | keep as Program(JSON) fixture evidence until each family is rewritten; single-fixture callers are now centralized behind `stageb_emit_program_json_v0_fixture()` |
| Program(JSON) contract pin | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh` | keep as explicit compat contract evidence |
| deprecation warning text | `src/runtime/deprecations.rs` | keep while raw compat flag exists |

Notes:
- `tools/selfhost/lib/identity_routes.sh` and `tools/selfhost_identity_check.sh`
  still route around the compat lane, but they no longer front a live
  `program-json` stage0/auto identity surface.
- `tools/selfhost/compat/run_stage1_cli.sh emit program-json` remains a retired
  wrapper contract with a dedicated smoke, not a live producer path.
- remaining bespoke `phase29bq` raw emit callers are the multi-case / cleanup /
  contract-pin scripts (`phase2`, `cleanup_try*`, and
  `phase29bq_hako_program_json_contract_pin_vm.sh`).

## Delete Order

1. Rewrite `tools/smokes/v2/lib/test_runner_builder_helpers.sh` so its final
   fallback no longer invokes raw `--program-json-to-mir`.
2. Add/keep a small representative smoke proof for that shared helper path.
3. Re-inventory `--program-json-to-mir`; if only EXE / Stage-B / dev proof
   callers remain, treat the CLI implementation as blocked by EXE/stage proof,
   not by shared smoke fallback.
4. After `--program-json-to-mir` shell callers are gone, reopen
   `src/runner/pipe_io.rs` deletion.
5. Only then start per-family migration of `phase29bq_hako_mirbuilder_*`
   Program(JSON) fixture producers, or centralize identical producer boilerplate
   behind a shared helper without weakening the pin.

## Acceptance

```bash
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
rg -l -g '!tools/historical/**' -- '--emit-program-json-v0' tools src
bash tools/checks/current_state_pointer_guard.sh
```
