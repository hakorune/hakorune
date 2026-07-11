# Phase 117x: vm compat/proof env hardening

- 目的: compat route が raw `--backend vm` に入る前に explicit env を要求し、vm-family keep を accidental ingress からさらに切り離す。
- 対象:
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/README.md`
  - `docs/development/architecture/selfhost_execution_ssot.md`
- success:
  - `runtime-route compat` / `runtime-mode stage-a-compat` は shell preflight で `NYASH_VM_USE_FALLBACK=1` を要求する
  - non-strict compat boundary smoke がそのまま通る
  - current pointers が `phase-117x` に揃う
