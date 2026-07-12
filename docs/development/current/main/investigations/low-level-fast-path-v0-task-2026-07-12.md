# Low-Level Fast Path V0 — Zero-New-Syntax Parked Task

Status: Parked workstream; not the active lane.
Date: 2026-07-12
Decision basis: keep source syntax small and move complexity into contracts,
facts, plans, verifiers, backend preflight, and backend-private artifacts.

## Goal

Build a verified low-level fast path without adding another source language.

```text
ordinary Hakorune source
  + existing uses metadata
  + existing @rune Contract(no_alloc / no_safepoint)
  + existing fastmem ContractName { ... }
  -> MIR facts and plans
  -> verifier
  -> backend capability preflight
  -> selected backend-private implementation
```

The V0 source keyword delta is zero.

## Explicitly excluded source surface

```text
unsafe {}
direct {}
layout {}
mir {}
asm {}
model {}
substrate fn
RawPtr<T>
pointer dereference/index/arithmetic
& / * / -> pointer operators
generic Span<T>
generic DirectArray<T>
source-selected atomic memory order
repr(C) / sizeof / alignof / offsetof
```

If machine-recipe notation is eventually required, it belongs in a separate
versioned backend artifact such as `TargetRecipeV0`, not ordinary `.hako`.

## Current inventory

### Live surfaces

```text
fastmem ContractName:
  parser / AST / MIR region / MemOp / verifier / MIR JSON / LLVM consumer live
  ContractName remains weakly bound and needs one registry owner

@rune Contract(no_alloc):
  EffectPlan + MIR verifier live
  currently rejects direct Alloc effects only

@rune Contract(no_safepoint):
  EffectPlan + MIR verifier live
  currently rejects explicit Safepoint only

DirectArrayI64:
  source-visible candidate, access plan, exact LLVM routes, kernel storage pilot

FastMem proof substrate:
  bounds, overflow, layout, alignment, provenance/no-escape checks live

specialized atomic:
  AtomicRemoteHeadPush / AtomicRemoteHeadDrain live in FastMem + LLVM
```

### Partial surfaces

```text
uses capability:
  syntax and ordered AST/MIR metadata accepted
  CapabilityPlan remains verified=false
  unknown names may currently be ignored during mapping

SpanI64 / SpanMutI64:
  SpanBorrowFact and SpanAccessPlan exist
  no ordinary-source producer or end-to-end lowering

intrinsic registry:
  llvm_py-local registry exists
  no mainline cross-backend semantic registry

FastLeafManifest:
  design exists with no-widen-now
  no generated artifact/live ny-llvmc consumer
```

### Missing surfaces

```text
bounded byte-view carrier
NativePtr -> bounded view conversion
FFI-scoped view lifetime and invalidation
general atomic type/API
ordinary Hako model versus selected intrinsic differential gate
end-to-end registry -> fact -> plan -> preflight -> hidden leaf gate
```

## Naming boundary

Do not activate public `Bytes` / `BytesMut` names in this task.

`Bytes` already collides with a language type-alias use and the plugin ABI
byte-payload vocabulary. The first internal carriers are provisionally:

```text
ByteViewV0
ByteViewMutV0
```

Public naming requires a separate language Decision. `NativePtr` remains
opaque and never gains direct source load/store/index/arithmetic operations.

## Authority graph

```text
ordinary Hako model function:
  visible semantic oracle for a selected operation

FastMemContractRegistryV0:
  ContractName -> required effects/capabilities/proof family

MIR Facts:
  observations and proof ingredients

MIR Plan:
  selected execution route truth

MIR verifier:
  plan obligations, bounds, no-escape, effect, and capability validation

backend capability preflight:
  target support and fail-fast before effects

ABI manifest:
  public ABI truth

FastLeafManifest:
  generated backend-private projection only

backend lowering:
  consumes a verified plan; does not re-infer semantics
```

No handwritten second intrinsic/contract registry may duplicate an existing
callable, ABI, FastMem layout, or route truth.

