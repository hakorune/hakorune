---
Status: SSOT
Decision: provisional
Date: 2026-03-24
Scope: execution-lane migration 中に見つかった legacy/delete-candidate を 1 箇所で triage し、lane-local retire 実装と混線させない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md
  - docs/development/current/main/design/code-retirement-history-policy-ssot.md
  - docs/development/current/main/design/selfhost-smoke-retirement-inventory-ssot.md
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29y/README.md
---

# Execution Lanes Legacy Retirement Inventory (SSOT)

## Goal

- execution-lane migration 中に見つかった legacy/delete 候補を lane-local README だけに散らさない。
- 「見つけた」「delete-ready」「まだ keep」の triage を 1 本で固定する。
- 実際の削除は既存の retire policy と phase owner に従って行う。

## Status Buckets

| bucket | meaning |
| --- | --- |
| `daily keep` | current daily lane で必要 |
| `stage1 proof keep` | stage1 bridge/proof/snapshot で必要 |
| `reference lane keep` | vm-hako reference/debug/bootstrap-proof lane で必要 |
| `bootstrap/recovery keep` | rust-vm or stage0 keep として必要 |
| `compat keep` | daily owner ではないが compat で必要 |
| `archive candidate` | live caller/gate から外れ、archive home を決めれば移せる |
| `delete-blocked` | remove target だが blocker が残る |
| `delete-ready` | successor proof と absence proof がそろっている |

## Rules

1. migration 中に legacy/delete 候補を見つけたら、まずここへ row を追加する。
2. actual removal は lane-local doc で owner を決めてから行う。
3. `delete-ready` にする条件:
   - successor or replacement proof exists
   - active docs/gates/callers no longer require the item
   - current owner phase agrees it is removable
4. code copy は作らない。
   - path / artifact token / proof link だけを記録する。
5. deletion procedure itself still follows `code-retirement-history-policy-ssot.md`.

## Seed Inventory (2026-03-24)

| item | current role | bucket | owner | delete blocker / note | successor proof |
| --- | --- | --- | --- | --- | --- |
| `src/stage1/program_json_v0/**` | bootstrap-only stage1 proof boundary | `delete-blocked` | `phase-29ci` | remaining caller inventory is not empty | `source -> direct MIR(JSON v0) -> backend/VM` convergence plus phase-29ci delete order |
| `src/runner/stage1_bridge/**` | future-retire bridge lane | `delete-blocked` | `phase-29ci` | bridge cluster still carries live bootstrap proof/workflow | phase-29ci bridge delete order and caller removal proof |
| `lang/bin/hakorune` | stage1 stable snapshot artifact | `stage1 proof keep` | `lang/README.md` + distribution docs | stage2+ distribution artifact is not active yet | future stage2+ distribution packaging |
| `vm-hako` runtime lane | semantic/reference/debug/bootstrap-proof lane | `reference lane keep` | `phase-29y` | lane remains active as reference/debug/bootstrap-proof | none; not a delete target in the current wave |
| `tools/smokes/v2/profiles/archive/vm_hako_caps/**` blocked pins | archived vm-hako blocker evidence (`app1` stack-overflow and mapbox blocked pins) | `archive candidate` | `phase-29y` | active gate no longer consumes them; keep only as manual replay evidence | `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh` |
| `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh` | archived vm-hako direct-helper tombstone probe | `archive tombstone` | `phase-29ci` + `phase-29y` | archived monitor evidence only; live lowerer owners are retired | phase-29ci outer caller audit and phase-29y gate both proceed without it |
| `lang/src/vm/mini_vm*.hako` and pre-S0 proof siblings | older proof/demo runtime siblings outside the live `mir_vm_s0_*` surface | `delete-blocked` | `phase-29y` | confirm no active callers/smokes before removal; do not touch live `mir_vm_s0_*` keep | live runtime surface is `src/runner/modes/vm_hako/**` + `lang/src/vm/boxes/mir_vm_s0_*` |
| `rust-vm` bootstrap/recovery substrate | bootstrap/recovery substrate and in-crate interpreter pieces used by explicit keeps | `bootstrap/recovery keep` | `phase-29y` | do not treat every VM-shaped component as the delete target; split source-execution route first | none; broad substrate is not a delete target in the current wave |
| `--backend vm` Rust source-execution keep | source prepare / parse / macro expansion / MIR compile / in-crate interpreter execution | `delete-blocked` | `phase-29y` | active proof/recovery callers are not inventoried to zero; naming lock and gated-alias decision must land before removal | `source -> direct MIR(JSON v0) -> LoweringPlan -> ny-llvmc/exe`, with `vm-hako` kept separately as reference/conformance |

## W3.5 Active `--backend vm` Caller Inventory Lock (2026-05-01)

Reading rules:

