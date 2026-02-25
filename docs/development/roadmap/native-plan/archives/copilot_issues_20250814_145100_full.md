# 🤖 Copilot様 作業予定・課題整理 (Phase 0-14 全体ロードマップ)
# Generated: 2025-08-14 (Git履歴から復元・更新)
# Purpose: Claude×Copilot協調開発のための情報共有

================================================================================
🎯 次期最優先タスク (Phase 8.5以降)
================================================================================

## 🚀 Phase 8.4完了報告 (2025-08-14)
Status: ✅ 完了 (Copilot PR #56マージ済み)

### ✅ AST→MIR Lowering完全実装
- User-defined Box: `box DataBox { init { value } }`
- Object creation: `new DataBox(42)`
- Field access: `obj.value` 
- Method calls: `c.increment()`
- Delegation: `from Parent.greet()`
- Static Main互換性維持

### 🧪 統合テスト結果（2025-08-14）
- ✅ **AST→MIR**: 完全動作
- ✅ **インタープリター**: 完全動作（結果30）
- 🚨 **VM**: 動作するが結果が`void`（要修正）
- 🚨 **WASM**: String constant未対応（Phase 8.5で解決）

### 📋 発見された課題
- VM実行結果問題: BoxCall後の戻り値が正しく返らない
- WASM対応不足: 複雑なMIR命令（String constant, BoxCall）に未対応
- 次期Phase 8.5での25命令MIR階層化が必要

================================================================================

## 🔧 Phase 8.5: MIR 25命令階層化（最優先）
Status: ⭐ **CRITICAL** 
Priority: **最重要** (Phase 8.4完了直後の次期目標)

### 🎯 実装目標
ChatGPT5 + AI大会議決定版25命令MIR実装
- 期間: 3週間
- 効果: VM/WASM問題根本解決
- 詳細仕様: `/docs/予定/native-plan/issues/phase_8_5_mir_25_instruction_specification.md`

### 📋 25命令セマンティック階層化
**Tier-0: 普遍コア（8命令）**
```mir
Const, BinOp, Compare, Branch, Jump, Phi, Call, Return
```

**Tier-1: Nyashセマンティクス（12命令）**
```mir
NewBox, BoxFieldLoad, BoxFieldStore, BoxCall, Safepoint,
RefGet, RefSet, WeakNew, WeakLoad, WeakCheck, Send, Recv
```

**Tier-2: 実装補助・最適化友好（5命令）**
```mir
TailCall, Adopt, Release, MemCopy, AtomicFence
```

### 🎯 期待される効果
- **VM問題解決**: BoxCallの正しい実装で戻り値問題修正
- **WASM対応**: 階層化により複雑MIR→単純WASM変換
- **Everything is Box**: BoxFieldLoad/Storeで明確なBox中心設計
- **JIT準備**: セマンティクス保持で高度最適化基盤確立

================================================================================

## 🏎️ Phase 8.6: VM性能改善（緊急）
Status: 🚨 **緊急** 
Priority: **High** (Phase 8.5完了後)

### 🚨 緊急問題
**現状**: VM（119.80ms）< Interpreter（110.10ms）= 0.9倍の性能劣化
**新問題**: VM BoxCall後の戻り値が`void`（Phase 8.4テストで発見）

### 📋 技術的課題
- VM実行エンジンのプロファイリング
- 命令ディスパッチ最適化（threaded code等）
- レジスタベースVM化検討  
- メモリプール最適化
- BoxCall実装修正（戻り値問題）

### 🎯 成功基準
- VM性能 > Interpreter性能（最低2倍目標）
- BoxCall戻り値の正常動作
- MIR→VM変換時間の短縮

================================================================================

## 🧪 Phase 8.7: Real-world Memory Testing
Status: 📋 **計画済み**
Priority: **High** (Phase 8.5-8.6完了後)

### 🎯 実装目標
kilo（テキストエディタ）実装によるfini/weak参照システム実証
- 期間: 2週間
- 詳細仕様: `/docs/予定/native-plan/issues/phase_8_7_real_world_memory_testing.md`

### 📋 検証項目
- 1000+オブジェクト管理テスト
- 循環参照回避確認（weak参照）
- fini()伝播の正確性確認
- WASM環境での動作確認

================================================================================
🗺️ Phase 0-14 全体ロードマップ (復元完了)
================================================================================

## Phase 0: Stabilize native CLI build (Linux/Windows)

Summary:
- CLIバイナリ nyash を最小構成で安定ビルド・実行できる状態にする。
- examples/GUI をデフォルトのビルド対象から外し、開発の足場を固める。

Why:
- 以降の MIR/VM/JIT 開発を素早く検証できる基盤づくり。

Scope:
- Cargo の features で GUI/examples 等を切り分け、デフォルトは CLI 最小にする。
- CLI オプションの動作点検（--dump-mir / --verify）。
- ローカル実行導線を README に明記（docs/guides/how-to-build-native/README.md）。

Tasks:
- Cargo.toml: examples/GUI を feature でガード（default は CLI 最小）。
- ビルド検証: `cargo build --bin nyash`（Linux/Windows）。
- 実行検証: `cargo run -- ./local_tests/sample.hako`。
- ドキュメント: 上記手順を how-to-build-native に追記/点検。

Acceptance Criteria:
- Linux/Windows で `cargo build --bin nyash` が成功する。
- `local_tests/` 配下の簡単な .hako が実行できる。
- 他 bin/examples が壊れていても `--bin nyash` だけで通る。

Out of Scope:
- examples/GUI の修理・最適化。
- JIT/AOT/WASM。

References:
- docs/guides/how-to-build-native/README.md
- docs/nativebuild大作戦/chatgptネイティブビルド大作戦.txt（Phase 0）
- CURRENT_TASK.md

Copilot Notes:
- まずは features 分離と `--bin nyash` でビルドが通る状態を作る。README の手順確認まで含めて PR に反映。

------------------------------------------------------------

## Phase 1: Minimal MIR + VM backend (lowering + runner)

Summary:
- AST → MIR の最小 lowering と、VM バックエンドでの実行を通す。

Scope:
- MIR: Const, BinOp, Compare, Branch, Jump, Phi, Return の最小命令
- Lowering: リテラル/二項演算/if/loop/return のみ
- VM: 上記命令の最小実装

Tasks:
- instruction.rs: 最小命令の定義
- builder.rs: 上記 AST 範囲を lowering
- vm.rs: 実装 + stats（命令数）

Acceptance Criteria:
- `--dump-mir` が最小サンプルで期待通り
- `--backend vm` で実行して結果一致

Out of Scope:
- 例外/関数/Box 参照/弱参照

------------------------------------------------------------

## Phase 2: Control-flow coverage (if/else/loop/phi correctness)

Summary:
- 制御フローの網羅と Phi の整合性検証を拡充。

Scope/Tasks:
- if/else nested, loop with breaks, nested loops のスナップショット
- Phi の入力ブロック/値の対応を Verifier で強化

Acceptance Criteria:
- 代表制御フローの snapshot が安定し、verify も通る

------------------------------------------------------------

## Phase 3: Exceptions (throw/try/catch/finally) minimal lowering

Summary:
- 例外機構の最小 lowering を導入（詳細設計は簡素）。

Scope/Tasks:
- MIR: Throw, TryBegin/TryEnd, Catch, FinallyBegin/End（最小）
- builder.rs: try/catch/finally ノードの下ろし
- VM: 例外伝播を最小で（未捕捉はエラー）

Acceptance Criteria:
- 代表 try/catch/finally のスナップショットと VM 実行

Out of Scope:
- 例外の型体系、詳細な stack map

------------------------------------------------------------

## Phase 4: Functions and calls (BoxCall minimal)

Summary:
- 関数呼び出し/BoxCall を最小導入（効果注釈は保守的）。

Scope/Tasks:
- MIR: Call, BoxCall（effects = READS_HEAP など保守）
- builder.rs: FunctionCall/MethodCall の最小対応
- VM: 呼び出し/戻り値

Acceptance Criteria:
- 簡単な関数定義/呼び出しの MIR/VM が通る

Out of Scope:
- 可変長/キーワード引数、FFI

------------------------------------------------------------

## Phase 5.0: Parser/AST stabilization for lowering

Summary:
- lowering 対象 AST の表現ぶれを修正、安定化。

Scope/Tasks:
- AST: If/Loop/Return/Assignment/Local などの統一
- Parser: エラー復帰/スパン情報の見直し

Acceptance Criteria:
- builder.rs の分岐がシンプル化、テストが安定

------------------------------------------------------------

## Phase 5.1: Control-flow edge cases + verifier hardening

Summary:
- ブロック未終端/未到達/自己分岐等の検証強化でクラッシュ回避。

Scope/Tasks:
- Verifier: 未終端ブロック検出、到達不能検出
- Builder: Jump/Branch の生成前後の状態管理改善

Acceptance Criteria:
- 不正ケースを含むスナップショット/verify が緑

------------------------------------------------------------

## Phase 5.2: Lowering for static box Main (BoxDeclaration → main body)

Summary:
- static box Main { main() { ... } } を MirBuilder で受け、main() の body を Program として lowering する経路を実装。

Scope/Tasks:
- AST: BoxDeclaration(is_static=true, name=Main) を検出 → main() を抽出
- Lowering: body を Program に変換して既存経路に渡す
- Tests: local_tests/mir_loop_no_local.hako で dump/VM が通る

Acceptance Criteria:
- `--dump-mir` が static Main サンプルで成功
- `--backend vm` で実行成功

References:
- docs/guides/how-to-build-native/issues/phase5_2_static_main_lowering.md

------------------------------------------------------------

## Phase 6: Box ops minimal (Ref/Weak + Barriers no-op)

Summary:
- 参照/弱参照/バリア（no-op）を最小導入。

Scope/Tasks:
- MIR: RefNew/RefGet/RefSet/WeakNew/WeakLoad/BarrierRead/Write
- Lowering: New/FieldAccess/MethodCall の最小対応
- VM: 参照テーブル/weak テーブルで動作（fini 不変は維持）

Acceptance Criteria:
- 代表サンプルで dump/VM/verify が通る

References:
- docs/guides/how-to-build-native/issues/phase6_box_ops_minimal.md

------------------------------------------------------------

## Phase 7: Async model (nowait/await) in MIR

Summary:
- nowait/await を MIR に導入し、現行 FutureBox と連携。

Scope/Tasks:
- MIR: FutureNew/FutureSet/Await（スレッドベース）
- Lowering: nowait→Future 作成、await→wait_and_get
- VM: FutureBox 実装を利用

Acceptance Criteria:
- 代表ケースで正しく並行実行→await 回収

References:
- docs/guides/how-to-build-native/issues/phase7_async_mir.md

------------------------------------------------------------

## Phase 8: MIR→WASM codegen (browser/wasmtime; sandboxed; Rust runtime free)

Summary:
- MIR から素の WebAssembly を生成し、ブラウザ/wasmtime（WASI）でサンドボックス実行する。
- Rust はコンパイラ本体のみ。実行は純WASM＋ホストimport（env.print など）。

Scope/Tasks:
- ABI/Imports/Exports 定義（exports: main/memory、imports: env.print(i32) 等の最小）
- 線形メモリと簡易ヒープ（bump/自由リスト）
- 命令カバレッジ（段階導入）: 算術/比較/分岐/loop/return/print、RefNew/RefSet/RefGet（Phase 6 整合）、Weak/Barrier はダミー

Acceptance Criteria:
- wasmtime 実行で戻り値/print が期待通り（PoC1–2）
- Ref 系がメモリ上で正しく動作（PoC2）
- Weak/Barrier のダミー実装を含むWASMが生成・実行（PoC3）
- CLI `--backend wasm` は未実装でもよいが、実装する場合は明瞭にエラーメッセージ/誘導

References:
- docs/予定/native-plan/README.md（Phase 8 節）
- docs/説明書/wasm/*（ユーザー向けメモ）

### Phase 8.3 完了状況 (2025-08-14)
✅ Box操作WASM実装 (RefNew/RefGet/RefSet)
✅ ベンチマークシステム統合 (13.5倍実行高速化実証)
✅ CLI統合完了

------------------------------------------------------------

## 🔧 Phase 8.4: AST→MIR Lowering完全実装 (最優先)
Note: 詳細は issues/phase_8_4_ast_mir_lowering.md に集約。本節は将来、要約のみを残す方針です.

Summary:
- ユーザー定義Box、フィールドアクセス等の未実装部分を完成
- Phase 8.3のBox操作WASMを実際にテスト可能にする

Priority: **Critical** (現在の最優先事項)
Expected Duration: 1週間

### 実装範囲
- [ ] ユーザー定義Box: `box DataBox { init { field } }`
- [ ] オブジェクト生成: `new DataBox()`  
- [ ] フィールドアクセス: `obj.field`
- [ ] フィールド代入: `obj.field = value`
- [ ] from構文: `from Parent.method()`
- [ ] override構文: `override method() { ... }`

### 成功基準
- Phase 8.3のBox操作WASMが実際に動作
- test_wasm_box_ops.hako が正常実行
- ユーザー定義Boxの完全サポート

------------------------------------------------------------

## 🧠 Phase 8.5: MIRセマンティック階層化（AI大会議決定版）
Note: 詳細は issues/phase_8_5_mir_25_instruction_specification.md に集約。本節は将来、要約のみを残す方針です.

Summary:
- 方針転換: ChatGPT5の20命令intrinsic戦略 → Gemini+Codex一致推奨の25命令階層化
- 理由: JIT/AOT最適化阻害・Everything is Box意味喪失・長期コスト増の問題判明
- 二相ロワリング: 25命令維持パス（VM/JIT/AOT）+ 20+intrinsic降格パス（WASM/最小実装）

Priority: High (Phase 8.4完了後)
Expected Duration: 3週間

### AI大会議分析結果
**Gemini先生（理論）**: 「賢いコンパイラは、賢いMIRから生まれる」
- RefNew/WeakLoadのintrinsic化は最適化機会を失う悪手
- セマンティック階層化で意味保持が最適化の鍵

**Codex先生（実装）**: 二相ロワリング戦略が実用的最適解
- 実装コスト: 5命令追加で10-20人日（intrinsic戦略より安い）
- マイクロベンチ実測でパフォーマンス検証

### 確定版MIR（25命令）- ChatGPT5完全仕様
**Tier-0: 普遍的コア（8命令）**
```mir
Const, BinOp, Compare, Branch, Jump, Phi, Call, Return
```

**Tier-1: Nyashセマンティクス（12命令）**
```mir
NewBox,        // 強所有のBox生成（所有森のノード）
BoxFieldLoad,  // Boxのフィールド読み（Everything is Box核心）
BoxFieldStore, // Boxのフィールド書き（mut効果）
BoxCall,       // Boxのメソッド呼び出し（動的/静的両方）
Safepoint,     // 分割finiや割込み許可ポイント
RefGet,        // 参照（強/弱を問わず）を値として取得
RefSet,        // 参照の差し替え（所有規則検証付き）
WeakNew,       // weak ハンドル生成（非所有リンク作成）
WeakLoad,      // weak から生存チェック付きで強参照取得（失効時null）
WeakCheck,     // weak の生存確認（bool）
Send,          // Bus送信（io効果）
Recv           // Bus受信（io効果）
```

**Tier-2: 実装補助・最適化友好（5命令）**
```mir
TailCall,      // 末尾呼び出し（スタック節約）
Adopt,         // 所有移管: this が子を強所有に取り込む
Release,       // 強所有を解除（weak化 or null化）
MemCopy,       // 小さなメモリ移動（構造体/配列最適化フック）
AtomicFence    // 並行時の順序保証（Actor/Port境界で使用）
```

### 二相ロワリング戦略
- パスA: VM/JIT/AOT向け（25命令のまま最適化）
- パスB: WASM/最小実装向け（25→20+intrinsic降格）
- バックエンド能力に応じて最適形式選択

### 効果（Effect）システム（ChatGPT5設計）
- **pure**: Const, BinOp, Compare, Phi, RefGet, WeakNew, WeakLoad, WeakCheck
- **mut**: BoxFieldStore, RefSet, Adopt, Release, MemCopy
- **io**: Send, Recv, Safepoint, AtomicFence
- **control**: Branch, Jump, Return, TailCall
- **context依存**: Call, BoxCall（呼び先効果に従属）

**最適化ルール**: 「pure同士の再順序化OK」「mutは同一Box/同一Fieldで依存保持」「ioは再順序化禁止」

### 検証（Verifier）要件
- **所有森**: `strong in-degree ≤ 1`（NewBox/Adopt/Release/RefSetで常時検査）
- **強循環禁止**: 強エッジのみ辿ってDAG（森）であること
- **weak/強相互**: 双方向とも強 → エラー（片側はWeakNew経由で弱化）
- **WeakLoad/WeakCheck**: 失効時はnull/falseを返す（例外禁止、決定的挙動）

### 🤖 Copilot協力期待
- **Tier-0/1実装**: Everything is Box哲学の完璧なIR化（BoxFieldLoad/Store核心）
- **weak参照システム**: WeakNew/WeakLoad/WeakCheck三位一体実装
- **所有移管**: Adopt/Release命令による安全で効率的なメモリ管理
- **効果システム**: pure/mut/io/control効果の正確な実装とVerifier統合
- **最適化フック**: TailCall/MemCopy/AtomicFenceの実装補助
- **二相ロワリング**: 25命令維持パス + 20+intrinsic降格パス構築

### 成功基準
- [ ] **25命令完全実装**: ChatGPT5仕様の完璧な実装
- [ ] **効果システム動作**: pure再順序化・mut依存保持・io順序保証
- [ ] **Verifier動作**: 所有森・strong循環・安全性検証
- [ ] **Golden MIRテスト**: 全バックエンドでMIR一致
- [ ] **行動一致テスト**: 同入力→同出力（weak失効時null/false含む）
- [ ] **性能要件**: VM≥Interpreter、WASM≥VM継続検証

### バックエンド指針（ChatGPT5設計）
- **Interpreter**: 25命令を素直に実装（正しさの基準）
- **VM**: Register-VM + direct-threading。Send/Recvはローカル判定時にインライン化
- **WASM**: Send/Recvはhost import。MemCopyはmemory.copyに対応
- **JIT（将来）**: TailCall最適化、WeakLoadは世代タグでO(1)生存チェック

References:
- docs/予定/native-plan/MIR仕様書.txt（ChatGPT5完全仕様）
- docs/予定/native-plan/issues/phase_8_5_mir_25_instruction_specification.md

------------------------------------------------------------

## 🏎️ Phase 8.6: VM性能改善 (緊急)

Summary:
- VMがインタープリターより遅い問題（0.9倍）を解決
- MIR→VM実行の最適化でインタープリターを上回る性能へ

Priority: High (Phase 8.5完了後)
Expected Duration: 2週間

### 問題分析
**現状**: VM (119.80ms) < Interpreter (110.10ms)
**推定原因**:
- MIR変換オーバーヘッド
- VM命令ディスパッチの非効率性
- メモリ管理コスト

### 技術的アプローチ
- [ ] VM実行エンジンのプロファイリング
- [ ] 命令ディスパッチ最適化（threaded code等）
- [ ] レジスタベースVM化検討
- [ ] メモリプール最適化

### 🤖 Copilot協力期待
- VM実装のボトルネック特定
- 効率的な命令ディスパッチ実装
- スタックマシン vs レジスタマシン判断

### 成功基準
- VM性能 > Interpreter性能（最低2倍目標）
- MIR→VM変換時間の短縮
- メモリ使用量の削減

------------------------------------------------------------

## 🧪 Phase 8.7: Real-world Memory Management Testing (ChatGPT協調設計)

Summary:
- 実用アプリケーション開発によるNyashメモリ管理システムの実証テスト
- finiシステム・weak参照の実用性を複雑なアプリケーションで検証

Priority: High (Phase 8.4-8.6完了直後)
Expected Duration: 2週間

### Phase 8.7A: kilo（テキストエディタ）
**技術的特徴**:
- サイズ: <1k LOC（超小型、最初の成功体験）
- メモリパターン: Editor -> (Rows -> Syntax) 木構造＋相互参照
- ChatGPT設計: Editor削除でRows自動解放、逆参照をweak化

**実装範囲**:
- [ ] Editor/Row/EditorState基本構造実装
- [ ] weak参照による循環参照回避（`me.editor = weak editor_ref`）
- [ ] fini()システムによる自動メモリ解放
- [ ] 大量オブジェクト（1000+ Rows）管理テスト

**検証ポイント**:
- [ ] Editor削除でRows自動解放確認
- [ ] 相互参照でメモリリークなし確認  
- [ ] weak参照の自動null化確認
- [ ] fini()伝播の正確性確認

### Phase 9.5予定: tiny-web-server（HTTPサーバ）
**将来実装**（JIT実装後）:
- 複雑度: 中〜高（Server -> Clients -> Requests並行処理）
- I/O管理: ソケット・ファイルハンドルの確実解放
- 同時接続・早期切断・例外経路でのfini伝播テスト

### 🤖 Copilot協力期待
- 実用的なメモリ管理パターンの実装
- weak参照構文の適切な使用
- デバッグ支援機能（--debug-memory, --trace-weak）
- WASM環境でのメモリ管理互換性

### 成功基準
- [ ] 全テストケースでメモリリークなし
- [ ] 循環参照でも正常解放確認
- [ ] WASM実行でもメモリ管理正常
- [ ] ベンチマーク性能劣化なし

### 期待される効果
- Nyashメモリ管理システムの実用性実証
- Everything is Box哲学の実用レベル確認  
- メモリ安全なプログラミングパターン確立

References:
- docs/予定/native-plan/issues/phase_8_7_real_world_memory_testing.md

------------------------------------------------------------

## 🚀 Phase 9: AOT WASM実装（最優先）

Summary:
- wasmtime compileによるAOT実行ファイル生成で確実なユーザー価値提供

Scope/Tasks:
- `wasmtime compile` 統合実装
- `--compile-native` / `--aot` CLI追加
- 単一バイナリ梱包（`include_bytes!`）
- 起動時間・配布サイズ最適化

Acceptance Criteria:
- `nyash --compile-native app.hako -o app.exe` 動作
- 起動時間大幅短縮（JIT起動コスト除去）
- 配布可能実行ファイル生成

Priority: **Critical** (Phase 8.6完了直後)
Expected Duration: 2-3週間

### 技術的実装詳細
🤖 Copilot協力期待:
- wasmtime::Config統一実装
- .cwasm生成・実行パイプライン  
- 互換性キー管理（CPU機能・wasmtimeバージョン）
- パッケージング（単一バイナリ梱包）

### パフォーマンス目標
- 現在のWASM JIT (13.5倍実行) → AOT (500倍目標：起動含む)
- 配布ファイルサイズ: <10MB目標
- 起動時間: <100ms目標

### 期待される効果
- **即座実用価値**: 配布可能実行ファイル生成
- **差別化優位**: Everything is BoxのネイティブAOT実現
- **LLVM準備**: AOT基盤確立でLLVM移行準備

------------------------------------------------------------

## 🌐 Phase 9.5: HTTPサーバー実用テスト（AOT検証）

Summary:
- AOT実装完了後の複雑アプリケーション検証（並行処理・メモリ管理・実用性能）

Scope/Tasks:
- tiny-web-server実装（HTTP/1.1対応）
- 同時接続・早期切断・例外経路テスト
- AOT環境での真の性能測定
- 配布可能HTTPサーバーデモ

Acceptance Criteria:
- `http_server.exe`として配布可能
- 同時100接続でメモリリークなし
- fini()システム確実動作（I/Oハンドル解放）
- AOT性能でベンチマーク測定

Priority: High (Phase 9完了直後)
Expected Duration: 2週間

### 技術的複雑度
```nyash
box HTTPServer {
    init { clients, requests, handlers }
    
    acceptConnections() {
        loop(me.running) {
            local client = me.socket.accept()
            nowait me.handleClient(client)  // 非同期並行処理
        }
    }
    
    handleClient(client) {
        local request = client.readRequest()
        local response = me.processRequest(request)
        client.sendResponse(response)
        client.fini()  // 重要: 確実なリソース解放
    }
}
```

### 検証ポイント
- **並行処理**: nowait/awaitのAOT実行性能
- **メモリ管理**: Server→Clients→Requests木構造+weak参照
- **I/Oリソース**: ソケット・ファイルハンドルの確実解放
- **実用性能**: リアルHTTP負荷でのAOT効果測定

### 🤖 Copilot協力期待
- Socket・HTTP実装の効率化
- 複雑なメモリ管理パターン検証
- 負荷テスト・ベンチマーク整備
- AOT最適化効果の定量測定

------------------------------------------------------------

## 🏆 Phase 10: LLVM Direct AOT（最高性能実現）

Summary:
- MIR→LLVM IR直接変換による最高性能AOT実現（Cranelift JITスキップ）

Scope/Tasks:
- MIR→LLVM IR lowering実装
- エスケープ解析・ボックス化解除
- LTO・PGO・高度最適化統合
- Everything is Box最適化

Acceptance Criteria:
- 1000倍高速化達成
- プロダクションレベル最適化
- 他言語との競争力確立

Priority: Medium (Phase 9.5完了後)
Expected Duration: 4-6ヶ月

### 技術アプローチ
🤖 Copilot協力期待:
- **LLVM統合**: MIR→LLVM IR変換基盤
- **エスケープ解析**: Box→スタック値最適化
- **型特殊化**: コンパイル時型推論・特殊化
- **LTO統合**: Link-time optimization
- **PGO対応**: Profile-guided optimization

### Everything is Box最適化戦略
- **Box回避**: スタック割り当て・直接レジスタ配置
- **NaN Boxing**: 効率的な値表現
- **型推論**: コンパイル時型特定・最適化
- **メモリレイアウト**: 連続配置・キャッシュ効率

### パフォーマンス目標
- **実行性能**: 1000倍高速化（現在13.5倍 → 目標13500倍相当）
- **メモリ効率**: Box割当数80%削減
- **起動時間**: ネイティブレベル（<10ms）
- **競合比較**: C/C++/Rust並みの性能

### Cranelift JIT位置づけ変更
**Phase 12以降の将来オプション**:
- JIT開発体験向上（nyashプログラマー向け）
- REPL・インタラクティブ実行
- プロファイル駆動最適化
- 言語完成後の付加価値機能

------------------------------------------------------------

## Phase 11-14: Infrastructure & Polish

### Phase 11: MIR Optimization Framework
- エスケープ解析基盤
- 型特殊化・ボックス化解除
- デッドコード除去

### Phase 12: Advanced JIT Features  
- Profile-guided optimization
- インライン展開
- レジスタ割り当て最適化

### Phase 13: Production Readiness
- GC統合最適化
- メモリ使用量最適化
- 起動時間短縮

### Phase 14: Packaging/CI polish

Summary:
- Windows/Linux の配布パッケージ化と CI 整備。

Scope/Tasks:
- GitHub Actions: Windows(MSVC)/WSL+cargo-xwin のマトリクス
- dist/: nyash(.exe) + LICENSE/README 同梱

Acceptance Criteria:
- リリースアーティファクトが自動生成される

================================================================================
🧠 AI大会議 + 実用優先戦略で得られた技術的知見 (2025-08-14更新)
================================================================================

## Gemini先生の助言（修正適用）
✅ エスケープ解析・ボックス化解除が性能の鍵  
✅ wasmtime compileは短期的に実用的 → **Phase 9で最優先実装**
✅ WASM実行は確実に高速（13.5倍実証済み）
🔄 Cranelift → LLVM段階的アプローチ → **実用優先でLLVM直接へ**

## codex先生の助言（重点化）
✅ MIR前倒し実装推奨（全バックエンドが恩恵）
✅ wasmtime互換性管理が重要 → **AOT実装で最重要**
✅ CPU差異対応 (baseline/v3二段ビルド)
✅ 起動時間・割当削減・配布体験がKPI → **AOT価値の核心**

## Claude統合分析（実用優先）
✅ 実用価値最大化: WASM+AOTで十分な競争力
✅ 開発効率: Cranelift JITの恩恵限定的（cargo build変わらず）
✅ Everything is Box最適化が差別化の核心
✅ 時間効率: 2-3ヶ月節約でLLVM集中投資

## 🎯 実用優先戦略の確定理由
- **ユーザー体験**: WASM既に動作、AOTで配布価値追加
- **開発効率**: Cranelift JITは重複投資（Rust開発環境改善せず）
- **競合優位**: AOT+LLVM早期実現で差別化
- **リソース効果**: 限られた開発時間の最大効率化

================================================================================
💡 Copilot様への具体的お願い・相談事項
================================================================================

## 🔧 Phase 8.3完了・次期フェーズ準備

### MIRダイエット準備
❓ 現在35命令→20命令削減のintrinsic戦略実装は？
❓ ChatGPT5推奨の3-point setアプローチ最適化は？
❓ Portability Contract v0での互換性確保方法は？

### Phase 9 AOT WASM実装（最優先）
❓ wasmtime compileの実用配備方法は？
❓ .cwasm生成・単一バイナリ梱包戦略は？
❓ 互換性キー管理（CPU機能・wasmtimeバージョン）は？
❓ 起動時間最適化の実装アプローチは？

### Phase 9.5 HTTPサーバー検証
❓ Socket・HTTP実装の効率的な設計は？
❓ 並行処理でのメモリ管理パターンは？
❓ AOT環境でのI/Oリソース管理は？
❓ 負荷テスト・ベンチマーク設計は？

### Phase 10 LLVM Direct AOT
❓ MIR→LLVM IR変換の効率実装は？
❓ エスケープ解析・ボックス化解除の実装戦略は？
❓ LTO・PGO統合の技術的ハードルは？

## 🚀 長期戦略相談

### Everything is Box最適化
❓ Box操作の根本的高速化戦略は？
❓ エスケープ解析によるスタック化判定は？
❓ 型特殊化・ボックス化解除の実装戦略は？

### ベンチマーク拡張
❓ AOT性能測定の追加指標は？
❓ 1000倍高速化実現のマイルストーン設計は？
❓ 他言語(JavaScript V8, Rust, C++)との競争力分析は？
❓ HTTPサーバー負荷テストの効率設計は？

================================================================================
📊 進捗管理・コミュニケーション
================================================================================

## 🤝 協調開発ルール

### コミット・マージ戦略
✅ 大きな変更前にはdocs/CURRENT_TASK.mdで情報共有
✅ ベンチマーク機能は最優先で維持
✅ CLI統合は両機能を統合的に対応
✅ 競合発生時は機能優先度で解決

### 進捗報告
📅 週次: 進捗状況をCURRENT_TASK.mdに反映
📅 完了時: 新機能のベンチマーク結果を共有
📅 問題発生: AI大会議で技術的相談

### 品質保証
✅ cargo check でビルドエラーなし
✅ 既存ベンチマークが regression なし
✅ 新機能のドキュメント整備
✅ テストケース追加・CI通過

================================================================================
🎯 期待される成果・インパクト
================================================================================

## Phase 8完了時の成果 (達成済み)
🏆 RefNew/RefGet/RefSet WASM完全動作
🏆 Box操作ベンチマーク追加
🏆 メモリレイアウト最適化効果測定
🏆 オブジェクト指向プログラミングWASM対応
🏆 25命令MIR階層化完了（Phase 8.5）
🏆 VM性能改善完了（Phase 8.6）

## Phase 9-10実用優先展望
🚀 **AOT WASM実装** (Phase 9 - 2-3週間): 配布可能実行ファイル
🚀 **HTTPサーバー検証** (Phase 9.5 - 2週間): 実用アプリデモ
🚀 **LLVM Direct AOT** (Phase 10 - 4-6ヶ月): 1000倍高速化
🚀 **実用競争力確立**: 他言語との差別化完成

## 言語としての完成度向上
💎 Everything is Box哲学のネイティブ実現
💎 開発効率性と実行性能の両立
💎 4つの実行形態対応（Interpreter/VM/WASM/AOT）+ 将来JIT
💎 現代的言語としての地位確立

================================================================================
📞 連絡・相談方法
================================================================================

技術的相談や進捗報告は、以下の方法でお気軽にどうぞ：

1. 📝 GitHub Issues・Pull Request
2. 📋 docs/CURRENT_TASK.md コメント
3. 🤖 AI大会議 (重要な技術決定)
4. 💬 コミットメッセージでの進捗共有

どんな小さなことでも相談大歓迎です！
一緒にNyashを最高の言語にしていきましょう🚀

================================================================================
最終更新: 2025-08-14 (実用優先戦略・Phase 9-10再設計完了)
作成者: Claude (AI大会議結果 + 実用優先戦略統合)

🎯 重要な変更点:
- Phase 9: JIT planning → AOT WASM実装（最優先）
- Phase 9.5: HTTPサーバー実用テスト追加（AOT検証）
- Phase 10: AOT exploration → LLVM Direct AOT（最高性能）
- Cranelift JIT: Phase 12以降の将来オプションに変更
- HTTPサーバー: kilo後のタイミングで実用性能検証に特化
================================================================================
================================================================================
🚀 Phase 9.0: Box FFI/ABI + Library-as-Box + RuntimeImports (WASM) — Proposal
================================================================================

Summary:
- 「あらゆるライブラリを箱にする」の共通ABI（Box FFI/ABI）を策定し、MIRとバックエンドの双方から利用可能にする。
- BID (Box Interface Definition) として型/メソッド/効果を記述し、各ターゲット（WASM/VM/言語コード生成）へ自動写像。
- WASM向けの実体として RuntimeImports をABI準拠で拡張（Canvas/Console最小セットから）。

Why:
- nyashがrustに依存せずnyasuをビルド（実行時は純WASM＋ホストAPI）
- MIRから任意言語へ変換（BID→各言語のFFI/インポートへマッピング）
- 「Everything is Box」APIを外部ライブラリにも一貫適用

Scope/Tasks:
1) ABI/BIDの策定（最優先）
   - 型: i32/i64/f32/f64/string(ptr,len)/bool/boxref/array、null/voidの扱い
   - 呼び出し規約: 名前解決（namespace.box.method）、エラー/例外、同期/非同期
   - 効果: pure/mut/io/control（既存MIR効果と整合）
   - 仕様文書: docs/box_ffi_abi.md、BIDフォーマット（YAML/JSON）

2) MIR拡張（extern call）
   - 命令: ExternCall(dst, iface, method, args[])
   - 検証: BIDと型/効果の整合、最適化時の順序制約
   - Lowering: 既存のBoxCall/Field opsとの橋渡し戦略

3) WASM RuntimeImports実装（ABI準拠）
   - import群: env.console.log, env.canvas.fillRect/fillText 等（ptr/lenで文字列）
   - JSホスト: importObject実装（DOM解決→Canvas/Console呼び出し）
   - 最小E2E: CanvasとConsoleが「直接WASM生成」経路で動作

4) コード生成への写像
   - WASM: ExternCall→import呼び出し（メモリから文字列復元）
   - VM: ExternCall→ホスト関数テーブル（モックで可）
   - 言語出力: TypeScript/Python/Rust へのFFI/ラッパ生成（後続）

5) 移行/互換
   - 既存 wasm-bindgen WebCanvasBox/WebConsoleBox はレガシー互換としつつ、
     同一APIをABI経由で提供して統一。

Acceptance Criteria:
- docs/box_ffi_abi.md（初版）＋BIDのサンプル（console, canvas）
- MIRにExternCallが追加され、WASMバックエンドでimport呼び出しが生成
- ブラウザで「直接WASM出力」→Canvas描画/Console出力が成功
- 既存PlaygroundのAPIをABI経由でも再現

Timeline (tentative):
- Week 1: ABI/BID仕様初版＋MIR ExternCall雛形
- Week 2: WASM RuntimeImports最小（console, canvas）＋E2Eデモ
- Week 3: MIR verifier効果整合＋WASMコード生成の安定化
- Week 4+: 他ライブラリBID追加、言語ごとのFFIコード生成の着手

Risks/Mitigations:
- 文字列/メモリ規約の複雑化 → 最初はUTF-8固定としptr/lenを明記、ヘルパ関数を用意
- 効果システムとの不整合 → BIDに効果を必須化しVerifierに反映
- 二重系（旧 wasm-bindgen 経路）との分岐 → 段階的にABI側へ置換、テスト共通化

Notes:
- 「全部をBox化」の中心はBID。RuntimeImportsはBIDのWASM実装に過ぎない。
- 全ライブラリ待ちではなく、Canvas/Consoleの最小実装で設計検証して拡張する。

------------------------------------------------------------

## 🔭 Phase 9.7: Box FFI/ABI基盤 + MIR ExternCall 追加（RuntimeImports対応）

Summary:
- 「あらゆるライブラリを箱に」を実現するための共通ABI（Box FFI/ABI）とBID（Box Interface Definition）を策定。
- MIRに `ExternCall` 命令を導入し、WASM/VM/言語出力で外部Box APIを一貫呼び出しできる基盤を整える。
- WASM向けには RuntimeImports をABI準拠で拡張（console/canvasの最小セットから）。

Why:
- nyashがRustに依存せずnyasuをビルドできるように（実行時は純WASM＋ホストimport）。
- MIR→任意言語出力時に、外部ライブラリ（=Box API）をBID→各言語FFIへ写像しやすくする。
- Everything is BoxのAPI設計を、外部ライブラリにも一貫適用。

Scope/Tasks:
- [ ] 1) ABI/BIDの策定（最優先）
  - 型: i32/i64/f32/f64/string(ptr,len)/bool/boxref/array、null/void
  - 呼出規約: 名前解決（namespace.box.method）、エラー/例外、同期/非同期
  - 効果: pure/mut/io/control（MIR効果と整合）
  - 成果物: `docs/box_ffi_abi.md`、BIDフォーマット（YAML/JSON）サンプル（console/canvas）
- [ ] 2) MIR拡張: `ExternCall`
  - 命令: `ExternCall(dst, iface_name, method_name, args[])`
  - Verifier: BIDと型/効果の整合、最適化（pure再順序化/ mut依存）維持
  - Lowering: 既存のBoxCall/Field opsとの橋渡し、静的解決と動的解決の方針メモ
- [ ] 3) WASM RuntimeImports（ABI準拠）
  - import群: `env.console.log(ptr,len)`, `env.canvas.fillRect(...)`, `env.canvas.fillText(...)` 等（stringはUTF-8でptr/len）
  - Host(JS): importObject実装（DOM解決→Canvas/Console呼び出し）
- [ ] 4) コード生成: ExternCall→バックエンド写像
  - WASM: import呼び出し（線形メモリから文字列復元）
  - VM: ホスト関数テーブルでスタブ実装（最小はログ/ダミー）
  - 言語出力: TypeScript/Python/Rust等へのFFI雛形（後続）
- [ ] 5) E2Eデモ
  - Nyashコード→MIR（ExternCall含む）→WASM生成→ブラウザでcanvas/console動作を確認

Acceptance Criteria:
- `docs/box_ffi_abi.md` 初版＋BIDサンプル（console/canvas）が存在
- MIRに `ExternCall` が追加され、WASMコード生成で import 呼び出しが生成される
- ブラウザで「直接WASM出力」→Canvas描画/Console出力が成功
- 既存PlaygroundAPIとABI経由APIが概ね一致（互換性の方向性明確）

Timeline (tentative):
- Week 1: ABI/BID仕様初版＋MIR `ExternCall` 雛形
- Week 2: WASM RuntimeImports最小（console/canvas）＋E2Eデモ
- Week 3: Verifier効果整合＋WASMコード生成の安定化
- Week 4+: 他ライブラリBID追加、言語ごとのFFIコード生成の着手

Risks/Mitigations:
- 文字列/メモリ規約の複雑化 → UTF-8固定＋ptr/len規約、ヘルパ導入
- 効果システムとの不整合 → BIDに効果を必須化しVerifierに反映
- 旧 wasm-bindgen 経路との二重系 → 段階的にABI側へ置換、テスト共通化

Order with Phase 10:
- Phase 9.7 を Phase 10 の前に実施するのが妥当（外部API基盤はAOT/JIT等の前提）。

------------------------------------------------------------

📚 Phase Index + Links（整理用）
- Phase 0–7: 基本MIR/VM/例外/非同期 既存記述の通り
- Phase 8.x: MIR→WASM（PoC→Lowering→25命令）既存記述の通り
  - 8.4 完了報告あり
  - 8.5 25命令仕様: docs/予定/native-plan/issues/phase_8_5_mir_25_instruction_specification.md
  - 8.6/8.7 既存記述の通り
- Phase 9.x: AOT/WASM ＋ ランタイム基盤
  - 9.7 ABI/BID + ExternCall: docs/予定/native-plan/issues/phase_9_7_box_ffi_abi_and_externcall.md
  - ABIドラフト: docs/予定/native-plan/box_ffi_abi.md
  - Issue 62 前提（WASM文字列定数）: docs/予定/native-plan/issues/issue_62_update_proposal.md
- Phase 10: LLVM Backend Skeleton（MIR→LLVM IR）
  - docs/予定/native-plan/issues/phase_10_x_llvm_backend_skeleton.md

------------------------------------------------------------

## 📦 Phase 9.8: BIDレジストリ + 自動コード生成ツール（WASM/VM/LLVM/言語）

Summary:
- 外部ライブラリをBoxとして配布・発見・利用するためのBIDレジストリと、BID→各ターゲットのスタブ生成（import/extern宣言）を自動化。

Scope/Tasks:
- BIDレジストリ仕様（署名・効果・バージョン・依存関係）
- 生成: WASM(importObject), VM(関数テーブル), LLVM(declare), TS/Python(RTEラッパ)
- CLI: `nyash bid gen --target wasm|vm|llvm|ts|py bid.yaml`

Acceptance:
- console/canvasのBIDから各ターゲットの骨子が自動生成される

------------------------------------------------------------

## 🔒 Phase 9.9: ExternCall 権限/ケイパビリティモデル（Sandbox/Allowlist）

Summary:
- 外部API呼び出しの安全化。BIDに必要権限を宣言し、ホスト側で許可/拒否。WASMはimport allowlist、VM/LLVMは関数テーブルで制御。

Scope/Tasks:
- 権限種別（console, canvas, storage, net, audio...）とポリシー
- 実行時プロンプト/設定ファイル/環境変数での許可
- 失権時の挙動（明示エラー）

Acceptance:
- 禁止権限のExternCallが実行時にブロックされ、明確なエラーが返る

------------------------------------------------------------

## 🧰 Phase 10.1: LLVM 外部関数マッピング方針（プラットフォーム抽象）

Summary:
- ExternCallのFQN→ネイティブ関数（printf等）への写像レイヤーと、OS差の抽象。初手はLinux/clang、他OSは後続。

Scope/Tasks:
- env.console.log → printf("%.*s",len,ptr) テンプレート
- プラットフォーム切替（feature）とリンク方針

Acceptance:
- 代表ExternCall（console.log）がAOTバイナリで出力可能

------------------------------------------------------------

## 📦 Phase 11: Boxライブラリ配布（パッケージ）＋バージョニング

Summary:
- Boxパッケージ（BID＋実装）の配布形式、互換性ルール、バージョン解決を定義。

Scope/Tasks:
- パッケージメタ（name, version, engines, permissions）
- 依存解決・衝突ルール（SemVer）
- 署名/検証（将来）

Acceptance:
- ローカルBIDパッケージを参照してExternCallが解決可能

------------------------------------------------------------

## 🧪 Phase 12: LSP/静的解析（init/fini/weak/委譲 ルール検証）

Summary:
- ChatGPT5提案の静的検証をLSPで提供。未宣言代入・weak循環・委譲競合を早期検出。

Scope/Tasks:
- ルール実装（init/weak/再代入fini/多重デリゲーション競合）
- VSCode拡張の最低限配布

Acceptance:
- 代表ケースで警告/自動修正候補が出る

------------------------------------------------------------

## 📊 Phase 13: テレメトリ/診断（Debug level＋Playgroundメトリクス）

Summary:
- 実行時ノイズ抑制と観測性強化。debug level、fini/weak/時間のメトリクス表示をPlayground/CLIで提供。

Scope/Tasks:
- DebugBox/環境変数でログレベル切替
- Playgroundにメータ（fini総数、weak失敗数、実行時間）

Acceptance:
- 代表シナリオでメトリクスが可視化される

------------------------------------------------------------

## 🧱 Phase 9.10: NyIR（公開IR）仕様化 + フォーマット + 検証器

Summary:
- 25命令MIRを公開IR（NyIR v1）として凍結。バージョニング、バイナリ`.nybc`/テキスト`.nyir`、厳格検証器を用意。

Scope/Tasks:
- docs/nyir/spec.md（命令の意味論/効果/検証/未定義なしの宣言）
- nyir-parser/nyir-serializer（.nyir/.nybc）
- Verifier: 所有森/weak/効果/Bus整合
- ツール: `nyashel -S`（Nyash→NyIRダンプ）, `nyir-run`（インタプリタ実行）

References:
- docs/nyir/spec.md（骨子）
- docs/nyir/phase_9_10_nyir_spec.md

Acceptance:
- 代表サンプルがNyIRで保存・検証・実行可能

------------------------------------------------------------

## 🧪 Phase 9.11: Golden NyIR + Differential 実行テスト（CI）

Summary:
- NyIRダンプをゴールデンとし、interp/vm/wasm/jitの出力一致をCIで検証。弱失効/分割finiなど境界条件も含む。

Scope/Tasks:
- golden/*.nyir の整備
- CIで各バックエンド実行→結果一致チェック

Acceptance:
- 主要サンプルで全バックエンド一致（差分検出時は原因特定に役立つログ）

------------------------------------------------------------

## 🧩 Phase 10.2: Host API層（C-ABI `ny_host_*` / WASM `nyir_host`）

Summary:
- Rust依存を薄い宿主APIへ集約。C-ABI公開（ファイル/メモリ/時間等）、WASMは`nyir_host` importで提供。

Scope/Tasks:
- `ny_host_*`関数群（read_file/free/clockなど）をC-ABIで実装
- Nyash側extern宣言と所有移管`*_from_raw`/`*_into_raw`
- WASM: import `nyir_host` 名前空間で最低限の関数提供

Acceptance:
- 代表I/OがHost API経由で動作し、Rust実装置換が容易

------------------------------------------------------------

## 🧱 Phase 10.3: ランタイム層の切り分け（corelang/rt/sys/std）

Summary:
- corelang（純Nyash）, rt（Box ABI/所有/weak/Safepoint/Bus）, sys（プラットフォーム）, std（Nyash実装）に整理。

Scope/Tasks:
- ドキュメント化＋最小コードの配置替えスケルトン

Acceptance:
- 層構造が明文化され、新規実装がガイドに従って進められる

------------------------------------------------------------

## 🧬 Phase 10.4: Box ABI（fat ptr）とLLVM属性（Effects）

Summary:
- Boxのfat pointer（data*, typeid, flags）の定義、Weakの世代タグ、SafepointのLLVM降ろし、Effect→LLVM属性（readonly/readnone等）。

Scope/Tasks:
- LLVM IR側のstruct宣言・属性付与の雛形

Acceptance:
- 代表関数で属性が付与され、最適化に寄与（noalias/argmemonly等は可能な範囲で）

------------------------------------------------------------

## 📚 Phase 10.5: コア標準（String/Array/Map）Nyash実装（Rust依存の段階的削減）

Summary:
- 現在Rust実装に依存している基本コンテナ（String/Array/Map）を、rt/sys層を活用してNyash実装に置換。セルフホストへの橋渡し。

Scope/Tasks:
- rt: 最低限のアロケータ/Box ABI/所有/weakを利用
- sys: `ny_host_*`（alloc/free/memcpy等）を経由
- std: Nyashで String/Array/Map の最小機能を実装（append/push/index/len など）
- 互換: 既存の言語表面の挙動に合わせる（差異は仕様で宣言）

Acceptance:
- 代表サンプルでString/Array/MapがNyash実装で動作し、Rust実装をリンクせずに通る
References:
- docs/予定/native-plan/issues/phase_10_5_core_std_nyash_impl.md

------------------------------------------------------------

## 🌀 Phase 14: セルフホスト・ロードマップ（Stage 0→3）

Summary:
- nyashc0（Rust）→nyashc1（Nyash+NyIR+LLVM）→自己再ビルド→Rust依存をsys層みに縮退。

Scope/Tasks:
- Stage 0: 既存実装でNyIR→LLVMまでの線を通す
- Stage 1: Nyashでコンパイラ本体（フロント）を書きNyIR出力→LLVMでネイティブ化
- Stage 2: 自己再ビルドの一致検証
- Stage 3: 標準をNyash実装へ移行、Rustはsysのみ

Acceptance:
- セルフホスト一周の実証（機能一致/ハッシュ近似）


------------------------------------------------------------

🧳 Parking Lot（要整合/後方貼り付け）
- WASM StringBox/文字列定数の取り扱いが古い記述と混在 → 最新は Issue 62 提案に従う（data segment + (ptr,len)）
- 8.4/8.5 の重複・表現ゆらぎ → 25命令仕様に一本化。古いintrinsic案は参考として保持
- wasm-bindgen経路と直接WASM経路の混在 → 当面は併存、ABI経路へ漸進的統合（9.7参照）

備考（運用）
- 本ファイルはフェーズ順の索引と整合メモを優先。詳細仕様は issues/ 配下の各mdに集約。
- 不整合な古い計画は Parking Lot に追い出し、段階的に整理・統合する。
