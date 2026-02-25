---
Status: SSOT
Scope: Copy emission policy + contract (materialize/alias only) + direct emission prohibition
Related:
- docs/development/current/main/design/valueflow-blockparams-ssot.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/compiler-task-map-ssot.md
Enforcement:
- tools/checks/no_direct_copy_emission.sh
---

# Copy Emission SSOT

## Decision

`MirInstruction::Copy` の **直生成は原則禁止**（例外はSSOTで明記）。
Copy発行は SSOT 入口（例: `CopyEmitter`）に寄せ、strict/dev+planner_required では
fail-fast（dominance）を一貫して適用できるようにする。

狙い: 「Copyの犯人探し」を終わらせ、責務境界の違反をコンパイル/ゲートで強制する。

- Copy の reason は `CopyEmitReason` enum で固定する（文字列の直書き禁止）。

## Rule (MUST)

- **禁止**: `instructions.push(MirInstruction::Copy { ... })`
- **禁止**: `add_instruction(MirInstruction::Copy { ... })`
- **禁止**: `add_instruction_before_terminator(MirInstruction::Copy { ... })`

例外はこのSSOTに列挙し、かつチェックの allowlist に反映すること（片方だけ更新は禁止）。

## Exceptions (Allowlist)

以下のみ、`Copy` の直生成を許可する（SSOT例外）。
（理由: JSON パーサ/fixture は IR を「構築」する責務であり、CopyEmitter を経由しない）

- `src/mir/builder/emission/copy_emitter.rs`（SSOT入口の実装本体。内部では Copy を add_instruction する）
- `src/runner/mir_json_v0.rs`
- `src/runner/json_v1_bridge/parse.rs`
- `tests/**` and `**/tests/**` (test-only fixtures)

## Status / Migration progress (informational)

- SSOT entrypoint: `src/mir/builder/emission/copy_emitter.rs`
- Migrated (partial; CopyEmitter 経由):
  - JoinIR bridge: ConditionalMethodCall else-copy
  - JoinIR bridge: IfMerge (converter; then/else) merge copy
  - JoinIR bridge: IfMerge handler copies
  - JoinIR bridge: NestedIfMerge handler copies
  - JoinIR bridge: NestedIfMerge (converter; then) merge copy
  - JoinIR bridge: merge-variable SSOT-ready entry (`emit_merge_copies_in_func`)
  - JoinIR merge rewriter: tail-call parameter binding copies (`tail_call_rewrite.rs`)

## Enforcement

このルールはリポジトリ側で強制する:

- `tools/checks/no_direct_copy_emission.sh`

スクリプトは allowlist 外での直Copyを検出した場合に FAIL する。

## Plan (SSOT; near-term)

“好き勝手に Copy を作る” を止めるため、次の順で進める（1コミット=1ファイルの粒度）。

1) CopyEmitter の挿入点APIを揃える（`in_block` / `before_terminator` / `after_phis`）
   - 追加: “detached block（後から func に付ける BasicBlock）” への Copy 発行も CopyEmitter に寄せる
     - dominance 検査は不可能なので、Verifier/後段の in-func 入口で担保する
2) `CopyEmitReason` を enum 化して SSOT 化（文字列 reason を縮退、typo を防止）【完了】
3) `tools/checks/no_direct_copy_emission.sh` を PASS する状態まで、残存直Copyを段階移行する

## Rationale (SSOT pointers)

- merge の意味論（ValueFlow）は Copy/PHI で表さない:
  - SSOT: `valueflow-blockparams-ssot.md`
- Verifier は修正しない（検出のみ）。壊れた SSA を Copy で救済しない:
  - SSOT: `ai-handoff-and-debug-contract.md`
