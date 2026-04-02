---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: investigate delete readiness for the remaining explicit legacy/compat callers rooted at `src/host_providers/llvm_codegen/legacy_mir_front_door.rs::compile_object_from_legacy_mir_json(...)`, including the compiled-stage1 surrogate caller.
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
- no new daily caller may be added to `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)`.
- investigation is caller-by-caller; do not reopen compare bridge daily ownership.
- the helper stays archive-later while any explicit legacy/compat caller remains.

## Relationship To 29x-99

- `29x-98` owns stop-line and delete-readiness.
- `29x-99` owns beauty-first path / namespace / filesystem recut planning.
- `29x-99` does not authorize helper deletion early; it only raises task granularity and move order clarity.

## Keep

- `src/host_providers/llvm_codegen/legacy_mir_front_door.rs::compile_object_from_legacy_mir_json(...)`
- `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` (archive-later surrogate caller)

## Current Caller Inventory

The current explicit helper caller inventory is one keep chokepoint plus one archive-later surrogate caller.

| Caller | Bucket | Note |
| --- | --- | --- |
| `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` | keep | canonical Rust compat-codegen chokepoint; still owns the explicit MIR(JSON) text helper call |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate caller; keeps the helper alive but is not a daily route |

## Final Helper Watch Split

Treat the last two caller surfaces as separate watches with different unblock conditions.

| Watch | Surface | Current role | Unblock condition | Verdict |
| --- | --- | --- | --- | --- |
| `watch-1` | `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` | keep chokepoint for `emit_object(mir_json_text) -> object path` | contract-preserving Rust root-first replacement for the `emit_object` branch | watch-only |
| `watch-2` | `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` | archive-later compiled-stage1 surrogate for `compile_obj(json_path)` | cleaner compiled-stage1 front door than the explicit compat helper | watch-only |

## Preferred End State

- do not solve `watch-1` and `watch-2` with different new helpers.
- first create one Rust-side no-helper root-first primitive for `MIR(JSON text) -> object path`.
- close `watch-1` on top of that primitive first.
- then shrink `watch-2` to `json_path -> read_to_string -> same primitive`.
- do not promote `LlvmBackendEvidenceAdapterBox` into a new daily/compat front door; it stays evidence/proof only.

## Watch-1 Caller Groups

`compat_codegen_receiver.rs` is the one Rust keep chokepoint, but its upstream callers still arrive through distinct contracts.