- count executable active callsites, not README examples, historical phase instructions, or `archive/**` replay material
- keep `--backend vm-hako` reference/conformance callers out of the `--backend vm` delete-readiness count
- keep `vm-compat-fallback` separate: it is explicit only via `NYASH_VM_USE_FALLBACK=1`
- result: `--backend vm` Rust source-execution keep is **not caller-zero**; do not add a gated alias or delete step yet

Repro query:

```bash
rg -n --glob '!target/**' \
  --glob '!tools/smokes/v2/profiles/archive/**' \
  --glob '!tools/smokes/v2/profiles/integration/apps/archive/**' \
  --glob '!docs/**' \
  -- '--backend[ =]vm\b' tools Makefile tests src lang examples benchmarks README.md
```

Active caller families:

| caller family | representative evidence | classification | current owner / next action |
| --- | --- | --- | --- |
| runner route owner | `src/runner/route_orchestrator.rs` dispatches `BootstrapRustVmKeep` to `execute_bootstrap_rust_vm_keep(...)` | `bootstrap/recovery keep` | keep; route vocabulary is now `bootstrap-rust-vm-keep` |
| compat fallback capsule | `src/runner/route_orchestrator.rs` dispatches `CompatFallback` to `execute_compat_vm_fallback_capsule(...)`; `tools/checks/vm_route_bypass_guard.sh` guards direct callsite ownership | `compat keep` | keep explicit; requires `NYASH_VM_USE_FALLBACK=1` |
| selfhost proof wrappers | `tools/selfhost/proof/run_stageb_compiler_vm.sh`, `bootstrap_selfhost_smoke.sh`, `selfhost_smoke.sh`, `selfhost_stage3_accept_smoke.sh`, `selfhost_vm_smoke.sh` | `stage1 proof keep` | keep as proof-only; do not promote to daily route |
| Stage-A Program(JSON v0) -> MIR compat bridge | `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | `compat keep` | classify as Program(JSON v0)->MIR bridge work, not VM retirement work |
| Program(JSON v0) / MIR emit helpers | `tools/hakorune_emit_mir.sh` Stage-B Program(JSON v0), selfhost-builder, and provider emit runners | `stage1 proof keep` / `compat keep` | keep until direct MIR(JSON v0) replacement removes these shell callers |
| plugin compatibility proof | `tools/plugins/plugin_v2_smoke.sh` explicit plugin host proof and optional functional proof paths | `compat keep` | keep as plugin compatibility proof only |
| active smoke suites using Rust VM source execution | `tools/smokes/v2/profiles/integration/{stageb,selfhost,joinir,core,apps,phase29x,phase21_5,proof,parser,argv,async}/**` and `tools/smokes/v2/profiles/quick/core/**` | mixed `proof keep` / `diagnostic keep` | keep; each suite needs owner-local migration before any alias gate |
| route trace / de-rust guard suites | `tools/smokes/v2/profiles/integration/phase29x/observability/**` and `tools/smokes/v2/profiles/integration/phase29x/derust/**` | `diagnostic keep` | keep; they pin bootstrap/reference/compat capsule selection |
| perf / engineering diagnostics | `tools/perf/**`, `tools/engineering/run_vm_stats.sh`, `tools/engineering/parity.sh`, `tools/debug/phi/**` | `diagnostic keep` | keep explicit; not daily ownership evidence |
| miscellaneous root/dev entrypoints | `Makefile`, `tools/exe_first_smoke.sh`, `tools/exe_first_runner_smoke.sh`, `tools/using_*_smoke.sh`, `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh` | `proof keep` / `dev diagnostic keep` | keep until each entrypoint has a documented successor route |

Excluded reference callers:

| excluded family | representative evidence | reason |
| --- | --- | --- |
| `vm-hako` capability/reference suites | `tools/smokes/v2/profiles/integration/vm_hako_caps/**` and `tools/smokes/v2/suites/integration/vm-hako-core.txt` | `vm-hako` is reference/conformance, not the Rust source-execution delete target |
| `vm-hako` LLVM/provider monitor proofs | `tools/smokes/v2/lib/llvm_backend_runtime_proof_common.sh`, `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/**`, `tools/smokes/v2/profiles/integration/compat/extern-provider-stop-line-proof/**` | monitor/reference evidence only |
| route observability `vm-hako` assertions | `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh` | verifies `lane=vm-hako-reference`; not counted as Rust VM keep demand |

Follow-up boundary:

- caller inventory is now source-backed enough to block premature deletion
- `stage_a_compat_bridge.rs` is classified as Program(JSON v0)->MIR compat bridge work, not VM retirement work
- future gated alias discussion can only start after owner-local migrations reduce the active caller families above
- no `vm-hako` gate/delete/rename is implied by this inventory

## Non-Goals

- replacing lane-local retirement orders
- storing old code in docs
- turning every keep row into an immediate delete target
