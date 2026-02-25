# DomainPlan / CorePlan Design Consult (for ChatGPT Pro)

Status: Draft (Phase 29ak, consult-only)  
Goal: Ask for design feedback without requiring source code reading.

## 0. What is Hakorune (1-paragraph summary)

Hakorune is a compiler pipeline that lowers `.hako` (Nyash) source into MIR and executes it via VM/LLVM backends. A large, tricky part is the **JoinIR line**: it recognizes “loop/if shaped” regions and rewrites/lower them through a structured pipeline (Plan → Frag → MIR merge) with strict SSOT boundaries so we can keep behavior stable while refactoring aggressively.

## 1. The current JoinIR/Plan/Frag architecture (SSOT view)

We are migrating from “pattern-name routing at the entry” to a **single pipeline**:

```
AST
  → Facts (observations + derived facts)
  → Normalize (canonicalize facts; pure transform)
  → Planner (candidate-set; 0/1/2+ → None/Some/Freeze)
  → DomainPlan (pattern-specific recipe vocabulary)
  → Normalizer (DomainPlan → CorePlan; SSOT conversion)
  → CorePlan (fixed vocabulary structure nodes only)
  → Lowerer/Emit (CorePlan → Frag; generation only)
  → Merge (Frag + Boundary → MIR blocks; contract checks)
```

Key SSOT docs:
- Plan/Frag SSOT registry: `docs/development/current/main/design/planfrag-ssot-registry.md`
- Freeze taxonomy: `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
- JoinIR Plan/Frag responsibilities: `docs/development/current/main/design/joinir-plan-frag-ssot.md`
- EdgeCFG/Frag overview: `docs/development/current/main/design/edgecfg-fragments.md`

## 2. Why this refactor exists (design constraints)

We want to remove “pattern-name entry dispatch” and avoid “by-name hacks” while keeping behavior stable:

- Default behavior must not change (release builds are the baseline).
- Fail-Fast is allowed only on clear contract violations; silent fallback is forbidden.
- No “hardcode to pass smoke” (e.g., `if pattern == "PatternX" { ... }` in arbitrary places).
- Avoid new permanent env vars; use existing strict/dev gates if needed.
- Observability should be stable, tagged, and SSOT’d (strict/dev only).

## 3. What `DomainPlan` and `CorePlan` are today

### 3.1 DomainPlan (pattern-specific recipe vocabulary)

`DomainPlan` is the “recipe” layer that still knows pattern semantics (Pattern1–9). Examples:

- `Pattern1SimpleWhile`
- `Pattern2Break`
- `Pattern3IfPhi`
- `Pattern4Continue`
- `Pattern5InfiniteEarlyExit` (loop(true) + early exit)
- `ScanWithInit` / `SplitScan` / `BoolPredicateScan`
- `Pattern9AccumConstLoop`

DomainPlan is consumed by a SSOT `PlanNormalizer` that emits `CorePlan`.

### 3.2 CorePlan (fixed vocabulary)

`CorePlan` is a stable, low-level “structure node” vocabulary meant to be composed, verified, and emitted without re-parsing AST or re-analyzing CFG. The intent is that **composition lives here** long-term.

## 4. Where we are today (implementation progress, without code details)

We have been moving pattern extraction SSOT into the plan layer and shifting routing to planner-first:

- Facts/Planner-first are implemented conservatively for Pattern1–9 (subsets) and adopted only when the planner output variant matches the expected rule.
- `single_planner` still exists as the order SSOT, but it has been simplified:
  - rule order and pattern names are SSOT’d (`rule_order.rs`)
  - special-case guards/filters (Pattern1 guard, Pattern8 static-box filter) were moved into planner/facts and removed from `single_planner`
  - `PlannerContext` exists to pass `pattern_kind` / `in_static_box` / `debug` to planner-side gating

JoinIR regression gate (SSOT):
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

Legacy integration packs that are intentionally SKIP’d (SSOT) exist for cases whose environment assumptions no longer match (e.g. LoopBuilder removal / plugins disabled).

## 5. The design question

We believe the clean end-state is “compose with orthogonal parts” rather than “add more overlapping complete patterns”.

The uncertainty: **what should remain in DomainPlan**, and **what should be expressed as CorePlan composition**?

## 6. Specific questions for ChatGPT Pro

### Q1) Should DomainPlan remain long-term, or shrink into a migration-only layer?

- A) Keep DomainPlan as a permanent public-ish vocabulary (pattern semantics live here forever)
- B) Treat DomainPlan as a transient recipe layer, and over time represent most structures as CorePlan composition

What end-state is the cleanest and most maintainable?

### Q2) What belongs in DomainPlan vs CorePlan?

We suspect:
- DomainPlan should keep “semantic-heavy” plans (scan/split/predicate) because they encode algorithmic intent.
- CorePlan should encode generic control structure (loop/if/join/exit/phi-ish edges) and be composable.

Is that a good boundary? If not, what boundary is better?

### Q3) How to make “non-overlapping” rules without pattern explosion?

We currently use CandidateSet (0/1/2+ → None/Some/Freeze) and a strict Freeze taxonomy.

What is the best practice to make rules “not overlap” in a compose-first system?
- Make Facts more lossless?
- Make Normalize stronger (canonicalize more forms)?
- Prefer “component inference” (emit parts) over “whole-plan inference”?

### Q4) How to keep observability stable while migrating?

We keep strict/dev-only tagged logs and forbid silent fallbacks.

What is the cleanest “observability contract” across Facts/Planner/Normalizer/Emit so that:
- release logs don’t change
- strict/dev can show stable tags
- diagnostics remain local (no re-analysis in emit)

### Q5) What minimal “final-form” invariants should be SSOT’d next?

If we pick only 1–2 SSOT docs/invariants to harden next (without adding major features), what gives the most leverage?
- “post-phi representation” invariants?
- effect classification (pure/control/rc/observability) invariants?
- unwind/ExitKind forward design?

## 7. Constraints recap (so advice is actionable)

- Behavior-preserving refactor is the priority.
- No hardcoded by-name hacks.
- Fail-Fast only on clear contract violations (strict/dev gates OK).
- Avoid new env vars; prefer existing strict/dev mechanism.
- SSOT-first: boundary docs + verification commands are first-class.

## 8. What kind of answer is most useful

Please answer with:
- A recommended “clean end-state” (DomainPlan vs CorePlan) and why.
- A short checklist of invariants to SSOT next.
- A migration strategy that keeps behavior stable (how to gradually shrink DomainPlan or keep it).

