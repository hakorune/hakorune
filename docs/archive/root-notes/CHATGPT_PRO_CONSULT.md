# ChatGPT Pro Consult — CorePlan “Lego” next primitives (Phase 29bq)

## Context (short)

We’re evolving Hakorune’s JoinIR/CorePlan planner to be **compositional**: `Skeleton + FeatureSet` with strict SSOT and fail-fast.  
Large “boxes/patterns” are being decomposed into:

- `facts/` (observation only)
- `canon/` (analysis-only view; conservative; **no AST rewrite**)
- `skeletons/` (block/layout allocation only)
- `features/` (delta apply only; no re-parse; no PHI handcrafting outside shared helpers)
- `pipelines` (fixed apply-order; SSOT; normalizers call pipeline only)

Decomposition for `generic_loop`, `scan/split`, `pattern5`, `loop_true`, `loop_cond` is done and gates are green.

Current active track is **Phase 29bq selfhost canary**: we unblock Stage-B/selfhost freezes by:

1) run canary (opt-in)  
2) extract failing function “StepTree” into a **minimal fixture**  
3) add it to fast gate list  
4) extend compiler expressivity minimally (1 stmt-type or 1 feature) under `strict/dev + planner_required` only  
5) keep regression gates green

## Hard constraints

- **No AST rewrite**: no algebraic transforms, no code motion that changes eval order/overflow/side effects. Only analysis-only views are allowed.
- **Release default unchanged**: any new expressivity is behind `strict/dev + HAKO_JOINIR_PLANNER_REQUIRED=1`.
- **No silent fallback**: strict/dev should freeze/tag, not “pretend OK”.
- Prefer structural fixes: SSOT docs + small primitive/features over ad-hoc special-cases.

## Goal of this consult

We want to avoid “exception pileup” while unblocking selfhost.  
Please advise what **minimal CorePlan primitives / skeletons / features** should be added next to cover the remaining parser-ish loops cleanly, without pattern explosion.

## What we already consider “special-rule triggers”

If a case falls into these, we *don’t* want to add ad-hoc acceptance; we want a proper SSOT + dedicated design:

- irreducible / multi-entry loop (skeleton non-unique)
- unwind/finally/cleanup boundary required (ExitMap+Cleanup semantics)
- coroutine/yield (function control doesn’t close)
- any need to violate “no rewrite” (eval order / overflow / side effects)

## Questions

### Q1) Next “Lego primitives”: what is the best minimal set?

Candidates we’re considering (pick order + rationale):

- `Seq/Block` as an explicit CorePlan node (n-ary), to eliminate implicit “effect list” handling.
- `CleanupWrap` + **cleanup region boundary** (SSOT), to keep nested exits semantics correct.
- `StepMode` generalization (`ExtractToStepBB` vs `InlineInBody`) with a verifier contract (strict/dev only).
- `Break/Continue(depth)` (depth=1 default), to support nested loop scoping without by-name labels.
- `BranchNSkeleton` (match/switch) vs forcing `If2` chains.

Which of these are truly necessary for selfhost progress, and what order minimizes future rework?

### Q2) For parser-ish loops, what is the “right abstraction boundary”?

We currently unblock with minimal fixtures and expand acceptance in:

- `loop_cond_break_continue` (loop(cond) + guard break + multi continue + conditional update/join)
- `loop_true_break_continue` (loop(true) with nested loop(true) depth<=1)
- `generic_loop_v1` (loop body as CorePlan tree; If/Exit; carriers)

Should remaining cases be solved primarily by:

- strengthening `canon` views (CondCanon/UpdateCanon) to improve skeleton uniqueness, or
- adding a new skeleton (e.g., “WhileTrue with internal guard breaks”), or
- adding features (per-edge continue args, value joins, etc.)?

### Q3) Verifier: what “fail-fast invariants” should we enforce for new primitives?

We want local, mechanical checks (no global analysis).  
Examples:

- `InlineInBody` initially forbids `Continue(depth=1)` and requires `step_bb` empty
- join/phi inputs must be complete for every predecessor edge (no implicit defaults)
- continue_target must match the skeleton mode (header vs step)

Please propose a minimal set of invariants for each recommended primitive.

### Q4) Process: when to “promote” canary exceptions into a decomposition phase?

We currently use a promotion trigger like: “same kind of extension needed twice” ⇒ pipeline/feature refactor.

What promotion triggers do you recommend to keep the design clean while moving fast?

## Attached source bundle

See `consult_sources_phase29bq.zip` in repo root. It contains:

- SSOT/design docs for skeleton/feature model + roadmap + 29bq phase
- Plan registry (implementation SSOT)
- Key `features/`, `skeletons/`, `canon/`, loop_cond/loop_true implementations
- fast gate list + representative fixtures

