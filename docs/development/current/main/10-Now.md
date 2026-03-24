---
Status: SSOT
Date: 2026-03-25
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/20-Decisions.md
  - docs/development/current/main/30-Backlog.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard だよ。
- 置くのは current summary、実行入口、正本リンクだけ。
- 進捗履歴や長文ログは `CURRENT_TASK.md`、phase README、design SSOT に逃がす。

## Root Anchors

- Root anchor: `CURRENT_TASK.md`
- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Current Read

- Active lane: `phase-29ci`
  - status: `reopen W5 active`
  - purpose:
    - retire `Program(JSON v0)` from repo-wide external/bootstrap boundary
    - unify public/bootstrap interchange on `MIR(JSON v0)`
    - keep hard delete out of this wave
  - scope lock:
    - `Boundary + Minimal MIR Meta`
    - `Program(JSON v0)` public/bootstrap surfaces are deprecate-now
    - internal `.hako` / host-provider Program(JSON) keep is allowed only as compat that terminates in MIR
  - current W5 target:
    - `launcher.hako` route orchestration is thinned into `LauncherDispatchBox`
    - raw direct `stage1_cli.hako emit program-json` lane remains retire-only / diagnostics-only
    - explicit env-route compat probes and raw compat flags remain alive
    - wrapper/public helper retirements remain landed
- Rune lane: `phase-29cu`
  - status: `formal-close-synced`
  - narrow-scope current truth:
    - declaration-local `attrs.runes`
    - Rust direct MIR carrier
    - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
    - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
    - `.hako` parser statement/program routes fail fast on Rune invalid placement
    - Rust function-target placement / ABI-facing verifier contract
    - `.hako` root-entry carrier value-contract parity for `CallConv("c")` / `Ownership(owned|borrowed|shared)`
    - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
    - `Program(JSON v0)` remains no-widen
  - latest landed carrier cut:
    - `.hako` compiler/mirbuilder state now carries a generic function-rune map instead of `entry_runes_json`
    - `.hako` MIR attrs injection is function-name driven instead of `main` hardcode
    - `.hako` Stage-B source route now carries root-entry Rune attrs through a real `Main.main` def instead of a synthetic transport shim
  - planned future reopen only:
    - `.hako` declaration-local full Rune carrier parity beyond root-entry transport
- Bootstrap-retire lane: `phase-29cj`
  - status: `formal-close-synced`
  - current stop-line is still `src/host_providers/mir_builder.rs::module_to_mir_json(...)`
  - latest landed `.hako` cuts now cover `BuilderUnsupportedTailBox`, `Stage1MirPayloadContractBox`, `Stage1CliProgramJsonInputBox`, `Stage1CliRawSubcommandInputBox`, `LauncherArtifactIoBox`, and `LauncherPayloadContractBox`
  - `MirBuilderBox.hako`, `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako` are now treated as near-thin-floor / close-synced owners
- Runtime lane: `phase-29y`
  - parked
  - operational reading is `llvm-exe` daily / `vm-hako` reference-debug-bootstrap-proof / `rust-vm` bootstrap-recovery-compat
  - active acceptance is `phase29y_vm_hako_caps_gate_vm.sh` only
- Substrate lane: `phase-29ct`
  - stop-line reached
- JSON v0 reading
  - `Program(JSON v0)` is retire/no-widen and no longer the target external/bootstrap boundary
  - `MIR(JSON v0)` is the current external/bootstrap interchange / gate boundary
  - allowed keep:
    - internal compat/test/bootstrap-only routes
    - `.hako` mirbuilder internal input until later delete waves

## Clean-Shape Status

1. `stage1/stage2` artifact semantics の整理（landed）
2. `ABI/export manifest + generated shim` 化（landed）
3. `hako_alloc` root の物理再編（landed）
4. transitional Rust export の daily-path 退役（landed）
5. handle/provider/birth の substrate-only 化（docs-locked）
6. `Stage3` gate 追加（landed）
   - build lane compares re-emitted Program/MIR payload snapshots from a known-good seed plus `.artifact_kind`
   - skip-build lane compares an explicit prebuilt pair

## Exact Links

- Mainline workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Execution lane policy: `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- Execution lane task pack: `docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md`
- Execution lane legacy inventory: `docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md`
- Bootstrap route SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Compiler structure SSOT: `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- Stage axis SSOT: `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
- Rune final shape SSOT: `docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md`
- Rune v0 rollout SSOT: `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
- Stage3 same-result gate: `tools/selfhost/stage3_same_result_check.sh`
- ABI inventory: `docs/development/current/main/design/abi-export-inventory.md`
- JSON v0 inventory: `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- Route split note: `docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md`
- Phase 29ci closeout: `docs/development/current/main/phases/phase-29ci/README.md`

## Restart Reminder

- 最初に `git status -sb` を見る。
- 次に `CURRENT_TASK.md` を読む。
- その次に `15-Workstream-Map.md` で lane 順を確認する。
- 詳細は `10-Now.md` を増やさず、phase README / design SSOT を開く。
