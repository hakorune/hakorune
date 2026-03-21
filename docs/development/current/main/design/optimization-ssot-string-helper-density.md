---
Status: SSOT
Scope: LLVM-HOT-20 以降の helper density 最適化方針（substring/concat/indexOf/length/runtime_data）
Related:
- CURRENT_TASK.md
- docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/auto-specialize-box-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/substring-view-materialize-boundary-ssot.md
- docs/development/current/main/design/helper-boundary-policy-ssot.md
- docs/development/current/main/design/optimization-portability-classification-ssot.md
---

# String Helper Density Optimization SSOT

## 目的

`kilo_kernel_small` の AOT 経路で、算術より helper 境界コスト（call/lookup/boxing/safepoint）が支配的になっている。
この SSOT は、特別扱いを散在させずに最適化を進めるための責務分離と導入順を固定する。

## 非目的

- 言語仕様変更
- `.hako` 側 workaround 追加
- ベンチ専用の by-name 分岐
- Rust runtime 内部 micro-tuning の深掘り（Class B 優先化）

## 実行方針（移植優先）

この SSOT の実装は portability-first で運用する。

- 優先: Class A（契約/IR/ABI/gate 固定）
- 条件付き: Class B（runtime 内部最適化、Temporary Bridge 明記必須）
- 診断限定: Class C（probe、常設禁止）

分類とコミット前チェックの正本:
`docs/development/current/main/design/optimization-portability-classification-ssot.md`

## 現状ベースライン（2026-02-23）

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=78`, `py_ms=112`, `ny_vm_ms=1017`, `ny_aot_ms=85`, `ratio_c_aot=0.92`
- main IR hot inventory（代表）:
  - `ny_check_safepoint=19`
  - `nyash.runtime_data.get_hh=5`
  - `nyash.runtime_data.set_hhh=4`
  - `nyash.string.substring_hii=4`
  - `nyash.string.concat_hh=4`
  - `nyash.any.length_h=3`
  - `nyash.string.indexOf_hh=2`

## 箱の責務（SSOT）

### 1) GeneralOptimizerBox

全メソッド共通で効く最適化を担当する。

- callsite monomorphic 化（IC/guard）
- helper 境界の lookup/boxing 回数削減
- safepoint 配置方針（entry/backedge 基調）

禁止:
- 個別メソッド名に依存した特別分岐の直接実装

`AutoSpecializeBox v0` はこの層の実装契約として運用する。
詳細: `docs/development/current/main/design/auto-specialize-box-ssot.md`

### 1b) HelperBoundaryPolicyBox

helper 境界の tuning 条件を 1 箇所へ隔離する。

- host handle alloc/reuse policy
- string span cache admission/promotion policy

禁止:

- helper 本体への閾値直書き
- ベンチ単位の ad-hoc 分岐常設

詳細: `docs/development/current/main/design/helper-boundary-policy-ssot.md`

### 2) IntrinsicRegistryBox

特別扱いを 1 箇所に隔離する。

- 「どのメソッドを intrinsic 候補として扱うか」の宣言
- receiver tag / result tag / length-like alias などの分類 SSOT
- Verifier による登録整合チェック（fail-fast）

禁止:
- lowering 各所での独自 method list ハードコード

### 3) BackendLayoutBox

意味不変の配置最適化を担当する。

- ThinLTO / PGO / hot-cold split
- i-cache / iTLB 方向の配置改善

禁止:
- MIR/意味論の変更

## IntrinsicRegistry 契約（最小）

分類は registry で宣言し、consumer は helper API 経由で参照する。

- `length-like`: `length`, `len`, `size`
- `receiver-string-tag-required`: `substring`, `indexOf`, `lastIndexOf`
- `string-result`: `substring`, `esc_json`, `node_json`, `dirname`, `join`, `read_all`, `toJson`

運用ルール:

1. method 判定は registry API のみを使う
2. 未登録メソッドは generic route へ落とす（best-effort分岐禁止）
3. registry 追加時は tests + gate + CURRENT_TASK.md を同コミットで更新

注記:

- `@hint` / `@contract` / `@intrinsic_candidate` の最小仕様と導入順は
  `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md` を正本にする。
- 本書は helper-density 最適化の責務分離と優先順のみを扱う。

## substring view / concat N-ary 導入ルール

### substring view

- 原則: `base + range` の view を許可
- ただし長期保持を避けるため materialize 境界を固定する
  - map/array 永続格納前
  - FFI 境界前
  - GC policy が要求する境界
- v0 の詳細契約は
  `docs/development/current/main/design/substring-view-materialize-boundary-ssot.md`
  を正本にする。

### concat N-ary

- `concat(concat(a,b),c)` を `concat3(a,b,c)` へ畳み、intermediate alloc を削減する
- 初期段階は `concat3/concat4` までの固定 arity で開始し、rope/cons は後段
- v0（cleanup-6）では **`concat3_hhh` のみ**導入し、対象は `binop "+"` lowering の raw SSA 連鎖検出に限定する
  - `lhs_raw` or `rhs_raw` が `nyash.string.concat_hh` 呼び出しのときだけ fold
  - AST rewrite は禁止（analysis-only / lowering-only）
  - fold 不成立時は既存 `concat_hh` に即時フォールバック（意味不変）

### runtime_data mono-route（Array receiver）

- `RuntimeDataBox.{get,set,has,push}` は array receiver が確定した callsite で
  `nyash.array.*_hh/*_hhh` へ寄せる（AS-03）。
- 意味互換を保つため、array 側に `runtime_data` 同等契約の export を用意する。
- 不成立時は既存 `nyash.runtime_data.*` route へ戻す（best-effort分岐禁止）。

## 導入順（固定）

1. registry 集約（挙動不変）
2. concat N-ary（低リスク）
   - v0: `concat3_hhh` のみ（`cleanup-6`）
3. runtime_data mono-route（Array receiver, no-env）
4. substring view + materialize 契約
5. safepoint 配置最適化（entry/backedge）
6. backend layout（ThinLTO/PGO/split）

## 受け入れ基準

必須:

- `tools/checks/dev_gate.sh quick`
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
- `cargo test -p nyash_kernel string_concat3_hhh_contract array_runtime_data_route_hh_contract_roundtrip -- --nocapture`

実測:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
- `ratio_c_aot` がベースライン（0.92）を持続、または改善

## Fail-Fast ポリシー

- intrinsic route が不成立なら generic route に戻す（silent no-op はしない）
- registry/consumer 間の契約破れは strict/dev で freeze 可能な形で検出する
- 新ログタグ導入時は `ai-handoff-and-debug-contract.md` を SSOT 更新する
