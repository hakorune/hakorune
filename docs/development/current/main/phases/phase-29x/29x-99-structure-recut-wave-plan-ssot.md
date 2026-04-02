---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: `phase-29x backend owner cutover prep` の beauty-first cleanup を、docs-first の大タスクと micro-task に分解する正本。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
---

# 29x-99 Structure Recut Wave Plan

## Goal

- `owner / compat / proof / archive` を path と module 名でも真実化する。
- helper deletion を急がず、physical layout と API 名の misleading さを先に減らす。
- `29x-98` の stop-line を崩さず、docs-first で re-cut 順を固定する。

## Fixed Rules

- `29x-98` owns delete-readiness and stop-line.
- `29x-99` owns beauty-first path / namespace / filesystem recut planning.
- docs-first: first lock target shapes and move order in docs, then move files, then change behavior.
- file move / rename / thin shim is preferred before helper deletion.
- do not use phase numbers as the long-term physical home for live code or live proof if a semantic bucket is available.
- no helper deletion is allowed while the current exact root-first replacement proof is still missing.

## Macro Waves

| Wave | Status | Goal | Why it exists now |
| --- | --- | --- | --- |
| `W1 docs-first path-truth pass` | landed | lock final buckets, names, and move order before code moves | current repo truth is semantically cleaner than its paths |
| `W2 mixed-file split pass` | landed | split files that still mix owner and compat/proof roles | biggest readability gain per file touched |
| `W3 smoke/proof filesystem recut` | landed | move live proof and archive evidence into semantic homes | phase-number directories still hide meaning |
| `W4 Hako-side caller drain prep` | landed | replace direct `.hako` callers with exact root-first proofs | one exact proof is green; direct caller demotion is complete |
| `W5 Rust compat receiver collapse` | active | reduce `env.codegen.*` legacy receivers to one compat chokepoint | current receiver logic is spread across multiple Rust files |
| `W6 final delete/archive sweep` | pending | delete legacy helper fronts and leave archive evidence only | last sweep after caller inventory reaches zero |

## Current Focus

- active macro wave: `W5 Rust compat receiver collapse`
- active micro-task:
  - `99R2 align tracing / observability at the chokepoint`
- next queued micro-task:
  - `99S1 move surrogate caller to compat/evidence adapter home`
- docs-for-structure lock remains in `99E` / `99F` and their detail rows.
- code reduction remains partially proof-gated by `29x-98`: `extern_provider.hako` now has one exact proof lane, the compat selfhost wrapper stack has been materialized onto `vm-hako`, and the Hako-side bridge is now archive-only; the next collapse is on the Rust receiver side.
  - `99E3` is absorbed into `W5` `99Q / 99R` Rust compat receiver collapse.
  - `99E4` is absorbed into `W2` `99I` owner API / evidence adapter split.

## Review Intake

The 2026-04-02 beauty-first review is adopted as a path-truth check, not as a new competing plan.

| Review point | Verdict | Task home | Read as |
| --- | --- | --- | --- |
| compat selfhost payload home was duplicated | stale-in-review | `99F1` landed | wrapper now points at `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` |
| `extern_provider.hako` still mixed runtime owner and compat codegen | landed | `99G` landed | runtime owner and compat codegen stub are already split |
| `llvm_codegen.rs` still mixed thin boundary and legacy MIR front door | landed | `99H` landed | legacy front door now lives in `legacy_mir_front_door.rs` |
| `phase2044` / `phase2120` were still phase-number semantic homes | landed | `99K-99M` landed | live proof/archive buckets now have semantic homes |
| `LlvmBackendBox` still reads as both owner facade and file/evidence entry | adopt-next | `99I` follow-up after `W4` | split is partial; owner facade slimming remains open |
| Rust legacy/codegen receivers still read as a spread surface | adopt-next | `99Q1-99S1` | receiver-body split landed; one chokepoint collapse remains open |
| `CodegenBridgeBox.emit_object_args(...)` / `emit_object_from_mir_json(...)` should die only after path truth and proof replacement exist | confirmed | `29x-98` + `99N1-99P3` | stop-line stays proof-gated |

