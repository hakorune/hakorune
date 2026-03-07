# patterns (legacy physical path for route-entry layer)

- active module surface は `crate::mir::builder::control_flow::joinir::route_entry` です。
- `patterns/` は legacy physical path alias として残しています。
- path lane の方針は `docs/development/current/main/design/route-physical-path-legacy-lane-ssot.md` を参照してください。
- route_entry は入口互換とルーティングのみを担当します。
- 意味論・契約・lowering は plan/recipe/parts 側に集約済みです。
- re-export は mod.rs に集約しています。
