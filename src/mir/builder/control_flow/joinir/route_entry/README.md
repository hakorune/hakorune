# route_entry

- active module surface は `crate::mir::builder::control_flow::joinir::route_entry` です。
- physical path も `src/mir/builder/control_flow/joinir/route_entry/` に同期済みです。
- historical phase docs に残る `joinir/patterns/` は旧 on-disk path の traceability note です。
- path lane の方針は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照してください。
- route_entry は入口互換とルーティングのみを担当します。
- 意味論・契約・lowering は plan/recipe/parts 側に集約済みです。
- re-export は mod.rs に集約しています。
