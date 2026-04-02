---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: investigate delete readiness for the remaining explicit legacy/compat callers rooted at `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`, including the compiled-stage1 surrogate caller.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
---

# 29x-98 Legacy Route Retirement Investigation

## Rule

- delete-ready remains none until the caller inventory reaches zero.
- no new daily caller may be added to `emit_object_from_mir_json(...)`.
- investigation is caller-by-caller; do not reopen compare bridge daily ownership.
- the helper stays archive-later while any explicit legacy/compat caller remains.

## Relationship To 29x-99

- `29x-98` owns stop-line and delete-readiness.
- `29x-99` owns beauty-first path / namespace / filesystem recut planning.
- `29x-99` does not authorize helper deletion early; it only raises task granularity and move order clarity.

## Keep

- `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`
- `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`
- `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` (archive-later surrogate caller)

## Current Caller Inventory

The current `emit_object_from_mir_json(...)` caller inventory is three keep lanes plus one archive-later surrogate caller.

| Caller | Bucket | Note |
| --- | --- | --- |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate caller; keeps the helper alive but is not a daily route |

## Direct Callers vs Wrapper Layers

Keep the direct caller inventory separate from wrapper/orchestrator layers.

| Surface | Layer | Status | Read as |
| --- | --- | --- | --- |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | direct caller | keep | explicit compat payload; still calls `CodegenBridgeBox.emit_object_args(...)` |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | direct caller | keep | gated compat/proof stub; still calls `CodegenBridgeBox.emit_object_args(...)` |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | direct caller | keep | explicit legacy receiver for `emit_object_from_mir_json(...)` |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | direct caller | keep | explicit legacy receiver for `emit_object_from_mir_json(...)` |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | direct caller | keep | explicit legacy receiver for `emit_object_from_mir_json(...)` |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | wrapper | archive-later | transport wrapper only; not a direct `emit_object` caller |
| `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | orchestrator | archive-later | historical pack owner only; not a direct `emit_object` caller |

## Current Reduction Verdict

- no archive-ready direct caller exists today.
- no low-blast caller reduction is visible on the current proof set.
- the next real movement requires an exact root-first replacement proof, not more wrapper trimming.
- helper deletion remains blocked until the direct caller inventory reaches zero.

## Cleanup Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `lang/src/vm/hakorune-vm/extern_provider.hako` + compat selfhost wrapper stack | current stop-line surfaces after bucket cleanup |
| Next | exact root-first replacement proof | required before any direct caller drain beyond the current stop-line |
| Later | `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)` / `CodegenBridgeBox.emit_object_args(...)` / Rust dispatch residues | delete only after caller inventory reaches zero |

## Replacement Matrix

The canonical successor family is the root-first daily route, concretely `env.codegen.compile_ll_text(...)` plus `env.codegen.link_object(...)` where the caller already owns LL text. The current legacy callers still consume MIR(JSON), so the successor is not a 1:1 drop-in yet.

| Current caller | Bucket | Current behavior | Likely successor family | Blocker |
| --- | --- | --- | --- | --- |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | keep | `env.codegen.emit_object` dispatch from MIR interpreter | root-first daily compile route (`env.codegen.compile_ll_text(...)` / `env.codegen.link_object(...)`) once the caller stops owning MIR(JSON) | still enters through legacy MIR(JSON) emit path |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | keep | loader-cold lane for `env.codegen.emit_object` with MIR JSON version patching | same root-first daily compile route family | same legacy MIR(JSON) entry contract |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | keep | plugin loader `emit_object` arm for MIR(JSON) | same root-first daily compile route family | same legacy MIR(JSON) entry contract |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate reads MIR(JSON) file and calls the legacy helper | compiled-stage1 should eventually bypass the legacy helper once a new front-door exists | helper still required for bootstrap/compat |

## Inventory Findings

- `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`
  - current contract is `env.codegen.emit_object(mir_json_string) -> object path`.
  - this file already carries `compile_ll_text` and `link_object` branches, but the `emit_object` arm still owns MIR(JSON), so it cannot flip to the canonical route without an upstream caller change.
- `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`
  - current contract is also `env.codegen.emit_object(mir_json_string) -> object path`.
  - this arm patches missing MIR version fields before calling the legacy helper, so a direct retarget would also need a replacement for that normalization step.
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
  - current contract is plugin-side `emit_object(mir_json) -> object path`.
  - this file has `compile_ll_text`, but it still lacks caller-side route/profile ownership and does not hold LL text on the `emit_object` arm.
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
  - current contract is `compile_obj(mir_path) -> object path`.
  - this is a compiled-stage1/bootstrap surrogate; it reads MIR(JSON) from disk and cannot cleanly jump to the root-first route without moving the owner upward into `.hako`.

## Upstream Cleanup Targets

- `hostbridge.rs`
  - cleanup target is upstream compat caller removal, not `hostbridge.rs` first.
  - current upstream owner is `lang/src/shared/host_bridge/codegen_bridge_box.hako::emit_object_args(...)`, plus legacy `hostbridge.extern_invoke(...)` proof callers.
- `loader_cold.rs`
  - cleanup target is upstream caller removal, not `loader_cold.rs` first.
  - current upstream owners are the `env.codegen.emit_object` extern lanes reached from `src/backend/mir_interpreter/handlers/calls/global.rs`, `src/backend/mir_interpreter/handlers/externals.rs`, and the legacy `CodegenBridgeBox` / `ExternCallLowerBox` call shape that still produces `env.codegen.emit_object`.
- `extern_functions.rs`
  - cleanup target is upstream caller removal, not `extern_functions.rs` first.
  - current upstream owner is still the legacy `CodegenBridgeBox.emit_object_args(...)` / `env.codegen.emit_object` route; daily callers should stop at `LlvmBackendBox`.
- `llvm_backend_surrogate.rs`
  - cleanup target is the compiled-stage1/module-dispatch caller set, not the surrogate file first.
  - current upstream owner is `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` via `try_dispatch(...)`, with compat/proof callers on `selfhost.shared.backend.llvm_backend.{compile_obj,link_exe}`.

## `CodegenBridgeBox.emit_object_args(...)` Caller Inventory

Direct code callers currently in tree:

| Caller | Bucket | Note |
| --- | --- | --- |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | archive-later compat/proof | provider-first stub / canary-only surface; not a daily owner |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | archive-later compat/proof | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` gated compatibility stub only |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | archive-later example/proof | explicit proof/example caller, not a daily route |

