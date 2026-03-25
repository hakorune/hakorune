---
Status: Closed Inventory
Decision: accepted
Date: 2026-03-15
Scope: thin backend boundary (`LlvmBackendBox` / `hako_aot`) の final runtime-proof owner を `.hako VM` に固定したうえで、runtime-proof blocker inventory を close-sync する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - src/runner/modes/vm_hako/subset_check.rs
  - lang/src/vm/boxes/mir_vm_s0.hako
  - lang/src/runtime/host/host_facade_box.hako
---

# P4: Runtime-Proof Owner Blocker Inventory

## Goal

- final runtime-proof owner を `.hako VM` として固定したまま、実装順を迷わない粒度まで blocker を分ける。
- regular Rust VM は temporary proof lane として扱い、final-owner 実装と混ぜない。

## Lane Split

1. final owner lane: `.hako VM`
   - target route: `--backend vm-hako`
   - done shape: `LlvmBackendBox` を含む `.hako` caller が vm-hako route で実行できる
2. temporary proof lane: regular Rust VM
   - target route: `--backend vm`
   - role: blocker closeout までの補助 proof
   - non-goal: final runtime-proof owner に昇格しない

## Closed Blockers

### B1. vm-hako subset-check rejects `newbox(LlvmBackendBox)`

- observed command:
  - `./target/release/hakorune --backend vm-hako <tmp LlvmBackendBox caller>`
- observed tag:
  - `[vm-hako/unimplemented] phase=s5l route=subset-check ... op=newbox(LlvmBackendBox)`
- owner:
  - `src/runner/modes/vm_hako/subset_check.rs`
- current state:
  - allowlist is `ArrayBox` / `StringBox` / `FileBox` / `Main` only
- minimum implementation slice:
  - allow `newbox(LlvmBackendBox)` in subset-check
- acceptance:
  - same repro command no longer fails at `subset-check op=newbox(LlvmBackendBox)`
- landed (2026-03-15):
  - `src/runner/modes/vm_hako/subset_check.rs` now allows `newbox(LlvmBackendBox)`
  - repro moved forward to the next blocker tag

### B2. vm-hako execution support after subset-check

- status:
  - landed
- observed progression:
  - after `B1`: `[vm-hako/unimplemented op=boxcall1 method=compile_obj]`
  - then regular Rust VM bridge blocker: `Method call: missing receiver for extern_invoke`
  - then bridge pseudo-box blocker: `NewBox hostbridge: Unknown Box type`
  - then backend env blocker: `env.codegen.link_object: C-API route disabled`
  - final backend payload blocker: `env.codegen.emit_object: [llvmemit/capi/failed] missing schema_version`
- landed owner files:
  - `lang/src/vm/boxes/mir_vm_s0.hako`
  - `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako`
  - `src/runner/modes/vm_hako/subset_check.rs`
  - `lang/src/shared/backend/llvm_backend_box.hako`
- landed shape:
  - `vm-hako` runtime now has narrow `LlvmBackendBox.compile_obj/1` and `link_exe/3` execution helpers
  - vm-hako backend helpers now route through owner-local helper methods that lower to canonical `Callee::Extern(env.codegen.*)`
  - daily compile path now calls `env.codegen.compile_json_path`
  - MIR(JSON v0) payload normalization is owned by Rust backend boundary `normalize_mir_json_for_backend(...)`, not by `.hako` caller-side file reads
  - final replay is `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
- rule:
  - keep `compile_obj/1` and `link_exe/3` narrow; do not widen generic custom-box boxcall execution

### T1. regular VM hostbridge runtime support gap

- status:
  - retired from the phase-29ck proof path
- current meaning:
  - regular `--backend vm` still does not provide final-owner runtime proof for `LlvmBackendBox`
  - and the phase-29ck `.hako VM` proof no longer depends on a regular-VM hostbridge seam
- owner area:
  - `lang/src/runtime/host/host_facade_box.hako`
  - Rust VM hostbridge execution path
- rule:
  - keep future widening separate from the landed seam-only support

## Fixed Order

1. `B1` vm-hako subset-check allowlist
2. implement the narrow `LlvmBackendBox.compile_obj/1` / `link_exe/3` runtime seam
3. retire the regular Rust VM hostbridge seam from `.hako VM` proof path
4. pin acceptance with `phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Current Truth

1. `phase29ck_vmhako_llvm_backend_runtime_proof.sh` is green on the final owner lane.
2. `.hako VM` is now the locked runtime-proof owner for the phase-29ck backend boundary path.
3. regular Rust VM hostbridge widening is not part of the current mainline proof path.
4. next exact front returns to `phase-29cl` compiled-stage1 surrogate shrink, not a new runtime-proof widening slice.

## Non-goals

- regular Rust VM を final runtime-proof owner にすること
- `B1` と `B2` を同じ commit series に混ぜること
- `native_driver.rs` widening と混線させること
