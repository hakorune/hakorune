---
Status: SSOT
Decision: accepted boundary; C-speed completion order accepted, widening parked
Date: 2026-08-05
Scope: inline metadata, MIR InlinePlan ownership, verifier gates, and backend responsibility boundaries.
Related:
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/current-optimization-mechanisms-ssot.md
  - docs/reference/mir/hints.md
  - docs/reference/runtime/substrate-capabilities.md
---

# InlinePlan SSOT

## Decision

Inline support is required for allocator-grade fast paths, but it must not be
implemented as an ad-hoc `inline` keyword or as backend-local symbol matching.

The accepted flow is:

```text
source rune metadata
-> MIR-owned InlinePlan / CallsiteInlinePlan
-> verifier acceptance
-> MIR transform or intrinsic route selection
-> backend emits the already-decided MIR/route
```

The backend is a consumer of already-decided shapes. It is not the inline
planner.

The C-speed amendment keeps the same source surface and adds no `Inline(auto)`
or callsite syntax:

```text
no rune          = future compiler-owned automatic profitability decision
Inline(prefer)   = advisory bias toward inline
Inline(avoid)    = advisory bias toward keep-call
Inline(required) = fail-fast elimination contract for every admitted exact
                   direct callsite in the current compilation product
```

`required` does not require deletion of an exported/address-taken callee body;
it requires residual admitted direct Call count zero. Indirect, unresolved,
cross-product, recursive, or otherwise unsealable callsites reject the strict
row before transform. The current implementation verifies narrow callee shape
and consumes selected calls, but module-wide residual-call proof remains a
named parked task below.

Explicit Home release is not an inline use case. Contextual `release root`
resolves to `VerifiedExplicitHomeReleasePlanV1` and the sole owner-ending
operation without an ordinary/generic wrapper Call, InlinePlan, or DCE
dependency.

## Vocabulary Split

Inline-related annotations have three different meanings:

```text
Hint:
  advisory; may be ignored without changing program meaning

Contract:
  verifier-backed obligation; may be used only after proof

Lowering:
  backend/lane acceptance requirement; failure is a compile-time reject in the
  lanes that opt into it
```

Current public surface:

```hako
@rune Inline(prefer)
@rune Inline(avoid)
@rune Inline(required)
@rune Hint(hot)
@rune Hint(cold)
```

Compatibility surface:

```hako
@rune Hint(inline)
@rune Hint(noinline)
@rune Lowering(inline_required)
```

M11c-required-vocab live surface:

```text
@rune Inline(required)
-> MIR InlinePlan request=required
-> requires=[]
-> verified=false
-> fallback=fail_fast
-> source=rune_inline_required
```

`Lowering(inline_required)` remains accepted as a compat spelling and still
maps to the same required InlinePlan request. `M11c-required-verify` owns the
verifier transition for this metadata. Required inline still is not
backend-active.

M11c-required-verify live surface:

```text
@rune Inline(required)
-> MIR InlinePlan request=required
-> verifier checks narrow leaf-inline shape
-> verified=true only when accepted
-> fail-fast diagnostics on unsupported shapes
```

The live advisory pure leaf row accepts:

```text
one entry block
Return terminator
instruction_count <= 8
Const / UnaryOp / BinOp / Compare / StaticDataLoad / Copy / Select / TypeOp
no nested Call
no dynamic dispatch
no recursive cycle
```

The live required leaf row, including its admitted single-base receiver-field
shape, uses the current implementation budget:

```text
instruction_count <= 16
```

`Hint(inline)` is not `Inline(required)`.

Future `@rune Profile(...)` expansion may produce `Hint(...)`, inline requests,
and `Contract(...)` facts, but Profile is not an InlinePlan truth source. The
MIR facts produced by expansion are the only facts the optimizer and backend
may consume.

Required field-set leaf shape:

```text
@rune Inline(required)
-> InlinePlan request=required
-> verifier shape=single_object_fieldset_leaf
-> proof_source=leaf_shape_inference
-> inferred_no_alloc=1
-> inferred_no_safepoint=1
-> fallback=fail_fast
```

This is the preferred source spelling for small leaf helpers such as receiver
reset methods and small result capsule setters. `Inline(required)` is a proof
request, not a safety grant: the verifier must inspect the body and accept only
a narrow leaf shape before transform.

The first accepted field-set leaf shape is `single_object_fieldset_leaf`:

