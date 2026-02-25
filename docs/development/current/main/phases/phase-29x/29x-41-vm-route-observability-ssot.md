---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X19 route observability 契約（`[vm-route/select]` 安定タグ）を固定。
Related:
  - docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-52-vm-route-observability-contract-ssot.md
  - src/runner/dispatch.rs
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh
---

# Phase 29x X19: VM Route Observability SSOT

## 0. Goal

route 選択を「推測」ではなく安定タグで観測できるようにし、
X20-X21 の優先順変更時に分岐理由の混線を防ぐ。

## 1. Tag Contract

`NYASH_VM_ROUTE_TRACE=1` のとき、次の 1行タグを stderr に固定出力する。

1. `backend=vm` + `NYASH_VM_HAKO_PREFER_STRICT_DEV=0`（または non-strict/non-dev）:
   - `[vm-route/select] backend=vm lane=vm reason=default`
2. `backend=vm` + `NYASH_VM_USE_FALLBACK=1`:
   - `[vm-route/select] backend=vm lane=compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1`
3. `backend=vm-hako`:
   - `[vm-route/select] backend=vm-hako lane=vm-hako reason=backend:vm-hako`

禁止:
- タグ名の揺れ（`[vm-route] choose=...` など）を新規導入すること
- 複数行/可変構造の route dump を標準契約にすること

## 2. Acceptance (X19)

- smoke:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
- 判定:
  - vm(default opt-out) / vm fallback / vm-hako の3経路で上記タグが観測できること
  - vm default/fallback は `rc=0`、vm-hako frame は `rc!=0` を維持すること

## 3. Notes for X20+

- X20 で strict/dev 既定を `vm-hako` 優先へ変更しても、
  このタグ contract は維持し、`lane=vm` 観測は opt-out (`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`) で固定する。
- compat lane は常に明示条件付き（`NYASH_VM_USE_FALLBACK=1`）を維持する。
