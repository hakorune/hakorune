---
Status: Provisional SSOT
Scope: `substring_hii` view 化 v0 の導入前契約（docs-first）
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/auto-specialize-box-ssot.md
- docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md
---

# Substring View / Materialize Boundary SSOT

## Goal

`nyash.string.substring_hii` の alloc/copy 密度を下げるため、`StringView(base, start, end)` を v0 で導入する。
ただしメモリ保持事故を防ぐため、materialize 境界を先に固定する。

## Non-goals (v0)

- rope/cons の導入
- `concat` 木の再構成
- 言語仕様変更（表面文法変更）

## v0 Data Contract

- `substring_hii` は「即 materialize」ではなく view を返してよい。
- view は `base_handle + range` だけを保持する。
- view 判定・解決は runtime 側 1 箇所に集約し、lowering 側に散らさない。
- ただし very short slice は eager materialize を許可する。
  - current threshold: `<= 8 bytes`
  - 理由: `StringViewBox::new` / `BoxBase::new` の identity cost を減らし、transient view churn を抑えるため

## Materialize Boundary (fixed)

次の境界では必ず materialize する。

1. map/array への永続格納前
2. FFI/C ABI 境界（`i8*` を要求する helper 呼び出し）前
3. GC/RC policy が「親バッファ保持を延命しすぎる」と判定した地点

次の境界では materialize しない（view のまま許可）。

1. `length` / `size`
2. `indexOf` / `lastIndexOf`
3. 同一関数内の read-only 連鎖

補足:

- 上の「view のまま許可」は mid/long slice を前提にした rule だよ。
- very short slice (`<= 8 bytes`) は例外で、read-only chain でも eager materialize を許可する。
- これは view semantics の撤回ではなく、transient view creation を減らすための explicit policy。

## Fail-Fast Contract

- materialize 境界で view が処理不能なら、generic route へ戻す（silent no-op 禁止）。
- view metadata の不整合は strict/dev で freeze 可能なタグで検出する。

## Rollout Order

1. docs-first（本書 + hotspot investigation 更新）
2. runtime 側 view 表現の最小導入（既定 OFF）
3. `substring_hii` だけを view 化し、materialize 境界を gate で固定
4. `kilo_kernel_small` / `kilo_text` の差分確認後に既定運用を判断

## v0 Acceptance

1. `tools/checks/dev_gate.sh quick`
2. `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
3. `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
