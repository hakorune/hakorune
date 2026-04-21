---
Status: Active
Date: 2026-04-22
Scope: `ArrayBox` surface canonicalization phase の design brief。surface contract / execution dispatch / exposure state の 3 層を docs-first で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-290x/290x-91-arraybox-surface-task-board.md
  - docs/development/current/main/phases/phase-290x/290x-92-arraybox-surface-inventory-ledger.md
  - apps/kilo_nyash/enhanced_kilo_editor.hako
---

# Phase 290x ArrayBox Surface Canonicalization Design Brief

## Purpose

`ArrayBox` は app lane で普通に使う surface なのに、method truth が 1 か所に集まっていない。

今回の目的は、`insert/remove/length-size` のような surface drift を
「また同じ修正を repo の別の場所に書き足す」形で増やさないこと。

## One Sentence

`ArrayBox` の truth を 3 層に分け、surface の正本を 1 か所に寄せる。

```text
surface contract
  -> what methods exist, canonical name, alias, arity, exposure state

execution dispatch
  -> how a canonical method reaches runtime behavior

exposure state
  -> whether the method is implemented, surfaced, documented, and smoke-pinned
```

## Three-Layer Contract

| Layer | Owns | Must not own |
| --- | --- | --- |
| surface contract | canonical method name, aliases, arity, stable user-facing contract | runtime mechanics, helper-local routing |
| execution dispatch | canonical method id -> runtime behavior entry | semantic ownership, docs truth |
| exposure state | runtime/std/docs/smoke visibility state | dispatch semantics |

## Proposed Authoring Point

The intended code-side authoring point is:

```rust
src/boxes/array/surface_catalog.rs
```

Suggested shape:

```rust
pub struct ArrayMethodSpec {
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub arity: u8,
    pub dispatch: ArrayDispatchKind,
    pub effect: ArrayEffectKind,
    pub exposure: ExposureFlags,
}
```

Phase-290x does **not** require this file to land in the docs slice.
It only fixes that this is the intended SSOT seam for the implementation slice.

## Canonicalization Decisions Locked Now

### 1. `length()` vs `size()`

- canonical user-facing name: `length()`
- compatibility alias: `size()`

Reason:

- current std/app usage is already `length()`-leaning
- `length()` reads naturally next to current string/container surfaces
- alias support stays acceptable, but the docs should stop presenting both as equal primaries

### 2. `apps/std/array.hako`

- role: user-facing sugar layer
- not the semantic owner of `ArrayBox`

That means:

- std wrappers should follow the catalog
- std wrappers should not redefine the truth of method names/effects/exposure

### 3. `phase-137x`

- remains observe-only
- does not own ArrayBox surface truth
- may reopen only if app implementation is truly blocked

## In Scope

1. Create a phase-local docs owner for ArrayBox surface cleanup
2. Record the current touchpoints for:
   - surface contract
   - execution dispatch
   - exposure state
3. Fix current pointers so restart/current docs no longer claim `insert()` is missing
4. Lock the phase direction:
   - `surface_catalog.rs`
   - `ArrayMethodId`
   - `ArrayBox::invoke_surface(...)`
5. Make `length()` canonical and `size()` alias in docs

## Out of Scope

- broad runtime refactor in this docs slice
- perf reopen for phase-137x
- new planner / legality ownership
- generic API redesign outside `ArrayBox`
- two-arg `lastIndexOf` implementation
- static-box receiver diagnostics redesign

## First Implementation Seam

The first code seam is now:

```text
catalog
  -> ArrayMethodId
  -> ArrayBox::invoke_surface(id, args)
  -> thin consumers:
       runtime/type_registry.rs
       boxes_array.rs
       calls/method/dispatch.rs
       method_resolution.rs
       effects_analyzer.rs
```

The goal is not “everything becomes slot dispatch”.
The goal is “one canonical surface, one canonical invoke seam”.

2026-04-22 status:

- `src/boxes/array/surface_catalog.rs` owns the first stable rows
- `ArrayBox::invoke_surface(...)` owns the stable runtime dispatch seam
- `type_registry`, `method_resolution`, `effects_analyzer`, and MIR interpreter ArrayBox dispatch read the catalog
- extended methods (`clear/contains/indexOf/join/sort/reverse`) remain deferred

## Acceptance For The Docs Slice

- a named phase front exists under `main/phases/phase-290x/`
- restart/current pointers lead to this phase
- `insert()` is no longer described as missing runtime surface
- the 3-layer split is explicit and discoverable
- the first implementation seam is named, but not over-implemented in docs
