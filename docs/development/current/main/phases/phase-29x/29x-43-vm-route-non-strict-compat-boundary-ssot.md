---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X21 non-strict compat lane の境界固定（縮退）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-42-vm-route-strict-dev-priority-ssot.md
  - src/runner/selfhost.rs
  - tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh
---

# Phase 29x X21: VM Route Non-Strict Compat Boundary SSOT

## 0. Goal

non-strict の Stage-A runtime route で compat lane を暗黙で通さず、
明示 opt-in のみに限定する。

## 1. Contract

- strict+planner_required:
  - 既存どおり fail-fast（Program(JSON v0) は拒否）
- non-strict:
  - `NYASH_VM_USE_FALLBACK=1` が無い場合は stage-a 入口で fail-fast
  - tag:
    - `[contract][runtime-route][expected=mir-json] route=stage-a source=<src> non_strict_compat=disabled require=NYASH_VM_USE_FALLBACK=1`
  - `NYASH_VM_USE_FALLBACK=1` がある場合のみ compat lane を許可

## 2. Allowed Compat Lanes (explicit-only)

明示時のみ次を許可する。

1. `lane=compat-program-to-mir`（`.hako` mirbuilder 変換）
2. `lane=compat-rust-json-v0-bridge`（最終互換）

## 3. Acceptance (X21)

- smoke:
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- 判定:
  - non-strict + fallback未指定で reject tag を観測し、`rc!=0`
  - non-strict + `NYASH_VM_USE_FALLBACK=1` で `vm-route` の `compat-fallback` tag を観測し、`rc=0`
