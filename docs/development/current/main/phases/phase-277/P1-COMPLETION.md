# Phase 277 P1: PHI順序検証強化（fail-fast）— 完了報告

Status: ✅ completed (2025-12-22)

Goal:
- strict mode（`NYASH_LLVM_PHI_STRICT=1`）で、PHI順序/配線失敗を “原因箇所で止める” ようにする。

Scope:
- LLVM harness（Python）側のみ
- 新しい環境変数は追加しない（Phase 277 P2 で統合した3つのみ）
- JoinIR/Rust側のパイプライン統一は対象外（根治は Phase 279）

---

## Changes

### 1) PHI created after terminator を strict で fail-fast

Target:
- `src/llvm_py/phi_wiring/wiring.py`（`ensure_phi`）

Behavior:
- strict=ON: RuntimeError（block_id/dst_vid を含む）
- strict=OFF: warning のみ（既定挙動維持）

### 2) PHI incoming 解決の “silent fallback 0” を strict で禁止

Target:
- `src/llvm_py/phi_wiring/wiring.py`（`wire_incomings`）

Behavior:
- strict=ON: “incoming unresolved / type coercion failed” を RuntimeError
- strict=OFF: 従来どおり 0 fallback（互換維持）

### 3) PHI ordering verifier を実行導線に接続

Target:
- `src/llvm_py/builders/function_lower.py`
- `src/llvm_py/phi_placement.py::verify_phi_ordering`

Behavior:
- strict=ON: ordering NG を RuntimeError（block list を含む）
- debug=ON: ordering のサマリを stderr に出力

---

## Verification

- strict=OFF: 既存 fixture が退行しないことを確認
- strict=ON: 既存 fixture が正常系として PASS（違反がないことを確認）
- debug=ON: verify の接続がログで確認できることを確認

---

## Notes / Next

- 旧 env var の後方互換性削除は Phase 278。
- “2本のコンパイラ（型伝播パイプライン差）” 根治は Phase 279。
