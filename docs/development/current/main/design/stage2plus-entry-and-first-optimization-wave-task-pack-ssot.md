---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `stage1 -> stage2-mainline` entry gate と、stage2-mainline 初回最適化 wave の fixed order を task-pack として固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/20-Decisions.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
---

# Stage2+ Entry And First Optimization Wave Task Pack (SSOT)

## Goal

- `stage1 bridge/proof` から `stage2-mainline daily mainline` へ入るための exact gate を 1 枚で読む。
- collection owner stop-line と stage2-mainline entry、first optimization wave を同じ fixed order に束ねる。
- broad optimization / Rune backend-active 化を混ぜず、first wave を `route/perf only` に固定する。
- broad substrate redesign はこの task pack に含めず、`K2` substrate era へ分離する。

## Fixed Reading

- stage axis:
  - `stage0` = Rust bootstrap / recovery keep
  - `stage1` = bridge / proof line
- `stage2-mainline` = daily `.hako` mainline / current distribution lane
- `stage2+` = umbrella / end-state distribution target
- owner/substrate axis:
  - `.hako` owns meaning / policy / route / acceptance / control
  - `.inc` is thin shim / boundary artifact only
  - native keeps metal / substrate (`ABI/alloc/GC/TLS/atomic/backend emission`)
- collection wave reading:
  - `Array -> Map -> RuntimeData cleanup` is already `done-enough` on the owner axis
  - treat those domains as regression packs during this task pack, not as the next owner rewrite
- replacement axis reading:
  - current task pack is post-`K1` regression/perf evidence
  - `K2-core RawArray` substrate replacement is a separate structural lane
  - `K2-wide` widening stays deferred beyond this pack
- first optimization wave reading:
  - `route/perf only`
  - mainline route is `.hako -> ny-llvmc(boundary) -> C ABI`
  - boundary contract is `llpath canonical emit`; current implementation uses `opt -passes=mem2reg` before `llc`
  - `HAKO_CAPI_TM` stays explicit bypass / compat-probe keep
  - Rune optimization metadata stays `parse/noop` in this wave

## Parent / Child Ownership

This task pack owns:

- exact order from stage1 exit gate to stage2-mainline entry
- fixed scope of the first optimization wave
- dashboard / pointer sync
- acceptance bundle across stage1, collection regression, and first-wave perf

Child SSOTs keep detailed contracts:

- `de-rust-stage-and-owner-axis-ssot.md`
  - stage/owner vocabulary
- `stage1-mir-authority-boundary-ssot.md`
  - stage1 canonical MIR authority exit gate
- `kernel-implementation-phase-plan-ssot.md`
  - collection-first kernel wave and stop-line reading
- `stage2-hako-owner-vs-inc-thin-shim-ssot.md`
  - `.hako authority / .inc thin shim / native keep` boundary
- `stage2-selfhost-and-hako-alloc-ssot.md`
  - stage2-mainline layering / artifact / distribution reading; `stage2+` remains the umbrella reading
- `perf-optimization-method-ssot.md`
  - measurement order and keep/reject method once the first wave is open
- `optimization-hints-contracts-intrinsic-ssot.md`
  - excluded lane in this pack; backend-active optimization metadata remains deferred

## Stage x Domain Handoff Matrix

| domain | `stage0` keep | `stage1` bridge / proof | `stage2-mainline` daily mainline |
| --- | --- | --- | --- |
| compiler | Rust bootstrap / recovery compiler keep | `.hako` canonical MIR authority; Rust bridge/materializer is transport-only proof lane | `.hako` compiler mainline |
| kernel | Rust runtime/kernel substrate keep | `Array -> Map -> RuntimeData cleanup` are regression packs; visible collection owner already sits on `.hako` | `.hako` kernel/runtime mainline |
| backend | compat / probe / bootstrap keep | `.hako -> ny-llvmc(boundary) -> C ABI` proof route; `.inc` stays transitional bridge/thin-shim space | `.hako` semantic owner with `.inc` thin shim only |
| substrate | native/Rust ABI + alloc + GC + handle + raw memory keep | explicit metal keep; no deeper substrate redesign before capability/manifest lock | native metal keep under `hako.abi / hako.value_repr / ownership-layout manifest` |

## Fixed Order

### 1. Vocabulary Sync

- sync all stage docs and dashboards to:
  - `stage0 keep / stage1 bridge+proof / stage2-mainline daily mainline / stage2+ umbrella`
- keep `stage2-mainline` and `stage2+` out of artifact-kind vocabulary
- keep standard distribution reading as `hakoruneup + self-contained release bundle`

### 2. Stage1 Exit Gate

Lock the following as prerequisites for stage2-mainline entry:

- `.hako` is the canonical MIR authority
- Rust `stage1_bridge` is thin materializer / transport only
- stage1-first identity route remains the default proof route
- stage1 success is not read as stage2-mainline completion

### 3. Collection Regression Freeze

