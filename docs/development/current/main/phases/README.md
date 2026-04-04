# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## 現在の Phase

- **Current (ACTIVE)**: Phase 88x archive/deletion rerun
- **Phase 87x（LANDED）**: embedded snapshot / wrapper repoint rerun（embedded Stage1 snapshot を canonical `facade/*` / `entry/*` へ更新）
- **Phase 86x（LANDED）**: phase index / current mirror hygiene（`phases/README.md` を 117 行から 65 行へ縮小）

### Recent Landed

- **Phase 85x（LANDED）**: next source lane selection（`86x phase index / current mirror hygiene` を選定）
- **Phase 84x（LANDED）**: runner wrapper/source contract thinning（Stage1 build/default entry contracts を canonical `entry/*` stubs へ寄せた）
- **Phase 83x（LANDED）**: selfhost top-level facade/archive decision（top-level selfhost wrappers は explicit public/front-door keep として固定）
- **Phase 82x（LANDED）**: next source lane selection
- **Phase 81x（LANDED）**: caller-zero archive rerun（true archive-ready surface は出ず、no-op closeout）
- **Phase 80x（LANDED）**: root/current pointer thinning（pointer docs を薄く整理）

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

**最終更新**: 2026-02-13
