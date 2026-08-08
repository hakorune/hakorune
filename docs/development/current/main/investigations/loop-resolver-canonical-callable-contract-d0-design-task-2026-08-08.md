---
Status: accepted architecture — implementation parked behind frontend authority
Date: 2026-08-08
Decision: one declared semantic contract, separate body conformance, no physical ABI axis
Parent: `loop-resolver-instance-call-target-d0-design-task-2026-08-08.md`
Language: `language-typed-callable-profile-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0

## Decision brief

```text
non-Clone parser-sealed source method
+ exact nominal Box declaration
+ resolved semantic signature
+ typed CallableContractSyntaxV1::Query behavior
+ same-declaration VerifiedHomeAbi
  -> one VerifiedDeclaredInstanceMethodContractV1

method body
  -> separate VerifiedCallableContractConformanceV1
```

The declaration product is the only resolver semantic authority. It may be
cataloged before body verification so recursive and mutually recursive calls
can resolve, but module publication later requires the separate conformance
receipt.

## What the declaration contract owns

```text
exact source/catalog/compilation brand
exact Box and method declaration identity
exact nominal receiver type
ordered semantic parameter/result signature
declared query behavior
same-declaration VerifiedHomeAbi
```

The aggregate creates one relational truth: all of these inputs belong to the
same declaration. Private checks may validate individual axes, but callers
cannot obtain public partial receipts and pair them later.

The contract does not own:

```text
body conformance
call-site receiver/arguments/result site
Recipe or CallSlot keys
MIR FunctionSignature / MirType / EffectMask
physical scalar ABI or register class
function pointer, provider, runtime route
selection, fallback, or retry
```

Types and arity come from the method signature. `query` supplies only the
whole-call behavioral obligation. A later physical verifier projects semantic
values to a target representation; the projection is never read back into the
source contract.

## Query, Pure, and Home

The accepted language target is:

```hako
@rune CallableContract(query)
length(): i64
```

`query` allows reads through the exact receiver and forbids writes, Home
transfer/share/end/escape, allocation, IO/FFI, Fault/non-local failure,
suspension, and non-local control. An ordinary mutable receiver query is not
Pure: Pure forbids receiver/heap/global reads as well.

The receiver demand uses the existing ownership vocabulary `Handle`. There is
no `NoHomeHandle`, `HandleOnly`, or empty `VerifiedHomeAbi` shortcut. Query
does not issue this axis: one source-backed `VerifiedHomeAbi` alone owns the
receiver/parameter demands and result relation. The declared contract co-seals
that product and means only that this call boundary does not transfer, create,
share, end, or escape a Home.

Existing `Contract(pure|readonly)` remains its current metadata family and
`Profile(...)` remains a compatibility bundle. Neither is silently promoted
to the canonical whole-call contract.

## Canonical issuer

One public issuer consumes the exact frontend/resolver capabilities and
returns only the aggregate:

```text
DeclaredInstanceMethodContractIssuerV1::issue(
  ordered_source_method,
  nominal_box_declaration,
  resolved_semantic_signature,
  typed_callable_contract_source_row,
  verified_home_abi,
)
  -> VerifiedDeclaredInstanceMethodContractV1
```

No public `Verified*::new(...)`, arbitrary test constructor, MIR-to-semantic
reverse projection, name-based repair, or FreeStatic alias is allowed.

The current `VerifiedCallableIndexV1` remains FreeStatic-only. A later sibling
instance target catalog borrows/references this declaration product rather
than widening the FreeStatic key or copying the contract fields.

## Declaration versus conformance

```text
source declaration
  -> declared obligation

body Facts / Recipe
  -> conformance verifier
  -> VerifiedCallableContractConformanceV1
```

The body verifier checks the declared query envelope. It does not infer,
strengthen, or replace the public contract. Missing or failed conformance is a
module-publication error, not a reason to select a fallback target.

## Disposition

```text
issuer implementation absent                       -> NoSafeSlice
exact-family source without CallableContract(query) -> Declined
contract present but declaration/type unavailable   -> Unresolved
foreign/duplicate/conflicting source identity        -> Rejected
exact same-brand declared aggregate                  -> Candidate
Candidate declaration with violating body            -> conformance Rejected
```

Precedence is `Rejected > Unresolved > Declined > Candidate`.
`NoSafeSlice` is a development state, not a source disposition.

## Current blocker and ordered follow-up

The resolver cannot issue this contract from raw `BoxMethodInventoryV1`.
R1-R5 preserve selected order and descriptive provenance, but the all-row
inventory ordinal is not an explicit-method source site and no non-Clone
parser source seal exists yet. Resolver sorting, JSON, raw `ExplicitSource`,
or name lookup cannot repair or promote that boundary.

The following is a subsystem view only. The complete executable order,
including old-target retirement, publishable-catalog co-seal, physical ABI
projection, and same-slice reference updates, is owned only by
`callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`.

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0                  closed
  -> FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0/I0    current
  -> typed CallableContract syntax + Rust/Hako parity
  -> resolver semantic declaration/signature
  -> OWN-HOME-ABI0-S0/query
  -> RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
  -> RESOLVER-INSTANCE-CALL-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
  -> publishable catalog / physical ABI projection
  -> production activation only after complete conformance
```

Each implementation slice updates its exact owner README and
`docs/reference/**` receipt in the same commit. No implementation row may open
Recipe, Builder/MIR, provider/runtime, fallback, or publication authority
early.
