# Phase 123x: proof gate shrink follow-up

- 目的: `tools/selfhost/proof/*` のうち public proof surface と internal engineering keep を切り分け、次段の docs/manual demotion に渡す。
- 対象:
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - `README.md`
  - `README.ja.md`
  - `tools/selfhost/README.md`
  - `docs/development/selfhosting/quickstart.md`
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- success:
  - public proof surface は `run_stageb_compiler_vm.sh` と `selfhost_vm_smoke.sh` に限定して読める
  - internal engineering callers / docs pressure が source-backed に固定される
  - `phase-124x vm public docs/manual demotion` に渡す manual-only pressure が分かる

## First-pass proof buckets

### keep-public proof surface

- `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- `tools/selfhost/proof/selfhost_vm_smoke.sh`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/selfhost_vm_smoke.sh`
- `tools/selfhost/run.sh --direct --source-file ...`
- `make smoke-selfhost`

### keep-internal engineering callers

- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/selfhost/lib/selfhost_run_routes.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh`
- `tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`

### docs/manual pressure

- `README.md`
- `README.ja.md`
- `tools/selfhost/README.md`
- `docs/development/selfhosting/quickstart.md`
- `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- `docs/development/architecture/selfhost_execution_ssot.md`
- `.github/pull_request_template.md`

## Decision Now

- keep `tools/selfhost/proof/run_stageb_compiler_vm.sh` public for now
  - top-level proof facade and `run.sh --direct` still resolve to it
  - `selfhost_run_routes.sh` and multiple phase29bq/29cc proofs still shell into it
- keep `tools/selfhost/proof/selfhost_vm_smoke.sh` public for now
  - top-level proof facade and `make smoke-selfhost` still expose it as a live proof entry
- shrink pressure first on docs/manual wording
  - raw `--backend vm` should stay visible only as explicit proof/debug ingress
  - public proof surface should not read like day-to-day runtime guidance

## Next Cut

1. docs/manual demotion
   - `README.md`
   - `README.ja.md`
   - `tools/selfhost/README.md`
   - `docs/development/selfhosting/quickstart.md`
2. keep internal engineering callers explicit
   - do not add new callers to `run_stageb_compiler_vm.sh` / `selfhost_vm_smoke.sh`
3. revisit backend gate only after compat exit blockers move