```text
allowed:
  Const
  Copy
  FieldSet on one stable base
  Return

forbidden:
  Call
  NewBox
  Array/Map op
  String op
  Publish/freeze
  Branch/Loop
  Dynamic dispatch
  Cross-module call
```

When that shape is accepted, `no_alloc` and `no_safepoint` are inferred from
the body shape. They are not silently granted by the source rune.

### Mixed-Base Publication Helpers Are Not Leaf Inline

`Inline(required)` must not grow into a general "small helper inliner" for
multi-base bodies. The PageQueue mixed-base helper shape is the canonical
negative example:

```hako
helper(index, page, kind) {
    me.last_selected_index = index
    me.last_selected_page_id = page.page_id
    me.last_selected_kind = kind
    me.last_selected_page = page
    me.select_count = me.select_count + 1
    return 1
}
```

This body has a receiver base (`me`) and a foreign base (`page`). It also mixes
scalar snapshot publication with possible handle publication. That is not
`single_object_fieldset_leaf`; widening the generic verifier for this shape
would blur inline policy, effect classification, and publication semantics.

Reopen order:

```text
1. Same-module helper call lowering must not coredump.
   Unsupported lowering must be a compile-time diagnostic.

2. Add EffectSummary vocabulary:
   receiver_reads / receiver_writes
   foreign_reads / foreign_writes
   handle_publications
   calls / allocations / safepoints

3. If current evidence still selects the shape, add a narrow
   ReceiverSnapshotPublicationPlanV0 recipe.
```

`ReceiverSnapshotPublicationPlanV0` is a future recipe that `Inline(required)`
may consume after effect verification. It is not a replacement for the generic
leaf verifier and must reject multiple foreign bases, foreign writes, nested
calls, branch/loop bodies, allocation, dynamic field access, and handle
publication that requires a runtime barrier.

`@rune Profile(...)` is parked for v0. If repeated explicit annotations become
real user-facing noise later, a profile may be reintroduced as a named bundle
that expands to primitive MIR facts. The profile name itself must still never
be backend-active. Backends and `.inc` shims must consume only already-verified
MIR shape or already-inlined MIR.

`Hint(always_inline)` is also not `inline_required`; do not introduce it as a
public guarantee. If an always-inline spelling is ever accepted, it remains an
optimizer hint. Required inline belongs to the canonical `Inline(required)`
family.

## Allocator Lane Policy Freeze (docs-first)

Canonical semantics:

```text
Inline(prefer): optimizer hint, unsupported shapes keep_call
Inline(avoid): optimizer anti-hint, keep_call
Inline(required): required inline contract (not a stronger hint)
Contract(no_alloc/no_safepoint): independent obligations for rows that cannot
                                infer them from an admitted narrow shape
backend: consume verified MIR InlinePlan / already-inlined MIR only
```

Lane behavior:

```text
required lane inactive:
  preserve metadata only (no transform)
required verifier lane active:
  missing contracts or unsupported shape -> fail_fast
required transform lane active:
  verified=false or transform-miss -> fail_fast
```

Allocator lane activation boundary:

```text
1. docs-only boundary lock
2. metadata preserve
3. verifier-only required-inline checks
4. MIR transform for selected verified same-module scalar leaf helpers only
5. backend consumer only (no rune-string planning in backend)
```

## MIR Ownership

Backends must not read source rune strings and decide to inline. Source metadata
is normalized into MIR-owned facts first.

Proposed function-level shape:

```json
{
  "inline_plans": [
    {
      "function": "Main.align_up/2",
      "request": "prefer",
      "hotness": "hot",
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "keep_call"
    }
  ]
}
```

Proposed callsite-level shape:

```json
{
  "callsite_inline_plans": [
    {
      "callsite": 17,
      "callee": "Main.align_up/2",
      "mode": "prefer",
      "proof": null,
      "fallback": "keep_call"
    }
  ]
}
```

Required inline rows may use:

```json
{
  "function": "MiHeap.alloc_small/1",
  "request": "required",
  "reason": "allocator_fast_path",
  "max_ir": 48,
  "requires": [],
  "verified": true,
  "fallback": "fail_fast"
}
```

The exact serialized schema may be narrowed by implementation cards, but the
truth remains MIR-owned. `.inc`, ll_emit, and C shims do not infer inline policy
from function names.

M11c-preserve live schema:

```json
{
  "inline_plans": [
    {
      "function": "Main.align_up/2",
      "request": "prefer",
      "hotness": null,
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "keep_call",
      "source": "rune_hint"
    }
  ]
}
```

Current hint mapping:

