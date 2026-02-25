---
Status: Provisional SSOT
Scope: 最適化を `.hako` 移植可能性で分類し、Rust 側過剰最適化の技術負債化を防ぐ。
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/helper-boundary-policy-ssot.md
- docs/development/current/main/design/de-rust-lane-map-ssot.md
- docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md
---

# Optimization Portability Classification SSOT

## Goal

- 速度改善を継続しつつ、将来の `.hako` 移植を困難にしない。
- 最適化を「性能」だけでなく「移植可能性」で管理する。

## Non-goals

- 最適化停止
- Rust 実装の即時撤去
- perf lane の実験トグル禁止

## Classification (SSOT)

### Class A: Portable Contract

`.hako` 側へ同じ契約で移植可能な最適化。

条件:
- 意味論を変えない（挙動既定値は不変）
- policy/contract が 1 箇所に集約されている
- fail-fast 契約がある

必要成果物:
- design SSOT（責務 + invariants）
- contract test（挙動固定）
- `.hako` 側の移植先（lane B/C どちらか）を明示

### Class B: Rust Runtime Specific

Rust 実行基盤の都合で現時点では `.hako` へ直接移植しない最適化。

条件:
- lock/allocator/ABI など Rust runtime 内部事情に依存
- 言語仕様や MIR 契約には影響しない

必要成果物:
- Rust 側に責務を隔離（散在禁止）
- 代替方針（将来の `.hako` 相当）を 1 行で記録
- 撤去/置換条件を明示

### Class C: Temporary Probe

計測・切り分け専用の一時措置。

条件:
- dev/perf 専用
- 既定OFF またはスクリプトローカル
- 本番経路の意味論に影響しない

必要成果物:
- 期限または撤去条件
- 観測目的（何を判定するか）
- 失敗時の戻し方

## Merge Gate Rule

最適化 PR/commit は、次を満たすこと。

1. Class A/B/C の分類が docs に記載されている。
2. Class A は contract test が同時に追加されている。
3. Class B/C は撤去/移植条件が明示されている。
4. 既定挙動が変わる場合は Decision を先に docs に書く。

## LLVM-HOT-20 Stop Line (pre-selfhost minimal)

最適化を継続するか selfhost へ進むかは、次の最小 4 本で判定する。

1. `cargo check --bin hakorune`
2. `tools/checks/dev_gate.sh quick`
3. `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_KILO_PARITY_LOCK_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
4. `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`

判定条件:

- step 3 は `phase21_5_perf_kilo_parity_lock_contract_vm.sh` を含み、`aot_status=ok` かつ `ratio_c_aot >= 0.95` を要求する。
- 上記 4 本が緑なら、LLVM-HOT-20 は monitor-only へ移行して selfhost 優先へ切替える。

## Current Inventory (LLVM-HOT-20 subset)

- `cleanup-55` (helper boundary policy centralization): **Class A**
  - 理由: policy 契約を 1 箇所に集約し、挙動不変で導入
- `cleanup-56` (env-switchable helper boundary policy): **Class A**
  - 理由: policy switch を SSOT 化し、invalid 値 fail-fast を固定
- `cleanup-54` (alloc atomic removal + cache slot compaction): **Class B**
  - 理由: runtime 内部（lock/alloc）最適化で `.hako` 直移植対象ではない

## Operational Rule

- 新しい最適化はまず Class A を目指す。
- Class B/C を選ぶ場合、理由を書かずに実装を先行しない。
- `phase21_5` investigation には「class」を 1 行で追記する。

## No-Regret Rule (de-rust migration)

`.hako` 移植が前提の期間は、次の順序で最適化を選ぶ。

1. Class A（移植後も残る契約/IR/ABI 最適化）を優先する。
2. Class B（Rust runtime 内部最適化）は、Class A の blocker 解消後に限定実施する。
3. Class C（probe）は診断専用で、常設経路へ昇格させない。

### Class A の具体対象

- policy/contract の SSOT 化（helper boundary, intrinsic registry, autospecialize）
- IR callshape の縮退（`*_hh -> *_hi` など）
- materialize 境界や poll 境界の契約固定
- gate/bench/asm 観測契約の固定

### Class B の具体対象

- lock/allocator/cache layout の micro tuning
- Rust 固有 API/型レイアウトに依存する branch 微調整

Class B は次を必須とする。

- `Temporary Bridge` ラベル（docs 1 箇所）
- 撤去条件（どの migration milestone で消すか）
- `.hako` 相当戦略の 1 行記録

## Commit Check (portability-first)

最適化コミット前に、次の 3 点を必ず記録する。

1. この差分は Class A/B/C のどれか。
2. `.hako` 移植後にも残るか（Yes/No）。
3. No の場合、撤去条件と代替先（.hako 側の箱/契約）。
