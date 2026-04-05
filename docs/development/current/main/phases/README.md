# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## 現在の Phase

- **Current (ACTIVE)**: Phase 149x concat const-suffix vertical slice
- **Phase 150x（PLANNED）**: array string-store vertical slice
- **Phase 151x（PLANNED）**: canonical lowering visibility lock
- **Phase 149x（ACTIVE）**: concat const-suffix vertical slice
- **Phase 148x（LANDED）**: borrowed text and sink contract freeze
- **Phase 147x（LANDED）**: semantic optimization contract selection
- **Phase 146x（LANDED）**: string semantic boundary tighten
- **Phase 145x（LANDED）**: compat quarantine shrink
- **Phase 144x（LANDED）**: string semantic owner follow-up
- **Phase 143x（LANDED）**: map owner cutover implementation
- **Phase 141x（LANDED）**: string semantic boundary review
- **Phase 140x（LANDED）**: map owner pilot
- **Phase 139x（LANDED）**: array owner pilot
- **Phase 138x（LANDED）**: nyash_kernel semantic owner cutover
- **Phase 137x（PAUSED）**: main kilo reopen selection
- **Phase 134x（LANDED）**: nyash_kernel layer recut selection
- **Phase 133x（LANDED）**: micro kilo reopen selection
- **Phase 132x（LANDED）**: vm default backend decision
- **Phase 131x（LANDED）**: vm legacy contract migration
- **Phase 130x（LANDED）**: vm public gate final cleanup
- **Phase 129x（LANDED）**: vm orchestrator/public gate follow-up
- **Phase 128x（LANDED）**: stage1 bridge vm gate softening
- **Phase 126x（LANDED）**: vm public gate shrink decision
- **Phase 125x（LANDED）**: vm bridge/backend gate follow-up
- **Phase 124x（LANDED）**: vm public docs/manual demotion
- **Phase 123x（LANDED）**: proof gate shrink follow-up
- **Phase 122x（LANDED）**: vm compat route exit plan
- **Phase 121x（LANDED）**: vm backend retirement gate decision
- **Phase 120x（LANDED）**: vm route retirement decision refresh
- **Phase 119x（LANDED）**: vm debug/observability surface review
- **Phase 118x（LANDED）**: proof wrapper surface review
- **Phase 117x（LANDED）**: vm compat/proof env hardening
- **Phase 116x（LANDED）**: execution surface alias pruning
- **Phase 115x（LANDED）**: vm route retirement planning
- **Phase 114x（LANDED）**: execution surface wording closeout
- **Phase 113x（LANDED）**: kernel vs vm-reference cluster wording correction
- **Phase 112x（LANDED）**: vm-family lane naming hardening
- **Phase 111x（LANDED）**: selfhost runtime route naming cleanup
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

- **Phase 141x（LANDED）**: string semantic boundary review
- **Phase 140x（LANDED）**: map owner pilot
- **Phase 139x（LANDED）**: array owner pilot
- **Phase 138x（LANDED）**: nyash_kernel semantic owner cutover
- **Phase 134x（LANDED）**: nyash_kernel layer recut selection
- **Phase 133x（LANDED）**: micro kilo reopen selection
- **Phase 132x（LANDED）**: vm default backend decision
- **Phase 131x（LANDED）**: vm legacy contract migration
- **Phase 130x（LANDED）**: vm public gate final cleanup
- **Phase 128x（LANDED）**: stage1 bridge vm gate softening
- **Phase 127x（LANDED）**: compat route raw vm cut prep
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
phases/phase-142x/
└── README.md

phases/phase-141x/
├── README.md
├── 141x-90-string-semantic-boundary-review-ssot.md
└── 141x-91-task-board.md
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