Proof-only direct `hostbridge.extern_invoke("env.codegen", "emit_object", ...)` callers currently in tree:

| Caller | Bucket | Note |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_compare_branch_canary_vm.sh` | proof-only | llvmlite compare/provider canary |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_canary_vm.sh` | proof-only | llvmlite provider canary |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_const42_canary_vm.sh` | proof-only | llvmlite provider canary |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_ternary_collect_canary_vm.sh` | archived proof-only | explicit emit/link proof on legacy lane; superseded by `phase29ck` runtime proof |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_map_set_size_canary_vm.sh` | archived proof-only | explicit emit/link proof on legacy lane; superseded by `phase29ck` runtime proof |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_provider_vm.sh` | archived proof-only | selfhost lowering probe for the legacy extern name; currently hard-skipped |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_vm.sh` | archived proof-only | selfhost lowering probe for the legacy extern name; currently hard-skipped |

## Direct Caller Findings

- `lang/src/runner/stage1_cli/core.hako`
  - the legacy `backend == "llvm"` branch is now retired from this raw compat lane.
  - the file no longer calls `CodegenBridgeBox.emit_object_args(...)`; it fail-fasts with an explicit unsupported-backend marker instead.
  - live stage1 artifact authority stays at `lang/src/runner/stage1_cli_env.hako`; daily backend callers still stop at `lang/src/shared/backend/llvm_backend_box.hako`.
- `lang/src/vm/hakorune-vm/extern_provider.hako`
  - only active when `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`; otherwise it returns an empty compat stub.
  - treat as compat/proof only, not daily/mainline.
  - dead alias `env.codegen.emit_object_ny` is retired; the gated stub now accepts only `env.codegen.emit_object`.
  - owner-surface arms and compat/proof C-ABI arms are now grouped explicitly in the file; no behavior change was made.
  - the old `phase251` lowering canaries are now quarantined under `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/` because they are inactive hard-skips, not active suite coverage.
  - cleanup target: pin a root-first selfhost lowering proof first, then retire this gated stub.
- `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
  - explicit proof/example caller that still demonstrates `emit_object_args(...)` plus `link_object_args(...)`.
  - not a daily route and not a current owner.
  - direct invoker is `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`.
  - the shell wrapper is transport only; the `.hako` file is the actual proof/example caller surface.
  - the payload now lives under `tools/compat/legacy-codegen/` so `tools/selfhost/examples/` stays generator-first.
  - `tools/compat/legacy-codegen/run_compat_pure_pack.sh` is the only remaining thin wrapper above that canonical compat wrapper; it is not a separate proof owner.
  - old aliases `tools/selfhost/run_hako_llvm_selfhost.sh` and `tools/selfhost/run_all.sh` are retired; keep the compat pack entry singular.
  - current root-first replacement proof exists only on the `.hako VM -> LlvmBackendBox -> C-API -> exe` lane (`phase29ck_vmhako_llvm_backend_runtime_proof.sh`), not as a drop-in for this historical safe-vm wrapper.
  - therefore this caller remains archive-later until the compat wrapper either gains a root-first equivalent or is retired as a whole.
  - cleanup target: demote or archive once proof/example coverage moves to the root-first route.
