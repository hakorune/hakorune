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
| stage2 bootstrap proof | `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | keep or rewrite only with available stage1 binary proof |

Guardrail:
- do not delete `src/runner/pipe_io.rs` `program_json_to_mir` until these
  four callers are migrated and `user_box_decls` preservation is pinned
  outside the raw CLI route.

## Remaining `--emit-program-json-v0`

| Owner | Caller family | Next action |
| --- | --- | --- |
| stage0 direct compat | `tools/selfhost/lib/identity_routes.sh`, `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost_identity_check.sh` | keep until stage0 direct compat lane is retired |
| Stage-B Program producer | `tools/selfhost/lib/selfhost_build_stageb.sh` | keep until selfhost build route can produce MIR directly |
| hako mirbuilder fixture producer | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*` | keep as Program(JSON) fixture evidence until each family is rewritten |
| Program(JSON) contract pin | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh` | keep as explicit compat contract evidence |
| deprecation warning text | `src/runtime/deprecations.rs` | keep while raw compat flag exists |

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
   Program(JSON) fixture producers.

## Acceptance

```bash
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
rg -l -g '!tools/historical/**' -- '--emit-program-json-v0' tools src
bash tools/checks/current_state_pointer_guard.sh
```
