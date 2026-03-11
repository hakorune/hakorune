---
Status: SSOT
Scope: `selfhost-bootstrap-route-ssot.md` から分離した evidence / legacy lane / diagnostics companion。
Related:
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md
---

# Selfhost Bootstrap Route Evidence And Legacy Lanes

## Purpose

`selfhost-bootstrap-route-ssot.md` は current active contract を読む入口に保ち、
この文書は legacy lane / binary-only contracts / blocker capture / supplemental evidence を保持する。

## Active Evidence Notes

- `phase-29ch` current reduced authority evidence と diagnostics registry は
  `docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md`
  を正本とする。
- G1 compare policy の詳細は
  `docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md`
  を正本とする。

## Binary-only `--hako-emit-mir-json` Contract (lane B)

目的:
- `hakorune` 単体バイナリで Stage1 emit-mir route を実行可能にし、repo checkout 依存を除去する。

定義（success）:
- repo外ディレクトリで次が成功すること。
  - `./hakorune --hako-emit-mir-json /tmp/out.mir ./input.hako`
- MIR(JSON) が `/tmp/out.mir` に生成されること。
- fail時は stage1 fail-fast 契約（例: `[stage1-cli] ...`）で終了し、silent fallback しないこと。
- timeout diagnostics（non-gating）:
  - pin: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - timeout source: `src/config/env/selfhost_flags.rs` `ny_compiler_emit_timeout_ms()`（unset時は `ny_compiler_timeout_ms()`） / `src/runner/stage1_bridge/mod.rs` `spawn_with_timeout(...)`
  - debug repro: `NYASH_STAGE1_EMIT_TIMEOUT_MS=12000 bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`

許可する外部依存:
- 入力ソース `input.hako`
- 出力先 `/tmp/out.mir`

禁止する内部依存（binary-only 達成条件）:
- `lang/src/**` の `.hako` ファイル読込
- `hako.toml` / `hakorune.toml` / `nyash.toml` の読込
- `*_module.toml` の読込

現状フロー（2026-02-18）:
1. Rust parent が stage1 route を起動する。
2. repo内（`lang/src/**` あり）では Stage1 child が using 解決・Stage-B・MirBuilder を実行する。
3. repo外（`lang/src/**` なし）では `NYASH_STAGE1_BINARY_ONLY_DIRECT=1` 明示時のみ binary-only direct route が Rust 側で MIR を生成して出力する（unset は OFF）。
4. どちらの経路でも parent が MIR(JSON) を出力ファイルへ書き込む。

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `BINARY-ONLY-B01` [done, 2026-02-18]: blocked smoke を追加し、repo外実行での現状依存を固定する。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_block_vm.sh`
   - note: B04 完了後は歴史互換ラッパー（legacy alias）として `*_ported_vm.sh` へ委譲（active contract は B04）。
2. `BINARY-ONLY-B02` [done, 2026-02-18]: `stage1_cli.hako` のファイル依存を埋め込みへ移す（default route の entry path 外部依存を撤去）。
3. `BINARY-ONLY-B03` [done, 2026-02-18]: modules map 依存（TOML / module manifests）を埋め込み snapshot へ移す（`NYASH_STAGE1_MODULES_SOURCE=toml` で従来収集に切替可能）。
4. `BINARY-ONLY-B04` [done, 2026-02-18]: binary-only smoke を ported へ昇格し、lane B monitor-only へ戻す。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
   - note: smoke 実行時は `NYASH_STAGE1_BINARY_ONLY_DIRECT=1` を明示して binary-only direct route を起動する。

運用メモ（命名）:
- active contract 実行は `phase29y_hako_emit_mir_binary_only_ported_vm.sh` を正本とする。
- `phase29y_hako_emit_mir_binary_only_block_vm.sh` は B01 履歴互換の legacy alias としてのみ保持する。
- legacy alias の撤去条件: `CURRENT_TASK.md` / `phase-29y` docs / 呼び出しスクリプトから `*_block_vm.sh` 参照が 0 件になった時点。

観測ルール（計測）:
- `strace -ff -e openat` で `--hako-emit-mir-json` 実行時の read を観測する。
- 非ゲートの monitor smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
- drift 疑い時のみ strict triage:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh --strict`

## Binary-only `--hako-run` Contract (lane B)

目的:
- `hakorune` 単体バイナリで Stage1 run route を実行可能にし、repo checkout 依存を除去する。

定義（success）:
- repo外ディレクトリで次が成功すること。
  - `./hakorune --backend vm --hako-run ./input.hako`
- fail時は stage1 fail-fast 契約で終了し、silent fallback しないこと。

