---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X48 の route pin inventory（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`）を allowlist + guard で固定し、strict/dev route drift を抑止する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md
  - tools/checks/phase29x_vm_route_pin_allowlist.txt
  - tools/checks/phase29x_vm_route_pin_guard.sh
  - tools/smokes/v2/lib/vm_route_pin.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh
---

# 29x-75: VM Route Pin Inventory Guard (SSOT)

## 0. Conclusion

- `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` は無制限に増やさない。
- 固定点は allowlist で明示し、guard で逸脱を fail-fast する。
- source lane（`src/**/*.rs`）へ hard pin を持ち込まない。

## 1. Inventory source

- Allowlist SSOT: `tools/checks/phase29x_vm_route_pin_allowlist.txt`
- Guard: `tools/checks/phase29x_vm_route_pin_guard.sh`
- Shared helper: `tools/smokes/v2/lib/vm_route_pin.sh`
- Smoke wrapper: `tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh`

## 2. Contract

1. pin callsite（直書き assignment または helper 呼び出し）は allowlist にある shell entrypoint でのみ許可する。
2. allowlist にない新規 callsite は guard が即 fail-fast する。
3. allowlist から消えた callsite は drift として guard が fail-fast する。
4. `src/**/*.rs` に `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を直書きしない。

## 3. Evidence command

- `bash tools/checks/phase29x_vm_route_pin_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh`
