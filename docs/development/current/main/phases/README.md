# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## 現在の Phase

- **Current (ACTIVE)**: Phase 111x selfhost runtime route naming cleanup
- **Phase 110x（LANDED）**: selfhost execution vocabulary SSOT
- **Phase 105（LANDED）**: digit OR-chain LLVM parity regression
- **Phase 104（LANDED）**: loop(true) + break-only digits（read_digits 系）
- **Phase 103（LANDED）**: if-only regression baseline（VM + LLVM EXE）
- **Phase 102（LANDED）**: real-app read_quoted loop regression（VM + LLVM EXE）
- **Phase 100（LANDED）**: Pinned Read‑Only Captures
- **Phase 99（LANDED）**: Trim/escape 実コード寄り強化（VM+LLVM EXE）
- **Phase 98（LANDED）**: Plugin loader fail-fast + LLVM parityの持続化
- **Phase 97（LANDED）**: LLVM EXE parity for MiniJsonLoader fixtures
- **Phase 96（LANDED）**: MiniJsonLoader next_non_ws loop E2E lock
- **Phase 95（LANDED）**: json_loader escape loop E2E lock
- **Phase 94（LANDED）**: escape route P5b `ch` reassignment E2E（`tools/selfhost/test_pattern5b_escape_minimal.hako` を strict VM E2E で固定）
- **Phase 93x（LANDED）**: archive-later engineering helper sweep

### Recent Landed

- **Phase 90x（LANDED）**: current-doc/design stale surface hygiene
- **Phase 89x（LANDED）**: next source lane selection
- **Phase 88x（LANDED）**: archive/deletion rerun（true archive-ready/delete-ready は出ず、no-op closeout）
- **Phase 87x（LANDED）**: embedded snapshot / wrapper repoint rerun
- **Phase 86x（LANDED）**: phase index / current mirror hygiene
- **Phase 85x（LANDED）**: next source lane selection
- **Phase 84x（LANDED）**: runner wrapper/source contract thinning
- **Phase 83x（LANDED）**: selfhost top-level facade/archive decision
- **Phase 82x（LANDED）**: next source lane selection

### Important Corridors

- **Phase 79x（LANDED）**: launcher emit_mir residual blocker follow-up（focused launcher probe red を source 側で解消）
- **Phase 69x–67x（LANDED）**: runner/selfhost folder recut（product/keep/reference, authority/compat/facade, selfhost split）
- **Phase 63x（LANDED）**: rust-vm final retirement decision（mainline retirement achieved / residual explicit keep frozen）
- **Phase 47x–42x（LANDED）**: stage0/runtime direct-core migration corridor

### Deeper History

- older landed phases remain in their `phase-*` folders
- `phase-29cc` remains the long-range Rust -> `.hako` migration orchestration track
- older `DONE/planned` items are preserved in git history and phase-local documents, not repeated here

## Phase フォルダ構成（推奨）

```
phases/phase-131/
├── README.md                          (Phase 全体概要)
├── 131-03-llvm-lowering-inventory.md (LLVM 部分のテスト・検証)
├── 131-11-case-c-summary.md          (Case C 実装サマリー)
└── phase131-11-case-c-root-cause-analysis.md (根本原因分析)
```

## 参照方法

1. **現在の Phase を知りたい** → [../10-Now.md](../10-Now.md)
2. **該当 Phase を詳しく知りたい** → フォルダを開く
3. **設計背景を知りたい** → [../design/](../design/README.md)
4. **調査ログを見たい** → [../investigations/](../investigations/README.md)

## Phase 命名規則

- **ファイル名**: `phase-<N>-<title>/` (例: `phase-131/`)
- **文書名**: `<N>-<NN>-<topic>.md` (例: `131-11-case-c-summary.md`)
  - Phase 番号で自然にソート可能
  - 同一 Phase 内で段階的に追跡可能

## 作成ルール（SSOT）

詳しくは [../DOCS_LAYOUT.md](../DOCS_LAYOUT.md) を参照。

- ✅ **置き場所**: `phases/phase-<N>/` 配下のみ
- ✅ **内容**: Phase の実装記録・進捗・チェックリスト・検証結果
- ❌ **避けるべき**: 複数 Phase で参照される設計・アーキテクチャ（→ design/ へ）

---

**最終更新**: 2026-04-05
