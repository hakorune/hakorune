# normalized_dev Removal SSOT

Status: SSOT  
Date: 2026-03-06  
Scope: `normalized_dev` を runtime 主経路から外し、段階的に削除する順序を固定する。

## Purpose

- `normalized_dev` を「compiler の本線 normalize 層」と誤認させない。
- runtime 主経路を `Facts -> Recipe -> Verifier -> Lower` に固定する。
- `Structured -> Normalized -> MIR` の dev-only 実験線を mainline から分離し、削除順序を明文化する。

## Decision

- `normalized_dev` は **runtime 主経路ではない**。
- `normalized_dev` は **Recipe / VerifiedRecipe の acceptance gate を置き換えない**。
- 削除は一括ではなく、**runtime quarantine -> plumbing removal -> module removal -> feature removal** の順で進める。

## Non-goals

- Recipe-first / Verified-only boundary の変更
- mainline release 挙動の変更
- `.hako` workaround による通過

## Runtime Boundary (SSOT)

- runtime 主経路:
  - `Facts -> Recipe -> Verifier -> Lower`
- forbidden:
  - `Structured -> Normalized -> MIR` を runtime entry として再導入しない
  - `normalized_dev` を canonical acceptance gate として扱わない

## Removal Phases

### Phase R1: Runtime Quarantine

Goal:
- `normalized_dev` を **runtime entry / bridge / runner** から切り離す。

Work:
- `join_ir_runner/api.rs` から normalized roundtrip 実行経路を削除する。
- `join_ir_vm_bridge/bridge.rs` から normalized direct routing を外し、Structured-only bridge に戻す。
- docs で `normalized_dev` を dev-only / removal lane と明記する。

Accept:
- `run_joinir_function(...)` は Structured path のみを使う。
- `bridge_joinir_to_mir_with_meta(...)` は Structured path のみを使う。
- `cargo build --release --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### Phase R2: Plumbing Removal

Goal:
- `normalized_dev` の公開トグル/公開 mode 語彙を削る。

Work:
- `JoinIrMode::{NormalizedDev, NormalizedCanonical}` を撤去または historical-only 化する。
- `NYASH_JOINIR_NORMALIZED_DEV_RUN` を `src/config/env` の公開 helper から外し、removal-lane 内部に閉じ込める。
- `join_ir_vm_bridge/mod.rs` の normalized bridge export を撤去する。

Accept:
- `rg -n "NormalizedDev|NormalizedCanonical|NYASH_JOINIR_NORMALIZED_DEV_RUN" src/config src/mir/join_ir_runner src/mir/join_ir_vm_bridge`
  - historical note / archive を除き runtime entry で 0件

### Phase R3: Module Removal

Goal:
- `src/mir/join_ir/normalized/**` の dev-only 実験箱を撤去する。

Work:
- `normalized.rs` の dev-only roundtrip / fixture / shape_guard 群を削除する。
- `join_ir_vm_bridge/normalized_bridge/**` は 2026-03-06 の R2 cleanup で先行削除済み。残りは `src/mir/join_ir/normalized/**` 本体。
- private fixture 依存 (`docs/private/.../normalized_dev/fixtures/`) を棚卸しし、不要物を撤去する。
- dedicated integration test `tests/normalized_joinir_min.rs` と submodules を削除する。
- pure feature-only residue (`if_sum_break_pattern.rs`, `parse_string_composite_pattern.rs`, `scope_manager_bindingid_poc`) を先行削除する。

Accept:
- `rg -n "normalized_dev|normalized-dev" src/mir/join_ir`
  - historical comment / deletion ledger を除き 0件
- `cargo test --features normalized_dev --test normalized_joinir_min --no-run`
  - `no test target named 'normalized_joinir_min'`

### Phase R4: Feature Removal

Goal:
- Cargo feature と残存 `#[cfg(feature = "normalized_dev")]` を撤去する。

Work:
- `Cargo.toml` の `normalized_dev = []` を削除する。
- `binding_context` / `loop_route_detection::legacy`（current physical path: `src/mir/loop_route_detection/legacy/`、historical token/retire条件は `route-physical-path-legacy-lane-ssot.md` を参照） / `join_ir::lowering` / frontend 周辺の conditional fields / helpers を通常実装へ吸収または削除する。

Accept:
- `rg -n 'feature *= *"normalized_dev"|normalized_dev' Cargo.toml src`
  - 0件
- `cargo build --release --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

Verification (2026-03-06):
- `rg -n '#\\[cfg\\((not\\()?feature = "normalized_dev"' src`
  - 0件
- `rg -n 'feature *= *"normalized_dev"|normalized_dev|NYASH_JOINIR_NORMALIZED_DEV_RUN' Cargo.toml src tests`
  - 0件
- ungated `ownership/ast_analyzer` helpers (`core/node_analysis/loop_helper/tests`) were synced to the always-on lane
  - `pub(super)` sibling visibility restored
  - `ASTNode::BlockExpr` ownership walk added
  - test imports fixed for lib-test compile
- `cargo build --release --bin hakorune`
  - PASS
- `cargo test --release --lib ast_analyzer --no-run`
  - PASS
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - PASS

## Current Phase

- 2026-03-06: **Phase R4 complete**

## Checks

```bash
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
rg -n "normalized_dev|normalized-dev" src/mir/join_ir_runner src/mir/join_ir_vm_bridge docs/development/current/main/design
```