```text
Inline(prefer) -> request=prefer
Inline(avoid)  -> request=avoid
Inline(required) -> request=required, fallback=fail_fast
compat Hint(inline)   -> request=prefer
compat Hint(noinline) -> request=avoid
Hint(hot)      -> request=none, hotness=hot
Hint(cold)     -> request=none, hotness=cold
```

`verified=false` and `fallback=keep_call` are part of the preservation
contract. Backends must not treat this row as a mandate.

M11c-soft-leaf live transform:

```text
Hint(inline) / request=prefer
-> same-module Callee::Global(name)
-> callee has one entry block, no PHI/control, no nested Call
-> callee body size <= 8 supported pure instructions
-> MIR optimizer expands the body at the callsite
-> unsupported shapes keep the original call
```

The supported first-row body vocabulary is intentionally narrow:

```text
Const
UnaryOp
BinOp
Compare
StaticDataLoad
Copy
Select
TypeOp
Return
```

Unannotated functions are not automatically inlined by this live-narrow row.
Default automatic leaf planning is an accepted C-speed target with production
activation 0; it must consume exact callsite facts and a bounded cost model,
not silently treat every small function as `prefer`.

`Hint(noinline)` / `request=avoid` wins over any soft inline attempt. This row
does not add required-inline semantics and does not make backends read
`inline_plans`.

M11c-required-vocab live schema:

```json
{
  "inline_plans": [
    {
      "function": "MiHeap.alloc_small/1",
      "request": "required",
      "hotness": null,
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "fail_fast",
      "source": "rune_lowering"
    }
  ]
}
```

`verified=false` remains valid for rejected or unverified shapes.
`M11c-required-verify` sets `verified=true` only after the leaf-inline shape
and any explicitly listed `plan.requires` obligations pass. Current canonical
`Inline(required)` plans carry `requires=[]`; accepted narrow shapes infer
`no_alloc` / `no_safepoint`. This still does not authorize backend-local
inlining.

## Inline Kinds

### MIR Function Inline

Use this for small `.hako` functions whose body can be expanded in MIR:

```text
align_up
size_to_bin
page_free_is_empty
block_next
```

First rows must be same-module and non-recursive. Cross-module, virtual method,
generic specialization, and dynamic dispatch inline are future rows.

### Intrinsic Route Lowering

Some calls should not be expanded as function bodies. They should lower to a
primitive route:

```text
hako.ptr.load_u64
hako.ptr.store_u64
hako.atomic.cas
hako.intrin.ctz_i64
hako.intrin.popcnt_i64
hako.intrin.prefetch
```

This remains separate from `@rune Inline(prefer)`. Intrinsic route selection must
flow through a registry/route fact, not through source-name matching in a
backend.

### Native Bitcode / LTO Inline

Native helper body import is reserved. It must not be the first allocator fast
path strategy because it risks moving semantic ownership back into C/native
helpers.

Preferred first strategy:

```text
hako.ptr / hako.atomic / hako.intrin route
-> direct MIR/LLVM primitive
```

Only after the substrate route is truthful should native bitcode/LTO be
considered.

## Required Inline Verifier Conditions

`Lowering(inline_required)` vocabulary is accepted by M11c-required-vocab.
M11c-required-verify now fail-fast rejects required-inline plans that do not
satisfy the supported narrow leaf-inline shape. Supported receiver-fieldset
leaf bodies infer `no_alloc` / `no_safepoint` from MIR shape; source-visible
`Contract(...)` rows are not required for that row.

Minimum required checks:

- callee exists and resolves to a single same-module body
- body size is within the row budget
- recursive inline cycle is absent
- unsupported dynamic dispatch is absent
- unsupported call is absent, unless it is intrinsic-routed or itself verified
  inline
- `no_alloc` / `no_safepoint` are either inferred from the accepted leaf shape
  or provided by a future row's explicit verifier-backed contract
- capability access stays within the row's allowed modules

After `INLINE-REQUIRED-RESIDUAL-CALL0-I0`, verification also requires:

- one owner-branded set of admitted exact direct callsites;
- every member rewritten exactly once or rejected before publication;
- residual admitted direct `Call` count zero after the selected transform and
  post-inline cleanup;
- no retry through a backend inliner, LTO, symbol-name rule, or compatibility
  path.

This is a target contract, not a claim about the current narrow verifier.

## Post-inline simplification boundary

