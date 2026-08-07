# Loop Physical Prepare Static-Call Fixture Design D0

Status: `Decision: accepted after worker source-authority audit; implementation may proceed in bounded S0/S1/P0 cells`
Date: 2026-08-07
Parent: `LOOP-PHYSICAL-PREPARE-P0`
Design authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

The current callable fixture is intentionally a `MethodCall` whose resolver
ledger has no direct callable target and whose return declaration is
unannotated. It is therefore a correct typed negative, but cannot provide the
genuine positive `PreparedCallableLoopPhysicalizationV1` witness required by
P0. This row adds one exact source-backed static-call fixture/profile without
opening physical lowering, a selector, or a production caller.

This is a bounded source-authority correction, not a second callable
architecture and not a new Recipe family.

## Decision

Use a separate `cfg(test)` fixture with the canonical catalog shape that the
current callable index actually owns:

```text
int_to_str(n: i64): i64
  local value = to_i64(n)
  loop (...) { ... }
  return value

to_i64(n: i64): i64
  return n
```

The exact names are fixture-local; the catalog namespace is `FreeStatic`. Do
not claim a `StringHelpers.to_i64` or Box-qualified namespace: that would
require a separate catalog/namespace design. The existing MethodCall fixture
remains a negative boundary and is not rewritten or target-injected.

The source observer records an explicit neutral call kind:

```text
SourceCallKindV1::Method(receiver-shape)
SourceCallKindV1::FreeStatic
```

`Other` is never reused as evidence for a free static call. The direct target
is taken only from the resolver-issued callable ledger. No name lookup or AST
re-resolution is added downstream.

The shape product may use the following equivalent representation, but must
keep the kind explicit rather than overloading a receiver enum:

```text
SourceCallBoundaryShapeV1::Method { receiver, argument_count }
SourceCallBoundaryShapeV1::FreeStatic { argument_count }
```

## Required boundary corrections

### 1. Keep source-shape files below 800 lines

`callable_single_loop_syntax_facts.rs` and
`callable_single_loop_source_map.rs` are already near the source limit. First
extract their embedded test modules into test-only sibling files, then place
the neutral call-shape vocabulary in a small shared
`callable_single_loop_source_shapes.rs` module. The production/test-only
observer files remain thin and each stays below 800 lines.

No broad rename or new root facade is allowed in this row.

### 2. Target owner relation

A direct static callee normally has a different `FunctionOwnerIdV1` from its
caller. The source map must not reject this as `ForeignOwner`. The valid
relation is:

```text
callee owner != caller owner is allowed
callee owner.compilation_brand == caller owner.compilation_brand
```

The resolver's direct-call verifier and the callable index/header remain the
sole authority. A different compilation brand is a typed foreign-compilation
reject. No raw owner may be minted for the fixture.

### 3. ABI must be derived, not injected

The prepare entry must not accept a bare external `ExactTrivialReturnAbiV1`
as proof. It derives the caller/callee result ABI through the existing exact
header/result-declaration classifier and the existing Completion declaration,
then seals the value-return relation.
An unsupported or missing declaration is a typed `NoSafeSlice`.

The positive fixture uses explicit `: i64` declarations on both callables.
The existing unannotated MethodCall fixture remains a negative.

The prepared prelude capability records the exact call kind, target, arity,
and derived result ABI. Its expected input is a call-kind contract, not a
hard-coded receiver enum.

Source-level catalog rejects (for example wrong static arity/name or a
non-exact header) remain resolver/index evidence. They must not be synthesized
as malformed headers merely to exercise a prepare-level reject. Prepare-level
tests cover only sealed products that actually reach that boundary.

## Ownership and non-goals

```text
resolver ledger / callable index / header:
  target, owner, compilation brand, and source result authority

source shape module:
  Method vs FreeStatic syntax shape only

source map:
  co-sealed source site and resolver target relation

physical prepare:
  one pre-effect compatibility relation and move-only Prepared product
```

Do not add:

```text
production resolver behavior
universal CallablePlan
new namespace or Box method catalog
AST rewrite or name-based target lookup
physical IDs / CFG / PHI / Builder effects
selector, retry, fallback, publication, or backend behavior
```

## Implementation order

```text
S0  extract syntax/source-map tests and add SourceCallKindV1 vocabulary
S1  observe exact FunctionCall FreeStatic shape and keep MethodCall negative
S1  permit same-compilation different-owner direct target; reject foreign brand
P0  derive caller/callee ABI and call-kind from exact header/Completion
    relation and issue positive
    Prepared witness plus owner/brand/arity/result/site/value negatives
```

Each cell is behavior-neutral and `cfg(test)`/caller-zero. The existing
`VerifiedLoopPhysicalDemandV1`, Tail, Completion, and common Recipe algebra
are reused; no parallel physicalizer or Recipe is introduced.

## Acceptance

The row is complete when:

- the positive fixture is resolved by the existing callable catalog and direct
  call ledger, with no target injection;
- the caller/callee different-owner same-brand relation is tested, and a
  foreign compilation brand is rejected before any physical effect;
- `FreeStatic` and `Method` call kinds are explicit and `Other` is not used as
  a static proof;
- caller and callee result ABI are derived from exact declarations, with
  missing/unsupported/mismatched cases rejected;
- the positive Prepared product and the listed negative boundaries are typed,
  move-only, and remain caller-zero;
- every touched source/check file remains below 800 lines;
- source-level catalog rejection and prepare-level `NoSafeSlice` are kept as
  distinct evidence classes;
- README, exact `docs/reference/**` contracts, active workstream,
  `CURRENT_STATE.toml`, and any guard index entry are updated in the same
  implementation commit.

The current MethodCall negative and the P0 no-physicalizer boundary remain in
the test matrix after this row closes.

## S0 implementation receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-S0` is closed as the bounded observer cell. The new
test-only fixture uses the existing top-level callable catalog and resolver
with exact `int_to_str(n: i64): i64` and `to_i64(n: i64): i64` declarations.
The `FunctionCall` initializer is observed as explicit `FreeStatic` with
arity one, and the direct target is verified from the resolver ledger with a
different owner in the same compilation brand. The existing
`helper.to_i64(n)` MethodCall remains a `Method` negative with no direct target.

This receipt does not close the full static-call design: source-map acceptance
for the same-brand different-owner relation is the next
`CALLABLE-STATIC-PREFIX-MAP-S1` task, followed later by declaration-derived
ABI and Prepared positive evidence. No physical or production authority was
opened.

## MAP-S1 implementation receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-MAP-S1` is closed as the bounded source-map cell. The
map retains the resolver-issued `to_i64` target when caller and callee owners
differ but their compilation brand matches. Independently sealed catalogs
prove that a foreign compilation brand is rejected as typed `ForeignOwner`
before a map product is issued. The existing MethodCall remains a typed
negative.

The next cell is `CALLABLE-STATIC-PREFIX-P0`, limited to declaration-derived
parameter/result ABI and one positive Prepared relation. Recipe, physicalizer,
Builder, selector, retry/fallback, publication, and production claims remain
closed.
