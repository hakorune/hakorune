# Loop physical prepare design correction R0

Status: `Decision: accepted after worker audit; bounded correction is fixed and implementation may proceed in caller-zero P0`
Date: 2026-08-07
Parent: `LOOP-PHYSICAL-PREPARE-P0`
Design authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

The post-Recipe architecture is accepted, but the current Rust authorities
leave three pairings implicit. Fix those pairings in one bounded design row
before any Builder, physicalizer, selector, or production caller is opened.
This is a correction of the existing P0 contract, not a new physical
architecture or a deeper task ladder.

## Required corrections

### 1. Callable input brand

`ResolvedFunctionLoweringInputV1` is an immutable `Clone + Copy` borrowed view.
It may have no callable index/header when created from a root source unit. A
callable prepared product must therefore consume a non-Clone
`VerifiedCallableFunctionLoweringInputV1` (name may remain private) that proves:

```text
exact ResolvedFunctionLoweringInputV1
current VerifiedCallableIndexV1
current VerifiedCallableHeaderV1
same owner/header/index relation
```

The callable prepare entry accepts only the branded view. Missing, foreign, or
mismatched catalog/header facts reject before a fresh session opens. The base
input's `Copy` behavior is not changed in this row.

### 2. Prelude target/result capability

`VerifiedCallablePreludeV1` currently carries source shape and an optional
resolved callable reference, not a callee header or result ABI. The prepare
boundary must resolve the target through the existing callable index/header
authority and issue one private prelude capability containing the exact target,
receiver/arity, and result contract. `direct_callable = None`, missing header,
arity mismatch, receiver mismatch, and unsupported result ABI are typed
`NoSafeSlice`; name/AST re-resolution and fallback are forbidden.

### 3. Terminal compatibility receipt

The prepared product must not pair bare ABI and Completion by proximity. The
prepare operation issues one move-only private
`VerifiedCallableTerminalCompatibilityV1` relation receipt proving:

```text
same function owner and target
tail statement == completion explicit site
completion is value-returning for this profile
tail value site/binding belongs to the co-sealed source claim
declared result spelling == exact ABI
```

The receipt is relational evidence, not a new semantic owner and not a second
ABI/Completion truth. It is consumed exactly once by the prepared product.

### 4. Generic G0 uses the same relation boundary

G0's typed source bundle and `VerifiedGenericAfterEffectG0` currently carry
owner/frame and a bare ABI but no terminal Completion/site relation. The G0
adapter must verify owner/source-type/return-ABI pairing and terminal
Tail/Completion pairing through the same private compatibility receipt. Any
missing Completion, owner mismatch, source-type/ABI mismatch, or tail/site
mismatch is pre-effect `NoSafeSlice`.

### 5. Lifetime wording

The prepared product is move-only, but its retained
`ResolvedFunctionLoweringInputV1` is a borrowed exact view. Only the moved
Loop demand and compatibility receipts are owned AST-free products. Therefore:

```text
co-seal/source-map drop:
  owned demand/receipts remain valid

prepared input:
  valid only while its borrowed resolved input lifetime remains valid
```

No source-view-drop claim may imply that borrowed AST/forest/projection data
outlives its source lifetime.

## Sole owners and forbidden authority

| Concern | Existing owner | P0 correction owner |
| --- | --- | --- |
| source/function/header/index | resolver and callable catalog | branded borrowed input receipt |
| Loop meaning and relations | Recipe/JoinSig/co-seal | moved common demand |
| prelude target/result | callable index/header | private prelude capability |
| terminal ABI/Completion pairing | existing ABI + Completion issuers | private compatibility receipt |
| ValueId/CFG/PHI | fresh `CanonicalSsaFunctionSessionV2` | unchanged; not in P0 |

The correction must not add a universal CallablePlan, second selector,
physical ID, Builder capability, AST rematch, name lookup, retry, fallback,
publication, or production caller.

## P0 implementation progress (caller-zero, not closeout)

The first bounded slice now exists in test-only
`src/mir/compiler/loop_physical_prepare.rs`:

```text
VerifiedCallableFunctionLoweringInputV1
VerifiedLoopPhysicalDemandV1
VerifiedCallablePreludeCapabilityV1
VerifiedCallableTerminalCompatibilityV1
PreparedCallableLoopPhysicalizationV1
```

It has no session/Builder effect and the focused contracts cover exact input
branding, owned co-seal lifetime, and the current MethodCall fixture's typed
`MissingPreludeTarget` reject. This is deliberately **partial P0 progress**.
The existing fixture has no resolver-issued direct callable target and its
return declaration is unannotated, so it cannot be used as a positive
Prepared/ABI witness. Do not inject a free-static target or treat receiver
`Other` as proof of a static call. A positive prelude/terminal witness requires
a separately verified static-call fixture/profile, which is a bounded source
authority decision before it is added. P0 remains open until the proof matrix
has a genuine positive plus the required foreign/arity/result/site/value and
consumption negatives.

Focused evidence for this partial slice is:

```text
cargo test --lib loop_physical_prepare
```

This is a caller-zero contract test, not a physical or production activation
gate.

## Pre-effect rejection boundary

Reject before session effects on:

```text
missing/foreign callable input index or header
owner/header/index mismatch
missing or unresolved prelude target
receiver/arity/result contract mismatch
tail owner/site/value-binding mismatch
completion owner/target/site/value-kind mismatch
declared result ABI missing/unsupported/mismatched
G0 source brand/typed bundle/After owner mismatch
duplicate/consumed/reissued co-seal or compatibility receipt
AST rematch, ValueId, BasicBlockId, CFG, PHI, or Builder presence
```

After a fresh session opens, any failure remains whole-session discard; it is
never reclassified as a pre-effect decline or retried in another route.

## Acceptance

This row is complete when:

- the callable brand, prelude capability, and terminal compatibility receipt
  are specified in the common SSOT and task map;
- G0 uses the same relation check without copying source truth;
- positive and every listed negative boundary has a typed test/proof plan;
- owned AST-free demand/receipts and borrowed input lifetime are stated
  separately;
- no production caller, physicalizer, selector, or Builder effect is added;
- the next row is exactly `LOOP-PHYSICAL-PREPARE-P0` caller-zero implementation;
- README/reference/current mirror/guard entries are synchronized in the same
  design-correction commit.

The proof matrix is fixed before implementation:

| Proof group | Positive | Required typed negatives |
| --- | --- | --- |
| callable input brand | current input + header/index | missing header/index, foreign owner, header/index mismatch |
| prelude capability | resolved target + exact arity/result | no target, missing callee header, receiver/arity/result mismatch |
| terminal compatibility | Tail + value Completion + exact ABI | owner/site/value-kind/declared-ABI mismatch, duplicate receipt |
| G0 adapter | source bundle + After + Completion relation | source brand/owner/type/ABI/site mismatch, missing Completion |
| lifetime/consumption | one prepared product survives co-seal drop within borrowed input lifetime | reissue, clone, source-lifetime overclaim, physical authority present |

The matrix is the design acceptance boundary. The P0 implementation must land
each group as a focused contract test or typed proof receipt before exposing
the prepared product to any later physical row. No Builder or physical test
is part of this correction row.

`LOOP-PHYSICAL-PREPARE-P0` may open only after this row is accepted. P0 remains
caller-zero and is not an I0; a real production switch receives a later I0
row after canary/parity gates.
