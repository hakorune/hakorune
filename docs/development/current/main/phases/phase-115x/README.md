# Phase 115x: vm route retirement planning

- 目的: `--backend vm` を通常実行経路ではなく compat/proof/debug override として凍結した上で、将来の route retirement 順を固定する。
- 対象:
  - `src/runner/route_orchestrator.rs`
  - `src/runner/dispatch.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/proof/*`
  - active vm-family observability / compat boundary smokes
- success:
  - `--backend vm` が keep/debug family でしか使われない current shape を inventory 化できる
  - compat / proof / debug の dependency を分けて retirement order を書ける
  - next lane が `alias pruning` か `explicit env hardening` のどちらかに絞れる
