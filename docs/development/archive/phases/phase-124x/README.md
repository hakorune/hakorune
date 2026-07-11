# Phase 124x: vm public docs/manual demotion

- 目的: public docs/manual で raw `--backend vm` と proof gates が日常 route に見えないよう wording を狭める。
- 対象:
  - `README.md`
  - `README.ja.md`
  - `tools/selfhost/README.md`
  - `docs/development/selfhosting/quickstart.md`
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `.github/pull_request_template.md`
- success:
  - public docs は `runtime-route mainline` を日常 route として読める
  - `--backend vm` は explicit proof/debug ingress としてだけ読める
  - `run_stageb_compiler_vm.sh` / `selfhost_vm_smoke.sh` は optional proof gate としてだけ見える

## Demotion Rules

- keep:
  - `runtime-route mainline` as the day-to-day selfhost route
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
- demote:
  - raw `--backend vm` from broad selfhost narrative
  - proof gates from day-to-day quickstart narrative
- do not change:
  - compat route contract
  - backend gate behavior
  - proof gate implementation/callers
