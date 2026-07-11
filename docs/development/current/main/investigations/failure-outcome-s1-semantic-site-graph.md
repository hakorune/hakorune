---
Status: Active task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted
---

# LANGV1-FAILURE-OUTCOME-S1-SEMANTIC-SITE-GRAPH-001

## Decision

Inventory semantic meaning at the `semantic operation outcome branch` level.
Keep source line/token matches as evidence occurrences only.

```text
stable semantic site = owner + operation + observable outcome branch
file = never a semantic owner
VMValue::Void = current-carrier evidence only
write_void = evidence/projection helper only
```

No parser, grammar profile, MIR, `VMValue`, `ConstValue`, runtime, cleanup, or
backend behavior changes are allowed in this task.

## Scope

1. Preserve the current line/token scan as `evidence_occurrences`.
2. Add a stable `semantic_sites` collection keyed by operation and outcome
   branch, not by source line.
3. Add `evidence_refs` from each semantic site to its occurrences.
4. Split provider-missing, undefined-register, compatibility equality/boxing,
   Weak upgrade, and backend zero/null projections into distinct sites.
5. Add `projects_site` for backend and bridge projections.
6. Mark helper-only occurrences, including `write_void`, as evidence-only.
7. Keep all unresolved rows pending; do not apply file-wide or token-wide
   classifications.

## Closed Site Kinds

```text
operation_outcome
boundary_projection
compatibility_adapter
internal_sentinel
```

`internal_sentinel` is a site kind, not a new semantic class. The existing
3505 semantic classes remain closed:

```text
optional_absence
successful_no_result
recoverable_failure
contract_fault
parser_or_builder_sentinel
foreign_null
compatibility_only
```

## Required Schema

```text
site_id
site_kind
layer
operation
outcome_branch
semantic_class
target_carrier
owner
profile
migration_action
backend_policy
current_carrier
evidence_refs
projects_site?
```

Evidence occurrences retain:

```text
evidence_id
source_path
line
token
evidence_kind
```

## Guard Contract

```text
duplicate semantic site id -> reject
missing owner/class/target -> reject
file-wide default classification -> reject
token-wide default classification -> reject
operation site without evidence -> reject
evidence without disposition -> reject
projection without projects_site -> reject
projection semantic-class drift -> reject
provider-missing fallback classified as Unit -> reject
undefined-register fallback classified as absence -> reject
compatibility equality classified as Option::None -> reject
zero/null numeric equivalence used as owner proof -> reject
```

## Non-Claims

```text
Unit activation = 0
Option::None activation = 0
Result::Err activation = 0
Fault runtime activation = 0
Weak upgrade behavior change = 0
null migration = 0
local default change = 0
VMValue change = 0
ConstValue change = 0
backend lowering change = 0
fallback addition = 0
```

## Acceptance

```text
semantic site identity is line-independent
all semantic sites have one owner and one target carrier
all evidence occurrences are retained and dispositioned
backend projections reference a source semantic site
unresolved Void rows remain pending rather than heuristically classified
existing fast gates remain green
```

## Next Stop

After the graph and guard are green, continue S2-S5 of 3505. Any remaining
owner ambiguity becomes a focused consultation; no runtime migration starts
from this task.