- `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
  - provider-first canary/proof stub only; not a daily owner.
  - `HAKO_LLVM_EMIT_PROVIDER` remains a canary selector, not a daily backend selector.
  - repo-local direct import caller inventory is zero; the box is exercised only through explicit proof/canary surfaces and env selection.
  - decision: keep explicit as compat/proof keep until the provider-first proof/canary surface moves to `LlvmBackendBox` or is archived.

## LLVMEmitBox Decision

- `lang/src/llvm_ir/emit/LLVMEmitBox.hako` stays `compat/proof keep`.
- it is not a daily owner and not a delete-ready target.
- current live usage is explicit proof/canary coverage (`phase2044` llvmlite provider canaries, `phase29ck` startup probes) plus the remaining compat bridge surface.
- do not reopen this decision unless those proof/canary callers either move to the root-first route or are archived.

## CodegenBridgeBox Producer Decision

- `lang/src/shared/host_bridge/codegen_bridge_box.hako::emit_object_args(...)` has no non-proof/non-compat daily-route dependency.
- the remaining direct callers are all archive-later proof/compat surfaces:
  - `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
  - `lang/src/vm/hakorune-vm/extern_provider.hako`
  - `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
- delete-readiness still stays `none` because those proof/compat surfaces are still live.
- next cleanup is archive sequencing for those caller surfaces, not deleting `CodegenBridgeBox` first.

## Proof-Only Caller Findings

| Surface group | Status | Daily-route dependency | Cleanup / archive condition |
| --- | --- | --- | --- |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_{compare_branch,canary,const42}_canary_vm.sh` | integration proof-only coverage; monitor-only keep | none | archive when the legacy helper caller inventory reaches zero and llvmlite canary evidence is no longer needed |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | archived proof-only coverage on the legacy emit/link lane | none | keep as replay evidence while `emit_object_from_mir_json(...)` remains archive-later; replay bundle is `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt` |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | archived proof-only lowering evidence for the legacy extern name | none | keep as quarantine evidence until a root-first selfhost lowering proof exists; replay bundle is `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt` |

## Archive Sequencing Matrix

