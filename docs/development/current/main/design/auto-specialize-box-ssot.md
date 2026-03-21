---
Status: SSOT
Scope: LLVM lowering の AutoSpecializeBox v0（helper density 対策の自動分岐）
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- src/llvm_py/instructions/mir_call/intrinsic_registry.py
---

# AutoSpecializeBox SSOT (v0)

## 目的

`substring/concat/indexOf/length/runtime_data` 周辺で「特別処理が散る」問題を抑えつつ、
call boundary コストを下げるため、**自動分岐の判定源を 1 箇所**に固定する。

本 SSOT は、`IntrinsicRegistry` を前提にした **AutoSpecialize v0** の契約を定義する。

## 非目的

- 汎用 JIT の多段 IC 実装（mono/poly/mega 全段）
- 新しい env toggle 追加
- MIR 仕様の変更

## 箱の責務

### AutoSpecializeBox（v0）

- 入力:
  - method 名
  - arity
  - resolver の type facts（`is_stringish` / `value_types`）
  - （binop `+` 時のみ）raw SSA（`lhs_raw` / `rhs_raw`）
  - （runtime_data route 時のみ）`box_name` / `receiver_vid` / `arg_vids`
- 出力:
  - 「どの helper route を選ぶか」のみ
- 禁止:
  - method list の独自保持（`IntrinsicRegistry` を参照する）
  - fallback semantics の変更

### IntrinsicRegistryBox

- method 分類 SSOT（length-like / receiver-tag-required / string-result）
- AutoSpecializeBox は registry の分類 API 以外を見ない

## v0 決定規則（最小）

### rule AS-01: string length route

以下を満たすとき、`any.length_h` より `string.len_h` を優先する。

1. method が `length-like`（`length|len|size`）
2. arity = 0
3. receiver が resolver 上 stringish（`is_stringish` または `value_types` が StringBox）

満たさない場合は既存 route を維持する。

### rule AS-02: concat3 fold route

string `+` lowering 時、以下を満たすとき `concat_hh` より `concat3_hhh` を優先する。

1. 左右 operand が stringish（既存の `lhs_tag && rhs_tag`）
2. 片側の raw SSA が `nyash.string.concat_hh(a, b)` 呼び出し
3. もう片側は handle 化可能（既存 `to_handle` で i64 化）

変換規則:

- `(concat_hh(a, b) + c)` -> `concat3_hhh(a, b, c)`
- `(a + concat_hh(b, c))` -> `concat3_hhh(a, b, c)`

満たさない場合は既存の `concat_hh(hl, hr)` に戻す。

### rule AS-03: runtime_data array mono-route

`RuntimeDataBox.{get,set,has,push}` lowering 時、以下を満たすとき
`nyash.runtime_data.*` より `nyash.array.*` の mono-route を優先する。

1. `box_name == RuntimeDataBox`
2. receiver が resolver 上 arrayish（`value_types[receiver] == {"kind":"handle","box_type":"ArrayBox"}`）
3. method ごとの arity が一致
4. method ごとの arity が一致（`get/has/push=1`, `set=2`）

route 変換:

- `get` -> `nyash.array.get_hh`
- `set` -> `nyash.array.set_hhh`
- `has` -> `nyash.array.has_hh`
- `push` -> `nyash.array.slot_append_hh`

不成立時は既存 `nyash.runtime_data.*` route に戻す（意味不変）。

### rule AS-03b: runtime_data array integer-key route

AS-03 が成立し、かつ key VID が `i64` と判定できる場合、
`get/set/has` は整数キー専用 route を優先する。

判定源（保守的）:

1. `resolver.integerish_ids` に key VID が含まれる
2. または `value_types[key]` が `i64/int/integer`

route 変換:

- `get` -> `nyash.array.get_hi`
- `set` -> `nyash.array.set_hih`
- `has` -> `nyash.array.has_hi`

`push` は `nyash.array.slot_append_hh` を使用する。
不成立時は AS-03 (`*_hh/*_hhh`) へ戻す（意味不変）。

### rule AS-03c: runtime_data array integer-key + integer-value set route

AS-03b の `set` で、key と value の両方が `i64` と判定できる場合、
`set_hih` より `set_hii` を優先する。

判定源（保守的）:

1. `resolver.integerish_ids` に key/value VID が含まれる
2. または `value_types[key/value]` が `i64/int/integer`

route 変換:

- `set` -> `nyash.array.set_hii`

不成立時は AS-03b (`set_hih`) へ戻す（意味不変）。

## Fail-Fast / 安全性

- AutoSpecialize は「より速い route の選択」に限定し、失敗時は既存 route に戻す。
- strict/dev で route 判定が壊れても、値意味を変えない（contract first）。
- debug log の新タグ追加は v0 では行わない（ログ契約増殖を防ぐ）。

## 導入順

1. docs SSOT 固定（この文書）
2. code-side helper（判定関数）追加
3. `method_call` / `stringbox` の length route へ限定導入
4. test + gate で契約固定

## 受け入れ基準

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py`
- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
- `cargo test -p nyash_kernel string_concat3_hhh_contract array_runtime_data_route_hh_contract_roundtrip array_runtime_data_route_hi_contract_roundtrip array_runtime_data_route_hii_contract_roundtrip -- --nocapture`
- `tools/checks/dev_gate.sh quick`
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