### 2026-04-02 Re-Cut Proposal Mapping

This table maps the later beauty-first re-cut proposal onto the current tree so we do not reopen already-landed slices.

| Proposal item | Verdict | Task home | Read as |
| --- | --- | --- | --- |
| new compat bridge for `MIR(JSON text) -> root-first compile` | stale-in-review | `99P2` + `llvm_emit_compat_box` landed | bridge role is already absorbed by current compat root-first callers; no new bridge file is required now |
| compat selfhost payload root-first conversion | landed | `99P1` | payload now proves the provider stop-line through the evidence adapter on `vm-hako` |
| `compat_codegen_extern_provider.hako` root-first conversion | landed | `99P2` | gated compat stub now root-hydrates MIR(JSON) and calls `LlvmBackendBox.compile_obj_root(...)` |
| truthify legacy emit bridge naming / shim role | adopt-next | `99T` | compat implementation name should stop presenting `CodegenBridgeBox` as the primary truth |
| one Rust compat-codegen chokepoint | landed and advancing | `99R2` | canonical shared receiver exists; next work is tracing / observability alignment |
| surrogate move to compat/evidence home | adopt-next | `99S1` | surrogate still lives under an owner-looking path and should move after tracing is aligned |

## Micro Tasks

### W1. Docs-First Path-Truth Pass

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99A` | landed | `phase2044` semantic bucket docs/manifest lock | `llvmlite trio = final live keep bucket`; other groups are bucket-runner only |
| `99B` | landed | `phase2120` keep/historical docs + suite split lock | `compat/pure-keep` / `archive/pure-historical` are canonical |
| `99C` | landed | compat selfhost stack wording lock | `payload -> transport wrapper -> pack orchestrator` is fixed across docs |
| `99D` | landed | direct caller vs wrapper inventory lock | `29x-98` keeps direct callers and wrappers separate |
| `99E` | landed | split-target inventory lock | target split inventory exists for `extern_provider.hako`, `llvm_codegen.rs`, `LlvmBackendBox`, `CodegenBridgeBox`, `LLVMEmitBox`, and `tools/selfhost` |
| `99F` | landed | file-move / shim order lock | docs say what moves first, what gets a thin shim, and what must not change behavior in the same slice |

#### `99E` split-target inventory

| Current surface | Preferred split target | Suggested home | Read as |
| --- | --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `runtime_extern_provider` + `compat_codegen_extern_provider` | `lang/src/vm/hakorune-vm/` | runtime owner vs compat stub |
| `src/host_providers/llvm_codegen.rs` | thin tool boundary + legacy MIR front door | `src/host_providers/llvm_codegen/` or `src/compat/codegen/` | daily tool seam vs legacy knot |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | `lang/src/shared/backend/` | canonical owner vs evidence entry |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | legacy shim only | `lang/src/compat/codegen/legacy_emit_object_bridge_box.hako` | shim path only; compat bridge is no longer owner-looking |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | legacy shim only | `lang/src/compat/codegen/llvm_emit_compat_box.hako` | shim path only; compat/proof box is no longer owner-looking |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | transport wrapper only | `tools/compat/legacy-codegen/` | wrapper/orchestrator, not direct caller |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | compat payload only | `tools/compat/legacy-codegen/` | proof/example payload |

#### `99F` file-move / shim order

1. docs lock target names and target homes
2. move payload / wrapper / orchestrator files first, keep thin shims or re-exports
3. split mixed owner/compat files next, keeping old entrypoints as thin wrappers only
4. verify discovery, runner wiring, and import references before behavior changes
5. delete old entrypoints only after exact replacement proof or caller inventory zero

### W2. Mixed-File Split Pass

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99G` | landed | split `extern_provider.hako` into runtime owner surface and compat codegen shim | owner arms and compat/proof arms no longer live in one file |
| `99H` | landed | split `src/host_providers/llvm_codegen.rs` into thin tool boundary and legacy MIR front door | `ll_text_to_object` no longer shares a home with `emit_object_from_mir_json(...)` |
| `99I` | landed | split `LlvmBackendBox` owner API and evidence adapter | canonical MIR/root-first APIs and JSON/evidence entrypoints are no longer mixed |
| `99J` | landed | move `CodegenBridgeBox` and `LLVMEmitBox` out of owner-looking paths | compat/proof surfaces stop living under misleading owner paths |

