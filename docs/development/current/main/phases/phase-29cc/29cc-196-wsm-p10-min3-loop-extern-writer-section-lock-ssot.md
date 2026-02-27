---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min3 として loop/branch/call writer section contract を固定し、route は bridge のまま維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-195-wsm-p10-min2-loop-extern-matcher-inventory-lock-ssot.md
  - src/backend/wasm/binary_writer.rs
  - src/backend/wasm/mod.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh
---

# 29cc-196 WSM-P10-min3 Loop/Extern Writer Section Lock

## Purpose
`WSM-P10-min2` で固定した analysis-only matcher の次段として、
loop/branch/call を含む writer section 合成 API を固定する。  
この段階では default route の native 昇格は行わず、bridge 契約を維持する。

## Contract
1. writer API は `build_loop_extern_call_skeleton_module(iterations)` を単一入口として固定する。
2. section 順は `Type -> Import -> Function -> Export -> Code` を固定する。
3. code section は最小語彙として `loop + br_if + call + local.set/get` を含む。
4. import/export ABI は固定する。
   - import: `env.console_log(i32) -> void`
   - export: `main() -> i32`（func index 1; index 0 は import）
5. route は bridge 維持（`plan=bridge-rust-backend`）とし、min3 では shape promotion を行わない。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min4`: 1 fixture を native emit へ昇格（bridge fallback は fail-fast 契約維持）。
