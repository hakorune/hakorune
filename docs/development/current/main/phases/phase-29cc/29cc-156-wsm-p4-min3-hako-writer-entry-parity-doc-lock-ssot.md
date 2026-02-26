---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min3（.hako-only roadmap P4）として `.hako` writer 入口と parity gate の最小契約を docs-first で lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-155-wsm-p4-min2-binary-writer-skeleton-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-156 WSM-P4-min3 `.hako` Writer Entry + Parity Docs Lock

## Purpose
P4-min2（Rust skeleton lock）から `.hako` 実装へ進む入口を固定する。  
この段ではコード実装を急がず、`.hako` writer の entry contract と parity gate を先に固定し、実装時の散逸を防ぐ。

## Decision
1. `.hako` 側 writer の最小入口は「const-return fixture 1件」のみを対象にする。
2. parity は「Rust binary writer 出力との byte-level 同値」を最小受け入れ基準にする。
3. 未対応 section / shape は fail-fast を維持し、silent fallback を禁止する。
4. WAT bridge 経路は比較対象として残すが、P4 lane の主経路は binary writer 直書きに固定する。

## Entry Contract (P4-min4 implementation target)
1. 入力 fixture:
   - `apps/tests/phase29cc_wsm_p4_min_const_return.hako`（新規予定）
2. 出力契約:
   - magic/version 固定
   - section 順序: type -> function -> export -> code
   - `main` export 必須
   - `i32.const <value>; end` body
3. parity 観点:
   - Rust writer helper（`build_minimal_i32_const_wasm`）と `.hako` writer の `.wasm` bytes が一致すること。

## Gate Contract
1. docs lock gate（this commit）:
   - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh`
2. implementation lock gate（next / P4-min4）:
   - fixture compile + bytes parity（Rust helper vs `.hako` writer）
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に統合予定

## Next
- `WSM-P4-min4`: `.hako` writer 最小実装（const-return fixture 1件）と parity smoke を追加して lock する。