## Task order

### F0 — authority and baseline inventory

- Classify every current FastMem ContractName, accepted MemOp, capability ID,
  EffectPlan requirement, backend consumer, and fail-fast path.
- Fix exact DirectArrayI64 initialized-length/get/set checked and
  proved-unchecked fixtures as the performance/semantics baseline.
- Record current exact-EXE instructions, cycles, allocations, route decisions,
  and whole-workload behavior before changing a plan.
- Correct stale parse-only comments only if the inventory proves the live
  implementation boundary.

### F1 — one ContractName registry owner (BoxShape only)

- Introduce one declarative `FastMemContractRegistryV0` owner or extend the
  existing FastMem contract owner rather than adding a parallel table.
- First row only: one existing real contract such as `PageMapV0`.
- Bind required effect IDs, capability IDs, proof family, and supported backend
  set declaratively.
- Reject unknown ContractName and missing explicit requirements.
- Do not set `CapabilityPlan.verified=true` in this slice.
- Do not add MemOps or backend routes.

Open design point before F1 implementation:

```text
Does fastmem PageMapV0 imply required effects/capabilities,
or must source explicitly declare matching @rune/uses rows?
```

This must be decided once. Parser, planner, and verifier must not each invent
their own answer.

### F2 — proof spine consolidation

- Close one producer/refresh/consumer spine for:

  ```text
  RangeIndexFact
  DirectArrayExtentFact
  RegionStabilityFact
  FastMem no-escape barrier classification
  ```

- Make backend lowering consume only the resulting access plan.
- Keep checked and proved-unchecked routes distinct.
- Prove no fallback/redecision and no Array semantic change.

### F3 — internal Span V0 end-to-end

- Produce `SpanBorrowFact` from an existing typed MIR/fixture boundary.
- Reuse FastMem escape-barrier proof ingredients rather than trusting only a
  caller-provided `no_escape` boolean.
- Project `SpanAccessPlan` into DirectArrayI64 lowering.
- Keep Span internal: no new keyword, generic type, source borrow syntax, or
  public collection conversion.

### F4 — internal bounded byte view

- Add internal `ByteViewV0` / `ByteViewMutV0` carriers with byte length,
  initialized range, mutability, owner/lifetime identity, and proof IDs.
- Construct them only from an accepted capability/provider boundary.
- Reuse FastMem bounds/overflow/layout/alignment/no-escape verification.
- Keep `Bytes`, `BytesMut`, RawPtr, and NativePtr operations unactivated.

### F5 — limited opaque NativePtr/FFI ingress

- Accept only an ABI proof of nonnull plus exact dereferenceable length and
  alignment.
- Convert to an invocation-scoped byte view, never to source pointer access.
- Reject return, field store, capture, provider/plugin re-export, unknown call,
  mutable alias, and use after lifetime end.
- Invalidate every view at the boundary exit.
- Unsupported backends fail before external effects; no VM fallback.

### F6 — one ordinary-Hako model/intrinsic row

- Select one measured operation that ordinary Hakorune can model as an
  abstract state/value transition.
- Add one semantic operation ID and connect:

  ```text
  model function
  -> registry row
  -> EffectPlan / CapabilityPlan
  -> RoutePlan
  -> backend preflight
  -> one backend-private implementation
  ```

- Compare model and selected implementation over the same bounded corpus.
- Do not expose hidden leaf symbols as a third public ABI.
- Generate FastLeafManifest from existing ABI/callable authority; do not
  hand-author it.

### F7 — specialized atomic expansion by evidence only

- Keep existing remote-head push/drain as the reference specialized routes.
- Add at most one fixed-slot i64 atomic operation per BoxCount card.
- Fix memory order in the contract/plan; no user-selected order in V0.
- Require model differential, race/concurrency fixture, backend preflight, and
  unsupported-backend fail-fast.

### F8 — perf keeper and closeout

