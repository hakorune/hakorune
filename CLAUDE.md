# Claude Quick Start (Minimal Entry)

このファイルは最小限の入口だよ。詳細はREADMEから辿ってねにゃ😺

---

## 🔄 **現在の開発状況** (2025-11-20)

### 🎊 **PHI Bug Option C実装ほぼ完了！** (2025-11-20 commit `461bdec4`)
- **267/268テスト PASS達成！** Option C実装大成功 🎉
- **Step 5-5-H完了**: Phantom block検証実装（exit_preds検証）
- **PHI決定性向上**: BTreeSet/BTreeMap化（4ファイル修正）
  - `if_phi.rs`, `loop_phi.rs`, `loop_snapshot_merge.rs`, `loopform_builder.rs`
- **根本原因解明**: HashMap非決定的イテレーションによるValueId変動
  - 詳細: `docs/development/current/main/valueid-*.md`
- **退行なし**: 267テスト全てPASS（1テストのみ非決定性残存）
- **次タスク**: Stage-B型エラー修正（String > Integer(13)）
- **後回し**: variable_map決定性化（builder.rs等のHashMap→BTreeMap）

#### 🔍 **MIRデバッグ完全ガイド**（超重要！）
```bash
# 基本MIR確認（最優先！ / 実行経路SSOT）
# - VM実行経路で「実際に走るMIR」を見る（推奨）
NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm program.hako

# 詳細MIR + エフェクト情報（実行経路SSOT）
NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm --mir-verbose --mir-verbose-effects program.hako

# 参考: コンパイルだけでMIRを見る（実行しない / 入口確認用）
# - 実行経路SSOT（最適化/検証/バックエンド差分）を追う目的では、上の NYASH_VM_DUMP_MIR を優先
# - `--dump-mir` は “compile-only” のため、実行時の挙動や backend 差分確認の主導線にはしない
./target/release/hakorune --dump-mir program.hako
./target/release/hakorune --dump-mir --mir-verbose --mir-verbose-effects program.hako

# JSON形式で詳細解析
./target/release/hakorune --emit-mir-json mir.json program.hako
jq '.functions[0].blocks' mir.json  # ブロック構造確認

#### 🧭 **LLVM harness 実行導線（SSOT）**
LLVM harness は **3つのビルドが必須**。これを満たさないと失敗する。

```bash
# 入口SSOT（ビルド+実行を一気通貫）
tools/run_llvm_harness.sh <program.hako>

# 失敗時の復旧（不足物のビルド）
cargo build --release -p nyash-rust --features llvm --bin hakorune
cargo build --release -p nyash-llvm-compiler
cargo build --release -p nyash_kernel

# IR を見る場合（harness）
NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_DUMP_IR=/tmp/phase285.ll \
  ./target/release/hakorune --backend llvm program.hako
```

**MIR確認の優先順位**:
- 実行経路のSSOT: `NYASH_VM_DUMP_MIR=1 ... --backend vm`
- JSON確認（LLVM経路の入力確認）: `--emit-mir-json`
- `--dump-mir` は compile-only（実行時差分の主導線にはしない）

# Option C デバッグ（PHI関連）
NYASH_OPTION_C_DEBUG=1 cargo test --release TEST_NAME 2>&1 | grep "Option C"

# LoopForm デバッグ
NYASH_LOOPFORM_DEBUG=1 cargo test --release TEST_NAME 2>&1 | grep "loopform"

# DCE（Dead Code Elimination）トレース ⭐NEW - 命令が消える問題のデバッグ
NYASH_DCE_TRACE=1 ./target/release/hakorune program.hako 2>&1 | grep "\[dce\]"
# 出力例:
#   [dce] Eliminating unused pure instruction in bb12: %29 = Const { dst: ValueId(29), value: Void }
#   [dce] Eliminating unused pure instruction in bb5: %38 = Copy { dst: ValueId(38), src: ValueId(36) }
# → 「命令は emit されてるのに実行時に undefined」問題の原因特定に有効！

# variable_map トレース (JoinIR PHI接続デバッグ) ⭐超重要
NYASH_TRACE_VARMAP=1 cargo test --release TEST_NAME 2>&1 | grep "\[trace:"
# 出力例:
#   [trace:varmap] pattern3_before_merge: sum→r123, count→r124
#   [trace:varmap] pattern3_after_merge: (merge完了後)
#   [trace:varmap] pattern3_exit_phi_connected: sum→r456(final)

# JoinIR 詳細デバッグ（ルーティング・ブロック割り当て）⭐Phase 195
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako 2>&1 | grep "\[trace:"
# 出力例:
#   [trace:pattern] route: Pattern1_Minimal MATCHED
#   [trace:joinir] pattern1: 3 functions, 13 blocks
#   [trace:blocks] allocator: Block remap: join_func_0:BasicBlockId(0) → BasicBlockId(4)
#   [trace:routing] router: function 'main' - try_cf_loop_joinir called
# Legacy: NYASH_JOINIR_DEBUG=1 also works (deprecated, Phase 82)

# 完全MIRダンプ（テスト時）
NYASH_MIR_TEST_DUMP=1 cargo test --release TEST_NAME 2>&1 > /tmp/mir_dump.log

# VM実行トレース
NYASH_CLI_VERBOSE=1 ./target/release/hakorune program.hako

# 決定性テスト（3回実行して一貫性確認）
for i in 1 2 3; do
  echo "=== Run $i ==="
  cargo test --release TEST_NAME 2>&1 | grep -E "(ValueId|test result)"
done
```

##### 🎯 **NYASH_TRACE_VARMAP の威力**

**何ができる？**
- JoinIR → MIR merge 時の variable_map 変化を可視化
- Pattern 3 のような複雑な PHI 接続を段階的に追跡
- Exit PHI が正しく variable_map に接続されたか確認
- バグ発見時間: **1.5時間 → 30秒** に短縮！

**戦略的なトレース点**:
- `pattern3_before_merge`: Merge 実行前の ValueId マッピング
- `pattern3_after_merge`: Merge 直後の状態
- `pattern3_exit_phi_connected`: Exit PHI が variable に接続された最終状態

