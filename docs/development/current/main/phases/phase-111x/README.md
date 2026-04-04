# Phase 111x: selfhost runtime route naming cleanup

- 目的: `tools/selfhost/run.sh` の public runtime surface を route-first に寄せる。
- canonical:
  - `--runtime-route mainline`
  - `--runtime-route compat`
- compatibility alias:
  - `--runtime-mode exe`
  - `--runtime-mode stage-a-compat`
  - `stage-a`
- 対象:
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/README.md`
  - `docs/development/architecture/selfhost_execution_ssot.md`
- success:
  - `--runtime-route mainline|compat` で既存 runtime surface を叩ける
  - `--runtime-mode` は壊さず alias として残す
  - current/docs の canonical wording が route-first に揃う
