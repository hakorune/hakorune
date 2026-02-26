---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P3-min1（.hako-only roadmap P3）として JS import object 生成契約を lock する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-152-wsm-p2-min1-wat2wasm-bridge-lock-ssot.md
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p3_min1_import_object_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-153 WSM-P3-min1 Import Object Contract Lock

## Purpose
`.hako`-only WASM 出力移行（P3）に向け、JS import object の生成契約を supported list と fail-fast 文言で固定する。

## Decision
1. import object 生成を決定順にした（module名順 + import名順）。
2. 未実装 import binding の fallback 文言を `Unsupported import binding: <name>` に固定した。
3. `src/backend/wasm/runtime.rs` に contract test 3件を追加した。
   - supported list（extern contract map の全 import を含む）
   - 標準 import 群に fallback stub が混入しない
   - unknown import は fail-fast 文言を持つ
4. smoke `phase29cc_wsm_p3_min1_import_object_lock_vm.sh` を追加し、`tools/checks/dev_gate.sh wasm-boundary-lite` に統合した。

## Acceptance
- `cargo test --features wasm-backend runtime_imports_js_object_ -- --nocapture`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p3_min1_import_object_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P4-min1`: `.hako` 側 wasm binary writer（section/LEB128）の最小契約を docs-first で固定する。