**使用例**:
```bash
# Pattern 3 バグの高速診断
NYASH_TRACE_VARMAP=1 cargo test --release test_loop_with_if_phi_sum -- --nocapture 2>&1 | tail -20
# → sum の ValueId 変化が一目瞭然！
```

#### 📊 **重要な発見：HashMap非決定性**
- Rustの`HashMap`/`HashSet`はHashDoS対策でランダムseed使用
- PHI生成順序が毎回変わる → ValueId割り当てが変動
- **解決**: `BTreeSet`/`BTreeMap`で決定的イテレーション保証
- **残課題**: `variable_map: HashMap<String, ValueId>` (builder.rs等)

#### 🔍 **hako_check - セルフホスティング.hako品質チェック** (Phase 153復活！)

**hako_check ツール**はセルフホスティングコンパイラの .hako ファイルをコード品質チェック！

```bash
# 基本的な使い方
./tools/hako_check.sh file.hako
./tools/hako_check.sh directory/

# 実用的な使い方
./tools/hako_check.sh apps/selfhost-runtime/boxes_std.hako    # 単発チェック
./tools/hako_check.sh apps/selfhost-runtime/ --dead-code      # デッドコード検出
./tools/hako_check.sh apps/selfhost-runtime/ --format json-lsp # エディタ統合用

# デバッグ情報が必要な時
HAKO_CHECK_DEBUG=1 ./tools/hako_check.sh file.hako    # [DEBUG] 出力を含める
```

**環境変数制御**:
- `HAKO_CHECK_DEBUG=0` (デフォルト): デバッグ出力フィルタリング（クリーンな出力）
- `HAKO_CHECK_DEBUG=1`: 詳細デバッグ出力（[DEBUG/...], [ControlForm::...] 等を表示）
- `HAKO_CHECK_VERBOSE=1`: 詳細モード（将来実装予定）

**検出ルール** (Phase 153):
- **HC011**: 呼ばれないメソッド（unreachable method）
- **HC012**: 参照されないスタティックボックス（dead static box）
- **HC019**: 到達不可能なコード（unreachable code / dead code）
- その他: arity mismatch など 15+ ルール

**出力例**:
```
[ERROR] ❌ MIR compilation error: Undefined variable: void
[lint/summary] failures: 1
```

**次のステップ (Phase 2-3)**:
- Rust側のデバッグ出力環境変数制御化
- エラーメッセージの構造化（ファイル:行番号表示、ヒント追加）

### 🎉 **Phase 33: Box Theory Modularization Complete!** (2025-12-07)
- **Phases 33-10 to 33-12 completed** (5 commits, +421 net lines, 10 new files)
- **Exit line architecture** (ExitLineReconnector, ExitMetaCollector Boxes)
- **Large file restructuring** (mod.rs 511→221 lines, loop_patterns 735→7 files)
- **Code quality** (single responsibility, testability, maintainability dramatically improved)
- **Documentation** (comprehensive comments + architecture guide)
- **Build status**: ✅ All tests passing
- **Architecture doc**: [phase-33-modularization.md](docs/development/architecture/phase-33-modularization.md)

#### Phase 33-10: Exit Line Modularization
- **✅ 箱化モジュール化完了**: ExitLineReconnector, ExitMetaCollector, ExitLineOrchestrator
- **✅ 単一責任の原則**: 各Boxが1つの関心事のみ処理
- **✅ 再利用性**: Pattern 3, 4等で共通利用可能
- **✅ テスト容易性**: 独立したBoxで単体テスト可能

#### Phase 33-11: Quick Wins
- **✅ 未使用import削除**: cargo fix による自動クリーンアップ（11ファイル）
- **✅ Pattern 4明確化**: Stub→明示的エラー、マイグレーションガイド追加
- **✅ Dispatcher統一確認**: common.rsで既に統一済み

#### Phase 33-12: Structural Improvements
- **✅ If lowering router**: 専用ファイル分離（172行）
- **✅ Loop pattern router**: 専用ファイル分離（149行）
- **✅ Pattern別ファイル**: 735行→7ファイル（Pattern 1-4各独立）
- **✅ mod.rs削減**: 511→221行（57%削減）

### 🎉 **Phase 25 MVP 完全成功！** (2025-11-15)
- **numeric_core BoxCall→Call変換** 完全動作！
- **2つの重大バグ修正**:
  1. nyash.toml モジュールマッピング欠落（224行目追加）
  2. numeric_core.hako JSONパース処理のバグ（全JSON→個別命令処理に修正）
- **型検出システム正常動作**: 型テーブルサイズ 1→3、MatI64インスタンス完全検出
- **変換例**: `BoxCall(MatI64, "mul_naive")` → `Call("NyNumericMatI64.mul_naive")`
- **検証**: 全テストパス（単体・E2E・変換確認・残骸確認）✅
- **🔧 追加修正（継続セッション）**:
  - **PHI型伝播修正**: 4回反復型伝播で copy → phi → copy チェーン完全対応（8d9bbc40）
  - **環境変数伝播**: microbench.sh に NYASH_AOT_NUMERIC_CORE 伝播追加（3d082ca1）
  - **両SSAパターン検証**: 冗長版（13 PHI）& 最適化版（1 PHI）両方で変換成功確認 ✅
  - **ログ転送問題根治**: hakorune_emit_mir.sh の provider 経路にログ転送追加（ユーザー実装）
  - **STRICT mode 調査**: check_numeric_core_invariants() 実装済みだが未使用（タイミング問題で無効化）
- **🛠️ 推奨ワークフロー確立**: `tools/dev_numeric_core_prep.sh` で環境変数自動設定 ✅

### 🎯 **Phase 15: セルフホスティング実行器統一化**
- **Rust VM + LLVM 2本柱体制**で開発中
- **Core Box統一化**: 3-tier → 2-tier 統一完了
- **MIR Callee型革新**: 型安全な関数解決システム実装済み

