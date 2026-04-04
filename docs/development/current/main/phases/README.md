# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## 現在の Phase

- **Current (ACTIVE)**: Phase 69x rust runner product/keep/reference recut
- **Phase 69x（ACTIVE）**: rust runner product/keep/reference recut（src/runner の product / keep / reference 読みを tree で揃える）
- **Phase 68x（LANDED）**: .hako runner authority/compat/facade recut（lang/src/runner の authority / compat / facade / entry 読みを tree で揃える）
- **Phase 67x（LANDED）**: selfhost folder split（tools/selfhost の mainline / proof / compat / lib split を tree にした）
- **Phase 66x（LANDED）**: next source lane selection（phase-65x handoff 後の次 source lane と folder-first corridor を選んだ）
- **Phase 65x（LANDED）**: stage1/selfhost mainline hardening（`.hako` / Stage1 authority cluster と shell contract owner を mainline 観点で固めた）
- **Phase 64x（LANDED）**: next source lane selection（rust-vm retirement corridor の結果を受けて次の source lane を選んだ）
- **Phase 63x（LANDED）**: rust-vm final retirement decision（mainline retirement achieved / full source retirement deferred / residual explicit keep frozen を確定）
- **Phase 62x（LANDED）**: rust-vm delete-ready removal wave（caller-zero / explicit replacement が証明されたものだけを狭く remove する想定だったが、実際は no-op で closeout）
- **Phase 61x（LANDED）**: residual rust-vm caller-zero audit rerun（phase-60x prune 後の caller-zero facts を source-backed に再監査した）
- **Phase 60x（LANDED）**: proof/compat keep pruning continuation（残っている explicit keep bucket をさらに狭めた）
- **Phase 59x（LANDED）**: rust-vm route-surface retirement continuation（route/default/help exposure をさらに狭めた）
- **Phase 58x（LANDED）**: next source lane selection（phase-57x の audit 結果を受けて successor lane を決めた）
- **Phase 57x（LANDED）**: rust-vm delete-ready audit / removal wave（keep-now / archive-later / delete-ready を source-backed に切り分け、broad source deletionなしで closeout）
- **Phase 56x（LANDED）**: proof/compat keep pruning（explicit keep のまま残している rust-vm surfaces を削れる形まで狭めた）
- **Phase 55x（LANDED）**: rust-vm route-surface retirement prep（route/default/help surfaces から rust-vm selectable feeling を外した）
- **Phase 54x（LANDED）**: next source lane selection（phase-53x handoff を受けて successor lane と retirement corridor を確定した）
- **Phase 53x（LANDED）**: residual VM source audit（残っている rust-vm / vm-hako source surfaces を inventory して keep-now / archive-later / delete-ready に分け、phase-54x に handoff した）
- **Phase 52x（LANDED）**: archive historical labeling polish（archive / historical wording を最小化し、legacy traces を historical-only に保ち、phase-53x に residual source audit を handoff した）
- **Phase 51x（LANDED）**: compat-codegen archival sweep（canonical compat-codegen payload / wrapper bucket を archive 側へ退避し、live docs / aliases を整理した）
- **Phase 50x（LANDED）**: rust-vm source/archive cleanup（残っている rust-vm / vm-gated source と helper surface を inventory して keep / archive / delete-ready に分ける）
- **Phase 49x（LANDED）**: legacy wording / compat route cleanup（current docs / guides / helper comments that still read like `rust-vm` is a day-to-day owner を rewrite した）
- **Phase 48x（LANDED）**: smoke/source cleanup（残っている `--backend vm` smoke / helper / docs examples を inventory して clean up した）
- **Phase 47x（LANDED）**: stage0/runtime direct-core finalization（day-to-day helper-route defaults are now off `--backend vm`）
- **Phase 46x（LANDED）**: next source lane selection（残っている live VM pressure を棚卸しして、後継 lane に `stage0/runtime direct-core finalization` を選んだ）
- **Phase 45x（LANDED）**: vm residual cleanup（`rust-vm` の残り owner surfaces を proof/oracle/compat keep に縮める）
- **Phase 44x（LANDED）**: stage0 direct/core follow-up（live stage0/selfhost owners を direct/core route 側へ寄せた）
- **Phase 43x（LANDED）**: next source lane selection（phase-42x closeout 後の successor lane として `direct/core follow-up` を選んだ）
- **Phase 42x（LANDED）**: vm caller starvation / direct-core owner migration（day-to-day caller を vm-gated routes から外し、direct/core owner へ寄せる）
- **Phase 41x（LANDED）**: stage0 direct/core route hardening（remaining direct/core route ownership を harden し、proof-only VM gates と compat keep を固定する）
- **Phase 40x（LANDED）**: stage0 vm archive candidate selection（drained vm-facing shims と stale compat wrappers を archive/delete した）
- **Phase 39x（LANDED）**: stage0 vm gate thinning（bootstrap/source route で `--backend vm` 残面を inventory して direct route と keep gate を分ける）
- **Phase 38x（LANDED）**: cleanup/archive sweep（legacy embedded smoke archive first, then delete-ready shim sweep）
- **Phase 37x（LANDED）**: bootstrap owner split（`selfhost_build.sh` / `build.rs` first, speed-first）
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
