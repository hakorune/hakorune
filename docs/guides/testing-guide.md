# テスト実行ガイド

## 📁 **テストファイル配置ルール（超重要！）**

⚠️ **ルートディレクトリの汚染防止ルール** ⚠️
```bash
# ❌ 絶対ダメ：ルートで実行
./target/release/hakorune test.hako        # ログがルートに散乱！
cargo test > test_output.txt             # 出力ファイルがルートに！

# ✅ 正しい方法：必ずディレクトリを使う
cd local_tests && ../target/release/hakorune test.hako
./target/release/hakorune local_tests/test.hako
```

**必須ルール：**
- **テストファイル**: 必ず `local_tests/` に配置
- **ログファイル**: 環境変数で `logs/` に出力するか、実行後即削除
- **デバッグ出力**: `local_tests/` または `logs/` に保存
- **一時ファイル**: `/tmp/` を使用

**なぜ毎回ルートが散らかるのか：**
1. テスト実行時にカレントディレクトリにログ出力
2. エラー時のデバッグファイルが自動削除されない
3. VM統計やMIRダンプがデフォルトでカレントに出力

## 🧪 テスト実行

```bash
# 基本機能テスト
cargo test

# テストファイル作成・実行例
mkdir -p local_tests
echo 'print("Hello Nyash!")' > local_tests/test_hello.hako
./target/debug/nyash local_tests/test_hello.hako

# 演算子統合テスト（local_testsから実行）
./target/debug/nyash local_tests/test_comprehensive_operators.hako

# 実用アプリテスト
./target/debug/nyash app_dice_rpg.hako

# JIT 実行フラグ（CLI, compat/proof keep）
./target/release/hakorune --backend vm \
  --jit-exec --jit-stats --jit-dump --jit-threshold 1 \
  --jit-phi-min --jit-hostcall --jit-handle-debug \
  examples/jit_branch_demo.hako
# 既存の環境変数でも可: 
#   NYASH_JIT_EXEC/NYASH_JIT_STATS(/_JSON)/NYASH_JIT_DUMP/NYASH_JIT_THRESHOLD
#   NYASH_JIT_PHI_MIN/NYASH_JIT_HOSTCALL/NYASH_JIT_HANDLE_DEBUG

# HostCallハンドルPoCの例（compat/proof keep）
./target/release/hakorune --backend vm --jit-exec --jit-hostcall examples/jit_array_param_call.hako
./target/release/hakorune --backend vm --jit-exec --jit-hostcall examples/jit_map_param_call.hako
./target/release/hakorune --backend vm --jit-exec --jit-hostcall examples/jit_map_int_keys_param_call.hako
./target/release/hakorune --backend vm --jit-exec --jit-hostcall examples/jit_string_param_length.hako
./target/release/hakorune --backend vm --jit-exec --jit-hostcall examples/jit_string_is_empty.hako
```

## PHI ポリシー（Phase‑15）と検証トグル

- Phase‑15 では PHI‑on（MIR14）が既定だよ。MIR ビルダーがブロック先頭へ `Phi` を配置し、検証も SSA 前提で実施するよ。
- レガシー検証で edge-copy 互換が必要なら `NYASH_MIR_NO_PHI=1` を明示してね（`NYASH_VERIFY_ALLOW_NO_PHI=1` も忘れずに）。
- 詳細は `docs/reference/mir/phi_policy.md` を参照してね。

テスト時の環境（推奨）
```bash
# 既定: 何も設定しない → PHI-on

# レガシー PHI-off の再現が必要なときだけ明示的に切り替え
export NYASH_MIR_NO_PHI=1
export NYASH_VERIFY_ALLOW_NO_PHI=1

# さらに edge-copy 規約を厳格チェックしたい場合（任意）
export NYASH_VERIFY_EDGE_COPY_STRICT=1
```

PHI-on の補助トレース
- `NYASH_LLVM_TRACE_PHI=1` と `NYASH_LLVM_TRACE_OUT=tmp/phi.jsonl` を組み合わせると、PHI がどの predecessor から値を受け取っているかを確認できるよ。

## PHI 配線トレース（JSONL）

- 目的: LLVM 側の PHI 配線が、PHI-on で生成された SSA と legacy edge-copy (PHI-off) の両方に整合しているかを可視化・検証する。
- 出力: 1 行 JSON（JSONL）。`NYASH_LLVM_TRACE_OUT=<path>` に追記出力。
- イベント: `finalize_begin/finalize_dst/add_incoming/wire_choose/snapshot` など（pred→dst 整合が分かる）

