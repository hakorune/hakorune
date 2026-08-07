---
Status: SSOT
Date: 2026-08-07
Decision: accepted — `LOOP-COMMON-PHYSICAL-DEMAND-AND-SESSION0-D0`
Activation: 0; the current executable row remains `RECIPE-COSEAL-I0-R0`
Scope: common Loop physical demand, fresh unpublished function session, failure discard, completion/DraftSeal handoff
Related:
  - docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/builder/resolved_lowering/README.md
---

# Loop Common Physical Demand and Session SSOT

## Decision

Close the post-Recipe boundary before physical implementation begins.

```text
resolver / source map
  -> VerifiedLoopRecipeCoSealV1
  -> disjoint physical-admission split
       loop: VerifiedLoopPhysicalDemandV1
       boundary evidence
  -> one thin profile admission
       VerifiedCallableLoopPhysicalAdmissionV1
       OR VerifiedGenericG0LoopPhysicalAdmissionV1
  -> one fresh unpublished function session
  -> outer callable lowerer + one common recursive Loop physicalizer
  -> open After continuation
  -> existing completion / DraftSeal
  -> one unpublished function draft
```

The common `VerifiedLoopPhysicalDemandV1` and each thin profile admission are
move-only, AST-free, and physical-ID-free. A profile admission consumes one
inner demand plus already sealed boundary capabilities and fixes their exact
compatibility before the first Builder effect. Only the common inner demand is
consumed by the recursive Loop physicalizer. Neither product is a new Recipe,
selector, SSA, CFG, PHI, transaction, Return writer, or publication owner.

The existing owners remain authoritative:

| Concern | Sole owner |
| --- | --- |
| source membership, owner, frame, Scope/Region | resolver ledger and source map |
| logical operations, keys, recursive nesting | `LoopRecipeV1` |
| logical ports, edges, carrier obligations | `LoopJoinSigV1` |
| source/effect/input relations | `VerifiedLoopRecipeCoSealV1` |
| `BindingRef -> ValueId`, lexical SSA | `CanonicalSsaFunctionSessionV2.identity` |
| physical blocks, predecessors, sealing | `CanonicalCfgSessionV1` |
| provisional and patched PHI lifecycle | the function session's one `PhiTxn` |
| source completion evidence | `VerifiedFunctionCompletionV1` plus exact terminal value/ABI capability |
| mutable physical completion consumption | fresh `CanonicalSsaFunctionSessionV2.completion` / `ResolvedFunctionCompletionConsumptionV1` |
| captured caller restore, unpublished discard, prepared close | `CanonicalFunctionLoweringSessionV1` |
| detached DraftSeal prepare and rejected-owner retention | `OpenFunctionDraftSealV1` |
| sole function commit terminal | `PreparedFunctionDraftSealV1::commit` through prepared session close |
| draft collection and module publication | `ModuleDraftCollectorV1` plus the existing module transaction / `ModuleBuilderInvocationSessionV1` |

## Why this boundary exists

Recipe completion is not physical completion. A verified Recipe proves the
logical program, but it does not prove:

- which already-resolved callable prelude supplies an external value;
- that every Recipe input can be materialized in the preheader;
- that the function return has an exact supported ABI;
- that the terminal source binding is the value returned;
- that one fresh session can finish CFG, SSA, PHI, completion, and DraftSeal;
- that late failure leaves the live caller unchanged.

These obligations must be sealed once before mutation. A physicalizer must not
rediscover them from AST, source names, route labels, or existing MIR.

## Product boundary

### Common product

The accepted conceptual shape has two layers:

```text
VerifiedCallableLoopPhysicalAdmissionV1
  loop: VerifiedLoopPhysicalDemandV1
    core: transferred Recipe/Core/JoinSig/source-effect relations
    topology: VerifiedLoopPhysicalTopologyV1
  boundary: VerifiedLoopCallableBoundaryV1

VerifiedGenericG0LoopPhysicalAdmissionV1
  loop: VerifiedLoopPhysicalDemandV1
  boundary: VerifiedGenericG0LoopBoundaryV1
```

