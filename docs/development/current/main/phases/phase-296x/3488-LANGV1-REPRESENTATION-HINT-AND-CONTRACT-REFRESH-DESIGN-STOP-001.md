# 3488 - LANGV1-REPRESENTATION-HINT-AND-CONTRACT-REFRESH-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change parser, MIR, verifier, backend,
or runtime behavior until the owner and ordering decisions below are accepted.

Decision: pending.

## Objective

Choose one semantic-refresh owner for all active `:T` contract carriers and
one directional boundary from source type contracts to derived representation
facts. Prevent direct-verifier and backend entry points from observing a
declared annotation without its required carrier, while keeping storage,
layout, planner, and Rune policy outside source `:T` authority.

## Trigger Evidence

```text
completed semantic owners:
  exact-numeric Box field
  parameter entry
  return exit
  local init/reassignment

observed ordering debt:
  direct MIR verification of while_expected.hako reaches count_to/1 with
  declared_return_type_name = i64 but no return_exit_contract carrier

representation debt:
  FunctionSignature / MirType / storage metadata / planner hints still need a
  complete authority audit after narrow contract activation
```

Implementation behavior is evidence, not semantic authority. This card does
not reopen the accepted meaning that canonical `x: T` is an eventual gradual
semantic contract.

## Decisions Required

### A. Contract refresh owner and order

Choose one owner that deterministically rebuilds and validates Box-field,
parameter-entry, return-exit, and local-slot carriers before every verifier,
MIR JSON, runtime, and backend capability boundary.

Questions:

1. Is `semantic_refresh` the sole owner, with all public consumers required to
   enter through a refresh-and-validate facade?
2. Which epochs/fingerprints prove a carrier is fresh after CFG/SSA rewrites?
3. Should missing active carriers always fail, or may the owner rebuild them
   from source-owned declaration metadata at that boundary?
4. How are direct unit-test and tool entry points prevented from bypassing the
   same owner without fixture-specific setup?

### B. Representation derivation boundary

Choose a one-way projection:

```text
source :T -> TypeContractSpec -> semantic carrier
semantic carrier + verifier facts -> MirType / storage / layout facts
explicit plan input -> PlanHint / RuneHint
```

Questions:

1. Which current consumers use source annotations directly as representation,
   storage, planner, or Rune authority?
2. Which projections remain valid derived facts, and which must be retired?
3. What fail-fast guard prevents MirType or exact-numeric facts from becoming
   semantic contract proof?
4. How should normative `types.md` semantics be separated from mutable
   implementation-status inventory without duplicating truth?

## Source Authority

```text
language law:
  docs/reference/language/semantic-contract-charter.md

type contract:
  docs/reference/language/types.md
  accepted 3479 guarantee vocabulary and owner matrix

active carrier owners:
  FunctionEntryContractOwner
  FunctionReturnContractOwner
  BoxFieldWriteContractOwner
  LocalSlotContractOwner

implementation evidence:
  semantic_refresh entry points
  verifier/backend/JSON call graph
  direct typed-return carrier-missing reproduction
```

## Non-Authority

```text
parser acceptance
source annotation text alone
FunctionSignature / MirType alone
exact-numeric facts alone
storage or backend layout metadata
planner/Rune hints
green VM execution
fixture-specific carrier construction
source path or use count
```

## Required Fail-Fast Boundary

```text
active contract without fresh carrier -> fail-fast
carrier/source declaration drift -> fail-fast
consumer bypasses refresh owner -> fail-fast
representation fact used as semantic proof -> fail-fast
source :T used directly as planner/Rune authority -> fail-fast
unsupported backend -> fail before effects
```

No lazy repair after execution starts, no VM/backend fallback, no by-name
carrier synthesis, and no environment-selected contract activation.

## Minimum Implementation Slice After Acceptance

One BoxShape card only:

1. Introduce or select one refresh-and-validate facade.
2. Route direct verifier, MIR JSON, VM, and backend-preflight entries through it.
3. Add freshness/drift guards for the four active exact-numeric carrier families.
4. Fix the typed-return direct-verifier reproduction structurally.
5. Produce a checked representation-consumer inventory and move mutable status
   out of normative type law without changing source semantics.
6. Add focused bypass, stale-carrier, and representation-as-proof negatives.

Do not activate a new type family, add backend lowering, widen proof elision,
or introduce a broad static type checker in that slice.

## Explicit Non-Claims

```text
contract_refresh_owner_decided = 0
representation_audit_complete = 0
types_status_ledger_split = 0
new_type_family_activation = 0
runtime_check_elision_widened = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Consultation Stop

Return with the selected owner, exact ordering, freshness schema, forbidden
bypasses, fixture matrix, and minimum implementation boundary. Do not edit code
from this card before that decision is accepted.
