---
Status: Landed
Date: 2026-04-26
Scope: BuildBox remaining BoxShape cleanup order.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-280-buildbox-bundle-resolver-split-card.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/entry/body_extraction_box.hako
---

# 291x-281: BuildBox Remaining Cleanup Order

## Goal

Make the next compiler-clean tasks explicit after `291x-280`.

`BuildBox` is still the live source-to-Program(JSON v0) authority, but several
helper families remain in the entry file. The remaining cleanup should proceed
as small BoxShape cards, not as a broad BuildBox rewrite.

## Ordered Tasks

1. `291x-282`: BuildBox parse-source narrowing SSOT handoff.
   - Replace the duplicate inline main-body scanner in `BuildBox` with
     `BodyExtractionBox.extract_main_body(...)`.
   - Remove BuildBox-local `_find_main_position`,
     `_find_char_skipping_strings`, `_extract_balanced_body`,
     `_extract_main_body`, and `_find_pattern` if no longer used.
   - Keep parser/body fallback semantics unchanged.

2. `291x-283`: BuildBox bundle input collector split.
   - Move opts/env bundle input collection and alias/require CSV parsing to a
     dedicated build-side input box.
   - Keep `BuildBundleResolverBox` as the resolver/merge authority.
   - Keep bundle CLI/env semantics unchanged.

3. `291x-284`: BuildBox defs/imports fragment injector split.
   - Move defs JSON construction, imports conversion, and JSON fragment
     injection to a dedicated build-side fragment box.
   - Keep `BuildBox` as the outer sequencing authority.
   - Do not change Program(JSON v0) shape.

4. `291x-285`: BuildBox facade closeout.
   - Update `lang/src/compiler/build/README.md` after the splits.
   - Confirm BuildBox only sequences source preparation, parser call, and
     Program(JSON v0) enrichment handoff.

## Immediate Next

Start with `291x-282`.

This is the smallest safe cleanup because `BodyExtractionBox` already exists
and `StageBBodyExtractorBox` already uses it. The card should be behavior
preserving and validated with Stage-B binop/min-emit plus current-state guards.

## Non-Goals

- Do not change Stage-B bundle behavior.
- Do not reuse legacy `entry/bundle_resolver.hako` for live BuildBox behavior.
- Do not reopen the residual `MapBox.has` no-growth fallback baseline.
- Do not add CoreMethodContract rows or hot lowering.

## Acceptance

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.
