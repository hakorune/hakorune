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

- This is the first outer caller wave: keep it separated from `P3-SHARED-SHELL-HELPER-AUDIT.md`, and do not mix shared shell helper edits into this bucket.

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

現在の caller-audit ledger:

- Bucket A: uniform raw `verify_program_via_builder_to_core` callers
  - `phase2041/*`
  - `phase2042/*`
  - `phase2043/mirbuilder_prefer_mirbuilder_*`
  - `phase2043/mirbuilder_*builder_only*`
  - `phase2111/mirbuilder_registry_*`
  - status: landed behind named runner helpers in `tools/smokes/v2/lib/test_runner.sh`
- Bucket B: special raw verify keeps
  - `phase2039/parser_embedded_json_canary.sh`
  - `phase2043/mirbuilder_internal_new_array_core_exec_canary_vm.sh`
  - status: next exact bucket
- Bucket C: already-thin wrapper families
  - `phase2044/*`
  - `phase2160/builder_min_*`
  - `phase2160/registry_optin_*`
  - status: thin keep / monitor-only by default
- Bucket D: MIR-file verify wrappers
  - `phase2170/*`
  - status: later separate bucket

この tail は shared helper でも live/bootstrap owner でもないので、caller-audit 用の後段 bucket として扱う。

## Fixed Delete Order

1. Rust-only bucket (`build surrogate keep`, `future-retire bridge`) を先に closeout-ready にする
2. `.hako` live/bootstrap owner 4 file の caller contract を audit する
3. shared shell helper keep 3 file を audit する
4. test-only smoke tail 43 file を caller-audit bucket として整理する
   - first bucket: uniform raw verify callers (landed)
   - second bucket: special raw verify keeps
   - later buckets: already-thin wrapper families and MIR-file verify wrappers
5. diagnostics/probe keep は live caller の後ろで retire 判断する

## Guardrails

- `.hako` live/bootstrap owner と shared shell helper を同じ patch で消さない
- smoke tail 43 file を shared helper keep と誤って同一 bucket にしない
- diagnostics/probe keep を live caller より先に retire しない
- current authority (`emit_program_json_v0_for_strict_authority_source(...)`) には触らない

## Retreat Finding

- bridge 内側が closeout-ready に近づいても、boundary 外側にはまだ 4 `.hako` owner + 3 shared helper + 43 test-only smoke caller が残っている
- `lang/src/runner/launcher.hako` は direct Program(JSON) / MIR checked path を owner-local helper へまとめ、top-level route selection も `LauncherDispatchBox` に寄せ済みなので、`.hako` live/bootstrap owner bucket は near-thin-floor に固定してよい
- `lang/src/runner/stage1_cli.hako` も direct `BuildBox.emit_program_json_v0(...)` / `MirBuilderBox.emit_from_program_json_v0(...)` checked path を owner-local helper に寄せられるので、`.hako` owner audit は runner file ごとに local helper 化を進めるのが安全
- `lang/src/runner/stage1_cli_env.hako` も exact source-only `emit-program` を same-file helper `Stage1SourceProgramAuthorityBox` に寄せ、さらに direct `MirBuilderBox.emit_from_program_json_v0(...)` checked path も shared same-file helper (`Stage1ProgramJsonMirCallerBox`) に寄せられるので、env-route owner でも exact emit contract を戻しつつ direct checked path を増やさずに薄くできる
- `lang/src/mir/builder/MirBuilderBox.hako` は `emit_from_source_v0(...)` の source-entry shim を `MirBuilderSourceCompatBox` に寄せ済みで、`emit_from_program_json_v0(...)` 本体は owner policy が濃いので同じ slice で混ぜない方が安全
- したがって、次の delete slice を shell helper や `.hako` owner へ広げると scope が跳ねる
- 次の実 caller audit は `.hako` owner 4 file を先に主語にし、shared helper 3 file と smoke tail 43 file は別 bucket として扱う

## Immediate Next

1. `.hako` live/bootstrap owner 4 file は monitor-only / near-thin-floor として凍結する
2. shared shell helper 3 file の contract を keep/remove 目線で audit する
3. first helper-local slice は `tools/hakorune_emit_mir.sh`
4. smoke tail 43 file は caller-audit ledger として別に畳む
5. first smoke-tail bucket は landed; next exact bucket は special raw verify keeps に固定する
