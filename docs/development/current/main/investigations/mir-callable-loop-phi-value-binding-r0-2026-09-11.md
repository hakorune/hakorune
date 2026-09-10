---
Status: implementation ready
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-VALUE-BINDING-R0
Parent: docs/development/current/main/investigations/mirbuilder-post-audit-follow-up-queue-2026-08-21.md
---

# MIR-CALLABLE-LOOP-PHI-VALUE-BINDING-R0

## Six-line brief

```text
Decision: connect the first source-backed callable Loop consumer to the existing canonical Binding SSA/PHI owner; preserve the current Recipe and physical skeleton.
Source authority + canonical issuer: CallableSemanticLoweringState and its verified source-bound schedule own BindingRef/role/site; BindingSsaBuilderV1 with MirBindingSsaAdapterV1 and PhiTxn owns physical ValueId/PHI issuance.
Non-authority: variable_map, composer-local phi_bindings, carrier_step_phis, names, ValueId order, source "latest" values, and any fallback or retry.
Fail-fast boundary: owner/site, BindingRef, generation, predecessor/edge, definition dominance, and seal are checked before CFG/PHI Builder effects; foreign, stale, missing, or unsealed evidence returns a named reject.
Smallest next slice: the existing RawInvocationChildPortV1 Ready edge -> CallableGenericLoopV1PhysicalAdapterV1 -> source-port composer, replacing the current carrier-local PHI issuance with the canonical owner relation one time.
Non-claims: no new Facts/Recipe issuer, local-completion handoff, generic fallback deletion, backend/OBJ/EXE result, runtime change, or aggregate R7 LegacyCallV0 retirement.
```

## Selected boundary

The production caller is the existing `RawInvocationChildPortV1::lower_loop`
Ready branch in `src/mir/builder/raw_loop_child_port.rs`. It already moves the
source-backed Recipe through
`CallableGenericLoopV1PhysicalAdapterV1::lower`, so this row does not add a
selector or a second Loop consumer. The current adapter calls the existing
`RecipeComposer::compose_source_generic_loop_v1_recipe_with_port`; the change
is limited to how its accepted binding rows reach physical SSA/PHI.

The logical schedule remains the sole source authority. It supplies each
`BindingRefV1` with its exact role and source site. The canonical Binding SSA
owner supplies reaching values and PHI transactions. `GenericLoopV1CarrierState`
and `CorePhiInfo` may transport an already-issued relation, but they must not
allocate a competing binding identity or infer one from a name.

## Value-generation contract

For every accepted binding, preserve the relation already recorded in
`docs/reference/mir/loop-recipe-contract.md`:

```text
Preheader -> define the initial generation
Header    -> condition reads h_n
Body      -> reads h_n; a rebind defines s_n
Backedge  -> transfers s_n to the next Header
After     -> false-edge Header value
Tail      -> canonical After value
```

The body value must not become the After value merely because it was created
later. The callable ledger's latest value must not override the role/edge
mapping. Missing or foreign owner/site evidence must stop before composer
mutation; unsealed PHI transactions must not escape the unpublished function.

## Acceptance

Use one valid source-backed graph without manual ledger pre-registration.
Cover zero, one, and multiple iterations, including a condition read and an
increment rebind. The positive path must observe the expected header/body/
backedge/After generations and keep the existing completion behavior.

Start each negative from that valid graph and mutate exactly one relation:

```text
foreign BindingRef or owner
stale generation / body-as-After edge
missing predecessor or wrong edge
unsealed or missing PHI publication
```

Each must return its named Binding SSA/PHI rejection before a new CFG/PHI or
other Builder effect. A generic nonzero result, a fixture that already fails
on liveness, or manual ledger injection is not evidence for this row.

## Task order and non-claims

This row precedes the existing
`MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-R0`. After both are closed, keep
the existing guard-cleanup row, then select the ordinary normalizer/physical
consumer completion. `Outside`, nested, legacy, fallback/retry, and the shared
non-callable JoinIR route remain unchanged.

The R7 compatibility-retirement family is parked independently at
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; this row does not reopen or delete
its shared LegacyCallV0 parser.

## Implementation checklist

```text
source schedule -> canonical BindingRef/role/site
               -> one BindingSsaBuilder/PhiTxn relation
               -> existing Loop recipe/composer transport
               -> verifier/lowerer and unpublished function completion
```

No new semantic receipt, name lookup, global cache, fallback, or public API is
allowed. Keep touched production Rust files below the 760-line design target
and the 800-line hard stop.