- `99I` follow-up remains queued after `W4`:
  - slim `LlvmBackendBox` until `compile_obj(json_path)` no longer makes the owner facade look like the evidence/file entry home
  - keep that rename/surface pass separate from current proof-gated caller-drain work

### W3. Smoke/Proof Filesystem Recut

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99G1` | active | suites / directory semantic recut | `phase2044` / `phase2120` / archive suites read as semantic homes instead of phase-number homes |
| `99K` | landed | physically recut `phase2044` into semantic buckets | llvmlite keep, hako-primary-no-fallback, and mirbuilder-provider stop sharing one live directory |
| `99L` | landed | physically recut `phase2120` into semantic buckets | compat/pure-keep / pure-historical / vm-adapter-legacy / native-reference get separate homes |
| `99M` | landed | bundle archive proof surfaces semantically | `phase2111` + `phase251` replay evidence can be read as one archive bundle |

#### `99K-99M` filesystem recut inventory

| Current home | Proposed home | Read as |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/codegen_provider_llvmlite_*` | `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/` | final live keep bucket |
| `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/hako_primary_no_fallback_*` | `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/` | separate core-exec proof bucket |
| `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/mirbuilder_provider_*` | `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/` | separate mirbuilder-provider proof bucket |
| `tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh` + `...loop_count...` | `tools/smokes/v2/profiles/integration/compat/pure-keep/` | active keep pins |
| `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_*` archive-backed pins | `tools/smokes/v2/profiles/archive/pure-historical/` | archive-backed replay evidence |
| `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/s3_vm_adapter_*` | `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/` | legacy VM adapter cluster |
| `tools/smokes/v2/profiles/integration/proof/native-reference/native_backend_*` | `tools/smokes/v2/profiles/integration/proof/native-reference/` | native reference canaries |
| `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/*` | `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/` | archive replay bundle for both legacy emit/link canaries and archived selfhost lowering probes |

### W4. Hako-Side Caller Drain Prep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99N1` | landed | lock compat selfhost wrapper replacement contract | wrapper input/output/env contract is written down as the drop-in target |
| `99N2` | landed | lock compat payload invariants | `hako_llvm_selfhost_driver.hako` required behavior and allowed drift are explicit |
| `99N3` | landed | map current root-first proof gap for compat selfhost wrapper | `phase29ck_vmhako_llvm_backend_runtime_proof` is compared against the drop-in contract line by line |
| `99O1` | landed | lock `extern_provider.hako` replacement contract | current compat codegen stub contract is explicit enough to judge replacement |
| `99O2` | landed | pin minimal root-first lowering proof target | one exact proof fixture/lane is named for the `extern_provider.hako` replacement |
| `99O3` | landed | lock direct-caller demotion prerequisites | preconditions for removing `.hako` direct callers are explicit and ordered |
| `99O4` | landed | implement minimal root-first lowering proof smoke | one `vm-hako` proof is green for the `extern_provider` stop-line surface |
| `99P1` | landed | demote compat selfhost payload direct caller | `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` is materialized onto `vm-hako` and no longer needs `CodegenBridgeBox.emit_object_args(...)` |
| `99P2` | landed | demote `extern_provider.hako` compat codegen caller | `compat_codegen_extern_provider.hako` root-hydrates MIR(JSON) and calls `LlvmBackendBox.compile_obj_root(...)` |
| `99P3` | landed | make `CodegenBridgeBox.emit_object_args(...)` archive-only | live Hako direct callers are zero |