| Surface | Current role | Replacement proof status | Sequence |
| --- | --- | --- | --- |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | archived explicit legacy emit/link proof | exact root-first replacements are green in `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_{ternary_collect,map_set_size}_runtime_proof.sh`; manual replay now lives in `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/run_all.sh` | archived; keep only as replay evidence |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` + `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | historical compat selfhost wrapper proof | root-first runtime proof exists only on the separate vm-hako owner lane: `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh` | archive-later, but not drop-in replaceable yet |
| `lang/src/vm/hakorune-vm/extern_provider.hako` + `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | legacy extern lowering proof with archived quarantine canaries | no root-first selfhost lowering proof is pinned yet | keep `extern_provider.hako` until a root-first lowering proof exists; archived `phase251` pair remains evidence only |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` + `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_{compare_branch,canary,const42}_canary_vm.sh` | provider-first llvmlite proof/canary surface | no root-first llvmlite provider proof replaces this exact surface | keep until llvmlite proof demand disappears or moves to archive |

- the three `phase2044` llvmlite canaries are live through the dedicated suite manifest `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt` and still match integration-profile discovery filters.
- the archived `phase2111` + `phase251` evidence now share the replay bundle `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt`.

## Root-First Proof Candidate Matrix

This matrix is only about the current `29x-98` stop-line surfaces. It does not reopen daily ownership.

| Surface | Candidate root-first proof | What it proves today | Drop-in replacement? | Blocker |
| --- | --- | --- | --- | --- |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` + `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh` | `.hako VM -> LlvmBackendBox -> C-API -> exe` works on the vm-hako owner lane | no | the compat wrapper still demonstrates `CodegenBridgeBox.emit_object_args(...)` + `link_object_args(...)` on the historical safe-vm route |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | none pinned yet | only the gated compat/proof stub is proven; archived `phase251` keeps the old lowering evidence visible | no | there is still no root-first selfhost lowering proof for `env.codegen.emit_object` replacement on this surface |
| archived `phase2111` emit/link pair | `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_{ternary_collect,map_set_size}_runtime_proof.sh` | exact root-first replacements are already green | yes, for the archived pair only | replacement is exact for those two payloads, not for the compat selfhost wrapper or `extern_provider.hako` |

## Proof-Only Direct Caller Group Recheck

The direct `env.codegen.emit_object` caller groups are now stable enough to read as three buckets:

| Group | Files | Status | Meaning |
| --- | --- | --- | --- |
| active monitor-only keep | `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_{canary,compare_branch,const42}_canary_vm.sh` | keep | only live proof-only direct caller group still under integration |
| archived legacy emit/link proof | `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | archive evidence | replay-only evidence for the old direct emit/link lane |
| archived selfhost lowering probe | `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | archive evidence | replay-only evidence while `extern_provider.hako` still has no root-first lowering replacement |

- within the live `phase2044` trio:
  - `codegen_provider_llvmlite_canary_vm.sh` stays the irreducible provider-plumbing keep
  - `codegen_provider_llvmlite_compare_branch_canary_vm.sh` and `codegen_provider_llvmlite_const42_canary_vm.sh` are merge-later candidates only; neither is archive-ready on current replacement coverage

## Phase2044 Directory Semantics

- `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/`, `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/`, and `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/` are the semantic homes for the recut phase2044 buckets.
- only these three files belong to the llvmlite monitor-only keep surface:
  - `codegen_provider_llvmlite_canary_vm.sh`
  - `codegen_provider_llvmlite_compare_branch_canary_vm.sh`
  - `codegen_provider_llvmlite_const42_canary_vm.sh`
- the canonical manifest is `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/llvmlite_monitor_keep.txt`.
- the dedicated suite manifest is `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt`.
- this dedicated suite manifest is the final live keep bucket for the llvmlite surface.
- the `hako_primary_no_fallback_*` scripts live under `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/`.
- the `mirbuilder_provider_*` scripts live under `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/`.
- near-term cleanup separates these semantics with bucket runners first:
  - `run_llvmlite_monitor_keep.sh`
  - `run_hako_primary_no_fallback_bucket.sh`
  - `run_mirbuilder_provider_bucket.sh`
- physical path splitting has landed; keep the discovery filters and archive references aligned with the new homes.
- keep the llvmlite trio suite-locked and leave the remaining groups bucket-runner only.
- no file in the llvmlite trio is currently archive-ready; any future reduction should be a merge of `compare_branch` / `const42`, not an archive move.

## Compat Pack Archive Conditions

- `tools/compat/legacy-codegen/run_compat_pure_pack.sh` is the only remaining historical compat-pack wrapper entry.
- current direct dependencies are:
  - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`
  - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