| Group | Current upstreams | Current contract | Read as |
| --- | --- | --- | --- |
| plugin-loader env.codegen | `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | `env.codegen.emit_object(mir_json_text)` | direct plugin-loader compat entry |
| MirInterpreter hostbridge dispatch | `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs` via `dispatch_loader_hostbridge_codegen_invoke(...)` | `hostbridge.extern_invoke("env.codegen", "emit_object", ...)` | compat dispatch bridge into the same chokepoint |
| MirInterpreter loader-cold extern | `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs` via `dispatch_loader_cold_codegen_extern(...)` | `env.codegen.emit_object` extern | legacy extern acceptance into the same chokepoint |

## Watch-1 Caller Reduction Order

| Order | Group | Why first / last |
| --- | --- | --- |
| `1` | MirInterpreter loader-cold extern | most legacy-specific accept path; still owns version-patching pressure |
| `2` | MirInterpreter hostbridge dispatch | compat dispatch bridge; daily compile/link no longer depends on this path |
| `3` | plugin-loader env.codegen | retire `emit_object` last so `env.codegen` can collapse into compile/link-only live seam |

## Watch-1 Replacement Gap

| Contract item | Current owner | What must replace it before demotion | Verdict |
| --- | --- | --- | --- |
| MIR(JSON text) input | `compat_codegen_receiver::emit_object(...)` | one Rust-side no-helper root-first primitive that still accepts text input | missing |
| version patching for old payloads | `compat_codegen_receiver::patch_mir_json_version(...)` | moved into the legacy wrapper path or retired by explicit upstream contract change | missing |
| trace / observability point | `compat_codegen_receiver::trace_call/trace_result` | preserved at the replacement chokepoint | available, but tied to current owner |
| `emit_object(mir_json_text) -> object path` result contract | `compat_codegen_receiver::emit_object(...)` | contract-preserving replacement over the single Rust text primitive | missing |

## Watch-2 Caller Groups

`llvm_backend_surrogate.rs` is not a generic runtime lane. It is a compiled-stage1 residue behind module-string dispatch.

| Group | Current upstreams | Current contract | Read as |
| --- | --- | --- | --- |
| compiled-stage1 module dispatch | `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` via `try_dispatch(...)` | `selfhost.shared.backend.llvm_backend.compile_obj(json_path)` | live bootstrap/compiled-stage1 residue |
| compiled-stage1 link path | `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` via `try_dispatch(...)` | `selfhost.shared.backend.llvm_backend.link_exe(obj_path, exe_path)` | same surrogate cluster; not the helper blocker |

## Watch-2 Replacement Gap

| Contract item | Current owner | What must replace it before demotion | Verdict |
| --- | --- | --- | --- |
| MIR(JSON file path) input | `llvm_backend_surrogate::compile_obj_from_json_path(...)` | `json_path -> read_to_string -> same Rust text primitive used by watch-1` | missing |
| file read + string helper bridge | `llvm_backend_surrogate::compile_obj_from_json_path(...)` | file-wrapper shim over the shared text primitive, or a cleaner compiled-stage1 front door | missing |
| object-path return contract | surrogate string-handle return | contract-preserving replacement in compiled-stage1 dispatch | missing |

## Direct Callers vs Wrapper Layers

Keep the direct caller inventory separate from wrapper/orchestrator layers.

| Surface | Layer | Status | Read as |
| --- | --- | --- | --- |
| `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` | direct caller | keep | canonical Rust compat-codegen chokepoint; still owns the explicit MIR(JSON) text helper call |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` | direct caller | archive-later | compiled-stage1/archive-later surrogate still calls the explicit helper from a compat home |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | direct caller | keep | explicit compat payload, but now routed through `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)`, not the legacy Rust helper |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | direct caller | keep | gated compat/proof stub, but now root-hydrates MIR(JSON) and calls `LlvmBackendBox.compile_obj_root(...)`, not the legacy Rust helper |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | wrapper | archive-later | transport wrapper only; not a direct `emit_object` caller |
| `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | orchestrator | archive-later | historical pack owner only; not a direct `emit_object` caller |

## Current Reduction Verdict

- no archive-ready explicit helper caller exists today.
- low-blast Hako-side caller reduction is landed; selfhost payload and `extern_provider.hako` no longer call the legacy Rust helper.
- Rust receiver collapse is landed; the remaining keep lane is the single `compat_codegen_receiver.rs` chokepoint.
- the generic `llvm_codegen::emit_object_from_mir_json(...)` symbol/export is deleted; the remaining helper is explicit at `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)`.
- physical helper deletion remains blocked until the remaining two-caller inventory reaches zero.

## Cleanup Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `watch-1 compat_codegen_receiver replacement watch` | keep the explicit helper until the Rust `emit_object` contract has a replacement |
| Next | `watch-2 surrogate replacement watch` | shrink the surrogate into `json_path -> read_to_string -> same text primitive` after `watch-1` closes |
| Later | `none` | no additional cleanup wave is queued before the watch resolves |

## Replacement Matrix

The canonical successor family is the root-first daily route, concretely `env.codegen.compile_ll_text(...)` plus `env.codegen.link_object(...)` where the caller already owns LL text. The current legacy callers still consume MIR(JSON), so the successor is not a 1:1 drop-in yet.

| Current caller | Bucket | Current behavior | Likely successor family | Blocker |
| --- | --- | --- | --- | --- |
| `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` | keep | shared Rust `emit_object(mir_json) -> object path` chokepoint for plugin loader and MIR interpreter | direct root-first adapter or no-helper compat receiver path | still owns MIR(JSON) text normalization plus route selection |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate reads MIR(JSON) file and calls the legacy helper | compiled-stage1 should eventually bypass the legacy helper once a new front-door exists | helper still required for bootstrap/compat |

## Inventory Findings

- `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs`
  - current contract is shared `emit_object(mir_json_string) -> object path`.
  - this file now owns MIR(JSON) version patching, trace emission, and the explicit helper call on behalf of both plugin-loader and MIR-interpreter adapter paths.
  - direct retarget requires either a root-first adapter that preserves this contract or whole-helper retirement for that lane.
  - watch verdict: not demotable now; this is the correct keep chokepoint until a Rust-side replacement exists.
- `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs`
  - current contract is `compile_obj(mir_path) -> object path`.
  - this is a compiled-stage1/bootstrap surrogate; it reads MIR(JSON) from disk and cannot cleanly jump to the root-first route without moving the owner upward into `.hako`.
  - watch verdict: not demotable now; this stays archive-later until compiled-stage1 has a cleaner front door.

## Upstream Cleanup Targets

- `compat_codegen_receiver.rs`
  - cleanup target is contract-preserving replacement, not adapter split again.
  - current upstream owners are the thin MIR-interpreter and plugin-loader adapters:
    - `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs`
    - `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
  - daily callers should continue to stop at `LlvmBackendBox`; this chokepoint remains compat-only.
- `compat/llvm_backend_surrogate.rs`
  - cleanup target is the compiled-stage1/module-dispatch caller set, not the surrogate file first.
  - current upstream owner is `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` via `try_dispatch(...)`, with compat/proof callers on `selfhost.shared.backend.llvm_backend.{compile_obj,link_exe}`.

