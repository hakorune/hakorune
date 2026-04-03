# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## 現在の Phase

- **Current (ACTIVE)**: Phase 37x bootstrap owner split
- **Phase 37x（ACTIVE）**: bootstrap owner split（`selfhost_build.sh` / `build.rs` first, speed-first）
- **Phase 36x（LANDED）**: selfhost source / stage1 bridge split（`selfhost.rs` source prepare / `stage1_cli` raw bridge first）
- **Phase 35x（LANDED）**: stage-a compat route thinning（`selfhost.rs` / `stage_a_compat_bridge.rs` first）
- **Phase 34x（LANDED）**: stage0 shell residue split（`child.rs` / `stage1_cli` / `core_executor` first）
- **Phase 33x（LANDED）**: shared helper family recut（`hako_check` / `emit_mir` helper-family truth fixed）
- **Phase 32x（LANDED）**: product / engineering split（mixed-owner source/smoke split, `build.rs` / `phase2100` first）
- **Phase 31x（LANDED）**: engineering lane isolation（`tools/engineering/**` への rehome / shim drain / source-smoke sweep）
- **Phase 30x（LANDED）**: backend surface simplification（`llvm/exe` product main / `rust-vm` engineering / `vm-hako` reference / `wasm` experimental）
- **Phase 29x（LANDED）**: backend owner cutover / explicit helper deletion / semantic proof-home recut
- **Phase 29cc（ACTIVE）**: Rust -> .hako migration orchestration（M0-M4 fixed order）
- **Phase 139（DONE）**: post-if `post_k` の return lowering を `ReturnValueLowererBox` に統一（出口 SSOT 完成）
- **Phase 140（DONE）**: `NormalizedExprLowererBox` 初版（pure expression のみ）
- **Phase 141 P0（DONE）**: impure 拡張点（contract）を SSOT 化（Call/MethodCall はまだ out-of-scope）
- **Phase 141 P1（DONE）**: “既知 intrinsic だけ” を許可して段階投入（length0）
- **Phase 141 P1.5（DONE）**: known intrinsic registry + available_inputs 3-source merge + diagnostics
- **Phase 141 P2+（planned）**: Call/MethodCall 対応（effects + typing の段階投入）
- **Phase 142-loopstmt P0（DONE）**: 正規化単位を statement（loop 1個）へ寄せる（パターン爆発を止める）
- **Phase 142-loopstmt P1（DONE）**: LLVM EXE smoke（同 fixture）を追加
- **Phase 143-loopvocab（planned）**: StepTree の語彙拡張（loop 内 if/break/continue を「語彙追加」で吸収）
- **Phase 91–92**: Selfhost depth‑2 coverage（P5b escape recognition → lowering）
- **Phase 94–100**: P5b escape E2E / Trim policy / pinned + accumulator（VM/LLVM EXE parity）
- **Phase 102**: real-app read_quoted loop regression（VM + LLVM EXE）
- **Phase 103**: if-only regression baseline（VM + LLVM EXE / plan）
- **Phase 113**: if-only partial assign parity（片側代入の保持 merge）
- **Phase 107–109**: real-app depth-scan / policy router SSOT / error hint SSOT
- **Phase 110–112**: ControlTree / StepTree（構造SSOT, dev-only）※設計SSOTは `../design/control-tree.md`

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
