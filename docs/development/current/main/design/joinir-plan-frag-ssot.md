# JoinIR Plan/Frag SSOT Documentation

**Status**: Active (2025-12-25)
**Phase**: Phase 286 P0 (docs-only)
**Purpose**: Single Source of Truth for Plan/Frag responsibilities, prohibitions, and freeze points

---

## 目標

JoinIR line を Plan/Frag に吸収する前提で、**責務・禁止事項・凍結点（freeze points）**をSSOTとして1ページに固定する。

「2本コンパイラ根治（VM/LLVM）」の合流点が明文化されていること。

---

## 1. Scope / Non-goals

### 対象（Scope）
- JoinIR line（意味ルート分類。Pattern1-9 は traceability-only legacy labels）における Plan → Frag → MIR merge の限定されたパイプライン
- Plan と Frag の境界における責務分離
- VM/LLVM 両コンパイラでの共通契約

### 対象外（Non-goals）
- MIR実行・VM/LLVM固有の最適化
- JoinIR外のlowering（AST → MIR 全体ではなく、限定されたパイプラインのみ）

---

## 2. 用語（Terms）

| 用語 | 定義 |
|------|------|
| **JoinIR line** | (Route classification/Plan; legacy labels are traceability-only) → (Frag+Boundary) → (MIR merge) の限定されたパイプライン |
| **Plan** | ルート意味抽出・配線準備フェーズ（current runtime: Facts → Recipe → Verifier → CorePlan/Frag） |
| **Frag** | フラグメント生成・エミットフェーズ（JoinModule + JoinFragmentMeta） |
| **Boundary** | JoinInlineBoundary（host↔JoinIR 変数マッピング） |
| **ExitKind** | 継続の種類（Return/Break/Continue/Kont） |
| **Freeze point** | 以降変更が禁止される確定ポイント |
| **SSOT** | Single Source of Truth（唯一の情報源） |

**注記**: JoinIR line は AST → MIR 全体ではなく、上記の限定された範囲を指す（JoinIR外のloweringは含まない）。

---

## 3. 責務（Responsibilities）

### Planが決めること

Plan 段階では以下を決定する：

- **ルート識別**: どの制御意味ルートか（legacy labels are traceability-only）
- **制御フロー構造**: ループ/if/try/scan の構造
- **キャリア変数**: ループ間で保持される変数
- **継続構造**: k_exit / k_continue の境界
- **ValueId 割り当て**: JoinIRローカルのValueId（host ValueId と衝突しない領域）

### Planが決めないこと

Plan 段階では以下を決定しない（Frag側の責務）：

- **ホスト変数**: host inputs の ValueId はFrag側で決定
- **最終的な命令列**: Copy命令の注入はFrag側
- **PHI構造**: header/exit PHIの最終構造はFrag側

### Fragが保持すること

Frag 段階では以下を保持する：

- **JoinInlineBoundary**: host↔JoinIR の全マッピング情報
- **JoinFragmentMeta**: expr_result / exit_meta / continuation_funcs
- **terminator SSOT**: emit_frag() を唯一のterminator生成ポイント

### Fragが保持しないこと

Frag 段階では以下を保持しない（Plan側の責務）：

- **ルート分類知識**: legacy label の違いを知らない（CorePlanのみ処理）
- **AST情報**: AST構造には直接アクセスしない

---

## 4. 禁止事項（Prohibitions - 最重要）

| 禁止事項 | 理由 |
|----------|------|
| **Planでの実行** | Planは純粋なデータ構造 |
| **Planでの名前解決** | 名前解決はFrag側 |
| **Planでの最適化** | Planは構造のみ記述 |
| **Planでのルール実装** | ルールはFrag側で実装 |
| **Fragでのルート識別** | ルート分類知識はPlan側（legacy pattern labels are traceability-only） |
| **明示的でないValueIdマッピング** | 全てBoundary経由 |
| ** terminator の多様な生成ポイント** | emit_frag() SSOT |
| **freeze point 以降の変更** | 不変条件違反 |

### 診断専用の扱い

開発中のデバッグ出力・トレースログは、debugタグ付き（既定OFF）でのみ許可。

### 実装注記（ValueId領域）

現状の実装では JoinIR-local ValueId として 100-999 領域を使用しているが、これは実装詳細であり変更されうる。SSOT としての原則は「host ValueId と衝突しない領域を使う」である。

---

## 5. 凍結点（Freeze Points）

