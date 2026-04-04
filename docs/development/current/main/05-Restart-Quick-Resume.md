---
Status: Active
Date: 2026-04-05
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-105 digit OR-chain LLVM parity regression`
- current front: `long digit OR-chain under loop(true) を LLVM EXE parity で戻す`
- blocker: `LLVM EXE で long digit OR-chain が 0/1 に潰れる`
- recent landed:
  - `phase-104 loop(true) + break-only digits（read_digits 系）`
  - `phase-103 if-only regression baseline（VM + LLVM EXE）`
  - `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`
  - `phase-100 Pinned Read-Only Captures`
  - `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-105/README.md`

## Successor Corridor

1. `phase-105 digit OR-chain LLVM parity regression`
2. `phase-110x selfhost execution vocabulary SSOT`
3. `phase-111x selfhost runtime route naming cleanup`
4. `phase-112x vm-family lane naming hardening`
5. `phase-113x kernel vs vm-reference cluster wording correction`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Next Cleanup Corridor

- separate `stage / route / backend override / lane / kernel`
- rename `runtime-mode exe` toward `runtime-route mainline`
- harden VM family lane names as `rust-vm-keep / vm-hako-reference / vm-compat-fallback`
- reserve `kernel` for `nyash_kernel`; treat `lang/src/vm` as VM/reference cluster

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase95_json_loader_escape_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase96_json_loader_next_non_ws_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_json_loader_escape_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_pinned_local_receiver_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_mutable_accumulator_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_string_accumulator_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase103_if_only_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase103_if_only_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase103_if_only_early_return_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase103_if_only_early_return_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_llvm_exe.sh
git diff --check
```

## Optional Checks

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```
