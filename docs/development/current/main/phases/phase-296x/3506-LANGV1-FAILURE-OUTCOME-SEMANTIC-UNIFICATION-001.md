---
Status: Queued follow-up
Date: 2026-07-12
Decision: pending activation design stop
---

# 3506 - LANGV1-FAILURE-OUTCOME-SEMANTIC-UNIFICATION-001

## Status

This is the post-foundation parent task for historical `void`/`null`-like
site migration. It is queued behind 3505 S0-S5 and
`LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001`; it is not the current
blocker and does not change `CURRENT_STATE.toml` while queued.

The task is intentionally a sequence of narrow implementation slices. The
parent does not authorize a repository-wide text replacement or a single
carrier-wide rewrite.

## Objective

Move each accepted semantic operation/outcome site from its historical carrier
to the canonical relation, while preserving evidence and rejecting unresolved
ownership. A current carrier such as `VMValue::Void` may remain physically
shared only when the semantic site, owner, and boundary contract are explicit.

The target is semantic convergence, not “all void becomes one type”:

```text
successful no-result       -> Normal(Unit)
ordinary value absence    -> Option::None
recoverable failure       -> Result::Err
contract/control failure  -> Fault
foreign nullable boundary -> ForeignNull
compatibility residue     -> explicit Compat2025 carrier/profile
backend/bridge projection -> projects_site(source semantic site)
```

## Preconditions

```text
3505 S0-S5 = green
semantic_sites have stable operation/outcome identity
evidence_refs and dispositions are exhaustive
selected site owner/target ambiguity = 0
LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001 = accepted
first activated boundary and backend policy = named
```

No implementation slice starts from source counts or a token-wide inference.
The selected slice must have an accepted owner, target carrier, migration
action, fixture boundary, and fail-fast policy.

## Ordered Slice Queue

The queue is a parent-level order, not permission to activate all slices at
once. Each slice gets its own implementation delta and gate.

```text
U1 Unit/no-result:
   migrate selected successful no-result boundaries to Normal(Unit);
   keep write_void as evidence/projection helper only

U2 ordinary absence:
   migrate selected optional/provider/Weak absence sites to Option::None;
   never infer absence from a shared Void carrier

U3 recoverable failure:
   migrate selected provider/file/FFI recoverable failures to Result::Err;
   keep failure and absence contracts distinct

U4 contract Fault:
   migrate selected undefined-register, missing-result, and unsupported
   capability branches to the accepted Fault owner; reject before effects

U5 foreign/backend boundary:
   make ForeignNull and zero/null/missing-result projections explicit;
   every projection references projects_site and cannot invent meaning

U6 compatibility retirement:
   isolate compatibility equality/boxing, NullBox/VoidBox/MissingBox, and
   remaining Canonical null users under Compat2025 or an explicit rejection;
   change literal_null profile only after source/API migration is complete
```

If a slice reveals a second owner or a different semantic outcome, stop and
return it to a focused consultation. Do not widen the active slice to absorb
the ambiguity.

## Structural Rules

```text
one semantic slice = one owner boundary = one acceptance packet
file/token is evidence, never semantic ownership
VMValue::Void/ConstValue::Void remain current-carrier evidence until replaced
write_void is never a semantic owner
no global replacement of void/null text
no implicit Fault -> Result/Option/Unit conversion
no provider-missing -> Unit fallback
no undefined-register -> absence fallback
no zero/null numeric equivalence as owner proof
unsupported backend fails before user-visible effects
VM validates the semantic-reference subset; EXE/AOT is the product proof
```

## Acceptance

```text
every activated site has one owner and one target carrier
every activated site retains evidence_refs and a disposition
selected semantic class has positive/negative fixtures
Unit, Option::None, Result::Err, Fault, ForeignNull, and Compat2025 remain
  observably distinct at their declared boundaries
backend projections reference a source semantic site
unselected or ambiguous sites remain pending
canonical null is not removed or reprofiled before source/API migration close
unsupported consumers fail fast rather than using VM fallback
inventory, semantic-site guard, relevant smoke, and dev_gate quick are green
```

## Explicit Non-Claims While Queued

```text
semantic_migration_activated = 0
runtime_behavior_changed = 0
VMValue_change = 0
ConstValue_change = 0
canonical_null_migration = 0
Weak_upgrade_behavior_change = 0
backend_lowering_change = 0
global_void_replacement = 0
selfhost_claim = 0
```

## Activation Boundary

The activation design stop must select exactly one first slice and record its
source authority, semantic owner, target carrier, fixture matrix, VM/EXE
coverage, and unsupported-backend behavior. Until then, this card is only a
durable task-order pointer; 3505 remains the active workstream.
