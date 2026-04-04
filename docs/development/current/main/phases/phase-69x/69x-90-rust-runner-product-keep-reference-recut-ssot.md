---
Status: Active
Date: 2026-04-04
Scope: split `src/runner` into product / keep / reference lanes after the `.hako` runner recut landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-68x/68x-90-hako-runner-authority-compat-facade-recut-ssot.md
---

# 69x-90 Rust Runner Product/Keep/Reference Recut SSOT

## Intent

- make rust runner ownership readable from paths instead of comments alone
- separate product, explicit keep, and reference surfaces without changing the current mainline contract
- keep the deliverable as tree shape plus narrow module/import cleanup

## Starting Read

- `src/runner/modes/llvm/**` and `wasm.rs` are product-facing
- `src/runner/modes/vm.rs` and `vm_fallback.rs` are explicit keep
- `src/runner/modes/vm_hako/**` and `vm_hako.rs` are reference/conformance
- `dispatch.rs` and `route_orchestrator.rs` are still cross-lane coordination surfaces

## Candidate Reading

### Product-facing

- `src/runner/modes/llvm/**`
- `src/runner/modes/wasm.rs`

### Keep-facing

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`

### Reference-facing

- `src/runner/modes/vm_hako/**`
- `src/runner/modes/vm_hako.rs`

## Current Inventory

### Product bucket

- `src/runner/modes/llvm/**`
- `src/runner/modes/wasm.rs`
- `src/runner/modes/mir.rs`
- `src/runner/modes/mir_interpreter.rs`

### Keep bucket

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`

### Reference bucket

- `src/runner/modes/vm_hako/**`
- `src/runner/modes/vm_hako.rs`

### Cross-lane / hold-first surfaces

- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`
- `src/runner/modes/mod.rs`
- `src/runner/modes/common_util/**`

## Inventory Notes

- current explicit keep core is still compact:
  - `vm.rs` 302 LOC
  - `vm_fallback.rs` 191 LOC
- current reference shell is smaller than the `.hako` reference payload behind it:
  - `vm_hako.rs` 125 LOC
- `dispatch.rs` (423 LOC) and `route_orchestrator.rs` (356 LOC) are still cross-lane coordination surfaces and should not move in the first split

## Target Layout

```text
src/runner/
  product/
  keep/
  reference/
```

## Ranking

1. `69xB1 product/reference split`
   - lower blast: mostly path truth and module wiring around already-separated families
2. `69xB2 keep split`
   - higher blast: rust-vm keep still touches explicit compat/proof surfaces and should move after product/reference paths settle

## Decision Rule

- product-facing runner code goes under `product/`
- explicit rust-vm stop-line code goes under `keep/`
- vm-hako reference/conformance code goes under `reference/`
- cross-lane coordination stays put until alias/module cleanup narrows callers

## Big Tasks

1. `69xA1` runner folder inventory lock
2. `69xA2` target layout ranking
3. `69xB1` product/reference split
4. `69xB2` keep split
5. `69xC1` alias/module cleanup
6. `69xD1` proof / closeout

## Current Progress

- `69xA1` landed: inventory is locked across product / keep / reference / cross-lane buckets
- `69xA2` landed: target layout ranking is fixed before path moves start
- `69xB1` landed: `llvm/`, `wasm.rs`, and `vm_hako*` moved under `product/` and `reference/` while `modes/` stayed as a thin compatibility re-export surface
- `69xB2` active: `vm.rs` and `vm_fallback.rs` are moving under `keep/` while `dispatch.rs` and `route_orchestrator.rs` stay hold-first