| Stage | 凍結されるもの | 以降禁止される操作 |
|-------|----------------|-------------------|
| **PlanFreeze** (V1-V9) | CorePlanの構造 | Plan構造の変更 |
| **BoundaryFreeze** | JoinInlineBoundaryの全フィールド | Boundaryマッピングの変更 |
| **ValueIdAllocate** | JoinIRローカルValueId（hostと非衝突領域） | 領域の再割り当て |
| **MergeComplete** | MIR block構造 | CFGの変更 |

---

## 6. 不変条件（Invariants / Fail-Fast）

### Plan段階の不変条件

- **V1**: 条件のValueIdは有効（pre-generated）
- **V2**: Exitの妥当性（Returnは関数内、Break/Continueはループ内）
- **V3**: Seqは非空
- **V4**: Ifのthen_plansは非空
- **V5**: ループはキャリアを1つ以上持つ
- **V6**: Frag entryはheader_bbを指す
- **V7**: block_effectsにheader_bbが含まれる
- **V10**: body_bbのeffectsはloop_plan.bodyに積む（block_effects[body_bb]は空でなければならない）
  - Phase 286 P2.7 追加: lowererはloop_plan.bodyをbody_bbにemitし、block_effectsのbody_bbは無視する

### Boundary段階の不変条件

- **B1**: join_inputsのValueIdはhost ValueIdと衝突しない領域^1
  - JoinIR lowering では `alloc_join_param()` を使用すること（再発防止）
- **B2**: exit_bindingsは対応するexit PHIを持つ
- **C1**: 同じjoin_valueは同じhost_valueにマッピング
- **C2**: condition bindingのjoin_valueはhost ValueIdと衝突しない領域^1
  - ConditionEnv 経由で割り当てられる

---
^1) 現状の実装では 100-999 の範囲を使用しているが、これは実装詳細であり将来の実装では変わりうる。

### Merge段階の不変条件（debug_assertions）

- **M1**: Header PHI dstは再定義されない
- **M2**: Exit PHI inputsは全て定義されている
- **M3**: terminator targetsは全て存在する
- **M4**: ValueIdは領域を守っている

### 破ったらどこで落とすか

- **Contract違反**: `contract_checks.rs` で早期return
- **Debug assert**: `debug_assert!` / `unreachable!` で即座に落とす
- **検証漏れ**: CIで `cfg(debug_assertions)` テストを実行

---

## 7. 2本コンパイラ根治の合流点

### 共通パス（Shared Path）

```
AST → JoinIR Plan → JoinIR Frag → MIR Merge
```

共通パスでは以下が共有される：

- **Route Classification**: 同じルート分類アルゴリズム（legacy pattern labels are traceability-only）
- **Boundary Construction**: 同じ JoinInlineBoundary 構造
- **MIR Merge**: 同じ merge_joinir_mir_blocks() ロジック

### 分岐点（Divergence Point）

```
MIR → [VM: MirInterpreter]
    → [LLVM: llvmlite/inkwell]
```

### 差分が許される場所

- **Method ID injection**: LLVM側のみ
- **Experimental features**: NYASH_JOINIR_VM_BRIDGE vs NYASH_JOINIR_LLVM_EXPERIMENT
- **Execution engines**: MirInterpreter vs llvmlite

### 差分が許されない場所

- **JoinIR structure**: 同じ構造でなければならない
- **PHI nodes**: 同じPHI構造でなければならない
- **ValueId mappings**: 同じマッピングでなければならない

---

## 8. デバッグ導線

詳細は CLAUDE.md へのリンク（重複させない）:

- **NYASH_CLI_VERBOSE=1**: 一般的な詳細ログ
- **HAKO_JOINIR_DEBUG=1**: JoinIR ルーティング・ブロック割り当て
- **NYASH_TRACE_VARMAP=1**: variable_map トレース（PHI接続デバッグ）
- **JoinIR architecture**: docs/development/current/main/joinir-architecture-overview.md

---

## 9. 関連ドキュメント（SSOT）

- Post-PHI final form: `docs/development/current/main/design/post-phi-final-form-ssot.md`

---

## 関連ドキュメント

- **JoinIR Architecture Overview**: [joinir-architecture-overview.md](./joinir-architecture-overview.md) - 全体アーキテクチャのSSOT
- **JoinIR Design Map**: [joinir-design-map.md](./joinir-design-map.md) - 実装導線の地図
- **Phase 286 README**: [../phases/phase-286/README.md](../phases/phase-286/README.md) - フェーズ進捗
- Post-PHI final form: `docs/development/current/main/design/post-phi-final-form-ssot.md`
- Effect classification: `docs/development/current/main/design/effect-classification-ssot.md`

---

## 変更ログ

- **2025-12-26**: Phase 286 P2.7 - V10追加（body_bb effects契約の明文化）
- **2025-12-25**: Phase 286 P0 - 初版作成（docs-only）
