# Plan Normalizer Modules (Legacy/Analysis Scope)

This directory splits facts/recipe contract -> CorePlan normalization into small modules.

Responsibilities:
- Keep pattern-specific knowledge localized per module.
- Share small, stable helpers (AST lowering, block layouts, phi bindings).
- Composer/entry 経路では使わない（Legacy/Analysis 目的に限定）。
- Do not re-run facts/canon analysis; consume facts/canon inputs only.

Forbidden:
- Do not re-parse AST for shape detection (facts/canon only).

Modules:
- helpers.rs: shared block layouts, phi bindings, AST-to-ValueId lowering helpers.
- pattern2_break.rs: Pattern2 break-loop normalization.
- pattern_scan_with_init.rs: Pattern6 scan-with-init normalization.
- pattern_split_scan.rs: Pattern7 split-scan normalization.