## `CodegenBridgeBox.emit_object_args(...)` Caller Inventory

Direct code callers currently in tree:

| Caller | Bucket | Note |
| --- | --- | --- |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | archive-later compat/proof | provider-first stub / canary-only surface; not a daily owner |

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
  - owner-surface arms and compat/proof C-ABI arms are now grouped explicitly in the file.
  - the gated compat codegen arm no longer calls `CodegenBridgeBox.emit_object_args(...)`; it now hydrates MIR(JSON) and calls `LlvmBackendBox.compile_obj_root(root, null)`.
  - the old `phase251` lowering canaries are now quarantined under `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/` because they are inactive hard-skips, not active suite coverage.
  - cleanup target: keep this stub explicit until an exact lowering replacement exists, then retire it after the bridge goes archive-only.
- `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
  - explicit proof/example caller materialized by the wrapper onto `vm-hako`.
  - not a daily route and not a current owner.
  - direct invoker is `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`.
  - the shell wrapper is transport only; the `.hako` file is the actual proof/example caller surface.
  - the payload now proves `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)` plus `LlvmBackendBox.link_exe(...)`.
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
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | archived proof-only coverage on the legacy emit/link lane | none | keep as replay evidence while `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)` remains archive-later; replay bundle is `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt` |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | archived proof-only lowering evidence for the legacy extern name | none | keep as quarantine evidence until a root-first selfhost lowering proof exists; replay bundle is `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt` |

## Archive Sequencing Matrix

| Surface | Current role | Replacement proof status | Sequence |
| --- | --- | --- | --- |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | archived explicit legacy emit/link proof | exact root-first replacements are green in `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_{ternary_collect,map_set_size}_runtime_proof.sh`; manual replay now lives in `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/run_all.sh` | archived; keep only as replay evidence |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` + `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | historical compat selfhost wrapper proof | wrapper now runs on `vm-hako`, but it still demonstrates the provider stop-line rather than the pure owner lane; owner-lane evidence stays `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh` | archive-later until the provider caller is demoted |
| `lang/src/vm/hakorune-vm/extern_provider.hako` + `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | legacy extern lowering proof with archived quarantine canaries | no root-first selfhost lowering proof is pinned yet | keep `extern_provider.hako` until a root-first lowering proof exists; archived `phase251` pair remains evidence only |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` + `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_{compare_branch,canary,const42}_canary_vm.sh` | provider-first llvmlite proof/canary surface | no root-first llvmlite provider proof replaces this exact surface | keep until llvmlite proof demand disappears or moves to archive |

- the three `phase2044` llvmlite canaries are live through the dedicated suite manifest `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt` and still match integration-profile discovery filters.
- the archived `phase2111` + `phase251` evidence now share the replay bundle `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt`.

## Compat Selfhost Wrapper Replacement Contract

The current stop-line for the compat selfhost wrapper is a drop-in shell/payload contract, not just backend success.

| Surface | Contract |
| --- | --- |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | positional CLI stays `<json_file_or_-'stdin'> [exe_out]` |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | required env stays `NYASH_LLVM_USE_CAPI=1`, `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`, `HAKO_CAPI_PURE=1`; `HAKO_CAPI_TM=1` stays optional |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | wrapper must export `_MIR_JSON` and `_EXE_OUT`, print the produced exe path, run that exe, and return the exe rc |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | failure codes stay explicit: usage `2`, env mismatch `3`, exe missing `4`, driver missing `5` |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | payload is a template materialized by the wrapper with MIR JSON and exe path literals |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | payload now proves `compile_obj_provider_stopline(...) -> link_exe(...)`, prints exe path, and returns `0/1/2/3` as today |

## Compat Selfhost Wrapper Proof Gap

The pinned root-first proof candidate proves the new owner lane, but not the current drop-in wrapper contract.

| Checkpoint | Compat wrapper today | Root-first candidate today | Gap |
| --- | --- | --- | --- |
| entrypoint | shell wrapper materializes a payload template and runs `target/release/hakorune --backend vm-hako` | temporary `.hako` file run by `target/release/hakorune --backend vm-hako` | wrapper still owns CLI/stdout/rc, candidate is harness-only |
| payload route | `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)` + `LlvmBackendBox.link_exe(...)` | `LlvmBackendBox.compile_obj(...)` + `link_exe(...)` | provider stop-line is still in the compat wrapper path |
| input contract | MIR JSON file or stdin via `_MIR_JSON` | fixed fixture file path | not CLI-compatible |
| output contract | print exe path, run exe, return exe rc | smoke harness checks exe creation and execution | wrapper stdout/rc contract is not proven |
| env contract | requires `HAKO_CAPI_PURE=1` plus C-ABI/CAPI guards | only C-ABI/CAPI guards are pinned in the proof | env contract is narrower |
| verdict | archive-later stop-line | valid owner-lane proof | not drop-in replaceable yet |

## Extern Provider Replacement Contract

The `extern_provider` stop-line is not the shell-wrapper contract. It is the gated compat codegen surface on the Hako-side provider path.

| Surface | Contract |
| --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | runtime owner surface keeps `env.get` and `env.console.*`; compat codegen/mirbuilder is delegated, not owned |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` remains the explicit gate for compat codegen/mirbuilder arms |
| `lang/src/vm/hakorune-vm/compat_codegen_extern_provider.hako` | keeps `env.codegen.emit_object` and `env.mirbuilder.emit` on the current compat/proof route while W4 caller demotion is still incomplete |
| `lang/src/vm/hakorune-vm/compat_codegen_extern_provider.hako` | gate-out behavior stays stub-like (`\"\"` / `null`), not fail-open into a new daily route |
| exact replacement requirement | preserve the gated surface name and current `args -> object path / null` behavior until one root-first lowering proof is accepted |

