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

Modules:
- helpers.rs: shared block layouts, phi bindings, AST-to-ValueId lowering helpers.
- common.rs: small normalizer helpers shared across loop-cond pipelines.
- cond_lowering_*.rs: header/body/value condition lowering entry points.
- loop_body_lowering.rs: statement/effect lowering helpers used by loop pipelines.
- value_join_args.rs: value-join argument normalization helpers.
- simple_while_coreloop_builder.rs: simple-while coreloop scaffold helper.
- loop_break.rs: test-only loop-break normalization harness.
