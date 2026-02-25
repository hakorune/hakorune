# Phase 277 P1: PHI順序検証を fail-fast 導線に接続する（Claude Code 指示書）

Status: instructions / validation

目的:
- PHI の “順序違反/配線欠落/型不整合” を **原因箇所で止める**（fail-fast）。
- strict mode（`NYASH_LLVM_PHI_STRICT=1`）が “実際に効く” 状態にする。

重要な制約:
- Phase 277 P1 は **LLVM harness（Python）側の検証強化**が主。JoinIR/Rust の型伝播パイプラインには踏み込まない（根治は Phase 279）。
- env var 増殖禁止（既存の3つのみ）。
- by-name hardcode 禁止（特定関数名の例外分岐などは増やさない）。

参照:
- 検証方針: `docs/development/current/main/phases/phase-277/P1-VALIDATION.md`
- PHI placeholder SSOT: `src/llvm_py/phi_wiring/wiring.py::ensure_phi`
- PHI ordering verifier: `src/llvm_py/phi_placement.py::verify_phi_ordering`
- 実行導線（現状SSOT）: `src/llvm_py/builders/function_lower.py`（`_finalize_phis` 経路）
- env vars SSOT: `src/llvm_py/phi_wiring/debug_helper.py`

---

## Step 1: strict mode で “PHIが遅く作られた” を即死にする

対象:
- `src/llvm_py/phi_wiring/wiring.py::ensure_phi`

現状:
- `bb.terminator` がある状態で PHI を作ろうとすると warning を出すだけ。

P1の変更:
- `NYASH_LLVM_PHI_STRICT=1` のときは fail-fast:
  - 例: `raise RuntimeError(...)`（block_id/dst_vid を含める）
- strict 以外は従来どおり warning + 継続（既定挙動を壊さない）。

Acceptance:
- “PHI after terminator” が strict で必ず落ちる。
- エラー文に `block_id`, `dst_vid`, “next file” を含める（迷子防止）。

---

## Step 2: strict mode で “fallback 0” を禁止する

対象候補（実態に合わせて最小1箇所から）:
- `src/llvm_py/phi_wiring/wiring.py::wire_incomings`
- もしくは `src/llvm_py/llvm_builder.py::finalize_phis`（ローカル実装が残っているので注意）

方針:
- incoming が解決できずに `0` を入れる分岐があるなら、strict で Err にする。
- Err には `block_id/dst_vid/pred_bid` を必ず含める。

注意:
- どの finalize 経路が SSOT かを明確にする（現状は `builders/function_lower.py` が実行導線）。
- “2本の finalize 実装” を統合するのは Phase 279 のスコープ。P1では SSOT 経路に検証を接続する。

Acceptance:
- strict で silent fallback が残っていない（少なくとも PHI incoming の “解決不能→0” は落ちる）。

---

## Step 3: `verify_phi_ordering()` を実行導線に接続する

対象:
- `src/llvm_py/phi_placement.py::verify_phi_ordering(builder)`

現状:
- 定義されているが、実行導線から呼ばれていない。
- llvmlite は reorder ができないため、verify/report の位置が重要。

接続点（推奨）:
- `src/llvm_py/builders/function_lower.py` の関数 lowering の終盤:
  - `lower_terminators(...)` の後（全命令が出揃った後）
  - strict のときは NG を Err にする
  - debug のときは block ごとのサマリを stderr に出す（`NYASH_LLVM_DEBUG_PHI=1`）

Acceptance:
- strict で ordering NG を確実に検出して落とせる。
- debug で NG block の数と block_id が出る（過剰ログは避ける）。

---

## Step 4: 最小の回帰確認

目的:
- “検証が増えたせいで全部が落ちる” を避けつつ、狙った違反を確実に捕まえる。

推奨:
- 代表 fixture を1つ選び、まず strict=OFF で PASS、strict=ON でも PASS を確認（正常系）。
- 既知の壊れ方（PHI late create / missing incoming）を意図的に起こす最小再現があるなら、それで strict で落ちることも確認。

No new env vars.

---

## Completion criteria

- strict mode が “順序違反/配線欠落” を原因箇所で fail-fast できる
- `verify_phi_ordering()` が実行導線に接続されている
- 既定（strict=OFF）での挙動は壊さない
- Phase 279（根治）へ繋がる前提が docs で明確になっている
