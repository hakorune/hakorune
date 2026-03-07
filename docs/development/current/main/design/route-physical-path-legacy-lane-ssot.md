---
Status: SSOT
Scope: route-related active module surface と legacy physical path の分離
Decision: accepted
Related:
- docs/development/current/main/design/joinir-design-map.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/plan-lowering-entry-ssot.md
- src/mir/builder/control_flow/joinir/patterns/README.md
- src/mir/loop_pattern_detection/mod.rs
---

# Route Physical Path Legacy Lane (SSOT)

目的:
- active module surface と on-disk path を混同しない。
- `joinir::route_entry` と `crate::mir::loop_route_detection` を current runtime の主語に固定する。
- `joinir/patterns/` と `src/mir/loop_pattern_detection/` は legacy physical path lane として管理する。

## Rule

- active docs / current guidance は module surface を先に書く。
- physical path を書く必要がある場合は、「legacy physical path」と明示する。
- historical / phase docs の old path pin は、この SSOT を起点に分類する。
- on-disk rename は、caller inventory と historical drift を棚卸ししてから別 phase で行う。

## Inventory

| Area | Active module surface | Legacy physical path | Current role | Rename stance |
|---|---|---|---|---|
| JoinIR route entry | `crate::mir::builder::control_flow::joinir::route_entry` | `src/mir/builder/control_flow/joinir/patterns/` | thin routing / registry / wrapper lane | small tree なので rename candidate。ただし phase/instruction docs が old path を大量に pin しているため、即 rename はしない |
| Loop route detection | `crate::mir::loop_route_detection` | `src/mir/loop_pattern_detection/` | structure-based classify + legacy helper lane | medium-risk rename candidate。tree が大きく、historical docs/notes も多いので dedicated phase が必要 |

## Keep / Rename Conditions

### `joinir/patterns/`

Keep now:
- active code はすでに `joinir::route_entry` surface へ寄っている。
- phase docs / instruction docs が `patterns/router.rs` などの old path を大量に pin している。

Rename later when:
- active docs が physical path note をほぼ不要にできる。
- old path の remaining hits が historical / archive / instruction docs に限定される。
- rename mapping を archive / inventory 側へ固定できる。

### `loop_pattern_detection/`

Keep now:
- active code は `crate::mir::loop_route_detection` surface へ寄っている。
- classifier / legacy helpers / tests を含む tree が大きく、rename diff が広い。
- historical phase docs が `src/mir/loop_pattern_detection/...` を多数 pin している。

Rename later when:
- active docs で module surface-first が定着している。
- historical path pin を archive / inventory に追い出せる。
- dedicated rename phase を切って build + fast gate + probe をまとめて固定できる。

## Non-goals

- historical docs の一括 rewrite
- old phase logs の path を current state に合わせて書き換えること
- この SSOT だけで on-disk rename を開始すること
