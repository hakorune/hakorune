---
Status: SSOT
Scope: `src/mir/loop_pattern_detection/` physical path の rename-candidate inventory と実行条件
Decision: accepted
Related:
- docs/development/current/main/design/route-physical-path-legacy-lane-ssot.md
- src/mir/mod.rs
- src/mir/loop_route_detection/mod.rs
- CURRENT_TASK.md
---

# Loop Route Detection Physical Path Retirement (SSOT)

目的:
- `loop_route_detection` physical rename の完了状態と、残る compatibility alias / historical path token を docs-first で管理する。
- caller / doc pin / compatibility alias の撤去条件を先に固定する。
- rename phase を BoxShape として閉じ、BoxCount や route acceptance の変更を混ぜない。

## Current State

- active module surface:
  - `crate::mir::loop_route_detection`
- compatibility alias:
  - `crate::mir::loop_pattern_detection`
- current physical path:
  - `src/mir/loop_route_detection/`
- historical physical path token:
  - `src/mir/loop_pattern_detection/`
- tree size snapshot at rename time:
  - classifier / features / kind / tests
  - `legacy/` helper lane
  - total files seen in top tree: 22

## Inventory

### active-current

- [src/mir/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/mod.rs)
  - current module declaration は `loop_route_detection`
  - `loop_pattern_detection` は compatibility alias として残っている
- [src/mir/loop_route_detection/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/loop_route_detection/mod.rs)
  - active surface / compatibility alias / current physical path を明記
- [route-physical-path-legacy-lane-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/route-physical-path-legacy-lane-ssot.md)
  - remaining legacy physical path lane として current inventory を保持
- [normalized-dev-removal-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/normalized-dev-removal-ssot.md)
  - `loop_route_detection::legacy` residue cleanup の pointer として参照

### historical-nonarchive

- [CURRENT_TASK.md](/home/tomoaki/git/hakorune-selfhost/CURRENT_TASK.md)
  - slice log / verification command / synced-files 履歴
- `docs/development/current/main/phases/**`
  - 多数の path pin が残る
- `docs/development/current/main/investigations/**`
  - old path traceability を保持

### archive-only

- `docs/archive/**`
  - old path rewrite の対象外

## Rename Preconditions

物理 rename を始める前に、次を満たすこと。

1. active-current docs が module surface-first で固定されている
2. compatibility alias をどこまで残すか決まっている
3. `docs/development/current/main/phases/**` の old path pin を historical lane として扱う方針が固定されている
4. rename diff を build + fast gate + probe で一度に検証できる

## Proposed Phase Order

1. inventory freeze
   - current caller / doc pin / alias use を固定する
2. module-surface inversion
   - 完了済み
   - `src/mir/mod.rs` で `loop_route_detection` を current declaration にした
3. physical rename prep
   - remaining active-current path token を 0 に近づける
4. physical rename
   - 完了済み
   - `src/mir/loop_pattern_detection/` -> `src/mir/loop_route_detection/`
   - compatibility alias は必要最小限だけ残した
5. alias retirement
   - `crate::mir::loop_pattern_detection` caller が 0 になった時点で閉じる

## Non-goals

- historical phase docs の一括 rewrite
- `legacy/` helper lane の意味変更
- route acceptance / classifier behavior の変更
