---
Status: Active
Decision: accepted
Date: 2026-03-15
Scope: thin backend boundary (`LlvmBackendBox` / `hako_aot`) の final runtime-proof owner を `.hako VM` に固定したうえで、blocker を lane 分離して最小 slice 順に inventory する。
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

## Current Blockers

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
  - active
- observed next blocker after `B1`:
  - `[vm-hako/unimplemented] phase=s5l route=subset-check ... op=boxcall(args>1)`
- likely owner files:
  - `lang/src/vm/boxes/mir_vm_s0.hako`
  - `src/runner/modes/vm_hako/subset_check.rs`
  - `lang/src/vm/hakorune-vm/**`
- note:
  - `mir_vm_s0.hako` already stores a generic token for `newbox`, so allocation itself is no longer the first blocker
  - the exact multi-arg boxcall source still needs owner identification before code changes
- rule:
  - identify the offending method/shape first; do not widen generic `boxcall(args>1)` blindly

### T1. regular VM hostbridge runtime support gap

- status:
  - known temporary-lane blocker
- current meaning:
  - regular `--backend vm` does not provide final-owner runtime proof for `LlvmBackendBox`
- owner area:
  - `lang/src/runtime/host/host_facade_box.hako`
  - Rust VM hostbridge execution path
- rule:
  - inventory only for now; no implementation unless vm-hako lane stalls

## Fixed Order

1. `B1` vm-hako subset-check allowlist
2. identify the exact `boxcall(args>1)` owner/method in vm-hako compile output
3. implement the minimum `B2` slice for that exact shape
4. revisit `T1` only if vm-hako lane becomes structurally blocked

## Non-goals

- regular Rust VM を final runtime-proof owner にすること
- `B1` と `B2` を同じ commit series に混ぜること
- `native_driver.rs` widening と混線させること