### 🤝 **AI協働開発体制**
```
Claude（私）: 戦略・分析・レビュー
ChatGPT: 実装・検証

現在の合意:
✅ Phase 15集中（セルフホスト優先）
✅ Builder根治は段階的（3 Phase戦略）
✅ 息が合っている状態: 良好
```

### 📚 **重要リソース**
- **開発マスタープラン**: [00_MASTER_ROADMAP.md](docs/private/roadmap2/phases/00_MASTER_ROADMAP.md)
- **現在のタスク**: [CURRENT_TASK.md](CURRENT_TASK.md)
- **Phase 15詳細**: [docs/private/roadmap2/phases/phase-15/](docs/private/roadmap2/phases/phase-15/)

---

## 🚨 重要：スモークテストはv2構造を使う！
- 📖 **スモークテスト完全ガイド**: [tools/smokes/README.md](tools/smokes/README.md)
- 📁 **v2詳細ドキュメント**: [tools/smokes/v2/README.md](tools/smokes/v2/README.md)

### 🎯 2つのベースライン（Two Baselines）

#### 📦 VM ライン（Rust VM - 既定）
```bash
# ビルド
cargo build --release

# 一括スモークテスト
tools/smokes/v2/run.sh --profile quick

# 個別スモークテスト（フィルタ指定）
tools/smokes/v2/run.sh --profile quick --filter "<glob>"
# 例: --filter "userbox_*"  # User Box関連のみ
# 例: --filter "json_*"     # JSON関連のみ

# 単発スクリプト実行
bash tools/smokes/v2/profiles/quick/core/selfhost_mir_m3_jump_vm.sh

# 単発実行（参考）
./target/release/hakorune --backend vm apps/APP/main.hako
```

#### ⚡ llvmlite ライン（LLVMハーネス）
```bash
# 前提: Python3 + llvmlite
# 未導入なら: pip install llvmlite

# 一括スモークテスト（そのまま実行）
tools/smokes/v2/run.sh --profile integration

# 警告低減版（ビルド後に実行・推奨）
cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm
tools/smokes/v2/run.sh --profile integration

# 個別スモークテスト（フィルタ指定）
tools/smokes/v2/run.sh --profile integration --filter "<glob>"
# 例: --filter "json_*"     # JSON関連のみ
# 例: --filter "vm_llvm_*"  # VM/LLVM比較系のみ

# 単発実行
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/peek_expr_block.hako

# 有効化確認
./target/release/hakorune --version | rg -i 'features.*llvm'
```

**💡 ポイント**:
- **VM ライン**: 開発・デバッグ・検証用（高速・型安全）
- **llvmlite ライン**: 本番・最適化・配布用（実証済み安定性）
- 両方のテストが通ることで品質保証！

#### 🔥 セルフホストライン（.hako コンパイラ - 爆速開発！）
```bash
# 🚀 ビルド不要！.hako 編集 → 即実行で爆速イテレーション
NYASH_USE_NY_COMPILER=1 NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
  ./target/release/hakorune program.hako

# タイムアウト延長（大きいファイル用）
NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_TIMEOUT_MS=60000 \
  NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
  ./target/release/hakorune program.hako

# デバッグ出力付き
NYASH_USE_NY_COMPILER=1 NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
  NYASH_CLI_VERBOSE=1 \
  ./target/release/hakorune program.hako
```

**💡 セルフホストの価値**:
- **cargo build 不要！** .hako 変更 → 即テスト（1-2分の待ち時間ゼロ）
- **開発加速**: Rust パーサー強化のドライバーにもなる
- **復活**: 2025-11-25 に StringBox.get() バグ修正で復活 (`4120ab65`)

#### 🚫 Selfhost / JoinIR strict の「.hako workaround」禁止（重要）

セルフホストは爆速だけど、**JoinIR strict / planner_required のブロッカー回避を .hako 側でやるのは原則NG**。
（スモークを通すためだけの while→loop 変換などで、Rust側の表現力が育たず負債化しやすい）

- **SSOT（方針）**: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- **SSOT（Gate/運用）**: `docs/development/current/main/10-Now.md`, `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- **原則**:
  - `.hako` 側での回避（構文の置換や by-name な特例）で通さない
  - Rust 側で **最小の箱（Facts→Recipe→Lower）** または **capability受理**を追加し、fixture+fast gate で契約固定する
  - AST rewrite（見かけ等価の式変形）禁止。観測は `CondBlockView` 等の analysis-only view で保守的に行う
- **例**: canary が `[joinir/control_tree/cap_missing/While]` で落ちたら
  - `.hako` の `while` を `loop(cond)` に書き換えて回避しない
  - Rust 側で While capability を受理し、最小 fixture を fast gate に pin して前進する

#### 🔎 JoinIR/plan デバッグの環境変数（統一）
- JoinIR/plan 周りの詳細ログは **`HAKO_JOINIR_DEBUG=1`** を使う（`NYASH_JOINIR_DEBUG` は legacy）
- タグが見えない時は `HAKO_SILENT_TAGS=0` を併用

#### 🔍 コード品質チェック - hako_check（Phase 153 復活！）

**hako_check を使いながら .hako 開発するワークフロー**:

```bash
# Step 1: .hako を実行テスト
NYASH_USE_NY_COMPILER=1 NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
  ./target/release/hakorune program.hako

# Step 2: コード品質チェック（基本チェック）
./tools/hako_check.sh program.hako

# Step 3: デッドコード検出（Phase 153 新機能！）
./tools/hako_check.sh --dead-code program.hako
#  → [HC019] unreachable method (dead code)
#  → [HC019] dead static box (never referenced)

# Step 4: 特定ルールのみチェック
./tools/hako_check.sh --rules dead_code program.hako

# Step 5: JSON-LSP フォーマット出力（エディタ統合用）
./tools/hako_check.sh --format json-lsp --dead-code program.hako
```

**開発フロー例**:
```bash
# .hako ファイル編集
vim src/compiler/my_feature.hako

# 即座に実行テスト（cargo build 不要！）
NYASH_USE_NY_COMPILER=1 NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
  ./target/release/hakorune src/compiler/my_feature.hako

