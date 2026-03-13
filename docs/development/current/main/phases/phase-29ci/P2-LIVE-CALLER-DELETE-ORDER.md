---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` の `.hako` live/bootstrap callers と `shell helper keep` の delete-order を exact caller bucket で固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - CURRENT_TASK.md
  - src/stage1/program_json_v0/README.md
---

# P2 Live Caller Delete Order

## Goal

`Program(JSON v0)` boundary の外側に残っている caller を、

- `.hako` live/bootstrap owner
- shared shell helper keep
- diagnostics/probe keep
- test-only smoke tail

へ分けて、Rust-only delete slice と混ざらない順序を固定する。

## Exact Caller Buckets

### `.hako` live/bootstrap owners

- `lang/src/mir/builder/MirBuilderBox.hako`
- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/launcher.hako`

この 4 file は live/bootstrap owner であり、`BuildBox.emit_program_json_v0(...)` または `MirBuilderBox.emit_from_program_json_v0(...)` をまだ実呼びしている。

### Shared shell helper keep

- `tools/hakorune_emit_mir.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/smokes/v2/lib/test_runner.sh`

この 3 file は shared helper contract として扱い、smoke tail と同じ patch に混ぜない。

### Diagnostics / probe keep

- `tools/dev/phase29ch_program_json_helper_exec_probe.sh`
- `tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh`
- `tools/dev/phase29ch_selfhost_program_json_helper_probe.sh`

この 3 file は diagnostics-only keep であり、live caller の後ろへ回す。

### Test-only smoke tail

`tools/smokes/v2/profiles/integration/core/**` 配下に、JSON v0 boundary を直接叩く test-only shell/apps caller が残っている。

主な群:

- `phase2034/mirbuilder_*`
- `phase2043/program_new_array_delegate_struct_canary_vm.sh`
- `phase2160/builder_min_*`
- `phase2160/registry_optin_*`（shared launch collapse は進み、explicit keep は direct-lower probe 1本）

この tail は shared helper でも live/bootstrap owner でもないので、caller-audit 用の後段 bucket として扱う。

## Fixed Delete Order

1. Rust-only bucket (`build surrogate keep`, `future-retire bridge`) を先に closeout-ready にする
2. `.hako` live/bootstrap owner 4 file の caller contract を audit する
3. shared shell helper keep 3 file を audit する
4. test-only smoke tail 43 file を caller-audit bucket として整理する
5. diagnostics/probe keep は live caller の後ろで retire 判断する

## Guardrails

- `.hako` live/bootstrap owner と shared shell helper を同じ patch で消さない
- smoke tail 43 file を shared helper keep と誤って同一 bucket にしない
- diagnostics/probe keep を live caller より先に retire しない
- current authority (`emit_program_json_v0_for_strict_authority_source(...)`) には触らない

## Retreat Finding

- bridge 内側が closeout-ready に近づいても、boundary 外側にはまだ 4 `.hako` owner + 3 shared helper + 43 test-only smoke caller が残っている
- `lang/src/runner/launcher.hako` は direct Program(JSON) / MIR checked path を owner-local helper へまとめられるので、`.hako` owner audit は 4 file を同時に触らず 1 owner ずつ薄くしていくのが安全
- `lang/src/runner/stage1_cli.hako` も direct `BuildBox.emit_program_json_v0(...)` checked path を owner-local helper に寄せられるので、`.hako` owner audit は runner file ごとに local helper 化を進めるのが安全
- `lang/src/runner/stage1_cli_env.hako` も authority box 内の direct `BuildBox.emit_program_json_v0(...)` path を same-file helper に寄せられるので、env-route owner でも direct checked path を増やさずに薄くできる
- `lang/src/mir/builder/MirBuilderBox.hako` は `emit_from_source_v0(...)` の source-entry shim を local helper 化できるが、`emit_from_program_json_v0(...)` 本体は owner policy が濃いので同じ slice で混ぜない方が安全
- したがって、次の delete slice を shell helper や `.hako` owner へ広げると scope が跳ねる
- 次の実 caller audit は `.hako` owner 4 file を先に主語にし、shared helper 3 file と smoke tail 43 file は別 bucket として扱う

## Immediate Next

1. `.hako` live/bootstrap owner 4 file の direct call sites を owner-local に整理する
2. shared shell helper 3 file の contract を keep/remove 目線で audit する
3. smoke tail 43 file は caller-audit ledger として別に畳む
