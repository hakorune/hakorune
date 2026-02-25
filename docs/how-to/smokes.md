# Smokes — How‑To（前提→手順→検証）

目的
- 代表スモークを素早く回して、回帰を検知する。

前提
- リリースビルド済み: `cargo build --release`
- LLVM を用いた AOT/ハーネス系は integration プロファイルで必要に応じて利用

手順（v2 ランナー推奨）
1) クイック確認（VM/動的プラグイン）
   - 実行: `tools/smokes/v2/run.sh --profile quick`
   - 代表的な言語機能・using の確認。冗長ログはフィルタ済み
2) プラグイン検証（VM/動的）
   - 実行: `tools/smokes/v2/run.sh --profile plugins`
   - フィクスチャ .so は自動ビルド・配置を試行（無ければ SKIP）
3) 統合確認（LLVM/LlvmLite ハーネス含む）
   - 実行: `tools/smokes/v2/run.sh --profile integration`
   - 必要に応じて PHI-on/off の比較や AOT 代表ケースを実行

手動スモーク（例）
- Core (LLVM): `examples/llvm11_core_smoke.hako`
- Async (LLVM only):
  - `apps/tests/async-await-min/main.hako`
  - `apps/tests/async-spawn-instance/main.hako`
  - `apps/tests/async-await-timeout-fixed/main.hako`（`NYASH_AWAIT_MAX_MS=100`）
- Selfhost Stage‑B canaries（opt-in）:
  - `SMOKES_ENABLE_STAGEB=1 tools/smokes/v2/profiles/integration/selfhost/selfhost_stageb_if_vm.sh`
  - `SMOKES_ENABLE_STAGEB=1 tools/smokes/v2/profiles/integration/selfhost/selfhost_stageb_index_vm.sh`
  - これらは `target/release/hakorune` を使用（`nyash` は deprecated で stdout が汚れるため使用しない）

アーカイブ（非推奨）
- 旧ランナー（JIT/Cranelift 時代）は削除または archive に移動済み。v2 ランナーのみを使用

便利フラグ
- `NYASH_LLVM_USE_HARNESS=1`: integration プロファイルで llvmlite ハーネスを使う
- `NYASH_MIR_NO_PHI=1`, `NYASH_VERIFY_ALLOW_NO_PHI=1`: レガシー PHI-off（edge-copy）モード
- `NYASH_USING_DYLIB_AUTOLOAD=1`: using kind="dylib" の自動ロードを有効化（dev 向け・既定OFF）
- `--skip-preflight`: preflight を省略（gate の反復実行向け、例: `tools/smokes/v2/run.sh --profile integration --filter phase29aq_* --skip-preflight`）

フレーク切り分け（SMOKES_REPRO）
- `SMOKES_REPRO=N` を付けて `tools/smokes/v2/run.sh` を実行すると、失敗したテストを最大 N 回だけ同条件で再実行する（フレーク検知用）。
- 例: `SMOKES_REPRO=20 tools/smokes/v2/run.sh --profile integration --filter phase29aq_string_parse_integer_sign_min_vm.sh`
- 失敗ログは `/tmp/hakorune_smoke_*.log` と `/tmp/hakorune_smoke_retry_*.log` に残る。
- JoinIR 回帰 gate（`tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`）は、失敗時に adopt 系 env を 1 行でダンプする（仮説ズレ防止）。
- run.sh 本体も FAIL 時に重要 env を 1 行ダンプ（profile/config/plugin_mode/adopt/tolerate/trace）する。

VM トレース（--trace-vm）
- `tools/smokes/v2/run.sh --trace-vm` は `NYASH_VM_TRACE=1` と `HAKO_SILENT_TAGS=0` をまとめて有効化する。
- 例: `tools/smokes/v2/run.sh --profile integration --filter phase29aq_string_parse_integer_sign_min_vm.sh --trace-vm`

検証
- 0 で成功、非 0 で失敗（CI 連携可）