- keep `Array`, `Map`, and `RuntimeData cleanup` at regression-pack status
- do not reopen owner migration in this pack unless a new exact blocker appears
- keep current Map micro work as evidence/regression only; do not elevate it into the next structural replacement milestone
- keep Array perf acceptance pinned to same-artifact compare against the current Rust baseline

### 4. Stage2-mainline Entry Lock

- lock stage2-mainline entry as:
  - `hako_kernel`
  - `.inc thin shim`
  - native metal keep
- boundary truth is:
  - `hako.abi`
  - `hako.value_repr`
  - ownership/layout manifest
- capability ladder docs must be locked before deeper substrate work reopens

### 5. First Optimization Wave

Allow only:

- route specialization
- hot-path residue reduction
- same-artifact perf proof
- thin-shim/perf observability required to judge the mainline lane

Keep the fixed order:

1. `leaf-proof micro`
2. `micro kilo`
3. `main kilo`

Do not include:

- Rune `Hint/Contract/IntrinsicCandidate` backend activation
- verifier / registry work
- new ABI surface
- broad generic optimizer widening
- deeper substrate redesign beyond the capability/manifest lock
- `K2-core RawArray` substrate replacement itself
- `K2-wide` widening / metal review

### 6. Deferred Lanes

- Rune/backend-active optimization metadata
- broad generic optimization beyond route/perf
- deeper native/substrate cut after stage2-mainline entry and first-wave proof

## Acceptance Bundle

### Final self-compile acceptance — queued, not yet proven

Decision (2026-09-05): implement this gate in the existing selfhost build and
identity harness after the unified resume order in
`selfhost-parser-mirbuilder-migration-order-ssot.md`. Existing G1 equality and
reduced CLI liveness are bootstrap evidence, not completion of this task.

Change: bind the final acceptance run to a pinned compiler source revision,
entrypoint, complete transitive source/import closure, build configuration,
Stage1 artifact hash, toolchain and expected program results. Store these in
the existing selfhost evidence owner, not a new task ledger.

Contract: Stage1 itself compiles that compiler closure through its parser and
MirBuilder into canonical MIR. Disable Rust source parsing, Program-to-MIR
lowering and compiler-policy delegation for this profile, including the
`nyash.stage1.emit_program_json_v0_h`, `emit_mir_from_source_v0_h` and
`emit_mir_from_program_json_v0_h` routes in
`crates/nyash_kernel/src/exports/stage1.rs` and the import substitutions in
`src/runner/json_v0_bridge/lowering/expr/call_ops.rs`. These are current
bootstrap dependencies, not evidence of a completed Hako compiler.
Generic published-MIR backend emission, linking, OS/runtime and ABI helpers
may remain native; they may not recover source/compiler meaning.

Done: build Stage2 from the MIR actually emitted by that Stage1 invocation,
record artifact provenance, then execute the generated Stage2 to compile the
pinned ordinary-source acceptance programs and run their artifacts with exact
expected output/exit. The positive run must pass with forbidden frontend
providers unavailable. A negative run deliberately reaching a forbidden
provider must reject before Stage2 publication. Route labels or log absence
alone do not prove non-delegation. Keep G1 comparison as a separate check;
Stage2 recompiling the compiler closure into Stage3 is a later fixed-point
check, not a substitute for the Stage1-to-Stage2 evidence.

Stop: unresolved compiler closure, use of a reduced stub in its place,
forbidden delegation, missing negative evidence, or inability to run generated
Stage2 leaves this final task open. Existing bootstrap tests remain valid in
their bounded role. No current selfhost implementation is activated here.

Evidence limits: `tools/selfhost/lib/identity_compare.sh` compares canonical
MIR/raw Program output; `tools/selfhost/lib/stage1_contract.sh` separately
checks bootstrap emission and reduced artifact liveness. The existing
`phase29bq_selfhost_identity_compat_route_guard_vm.sh` expectations must be
reconciled with the current identity script's route rejection before that
smoke is reused as negative acceptance.

### Stage1 Exit Gate

- `tools/selfhost/compat/run_stage1_cli.sh emit mir-json apps/tests/hello_simple_llvm.hako`
- `NYASH_USE_STAGE1_CLI=1 NYASH_STAGE1_MODE=emit-mir ./target/release/hakorune --emit-mir-json <out> apps/tests/hello_simple_llvm.hako`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 <...> --bin-stage2 <...>`

### Collection Regression Pack

- `bash tools/smokes/v2/profiles/integration/apps/phase29cm_collections_hot_raw_route_contract_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
- `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh`
- `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_map_provider_vm.sh`

### First Optimization Wave

- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 3`
- `bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3`
- judge perf only on the `.hako -> ny-llvmc(boundary) -> C ABI` route

## Non-Goals

- no new public ABI
- no `stage2-mainline` or `stage2+` artifact-kind creation
- no Rune backend-active optimization in this pack
- no reopening of collection owner migration without a new exact blocker
- no native zero / source zero claim
- no treating this pack as the `K2` substrate-era ledger