#### `99N1` compat selfhost wrapper replacement contract

| Surface | Contract |
| --- | --- |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | positional CLI stays `<json_file_or_-'stdin'> [exe_out]` |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | required env guard stays `NYASH_LLVM_USE_CAPI=1`, `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`, `HAKO_CAPI_PURE=1`; `HAKO_CAPI_TM=1` stays optional |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | wrapper must still export `_MIR_JSON` and `_EXE_OUT` before invoking the payload |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | success contract stays `stdout prints exe path`, then the produced executable is run, and wrapper exit code matches executable exit code |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | failure contract stays explicit: usage `2`, env mismatch `3`, exe missing `4`, driver missing `5` |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | payload template is materialized by the wrapper with MIR JSON and exe path literals; wrapper CLI/env contract stays unchanged |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | payload success contract stays `emit object -> link exe -> print exe path -> return 0` |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | payload failure contract stays `_MIR_JSON missing -> 1`, `emit fail -> 2`, `link fail -> 3` |

#### `99N2` compat payload invariants

| Invariant | Why it stays fixed now |
| --- | --- |
| `hako_llvm_selfhost_driver.hako` remains a proof/example payload, not a daily owner | keep stop-line semantics explicit |
| payload remains the direct `.hako` caller surface; wrapper remains transport only | direct caller vs wrapper inventory must stay readable in `29x-98` |
| payload no longer calls `CodegenBridgeBox.emit_object_args(...)` directly | `99P1` removes the direct bridge caller while preserving the shell contract |
| wrapper continues to resolve the payload from `tools/compat/legacy-codegen/` only | do not re-open the old `tools/selfhost/examples/` duplicate home |
| `run_compat_pure_pack.sh` remains an orchestrator above the canonical wrapper, not a proof owner | sequencing must stay stable while replacement proof is missing |

#### `99N3` root-first proof gap map for compat selfhost wrapper

| Checkpoint | Current compat wrapper lane | Current root-first proof candidate | Gap |
| --- | --- | --- | --- |
| entrypoint | shell wrapper materializes a payload template and runs `target/release/hakorune --backend vm-hako` | temporary `.hako` source run by `target/release/hakorune --backend vm-hako` | wrapper still proves shell CLI/stdout/rc, candidate is harness-only |
| payload | `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)` / `LlvmBackendBox.link_exe(...)` | direct `LlvmBackendBox.compile_obj(...)` / `link_exe(...)` | provider stop-line is still in the compat wrapper path |
| input contract | MIR JSON file or stdin, exported as `_MIR_JSON` | fixed MIR fixture path in the proof script | no drop-in CLI/input parity yet |
| output contract | wrapper prints exe path, then runs the exe and returns the exe rc | proof script asserts exe exists and runs it inside the smoke harness | shell-wrapper stdout/rc contract is not proven |
| env contract | requires `HAKO_CAPI_PURE=1` in addition to C-ABI/CAPI guards | candidate proof only pins `NYASH_LLVM_USE_CAPI=1` and `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` | env surface is narrower in the proof |
| replacement verdict | archive-later compat wrapper | valid root-first owner-lane proof | not a drop-in replacement today |

#### `99O1` extern-provider replacement contract

