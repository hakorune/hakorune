# LoopForm SSOT（単一起点）設計ノート

目的
- ループのPHI整形・前処理（preheader Copy、header PHI seed、latch/continue 合流）を単一モジュールに集約してドリフトを防ぐ。
- ビルダー（Direct MIR）とブリッジ（JSON v0 → MIR）の双方で同一の規約と検証を通す。

SSOT（Single Source of Truth）
- 中心: `src/mir/phi_core/loop_phi.rs`
  - 型: `IncompletePhi`, `VarSnapshot`
  - API: `prepare_loop_variables_with`, `seal_incomplete_phis_with`, `build_exit_phis_with`
  - デバッグ: `phi_core::common::debug_verify_phi_inputs`（到達検証・重複検出）
- Direct MIR（既にSSOT使用）
  - `src/mir/loop_builder.rs` が `LoopPhiOps` を実装し、`prepare/seal/exit` を phi_core へ委譲。
  - 形状: `preheader → header(φ) → body → latch → header|exit`（LoopForm準拠）。
- JSON v0 Bridge（段階移行→完了済みの範囲）
  - header PHI（seed/完成）・exit PHI を `LoopPhiOps` アダプタ経由で SSOT API に委譲。
  - break/continue スナップショットは thread‑local stack で収集し、seal/build_exit に渡す。
  - 代表 parity カナリア（opt‑in）で Direct と Bridge の一致を検証。

規約（不変条件）
- header の PHI 入力は「preheader 経由の定義済み値」と「latch/continue からの値」だけ。
- preheader で Copy を先行挿入し、PHI 入力は Copy の出力を参照する（Use-Before-Def回避）。
- 1 predecessor なら直接 bind（PHI省略）、2つ以上で PHI を生成。
- 検証は Fail‑Fast ではなく開発時 WARN（`debug_assert`）だが、将来 Core 側で整形に移管予定。

4箱構成（Phase 26-F 系の整理）
- **LoopVarClassBox**（`loop_var_classifier.rs`）
  - 変数のスコープ分類専用箱だよ。Pinned / Carrier / BodyLocalExit / BodyLocalInternal を決めるだけで、PHI 発行はしない。
- **LoopExitLivenessBox**（`loop_exit_liveness.rs`）
  - ループ `exit` 直後で「実際に使われる可能性がある変数」を集める箱だよ。Phase 26-F 時点では `live_at_exit` は保守的近似で、将来 `get_block_instructions()` などを使った MIR スキャンに差し替える予定。
  - `ExitLivenessProvider` を実装していて、`ExitPhiBuilder` は Box<dyn ExitLivenessProvider> を受け取る形にしたので、Legacy（既定）と MirScan 版の差し替えがそのまま出来る。
  - 環境変数 `NYASH_EXIT_LIVE_ENABLE=1` で将来の実装を段階的に有効化、`NYASH_EXIT_LIVENESS_TRACE=1` でトレースを出せるようにしてある。
- **BodyLocalPhiBuilder**（`body_local_phi_builder.rs`）
  - 上の 2つの箱の結果を統合する決定箱だよ。
    - `class.needs_exit_phi()` が true（Pinned / Carrier / BodyLocalExit）のものは従来どおり exit PHI 候補。
    - それ以外でも、`BodyLocalInternal` かつ `live_at_exit` に含まれ、かつ `is_available_in_all` で全 pred 定義が確認できるものだけを安全側で救済候補にできるようにしてある（この救済ロジック自体は `NYASH_EXIT_LIVE_ENABLE` でガード）。
- **PhiInvariantsBox**（`phi_invariants.rs`）
  - 最後に「全 pred で定義されているか」「不正な incoming が無いか」を Fail-Fast でチェックする箱だよ。ここで落ちる場合は LoopForm/BodyLocal 側の構造バグとみなしている。

