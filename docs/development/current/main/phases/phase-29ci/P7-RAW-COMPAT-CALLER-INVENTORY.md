---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: raw Program(JSON v0) compat flags (`--emit-program-json-v0`, `--program-json-to-mir`) の caller bucket と削除順を固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/P6-STAGE1-MIR-ROUTE-VOCABULARY.md
  - docs/development/current/main/phases/phase-29ci/P12-REMAINING-RAW-COMPAT-CALLERS.md
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - src/runner/pipe_io.rs
---

# P7 Raw Compat Caller Inventory

## Goal

P6 で重複 public alias (`--hako-emit-program-json`) を削除した後、raw compat flags をすぐ hard-delete できるかを exact caller bucket で固定する。

Conclusion:
- `--emit-program-json-v0` はまだ削らない。
- `--program-json-to-mir` はまだ削らない。
- unreferenced diagnostic helper `tools/dump_stageb_min_mir.sh` はこの slice で削除する。

## `--emit-program-json-v0` Buckets

| Bucket | Representative callers | Action |
| --- | --- | --- |
| stage0 identity / direct compat | `tools/selfhost/lib/identity_routes.sh`, `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost_identity_check.sh` | keep until stage0 direct compat lane is retired |
| Stage-B producer helper | `tools/selfhost/lib/selfhost_build_stageb.sh` | migrate only with selfhost build route proof |
| hako mirbuilder fixture producer | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*` | keep; these pin Program(JSON) fixtures for `.hako mirbuilder` |
| Program(JSON) contract pin | `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh` | keep as explicit contract evidence |
| parser dual-route probe | `tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh` | migrated in P11; Rust-side now uses AST JSON, Hako-side keeps wrapper Program(JSON) observation |

## `--program-json-to-mir` Buckets

| Bucket | Representative callers | Action |
| --- | --- | --- |
| CLI implementation | `src/runner/pipe_io.rs` | keep until all external callers migrate; tests preserve `user_box_decls` behavior |
| shared emit helper fallback | `tools/hakorune_emit_mir.sh` | retired in P8; helper now stops at selfhost/provider routes |
| selfhost EXE / Stage-B delegate | `tools/selfhost/lib/selfhost_build_exe.sh`, `tools/selfhost_exe_stageb.sh` | keep; exact build helpers still terminate through this bridge |
| dev/proof probe | `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | keep as historical reduced-stage proof unless replaced by MIR-first proof |
| smoke/test helper fallback | `tools/smokes/v2/lib/test_runner_builder_helpers.sh` | keep until shared builder fallback helper is rewritten |
| retired smoke fallback | `tools/smokes/v2/profiles/integration/core/phase2043/program_new_array_delegate_struct_canary_vm.sh` | retired in P10; canary now reports explicit SKIP instead of raw CLI fallback when `.hako MirBuilder` is not ready |
| historical pyvm helper | `tools/historical/pyvm/common.sh` | historical keep; do not mix with current phase cleanup |

## Deleted In This Slice

- `tools/dump_stageb_min_mir.sh`
  - reason: unreferenced standalone diagnostic helper
  - old route: Stage-B Program(JSON v0) dump -> raw `--program-json-to-mir`
  - replacement: use maintained phase29ci / selfhost route probes or direct MIR emit helpers

## Next Slice

Start with caller migration, not raw CLI deletion.

Candidate A (`tools/hakorune_emit_mir.sh` thin fallback, landed in P8):
1. probe `tools/hakorune_emit_mir.sh` without `try_legacy_program_json_delegate`
2. require representative `hako-mainline` / `hako-helper` emit smokes to pass through provider/selfhost routes
3. if green, delete only that legacy fallback function
4. if not green, keep the fallback and record the missing provider/selfhost route proof

Candidate B (`tools/smokes/v2/lib/test_runner_builder_helpers.sh` shared fallback, next after P12):
1. replace raw `--program-json-to-mir` with provider/selfhost builder route
2. keep builder-only and core-exec result routing unchanged
3. prove with representative phase2043 / mirbuilder-provider smokes
4. then re-inventory `--program-json-to-mir`

Candidate C (`--emit-program-json-v0` fixture producer):
1. pick one small `phase29bq_hako_mirbuilder_*` smoke family
2. decide whether it truly needs Program(JSON) fixture evidence or can consume MIR(JSON)
3. if MIR(JSON) is sufficient, rewrite that family to `--emit-mir-json` / `--mir-json-file`
4. keep Program(JSON) contract pin smoke separate

Guardrail:
- do not delete `src/runner/stage1_bridge/program_json_entry/**` until `--emit-program-json-v0` caller inventory reaches zero
- do not delete `src/runner/pipe_io.rs` `program_json_to_mir` path until all `--program-json-to-mir` shell callers are gone and `user_box_decls` preservation is pinned elsewhere
