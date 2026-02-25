# Phase 277 P0: PHI型推論ドキュメント整備（design-first）

Status: planned / docs

Goal: Phase 275/276 で導入・修正された PHI 型推論（MIR→LLVM harness）について、導線・責務・SSOT を “迷子にならない形” で固定する。

Scope:
- 実装のリファクタではなく **ドキュメントSSOT**。
- “どのファイルが何の責務か” を明確にし、次回のデバッグで「触る場所」を 1 本化する。

Non-goals:
- 新しい env var 追加
- PHI アルゴリズム変更（仕様変更）
- 既定挙動変更

---

## 1) SSOT の入口を定義する

必ず最初に入口を 1 箇所に固定する：
- `docs/development/current/main/phases/phase-277/README.md` を「入口SSOT」にする
- PHI 関連の “概念” と “コードの参照先” を README から辿れるようにする

---

## 2) “2本のパイプライン” を防ぐ説明を入れる

このセッションで顕在化した事故：
- ルートAでは BinOp 型伝播→PHI 型解決
- ルートBでは PHI 型解決→BinOp 型伝播

結果：
- 同じ fixture が片方で PASS、片方で FAIL（実質 “2本のコンパイラ”）

P0 では、これを README で明示し、根治フェーズ（Phase 279）へ誘導する文言を固定する。

---

## 3) コード責務マップ（最低限）

以下の “責務の箱” を docs に書き出す（箇条書きでよい）：

- **MIR 側（型情報のSSOT）**
  - `MirInstruction.dst_type` の意味（instruction-local SSOT）
  - `value_types`（propagation/inference の結果）の意味（analysis SSOT）
  - PHI の `dst_type` の意味（PHI-local SSOT）

- **JoinIR / bridge 側**
  - “どのルートで type propagation が走るか”
  - “どの段で value_types が更新されるか”

- **LLVM harness 側（消費者）**
  - `type_helper.py` が SSOT であること（Phase 276 P0）
  - `dst_type_to_llvm_type` の contract（`f64` / `i64` handle / `void`）

---

## 4) デバッグ導線（最小）

“迷子防止” のため、以下を docs に固定する：
- PHI関連の推奨 env var（Phase 277 P2 の統合版）
  - `NYASH_LLVM_DEBUG_PHI=1`
  - `NYASH_LLVM_DEBUG_PHI_TRACE=1`
  - `NYASH_LLVM_PHI_STRICT=1`
- 典型的な確認コマンド（1〜2本だけ）
- 失敗時に「次に見るファイル」を 1 行で指示（type_helper → wiring → resolver の順など）

---

## 5) 完了条件

- README から “PHI型推論の導線” が 1 本で読める
- “2本のパイプライン” の危険と、根治フェーズ（Phase 279）へのリンクが明示されている
- 既存 docs との矛盾がない（Phase 275/276/277 の整合）
