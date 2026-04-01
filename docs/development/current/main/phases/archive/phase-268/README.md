# Phase 268: if_form.rs への Frag 適用 + Entry Edge-args SSOT化

Status: ✅ 完了（P0 + P1）
Date: 2025-12-21

## 目的

**EdgeCFG Fragment を "層を跨がずに" 実戦投入**

- **P0**: `if_form.rs` の emit_conditional + emit_jump を Frag+emit_frag に置換（1箇所のみ）
- **P1**: compose::if_() の then/else entry edge-args を SSOT 化（TODO 解消）

## 実装結果

### P0: 最小適用（emission 層経由）

**アーキテクチャ戦略**:
- `if_form.rs` に直接 Frag 構築コードを書かず、`emission/branch.rs` に薄い入口関数 `emit_conditional_edgecfg()` を追加
- **理由**: 層が綺麗（Frag 構築は emission 層に閉じる）、差分が小さい（if_form.rs は API 呼び出し差し替えのみ）、デバッグ容易（層境界が明確）

**アーキテクチャ図**:
```
if_form.rs (MirBuilder 層)
  ↓ 呼び出し
emission/branch.rs::emit_conditional_edgecfg() (emission 層: 薄ラッパー)
  ↓ 内部で使用
Frag 構築 + compose::if_() + emit_frag() (EdgeCFG Fragment API)
  ↓ 最終的に呼び出し
set_branch_with_edge_args() / set_jump_with_edge_args() (Phase 260 SSOT)
```

**変更内容**:

1. **emission/branch.rs** に `emit_conditional_edgecfg()` 関数追加（~50行）
   - 責務: Frag 構築 + compose::if_() + emit_frag() の薄いラッパー
   - Then/Else Frag 構築（then_exit_block/else_exit_block から Normal exit 作成）
   - Join Frag 構築
   - compose::if_() で合成
   - emit_frag() で MIR terminator に変換

2. **if_form.rs** API 呼び出し差し替え
   - Lines 109-114: 削除（emit_conditional 呼び出し）
   - Line 147: 削除（emit_jump 呼び出し）
   - Line 202: 削除（emit_jump 呼び出し）
   - Line 206: 追加（emit_conditional_edgecfg 呼び出し、~10行）
   - **差分**: 削除3箇所 + 追加1箇所のみ（層が綺麗）

**テスト結果**:
- ✅ cargo build --release: 成功
- ✅ cargo test --lib --release: 1395 tests PASS
- ✅ quick smoke: 45/46 PASS（既存状態維持）

### P1: compose::if_() Entry Edge-args SSOT化

**目的**: compose::if_() 内部で then/else entry edge-args を "勝手に空 Vec で生成" しない → 呼び出し側が明示的に渡す（SSOT 原則）

**変更内容**:

1. **compose.rs** (Lines 106-127): compose::if_() シグネチャ変更
   - Before: `if_(header, cond, t, e, join_frag)`
   - After: `if_(header, cond, t, then_entry_args, e, else_entry_args, join_frag)`
   - TODO コメント削除完了（Phase 267 P2+ TODO 解消）

2. **emission/branch.rs** (Lines 110-125): emit_conditional_edgecfg() から空 EdgeArgs を渡す
   ```rust
   compose::if_(
       pre_branch_bb,
       condition_val,
       then_frag,
       EdgeArgs { layout: CarriersOnly, values: vec![] },  // then entry args
       else_frag,
       EdgeArgs { layout: CarriersOnly, values: vec![] },  // else entry args
       join_frag,
   )
   ```

3. **テスト更新**（3箇所）:
   - compose.rs: Lines 638-652, 720-734（2箇所）
   - emit.rs: Lines 555-569（1箇所）
   - 全て新シグネチャに更新、空 EdgeArgs を渡す

**テスト結果**:
- ✅ cargo build --release: 成功（0エラー）
- ✅ cargo test --lib --release: **1444/1444 PASS**
- ✅ quick smoke: **45/46 PASS**（既存状態維持）

## 核心的な設計判断

### P0: なぜ emission 層経由か

1. **層が綺麗**: Frag 構築ロジックを emission 層に閉じ込め、MirBuilder 層から分離
2. **差分が小さい**: if_form.rs は API 呼び出し差し替えのみ（3箇所削除 + 1箇所追加）
3. **デバッグ容易**: 層境界が明確で問題切り分けが簡単
4. **拡張性**: 将来他の箇所（loop_form.rs 等）も同じパターンで統一可能

### P1: なぜ SSOT 化か

1. **推測禁止**: compose::if_() 内部で then/else entry edge-args を "勝手に空 Vec で生成" しない
2. **呼び出し側 SSOT**: P0 で emission/branch.rs に薄いラッパーを作ったので、edge-args も同じ層で SSOT として渡す
3. **P0 との整合性**: Frag 構築と edge-args 提供を同じ層（emission）に集約

## 重要な発見

### Frag "from" ブロックの厳密性

- then_exit_block/else_exit_block は「実際に merge へ飛ぶブロック」と一致必須
- ✅ 正しい: if_form.rs Line 141, 197 で取得済みの値を使用
- ❌ 誤り: "それっぽい" ブロックから Normal exit を作成（ズレる）

### JoinIR Fallback との非交差

- JoinIR 経路は PHI のみ処理（terminator 非依存、Line 287-298）
- terminator 生成（emit_conditional_edgecfg）と PHI 生成（JoinIR）は完全に分離されている
- 競合可能性なし

## 次フェーズへの橋渡し

**Phase 269**: route family への展開
- scan_with_init / split_scan / bool_predicate_scan を Frag 化
- NormalizedShadow/JoinIR への適用
- pattern番号分岐削減
- fixture + smoke test

## 関連ドキュメント

- **設計図**: `docs/development/current/main/design/edgecfg-fragments.md`
- **現在のタスク**: `docs/development/current/main/10-Now.md`
- **バックログ**: `docs/development/current/main/30-Backlog.md`
- **Phase 267**: `docs/development/current/main/phases/archive/phase-267/README.md`

## 受け入れ基準（全達成）

### P0 成功条件
- ✅ `cargo build --release` 成功
- ✅ `cargo test --lib --release` で 1395 tests PASS
- ✅ `tools/smokes/v2/run.sh --profile quick` で 45/46 PASS
- ✅ MIR dump で Branch/Jump terminator 正常生成確認
- ✅ JoinIR fallback 経路動作確認

### P1 成功条件
- ✅ `cargo build --release` 成功
- ✅ `cargo test --lib --release` で 1444 tests PASS
- ✅ compose.rs/emit.rs unit tests 全 PASS
- ✅ `tools/smokes/v2/run.sh --profile quick` で 45/46 PASS 維持
- ✅ TODO コメント削除完了（Phase 267 P2+ TODO 解消）
- ✅ Edge-args パラメータ SSOT 化完了
- ✅ ドキュメント更新完了（4ファイル）:
  - ✅ 10-Now.md 追記
  - ✅ 30-Backlog.md 更新
  - ✅ edgecfg-fragments.md 追記
  - ✅ phases/phase-268/README.md 新規作成

## まとめ

**Phase 268 P0-P1 完全成功！**

- ✅ EdgeCFG Fragment の最初の実戦投入（if_form.rs）
- ✅ emission 層経由で層境界を綺麗に保つアーキテクチャ確立
- ✅ compose::if_() の entry edge-args SSOT 化完了
- ✅ 全テスト PASS（1444 tests + 45/46 smoke）
- ✅ TODO 削除完了

**次のステップ**: Phase 269 で scan_with_init / split_scan / bool_predicate_scan への Frag 適用 + fixture/smoke test
