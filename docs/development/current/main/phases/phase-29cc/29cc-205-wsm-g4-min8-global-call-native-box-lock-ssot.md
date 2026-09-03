---
Status: Retired (legacy success contract removed)
Phase: 29cc
Task: WSM-G4-min8
Title: WASM Global Call Native Box Lock
Depends:
  - src/backend/wasm/codegen/mod.rs
  - src/backend/wasm/codegen/instructions.rs
  - apps/tests/phase29cc_wsm_g4_min8_global_call_probe_min.hako
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min8_global_call_probe_vm.sh
---

# 29cc-205 WSM-G4-min8 Global Call Native Box Lock (retired)

## Goal

The former success lock is retired. The existing user-defined method probe now
records the selected WASM profile's explicit pre-WAT rejection; canonical WASM
method support belongs to a future backend family row.

## Scope

1. Keep the probe fixture as a stable unsupported-input specimen.
2. Reject the canonical user-method shape before WAT/artifact generation.
3. Keep the direct `LegacyCallV0(Global)` rejection proof in the backend owner.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min8_global_call_probe_vm.sh`

## Notes

- This retired lock does not change route policy. It no longer claims that a
  user-defined method is accepted by the selected WASM profile.
- `WSM-G4-min3/min4` は現時点で prebuilt 安定性を優先し、fixture 側は marker 出力、
  playground 側は marker-driven JS draw hook（`runCanvasDemoForMarker`）で運用する。
  WasmBox 直ルートは別タスクで昇格する。