# コード品質確認（未使用関数・デッドコード検出）
./tools/hako_check.sh --dead-code src/compiler/my_feature.hako

# 問題があれば修正 → 再実行
# （ループ高速！セルフホスト + hako_check で完全な検証ワークフロー）
```

**チェック内容** (Phase 153):
- **HC011**: 呼ばれないメソッド（デッドメソッド）
- **HC012**: 参照されないスタティックボックス
- **HC019**: 到達不可能な関数・ブロック（新！デッドコード検出）
- その他: arity mismatch など 15+ ルール

**💡 価値**:
- **JoinIR ベース**: コンパイラ側の制御フロー情報を活用（正確！）
- **.hako 移植時の安全ネット**: Phase 160+ で .hako JoinIR/MIR 移植するときの検証に使える
- **即座にフィードバック**: セルフホストの爆速開発と組み合わせ最強！

## Start Here (必ずここから)
- 現在のタスク: [CURRENT_TASK.md](CURRENT_TASK.md)
  - 📁 **Main**: [docs/development/current/main/](docs/development/current/main/)
  - 📁 **LLVM**: [docs/development/current/llvm/](docs/development/current/llvm/)
  - 📁 **Self**: [docs/development/current/self_current_task/](docs/development/current/self_current_task/)
- ドキュメントハブ: [README.md](README.md)
- 🚀 **開発マスタープラン**: [00_MASTER_ROADMAP.md](docs/private/roadmap2/phases/00_MASTER_ROADMAP.md)
 - 📊 **JIT統計JSONスキーマ(v1)**: [jit_stats_json_v1.md](docs/reference/jit/jit_stats_json_v1.md)

## 🧱 先頭原則: 「箱理論（Box-First）」で足場を積む
Nyashは「Everything is Box」。実装・最適化・検証のすべてを「箱」で分離・固定し、いつでも戻せる足場を積み木のように重ねる。

- 基本姿勢: 「まず箱に切り出す」→「境界をはっきりさせる」→「差し替え可能にする」
  - 環境依存や一時的なフラグは、可能な限り「箱経由」に集約（例: JitConfigBox）
  - VM/JIT/GC/スケジューラは箱化されたAPI越しに連携（直参照・直結合を避ける）
- いつでも戻せる: 機能フラグ・スコープ限定・デフォルトオフを活用し、破壊的変更を避ける
  - 「限定スコープの足場」を先に立ててから最適化（戻りやすい積み木）
- AI補助時の注意: 「力づく最適化」を抑え、まず箱で境界を確立→小さく通す→可視化→次の一手
- **Fail-Fast原則**: フォールバック処理は原則禁止。エラーは早期に明示的に失敗させる。過去に何度も分岐ミスでエラーの発見が遅れたため、特にChatGPTが入れがちなフォールバック処理には要注意

実践テンプレート（開発時の合言葉）
- 「箱にする」: 設定・状態・橋渡しはBox化（例: JitConfigBox, HandleRegistry）
- 「境界を作る」: 変換は境界1箇所で（VMValue↔JitValue, Handle↔Arc）
- 「戻せる」: フラグ・feature・env/Boxで切替。panic→フォールバック経路を常設
- 「見える化」: ダンプ/JSON/DOTで可視化、回帰テストを最小構成で先に入れる
- 「Fail-Fast」: エラーは隠さず即座に失敗。フォールバックより明示的エラー

## 🤖 **Claude×Copilot×ChatGPT協調開発**
### 📋 **開発マスタープラン - 全フェーズの統合ロードマップ**
**すべてはここに書いてある！** → [00_MASTER_ROADMAP.md](docs/private/roadmap2/phases/00_MASTER_ROADMAP.md)

**現在のフェーズ：Phase 15 (Nyashセルフホスティング実行器統一化 - Rust VM + LLVM 2本柱体制)**

### 🏆 **Phase 15.5完了！アーキテクチャ革命達成**
- ✅ **Core Box Unification**: 3-tier → 2-tier 統一化完了
- ✅ **MIRビルダー統一化**: 約40行の特別処理削除
- ✅ **プラグインチェッカー**: ChatGPT5 Pro設計の安全性機能実装
- ✅ **StringBox問題根本解決**: slot_registry統一による完全修正

### 🎉 **Phase 2.4完了！NyRT→NyKernelアーキテクチャ革命**
- ✅ **NyKernel化成功**: `crates/nyrt` → `crates/nyash_kernel` 完全移行
- ✅ **42%削減達成**: `with_legacy_vm_args` 11箇所系統的削除完了
- ✅ **Plugin-First統一**: 旧VM依存システム完全根絶
- ✅ **ビルド成功**: libnyash_kernel.a完全生成（0エラー・0警告）
- ✅ **ChatGPT5×Claude協働**: 歴史的画期的成果達成！

### 🚀 **Phase 15戦略確定: Rust VM + LLVM 2本柱**
```
【Rust VM】  開発・デバッグ・検証用（712行、高品質・型安全）
【LLVM】     本番・最適化・配布用（Python/llvmlite、実証済み）
【PyVM】     JSON v0ブリッジ専用（セルフホスティング・using処理のみ）
【削除完了】 レガシーインタープリター（~350行削除済み）
```

📋 **詳細計画**: [Phase 15.5 README](docs/private/roadmap2/phases/phase-15.5/README.md) | [CURRENT_TASK.md](CURRENT_TASK.md)

## 🏃 開発の基本方針: 80/20ルール - 完璧より進捗

### なぜこのルールか？
**実装後、必ず新しい問題や転回点が生まれるから。**
- 100%完璧を目指すと、要件が変わったときの手戻りが大きい
- 80%で動くものを作れば、実際の使用からフィードバックが得られる
- 残り20%は、本当に必要かどうか実装後に判断できる

### 実践方法
1. **まず動くものを作る**（80%）
2. **改善アイデアは `docs/development/proposals/ideas/` フォルダに記録**（20%）
3. **優先度に応じて後から改善**

## 🚀 クイックスタート

### 🎯 **2本柱実行方式** (推奨!)
```bash
# 🔧 開発・デバッグ・検証用 (Rust VM)
./target/release/hakorune program.hako
./target/release/hakorune --backend vm program.hako

