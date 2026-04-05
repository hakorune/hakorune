# Phase 127x: compat route raw vm cut prep

- 目的: compat route の contract を raw `vm-route/*` tag 断言から外し、`runtime-route compat` を temp-MIR handoff に切り替える前提を作る。
- 対象:
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
- success:
  - compat boundary smoke が raw `vm-route/pre-dispatch` / `lane=vm-compat-fallback` を contract pin しない
  - route-first tag (`runtime_route=compat`, `mode=stage-a-compat`) と explicit fallback env だけで compat keep を説明できる
  - 次 lane で `selfhost_run_routes.sh` compat branch を temp-MIR handoff 化できる

## Decision Now

- `phase29bq` route/parity smokes are already route-first
- remaining prep seam is `phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- do not cut `selfhost_run_routes.sh` in this phase; first make the smoke contract route-first

## Next

1. switch `tools/selfhost/lib/selfhost_run_routes.sh` compat branch from raw `--backend vm` to temp-MIR handoff
2. keep explicit fallback env contract (`NYASH_VM_USE_FALLBACK=1`)
3. then move to `phase-128x stage1 bridge vm gate softening`