クイック実行（v2）
```bash
# 代表サンプルを LLVM ハーネスで実行し PHI トレースを採取（v2 スクリプト）
bash tools/smokes/phi_trace_local.sh

# 結果の検証（要: python3）
python3 tools/phi_trace_check.py --file tmp/phi_trace.jsonl --summary
```

ショートカット
- `tools/smokes/phi_trace_local.sh`（ビルド→サンプル実行→チェックを一括）
- `tools/smokes/v2/run.sh --profile quick|integration` で代表スモークを実行

## MIR デバッグの入口まとめ

### 1. CLI レベルの MIR ダンプ

- VM 実行経路（SSOT, compat/proof keep）で「実際に走るMIR」を一緒に吐く:
  - `NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm path/to/program.hako`
- 参考: 実行せずにコンパイルだけで MIR を確認（入口確認用 / compile-only）:
  - `./target/release/hakorune --dump-mir path/to/program.hako`
- JSON で詳細解析したい場合:
  - `./target/release/hakorune --emit-mir-json mir.json path/to/program.hako`
  - 例: `jq '.functions[0].blocks' mir.json` でブロック構造を確認。

### 1.5 VM step budget exceeded（無限ループ）を 1 回で切る

VM 実行中に次のようなエラーが出たときの “最短導線”。

```text
vm step budget exceeded ... mir_dump=/tmp/mir_dump_stepbudget_<fn>_<pid>.txt mir_dump_snip=/tmp/mir_dump_stepbudget_snip_<fn>_<pid>.txt
```

- 何が起きているか:
  - VM が同じブロック列を周回し続けており、無限ループ（または極端な長ループ）で止まっている。
- すぐにやること:
  - `mir_dump_snip` を先に見る（周辺ブロックだけの短縮版）。
  - `loop_signature=bbA->bbB->...` が出ていれば、その輪郭が “周回リング”。
  - `trace_tail=[bb:idx:inst|...]` が出ていれば、直近の実行列が “最後尾”。
- dump の探し方:
  - `ls -t /tmp/mir_dump_stepbudget_snip_* | head -n 1`
  - `ls -t /tmp/mir_dump_stepbudget_* | head -n 1`
- 典型的な読み方:
  - `rg -n "phi \\[|Jump \\{|Branch \\{|bb[0-9]+" <snip> | head`
  - `rg -n "const 0|const 1" <snip> | head`（index/carry が初期値に戻ってないか）
- ステップ上限を下げて “速く落とす”:
  - `HAKO_VM_MAX_STEPS=200000`（`0` は無制限。診断専用）

### 2. Scope / Loop ヒント（NYASH_MIR_HINTS）

- 環境変数でヒント出力を制御:
  - `NYASH_MIR_HINTS="<target>|<filters>..."`
- 例:
  - `NYASH_MIR_HINTS="trace|all"`（stderr へ全ヒント）
  - `NYASH_MIR_HINTS="jsonl=tmp/hints.jsonl|loop"`（ループ関連のみ JSONL 出力）
- 詳細: `docs/guides/scopebox.md`, `src/mir/hints.rs`。

### 3. __mir__ ロガー（.hako から仕込む MIR ログ）

- 目的:
  - `.hako` 側のループや SSA まわりを「MIR レベル」で観測するための dev 専用フック。
  - 実行意味論には影響しない（Effect::DebugLog のみ）。
- 構文（.hako 内）:

  ```hako
  __mir__.log("label", v1, v2, ...)
  __mir__.mark("label")
  ```

  - 第1引数は String リテラル必須（それ以外は通常の呼び出し扱い）。
  - `log` は第2引数以降の式を評価し、その ValueId 群を記録。
  - `mark` はラベルだけのマーカー。
  - 戻り値は Void 定数扱いのため、式コンテキストに書いても型崩れしない。

- 実行時の有効化:
  - `NYASH_MIR_DEBUG_LOG=1 ./target/release/hakorune path/to/program.hako`
  - VM の MIR interpreter が次のようなログを stderr に出力:

    ```text
    [MIR-LOG] label: %10=123 %11="foo"
    ```

- 実装位置:
  - lowering: `src/mir/builder/calls/build.rs` の `try_build_mir_debug_call`（receiver が `__mir__` のときに `MirInstruction::DebugLog` を挿入）。
  - 実行: `src/backend/mir_interpreter/handlers/mod.rs`（`NYASH_MIR_DEBUG_LOG=1` のときだけログを出す）。