| Surface | Contract |
| --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | runtime owner surface keeps `env.get` and `env.console.*` behavior unchanged while compat codegen/mirbuilder arms stay explicitly gated |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` remains the explicit gate that enables compat codegen/mirbuilder delegation |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `env.codegen.emit_object` and `env.mirbuilder.emit` are not owner arms; they remain explicit compat/proof delegation surfaces |
| `lang/src/vm/hakorune-vm/compat_codegen_extern_provider.hako` | accepts `name` plus `args`, and keeps `env.codegen.emit_object` on the legacy bridge route while the stop-line remains active |
| `lang/src/vm/hakorune-vm/compat_codegen_extern_provider.hako` | gate-out behavior stays stub-like (`""` or `null`), not fail-open into a new daily route |
| `lang/src/vm/hakorune-vm/compat_codegen_extern_provider.hako` | exact replacement must preserve the gated `env.codegen.emit_object` surface name and its `args -> object path / null` behavior until the proof is accepted |

#### `99O2` minimal root-first lowering proof target

| Item | Target |
| --- | --- |
| proof lane shape | a small `vm-hako` smoke that directly exercises the `extern_provider` surface, not the shell compat wrapper |
| fixture shape | one `.hako` fixture that reaches `env.codegen.emit_object` through the current compat provider surface under `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` |
| success contract | root-first route produces an object path and links/runs an executable without reopening `CodegenBridgeBox.emit_object_args(...)` as a daily owner |
| relation to `phase29ck_vmhako_llvm_backend_runtime_proof.sh` | reuse the root-first owner lane as implementation evidence, but do not treat it as a drop-in wrapper proof |
| non-goal | do not prove wrapper CLI/env/stdout parity here; that belongs to `99N1-99N3` |

#### `99O3` direct-caller demotion prerequisites

| Prerequisite | Why it must be true first |
| --- | --- |
| `99N1-99N3` are landed | compat selfhost wrapper contract/gap must be fixed before any demotion decision |
| `99O1` is landed | the `extern_provider` stop-line must be explicit before it can be replaced |
| `99O2` is landed with one named proof target | there must be one exact proof lane to gate the demotion work |
| direct caller inventory remains explicit in `29x-98` | do not blur direct callers with wrappers/orchestrators during demotion |
| no helper deletion | `CodegenBridgeBox.emit_object_args(...)` and `emit_object_from_mir_json(...)` stay live until `99Q1-99S1` make the Rust chokepoint explicit |

#### `99O4` minimal root-first lowering proof implementation target

| Item | Implementation target |
| --- | --- |
| fixture | one small `.hako` fixture that reaches `env.codegen.emit_object` through the current compat provider surface |
| runner lane | `vm-hako` |
| env gate | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` plus the current C-API owner-lane guards |
| evidence shape | prove object-path creation and linked executable creation/run |
| implemented lane | `tools/smokes/v2/profiles/integration/compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm.sh` |
| implementation note | `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)` is the proof-only adapter that keeps the owner facade thin while the stop-line still exists |
| non-goal | do not replace the shell compat wrapper; do not prove wrapper CLI/stdout parity here |

