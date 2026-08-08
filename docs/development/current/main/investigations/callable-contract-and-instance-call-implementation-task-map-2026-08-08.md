---
Status: accepted task map; execution is frontend inventory R2
Date: 2026-08-08
Decision: current Hakorune authority wins over the external type-profile proposal
Reference: `docs/reference/language/callable-contracts.md`
---

# Callable contract and instance-call implementation task map

## Final design

```text
ordered duplicate-free Box method source
  -> resolver declaration capability
  -> declared CallableContract(query)
  -> reusable instance target
  -> exact source-bound call relation
  -> Recipe CallSlot
  -> Verify / Lower

method body
  -> semantic conformance
  -> module publication gate
```

The external review's architecture is accepted with these mandatory Hakorune
corrections:

```text
accepted source: CallableContract(query)
rejected source: CallableContract(exact_trivial_i64)

signature owns: arity and semantic parameter/result types
query owns: exact receiver direct-state reads and bounded no-effect behavior
Pure owns: no receiver/heap/global read
physical verifier owns: MIR representation and target ABI
```

Declaration and conformance are separate. An annotation declares an
obligation; it does not prove the body. Production publication requires both.

## Single authority table

| Meaning | Sole owner | Forbidden reconstruction |
| --- | --- | --- |
| source order, duplicate, member site | parser-owned `BoxMethodInventoryV1` source seal | `HashMap`, name sort, resolver rescan |
| nominal Box/method identity and signature | resolver declaration inventory | method/Box name, Builder catalog |
| declared query behavior and receiver `Handle` demand | canonical callable-contract issuer | body inference, `EffectMask`, `FunctionSignature` |
| implementation compliance | body conformance verifier | annotation presence alone |
| reusable declaration target | resolver target catalog | call-site text, runtime registry |
| caller/receiver/arguments/result relation | source-bound call relation | Recipe or Lower re-resolution |
| logical call operation | verified Recipe `CallSlot` | provider/runtime fallback |

`NoSafeSlice` means a required issuer is not implemented. It is not a source
disposition. After an issuer exists, disposition is:

```text
Rejected > Unresolved > Declined > Candidate
```

## Ordered finite ladder

### A. Frontend source authority

1. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R1` — closed
   - replace the AST `HashMap` field;
   - compile compatibility consumers through explicit `CompatibilityOnly`;
   - no source-authority claim.
2. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R2`
   - shared pending/direct issuance substrate;
   - interface/static parser issuance and duplicate/site proof;
   - build_cfg transforms declarations without losing metadata;
   - ordinary source authority remains zero.
3. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R3`
   - ordinary Box sole-inventory cutover;
   - selected build-gate, generated property, and delegate atomic batches;
   - generated rows stay non-source provenance.
4. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R4`
   - ordered JSON v2;
   - legacy JSON v1 imports only `CompatibilityOnly`.
5. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R5`
   - migrate remaining Builder compatibility projections;
   - delete old AST-map helpers when callers reach zero.

### B. Hako parser parity

6. `HAKO-PARSER-BOX-DECLARATION-CARRIER-D0/H1-H6`
   - split the current oversized parser facade before adding responsibility;
   - issue the same ordered declaration carrier while parsing once;
   - carry `CallableContract(query)`;
   - normalized Rust/Hako parity is test evidence, never semantic transport.

### C. Resolver declaration and declared contract

7. `RESOLVER-INSTANCE-METHOD-DECLARATION-RECEIPT-I0`
   - exact nominal Box/method identity, signature, catalog brand;
   - no behavioral contract or target.
8. `RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0`
   - one public issuer and one aggregate;
   - bounded positive fixture `length(): i64`;
   - neither `length` nor `i64` is semantic authority.

### D. Target and source-bound logical call

9. `LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0`
   - catalog-owned reusable opaque target reference;
   - existing FreeStatic index unchanged;
   - no call-site or Recipe facts.
10. `LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0/I0`
    - exact caller/receiver/argument/result sites and exact target;
    - caller-zero logical product;
    - no Builder or physical call.
11. `LOOP-RECIPE-CALLSLOT-COSEAL-I0`
    - deterministic source relation to existing typed Recipe `CallSlot`;
    - full source evidence and verifier coverage;
    - no provider selection or fallback.

### E. Body conformance and activation

12. `CALLABLE-CONTRACT-CONFORMANCE-D0/I0`
    - verify direct receiver-read footprint, no writes/Home escape/allocation/
      IO/FFI/failure escape/suspension/non-local control;
    - never infer a replacement public contract from the body.
13. `CALLABLE-CONTRACT-MODULE-PUBLICATION-GATE-I0`
    - declared contract plus matching body conformance required;
    - incomplete or rejected conformance prevents publication.
14. Named production activation
    - one selected caller switches to the verified route;
    - delete that caller's old lookup/retry/fallback in the same commit.

## Legacy retirement ledger

| Legacy surface | Keep through | Delete when |
| --- | --- | --- |
| AST compatibility method map | R1-R4 | R5 caller zero and JSON v2 parity |
| name-sorted compatibility iteration | legacy JSON/Builder consumers | those named consumers migrate |
| Builder same-module name/arity catalog | current production compatibility | resolver target is selected and its caller cut over |
| FreeStatic resolved index | indefinitely for FreeStatic only | never reused for instance methods |
| `Contract(pure|readonly)` metadata | existing metadata lane | a separate Decision explicitly migrates it |
| test-only normalized Rust/Hako comparison | parity tests | may remain as evidence; never runtime authority |

No compatibility row may be promoted to `ExplicitSource`; no resolver may
recover source order/site from a compatibility map.

## Mandatory tests and documentation

Every implementation row closes in one implementation-coupled commit with:

```text
implementation
focused positive/negative tests
owner module README
affected docs/reference receipt
active card closeout and next pointer
all touched source files < 800 lines
```

Required test families are:

```text
frontend:
  order, direct/selected duplicate, exact sites, provenance, JSON v1/v2

resolver contract:
  Candidate/Declined/Unresolved/Rejected, foreign brand, conflict,
  partial aggregate unconstructible

target/relation:
  reusable target, exact receiver/arity/types, foreign caller/site,
  no FreeStatic/name fallback

conformance/publication:
  direct receiver read accepted for query, writes/effects/control rejected,
  declaration without conformance cannot publish
```

Reference updates are not deferred to a final cleanup row. The exact landed
surface updates `docs/reference/language/callable-contracts.md` and its owning
module README in the same commit. Future rows remain described as future until
their issuer and negative matrix land.

## Global stop lines

```text
no exact_trivial_i64 source profile
no receiver-read Pure widening
no source order/site recovery from HashMap or names
no public partial semantic-receipt constructors
no declaration annotation treated as body proof
no target from method/Box name
no instance fallback to FreeStatic
no Recipe before the exact source-bound call relation
no production publication before conformance
no Builder/provider/runtime retry or fallback
```
