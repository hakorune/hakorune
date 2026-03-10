---
Status: Accepted
Decision: accepted
Date: 2026-03-09
Scope: `phase-29cg` の reduction checklist。
Related:
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29cg/P0-STAGE2-BOOTSTRAP-REDUCTION-INVENTORY.md
  - tools/selfhost_identity_check.sh
  - tools/selfhost/build_stage1.sh
---

# 29cg-10 Stage2 Bootstrap Reduction Checklist

## 1) Inventory lock

- [x] `SBR-01` exact owner fixed
- [x] `SBR-02` exact condition fixed
- [x] `SBR-03` reduction target fixed

## 2) Contract definition

- [x] `SBR-04` define stage1-first Stage2 build contract for `stage1-cli`
- [x] `SBR-05` define acceptance proof for removing one default-bootstrap dependency

## 3) Execution rule

- [x] `SBR-06` reduce exactly one Stage2 default-bootstrap dependency
- [x] `SBR-07` keep `phase-29cf` inventory unchanged while `phase-29cg` executes reduction
- [x] `SBR-08` review mixed worker diff and adopt only minimal Rust-side safe-keep patches

## 4) Done judgment

- [x] `tools/selfhost_identity_check.sh` no longer needs the current default-bootstrap note for the reduced case
- [x] `selfhost-bootstrap-route-ssot.md` can reclassify one `future retire target`
- [x] reduced case can describe the bridge as `temporary bootstrap boundary`, not as current route authority

## 5) Current contract note

- `stage1-cli` reduction means `bridge-first Stage2 build`, not raw `NYASH_BIN=$STAGE1_BIN`
- proof sources:
  - raw direct contract returns `97`
  - `stage1_contract_exec_mode` emits Program(JSON), and `stage1_cli_env.hako` now carries helper defs (`defs_len=22`) for entry-local `Main` helpers
  - `stage1_contract_exec_mode ... emit-mir ...` now returns MIR(JSON)
  - `HAKO_STAGE1_MODULE_DISPATCH_TRACE=1` shows the MirBuilder module-dispatch route is hit and returns `output_bytes=213003` / `output_handle=97`
  - direct kernel/plugin proof accepts the same `stage1_cli_env.hako` Program(JSON v0) and returns MIR(JSON)
  - direct kernel proof for `lang.compiler.entry.using_resolver_box.resolve_for_source` returns an intentionally empty string in the surrogate lane
  - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` passes with `verify_rc=0`
  - bridge/runtime extern-like names no longer depend on `HAKO_MIR_BUILDER_CALL_RESOLVE` for `Callee::Extern`
  - mixed worker stash review adopted only Rust-side minimal arity canonicalization in `callsite_canonicalize.rs` and `json_v0_bridge/lowering/program.rs`; broader `lang/src/mir/builder/**`, `tools/selfhost/lib/stage1_contract.sh`, and `src/llvm_py/**` stash lanes remain deferred
  - reduced-case `build_stage1.sh` bridge-first path is now green for `NYASH_BIN=<stage1-cli bootstrap> ... --artifact-kind stage1-cli`
  - fresh `.next` artifact now passes direct env-mode `emit-program` and `emit-mir` probes for `apps/tests/hello_simple_llvm.hako`
  - `tools/selfhost_identity_check.sh --mode smoke` is now green on the `stage1-cli bridge-first bootstrap` route
  - the same prebuilt pair also passes `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2`
  - build-enabled `tools/selfhost_identity_check.sh --mode full` is also green
  - exact next step is to freeze/promote this solved bucket and hand off to `phase-29ch` for MIR-direct bootstrap unification

## 6) Restart quick entry (2026-03-10)

- final goal: `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM`
- bootstrap rule: `Program(JSON v0)` bridge is bootstrap-only and remains a retire target
- current minimal task: `phase-29cg` is not the MIR-direct unification phase; first close exactly one imported helper/source closure bucket in the reduced Stage2 object
- solved bucket keep-closed:
  - `bridge return-path`
  - `extern classification`
  - `current LLVM PHI repair`
- next owner order:
  - `src/stage1/program_json_v0.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` (only if needed)
- proof-first next steps:
  - `bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
  - `NYASH_BIN=<stage1-cli bootstrap> bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli ...`
  - only after the proof pair stays green/non-regressed, move to the next reduction slice
