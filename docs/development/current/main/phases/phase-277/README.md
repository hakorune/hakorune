# Phase 277: PHI関連改善シリーズ

## 概要

Phase 275/276で完了したFloat型PHI対応・型取得SSOT化の後続改善として、PHI関連の環境変数統合・ドキュメント整備を実施。

このPhaseの狙いは「PHIまわりの迷子を無くす」こと：
- どの層が何を決めるか（SSOT）を固定する
- PHI順序/配線の違反を “後段で壊れる” ではなく “原因で止まる” に寄せる
- そして根治として「2本のコンパイラ（パイプライン差による二重バグ）」を Phase 279 で潰せるように導線を引く

入口（関連）:
- Now: `docs/development/current/main/10-Now.md`
- Backlog: `docs/development/current/main/30-Backlog.md`

---

## サブフェーズ一覧

### Phase 277 P0: PHI型推論ドキュメント整備（予定）

- 目的: Phase 275/276で実装したPHI型推論ロジックのドキュメント化
- 内容:
  - MIR型伝播 → LLVM IR型生成のフロー図
  - type_helper.py（LLVM harness 側の型取得SSOT）の設計ドキュメント
  - PHI型推論のベストプラクティス
- 設計メモ（このPhase配下のSSOT案）:
  - `docs/development/current/main/phases/phase-277/P0-DESIGN.md`
- 指示書（Claude Code）:
  - `docs/development/current/main/phases/phase-277/P0-INSTRUCTIONS.md`

### Phase 277 P1: PHI順序検証強化（fail-fast） ✅

- 目的: PHI命令の配置順序検証を強化
- 内容:
  - phi_placement.py の検証ロジック強化
  - LLVM IR仕様準拠チェック（PHI → 非PHI → terminator）
  - 順序違反時のエラーメッセージ改善
- 検証メモ（このPhase配下のSSOT案）:
  - `docs/development/current/main/phases/phase-277/P1-VALIDATION.md`
- 指示書（Claude Code）:
  - `docs/development/current/main/phases/phase-277/P1-INSTRUCTIONS.md`
- 完了報告:
  - `docs/development/current/main/phases/phase-277/P1-COMPLETION.md`

### Phase 277 P2: PHI関連環境変数の統合・整理 ✅

**完了日**: 2025-12-22

- 目的: PHI関連環境変数を **8個 → 3個** に統合
- 完了ドキュメント: `P2-COMPLETION.md`
- 達成内容:
  - ✅ debug_helper.py 作成（環境変数チェックのSSOT）
  - ✅ 3つの統合関数実装（is_phi_debug_enabled 他）
  - ✅ 9ファイル修正完了（wiring.py, tagging.py 他）
  - ✅ 後方互換性対応（Phase 278で削除予定）
  - ✅ ドキュメント更新（environment-variables.md）
- 効果:
  - ユーザビリティ向上（覚える変数 8個→3個、62%削減）
  - 保守性向上（環境変数チェックのSSOT化）
  - ドキュメント簡潔化

---

## 統合後の環境変数（P2完了版）

```bash
# PHI一般デバッグ（生成・型推論・順序）
NYASH_LLVM_DEBUG_PHI=1

# PHI詳細トレース（wiring・vmap変化）
NYASH_LLVM_DEBUG_PHI_TRACE=1

# PHI厳格モード（ゼロフォールバック禁止）
NYASH_LLVM_PHI_STRICT=1
```

**詳細**: `docs/reference/environment-variables.md` の「PHI デバッグ関連」セクション

---

## 関連Phase

- **Phase 275**: Float型PHI対応（MIR型伝播 → LLVM IR double生成）
- **Phase 276**: 型取得SSOT化（type_helper.py）
- **Phase 278**: 後方互換性削除（旧環境変数サポート削除予定）
- **Phase 279**: パイプラインSSOT統一（“2本のコンパイラ” 根治）

---

## ファイル構成

```
phase-277/
├── README.md                 # 本ファイル（Phase 277概要）
├── P0-INSTRUCTIONS.md        # P0指示書（Claude Code）
├── P2-COMPLETION.md          # P2完了報告
├── P0-DESIGN.md              # P0設計ドキュメント（docs）
├── P1-INSTRUCTIONS.md        # P1指示書（Claude Code）
├── P1-VALIDATION.md          # P1検証強化ドキュメント（validation）
└── P1-COMPLETION.md          # P1完了報告
```

---

## 重要なSSOT（どこが何を決めるか）

最小の地図（迷ったらここから辿る）:

- **PHI env var（統合SSOT）**: `src/llvm_py/phi_wiring/debug_helper.py`
  - 使う側は `is_phi_debug_enabled()` / `is_phi_trace_enabled()` / `is_phi_strict_enabled()` だけを見る
  - 旧 env var の撤去は Phase 278

- **LLVM harness 側の型取得SSOT**: `src/llvm_py/phi_wiring/type_helper.py`
  - `get_phi_dst_type(...)` と `dst_type_to_llvm_type(...)` が入口
  - “PHIのdst_typeをどこから取るか” をここに集約する（Phase 276 P0）

- **PHI placeholder を block head に作るSSOT**: `src/llvm_py/phi_wiring/wiring.py::ensure_phi`
  - llvmlite は “後から命令を並べ替える” が基本できない
  - よって PHI は “作るタイミング” が勝負（PHI-first の契約をここで守る）

- **順序検証（verifier）**: `src/llvm_py/phi_placement.py`
  - 現状は “並べ替え” ではなく “検証/レポート” のみ（llvmlite制約）
  - Phase 277 P1 で fail-fast 導線を強化する（strict mode の意味を強くする）

- **根治（パイプライン二重化の解消）**: `docs/development/current/main/phases/phase-279/README.md`

---

## 今後の予定

1. **Phase 277 P0**: PHI型推論ドキュメント整備
2. **Phase 277 P1**: PHI順序検証強化
3. **Phase 278**: 後方互換性削除

---

**Phase 277 P2 完了！** 次はP0/P1の計画策定へ。