The exact Rust field split may remain private, but the following contract is
fixed.

The common splitter consumes the co-seal once and performs a disjoint move into
the inner Loop demand and boundary evidence. It never borrows, clones, or
re-catalogs the co-seal. The inner receives, without duplication:

- verified Recipe/Core and JoinSig;
- semantic owner/origin/source-kind, loop source and execution frame;
- Scope/Region relation;
- exact operation-source and input-source relations;
- the logical Loop After capability.

The inner `topology` is a non-owning, key-only projection over existing
logical keys:

- Recipe binding/value/item/block keys and their placement roles;
- Recipe input value -> logical preheader port;
- Recipe item -> owning Loop/Block + exact input/output value keys;
- JoinSig port/edge obligations -> logical placement roles.

`BindingRef`, source site, source/effect relation, and semantic identity truth
remain solely in the moved co-seal relation owner held by the inner demand.
Topology refers to those logical keys; it does not issue, copy, or re-verify
their source truth. It exists so the physical consumer performs no shape
matching or policy decision.

The callable sibling `boundary` co-seals the non-Loop obligations:

- exact prelude caller/site/target/result contract when a prelude result is
  required;
- destination source `BindingRef` for that result;
- exact terminal return statement and value sites;
- exact terminal source `BindingRef`;
- exact supported return ABI capability;
- the matching `VerifiedFunctionCompletionV1`.

The prelude contract contains no `ValueId`. The outer callable lowerer consumes
the callable admission, emits the prelude through the existing call owner,
immediately binds its physical result inside the same session, passes only the
inner Loop demand to the Loop physicalizer, and retains the sibling boundary
for Tail and Completion.

The Generic G0 admission wraps one instance of the same common inner-demand
type but retains its existing `L0.After/b1` boundary capability. It neither
reuses the callable prefix `value` Tail nor creates a G0 physicalizer.

The G0 adapter must consume the existing S4 product into a common co-seal view
by a disjoint move of its already verified Core/relations/After evidence. If
that view cannot be issued without copying source truth or re-verifying the
Recipe, the G0 adapter is `NoSafeSlice` and parity remains parked.

### These are admission envelopes, not callable megaboxes

No profile admission becomes a universal callable semantic owner. It owns
no new truth and implements no new Call, ABI, Loop, Return, or publication
algorithm. It only proves that already sealed capabilities belong to the same
callable execution:

```text
source/target identity  -> existing resolver/catalog authority
argument/result ABI     -> existing verified ABI capabilities
Loop meaning            -> Recipe/JoinSig/co-seal
terminal disposition    -> existing completion capability
physical commit         -> existing DraftSeal owner

profile admission envelope
  -> one exact owner/site/BindingRef compatibility proof
  -> one fixed Prelude -> Loop -> Tail -> Completion order
```

Prelude/Input, Loop, Tail/Return, and Completion stay typed sub-capabilities.
The two-layer product prevents the Loop physicalizer from observing the
callable boundary at all; only the outer callable lowerer sees both siblings.
They are not flattened into an opaque `CallablePlan` payload. The envelope
moves or borrows sealed evidence and cannot copy facts into a second catalog.
A non-Loop callable remains outside this Loop-specific admission envelope, so
this D0 does not pre-empt the final general callable design.

### Completion and ABI are separate

`VerifiedFunctionCompletionV1` is necessary but insufficient. It seals exit
cardinality, terminal statement kind, target function, cleanup, and declared
result contract. It does not by itself carry the return value `BindingRef`,
the return expression site, or a concrete physical ABI. An unannotated
explicit return can therefore pass completion verification without being safe
for this physical row.

Each profile admission with a value return requires both:

```text
exact terminal value site + BindingRef + exact return ABI
AND
matching VerifiedFunctionCompletionV1
```