The current optimizer runs canonical simplification and memory cleanup before
the late leaf-inline pass. Therefore inline-created `Copy`, constant
expressions, dead temporaries, and newly trivial CFG are not presently closed
by a MIR-owned post-inline cleanup wave.

The accepted future boundary is:

```text
InlinePlan transform
-> bounded PostInlineSimplifyPlanV1
-> selected canonical SimplifyCFG / constant / CSE / DCE consumers
-> required residual-call verifier
-> backend
```

The exact pass subset and iteration bound belong to
`INLINE-POST-SIMPLIFY0-D0`. An unbounded fixed point, backend-only cleanup,
effectful-instruction deletion, or changing program meaning to satisfy
`required` is forbidden.

## Backend Boundary

Allowed backend behavior:

- emit already-inlined MIR
- emit already-selected intrinsic/capability routes
- emit fail-fast diagnostics carried by MIR/lowering facts

Forbidden backend behavior:

- searching callee bodies and inlining them
- checking `Mi*`, `Allocator*`, or other app-specific names
- treating `Hint(inline)` as a semantic guarantee
- deriving `no_alloc` / `no_safepoint` from a method name
- adding `.inc` branches such as "if symbol is size_to_bin, inline it"

## Implementation Rows

Use `M11c` for InlinePlan work. `M11b` is already reserved for static const
tables.

Recommended order relative to existing allocator substrate rows:

```text
M11c-docs:
  InlinePlan boundary lock. No behavior change.

M11b-eval:
  const expression / const fn table generation.
  Keeps table generation complete before inline transforms start.

M11c-preserve:
  preserve existing Hint(inline/noinline/hot/cold) into MIR InlinePlan metadata.
  Live-narrow. No backend use.

M11c-soft-leaf:
  best-effort same-module leaf MIR inline.
  Live-narrow. Failed inline keeps the call.

M10c-pre:
  pointer/handle return proof vocabulary.

M10c:
  strong LLVM attrs widening after pointer proof.

M11c-required-vocab:
  substrate-only Lowering(inline_required) vocabulary.
  Parser parity is required because this adds rune vocabulary.
  Live-narrow. Preserves request=required metadata only; no verifier/backend use.

M11c-contract-repeat:
  allow distinct Contract(...) runes on the same declaration.
  Live-narrow. This is parser metadata shape only; explicit obligations remain
  available for later rows whose effects cannot be inferred from shape.

M11c-required-verify:
  required inline verifier connection to narrow shape inference, any explicit
  plan requirements, and call graph checks.
  Live-narrow. Sets verified=true only for accepted leaf required-inline plans;
  backend use remains disabled.

M11d:
  EffectPlan / CapabilityPlan boundary.
  Defines the MIR-owned effect and capability facts required before Profile can
  safely expand allocator.fast or substrate.leaf into strict lowering facts.

M12:
  mimalloc raw-page proof.

M12b:
  Profile registry docs.
  Reserves profile names and expansion targets only.

M12c:
  Profile expansion to primitive rune/Plan facts.
  Backend still reads InlinePlan / EffectPlan / CapabilityPlan, not Profile
  strings.

M13:
  allocator fast-path EXE proof.
  Verified required InlinePlan is consumed by the MIR optimizer for a scalar
  same-module allocator-fast leaf before pure-first EXE. Backends still do not
  inspect InlinePlan rows or profile names.
```

This order keeps static table data, inline planning, pointer proof, and
allocator proof separated.

## C-speed boundary beyond inline

Inline owns only an ordinary callable boundary. It is necessary on some hot
paths, but it is not the whole C-speed contract. A broader speed claim must
also consume receipts from the independent owners for:

```text
exact value representation and ABI
exact direct/static dispatch without dynamic name lookup
StaticUnique Home paths with RC/control-cell work zero
allocator and raw-memory capability plans
loop/vectorization, bounds, alias, and memory-access plans
exact-front assembly/instruction evidence plus meso/whole contradiction gates
```

Those receipts stay in their existing SSOTs; InlinePlan must not absorb their
policy. `INLINE-C-SPEED0-G0` proves only the selected call-boundary contribution
and must not promote that result into a whole-language C-speed claim.

## C-speed completion task order

These rows are parked and do not move the current MirBuilder/Generic design
stop. They execute in this order when the optimization lane is explicitly
selected:

