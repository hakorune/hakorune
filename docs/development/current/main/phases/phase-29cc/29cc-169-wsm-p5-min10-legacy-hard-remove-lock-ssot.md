---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min10（.hako-only roadmap P5）として legacy route policy 値の受理を hard-remove し、parse 境界 fail-fast 契約を SSOT 化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-168-wsm-p5-min9-legacy-retire-execution-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/config/env/runner_flags.rs
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-169 WSM-P5-min10 Legacy Hard-Remove Lock

## Purpose
`WSM-P5-min9` で実行境界 retire（accepted-but-blocked）まで固定した legacy lane を、受理境界（policy parse）から完全に削除する。

## Decision
1. `NYASH_WASM_ROUTE_POLICY` の受理値は `default | rust_native`。
2. `legacy` / `legacy-wasm-rust` を含む非 `default|rust_native` 値は parse 境界で fail-fast する。
3. fail-fast タグは `[freeze:contract][wasm/route-policy]` を使用し、`(allowed: default|rust_native)` を明示する。
4. wasm compile route enum/parse/route/test/docs を同コミットで同期し、legacy 実行分岐は残さない（hard-remove）。

## Implemented
1. `src/config/env/runner_flags.rs`
   - route policy parser を `default | rust_native` へ固定。
   - legacy alias 受理テストを reject テストへ置換。
2. `src/runner/modes/wasm.rs`
   - `LegacyRust` compile route 分岐を削除。
   - route policy name は `default` のみを保持。
3. `tests/wasm_demo_min_fixture.rs`
   - legacy route retired 契約を、legacy policy parse reject 契約へ更新。
   - legacy 指定時は route trace 以前に fail-fast することを固定。
4. `tools/checks/dev_gate.sh`
   - `wasm-boundary-lite` へ `phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh` を追加し、min8/min9 実行ロックを置換。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P6-min1`: `NYASH_WASM_ROUTE_POLICY` を default-only no-op 方針として維持/撤去の判断を docs-first で固定する。
