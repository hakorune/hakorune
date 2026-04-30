---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: `tools/smokes/v2/lib/test_runner_builder_helpers.sh` の raw Program(JSON)->MIR CLI fallback を non-raw builder route へ移す。
Related:
  - docs/development/current/main/phases/phase-29ci/P12-REMAINING-RAW-COMPAT-CALLERS.md
  - tools/smokes/v2/lib/test_runner_builder_helpers.sh
  - tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/mirbuilder_provider_emit_core_exec_canary_vm.sh
---

# P13 Shared Smoke Helper Fallback Retire

## Goal

P12 で次に削る対象にした shared smoke helper fallback から、raw
`--program-json-to-mir` bridge を外す。

この slice は smoke helper の fallback route だけを置換する。
selfhost EXE / Stage-B delegate / phase29cg proof の raw caller は別 slice に
残す。

## Decision

- `run_program_json_v0_via_rust_cli_builder(...)` を
  `run_program_json_v0_via_non_raw_builder_fallback(...)` に置換する。
- fallback は selfhost/min builder route を先に使う。
- failed primary route の `HAKO_V1_EXTERN_PROVIDER=1` stub は recovery path に
  持ち込まない。以前の raw CLI fallback と同じく、stub provider 失敗を
  recovery path で隠さず、non-raw builder route で再構築する。
- builder-only / core-exec result routing は変えない。

## Result

`--program-json-to-mir` live shell callers are now:

1. `tools/selfhost/lib/selfhost_build_exe.sh`
2. `tools/selfhost_exe_stageb.sh`
3. `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`

`tools/smokes/v2/lib/test_runner_builder_helpers.sh` no longer calls raw
`--program-json-to-mir`.

## Acceptance

```bash
bash -n tools/smokes/v2/lib/test_runner_builder_helpers.sh
bash tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/mirbuilder_provider_emit_core_exec_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase2043/mirbuilder_runner_min_typeop_cast_core_exec_canary_vm.sh
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
bash tools/checks/current_state_pointer_guard.sh
```