### W5. Rust Compat Receiver Collapse

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99Q1` | landed | lock one Rust compat-codegen chokepoint contract | one receiver module owns the legacy codegen accept path contract |
| `99Q2` | landed | reduce MirInterpreter receivers to thin adapters | `hostbridge.rs` and `loader_cold.rs` only forward into the chokepoint |
| `99Q3` | landed | reduce plugin-loader receiver to a thin adapter | `extern_functions.rs` only forwards into the chokepoint |
| `99R1` | landed | collapse route ownership into one compat namespace | route ownership for legacy codegen entry is visible in one Rust home |
| `99R2` | active | align tracing / observability at the chokepoint | legacy codegen acceptance is observable in one place |
| `99S1` | pending-after-R2 | move surrogate caller to compat/evidence adapter home | `llvm_backend_surrogate.rs` no longer extends the old helper from an owner-looking surface |

- W5 prep is now partially landed:
  - MirInterpreter codegen receiver bodies live in `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs`
  - plugin loader `env.codegen` now enters `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` directly
  - shared Rust compat receiver now lives in `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs`
  - `hostbridge.rs` / `loader_cold.rs` now forward `env.codegen.*` into adapter-stage homes only
  - `extern_functions.rs` no longer owns direct codegen behavior
  - route ownership is now collapsed into one explicit compat-codegen namespace, so `99R2` is now the active micro-task

#### `99Q1` one explicit Rust compat-codegen chokepoint contract

| Contract surface | Lock |
| --- | --- |
| canonical target home | `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` |
| owned legacy accept path | `env.codegen.emit_object`, `env.codegen.compile_ll_text`, `env.codegen.link_object` |
| MirInterpreter adapters | `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` and `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` become thin forwarders only |
| plugin-loader adapter | `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` keeps routing `env.codegen` into the chokepoint only |
| interim split bodies | `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs` is the remaining adapter-stage home around the shared receiver |
| lane policy | `src/backend/mir_interpreter/handlers/extern_provider/lane.rs` stays classification-only; it does not become the codegen owner |
| non-goal | do not mix `env.mirbuilder.emit`, helper deletion, or route-policy widening into `99Q1` |

- acceptance for `99Q1`:
  - one canonical Rust compat-codegen receiver home is named
  - `hostbridge.rs`, `loader_cold.rs`, and `extern_functions.rs` are explicitly treated as adapter surfaces, not receiver homes
  - the next steps (`99Q2-99R2`) can reduce files without reopening the contract debate

#### `99R1` collapse route ownership into one compat namespace

| Route surface | Current state | Target read |
| --- | --- | --- |
| `src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs` | canonical shared receiver | explicit compat-codegen namespace owner |
| `src/backend/mir_interpreter/handlers/extern_provider/codegen.rs` | adapter-stage home for `ValueId -> String/Path` translation | no route policy; forwarding adapter only |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | forwarding-only surface after `99Q2` | never owns `env.codegen` behavior |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | forwarding-only surface after `99Q2` | never owns `env.codegen` behavior |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | forwarding-only surface after `99Q3` | never owns `env.codegen` behavior |

- acceptance for `99R1`:
  - route policy for legacy codegen acceptance is readable from `compat_codegen_receiver.rs`
  - plugin-loader route ownership no longer depends on a separate `enabled/codegen.rs`
  - adapter-stage files are visibly translation-only and do not compete as owners
  - the next step (`99R2`) can align tracing without reopening namespace ownership

### W6. Final Delete/Archive Sweep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99T` | pending-after-S1 | truthify legacy emit bridge naming and keep shim-only export | compat implementation no longer presents `CodegenBridgeBox` as the primary truth |
| `99U` | pending-after-T | delete `CodegenBridgeBox.emit_object_args(...)` | no live direct caller remains |
| `99V` | pending-after-U | delete `emit_object_from_mir_json(...)` and sync final compat/archive residue | caller inventory is zero and `owner / compat / proof / archive` reads cleanly in tree and docs |

## Split Targets

| Current surface | Target split / home | Read as |
| --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `runtime_extern_provider` + `compat_codegen_extern_provider` | runtime owner vs compat stub |
| `src/host_providers/llvm_codegen.rs` | thin tool boundary + legacy MIR/codegen compat module | daily boundary vs legacy knot |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | canonical root-first owner vs evidence-only entry |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | compat/codegen namespace | not a daily owner surface |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | compat/codegen namespace | compat/proof box, not owner |
| `tools/compat/legacy-codegen/**` | compat/legacy-codegen payload / wrapper / orchestrator buckets | stop mixing selfhost core and legacy-codegen proof |
| `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/**` + `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/**` + `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/**` | semantic proof buckets | stop using phase number as live semantic home |
| `tools/smokes/v2/profiles/integration/compat/pure-keep/**` + `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/**` + `tools/smokes/v2/profiles/integration/proof/native-reference/**` + `tools/smokes/v2/profiles/archive/pure-historical/**` | semantic proof + archive buckets | same as above |

