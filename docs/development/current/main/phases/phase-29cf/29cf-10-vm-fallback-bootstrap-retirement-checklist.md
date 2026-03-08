---
Status: Accepted (monitor-only)
Decision: accepted
Date: 2026-03-09
Scope: `phase-29cf` の `VM fallback compat lane` / `bootstrap boundary reduction` を checkbox で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29cf/P0-VM-FALLBACK-AND-BOOTSTRAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/modes/vm_fallback.rs
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/build_stage1.sh
---

# 29cf-10 VM Fallback / Bootstrap Boundary Retirement Checklist

## 0) Current Snapshot

- [x] `phase-29cc` top-level closeout remains accepted
- [x] `compat-fallback` is not a current mainline route
- [x] `stage1` is the current selfhost identity route
- [x] `stage0` / `auto` are compatibility-only recovery routes

## 1) VM fallback compat lane

- [x] `VCF-01` classify `vm` / `vm-hako` / `compat-fallback`
  - current mainline keep: `vm`, `vm-hako`
  - compat keep: `compat-fallback`
- [x] `VCF-02` classify explicit fallback trigger
  - `NYASH_VM_USE_FALLBACK=1` remains explicit compat keep
- [x] `VCF-03` classify Stage-A compat bridge lane
  - Stage-A compat bridge stays `compat keep`
- [x] `VCF-04` classify retire conditions for `compat-fallback`
  - decision: `failure-driven keep` (`monitor-only`)
  - contract: `compat-fallback` remains explicit opt-in via `NYASH_VM_USE_FALLBACK=1`; bypass stays fail-fast (`[freeze:contract][vm-route/compat-bypass]`)
  - reopen condition: promote to `future retire target` only when active fallback caller=0 is confirmed and `vm` / `vm-hako` mainline stays green without compat lane

## 2) Bootstrap boundary reduction

- [x] `BSR-01` inventory Stage0 / Stage1 / Stage2 keep boundary
- [x] `BSR-02` inventory `stage1-cli` vs default bootstrap dependency
- [x] `BSR-03` fix Stage2 default-bootstrap dependency as next reduction target
- [x] `BSR-04` reclassify one real bootstrap dependency cut as future-wave target
  - decision: `future retire target` (`phase-29cf` では code cut しない)
  - current lock: `stage1` identity route stays mainline; `stage0` / `auto` stay compatibility-only; Stage2 default-bootstrap dependency remains the next dedicated reduction target
  - reopen condition: execute in a dedicated bootstrap-reduction phase when Stage1-first build path can remove one default-bootstrap dependency

## 3) Follow-up rule

- [x] This checklist does not reopen `phase-29cc`
- [x] retire/reduction decisions must be docs-first
- [x] close the phase after `VCF-04` and `BSR-04` are explicitly fixed as `compat keep` / `future-wave target`
