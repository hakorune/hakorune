# 166x-90: semantic refresh and generic relation SSOT

Status: SSOT
Date: 2026-04-11
Scope: cut the next MIR-structure cleanup after `phase-165x` so semantic metadata refresh and generic relation ownership stop fragmenting across domain passes.

## Goal

- keep `canonical MIR -> generic relation facts -> domain facts -> candidates -> transform/backend` as the structural stack
- stop helper-name, alias-root, and PHI-base recovery from being re-owned by each domain pass
- keep generic def-use, semantic operand-role, storage class, and fact/candidate layering separate

## Diagnosis

The current issue is not "too much analysis" by itself.

The structural risk is:

- domain fact builders still recover semantics from helper/runtime names
- `copy root` normalization exists in multiple domains
- PHI base continuity is generic in intent but still requires domain-local normalization closures
- metadata refresh order is scattered across `MirCompiler` and individual transform passes

## Authority

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust implementation under src/mir/**`
4. `runtime / LLVM consumers`

This phase only touches step 2 and step 3.

## Keep These Splits

- `used_values()` vs semantic operand-role classification
- `storage_class` vs boundary/lifecycle policy
- `fact` vs `candidate`

These answer different questions and are not the current duplication problem.

## Fix These Owners

### 1. Semantic Refresh Owner

Add one MIR-side refresh entry point and move refresh ordering there.

Target order:

1. generic value typing / storage facts
2. generic operand-role / relation facts
3. domain facts
4. domain candidates
5. downstream selections/layouts

The exact internal grouping may evolve, but the owner must be single.

### 2. Generic Value Origin Owner

`copy root` / alias-root normalization must not stay duplicated across:

- string corridor
- sum placement
- escape analysis
- future user-box/local-body genericization

The owner should live in a generic MIR seam, not a domain recognizer.

### 3. Generic PHI Relation Owner

PHI carry/base interpretation is generic.

Domain layers may ask:

- same base?
- mixed?
- unknown?
- plan window preserved?

But they should not own the traversal rules themselves.

### 4. Compat Semantic Recovery Quarantine

Legacy/helper/runtime-export name recovery may still exist during migration, but it must be isolated as compat canonicalization instead of remaining in domain fact builders.

Canonical-domain passes should trend toward reading canonical ops first.

Current landed cut:

- `src/mir/string_corridor_compat.rs` owns string-lane helper/runtime-name recovery
- `src/mir/string_corridor.rs` stays canonical-first and falls back to the compat seam only after canonical-op detection

## Deferred

The following stays out of the first structural cut:

- broad generic `boundary_fact` / `lifecycle_outcome` extraction
- new optimization transforms
- new acceptance shapes
- runtime/helper or LLVM policy changes

## Ordered Task Cut

1. `166xA`: docs lock
2. `166xB`: semantic refresh owner
3. `166xC`: generic `value_origin` owner
4. `166xD`: generic `phi_relation` owner
5. `166xE`: compat semantic recovery quarantine
6. `166xF`: boundary/lifecycle extraction decision

## Landed Decision

`166xF` is now closed with an explicit defer:

- keep `StringOutcomeFact` / `StringPlacementFact` local to the string lane
- keep `EscapeBarrier` / `SumObjectizationBarrier` as barrier-cause vocabularies
- do not create one mixed generic seam until another real lifecycle/outcome consumer exists
- if generic extraction reopens later, split it by question instead of collapsing lifecycle and barrier causes together

## Acceptance

- phase docs point to one ordered task stack
- root/current/workstream docs point to `phase-166x` as the landed structural cleanup follow-on after `phase-165x`
- `boundary_fact` extraction is explicitly documented as deferred until after refresh/relation ownership stabilizes and another lifecycle consumer exists
