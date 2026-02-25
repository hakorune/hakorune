---
Status: Draft (questionnaire)
Audience: external review (ChatGPT Pro)
Scope: CorePlan / JoinIR loop coverage strategy
---

# Inquiry: CorePlan “unknown loop” strategy — decompose vs general loop support

## Background (current situation)

Nyash/Hakorune has migrated loop lowering to a CorePlan/FlowBox pipeline. LoopBuilder is removed, so any loop shape not
recognized by plan/facts→planner→composer is fail-fast as `[joinir/freeze]`.

This is now biting selfhost tooling (e.g. `tools/hako_check/*`), where “generic scanning loops” exist and trigger
freeze. We want a long-term clean design that stays small, compositional, and verifiable.

## Existing direction (SSOT pointers)

- CorePlan/FlowBox migration roadmap: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- CorePlan done criteria: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- FlowBox tags schema/coverage: `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`
- Return-in-loop minimal (current approach): `docs/development/current/main/design/return-in-loop-minimal-ssot.md`
- Selfhost policy: prefer compiler strengthening for stdlib/common helpers:  
  `docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md`
- Selfhost tooling subset policy (loopless):  
  `docs/development/current/main/design/selfhost-tools-loopless-subset-ssot.md`

## The design tension

We believe CorePlan’s purpose is to:
- Represent control-flow as composable “FlowBoxes” (ports + ExitMap + join payload), and
- Allow loop internal control structures (break/continue/return) via composition, not via “pattern name” branching.

But “unknown loops” (generic scanning loops with mixed exits, complex condition/update shapes) currently freeze.

Question: Should we make CorePlan more general to accept unknown loops, or further decompose CorePlan into smaller
combinators so unknown loops can be expressed as compositions of known boxes?

## Constraints (hard)

- No by-name/hardcoded dispatch based on function names.
- No silent fallback; strict/dev must freeze early with stable tags/codes; release behavior/logs should remain stable.
- Prefer SSOT boundaries; avoid “emit/merge re-parses CFG/AST” backdoors.
- Keep CorePlan vocabulary minimal and locally verifiable.

## Options under consideration

### Option A: Keep CorePlan subset-only (recommended for tooling)

- Tooling (`tools/*`) stays loopless (recursion-only subset), avoiding “generic loops”.
- CorePlan remains a structured subset for stdlib/app loops only.

Pros: minimal CorePlan, avoids turning CorePlan into a generic CFG language.  
Cons: tooling becomes constrained; may feel “language is weaker” even though only tooling is restricted.

### Option B: Add “general loop” support to CorePlan

Introduce a CorePlan node (or CoreLoopBody DSL) that can represent a general `while/loop` with multiple exits and
arbitrary condition/update, then lower it to MIR.

Pros: fewer freezes; tooling can use natural loops.  
Cons: high risk of CorePlan becoming “a second compiler/CFG language”; verification surface explodes.

### Option C: Decompose CorePlan into smaller composable primitives (“FlowBox combinators”)

Keep Loop skeletons + features, but make “unknown loops” representable by composing smaller nodes:
- Loop skeleton(s): `Loop`, `If2`, `BranchN`, `Seq`, `LeafEffects`
- Feature attachments: ExitMap, cleanup wrappers, value-join payload mapping
- Minimal intra-loop control effects: `ExitIfReturn` + (maybe) `ExitIfBreak/ExitIfContinue`

Pros: stays compositional; unknown shapes may be absorbed by normalizing to these primitives; local verification.  
Cons: requires clear SSOT definitions for ports/payload, and careful staging of normalization.

## Questions for ChatGPT Pro

1) What is the cleanest long-term approach among A/B/C (or hybrid), given the constraints?

2) If C is best, what is the minimal “primitive set” you would standardize as SSOT?
   - Which should be CorePlan vocabulary vs “feature facts” vs “composer combinators”?

3) For “generic scanning loops” (string scanning, arity counting, parentheses scanning), what is the preferred
   representation?
   - Should we standardize them as ScanWithInit/SplitScan expansions, or treat them as a separate skeleton?

4) Where should “unknown loop” live when it is still unsupported?
   - Ok(None) vs Freeze (strict/dev), and what should the Freeze taxonomy look like?

5) How to avoid “pattern explosion” (Pattern1–N style) while keeping the system expressive?

6) For selfhost tooling specifically, is “loopless subset” a good boundary, or should tooling be allowed to use a
   restricted form of loops?

## Desired output (from Pro)

- A recommended architecture (SSOT boundaries) that stays small but expressive.
- A concrete next-step plan (1–2 phases) to move from current state to that architecture without destabilizing release.

