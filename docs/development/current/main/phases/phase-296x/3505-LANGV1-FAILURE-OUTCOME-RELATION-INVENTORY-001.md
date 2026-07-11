# 3505 - LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001

## Status

Active implementation task. Repository artifact lifecycle C2 classification is
resolved for the deterministic rows; 77 consultation rows remain
warning-unregistered and C3 movement is deferred. This card changes no parser, grammar
profile, MIR operation, runtime value carrier, VM behavior, cleanup behavior,
or backend lowering.

## Decision

`3504` accepted Candidate A:

```text
relation/spec + exhaustive inventory only
```

The first slice makes the relation machine-readable and exhaustive before any
semantic migration is activated.

## Scope

1. Add a normative relation/spec document for `Unit`, `Option`, `Result`,
   `Fault`, `UninitializedSlot`, `ForeignNull`, and `CompatNull`.
2. Add a machine-readable site inventory. Every row must include:

```text
site_id
layer
surface_or_symbol
current_carrier
semantic_class
target_carrier
owner
profile
migration_action
backend_policy
evidence
```

3. Classify every live site in these closed classes:

```text
optional_absence
successful_no_result
recoverable_failure
contract_fault
parser_or_builder_sentinel
foreign_null
compatibility_only
```

4. Cover source null literals, uninitialized locals, Weak upgrade, null-like
   boxes, Option/Result constructors, Throw/Catch, cleanup, VM/provider/FFI
   errors, and backend zero/null/missing-result synthesis.
5. Add an exhaustiveness checker:

```text
duplicate site -> reject
missing owner -> reject
unknown class -> reject
implicit conversion -> reject
Unit/absence conflation -> reject
foreign null policy missing -> reject
```

6. Add a conflict ledger for the known contradictions:

```text
null vs void
local default null
Weak upgrade -> Void
env missing/error -> Void
clock failure -> zero
MissingBox == Void compatibility
Canonical literal_null
Canonical postfix_catch vs catchable Fault set = 0
```

## Explicit Non-Scope

```text
parser behavior change = 0
grammar/profile change = 0
MIR operation change = 0
VMValue/runtime carrier change = 0
Weak upgrade change = 0
local default change = 0
cleanup behavior change = 0
backend lowering change = 0
runtime/backend fallback = 0
selfhost claim = 0
```

## Acceptance

```text
relation/spec has one owner and explicit precedence
all live null-like sites have exactly one class
all sites have exactly one owner and target carrier
duplicate/unclassified rows fail deterministically
known conflict ledger is complete
Canonical null and catch registry rows remain unchanged
all existing fast gates remain green
```

## Claims

```text
failure_outcome_relation_spec = 1
failure_outcome_site_inventory = 1
failure_outcome_exhaustiveness_checker = 1
failure_outcome_runtime_activation = 0
canonical_null_migration = 0
weak_upgrade_option_activation = 0
uninitialized_local_activation = 0
catch_profile_change = 0
```
