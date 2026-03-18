---
Status: Draft
Scope: external consultation prompt for `BoxBase::new` / `StringViewBox::new`
Related:
- docs/development/current/main/design/box-identity-view-allocation-design-note.md
- src/box_trait.rs
- crates/nyash_kernel/src/exports/string_view.rs
- crates/nyash_kernel/src/tests.rs
---

# External Consultation Question

## Context

I am optimizing a Rust runtime for a language implementation.

We have a base object identity primitive:

- `BoxBase::new()` assigns a fresh `box_id` from a global `AtomicU64`
- every box instance exposes that id through `box_id()`
- GC/finalization and some identity-sensitive paths depend on ids being unique for every live instance

Current relevant code structure:

- `src/box_trait.rs`
  - `next_box_id() -> u64`
  - `BoxBase::new()`
- `crates/nyash_kernel/src/exports/string_view.rs`
  - `StringViewBox { base_handle, base_obj, start, end, base: BoxBase }`
  - every `StringViewBox::new(...)` calls `BoxBase::new()`
  - `StringViewBox` is a substring metadata box, not just a raw borrowed slice
  - `clone_box()` / `share_box()` materialize to `StringBox`
- there is one explicit special case:
  - a `BorrowedHandleBox` alias wrapper may reuse the source handle as a stable id
  - but this is a narrow alias-wrapper contract, not a general rule

## Current Perf Situation

Microbenchmark hotspot order is currently:

1. `substring_hii`
2. `Registry::alloc`
3. `BoxBase::new`

I already optimized:

- substring dispatch path
- handle registry allocation fast path

Now `BoxBase::new` remains visible, but I do not want to break correctness just to reduce its cost.

## Constraints

1. I cannot allow two live boxes to share the same `box_id`
2. I cannot break GC/finalization bookkeeping keyed by `box_id`
3. I cannot silently change `StringViewBox` from “distinct box” to “mere alias” without an explicit contract change
4. I prefer upstream reductions in `StringViewBox::new()` call count over changing generic identity semantics

## Existing View Contract

`StringViewBox` currently means:

- read-only substring metadata may remain as a view
- clone/materialize boundaries convert to owned string boxes
- short slice / nested short slice / mid slice behavior is pinned by tests

So changing when we materialize vs when we create a view is possible, but that is a contract decision.

## What I Want Help With

Please evaluate these design directions specifically for this kind of runtime:

1. Keep fresh per-instance ids, and only reduce `StringViewBox::new()` call count upstream
2. Lazy id generation:
   - only assign a fresh id on first observable `box_id()` use
   - preserve uniqueness for every live instance
3. Alias-style id reuse for transient views:
   - under what exact conditions, if any, could a transient substring view safely reuse an existing id?
4. Object pooling for view objects:
   - is there any realistic safe pattern if `box_id` uniqueness must remain observable?

## The Main Question

Given these constraints, what is the cleanest optimization strategy?

I want an answer that distinguishes:

- clearly safe
- safe only with an explicit contract change
- unsafe / likely wrong

If you suggest lazy ids or alias-id reuse, please explain exactly how to preserve:

- uniqueness of live objects
- GC/finalization correctness
- semantics of distinct transient view objects

## Repo-Specific Note

I am not looking for generic “make allocations faster” advice.
I want a design recommendation for a system where object identity is correctness-bearing, but substring-view creation is performance-sensitive.

