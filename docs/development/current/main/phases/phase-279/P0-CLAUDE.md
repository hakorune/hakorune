# Phase 279 P0: Type propagation pipeline SSOT unification（Claude Code 指示書）

Status: instructions / implementation

目的（根治）:
- “2本のコンパイラ” を潰す。
  - 例: ルートAは BinOp 型伝播→PHI 型解決、ルートBは PHI 型解決→BinOp 型伝播、のような **順序ドリフト**で同一fixtureが壊れる。
- 型伝播（Copy/BinOp/PHI…）の入口と順序を **1本のSSOT**に固定し、どのルートでも同じ結果になることを保証する。

スコープ（P0）:
- Rust側の “型伝播パイプライン” を SSOT 1箇所に集約し、主要ルートから必ず呼ぶ
- fail-fast ガードで “PHI-before-BinOp” を禁止する
- 代表fixtureで “ルート差が消えた” を確認する

Non-goals:
- coercion / language semantics の変更
- 新しい env var 追加
- LLVM harness 側の場当たり的推論追加（best-effort禁止）

参照:
- Phase 279 SSOT: `docs/development/current/main/phases/phase-279/P0-INSTRUCTIONS.md`
- 現状の2ルート（要確認）:
  - builder lifecycle: `src/mir/builder/lifecycle.rs`
  - JoinIR bridge: `src/mir/join_ir_vm_bridge/joinir_function_converter.rs`
- PHI推論箱: `src/mir/phi_core/phi_type_resolver.rs`

---

## Step 0: “入口SSOT” の置き場所を決める

新規モジュールを作って、全ルートがそこを呼ぶ形にする。

推奨パス（例）:
- `src/mir/type_propagation/pipeline.rs`

中身（最小）:
- `pub(crate) fn run_type_propagation_pipeline(func: &mut MirFunction, value_types: &mut ValueTypes, mode: ...) -> Result<(), String>`
  - Copy type propagation
  - BinOp re-propagation
  - PHI type inference（PhiTypeResolver）
  - （必要最小の後続だけ）

注意:
- “既にあるロジックを移動して呼び出し一本化” が目的。新しい推論ルールを増やさない。

---

## Step 1: lifecycle.rs から SSOT pipeline を呼ぶ

対象:
- `src/mir/builder/lifecycle.rs`

要件:
- lifecycle 内の “Copy/BinOp/PHI 推論の順序” を削って、SSOT pipeline 呼び出しに置き換える。
- PHI推論は **必ず BinOp re-propagation の後**（順序固定）。

受け入れ:
- lifecycle 側に “局所の順序実装” が残っていない。

---

## Step 2: joinir_function_converter.rs から SSOT pipeline を呼ぶ

対象:
- `src/mir/join_ir_vm_bridge/joinir_function_converter.rs`

要件:
- joinir_function_converter 内の “Copy/BinOp/PHI 推論の順序” を削って、同じ SSOT pipeline 呼び出しに置き換える。

受け入れ:
- JoinIR bridge ルートでも “PHI-before-BinOp” が起きない構造になる。

---

## Step 3: fail-fast ガード（順序ドリフト禁止）

方針:
- “PHI type inference が走る時点で BinOp re-propagation が未実行” を Err にするガードを SSOT pipeline 内に入れる。

目的:
- 将来の改修で順序が崩れたときに、静かに壊れず “原因で止まる” ようにする。

---

## Step 4: 代表fixtureでルート差が消えたことを確認

最低条件:
- “Copy chain + Int+Float promotion + PHI” を含む fixture を使う（Phase 275 系など）。
- その fixture を “両ルート” で通して、`value_types` / PHI `dst_type` が一致することを確認する。

注意:
- 新しいCIは増やさない。ローカルの代表スモークで十分。

---

## Step 5: docs 更新（SSOTの参照先を固定）

更新対象:
- `docs/development/current/main/phases/phase-279/README.md`

必須:
- “このファイルが pipeline unification の入口SSOT” と明記
- 主要呼び出し元（lifecycle / joinir_function_converter）を列挙
- “LLVM harness は consumer（best-effort禁止）” を明記

---

## 完了条件（P0）

- SSOT pipeline の入口が 1 箇所
- lifecycle / JoinIR bridge がその入口を必ず呼ぶ
- “PHI-before-BinOp” を fail-fast で禁止できる
- 代表fixtureで “ルート差による二重バグ” が再現しない