- `tools/smokes/v2/profiles/integration/proof/pure-legacy-cluster/run_all.sh` is now only an orchestrator; the legacy cluster is split into:
  - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`
  - `tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh`
  - `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/run_vm_adapter_legacy_cluster.sh`
  - `tools/smokes/v2/profiles/integration/proof/native-reference/run_native_reference_bucket.sh`
- current blockers are:
  - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` still demonstrates the old `CodegenBridgeBox.emit_object_args(...)` plus `link_object_args(...)` route.
  - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh` still owns the two active historical pure C-API keep pins (`array_set_get`, `loop_count`), now locked by `tools/smokes/v2/suites/integration/compat/pure-keep.txt`; the archive-backed historical pins are now locked by `tools/smokes/v2/suites/archive/pure-historical.txt`.
  - `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/run_vm_adapter_legacy_cluster.sh` is a separate legacy cluster under the proof directory, and `tools/smokes/v2/profiles/integration/proof/native-reference/run_native_reference_bucket.sh` is its native reference companion.
  - `HAKO_CAPI_PURE=1` is still documented as a compat-only route, not as removed/no-op.
- archive-ready only when all three hold:
  1. the phase2120 active pure canaries are either replaced by current root-first/native proofs or moved under archive-only replay; no replacement exists yet for `array_set_get` / `loop_count`, so they stay keep.
  2. the selfhost compat wrapper either gains a root-first drop-in replacement or is retired as a whole.
  3. current docs no longer need `HAKO_CAPI_PURE=1` as a live compat entry toggle.
- delete-ready is still `none`.

## Low-Blast Cleanup Candidates

Ranked from lowest blast radius to higher dependency risk:

1. `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_{compare_branch,canary,const42}_canary_vm.sh`
   - keep now as integration monitor-only proofs
   - bucket semantics are now isolated by dedicated runner plus dedicated suite manifest; archive-later once legacy helper callers reach zero and llvmlite evidence is no longer needed
2. `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt`
   - archive replay bundle for `phase2111` and `phase251`
   - keep as evidence-only carrier while the helper and compat/proof callers remain
3. `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
   - keep now as compat/proof only
   - archive-later after the provider-first proof surface is archived or moved to root-first
4. `lang/src/shared/host_bridge/codegen_bridge_box.hako`
   - correct producer to retire later
   - blocked until the remaining proof/compat callers drain