# ⚡ 本番・最適化・配布用 (LLVM)
./target/release/hakorune --backend llvm program.hako

# 🛡️ プラグインエラー対策
NYASH_DISABLE_PLUGINS=1 ./target/release/hakorune program.hako

# 🔍 詳細診断
NYASH_CLI_VERBOSE=1 ./target/release/hakorune program.hako
```

### 🚀 **Phase 15 セルフホスティング専用**
```bash
# JSON v0ブリッジ（PyVM特殊用途）
NYASH_SELFHOST_EXEC=1 ./target/release/hakorune program.hako

# using処理確認
./target/release/hakorune --enable-using program_with_using.hako

# ラウンドトリップテスト
./tools/ny_roundtrip_smoke.sh
```

### 🐧 Linux/WSL版
```bash
# 標準ビルド（2本柱対応）
cargo build --release

# 開発・デバッグ実行（Rust VM）
./target/release/hakorune program.hako

# 本番・最適化実行（LLVM）
./target/release/hakorune --backend llvm program.hako
```

### 🪟 Windows版
```bash
# Windows実行ファイル生成
cargo build --release --target x86_64-pc-windows-msvc

# 生成された実行ファイル
target/x86_64-pc-windows-msvc/release/nyash.exe
```

### 🌐 **WASM/AOT版**（開発中）
```bash
# ⚠️ WASM機能: レガシーインタープリター削除により一時無効
# TODO: VM/LLVMベースのWASM実装に移行予定

# LLVM AOTコンパイル（実験的）
./target/release/hakorune --backend llvm program.hako  # 実行時最適化
```

### 🎯 **2本柱ビルド方法** (2025-09-28更新)

#### 🔨 **標準ビルド**（推奨）
```bash
# 標準ビルド（2本柱対応）
cargo build --release

# LLVM（llvmliteハーネス）付きビルド（本番用）
cargo build --release --features llvm
```

#### 📝 **2本柱テスト実行**
```bash
# 1. Rust VM実行 ✅（開発・デバッグ用）
cargo build --release
./target/release/hakorune program.hako

# 2. LLVM実行 ✅（本番・最適化用, llvmliteハーネス）
cargo build --release --features llvm
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm program.hako

# 3. プラグインテスト実証済み ✅
# CounterBox
echo 'local c = new CounterBox(); c.inc(); c.inc(); print(c.get())' > test.hako
./target/release/hakorune --backend llvm test.hako

# StringBox
echo 'local s = new StringBox(); print(s.concat("Hello"))' > test.hako
./target/release/hakorune test.hako

```

⚠️ **ビルド時間の注意**:
- 標準ビルド: 1-2分（高速）
- LLVMビルド: 3-5分（時間がかかる）
- 必ず十分な時間設定で実行してください

## 🚨 **Claude迷子防止ガイド** - 基本的な使い方で悩む君へ！

### 😵 **迷ったらこれ！**（Claude Code専用）

```bash
# 🎯 基本実行（まずこれ）- Rust VM
./target/release/hakorune program.hako

# ⚡ 本番・最適化実行 - LLVM
./target/release/hakorune --backend llvm program.hako

# 🛡️ プラグインエラー対策（緊急時のみ）
NYASH_DISABLE_PLUGINS=1 ./target/release/hakorune program.hako

# 🔍 詳細診断情報
NYASH_CLI_VERBOSE=1 ./target/release/hakorune program.hako

# ⚠️ PyVM特殊用途（JSON v0ブリッジ・セルフホスト専用）
NYASH_SELFHOST_EXEC=1 ./target/release/hakorune program.hako
```

### 🚨 **Phase 15戦略確定**
- ✅ **Rust VM + LLVM 2本柱体制**（開発集中）
- ✅ **PyVM特化保持**（JSON v0ブリッジ・using処理のみ）
- ✅ **レガシーインタープリター削除完了**（~350行削除済み）
- 🎯 **基本はRust VM、本番はLLVM、特殊用途のみPyVM**

### 📊 **環境変数優先度マトリックス**（Phase 15戦略版）

| 環境変数 | 必須度 | 用途 | 使用タイミング |
|---------|-------|-----|-------------|
| `NYASH_CLI_VERBOSE=1` | ⭐⭐⭐ | 詳細診断 | デバッグ時 |
| `NYASH_DISABLE_PLUGINS=1` | ⭐⭐ | エラー対策 | プラグインエラー時 |
| `NYASH_SELFHOST_EXEC=1` | ⭐ | セルフホスト | JSON v0ブリッジ専用 |
| ~~`NYASH_VM_USE_PY=1`~~ | ⚠️ | PyVM特殊用途 | ~~開発者明示のみ~~ |
| ~~`NYASH_ENABLE_USING=1`~~ | ✅ | using処理 | ~~デフォルト化済み~~ |

**💡 2本柱戦略**：基本は`./target/release/hakorune`（Rust VM）、本番は`--backend llvm`！

**⚠️ PyVM使用制限**: [PyVM使用ガイドライン](docs/reference/pyvm-usage-guidelines.md)で適切な用途を確認

### ✅ **using system完全実装完了！** (2025-09-24 ChatGPT実装完了確認済み)

**🎉 歴史的快挙**: `using nyashstd`が完璧動作！環境変数なしでデフォルト有効！

**✅ 実装完了内容**：
- **ビルトイン名前空間解決**: `nyashstd` → `builtin:nyashstd`の自動解決
- **自動コード生成**: nyashstdのstatic box群（string, integer, bool, array, console）を動的生成
- **環境変数不要**: デフォルトで有効（--enable-using不要）

**✅ 動作確認済み**：
```bash
# 基本using動作（環境変数・フラグ不要！）
echo 'using nyashstd' > test.hako
echo 'console.log("Hello!")' >> test.hako
./target/release/hakorune test.hako
# 出力: Hello!

