ScopeBox and MIR Scope Hints (Dev/CI option)

Overview
- ScopeBox is an optional, compile-time-only wrapper that makes lexical scopes explicit in the AST for diagnostics and macro visibility. It is a no-op for execution: MIR lowering treats ScopeBox like a normal block and semantics are unchanged.

How to enable
- Inject ScopeBox wrappers during core normalization by setting:
  - `NYASH_SCOPEBOX_ENABLE=1`
  - Selfhost compiler path: Runner maps `NYASH_SCOPEBOX_ENABLE=1` to child arg `--scopebox` and applies a JSON prepass via `apps/lib/scopebox_inject.hako`（現状は恒等: 構文確認のみ）。
- Injection points:
  - If.then / If.else bodies
  - Loop.body
  - Bare blocks are represented by `Program { statements }` and still pass
    through builder scope metadata hooks.

MIR Scope Metadata
- The old standalone MIR hint trace envs (`NYASH_MIR_HINTS`,
  `NYASH_MIR_TRACE_HINTS`) were retired with `src/mir/hints.rs`.
- ScopeBox remains a compile-time-only structure aid; builder scope metadata is
  internal and no longer exported as a separate user-facing trace contract.

Zero-cost policy
- ScopeBox is removed implicitly during MIR lowering (treated as a block).
  Execution and IR are unchanged.

Notes (Selfhost path)
- 当面は JSON v0 に `ScopeBox` 型は導入しない（互換維持）。前処理は恒等（identity）から開始し、将来は安全な包み込み/ヒント付与を検討する。