今後の移行
- Bridge 側に `LoopPhiOps` 実装を追加し、`prepare/seal/exit` を直接呼ぶ。
- ループ形状の生成をユーティリティ化（builder/bridge 双方から共通呼び出し）。
- ExitLivenessProvider は 26-G 以降で MIR 命令列スキャン版に差し替える予定（現状の MirScanExitLiveness は header/exit_snapshots の union 近似）。
- Header/Exit φ については、Phase 26-H / 27.x で導入した JoinIR（関数正規化IR）側の `LoopHeaderShape` / `LoopExitShape` と `loop_step` / `k_exit` 引数に最終的に吸収し、Rust 側では LoopVarClass / LoopExitLiveness / BodyLocalPhi / PhiInvariants による前処理＋検証だけを残す方針。

---

## LoopForm v2 ケース表

| Case | loop 条件形 | exit preds の構成 | 想定される PHI 入力の形 | 対応テスト | 対応 .hako |
|------|-----------|-----------------|---------------------|----------|-----------|
| **A** | `loop(i < n)` | header / body | header fallthrough + break | `loop_conditional_reassign_exit_phi_header_and_break` | - |
| **B** | `loop(1 == 1)` | body のみ | break のみ | `loop_constant_true_exit_phi_dominates` | `apps/tests/minimal_ssa_skip_ws.hako` |
| **C** | `loop(1 == 1)` + body-local | body のみ（一部の経路のみ定義） | break のみ（BodyLocalInternal は除外） | `loop_body_local_exit_phi_body_only` | - |
| **D** | `loop(i < n)` + continue | header / body / continue_merge | header + break + continue_merge | `loop_continue_merge_header_exit` | - |

### ケース説明

#### Case A: header+break（標準パターン）
- **条件**: `loop(i < n)` のような動的条件
- **特徴**: header→exit と body→exit の両方が CFG 上存在
- **exit PHI**: header fallthrough + break 経路の両方を含む
- **検証**: `loop_conditional_reassign_exit_phi_header_and_break`

#### Case B: constant-true+break-only（header 除外パターン）
- **条件**: `loop(1 == 1)` のような定数 true
- **特徴**: exit pred は break 経路のみ（`header→exit` は無い）
- **exit PHI**: break 経路のみ（header は CFG predecessor でないため除外）
- **検証**: `loop_constant_true_exit_phi_dominates` + `minimal_ssa_skip_ws.hako`

#### Case C: body-local変数（BodyLocalInternal 除外パターン）
- **条件**: body 内で宣言された変数が一部の exit 経路でのみ定義される
- **特徴**: 変数が全 exit predecessors で定義されていない
- **exit PHI**: BodyLocalInternal 変数は除外（PHI 生成しない）
- **検証**: `loop_body_local_exit_phi_body_only`

#### Case D: continue+break（continue_merge パターン）
- **条件**: continue 文を含むループ
- **特徴**: `continue_merge → header → exit` の経路あり
- **exit PHI**: header + break + continue_merge の 3 系統
- **検証**: `loop_continue_merge_header_exit`

### 実装ファイル

| ファイル | 役割 |
|---------|-----|
| `src/mir/loop_builder.rs` | ループ構造生成・[LoopForm] コメント付き |
| `src/mir/phi_core/loop_snapshot_merge.rs` | Case A/B 分岐ロジック |
| `src/mir/phi_core/exit_phi_builder.rs` | Exit PHI 生成・Phantom block 除外 |
| `src/tests/mir_loopform_conditional_reassign.rs` | 4 ケース全てのテスト（[LoopForm-Test] タグ付き） |

---

## Phase 26-H 以降: PHI/Loop 箱 → JoinIR 移行対応表

JoinIR（関数正規化IR）の導入により、φ ノードが関数引数に、merge ブロックが join 関数に変換される。各箱の将来的な扱いを整理する。