- 利用例（ループ観測の定番パターン）:

  ```hako
  loop(i < n) {
    __mir__.log("loop/head", i, n)
    ...
  }
  __mir__.mark("loop/exit")
  ```

  - FuncScanner / Stage‑B では、skip_whitespace や scan_all_boxes のループ頭・出口に挿入して、ValueId と値の流れを追跡する用途で使用している。

## 🔌 **プラグインテスター（BID-FFI診断ツール）**
```bash
# プラグインテスターのビルド
cd tools/plugin-tester
cargo build --release

# プラグインの診断実行
./target/release/plugin-tester ../../plugins/nyash-filebox-plugin/target/debug/libnyash_filebox_plugin.so

# 出力例：
# Plugin Information:
#   Box Type: FileBox (ID: 6)  ← プラグインが自己宣言！
#   Methods: 6
#   - birth [ID: 0] (constructor)
#   - open, read, write, close
#   - fini [ID: 4294967295] (destructor)
```

**plugin-testerの特徴**:
- Box名を決め打ちしない汎用設計
- プラグインのFFI関数4つ（abi/init/invoke/shutdown）を検証
- birth/finiライフサイクル確認
- 将来の拡張: TLV検証、メモリリーク検出

## 🐛 デバッグ

### パーサー無限ループ対策（2025-08-09実装）
```bash
# 🔥 デバッグ燃料でパーサー制御
./target/release/hakorune --debug-fuel 1000 program.hako      # 1000回制限
./target/release/hakorune --debug-fuel unlimited program.hako  # 無制限
./target/release/hakorune program.hako                        # デフォルト10万回

# パーサー無限ループが検出されると自動停止＋詳細情報表示
🚨 PARSER INFINITE LOOP DETECTED at method call argument parsing
🔍 Current token: IDENTIFIER("from") at line 17
🔍 Parser position: 45/128
```

**対応状況**: must_advance!マクロでパーサー制御完全実装済み✅  
**効果**: 予約語"from"など問題のあるトークンも安全にエラー検出

### アプリケーション デバッグ
```nyash
// DebugBox活用
DEBUG = new DebugBox()
DEBUG.startTracking()
DEBUG.trackBox(myObject, "説明")
print(DEBUG.memoryReport())
```
## Macro-based Test Runner (MVP)

Nyash provides a macro-powered lightweight test runner in Phase 16 (MVP).

- Enable and run tests in a script file:
  - `nyash --run-tests apps/tests/my_tests.hako`
  - Discovers top-level `test_*` functions and Box `test_*` methods (static/instance).
- Filtering: `--test-filter NAME` (substring match) or env `NYASH_TEST_FILTER`.

---

## Phase 285: Lifecycle / WeakRef / Leak diagnostics（クロスバックエンド）

言語SSOT:
- `docs/reference/language/lifecycle.md`（`fini` / weak / cleanup / GC方針）
- `docs/reference/language/types.md`（truthiness と `null`/`void`）

目的:
- Rust VM と LLVM（harness）で、weak と lifecycle の挙動が一致しているかを短い fixture で固定する。
- 強参照サイクル等で「終了時に強参照rootが残っている」状況を、default-off の診断で観測できるようにする（実装後）。

推奨の運用（ルート汚染防止）:
- fixture は `local_tests/` に置く。

例（weakが効いていること）:
```bash
mkdir -p local_tests
$EDITOR local_tests/phase285_weak_basic.hako

./target/release/hakorune --backend vm local_tests/phase285_weak_basic.hako
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm local_tests/phase285_weak_basic.hako
```

例（強参照サイクル + 終了時診断）:
```bash
$EDITOR local_tests/phase285_cycle.hako

# 実装後: leak report をONにして終了時に root を表示
NYASH_LEAK_LOG=1 ./target/release/hakorune --backend vm local_tests/phase285_cycle.hako
NYASH_LLVM_USE_HARNESS=1 NYASH_LEAK_LOG=1 ./target/release/hakorune --backend llvm local_tests/phase285_cycle.hako
```

実装者向けの詳しい手順:
- `docs/development/current/main/phases/phase-285/CLAUDE_CODE_RUNBOOK.md`
- Entry policy when a main exists:
  - `--test-entry wrap` → run tests then call original main
  - `--test-entry override` → replace entry with test harness only
  - Force apply: `NYASH_TEST_FORCE=1`
- Parameterized tests (MVP): `NYASH_TEST_ARGS_DEFAULTS=1` injects integer `0` for each parameter (static/instance tests).
- Exit code = number of failed tests (0 on success).

Notes
- The feature is behind the macro gate; CLI `--run-tests` enables it automatically.
- Future versions will add JSON-based per-test arguments and richer reporting.
