Status: Completed (Blocked by Phase 255)
Scope: Phase 254 (`--profile quick` 回帰: JoinIR 未対応 loop パターン（StringUtils.index_of/2）)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/archive/phase-253/README.md
- docs/development/current/main/phases/phase-255/README.md (次フェーズ: multi-param loop wiring)

# Phase 254: `StringUtils.index_of/2` の loop shape を JoinIR で受理する

## 現象（最初の FAIL）

`./tools/smokes/v2/run.sh --profile quick` が `json_lint_vm` で失敗する。

エラー:

```
[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.
Function: StringUtils.index_of/2
```

## 対象の Nyash コード（最小形）

`apps/lib/json_native/utils/string.hako`:

```nyash
index_of(s, ch) {
  local i = 0
  loop(i < s.length()) {
    if s.substring(i, i + 1) == ch { return i }
    i = i + 1
  }
  return -1
}
```

特徴:
- while ループ（`i < s.length()`）
- loop body 内に `if (...) { return i }`
- tail に `i = i + 1`
- ループ後に `return -1`

## 問題の構造（なぜ落ちるか）

- 現状は LoopBuilder が削除済みなので、JoinIR route/shape がマッチしない loop はすべて freeze する（フォールバックなし）。
- つまり “この loop 形” を JoinIR route/shape として受理できるようにするしかない。

## 解決方針（構造的）

### 方針 A（推奨・最小）: 「index_of 形」を route shape として追加する

- “箱”として 1つ追加し、1つの質問だけに答える:
  - 「`index_of` 形の loop を JoinIR に lowering できるか？」
- by-name（関数名/Box名）でのディスパッチは禁止。
  - 形（構造）だけでマッチすること。

### 方針 B（非推奨）: freeze を回避するための例外ルート

- LoopBuilder が無い以上、ここで例外 fallback を入れるのは Fail-Fast 原則違反になりやすい。
  - 例: “Unsupported なら解釈器で実行” のような逃げ道は作らない。

## 実装タスク（P0）

### 1) 構造抽出（docs → interface）

- どの AST/StepTree 形を受理するかを README or doc comment に明記
  - 必須: loop cond が `<`/`<=` の比較で、rhs が `s.length()` または定数/変数（Phase 251/252 の流れと整合）
  - 必須: loop 内 if が “then return i” である
  - 必須: update が `i = i + 1`（Phase 253 で analyzer を緩くしたので、ここは loop route-shape 側の契約にする）

### 2) 新しい lowering 箱（案）

候補配置:
- `src/mir/join_ir/lowering/loop_routes/` 配下に `index_of/` を作り、`lower_box.rs` を置く
  - 目的: 既存 `loop_with_*` 巨大ファイルに寄せず、責務を分離する（設計優先）

入出力（案）:
- 入力: loop condition AST, loop body AST, `current_static_box_name`, env/JoinValueSpace など（loop_break route と同等の配線）
- 出力: `(JoinModule, JoinFragmentMeta)`（既存パターンと同様）

### 3) condition lowering の利用

- `s.substring(i, i+1) == ch` の中で `substring` が必要になる。
  - CoreMethodId では `StringSubstring` は `allowed_in_init=true` / `allowed_in_condition=false`。
- ここは “value expression” として lowering するため、
  - 既存 `condition_lowerer::lower_value_expression` / `MethodCallLowerer::lower_for_init` のルールに合わせる
  - 条件全体の bool は `Compare` で生成（JoinIR の `MirLikeInst::Compare`）

### 4) テストと fixture（仕様固定）

- unit test:
  - “index_of 形の loop がマッチする” / “JoinIR が生成される” を固定
- v2 smoke fixture:
  - `apps/tests/phase254_p0_index_of_min.hako`
  - `tools/smokes/v2/profiles/quick/apps/phase254_p0_index_of_vm.sh`（軽いなら quick へ、重いなら integration）

## 受け入れ基準

- `./tools/smokes/v2/run.sh --profile quick` が PASS
- by-name 分岐を追加していない（構造のみでマッチ）
- unsupported の場合は明確な Err（freeze）を維持しつつ、対象形は通る

## 進捗（P0/P1 完了）

### ✅ 完了項目

- **Task 1**: 最小 fixture + smoke scripts（integration）: ✅ 完了
  - `apps/tests/phase254_p0_index_of_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_llvm_exe.sh`

- **Task 2**: デバッグ実行（現状把握）: ✅ 完了
  - if_phi_join route（historical numbered label: `3`）と判定されるが "if-sum ではない" で reject される
  - loop_canonicalizer の `ConstStep` 不足で fail するケースがある

- **Task 3**: scan_with_init detector box 実装: ✅ 完了
  - historical path token: old scan_with_init detector basename under the retired `joinir/patterns/` lane
  - old label-3 lane より前に配置（router priority 調整）
  - `LoopBreak` と `IfPhiJoin` の双方の分類に対応
  - **検出成功**: label-6 scan_with_init match を示す historical debug token を確認

- **Task 4**: ScanWithInit Lowerer 実装: ✅ 完了
  - **Task 4-1**: extract_scan_with_init_parts() - 構造抽出関数実装
  - **Task 4-2**: scan_with_init_minimal.rs - JoinIR lowerer（main/loop_step/k_exit 生成）
  - **Task 4-3**: MirBuilder 統合 - boundary 構築と JoinIRConversionPipeline 実行
  - **Task 4-4**: mod.rs 登録

### ❌ ブロッカー（Phase 255 へ引き継ぎ）

**現象**: Integration テスト失敗
```
[ERROR] ❌ [rust-vm] VM error: Invalid value:
[rust-vm] use of undefined value ValueId(10)
(fn=StringUtils.index_of/2, last_block=Some(BasicBlockId(4)),
 last_inst=Some(Compare { dst: ValueId(14), op: Ge, lhs: ValueId(10), rhs: ValueId(13) }))
```

**根本原因**: JoinIR→MIR merge/boundary システムが**複数ループ変数を想定していない**

- 現状の仕様: early numbered-route line（historical labels: `1-5`）は単一ループ変数前提（例: `i` のみ）
- scan_with_init route（historical numbered label: `6`）の要求: 3変数ループ（`s`, `ch`, `i`）
- 問題: PHI ノードが 1つしか作られない（`s` のみ）、`ch` と `i` が undefined

**影響範囲**:
- `JoinInlineBoundaryBuilder` の `with_loop_var_name()` が単一変数想定
- `exit_bindings` が単一 carrier 用に設計
- `JoinIRConversionPipeline` が複数 PHI 作成に未対応

**Phase 254 の受け入れ境界**:
- ✅ scan_with_init route（historical numbered label: `6`）が検出される
- ✅ JoinIR が正しく生成される（main/loop_step/k_exit 構造）
- ✅ substring が BoxCall として init-time に emit される
- ❌ 実行（VM/LLVM）で PASS にするのは **Phase 255 の範囲**

### 次フェーズ（Phase 255）

詳細: [docs/development/current/main/phases/phase-255/README.md](phase-255/README.md)

課題:
- Multi-param loop の boundary/PHI/wiring を SSOT 化
- LoopState（i）と invariants（s, ch）を分けて wiring
- 受け入れ: phase254_p0_index_of の integration テストが PASS
