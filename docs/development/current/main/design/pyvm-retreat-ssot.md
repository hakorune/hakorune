# PyVM Retreat (SSOT)

Status: SSOT  
Scope: 日常導線を Rust VM / LLVM に固定し、PyVM を historical / direct-only に寄せるための運用契約。

Related:
- `CURRENT_TASK.md`
- `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- `docs/reference/environment-variables.md`
- `tools/smokes/fast_local.sh`

## Goal

- 日常の gate / selfhost / quick smoke は Rust VM / LLVM だけで判断する。
- PyVM は「歴史資産の比較・診断」のみで保持し、既定導線から外す。
- 入口契約を固定して route drift を防ぐ。

## Route policy (SSOT)

### Runtime selfhost route

- 実装入口: `src/runner/selfhost.rs`
- 既定: Stage-A child + Rust VM 実行
- `NYASH_VM_USE_PY` による route 分岐は撤去済み（runtime 側では無視）。

### JSON pipe route

- 実装入口: `src/runner/pipe_io.rs`
- 既定: CoreExecutor (`run_json_v0`) へ直行
- `NYASH_PIPE_USE_PYVM` による route 分岐は撤去済み（pipe 側では無視）。

## Inventory (A)

### Daily route (must stay green)

- `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`

### Legacy route (historical / opt-in)

- retired runner entry:
  - `src/runner/modes/pyvm.rs` は撤去済み（runtime dispatch から分離）
- runtime modules:
  - `src/runner/modes/common_util/legacy/pyvm.rs`
  - `tools/historical/pyvm/pyvm_runner.py`
- smoke examples:
  - `tools/historical/pyvm/pyvm_stage2_smoke.sh`
  - `tools/historical/pyvm/pyvm_vs_llvmlite.sh`
  - `python3 tools/historical/pyvm/pyvm_runner.py --in /path/to/mir.json`

## Smoke policy (C)

- quick/integration の日常スモークは Rust VM/LLVM のみで判断する。
- `tools/smokes/fast_local.sh` の PyVM smoke は既定実行しない。
- PyVM を回す場合は historical direct route のみ許可する:
  - `tools/historical/pyvm/pyvm_runner.py`
  - `tools/historical/pyvm/pyvm_*.sh`

## Acceptance

- `cargo check --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
- `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`

## Removal condition (future)

以下が満たせたら PyVM 物理撤去（コード削除）へ進める:

- 日常 gate と selfhost が、PyVM flags なしで一定期間 green
- `SMOKES_USE_PYVM=1` 前提の常用スクリプトが 0
- `docs/reference/environment-variables.md` で PyVM flags を removed（runtime/pipe no-op）へ昇格
