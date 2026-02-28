---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: `.hako` 主体で runtime/plugin 意味論を完結し、最終的に host 境界を最小 C ABI に固定する移行順序を定義する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/abi/nyrt_c_abi_v0.md
---

# Hako Fullstack Host-ABI Completion (SSOT)

## Conclusion

- 「runtime/plugin loader は `.hako` へ移植不可」という扱いは採用しない。
- 目的を分離する:
  1. 意味論（resolver/loader/invoke/future/value_codec/runtime_data 等）は `.hako` へ移植する。
  2. OS 依存処理（dlopen/syscall/process/time/fs/net）は host C ABI に最小化して残す。
- 最終形は `.hako` 主体 + 最小 host C ABI で成立する。

## Non-negotiables

1. Canonical ABI 面は 2 つを維持する:
   - Core C ABI
   - TypeBox ABI v2
2. 第3 ABI 面を追加しない。
3. host shim は「配線のみ」。意味論判断を入れない。
4. strict/dev は fail-fast。silent fallback を禁止する。
5. no-delete-first を維持し、route-zero + stability を先に固定する。

## Architecture Target (fixed)

### A. `.hako` side (logic owner)

- runtime/plugin loader の意味論:
  - method resolve
  - instance lifecycle
  - invoke routing
  - value codec
  - async/future semantics
  - runtime_data / semantics orchestration
- これらは `.hako` box/module 側へ集約する。

### B. host side (thin owner)

- OS 依存の最小 API 呼び出しだけを担当:
  - dynamic library load/symbol lookup
  - process/environment/time
  - file/socket primitive
- host 側は marshalling + error code 変換のみを行い、ルーティング判断は禁止。

## Migration Order (fixed)

### Step 0: Docs sync + inventory lock

- 「意味論を `.hako`」「OS依存を host」に分割した inventory を固定する。
- `hako-runtime-c-abi-cutover-order-ssot.md` と矛盾がないことを受理条件にする。

### Step 1: Host ABI surface lock

- host API を用途別に固定する（loader/process/fs/net/time）。
- 署名契約（borrowed/owned, error contract）を表で固定する。

### Step 2: `.hako` facade lock

- `.hako` 側に HostFacade（呼び口）を作る。
- 既定経路を facade 経由に統一し、direct host call を禁止する。

### Step 3: Runtime/plugin meaning migration

- `1 boundary = 1 commit` で順次置換:
  - plugin_loader_v2 `method_resolver -> instance_manager -> bridge/loader`
  - nyash_kernel plugin `invoke_core/birth/runtime_data/semantics -> value_codec -> future/invoke`
- 各境界で fixture+gate を同コミットで固定する。

### Step 4: Default route cutover

- mainline/CI 既定を `.hako + ABI` に固定。
- compat は default-off（明示時のみ）へ固定。

### Step 5: Source-zero preparation

- Rust 実装は保険残置のまま「未使用監査」を継続する。
- portability 連続 green と route drift 監査を完了条件にする。

### Step 6: Source-zero execution (future phase)

- 上記条件が満たされたら、Rust runtime/plugin 実装を段階撤去する。
- 撤去は別 lock で実施し、この文書では順序のみ固定する。

## Acceptance Gates

1. `tools/checks/dev_gate.sh runtime-exec-zero`
2. `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
3. `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
4. `tools/checks/dev_gate.sh portability`
5. `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2`

## Explicitly Deferred

1. Rust source 物理削除の即時実行
2. 3rd ABI 面の新設
3. runtime/plugin 移行と同時の最適化タスク再開
