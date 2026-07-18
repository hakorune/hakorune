# Plan Normalizer Modules (Legacy/Analysis Scope)

This directory splits facts/recipe contract -> CorePlan normalization into small modules.

Responsibilities:
- Keep legacy route-specific knowledge localized per module.
- Share small, stable helpers (AST lowering, block layouts, phi bindings).
- Composer/entry 経路では使わない（Legacy/Analysis 目的に限定）。
- Do not re-run facts/canon analysis; consume facts/canon inputs only.
- Runtime 側で残す helper は semantic alias を優先し、legacy file/type 名は注記付きで閉じ込める。

Forbidden:
- Do not re-parse AST for shape detection (facts/canon only).
- Do not list retired route files as active modules.
- Do not add new route-specific normalizer files for CorePlan acceptance; use
  skeletons/features/parts/recipe_tree and update REGISTRY instead.

Modules:
- add_result_representation.rs: pure, route-neutral CorePlan Add result
  representation decision; it owns no allocation or production connection.
- ../expression_port.rs: one sealed, stack-scoped child-demand port. The raw
  implementation delegates to the existing PATH0 role vocabulary and always
  yields `Unlocated`; the test-only located implementation validates its
  source-view brand before exposing an exact MethodCall site. It owns no path
  table, target/ABI fact, ledger claim, or Builder state.
- helpers.rs: shared block layouts, phi bindings, AST-to-ValueId lowering helpers.
- common.rs: small normalizer helpers shared across loop-cond pipelines.
- cond_lowering_*.rs: header/body/value condition lowering entry points.
- loop_body_lowering.rs: statement/effect lowering helpers used by loop pipelines.

P0a boundary:
- `lower_value_ast` is the raw facade over the port-driven value normalizer.
- GenericLoop composer threading and boolean/short-circuit condition descent
  remain P0b work; BlockExpr preludes and explicit externcall are outside the
  first located Loop profile. A located consumer must not silently enter those
  raw helpers.

Retired:
- loop_break.rs: removed in 291x-711; break expansion belongs in ExitMap /
  feature pipelines, not a test-only normalizer shelf.
- simple_while_coreloop_builder.rs: removed; simple-while expansion belongs in
  generic loop skeleton/features or a documented FlowPlanner row.
