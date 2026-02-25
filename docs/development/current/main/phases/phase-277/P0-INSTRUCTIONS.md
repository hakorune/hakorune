# Phase 277 P0: PHI型推論 SSOT docs を完成させる（Claude Code 指示書）

Status: instructions / docs-only

目的:
- PHI型推論（MIR→LLVM harness）の導線・責務・SSOT を **1本に固定**し、次回のデバッグで迷子にならないようにする。

スコープ:
- docs のみ（設計と導線の固定）
- 実装変更・仕様変更は行わない

Non-goals:
- “2本のコンパイラ（パイプライン差）” の根治（Phase 279）
- env var 追加（禁止）

入口SSOT:
- `docs/development/current/main/phases/phase-277/README.md`

参照:
- P0設計メモ: `docs/development/current/main/phases/phase-277/P0-DESIGN.md`
- P2完了（env var 統合）: `docs/development/current/main/phases/phase-277/P2-COMPLETION.md`
- env vars: `docs/reference/environment-variables.md`
- type helper SSOT（Phase 276 P0）: `src/llvm_py/phi_wiring/type_helper.py`

---

## Step 1: README を “PHI型推論の地図” にする

`docs/development/current/main/phases/phase-277/README.md` に以下の節を追加/更新して、READMEだけ読めば導線が分かる状態にする。

必須内容（短くてOK）:
- 何が SSOT か（どのファイルが “決める” か）
- どこが consumer か（LLVM harness が何を期待するか）
- どこを見れば原因が特定できるか（迷子防止）

最低限の “責務マップ”:
- MIR 側:
  - `MirInstruction.dst_type`（instruction-local）
  - propagated `value_types`（analysis）
  - PHI `dst_type`（PHI-local）
- LLVM harness 側:
  - PHI env var SSOT: `src/llvm_py/phi_wiring/debug_helper.py`
  - 型取得 SSOT: `src/llvm_py/phi_wiring/type_helper.py`
  - PHI placeholder SSOT: `src/llvm_py/phi_wiring/wiring.py::ensure_phi`
  - 順序検証: `src/llvm_py/phi_placement.py`（現状は verify/report）

注意:
- llvmlite は基本 “命令の並べ替え” ができないことを明記する（PHI-first は生成時に守る）。
- “2本のパイプライン” 問題は Phase 279 へリンクし、P0 で根治しないことを明確化する。

---

## Step 2: デバッグ導線（最小）を README に固定

README に以下を固定する（1〜2コマンドだけ、冗長にしない）:

- 推奨 env var（Phase 277 P2 統合版）
  - `NYASH_LLVM_DEBUG_PHI=1`
  - `NYASH_LLVM_DEBUG_PHI_TRACE=1`
  - `NYASH_LLVM_PHI_STRICT=1`

- 典型コマンド（例）
  - `NYASH_LLVM_DEBUG_PHI=1 NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/<fixture>.hako`

- 失敗時の “次に見るファイル” を 1 行で指示（固定順）
  - `type_helper.py → wiring.py → llvm_builder.py → resolver.py`

---

## Step 3: docs 整合チェック（最小）

確認:
- `docs/development/current/main/10-Now.md` の “次にやる” が Phase 277 を指していること
- `docs/development/current/main/30-Backlog.md` の Phase 277/278/279 が矛盾していないこと

Acceptance:
- README が入口SSOTとして成立（READMEだけで導線が追える）
- P0-DESIGN/P1-VALIDATION/P2-COMPLETION へリンクがある
- 新しい env var を増やしていない