禁止する内部依存（binary-only 達成条件）:
- `lang/src/**` の `.hako` ファイル読込
- `hako.toml` / `hakorune.toml` / `nyash.toml` の読込
- `*_module.toml` の読込

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `BINARY-ONLY-RUN-01` [done, 2026-02-19]: blocked smoke を追加し、repo外 `--hako-run` の現状 blocker（`lang/src/**` read fail-fast）を固定する。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_block_vm.sh`
2. `BINARY-ONLY-RUN-02` [done, 2026-02-19]: stage1 run route に binary-only direct route を追加し、repo file 依存（`lang/src/**` read）を撤去する。
3. `BINARY-ONLY-RUN-03` [done, 2026-02-19]: repo外 `--hako-run` を ported 契約へ昇格し、monitor-only へ戻す。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
   - note: smoke 実行時は `NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1`（または `NYASH_STAGE1_BINARY_ONLY_DIRECT=1`）を明示して direct route を起動する。

運用メモ（命名）:
- active contract 実行は `phase29y_hako_run_binary_only_ported_vm.sh` を正本とする。
- `phase29y_hako_run_binary_only_block_vm.sh` は RUN-01 履歴互換の legacy alias としてのみ保持する。
- legacy alias の撤去条件: `CURRENT_TASK.md` / `phase-29y` docs / 呼び出しスクリプトから `*_block_vm.sh` 参照が 0 件になった時点。
- backend mismatch（例: `--backend llvm --hako-run`）は non-gating pin で fail-fast 契約を固定する:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
- selfhost readiness proxy（non-gating）:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
  - contract:
    - repo外 `--hako-emit-mir-json` を 2 回連続実行し、canonical MIR が一致すること（N->N+1->N+2 の代理固定）。
    - 同一 workdir で `--backend vm --hako-run` が成功し、stale blocker が再発しないこと。
  - note:
    - これは binary-only 導線の安定性固定であり、`stage1 -> stage2` 実バイナリ生成の G1 identity 完了を置き換えるものではない。

## Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)

目的:
- Rust route 先行で観測された nested ternary parity debt を、fail-fast境界を崩さずに段階的に縮退する。

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `B-TERNARY-01` [done, 2026-02-25]:
   - 対象: probe形（int/int固定）以外の nested ternary（var/int 混在）を最小受理で追加する。
   - 受け入れ:
     - baseline parity lock: `STRICT=1 tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`
     - var-values acceptance lock: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh`
   - note:
     - `phase29y_hako_emit_mir_nested_ternary_var_values_min.hako` は `ternary_no_lower` ではなく MIR 出力まで到達する。
     - canonical signature mismatch は B-TERNARY-03 判定対象として保持する。
2. `B-TERNARY-02` [done, 2026-02-25]:
   - 対象: 未対応形は `unsupported:ternary_no_lower` を維持し、fail-fast境界を fixture+smoke で固定する。
   - 受け入れ:
     - fixture: `apps/tests/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_min.hako`
     - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
3. `B-TERNARY-03` [done, 2026-02-25]:
   - 対象: parity lock を lane-B fast gate へ昇格するか判定し、採否を docs に固定する（昇格/据え置きの二択）。
   - 判定入力:
     - `phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`（strict/non-strict）
     - `phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
   - 判定出力:
     - 昇格する場合: lane-B fast gate への組み込み差分を同コミットで固定。
     - 昇格しない場合: non-gating diagnostic pin 維持を明記して終了。
   - decision (2026-02-25):
     - `phase29y_hako_emit_mir_nested_ternary_var_values_min.hako` で canonical signature mismatch が残存するため、
       lane-B fast gate への昇格は **据え置き**（non-gating diagnostics 維持）とする。

禁止:
- B-TERNARY-01 と B-TERNARY-02/03 を同コミットで混在させない。
- fail-fast marker を silent fallback へ置き換えない。

## Blocker Capture (planner_required)

BoxCount 作業で “落ちた瞬間に” 最短で状況を固定するための採取手順。

- 実行（ログは `/tmp` に固定）:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh apps/tests/<fixture>.hako <label>`
- 生成物:
  - `/tmp/phase29bq_joinir_blocker_<label>_<pid>.log`
  - `/tmp/phase29bq_joinir_blocker_<label>_<pid>.summary`

### Record format

- 記録先:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/phases/<phase>/README.md`
- 記録する2行:
  1. `/tmp/*summary` の先頭1行
  2. `/tmp/*summary` の `first_freeze_or_reject` の1行

## OOM/Canary Log Path

- Selfhost gate:
  - `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Logs:
  - `/tmp/phase29bq_selfhost_*.log`
- Note:
  - canary JSON 実行時は debug-fuel unlimited を固定（必要時）