```text
INLINE-CURRENT-AUTHORITY-CLOSEOUT0-D0
-> INLINE-REQUIRED-CALLSITE-PLAN0-S0
-> INLINE-POST-SIMPLIFY0-D0
-> INLINE-POST-SIMPLIFY0-I0
-> INLINE-REQUIRED-RESIDUAL-CALL0-I0
-> INLINE-AUTO-LEAF-COST0-D0
-> INLINE-AUTO-LEAF0-I0
-> INLINE-C-SPEED0-G0
-> INLINE-REFERENCE-CLOSEOUT0-DOC0
```

`INLINE-CURRENT-AUTHORITY-CLOSEOUT0-D0` seals current truth before widening:
advisory pure-leaf budget 8, required-leaf budget 16, supported instruction
vocabulary, shape-inferred `no_alloc`/`no_safepoint`, compatibility aliases,
and the exact distinction between verifier acceptance and residual-call proof.

`INLINE-REQUIRED-CALLSITE-PLAN0-S0` publishes a caller-zero
`VerifiedRequiredInlineCallsitePlanV1` from exact callable identity, current
compilation-product membership, call graph/SCC facts, arity/receiver ABI, and
source provenance. It owns no transform.

`INLINE-POST-SIMPLIFY0-D0/I0` selects and then activates one bounded cleanup
wave after inline. `INLINE-REQUIRED-RESIDUAL-CALL0-I0` then consumes the sealed
callsite plan, requires every admitted site to have been rewritten exactly
once, and fails publication if any admitted direct Call remains after that
cleanup. It does not require callee symbol deletion and does not ask a backend
to repair a miss.

`INLINE-AUTO-LEAF-COST0-D0` fixes the static cost/code-size model and
`prefer`/`avoid` bias. `INLINE-AUTO-LEAF0-I0` admits only exact same-product
leaf callsites first; source annotation count may remain zero.

`INLINE-C-SPEED0-G0` uses exact/meso/whole fronts, assembly, instruction count,
and contradiction guards. A smaller MIR or erased Call without measured owner
improvement is not enough for a C-speed claim.

Generic inline is a separate later dependency chain:

```text
GEN-TYPE-SUBSTITUTION0-S0
-> GEN-INSTANCE-KEY0-S0
-> GEN-HOME-ABI0-S0
-> GEN-MONOMORPHIZE0-I0
-> GEN-INLINE0-I0
```

Current generic parsing/arity evidence is not type substitution,
monomorphization, or a concrete MIR instance. No dictionary or erased-generic
fallback is introduced by this chain. A generic `Inline(required)` contract is
checked per sealed concrete instance.

Multi-block/Home-aware inline remains conditional:

```text
exact direct call + ABI keeper
-> perf/asm still selects the call boundary as owner
-> INLINE-STRUCTURED-EVIDENCE0-D0
-> optional INLINE-STRUCTURED0-S0/I0
```

Until that evidence exists, multi-block hot core methods use
`HotCoreMethodSummaryV0` / `DirectExactHotCoreCallPlanV0`; the leaf verifier is
not widened into a universal CFG/Home inliner.

## Implementation-coupled reference updates

Every `I0` above updates the exact live reference surface and examples in the
same commit. `INLINE-REFERENCE-CLOSEOUT0-DOC0` audits the already-synchronized
state; it is not permission to defer documentation. At minimum each relevant
cell checks:

```text
docs/reference/language/runes.md
docs/reference/language/quick-reference.md
docs/reference/mir/hints.md
docs/reference/mir/metadata-facts-ssot.md
docs/reference/runtime/substrate-capabilities.md
docs/development/current/main/design/current-optimization-mechanisms-ssot.md
source/parser/MIR support matrix and migration notes
```

The receipt records current budgets and vocabulary, default-auto activation,
required admitted/residual counts, pass order, backend consumer boundary,
unsupported callsite diagnostics, and exact generic/structured non-claims.

## Diagnostics

Stable diagnostics for future rows:

```text
[inline-plan/required-not-verified]
[inline-plan/body-too-large]
[inline-plan/recursive-cycle]
[inline-plan/dynamic-dispatch]
[inline-plan/unsupported-call]
[inline-plan/missing-contract]
[inline-plan/backend-boundary]
```

## Non-Goals

- no `inline` keyword
- no public `always_inline` guarantee
- no `.inc` / ll_emit inliner
- no app-specific inline switch
- no backend-active use of `Hint(inline)` before MIR InlinePlan exists
- no backend-active required inline before a later lowering row consumes the
  verified MIR fact
- no release-to-inline dependency or generic release wrapper
- no automatic inline claim before `INLINE-AUTO-LEAF0-I0`
- no structured/multi-block/Home-aware claim without its evidence-gated row
