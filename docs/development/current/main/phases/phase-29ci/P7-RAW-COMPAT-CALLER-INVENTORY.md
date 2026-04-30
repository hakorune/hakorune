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
- `--program-json-to-mir` は P16 で削除済み。
- unreferenced diagnostic helper `tools/dump_stageb_min_mir.sh` はこの slice で削除する。

## `--emit-program-json-v0` Buckets

| Bucket | Representative callers | Action |
| --- | --- | --- |
| neutral shell compat owner | `tools/lib/program_json_v0_compat.sh` | only current shell emit spelling of `--emit-program-json-v0`; archive pins source this helper after P20 |
| explicit compat/direct emit keeper | `tools/selfhost/lib/stage1_contract.sh` via neutral helper | keep until the explicit compat probe lane is retired |
| Stage-B producer helper | `tools/selfhost/lib/selfhost_build_stageb.sh` via neutral helper | migrate only with selfhost build route proof |
| hako mirbuilder fixture producer | `tools/smokes/v2/lib/stageb_helpers.sh` via `phase29bq_hako_mirbuilder_*` smokes | keep; these pin Program(JSON) fixtures for `.hako mirbuilder` and share the thin producer helper |
| Program(JSON) contract pin | `tools/smokes/v2/lib/stageb_helpers.sh` via `phase29bq_hako_program_json_contract_pin_vm.sh` | keep as explicit contract evidence |
| parser dual-route probe | `tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh` | migrated in P11; Rust-side now uses AST JSON, Hako-side keeps wrapper Program(JSON) observation |

Notes:
- `tools/selfhost/lib/identity_routes.sh` / `tools/selfhost_identity_check.sh`
  still own stage1 routing policy, but the retired wrapper-local
  `program-json` caller surface is no longer a live bucket here.
- `tools/selfhost/stage3_same_result_check.sh` and
  `tools/dev/phase29ch_selfhost_program_json_helper_probe.sh` now materialize
  Program(JSON) through the stage1 env contract helper instead of the retired
  `run_stage1_cli.sh emit program-json` wrapper surface.
- `phase29bq` single-fixture, multi-case, cleanup, and contract-pin callers now
  funnel through `stageb_emit_program_json_v0_fixture()`; smoke helper syntax
  ultimately resolves to `tools/lib/program_json_v0_compat.sh`.

## `--program-json-to-mir` Buckets

| Bucket | Representative callers | Action |
| --- | --- | --- |
| CLI implementation | `src/runner/pipe_io.rs` | retired in P16; `user_box_decls` proof lives in `src/host_providers/mir_builder.rs` |
| shared emit helper fallback | `tools/hakorune_emit_mir.sh` | retired in P8; helper now stops at selfhost/provider routes |
| selfhost EXE / Stage-B delegate | `tools/selfhost/lib/selfhost_build_exe.sh`, `tools/selfhost_exe_stageb.sh` | retired in P14; both use `tools/selfhost/lib/program_json_mir_bridge.sh` |
| dev/proof probe | `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | retired in P15; proof now uses `program_json_mir_bridge.sh` |
| smoke/test helper fallback | `tools/smokes/v2/lib/test_runner_builder_helpers.sh` | retired in P13; shared helper fallback now uses non-raw builder route |
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

Candidate B (`tools/smokes/v2/lib/test_runner_builder_helpers.sh` shared fallback, landed in P13):
1. replace raw `--program-json-to-mir` with provider/selfhost builder route
2. keep builder-only and core-exec result routing unchanged
3. prove with representative phase2043 / mirbuilder-provider smokes
4. then re-inventory `--program-json-to-mir`

Candidate C (`selfhost EXE / Stage-B delegate raw bridge, landed in P14):
1. add a selfhost-owned non-raw Program(JSON)->MIR bridge
2. migrate selfhost EXE helper and Stage-B EXE helper together
3. prove bridge shape with direct conversion probes
4. re-inventory `--program-json-to-mir`

Candidate D (`phase29cg dev proof raw bridge, landed in P15):
1. keep stage1-cli -> Program(JSON) proof shape
2. replace raw Program(JSON)->MIR CLI call with `program_json_mir_bridge.sh`
3. leave missing-stage1 fail-fast behavior unchanged
4. re-inventory `--program-json-to-mir`

Candidate E (`--emit-program-json-v0` fixture producer):
1. pick one small `phase29bq_hako_mirbuilder_*` smoke family
2. decide whether it truly needs Program(JSON) fixture evidence or can consume MIR(JSON)
3. if MIR(JSON) is sufficient, rewrite that family to `--emit-mir-json` / `--mir-json-file`
4. keep Program(JSON) contract pin smoke separate

Guardrail:
- do not delete `src/runner/stage1_bridge/program_json_entry/**` until `--emit-program-json-v0` caller inventory reaches zero
- `src/runner/pipe_io.rs` `program_json_to_mir` path is deleted in P16; `user_box_decls` preservation is pinned in `src/host_providers/mir_builder.rs`
