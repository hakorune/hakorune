# Phase 116x: execution surface alias pruning

- 目的: public/front-door に残る runtime alias のうち、不要な thin alias を削って `runtime-route compat` / `runtime-mode stage-a-compat` を canonical に寄せる。
- 対象:
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/README.md`
  - `docs/development/architecture/selfhost_execution_ssot.md`
- success:
  - naked `stage-a` alias が live surface から消える
  - compat route は `runtime-route compat` / `runtime-mode stage-a-compat` の二段に揃う
  - proof/compat smoke がそのまま通る
