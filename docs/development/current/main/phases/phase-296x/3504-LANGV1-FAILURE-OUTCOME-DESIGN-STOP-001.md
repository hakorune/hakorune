# 3504 - LANGV1-FAILURE-OUTCOME-DESIGN-STOP-001

## Status

Accepted design stop. Do not change grammar profiles, null/void runtime
representation, Option/Result APIs, catch behavior, local defaults, Weak
upgrade results, VM errors, cleanup, or backend lowering before a later
activation card.

Decision: accepted

Selected first slice:

```text
relation/spec + exhaustive inventory only
```

## Goal

Close one source-level relation among absence, recoverable failure, Fault, and
successful no-result computation without allowing current runtime
representation to become language authority.

Target vocabulary from the ordered workstream:

```text
Option::None = ordinary value absence
Result::Err  = recoverable failure returned as a value
Fault        = violated language/runtime contract
Normal(Unit) = successful computation with no useful result
null         = Compat2025 migration surface only
```

## Current Contradictions

```text
types.md:
  null and void are distinct source spellings but one runtime Void value
  weak_to_strong failure returns null/void

statements.md:
  local x desugars to local x = null

enum surface:
  Option::None is explicitly not null
  Option::Some(null|void) is forbidden

grammar registry:
  literal_null is currently Canonical in both profiles

runtime:
  VMError variants, Throw/Catch, TaskFailed/TaskCancelled, VoidBox/NullBox,
  MissingBox, and foreign/provider failures do not yet project through one
  accepted Fault/Result/absence relation

semantic kernel:
  Outcome and cleanup precedence are accepted conceptually, but do not yet
  activate catchable Faults or a broad runtime Outcome carrier
```

These are evidence and migration debt, not authority to preserve the current
equivalences.

## Required Decisions

1. Is canonical `void` the sole Unit literal/value, and may Unit share a runtime
   representation with compatibility null while remaining semantically
   distinct?
2. Does canonical value absence require `Option::None` at every language API,
   including `WeakRef.weak_to_strong()`, or is a narrow nullable carrier still
   permitted?
3. What replaces unannotated `local x` default initialization: Unit, an
   explicit uninitialized state, rejection, or another closed rule?
4. What is the closed Fault taxonomy and stable tag owner? Classify type,
   bounds, division, missing member, backend capability, lifecycle, task, FFI,
   and user throw behavior.
5. Which failures are catchable in Canonical? Decide whether `catch` handles a
   finite FFI/compat set, user-declared Result only, or no Faults.
6. Confirm that Fault never converts implicitly to `Result::Err`, absence, Unit,
   zero, or backend fallback.
7. How does the accepted cleanup precedence map to body Fault, cleanup Fault,
   Return, Break, Continue, and top-level diagnostics?
8. What is the explicit foreign-null carrier and at which ingress/egress owner
   is it converted?
9. What migration order removes Canonical `null` without global text
   replacement, and when may `literal_null` move to Compat2025-only?
10. Which single first implementation slice proves the relation without
    opening a broad exception system or rewriting every API at once?

## Inventory Required Before Acceptance

Classify every live null-like site into one row:

```text
optional_absence
successful_no_result
recoverable_failure
contract_fault
parser_or_builder_sentinel
foreign_null
compatibility_only
```

Inventory must separately cover:

```text
source null literals
local declarations without initializer
WeakRef upgrade failure
NullBox / VoidBox / MissingBox
dropped WeakRef observations
Option / Result constructors and matches
Throw / Catch / cleanup
VMError and provider/FFI errors
backend zero/null/missing-result synthesis
```

Counts and current runtime equality are migration evidence only.

## Candidate First Slices

```text
A. Relation/spec + exhaustive inventory only.
   Accept the closed vocabulary and classify all live sites before behavior
   changes. No grammar/profile activation yet.

B. Unit/void boundary first.
   Introduce one Unit semantic carrier and migrate no-result returns while
   leaving absence/null migration inactive.

C. Weak upgrade/Option boundary first.
   Change weak_to_strong failure to Option::None through one typed carrier,
   with unsupported backends fail-fast.

D. Canonical null migration first.
   Migrate source/API sites by classification, then change the registry row.
   This is the broadest candidate and requires complete source evidence.
```

Recommendation before consultation: accept A as the design/inventory slice,
then choose one narrow semantic activation from measured migration evidence.
Do not move `literal_null` to Compat2025 before Canonical source and API users
are removed.

## Accepted Relation

```text
Unit:
  canonical source spelling = void

Option::None:
  canonical ordinary value absence

Result::Err:
  canonical recoverable failure value

Fault:
  violated contract/control outcome, not a language value
  canonical catchable Fault set = empty

UninitializedSlot:
  slot-only local state, not Unit/None/null

Weak upgrade:
  Option::Some(BoxRef) | Option::None

ForeignNull:
  boundary-only FFI carrier

CompatNull:
  Compat2025-only migration carrier

canonical null:
  rejected only after migration inventory and API migration close
```

Fault taxonomy is domain/code based (`contract`, `bounds`,
`arithmetic`, `member`, `capability`, `lifecycle`,
`task`, `foreign`, `resource`, `control`,
`internal`). Fault never converts implicitly to Result::Err,
Option::None, Unit, zero, or backend fallback. Cleanup preserves
semantic-kernel precedence: cleanup Fault becomes primary, later faults are
suppressed, and Return/Break/Continue from cleanup become control Faults.

## Accepted Ownership Boundaries

```text
Unit / no-result:
  UnitOutcomeOwner

Option / Result:
  OptionValueOwner / ResultValueOwner

local x without initializer:
  LocalInitializationStateOwner

Weak upgrade:
  WeakUpgradeOutcomeOwner

Fault construction:
  FaultRegistryOwner plus operation owner

cleanup / top-level:
  CleanupOutcomeOwner / ProgramOutcomeOwner

foreign null/status:
  FfiBoundaryContractOwner
```

The first slice records these relations and classifies every live null-like
site. It does not activate any new runtime carrier or profile rule.

## Next Card

```text
3505 - LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
```

## Stable Boundaries

```text
parser/runtime behavior is evidence, not authority
Fault before effects where statically knowable
cleanup always follows accepted semantic-kernel precedence
no implicit Fault -> Result conversion
no Canonical -> Compat2025 retry
no VM/backend fallback
unsupported backend rejects before effects
```

## Minimum Accepted Packet

The consultation answer must provide:

```text
closed semantic vocabulary
site/operation ownership matrix
Fault taxonomy and catchability set
Unit and absence representation law
local-default and weak-upgrade policies
foreign-null boundary
cleanup/top-level propagation law
null migration/profile sequence
stable tags
fixture matrix
one minimum implementation card
claims and non-claims
```

## Non-Claims

```text
failure_outcome_design_accepted = 1
canonical_null_surface = 1
compat2025_null_only = 0
unit_runtime_carrier_activated = 0
weak_upgrade_option_activation = 0
local_default_policy_changed = 0
catchable_fault_set_closed = 1
ffi_null_contract_activated = 0
broad_exception_system = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```
