# Frontend Owner Proof Index

Status: SSOT  
Scope: frontend / bootstrap owner buckets and the smallest canonical proofs that reopen them.

Related:
- [CURRENT_TASK.md](/home/tomoaki/git/hakorune-selfhost/CURRENT_TASK.md)
- [selfhost-bootstrap-route-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/selfhost-bootstrap-route-ssot.md)
- [src/host_providers/mir_builder/README.md](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/README.md)
- [lang/src/runner/README.md](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/README.md)

## Purpose

- Keep `owner -> fixture/probe -> acceptance` readable from one page.
- Reopen the smallest proof first instead of growing broad regression packs.
- Make frontend cleanliness work choose the right seam before touching code.

## 1. MIR Builder Owner

Owner:
- [src/host_providers/mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs)

Responsibilities:
- strict source -> Program(JSON)
- Program(JSON) -> `MirModule`
- `user_box_decls` shaping
- shared MIR JSON stop-line

Canonical proofs:
- `cargo test mir_builder -- --nocapture`
- `cargo test user_box_decls -- --nocapture`
- `cargo test program_json_to_mir_file -- --nocapture`
- `bash tools/dev/phase29ch_program_json_cold_compat_probe.sh`

## 2. Runner Authority Owner

Owners:
- [lang/src/runner/stage1_cli_env.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli_env.hako)
- [lang/src/runner/stage1_cli.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako)
- [lang/src/runner/launcher.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/launcher.hako)

Responsibilities:
- source/program-json input shaping
- emit-program / emit-mir checked authority
- compat keep quarantine
- launcher raw/subcmd dispatch

Canonical proofs:
- `bash tools/dev/phase29ch_program_json_cold_compat_probe.sh`
- `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli_env.hako /tmp/stage1_cli_env_probe.mir.json`
- `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_probe.mir.json`
- `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/launcher.hako /tmp/launcher_probe.mir.json`

## 3. Shell Contract Owner

Owners:
- [tools/selfhost/lib/stage1_contract.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/stage1_contract.sh)
- [tools/selfhost/lib/identity_routes.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/identity_routes.sh)
- [tools/selfhost/build_stage1.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/build_stage1.sh)

Responsibilities:
- exact env-mode transport
- payload marker validation
- route identity
- selfhost bootstrap wrapper contract

Canonical proofs:
- `bash -n tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/identity_routes.sh tools/selfhost/build_stage1.sh`
- `bash tools/dev/phase29ch_program_json_cold_compat_probe.sh`
- `bash tools/selfhost/run_lane_a_daily.sh`

## 4. Backend Boundary Owner

Owners:
- [lang/src/shared/backend/llvm_backend_box.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/shared/backend/llvm_backend_box.hako)
- [src/host_providers/llvm_codegen.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/llvm_codegen.rs)
- [lang/c-abi/shims/hako_aot_shared_impl.inc](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_aot_shared_impl.inc)
- [lang/c-abi/shims/hako_llvmc_ffi.c](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi.c)

Responsibilities:
- boundary-first object/exe route
- pure-first compile coverage
- explicit compat keep replay

Canonical proofs:
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_first_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_print_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_length_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_length_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_indexof_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_length_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_compat_keep_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_forwarder_min.sh`
- `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`

## Reopen Rule

1. Pick the owner bucket first.
2. Reopen the smallest proof listed for that bucket.
3. Promote only after the smallest proof is green.
4. Do not mix frontend-owner cleanup with unrelated backend coverage expansion in the same commit series unless the proof bucket is shared.