5. `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` and `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
   - archive-later compat wrapper proof; document and sequence before touching producer deletion
6. `lang/src/vm/hakorune-vm/extern_provider.hako`
   - blocked on a root-first selfhost lowering proof
7. Rust dispatch residues under `src/backend/mir_interpreter/handlers/extern_provider/*` and `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
   - blocked until upstream `.hako` callers stop generating `env.codegen.emit_object`

## Phase2111 Replacement Closure

Exact root-first replacements for the two `phase2111` payloads are green.

| Payload | Root-first replacement |
| --- | --- |
| `s3_link_run_llvmcapi_ternary_collect_canary_vm.sh` | `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_ternary_collect_runtime_proof.sh` |
| `s3_link_run_llvmcapi_map_set_size_canary_vm.sh` | `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh` |

The legacy emit/link pair has been moved under `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/`.

## Upstream Producer Findings

- `.hako`-side producer shape
  - true upstream owner is `lang/src/shared/host_bridge/codegen_bridge_box.hako`.
  - `CodegenBridgeBox.emit_object_args(...)` normalizes the payload and still issues `env.codegen.emit_object(mir_json)`.
  - current `.hako` callers that reach this producer are compat/proof or proof/example only; no new daily caller should be added.
- Rust-side dispatch shape
  - `src/backend/mir_interpreter/handlers/calls/global.rs` and `src/backend/mir_interpreter/handlers/externals.rs` still accept `env.codegen.emit_object`, but only as legacy extern dispatch surfaces.
  - `src/backend/mir_interpreter/handlers/extern_provider/lane.rs` classifies `env.codegen.emit_object` as `LoaderCold`, not a daily fast lane.
  - `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` and `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` still reach the helper, but they are receivers for existing compat/proof callers, not the cleanup starting point.
- cleanup rule
  - do not start by deleting Rust dispatch files.
  - first remove the upstream callers that still generate `env.codegen.emit_object`.
  - after the upstream caller inventory reaches zero, collapse the Rust dispatch residues and then delete `emit_object_from_mir_json(...)`.

## Ordered Investigation Queue

1. `lang/src/runner/stage1_cli/core.hako`
   - landed: raw compat llvm branch retired; no longer a direct caller.
2. `lang/src/vm/hakorune-vm/extern_provider.hako`
3. `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
4. proof-only direct `hostbridge.extern_invoke("env.codegen", "emit_object", ...)` callers
5. `lang/src/llvm_ir/emit/LLVMEmitBox.hako` keep/archive decision
   - landed: keep as compat/proof keep; no repo-local direct import callers remain.
6. `lang/src/shared/host_bridge/codegen_bridge_box.hako` upstream producer keep/archive conditions
   - landed: no non-proof/non-compat daily dependency remains; remaining direct callers are archive-later proof/compat surfaces.
7. archive sequencing for the remaining proof/compat caller surfaces
8. Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`) only after upstream caller inventory reaches zero

## Retirement Order

1. keep `hostbridge.rs`, `loader_cold.rs`, and `extern_functions.rs` as explicit compat callers until their upstream callers stop owning MIR(JSON) and switch to the root-first daily route.
2. keep `llvm_backend_surrogate.rs` as archive-later until the compiled-stage1/bootstrap path has a cleaner front door than `emit_object_from_mir_json(...)`.
3. once upstream `.hako` callers and proof/example callers stop generating `env.codegen.emit_object`, collapse the Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`).
4. only when all caller surfaces are gone: delete `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`.

## Investigation TODO

1. keep `LLVMEmitBox` fixed as compat/proof keep; do not reopen it as a daily route.
2. keep `CodegenBridgeBox.emit_object_args(...)` fixed as an archive-later producer; do not treat it as a daily route.
3. confirm proof-only direct `hostbridge.extern_invoke(..., "emit_object", ...)` callers remain proof-only and not daily dependencies.
4. record archive conditions for the remaining proof/compat caller surfaces before touching `CodegenBridgeBox` or Rust dispatch residues.
  - `phase2111` explicit emit/link pair is archived and `phase251` legacy lowering pair is quarantined.
  - `phase2044` bucket semantics and compat selfhost wrapper ownership are explicitized.
  - `phase2044` llvmlite monitor-only keep bucket cleanup is complete; the next sequencing target is the remaining proof/example callers:
    - `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
    - `lang/src/vm/hakorune-vm/extern_provider.hako`
5. keep the legacy helper archive-later until the caller set reaches zero.
6. push new daily callers through `LlvmBackendBox -> env.codegen.compile_ll_text(...) -> env.codegen.link_object(...)`, not through `env.codegen.emit_object`.
7. when the caller set reaches zero, delete `emit_object_from_mir_json(...)`, then collapse the Rust dispatch residues and phase docs.

## Delete Condition

- all explicit callers have moved to root-first or daily route surfaces.
- compare/archive proof coverage is preserved in archive assets.
- `emit_object_from_mir_json(...)` no longer appears in code-side caller inventory.

## Non-Goals

- no new compare bridge.
- no reopening of file-based `mir_json_file_to_object(...)`.
- no daily caller growth.