## 99E Split Target Inventory

This table freezes the intended destination before any path move happens.

| Surface | From | To | Shim needed | Notes |
| --- | --- | --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | runtime owner + compat codegen stub | `runtime_extern_provider.hako` + `compat_codegen_extern_provider.hako` | yes, thin re-export during transition | this is the main mixed-file split target |
| `src/host_providers/llvm_codegen.rs` | thin boundary + legacy MIR front door | `src/host_providers/llvm_tool_boundary.rs` + `src/compat/codegen/legacy_mir_codegen.rs` | yes, thin compat bridge | `ll_text_to_object` and `emit_object_from_mir_json(...)` should stop sharing one home |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | `llvm_backend_box.hako` + `llvm_backend_evidence_adapter_box.hako` | maybe, if caller imports need a bridge | keep the owner spine readable |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | legacy shim in owner-looking path | `compat/codegen/legacy_emit_object_bridge_box.hako` | yes, re-export only | path should stop implying daily ownership |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | legacy shim in owner-looking path | `compat/codegen/llvm_emit_compat_box.hako` | yes, re-export only | keep the box explicit as compat/proof |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | proof/example payload | `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | maybe, if wrapper path remains stable | keep selfhost core and legacy-codegen proof separate |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | transport wrapper | `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | no, path rename only | wrapper/orchestrator should read as compat only |
| `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | pack orchestrator | `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | no, path rename only | keep pack orchestration out of selfhost core |

## Move-Order Rule

1. lock target names and roles in docs
2. move paths with thin shims or re-exports only
3. verify discovery / runner / import references
4. then change behavior or delete old entrypoints

Do not combine `move + semantic change + helper deletion` in one slice.

## 99F Move / Shim Order

1. move the payload and wrapper paths first
   - `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
   - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
   - `tools/compat/legacy-codegen/run_compat_pure_pack.sh`
2. split mixed owner/compat modules next with thin shims only
   - `lang/src/vm/hakorune-vm/extern_provider.hako`
   - `src/host_providers/llvm_codegen.rs`
   - `lang/src/shared/backend/llvm_backend_box.hako`
3. move proof boxes out of owner-looking paths with re-exports only if needed
   - `lang/src/shared/host_bridge/codegen_bridge_box.hako`
   - `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
4. keep behavior unchanged until discovery / runner references are updated
5. delete old entrypoints only after caller inventory and archive evidence are both explicit

## Stop-Line Dependencies

| Surface | Current state | What unblocks it |
| --- | --- | --- |
| `extern_provider.hako` compat codegen arm | keep | exact root-first selfhost lowering proof |
| compat selfhost wrapper stack | archive-later | exact root-first drop-in proof or explicit whole-stack retirement |
| `CodegenBridgeBox.emit_object_args(...)` | keep | direct Hako caller inventory reaches zero |
| `emit_object_from_mir_json(...)` | archive-later | direct caller inventory reaches zero |

## Acceptance

- `CURRENT_TASK.md`, `10-Now.md`, `15-Workstream-Map.md`, and `05-Restart-Quick-Resume.md` point to this wave plan without taking ownership away from `29x-98`.
- `phase-29x/README.md` and `29x-91-task-board.md` show both macro waves and micro tasks.
- `29x-98` remains the delete-readiness owner; `29x-99` remains the path-truth / recut owner.
- current active work is readable as:
  - macro: `W5 Rust compat receiver collapse`
  - micro: `99R2 align tracing / observability at the chokepoint`
  - next: `99S1 move surrogate caller to compat/evidence adapter home`
  - detail: `99N1-99N3` landed for the compat wrapper stack, `99O1-99O4` landed for the extern-provider stop-line and exact proof lane, `99P1-99P3` landed for the Hako-side caller drain, and `99Q1-99S1` now collapse the Rust accept path to one compat chokepoint
