---
Status: Landed
Date: 2026-04-23
Scope: Fix Rust VM unified method dispatch when a core-box receiver is duplicated by alias value.
Related:
  - docs/development/current/main/phases/phase-292x/292x-112-pure-compile-minimal-ret-branch-deletion-card.md
  - src/backend/mir_interpreter/handlers/calls/method.rs
  - src/backend/mir_interpreter/handlers/calls/method/tests.rs
---

# 292x-113: MapBox Duplicate Receiver Unified Dispatch

## Problem

The ret/branch deletion probe exposed a smaller precondition bug in the Rust VM
method-call path:

```hako
local m = new MapBox()
m.set("a", "b")
print("" + m.get("a"))
```

Before this card, the Rust VM could pass an alias of the receiver as the first
surface argument. `MapBox.get("a")` then looked up the receiver object as the
key and returned:

```text
[map/missing] Key not found: MapBox(size=1)
```

This also made `BackendRecipeBox` profile validation fail with missing
`route_profile`, because `profile.set(...)` mutated the map while
`profile.get("route_profile")` received the duplicated receiver as its key.

## Fix

- Generalized method-argument normalization from plugin-only duplicate receiver
  stripping to core BoxRef pointer stripping.
- Kept primitive values conservative: equal string/integer values are not
  treated as duplicate receivers unless they are the same ValueId at the caller
  boundary.
- Added a Rust unit test where a `MapBox` receiver alias is passed before the
  real key/value arguments.

## Verification

```bash
cargo fmt --check
cargo test -q method_callee_mapbox_set_get_strips_duplicate_receiver_arg --lib
cargo test -q method_callee_stringbox_length_strips_duplicate_receiver_arg --lib
cargo build --release --bin hakorune
```

Manual probe:

```text
m.set("a", "b"); m.get("a") -> b
```

## Residual

This fixes the Rust VM duplicate-receiver bug, but it does not make
`pure_compile_minimal_paths` path #1/#2 delete-ready yet. After this fix, the
daily Hako LL and llvmlite monitor canaries move past `route_profile` missing
and fail later with stack overflow. That triage is tracked by `292x-114`.
