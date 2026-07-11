# Phase 277 P1: PHI順序検証強化（validation）

Status: planned / validation

Goal: PHI placement/order を fail-fast で検出しやすくし、LLVM harness の “後段で壊れる” ではなく “原因箇所で止まる” を実現する。

Scope:
- 検証とエラーメッセージの改善（実装は最小・局所）
- “順序違反” と “型不整合” の可観測性を上げる

Non-goals:
- 新しい env var 追加
- 大規模なパイプライン統一（Phase 279）

---

## 1) 何を検証するか（契約SSOT）

最低限、この契約を SSOT として明文化する：

- **Block内順序**:
  - PHI 群
  - non-PHI 命令群
  - terminator（Branch/Jump/Return）
  - この順序以外は “バグ” として扱う

- **PHI 入力の完全性**:
  - incoming が欠ける場合は fail-fast（既定で silent fallback をしない）
  - strict mode（`NYASH_LLVM_PHI_STRICT=1`）では必ず Err

- **型整合**:
  - `dst_type` と実際に生成する LLVM type が一致していること
  - mismatch を “CRITICAL” として可視化する（Phase 276 P0 の方針を踏襲）

---

## 2) 実装ポイント（現状コードに合わせた最小）

現状の構造（要点）:
- llvmlite は “命令の並べ替え” が基本できないため、PHI-first は **生成時**に守る必要がある
  - `src/llvm_py/phi_placement.py` は “reorder” ではなく “verify/report” が主
- PHI 配線は `finalize_phis` で行われる（PHI placeholder 作成→incoming 配線）
  - 実際のSSOT呼び出しは `src/llvm_py/builders/function_lower.py` の `_finalize_phis(builder, context)` 経路
  - `NyashLLVMBuilder.finalize_phis()` は別実装が残っており、P1では **どちらをSSOTにするか**を明示する

実装点（推奨）:

1) **“PHIを遅く作ってしまった” を strict で即死**
- 対象: `src/llvm_py/phi_wiring/wiring.py::ensure_phi`
  - すでに `bb.terminator` を検知して warning を出している
  - P1では `NYASH_LLVM_PHI_STRICT=1` のとき、ここを fail-fast（例: `raise` / `unreachable` 相当）にする
- 期待効果: “順序違反の原因” で止まる

2) **fallback 0 の採用を strict で禁止**
- 対象: PHI incoming を解決できず `0` を選ぶ箇所
  - `src/llvm_py/llvm_builder.py` および `src/llvm_py/phi_wiring/wiring.py::wire_incomings` に存在
- P1では strict のとき:
  - “missing snapshot / unresolved” を明示エラーにする
  - エラー文に `block_id / dst_vid / pred_bid` を含める

3) **PHI ordering verifier を “実行経路に接続”**
- 現状 `src/llvm_py/phi_placement.py::verify_phi_ordering(builder)` が未使用
- P1では呼び出し点を 1 箇所に固定する:
  - 候補: `src/llvm_py/builders/function_lower.py` の `lower_terminators(...)` 後
  - strict のときは ordering NG を Err にする
  - debug のときは詳細を stderr に出す（`NYASH_LLVM_DEBUG_PHI=1`）

補足:
- ここで reorder はできないので、verifier は “最後に怒る” ではなく
  “生成時の契約が破られていないことを確認する” 目的で使う

---

## 3) エラーメッセージ（迷子防止）

エラー文は必ず以下を含める：
- block id
- dst ValueId（PHIの対象）
- expected vs actual（型/順序）
- 次に見るファイル（1つ、固定）

推奨:
- ordering なら `src/llvm_py/phi_wiring/wiring.py`（PHI生成の入口）
- missing incoming なら `src/llvm_py/llvm_builder.py`（snapshot/value解決の入口）

---

## 4) 最小テスト

P1 の目的は “検証が働くこと” なので、最小の再現でよい：
- 既存の PHI を含む fixture を 1 つ選ぶ（Phase 275 のものなど）
- strict mode で実行して、違反があれば落ちることを確認する

No new CI jobs.

---

## 5) 完了条件

- PHI順序違反が “原因箇所で” fail-fast する
- strict mode が意味を持つ（silent fallback が残っていない）
- 既存の正常ケース（代表スモーク）が退行しない
