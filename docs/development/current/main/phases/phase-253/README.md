Status: Completed
Scope: Phase 253 (`--profile quick` 回帰: mutable-acc-spec / accumulator 判定の改善)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-252/README.md

# Phase 253: `json_lint_vm` 回帰（mutable-acc-spec）

## 現象（最初の FAIL）

`./tools/smokes/v2/run.sh --profile quick` が `json_lint_vm` で失敗する。

エラー:

```
[ERROR] ❌ MIR compilation error: [joinir/mutable-acc-spec] Assignment form not accumulator pattern (required: target = target + x)
```

## 背景（なぜここで落ちるか）

`loop_break` route（legacy Pattern2 label）の pipeline は、ループ本体の代入から「mutable accumulator（`x = x + y`）」を検出して
最適化/簡略化に利用する。ところが現在の analyzer が Fail-Fast すぎて、
“accumulator ではない単なる代入” を見つけた時点で Err にしてしまい、JoinIR 経路全体を落としている。

対象 SSOT:
- `src/mir/loop_route_detection/legacy/mutable_accumulator_analyzer.rs`

## 方針（構造的に直す）

### 原則

- “accumulator pattern を検出できた時だけ” spec を返す。
- それ以外は Err ではなく `Ok(None)` に戻して **別経路（通常 lowering）へ譲る**。
- 例外として、本当に矛盾があるケースだけ Err（例: 同一変数への複数代入など、既に `Ok(None)` にしている）。

### 対処療法の禁止

- 特定関数名（`StringUtils.*`）や特定 script 名（`json_lint_vm`）で分岐しない。
- “`-` の時だけ” のような場当たりでなく、spec と契約として整理する。

## 実装タスク（P0）

### 1) Analyzer の振る舞いを “検出器” に寄せる

ファイル:
- `src/mir/loop_route_detection/legacy/mutable_accumulator_analyzer.rs`

変更案:
- 以下のケースを `Err` ではなく `Ok(None)` に変更する（= accumulator ではないと判断する）
  - `value_node` が `BinaryOp` ではない（例: `i = s.length() - 1`）
  - `BinaryOperator` が `Add` 以外（例: `i = i - 1`）
  - 左辺が `target` と一致しない（例: `x = y + x`）
  - RHS が Literal/Variable 以外（例: `x = x + (i + 1)`）

目的:
- “accumulator っぽくない代入” が混ざる loop でも、JoinIR 全体を落とさずに進める。

### 2) `-`（decrement）を accumulator として扱うかの設計を決める（P1 で可）

選択肢:
- A) `i = i - 1` は “accumulator としては未対応” なので `Ok(None)`（安全・最小）
- B) `i = i - 1` を “step=-1” として spec に載せる（将来の表現力は上がるが、下流の取り扱い整備が必要）

まずは quick を緑に戻す目的で A を推奨。

## テスト（仕様固定）

- unit tests を追加して「非 accumulator 代入があっても Err にならず `Ok(None)`」を固定する。
  - 例: ループ body に `local i = s.length() - 1` 相当の Assignment があるケース
  - 例: `i = i - 1` があるケース

## 受け入れ基準

- `./tools/smokes/v2/run.sh --profile quick` が PASS
- “たまたま `json_lint_vm` だけ通す” ための by-name 分岐を追加していない
- analyzer の戻り値契約が docs と tests で固定されている

## 結果（Phase 253 終点）

- `mutable_accumulator_analyzer` は “検出器” として振る舞うようになり、非 accumulator 代入で Err を出さず `Ok(None)` に譲る。
- quick の最初の FAIL は次に切り出し（Phase 254）:
  - `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern`
  - Function: `StringUtils.index_of/2`
