# NyRT Host Surface v0 (Core C ABI Host-Facing Slice)

Updated: 2026-02-28

Purpose:
- `.hako` 主体移行時の host 境界を「最小 surface」に固定する。
- 意味論実装を host 側へ再流入させないため、用途別に許可 API を明記する。

Related:
- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- `docs/reference/abi/nyrt_c_abi_v0.md`
- `docs/development/current/main/design/hako-host-facade-contract-ssot.md`
- `include/nyrt.h`
- `include/nyrt_host_api.h`

## 1. Surface Categories (fixed)

Host-facing API は次の 5 カテゴリに限定する。

1. Runtime lifecycle/bootstrap
2. Runtime execution/verification
3. Host reverse-call bridge (plugin -> host)
4. Handle lifecycle
5. Runtime V0 helper slice

## 2. Canonical Symbol Table

| Category | Symbol(s) | Header | Contract Summary |
| --- | --- | --- | --- |
| Runtime lifecycle/bootstrap | `nyrt_init`, `nyrt_teardown` | `include/nyrt.h` | runtime bootstrap/teardown only |
| Runtime execution | `nyrt_load_mir_json`, `nyrt_exec_main` | `include/nyrt.h` | MIR text load + main execution entry |
| Runtime verification/safety | `nyrt_verify_mir_json`, `nyrt_safety_check_mir_json` | `include/nyrt.h` | fail-fast verifier/safety gates |
| Host reverse-call bridge | `nyrt_hostcall`, `nyrt_host_call_name`, `nyrt_host_call_slot` | `include/nyrt.h`, `include/nyrt_host_api.h` | TLV bridge (`slot` preferred for stable dispatch) |
| Handle lifecycle | `nyrt_handle_retain_h`, `nyrt_handle_release_h` | `include/nyrt.h` | borrowed/owned lifecycle boundary |
| Runtime V0 helper slice | `string_len`, `array_get_i64`, `array_set_i64` (route lock) | runtime/plugin route lock docs | `.hako` runtime entry boxes from VM callsite (`string_core_box`/`array_core_box`) |

## 2.1 Current vs Planned Extensions (explicit)

`process/fs/net/time` は host category として予約するが、v0 canonical symbol にはまだ追加しない。

| Category | v0 status | Canonical symbol status |
| --- | --- | --- |
| Runtime lifecycle/bootstrap | Active | fixed |
| Runtime execution/verification | Active | fixed |
| Host reverse-call bridge | Active | fixed |
| Handle lifecycle | Active | fixed |
| Runtime V0 helper slice | Active | fixed-route |
| Process | Planned | not-exported |
| File system | Planned | not-exported |
| Network | Planned | not-exported |
| Time | Planned | not-exported |

Rule:
1. planned category を実装に昇格する場合は docs-first で symbol table に追加する。
2. `include/nyrt.h` / `include/nyrt_host_api.h` へ先に実装を足してから docs を追従させる運用は禁止。

## 3. Ownership and Error Contract

1. ABI contract is `args borrowed / return owned`.
2. `retain_h(0) -> 0`, `release_h(0)` is no-op.
3. strict/dev path is fail-fast; silent fallback is prohibited.

## 4. Explicitly Disallowed in Host Layer

1. Method resolution policy
2. Plugin loader routing policy
3. Value codec semantic decisions
4. Runtime semantic fallback branches

These must remain in `.hako` side logic owner.

## 5. Verification

1. `bash tools/checks/abi_lane_guard.sh`
2. `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
3. `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
4. `tools/checks/dev_gate.sh runtime-exec-zero`

Expected: all green while maintaining only two canonical ABI surfaces (Core C ABI, TypeBox ABI v2).

## 6. Promotion Gate (Step-2 -> Step-3)

1. HostFacade contract:
   - `docs/development/current/main/design/hako-host-facade-contract-ssot.md`
2. `tools/checks/dev_gate.sh runtime-exec-zero`
3. `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
4. `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
5. `tools/checks/dev_gate.sh portability`
6. `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2`