# 実装箇所
src/runner/pipeline.rs       # builtin:nyashstd解決
src/runner/modes/common_util/resolve/strip.rs  # コード生成
```

**📦 含まれるnyashstd機能**：
- `string.create(text)`, `string.upper(str)`
- `integer.create(value)`, `bool.create(value)`, `array.create()`
- `console.log(message)`

**🎯 完成状態**: ChatGPT実装で`using nyashstd`完全動作中！

## 🧪 テストスクリプト参考集（既存のを活用しよう！）
```bash
# 基本的なテスト
./target/release/hakorune local_tests/hello.hako              # Hello World
./target/release/hakorune local_tests/test_array_simple.hako  # ArrayBox
./target/release/hakorune apps/tests/string_ops_basic.hako    # StringBox

# MIR確認用テスト
NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm apps/tests/loop_min_while.hako
NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm apps/tests/esc_dirname_smoke.hako

# 統一Call テスト（Phase A完成！）
NYASH_MIR_UNIFIED_CALL=1 NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm test_simple_call.hako
NYASH_MIR_UNIFIED_CALL=1 ./target/release/hakorune --emit-mir-json test.json test.hako
```

## ⚡ 重要な設計原則

### 🏗️ Everything is Box
- すべての値がBox（StringBox, IntegerBox, BoolBox等）
- ユーザー定義Box: `box ClassName { field1: TypeBox field2: TypeBox }`
- **MIR14命令**: たった14個の命令で全機能実現！
  - 基本演算(5): Const, UnaryOp, BinOp, Compare, TypeOp
  - メモリ(2): Load, Store  
  - 制御(4): Branch, Jump, Return, Phi
  - Box(2): NewBox, BoxCall
  - 外部(1): ExternCall

### 🌟 完全明示デリゲーション
```nyash
// デリゲーション構文（すべてのBoxで統一的に使える！）
box Child from Parent {  // from構文でデリゲーション
    birth(args) {  // コンストラクタは「birth」に統一
        from Parent.birth(args)  // 親の初期化
    }
    
    override method() {  // 明示的オーバーライド必須
        from Parent.method()  // 親メソッド呼び出し
    }
}

// ✅ ビルトインBox、プラグインBox、ユーザー定義Boxすべてで可能！
box MyString from StringBox { }          // ビルトインBoxから
box MyFile from FileBox { }             // プラグインBoxから
box Employee from Person { }            // ユーザー定義Boxから
box Multi from StringBox, IntegerBox { } // 多重デリゲーションも可能！
```

### 🔄 統一ループ構文
```nyash
// ✅ 唯一の正しい形式
loop(condition) { }

### 🌟 birth構文 - 生命をBoxに与える
```nyash
// 🌟 「Boxに生命を与える」直感的コンストラクタ
box Life {
    name: StringBox
    energy: IntegerBox
    
    birth(lifeName) {  // ← Everything is Box哲学を体現！
        me.name = lifeName
        me.energy = 100
        print("🌟 " + lifeName + " が誕生しました！")
    }
}

// ✅ birth統一: すべてのBoxでbirthを使用
local alice = new Life("Alice")  // birthが使われる
```

### 🌟 ビルトインBox継承
```nyash
// ✅ Phase 12.7以降: birthで統一（packは廃止）
box EnhancedP2P from P2PBox {
    additionalData: MapBox
    
    birth(nodeId, transport) {
        from P2PBox.birth(nodeId, transport)  // 親のbirth呼び出し
        me.additionalData = new MapBox()
    }
}
```

### 🎯 正統派Nyashスタイル
```nyash
// 🚀 Static Box Main パターン - エントリーポイントの統一スタイル
static box Main {
    console: ConsoleBox    // フィールド宣言
    result: IntegerBox
    
    main() {
        // ここから始まる！他の言語と同じエントリーポイント
        me.console = new ConsoleBox()
        me.console.log("🎉 Everything is Box!")
        
        // local変数も使用可能
        local temp
        temp = 42
        me.result = temp
        
        return "Revolution completed!"
    }
}
```

### 📝 変数宣言厳密化システム
```nyash
// 🔥 すべての変数は明示宣言必須！（メモリ安全性・非同期安全性保証）

// ✅ static box内のフィールド
static box Calculator {
    result: IntegerBox     // 明示宣言
    memory: ArrayBox
    
    calculate() {
        me.result = 42  // ✅ フィールドアクセス
        
        local temp     // ✅ local変数宣言
        temp = me.result * 2
    }
}

// ❌ 未宣言変数への代入はエラー
x = 42  // Runtime Error: 未宣言変数 + 修正提案
```

### ⚡ 実装済み演算子
```nyash
// 論理演算子（完全実装）
not condition    // NOT演算子
a and b         // AND演算子  
a or b          // OR演算子

// 算術演算子
a / b           // 除算（ゼロ除算エラー対応済み）
a + b, a - b, a * b  // 加算・減算・乗算
```

### 🎯 match式（パターンマッチング）
```nyash
// 値を返す式として使用
local dv = match d {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    _ => 0
}

// ブロックで複雑な処理も可能
local result = match status {
    "success" => { log("OK"); 200 }
    "error" => { log("NG"); 500 }
    _ => 404
}

// 文として使用（値を捨てる）
match action {
    "save" => save_data()
    "load" => load_data()
    _ => print("Unknown")
}
```

### ⚠️ 重要な注意点
```nyash
// ✅ 正しい書き方（Phase 12.7文法改革後）
box MyBox {
    field1: TypeBox
    field2: TypeBox
    
    birth() {
        // 初期化処理
    }
}
```

### 🏗️ アーキテクチャ決定事項（2025-09-11）
**Box/ExternCall境界設計の最終決定**:
- **基本Box**: nyrt内蔵（String/Integer/Array/Map/Bool）
- **拡張Box**: プラグイン（File/Net/User定義）  
- **ExternCall**: 最小5関数のみ（print/error/panic/exit/now）
- **統一原則**: すべてのBoxはBoxCall経由（特別扱いなし）
- **表現統一**: Box=ハンドル(i64)、i8*は橋渡しのみ

詳細: [Box/ExternCall設計](docs/development/architecture/box-externcall-design.md)