- Follow the perf-owner-first method before and after each execution change.
- Keeper requires exact-front instruction improvement, whole-workload
  non-regression, exact semantic/model parity, and zero fallback/redecision/
  escape violations.
- A non-keeper is reverted without widening source syntax.

## Required gates

1. **Zero-new-syntax gate**
   - canonical grammar adds none of the excluded spellings;
   - ordinary parser remains the only `.hako` grammar.

2. **Contract closure gate**
   - every active ContractName is registered or explicitly parked;
   - unknown names fail fast;
   - no wildcard/default contract.

3. **Capability closure gate**
   - required IDs are known;
   - unknown/typo `uses` entries never authorize execution;
   - `verified=true` has one explicit verifier owner.

4. **Effect-scope gate**
   - claims remain no broader than current direct Alloc and explicit
     Safepoint checks until transitive call summaries are implemented.

5. **Proof completeness gate**
   - bounds, overflow, extent, stability, layout, alignment, initialized range,
     mutability, lifetime, and no-escape obligations are independently visible;
   - no single `verified` boolean substitutes for missing proof ingredients.

6. **Plan-only backend gate**
   - backend consumes the exact selected plan;
   - no box/method/profile-name inference;
   - no fallback or route redecision.

7. **Model differential gate**
   - ordinary Hako model and backend-private implementation agree on exact
     results/failures for the declared corpus.

8. **ABI isolation gate**
   - Core C ABI and TypeBox ABI v2 remain the only canonical ABI surfaces;
   - hidden leaves remain generated and non-exported.

9. **Backend preflight gate**
   - unsupported target rejects before external effects;
   - VM is not a product fallback.

10. **Perf keeper gate**
    - baseline front, owner, instructions, cycles, allocations, and workload
      are recorded;
    - no performance claim without measured evidence.

11. **Source-size gate**
    - every source file remains below 800 lines;
    - split contract registry, proof producers, plan, verifier, backend adapter,
      and fixtures by responsibility.

## Implementation may claim

Only after the corresponding rows are green:

```text
low-level V0 adds zero source keywords
selected fast paths are contract- and plan-driven
NativePtr remains opaque
selected internal views are bounded and no-escape
unsupported backends fail before effects
ordinary Hako model parity holds for the selected operation
public ABI surface count changed = 0
```

## Implementation must not claim

```text
general unsafe Hakorune
general pointer or FFI memory safety
generic Span<T> / DirectArray<T>
public Bytes / BytesMut API
general AtomicI64 or selectable memory order
transitive no_alloc/no_safepoint before call summaries prove it
all backends support FastMem
FastLeafManifest is already live
inline MIR/asm/model language support
backend selfhost complete
```

## Stop conditions

Stop and return to design if:

1. a new keyword or inline sublanguage becomes necessary;
2. ContractName/effect/capability truth is duplicated across layers;
3. implicit requirements versus explicit source declarations are unresolved;
4. unknown `uses` names remain silently ignored while authorizing execution;
5. `CapabilityPlan.verified` lacks one owner and exact meaning;
6. no_alloc/no_safepoint claims exceed implemented direct checks;
7. a view can escape, alias mutably, or survive its owner/lifetime;
8. `Bytes` naming is activated without resolving its existing collisions;
9. NativePtr gains source dereference/index/arithmetic;
10. backend infers a route from names instead of a verified plan;
11. unsupported backend silently falls back to VM/generic execution;
12. a hidden typed entry becomes a public third ABI;
13. BoxShape and a new MemOp/backend BoxCount are mixed in one commit;
14. perf work begins before the exact-front baseline identifies the owner;
15. one source file approaches 800 lines.

## Relationship to existing workstreams

This task refines, and does not replace:

```text
docs/development/current/main/workstreams/direct-memory-current.md
docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
docs/development/current/main/design/span-no-escape-ssot.md
docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
docs/development/current/main/design/perf-owner-first-optimization-ssot.md
```

It may become active only after `CURRENT_STATE.toml` selects it. The current
SourceSnapshot source-carrier design stop remains unchanged.