## Minimal Root-First Lowering Proof Target

`99O2` should not try to replace the shell compat wrapper. The smallest exact proof target is the Hako-side provider surface itself.

| Item | Target |
| --- | --- |
| proof lane | one `vm-hako` smoke that reaches `env.codegen.emit_object` through the current compat provider surface |
| env | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` with the current C-API / CAPI owner-lane guards |
| success | root-first route produces an object path and links/runs an executable |
| implemented lane | `tools/smokes/v2/profiles/integration/compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm.sh` |
| reuse | `phase29ck_vmhako_llvm_backend_runtime_proof.sh` can be reused as owner-lane evidence, but it is not itself the exact stop-line proof |
| non-goal | do not prove wrapper CLI/env/stdout parity here; that is the compat selfhost wrapper contract, not the extern-provider contract |

## Root-First Proof Candidate Matrix

This matrix is only about the current `29x-98` stop-line surfaces. It does not reopen daily ownership.

| Surface | Candidate root-first proof | What it proves today | Drop-in replacement? | Blocker |
| --- | --- | --- | --- | --- |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` + `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh` | `.hako VM -> LlvmBackendBox -> C-API -> exe` works on the vm-hako owner lane | no | the compat wrapper now runs on `vm-hako`, but it still proves the provider stop-line through `compile_obj_provider_stopline(...)` rather than the pure owner lane |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `tools/smokes/v2/profiles/integration/compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm.sh` | exact `vm-hako` proof now shows the gated provider surface can produce an object path and linked executable | not yet | Rust-side chokepoint collapse and final helper retirement are still pending |
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
  - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` now materializes the payload onto `vm-hako`; it keeps the shell contract but no longer demonstrates the old bridge route directly.
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
  - `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` and `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` now forward into `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs`; they are not direct helper callers anymore.
- cleanup rule
  - do not start by deleting Rust dispatch files.
  - first remove the remaining explicit helper callers or replace them with root-first/evidence-safe equivalents.
  - after the upstream caller inventory reaches zero, collapse the Rust dispatch residues and then delete `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)`.

## Ordered Investigation Queue

1. `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs`
   - keep chokepoint; still owns the explicit helper call.
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs`
   - archive-later surrogate; still owns the compiled-stage1 helper call.
3. archive sequencing for the remaining proof/compat caller surfaces
4. Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`) only after upstream caller inventory reaches zero

## Retirement Order

1. keep `compat_codegen_receiver.rs` as the explicit Rust compat caller until its MIR(JSON) contract is replaced by a root-first/evidence-safe equivalent.
2. keep `compat/llvm_backend_surrogate.rs` as archive-later until the compiled-stage1/bootstrap path has a cleaner front door than `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)`.
3. once the explicit helper caller inventory reaches zero, collapse the Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`).
4. only when all caller surfaces are gone: delete `src/host_providers/llvm_codegen/legacy_mir_front_door.rs::compile_object_from_legacy_mir_json(...)`.

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
5. keep the explicit helper archive-later until the caller set reaches zero.
6. push new daily callers through `LlvmBackendBox -> env.codegen.compile_ll_text(...) -> env.codegen.link_object(...)`, not through `env.codegen.emit_object`.
7. when the caller set reaches zero, delete `compile_object_from_legacy_mir_json(...)`, then collapse the Rust dispatch residues and phase docs.

## Delete Condition

- all explicit callers have moved to root-first or daily route surfaces.
- compare/archive proof coverage is preserved in archive assets.
- the generic `emit_object_from_mir_json(...)` symbol/export no longer appears in code-side caller inventory.
- `compile_object_from_legacy_mir_json(...)` no longer appears in code-side caller inventory.

## Non-Goals

- no new compare bridge.
- no reopening of file-based `mir_json_file_to_object(...)`.
- no daily caller growth.