## 📚 ドキュメント構造

### 📋 **ドキュメント配置ルール（SSOT）** ⭐NEW
**入口**: [docs/development/current/main/DOCS_LAYOUT.md](docs/development/current/main/DOCS_LAYOUT.md)

**3つの最小ルール**（必読！）:
1. **Phase 文書** → `docs/development/current/main/phases/phase-<N>/` （`main/` 直下に増やさない）
2. **設計図** → `docs/development/current/main/design/` （複数 Phase で参照される）
3. **調査ログ** → `docs/development/current/main/investigations/` （結論は 10-Now.md に反映）

**受け皿フォルダ別 README**:
- [phases/README.md](docs/development/current/main/phases/README.md) - Phase ログの説明
- [design/README.md](docs/development/current/main/design/README.md) - 設計図の説明
- [investigations/README.md](docs/development/current/main/investigations/README.md) - 調査ログの説明

### 🎯 最重要ドキュメント（開発者向け）
- **[Phase 15 セルフホスティング計画](docs/private/roadmap2/phases/phase-15/self-hosting-plan.txt)** - 80k→20k行革命
- **[Phase 15 ROADMAP](docs/private/roadmap2/phases/phase-15/ROADMAP.md)** - 現在の進捗チェックリスト
- **[Phase 15 INDEX](docs/private/roadmap2/phases/phase-15/INDEX.md)** - 入口の統合
- **[CURRENT_TASK.md](CURRENT_TASK.md)** - 現在進行状況詳細
- **[native-plan/README.md](docs/development/roadmap/native-plan/README.md)** - ネイティブビルド計画

### 📖 利用者向けドキュメント
- 入口: [docs/README.md](docs/README.md)
  - Getting Started: [docs/guides/getting-started.md](docs/guides/getting-started.md)
  - Language Guide: [docs/guides/language-guide.md](docs/guides/language-guide.md)
  - Reference: [docs/reference/](docs/reference/)

### 🎯 リファレンス
- **言語**:
  - [Quick Reference](docs/reference/language/quick-reference.md) ⭐最優先 - 1ページ実用ガイド
  - [LANGUAGE_REFERENCE_2025.md](docs/reference/language/LANGUAGE_REFERENCE_2025.md) - 完全仕様
- **MIR**: [INSTRUCTION_SET.md](docs/reference/mir/INSTRUCTION_SET.md)
- **API**: [boxes-system/](docs/reference/boxes-system/)
- **プラグイン**: [plugin-system/](docs/reference/plugin-system/)


## 📖 ドキュメントファースト開発（重要！）

### 🚨 開発手順の鉄則
**絶対にソースコードを直接読みに行かない！必ずこの順序で作業：**

1. **📚 ドキュメント確認** - まず既存ドキュメントをチェック
2. **🔄 ドキュメント更新** - 古い/不足している場合は更新
3. **💻 ソース確認** - それでも解決しない場合のみソースコード参照

### 🎯 最重要ドキュメント（2つの核心）

#### 🔤 言語仕様
- **[クイックリファレンス](docs/reference/language/quick-reference.md)** ⭐最優先 - 1ページ実用ガイド（ASI・Truthiness・演算子・型ルール）
- **[構文早見表](docs/quick-reference/syntax-cheatsheet.md)** - 基本構文・よくある間違い
- **[完全リファレンス](docs/reference/language/LANGUAGE_REFERENCE_2025.md)** - 言語仕様詳細

#### 📦 主要BOXのAPI
- **[Box/プラグイン関連](docs/reference/boxes-system/)** - APIと設計

### ⚡ API確認の実践例
```bash
# ❌ 悪い例：いきなりソース読む
Read src/boxes/p2p_box.rs  # 直接ソース参照

# ✅ 良い例：ドキュメント優先
Read docs/reference/  # まずドキュメント（API/言語仕様の入口）
# → 古い/不足 → ドキュメント更新
# → それでも不明 → ソース確認
```

## 🔧 重要設計書（迷子防止ガイド）

**設計書がすぐ見つからない問題を解決！**

### 🏗️ **アーキテクチャ核心**

#### JoinIR 設計図（超重要！）
- **[JoinIR アーキテクチャ概要](docs/development/current/main/joinir-architecture-overview.md)** ⭐超重要 - Loop/If/ExitLine/Boundary/PHI の全体図・契約・不変条件（normative SSOT）
- **[JoinIR 設計マップ](docs/development/current/main/design/joinir-design-map.md)** ⭐NEW - 実装導線の地図（どのファイルを触るか/入口/Pattern 分類/Allocator SSOT/Boundary SSA/チェックリスト）
- **[AI Plan Review Checklist](docs/development/current/main/design/ai-plan-review-checklist-ssot.md)** - LLM plan drift 防止用チェックリスト（BoxCount/BoxShape宣言・Recipe-first不変条件・Mechanical checks）

#### Recipe-first アーキテクチャ（最終形）⭐超重要
- **[Recipe-first Entry Contract](docs/development/current/main/design/recipe-first-entry-contract-ssot.md)** - Facts→Recipe→Verifier→Lower の主軸フロー
- **[RecipeTree と Parts](docs/development/current/main/design/recipe-tree-and-parts-ssot.md)** - Recipe 構造の SSOT
- **[generic_loop_v1 Shape SSOT](docs/development/current/main/design/generic-loop-v1-shape-ssot.md)** - Body shape 定義

**最終アーキテクチャ（方向性）**:
```
AST
 ↓
JoinIR (観測レイヤ: StepTree/ControlForm) ← 薄い、SSOT は置かない
 ↓
Facts (BodyShape + CondProfile)
 ↓
Verifier → VerifiedRecipe ← SSOT はここ
 ↓
Lower → MIR (CFG + SSA)
```

#### Plan Rule / CorePlan 入口
- **[Plan Registry (SSOT)](src/mir/builder/control_flow/plan/REGISTRY.md)** - 箱（plan rule）の責務・入口・契約を1枚に固定
  - **AcceptKind enum**: Facts→Lower 契約を型で強制（新しい受理形を追加 → Lower がコンパイルエラー）
  - 例: `loop_cond_break_continue` は `LoopCondBreakAcceptKind` で全 variant を match 必須

