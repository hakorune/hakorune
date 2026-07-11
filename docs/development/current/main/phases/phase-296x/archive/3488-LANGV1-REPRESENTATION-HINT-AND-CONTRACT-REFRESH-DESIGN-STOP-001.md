# 3488 - LANGV1-REPRESENTATION-HINT-AND-CONTRACT-REFRESH-DESIGN-STOP-001

## Status

Complete design consultation. Parser, MIR, verifier, backend, and runtime
behavior remain unchanged by this card; implementation is owned by 3489.

Decision: accepted.

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

## Accepted Decisions

### A. Contract refresh owner and order

`semantic_refresh` is the sole rebuild/validation owner. Verifier, MIR JSON,
VM, backend preflight, and direct tool/test entry points must use one
`refresh_and_validate_for_boundary` facade.

The owner may deterministically rebuild carriers from source-owned declaration
metadata. After refresh, every active exact-numeric contract must have a fresh,
complete carrier or fail-fast. No consumer may synthesize its own carrier.

Ordering is fixed:

```text
source-owned declaration metadata
-> semantic_refresh rebuilds all active carrier families
-> completeness/freshness/drift validation
-> semantic consumer
-> derived representation projection
-> export, execution, or backend lowering
```

The first implementation uses structural freshness: deterministic rebuilt
carrier equality plus the current MIR/CFG/SSA/BindingId-derived inventories.
It must not introduce a parallel epoch allocator merely to satisfy this card.
If durable epochs are required by an existing rewrite boundary, they remain
owned by that boundary and are projected into the validation result.

### B. Representation derivation boundary

The one-way projection is accepted:

```text
source :T -> TypeContractSpec -> semantic carrier
semantic carrier + verifier facts -> MirType / storage / layout facts
explicit plan input -> PlanHint / RuneHint
```

`FunctionSignature`, `MirType`, storage/layout metadata, and exact-numeric
facts may remain derived representation facts. They cannot prove a semantic
contract or rebuild a missing carrier. Plan/Rune hints require explicit plan
input and cannot be inferred directly from source `:T`.

Normative meaning remains in `docs/reference/language/types.md`. Mutable owner,
backend capability, and migration state move to one development status ledger
that references normative row/owner IDs rather than restating semantics.

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

## Selected Implementation Slice

One BoxShape card only:

Owned by
`3489-LANGV1-REPRESENTATION-HINT-AND-CONTRACT-REFRESH-OWNER-001`.

Do not activate a new type family, add backend lowering, widen proof elision,
or introduce a broad static type checker in that slice.

## Explicit Non-Claims

```text
contract_refresh_owner_decided = semantic_refresh
refresh_and_validate_facade_required = 1
representation_derivation_boundary_decided = 1
representation_fact_as_semantic_proof_forbidden = 1
source_type_to_plan_rune_direct_authority_forbidden = 1
types_status_ledger_split_decided = 1
contract_refresh_owner_implemented = 0
representation_audit_complete = 0
types_status_ledger_split = 0
new_type_family_activation = 0
runtime_check_elision_widened = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Closeout

The owner, ordering, deterministic rebuild policy, representation boundary,
forbidden bypasses, and minimum implementation scope are accepted. Proceed to
3489 without opening a second refresh owner or a new type-family activation.
