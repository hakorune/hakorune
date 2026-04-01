---
Status: SSOT
Scope: route-related active module surface と remaining legacy physical path の分離
Decision: accepted
Related:
- docs/development/current/main/design/joinir-design-map.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/plan-lowering-entry-ssot.md
- docs/development/current/main/design/loop-route-detection-physical-path-retirement-ssot.md
- src/mir/builder/control_flow/joinir/route_entry/README.md
- src/mir/loop_route_detection/mod.rs
---

# Route Physical Path Legacy Lane (SSOT)

目的:
- active module surface と on-disk path を混同しない。
- `joinir::route_entry` と `crate::mir::loop_route_detection` を current runtime の主語に固定する。
- route-related old physical path token（例: `joinir/patterns/`, `loop_pattern_detection/`）は historical/traceability note としてだけ扱う。

## Rule

- active docs / current guidance は module surface を先に書く。
- physical path を書く必要がある場合は、current か historical かを明示する。
- historical / phase docs の old path pin は、この SSOT を起点に分類する。
- on-disk rename は、caller inventory と historical drift を棚卸ししてから別 phase で行う。

## Inventory

| Area | Active module surface | Current physical path | Historical physical path | Current role | Rename stance |
|---|---|---|---|---|---|
| JoinIR route entry | `crate::mir::builder::control_flow::joinir::route_entry` | `src/mir/builder/control_flow/joinir/route_entry/` | `src/mir/builder/control_flow/joinir/patterns/` | thin routing / registry / wrapper lane | 2026-03-07 slice 92 で rename 完了。old path は historical docs / grep traceability のみ |
| Loop route detection | `crate::mir::loop_route_detection` | `src/mir/loop_route_detection/` | `src/mir/loop_pattern_detection/` | structure-based classify + legacy helper lane | 2026-03-07 slice 95-96 で rename + alias retirement 完了。old module/path token は historical docs / grep traceability のみ |

## Lane Notes

### `joinir::route_entry`

Current state:
- active code と current physical path は `route_entry/` に同期済み。
- `joinir/patterns/` はもう live code path ではない。
- phase docs / instruction docs に残る `patterns/router.rs` などは historical physical path pin として読む。

Follow-up:
- active docs では current physical path を優先し、old `joinir/patterns/` は historical note にだけ残す。
- old path token retire は grep contract / archive replay の caller inventory を見ながら別 slice で進める。

### `loop_route_detection`

Current state:
- active code と current physical path は `src/mir/loop_route_detection/` に同期済み。
- old `src/mir/loop_pattern_detection/` は historical physical path token へ後退した。
- old `crate::mir::loop_pattern_detection` module token も historical note へ後退した。
- historical phase docs が `src/mir/loop_pattern_detection/...` を多数 pin している。

Follow-up:
- active docs では current physical path `src/mir/loop_route_detection/` を優先する。
- old `loop_pattern_detection/` / `crate::mir::loop_pattern_detection` token は historical note / grep traceability に限定する。
- future cleanup は historical pin token の圧縮だけを扱う。

## Non-goals

- historical docs の一括 rewrite
- old phase logs の path を current state に合わせて書き換えること
- `joinir/patterns/` historical pin を current path へ一括 rewrite すること
