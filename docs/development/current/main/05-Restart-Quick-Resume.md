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

- lane: `phase-129x vm orchestrator/public gate follow-up`
- current front: public `vm` gate surfaces を source-backed に再点検する
- blocker: `src/runner/route_orchestrator.rs` の public gate surface と `src/runner/stage1_bridge/direct_route/mod.rs` の binary-only legacy gate がまだ残る
- landed: `phase-128x` backend-hint chain narrowing
- active next: `phase-129x vm orchestrator/public gate follow-up`
- recent landed:
  - `phase-127x compat route raw vm cut prep`
  - `phase-128x stage1 bridge vm gate softening`
  - `phase-125x vm bridge/backend gate follow-up`
  - `phase-124x vm public docs/manual demotion`
  - `phase-123x proof gate shrink follow-up`
  - `phase-122x vm compat route exit plan`
  - `phase-121x vm backend retirement gate decision`
  - `phase-120x vm route retirement decision refresh`
  - `phase-119x vm debug/observability surface review`
  - `phase-118x proof wrapper surface review`
  - `phase-117x vm compat/proof env hardening`
  - `phase-116x execution surface alias pruning`
  - `phase-115x vm route retirement planning`
  - `phase-114x execution surface wording closeout`
  - `phase-113x kernel vs vm-reference cluster wording correction`
  - `phase-112x vm-family lane naming hardening`
  - `phase-111x selfhost runtime route naming cleanup`
  - `phase-110x selfhost execution vocabulary SSOT`
  - `phase-105 digit OR-chain LLVM parity regression`
  - `phase-104 loop(true) + break-only digits（read_digits 系）`
  - `phase-103 if-only regression baseline（VM + LLVM EXE）`
  - `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-128x/README.md`

## Successor Corridor

1. `phase-129x vm orchestrator/public gate follow-up`
2. `phase-130x vm public gate final cleanup`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Next Cleanup Corridor

- keep `stage / route / backend override / lane / kernel` split fixed
- keep VM family lane names fixed
- keep `--backend vm` in compat/proof/debug only and demote broad docs/manual wording before touching the backend gate again
- default `stage1_cli_env.hako` child path is backend-hint free
- current buckets:
  - compat route: `run.sh --runtime --runtime-route compat`
  - proof gates: `tools/selfhost/proof/run_stageb_compiler_vm.sh`, `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - debug/observability: phase29x vm-family smokes

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