For the first row the supported ABI is the already verified exact trivial
`i64` capability. Unannotated, dynamic, unknown, or inferred-by-name return
types reject before Builder effects. Later ABI profiles require separate
verified capabilities, not widening inside the physicalizer.

### Loop After and callable Tail are separate

The selected callable profile returns the prefix `value` binding:

```hako
local value = helper.to_i64(n)
local i = 0
loop i < 1 { i = i + 1 }
return value
```

Its terminal operand is not a Loop carrier After value. Generic G0 currently
returns `L0.After/b1`; that is a different profile adapter.

```text
logical Loop After capability != callable terminal Tail capability
```

The callable admission keeps both fields distinct. A profile adapter may prove
the same binding supplies both, but no consumer may infer that equality.
Generic G0's `VerifiedGenericAfterEffectG0` remains its boundary input and is
adapted beside the same common inner demand; it is neither the common Loop
authority nor the callable Tail authority.

## Forbidden contents

The upper/inner demands and their co-seal must not contain:

```text
AST / StmtRef / ExprRef
source or callable name selectors
path-suffix or ordinal rematching
legacy route ID or scheduler cursor
ValueId / BasicBlockId / PHI destination
MirBuilder or CanonicalSsaFunctionSessionV2
PhiTxn or rollback journal
ResolvedFunctionCompletionConsumptionV1
retry / fallback / reselection
commit or publication capability
```

The old DirectAccum-only `VerifiedLoopPhysicalInputV1` contains Recipe and
JoinSig only. It is a pilot input and must not be renamed or reused as the
final common demand; it lacks the co-sealed source/effect, topology, ABI,
Tail, and completion admission contract.

No session brand is added merely to pair either demand with a session. The
pre-effect issuer verifies semantic owner/frame/scope contracts; the consumer
then checks them against the freshly opened existing session. If the existing
session cannot expose the required admission facts, the result is
`SessionAdmissionUnavailable`, not a second session identity.

## Exact consumption

The common splitter consumes one `VerifiedLoopRecipeCoSealV1` and either issues
one non-Clone inner demand plus disjoint boundary evidence or returns a typed
rejection retaining the sole unconsumed owner. A thin callable or G0 adapter
then consumes exactly one pair and issues one admission. Neither step re-runs
Recipe verification, mints keys, or consults the legacy scheduler.

The outer profile entry consumes one admission while borrowing one
`CanonicalSsaFunctionSessionV2`. It transfers the inner demand exactly once to
the physicalizer and retains its sibling boundary until Tail/Completion.
Lowering by `&demand`, cloning a split/admission product, recreating one from
MIR, or trying a second route is forbidden.

Logical keys map to physical owners as follows:

| Logical evidence | Physical interpretation |
| --- | --- |
| `LoopBindingKey` + source `BindingRef` | canonical identity/BindingSSA |
| Recipe input + preheader relation | outer prelude/input materialization |
| `LoopItemKey` + owning block + value keys | common recursive physicalizer |
| JoinSig port/edge role | canonical CFG allocation and sealing |
| carrier obligation | canonical identity plus the one PHI transaction |
| Loop After capability | open continuation result only |
| terminal Tail capability | outer callable lowerer and completion consumer |

The physicalizer returns an open After/continuation receipt. It must not write
`Return`, take the function, publish a draft, or close the module.

## Fresh session and atomic failure law

Neither demand owns freshness or rollback. Existing transactions do.

```text
preflight failure
  -> no Builder/session effect

physical failure before or after provisional MIR effects
  -> rollback only still-pending unpatched provisional PHIs
  -> retain any PHI cleanup failure in the typed failure
  -> discard the complete unpublished function session
  -> restore the captured caller once
  -> no repair, retry, fallback, or route advance

fresh request
  -> open a new candidate/session from source authority
  -> allocate new physical IDs
  -> lower independently
```

The sole owners are:

- fresh module/candidate state: existing `ModuleBuilderInvocationSessionV1`
  with the canonical Fresh seed policy;