| 現在の箱 | ファイル | Phase | 将来的な扱い | 理由 |
|---------|---------|-------|------------|------|
| **HeaderPhiBuilder** | `header_phi_builder.rs` | 26-C | 🔄 JoinIR に吸収 | header φ → `loop_step` 関数の引数に変換 |
| **ExitPhiBuilder** | `exit_phi_builder.rs` | 26-D | 🔄 JoinIR に吸収 | exit φ → `k_exit` 継続の引数に変換 |
| **LoopSnapshotManager** | `loop_snapshot_manager.rs` | 26-C | 🔄 JoinIR に吸収 | スナップショット → 関数呼び出し時の引数に統合 |
| **LoopSnapshotMerge** | `loop_snapshot_merge.rs` | 26-C | 🔄 JoinIR に吸収 | スナップショット合流 → 関数呼び出しで自然に表現 |
| **PhiInputCollector** | `phi_input_collector.rs` | 26-B | 🔄 JoinIR に吸収 | φ 入力収集 → 関数引数決定ロジックに統合 |
| **IfBodyLocalMerge** | `if_body_local_merge.rs` | 26-F-2 | 🔄 JoinIR に吸収 | if merge φ → join 関数の引数に変換 |
| **PhiBuilderBox** | `phi_builder_box.rs` | 26-E | ❌ 削除候補 | 統一箱は JoinIR 生成で不要に |
| **IfPhi** | `if_phi.rs` | Phase 1 | ❌ 削除候補 | if φ → join 関数に完全統合 |
| **LoopPhi** | `loop_phi.rs` | Phase 1 | ❌ 削除候補 | loop φ → 関数引数に完全統合 |
| **LoopFormBuilder** | `loopform_builder.rs` | Phase 1 | ❌ 削除候補 | JoinIR 生成ロジックに置き換え |
| **LoopVarClassifier** | `loop_var_classifier.rs` | Option C | ✅ LoopForm 前段として残す | 変数分類は MIR → JoinIR 変換の前処理として必要 |
| **LoopExitLiveness** | `loop_exit_liveness.rs` | 26-F-4 | ✅ LoopForm 前段として残す | exit 後使用変数は `k_exit` 引数決定に必要 |
| **BodyLocalPhiBuilder** | `body_local_phi_builder.rs` | 26-B | ✅ LoopForm 前段として残す | 前処理段階での統合決定ロジック |
| **LocalScopeInspector** | `local_scope_inspector.rs` | Option C | ✅ LoopForm 前段として残す | スコープ分析は前処理として必要 |
| **PhiInvariants** | `phi_invariants.rs` | 26-F-3 | ✅ 検証用として残す | JoinIR 変換前後の検証に有用 |
| **Common** | `common.rs` | Phase 1 | ✅ ユーティリティとして残す | 共通ヘルパー関数群 |
| **Conservative** | `conservative.rs` | Phase 1 | ✅ フォールバックとして残す | 保守的 PHI 生成（レガシー互換） |

### 記号説明
- 🔄 **JoinIR に吸収**: φ ノード → 関数引数変換により不要になる
- ❌ **削除候補**: JoinIR 生成ロジックで完全に置き換えられる
- ✅ **残す**: MIR → JoinIR 変換の前処理／検証として必要

### 移行戦略（Phase 27 以降）
1. **Phase 27**: 一般化 MIR → JoinIR 変換実装
   - `lower_min_loop_to_joinir` の一般化
   - LoopVarClassifier / LoopExitLiveness を前処理として利用
2. **Phase 28**: JoinIR 実行器実装（VM / LLVM）
   - 関数呼び出し／継続ベースの実行
3. **Phase 29**: レガシー PHI 箱の段階的削除
   - HeaderPhiBuilder → JoinIR 引数生成に置き換え
   - ExitPhiBuilder → JoinIR 継続引数に置き換え
   - PhiBuilderBox / IfPhi / LoopPhi の完全削除

---

関連
- `src/mir/loop_builder.rs`
- `src/runner/json_v0_bridge/lowering/loop_.rs`
- `src/mir/phi_core/common.rs`
- `src/mir/phi_core/loop_snapshot_merge.rs`
- `src/mir/phi_core/exit_phi_builder.rs`
- `src/tests/mir_loopform_conditional_reassign.rs`
- **Phase 26-H**: `src/mir/join_ir.rs` - JoinIR 型定義・自動変換実装
