---
Status: active — bounded passive Home relation vocabulary
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-abi-d0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
---

# OWN-HOME-RELATION0-S0

## Objective

Add the passive, resolver-independent vocabulary needed by the later Home ABI
issuer. This row creates no source classifier, grammar, Home Flow, target,
Recipe, Builder/MIR, runtime, or production caller.

## Product boundary

The module owns only branded relation data and typed rejection reasons:

```text
HomeRelationBrandV1
HomeRootRefV1
HomeDestinationV1
HomeDemandV1
HomeResultRelationV1
HomeRelationRejectV1
```

The capability values are vocabulary, not classification:

```text
HomeDemandV1 = Handle | Home | SharedHome | Trivial
HomeResultRelationV1 =
  Unit | Trivial | HomeToCaller | FromReceiver |
  FromParameter(index) | SharedHomeToCaller
```

`HomeRootRefV1` and `HomeDestinationV1` carry one fresh relation brand and an
opaque source ordinal. They do not carry `ValueId`, `BasicBlockId`, runtime
handles, reference counts, or physical layout. No public constructor may
forge a branded relation; tests use a module-local issuer only.

## Acceptance

1. One focused resolver-semantic module remains below the 760-line split
   trigger and imports no Builder/MIR/Recipe/runtime/backend module.
2. Brand, root, and destination relations reject foreign brands and preserve
   exact source ordinals.
3. Demand/result enums are exhaustive and contain no implicit `Unknown` to
   `Trivial` conversion.
4. Rejection reasons are typed and stable for foreign brand and duplicate
   root/destination source slots; unsupported or missing capability cases are
   intentionally deferred to the later Home ABI issuer.
5. The module has no production callers; focused positive/negative tests and
   this task/reference README update land in the same commit.

## Explicit non-claims

```text
No Home ABI issuer
No type/capability classifier
No receiver/parameter/result co-seal
No Home Flow or Ownership SSA
No take/share/release grammar
No physical Unique/Shared representation
No target/CallSlot/Recipe/Builder/MIR/runtime path
```

## Verification

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust home_relation
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

After this row closes, `OWN-HOME-ABI0-S0` may open for the exact
same-brand declaration/Home classifier co-seal defined by the D0 card.