- fresh function state and caller capture:
  `CanonicalFunctionLoweringSessionV1` over the existing function-owned state
  transaction;
- PHI-local provisional abort: the session's `PhiTxn`;
- whole unpublished function discard:
  `CanonicalFunctionLoweringSessionV1::discard_unpublished` or rejected
  DraftSeal discard;
- function commit: `PreparedFunctionDraftSealV1::commit` through the prepared
  function-session close;
- module commit: the existing module transaction / `ModuleBuilderInvocationSessionV1`
  terminal after `ModuleDraftCollectorV1` admission.

There is no Loop-local Builder clone, `LoopEmissionDraft`, undo log, second
transaction, or same-session retry. A fresh-session proof compares semantic
result and live-caller fingerprints; it must not require `ValueId` or
`BasicBlockId` numbers to match across sessions.

`PhiTxn::abort_on_err` rolls back only provisional PHIs that are still pending
and unpatched. It does not repair patched PHIs, other MIR instructions, or ID
allocation. Those effects remain inside the poisoned unpublished function and
are removed only by whole-session discard.

## Target common finish order

The new common path must converge on this order and finish every existing owner
before DraftSeal. Current profile pilots differ; their order is parity evidence,
not authority for the final common path.

```text
1. materialize verified callable prelude and Recipe inputs
2. physicalize the recursive Loop, leaving After open
3. materialize the verified Tail operand in the outer callable lowerer
4. close semantic scopes and seal the After/terminal CFG
5. finish CanonicalCfgSessionV1
6. finish semantic, If-control, and identity/BindingSSA preconditions
7. commit the one PhiTxn
8. finish the remaining resolved-binding ledger and
   ResolvedFunctionCompletionConsumptionV1
9. prepare every detached DraftSeal check
10. commit DraftSeal once
```

The current production resolved DirectAccum lowerer is a parity oracle, not the
final common owner. Its `CanonicalDirectAccumSsaLowererV1::lower` path lacks a
whole-function `CanonicalCfgSessionV1::finish` call; the test-only caller-zero
finish path already has one. The production omission must not be copied into
the common path. The common canary must prove CFG finish before DraftSeal.

## Typed rejection boundary

Before Builder effects, reject at least:

```text
foreign owner/origin/source-kind/frame/Scope/Region
missing, duplicate, foreign, or unconsumed logical key/relation
Recipe item/block owner mismatch
JoinSig port/edge mismatch
input without an exact preheader producer
prefix target/result ABI unavailable
prefix destination BindingRef mismatch
terminal value site/BindingRef mismatch
missing or unsupported exact return ABI
completion owner/site/result-kind mismatch
Loop After confused with callable Tail
unsupported logical operation, exit, or recursive depth
second Recipe/SSA/CFG/PHI/completion owner
physical ID or Builder capability present in the demand
```

These are typed `NoSafeSlice`/contract rejections. They do not fall back to a
profile-specific physicalizer or the 19-route scheduler.

After physical effects begin, any failure is terminal for that unpublished
session. It is not reclassified as a pre-effect decline.

## One recursive algebra; 19 is coverage only

