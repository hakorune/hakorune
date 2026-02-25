---
Status: SSOT
Scope: vm-hako array_get/array_set shim contract (runtime lane)
Decision: accepted (interim shim, fail-fast)
Updated: 2026-02-13
Related:
- src/runner/mir_json_v0.rs
- src/runner/modes/vm_hako.rs
- lang/src/vm/boxes/mir_vm_s0.hako
- tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_get_parity_vm.sh
- tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_set_parity_vm.sh
---

# vm-hako Array Shim Contract (SSOT)

## 目的

runtime lane（D6）で `array_get` / `array_set` を段階実装する間、
Rust loader と `.hako` runner の観測結果を一致させる。

## 現在の契約（interim shim）

- `array_get(dst,array,index)`
  - `index` は shape 検証のみ（値としては未使用）
  - 取得値は `array` レジスタの現在値（register-slot read）
- `array_set(array,index,value)`
  - `index` は shape 検証のみ（値としては未使用）
  - `array` レジスタへ `value` レジスタを上書き（register-slot write）

## fail-fast

- subset-check で必須フィールドが欠けたら reject
  - `array_get`: `dst/array/index`
  - `array_set`: `array/index/value`
- silent fallback は禁止

## 非目標（この段階ではやらない）

- 実配列セマンティクス（index 値で要素アクセス）
- ArrayBox 実体との接続
- optimizer の配列特化

## 次段（将来）

- 実配列セマンティクスは別 lane として導入
- 導入時は以下を同コミットで固定する
  - loader / `.hako` runner の同時更新
  - parity fixture（set→get / out-of-range reject）
  - SSOT 更新
