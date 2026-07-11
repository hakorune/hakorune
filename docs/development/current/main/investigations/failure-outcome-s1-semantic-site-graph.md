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
owner_domain
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
review_status
pending_reason?
```

`site_id` is a semantic-site identity, not a source-location key. Its closed
grammar is exactly four dot-separated lower-snake-case segments:

```text
<layer>.<owner-domain>.<operation>.<outcome-branch>
```

The generator must validate the segment count and the declared vocabulary for
`layer`, `owner-domain`, `operation`, and `outcome-branch`; unknown symbols or
hand-written hierarchical extensions are rejected. Source locations belong
only to evidence occurrences:

Evidence occurrences retain:

```text
evidence_id
source_path
line
token
evidence_kind
```

Pending rows that are waiting for the `missing_argument_zero` call/default-
argument owner review must publish a manifest count. The manifest also keeps
the previous/baseline count so the guard can require a non-increasing count;
an unchanged count after a classification batch is a consultation stop, not
an untracked indefinite pending state.

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
semantic site id with segment count other than 4 -> reject
semantic site id with unknown layer/vocabulary symbol -> reject
evidence location key used as semantic site id -> reject
compatibility_only site without profile -> reject
missing_argument_zero pending count missing -> reject
missing_argument_zero pending count increased from previous manifest -> reject
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
all semantic site ids match the closed four-segment grammar
all semantic sites have one owner and one target carrier
all evidence occurrences are retained and dispositioned
compatibility_only sites have an explicit profile
missing_argument_zero pending count is manifest-visible and non-increasing
backend projections reference a source semantic site
unresolved Void rows remain pending rather than heuristically classified
existing fast gates remain green
```

## Implementation Artifact

The deterministic graph generator is
`tools/docs/failure_outcome_semantic_site_graph.py`; its manifest is
`tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json`. The
current inventory contains 602 evidence occurrences and 54 semantic sites,
including 10 explicit boundary projections. `missing_argument_zero` has a
manifest-visible pending count of 0 after excluding unrelated Option/Result
payload fixtures. These are inventory facts only; semantic
activation remains zero.

## Next Stop

After the graph and guard are green, continue S2-S5 of 3505. Any remaining
owner ambiguity becomes a focused consultation; no runtime migration starts
from this task.

The post-foundation migration is taskized separately as the queued parent
`3506-LANGV1-FAILURE-OUTCOME-SEMANTIC-UNIFICATION-001`. It cannot activate
until 3505 S0-S5 are green and
`LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001` selects one first
semantic boundary. See
`docs/development/current/main/phases/phase-296x/3506-LANGV1-FAILURE-OUTCOME-SEMANTIC-UNIFICATION-001.md`.