#### MIR・言語仕様
- **[MIR Callee革新](docs/development/architecture/mir-callee-revolution.md)** - 関数呼び出し型安全化・シャドウイング解決
- **[構文早見表](docs/quick-reference/syntax-cheatsheet.md)** - 基本構文・よくある間違い
- **[名前空間・using system](docs/reference/language/using.md)** - ドット記法・スコープ演算子・Phase 15.5計画

### 📋 **Phase 15.5重要資料**
- **[Core Box統一計画](docs/private/roadmap2/phases/phase-15.5/README.md)** - builtin vs plugin問題
- **[Box Factory設計](docs/reference/architecture/box-factory-design.md)** - 優先順位問題・解決策
- **[Callee実装ロードマップ](docs/private/roadmap2/phases/phase-15/mir-callee-implementation-roadmap.md)**

### 📖 **完全リファレンス**
- **[言語仕様](docs/reference/language/LANGUAGE_REFERENCE_2025.md)** - 全構文・セマンティクス
- **[プラグインシステム](docs/reference/plugin-system/)** - プラグイン開発ガイド
- **[Phase 15 INDEX](docs/private/roadmap2/phases/phase-15/INDEX.md)** - 現在進捗

### 🗂️ **control_flow モジュール構造**
**17モジュール分割完了** - `src/mir/builder/control_flow/` 参照
- `joinir/patterns/` - Pattern1-4ループ処理
- `joinir/merge/` - MIRマージ処理
- `exception/` - try/catch/throw処理

## 🔧 開発サポート

### 🎛️ 重要フラグ一覧（Phase 15）
```bash
# プラグイン制御
NYASH_DISABLE_PLUGINS=1     # Core経路安定化（CI常時）
NYASH_LOAD_NY_PLUGINS=1     # nyash.tomlのny_pluginsを読み込む

# 言語機能
--enable-using              # using/namespace有効化
NYASH_ENABLE_USING=1        # 環境変数版

# パーサー選択
--parser ny                 # Nyパーサーを使用
NYASH_USE_NY_PARSER=1       # 環境変数版
NYASH_USE_NY_COMPILER=1     # NyコンパイラMVP経路

# デバッグ
NYASH_CLI_VERBOSE=1         # 詳細診断
NYASH_DUMP_JSON_IR=1        # JSON IR出力
```

### 🐍 Python LLVM バックエンド
**場所**: `/src/llvm_py/` - llvmliteベースのMIR14→LLVM変換（2000行程度）
```bash
cd src/llvm_py && ./venv/bin/python llvm_builder.py test.json -o output.o
```

### 💡 アイデア管理（docs/development/proposals/ideas/ フォルダ）

**80/20ルールの「残り20%」を整理して管理**

```
docs/development/proposals/ideas/
├── improvements/     # 80%実装の残り20%改善候補
├── new-features/     # 新機能アイデア  
└── other/           # その他すべて（調査、メモ、設計案）
```

### 🧪 テスト実行

**詳細**: [テスト実行ガイド](docs/guides/testing-guide.md)

#### Phase 15 推奨スモークテスト
```bash
# コアスモーク（プラグイン無効）
./tools/jit_smoke.sh

# ラウンドトリップテスト
./tools/ny_roundtrip_smoke.sh

# プラグインスモーク（オプション）
NYASH_SKIP_TOML_ENV=1 ./tools/smoke_plugins.sh

# using/namespace E2E（要--enable-using）
./tools/using_e2e_smoke.sh
```

**ルート汚染防止**: `local_tests/`ディレクトリを使う！


### 🐛 デバッグ

#### パーサー無限ループ対策
```bash
# 🔥 デバッグ燃料でパーサー制御
./target/release/hakorune --debug-fuel 1000 program.hako      # 1000回制限
./target/release/hakorune --debug-fuel unlimited program.hako  # 無制限
./target/release/hakorune program.hako                        # デフォルト10万回
```

**対応状況**: must_advance!マクロでパーサー制御完全実装済み✅

## 🤝 プロアクティブ開発方針

エラーを見つけた際は、単に報告するだけでなく：

1. **🔍 原因分析** - エラーの根本原因を探る
2. **📊 影響範囲** - 他のコードへの影響を調査
3. **💡 改善提案** - 関連する問題も含めて解決策を提示
4. **🧹 機会改善** - デッドコード削除など、ついでにできる改善も実施

詳細: [開発プラクティス](docs/guides/development-practices.md)

## 🎆 面白事件ログ（爆速開発の記録）

### 世界記録級の事件たち：
- **JIT1日完成事件**: 2週間予定が1日で完成（8/27伝説の日）
- **プラグインBox事件**: 「こらー！」でシングルトン拒否
- **AIが人間に相談**: ChatGPTが「助けて」と言った瞬間
- **危険センサー発動**: 「なんか変だにゃ」がAIを救う

詳細は[開発事件簿](docs/private/papers/paper-k-explosive-incidents/)へ！

## ⚠️ Claude実行環境の既知のバグ

詳細: [Claude環境の既知のバグ](docs/tools/claude-issues.md)

### 🐛 Bash Glob展開バグ（Issue #5811）

```bash
# ❌ 失敗するパターン
ls *.md | wc -l          # エラー: "ls: 'glob' にアクセスできません"

# ✅ 回避策1: bash -c でラップ
bash -c 'ls *.md | wc -l'

# ✅ 回避策2: findコマンドを使う
find . -name "*.md" -exec wc -l {} \;
```

## 🚨 コンテキスト圧縮時: 作業停止→状況確認→CURRENT_TASK.md確認→ユーザー確認

---

Notes:
- ここから先の導線は README.md に集約
- 詳細情報は各docsファイルへのリンクから辿る
- このファイルは500行以内が目安（あくまで目安であり、必要に応じて増減可）
- Phase 15セルフホスティング実装中！詳細は[Phase 15](docs/private/roadmap2/phases/phase-15/)へ