The inner physical demand accepts the one recursive `LoopRecipeV1` algebra. It does
not contain `DirectAccum`, `GenericG0`, `LoopTrue`, `LoopCond`, or the 19 legacy
route labels as physical variants.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one
profile admissions:       bounded callable/G0 adapters
inner Loop demand:        one
common physicalizer:      one
```

If the selected callable profile cannot later enter the existing family
selection envelope exactly, production selection returns `NoCandidate` and
parks it. Shape similarity must not relabel it as LoopV0 or Generic G0, and a
20th Recipe kind or second selector is forbidden.

## Finite implementation ladder

This D0 closes the common demand, fresh-session, failure-discard, and
completion/DraftSeal architecture together. Do not reopen them as a deep chain
of nested design suffixes unless a code audit proves one named missing owner.

| Order | Row | One claim | Stop line |
| ---: | --- | --- | --- |
| 0 | `RECIPE-COSEAL-I0-R0` | caller-zero logical co-seal | current executable row; stop after closeout |
| 1 | `LOOP-COMMON-PHYSICAL-DEMAND-I0-R0` | co-seal -> one disjoint split and one profile-neutral inner Loop demand | no callable/G0 selection, Builder, session, or physical ID |
| 2 | `CALLABLE-LOOP-PHYSICAL-ADMISSION-I0-R0` | callable co-seal boundary + same inner demand -> one callable admission | caller-zero; no physicalizer or production selector |
| 3 | `GENERIC-G0-COMMON-PHYSICAL-ADMISSION-P0` | existing G0 S4 product -> common co-seal view -> the same inner-demand schema + distinct G0 After boundary | no G0 physicalizer, callable Tail reuse, or production caller |
| 4 | `LOOP-PHYSICALIZER-COMMON-OWNER-R0` | split the over-budget DirectAccum owner into common services plus thin adapter | behavior-neutral; no new accepted Recipe |
| 5 | `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` | inner demand + borrowed V2 session -> open After candidate | caller-zero; no Return/DraftSeal/publication |
| 6 | `CALLABLE-LOOP-PHYSICAL-CANARY-I0-R0` | exact prelude + Loop + Tail + CFG/SSA/PHI finish + completion/DraftSeal on one fresh unpublished function | late failure discards whole session; production caller zero |
| 7 | existing `GENERIC-G0-COMPLETION-P0` | G0 `L0.After/b1` boundary -> the same completion/DraftSeal owner | no second Return writer or callable Tail reuse |
| 8 | `LOOP-CALLER-ZERO-PARITY-G0` | callable and G0 admissions prove the same inner-demand schema/physicalizer while preserving distinct Tail/After contracts | no family relabeling or production selection |
| 9 | existing M8 S6A..S6G | close the 19-ingress portable coverage cohorts and all-19 proof | does not complete M9 or select production |
| 10 | existing M9 S7A..S7G | close Rust/.hako portable producer parity | does not activate the physical caller |
| 11 | `LOOP-PRODUCTION-SELECTION-D0` | decide exact family admission after all required gates | human consultation stop; `NoCandidate` is valid |
| 12 | existing `M10b-I0-R0` | one production switch and same-commit retry/scheduler/old-edge deletion | no fallback |
| 13 | existing R1/M11/M12/R2 rows | manifest-led legacy retirement and sole-authority proof | cutover must already be green |

The physical canary does not complete M8, M9, production activation, or legacy
retirement. `Recipe complete`, `physical canary complete`, `production
selected`, and `legacy retired` remain four distinct claims.

## Implementation and documentation obligation

Every implementation row above must update its exact live references in the
same commit after code and focused tests land:

- `docs/reference/mir/loop-recipe-contract.md` for the landed co-seal/demand/
  physical boundary and sole-owner claims;
- `docs/reference/mir/generic-loop-stage-matrix.md` for caller-zero,
  canary, activation, and retirement status;
- `src/mir/loop_recipe_contract/README.md` and the owning canonical lowering
  README when their code contract changes;
- `docs/reference/mir/phi_policy.md` and `phi_invariants.md` only when a
  physical PHI contract actually changes;
- `CURRENT_STATE.toml` and the active rolling workstream for the next exact
  row and compact closeout;
- `docs/tools/check-scripts-index.md` only if a reusable public guard entry is
  added.

References must describe only landed behavior. This design SSOT may name the
accepted target now, but the reference pages must not claim physical,
production, backend, or retirement capability before the corresponding
implementation receipt exists.

## Current stop

The architecture is accepted in advance, but execution authority does not
move. The current row remains the bounded caller-zero
`RECIPE-COSEAL-I0-R0`. After it lands, stop and ask before opening
`LOOP-COMMON-PHYSICAL-DEMAND-I0-R0`.
