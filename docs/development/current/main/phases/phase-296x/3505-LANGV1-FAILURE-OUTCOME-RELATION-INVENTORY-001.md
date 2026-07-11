# 3505 - LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001

## Status

Active implementation task. Repository artifact lifecycle C2 classification is
resolved for the deterministic rows; 77 consultation rows remain
warning-unregistered and C3 movement is deferred. This card changes no parser, grammar
profile, MIR operation, runtime value carrier, VM behavior, cleanup behavior,
or backend lowering.

Current progress:

```text
S0_relation_ssot = complete
S1_source_evidence_queue = complete
S1_semantic_classification = complete
S2_runtime_provider_inventory = complete
S3_control_flow_inventory = complete
S4_exhaustiveness_checker = complete
S5_conflict_ledger_closeout = implementation_in_progress
```

The accepted owner-boundary consultation is recorded in
`docs/development/current/main/investigations/failure-outcome-s1-vmvalue-void-owner-design-stop.md`.
The accepted S2 source-owner decision opened
`3507-LANGV1-FAILURE-OUTCOME-S2-PROJECTION-BINDING-001` as the projection
binding implementation slice; that slice is complete.
The S2 runtime/provider inventory, projection binding, S3 control-flow
evidence inventory, and S4 exhaustiveness checker are complete. The next
active slice is
`docs/development/current/main/phases/phase-296x/3510-LANGV1-FAILURE-OUTCOME-S5-CONFLICT-LEDGER-CLOSEOUT-001.md`.
S1 graph artifacts are the identity source; S2 changes inventory
tooling/manifests only and runtime activation remains zero.

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

## Execution Taskboard

The first slice is split into behavior-preserving documentation and tooling
boxes. No box below may change a parser profile, runtime carrier, VM result, or
backend lowering.

```text
S0 relation SSOT:
  create one normative relation document and record precedence/ownership

S1 source inventory:
  add one deterministic machine-readable inventory generator and manifest
  cover source null/Void/Option/Result/Catch spellings and profile metadata

S2 runtime/provider inventory:
  cover VMValue/ConstValue conversions, Weak upgrade, null-like boxes,
  extern/provider status, zero/missing-result synthesis, and FFI boundaries

S3 control-flow inventory:
  cover local defaults, Return/Unit, Fault, cleanup precedence, catchability,
  and top-level outcome normalization without changing behavior

S4 exhaustiveness guard:
  reject duplicate site_id, missing owner/class, unknown class, implicit
  conversion, Unit/absence conflation, missing foreign-null policy, invalid
  four-segment site_id grammar, missing compatibility profile, and increasing
  missing_argument_zero pending count

S5 conflict ledger and closeout:
  make the known contradictions queryable, run all gates, and prepare the
  next design stop; activation remains disabled
```

### Worker Inventory Baseline

The initial read-only scan is evidence for queue construction, not semantic
classification. Current counts over `src` and `docs/reference` are:

```text
VMValue::Void = 144 hits / 46 files
ConstValue::Null = 50 hits / 43 files
ConstValue::Void = 125 hits / 75 files
weak_to_strong = 29 hits / 9 files
MissingBox = 22 hits / 11 files
postfix_catch = 22 hits / 12 files
env.get = 130 hits / 49 files
env.file.read = 8 hits / 6 files
env.now_ms = 22 hits / 8 files
Option::None = 22 hits / 7 files
Result::Err = 19 hits / 7 files
```

The generator must retain source location and evidence kind so these counts
cannot be mistaken for unique semantic sites. Tests, compatibility adapters,
and historical docs require explicit layer/profile rows rather than blind
token deduplication.

### Stable Inventory Schema

Evidence occurrences and semantic sites have separate identities. The current
line/token queue is evidence only; S1 must not promote its location-shaped key
to semantic ownership.

Evidence occurrence row:

```text
evidence_id
source_path
line
token
evidence_kind
```

Semantic site row:

```text
site_id
layer
owner_domain
operation
outcome_branch
current_carrier
semantic_class
target_carrier
owner
profile
migration_action
backend_policy
evidence_refs
projects_site?
```

`site_id` is exactly four dot-separated lower-snake-case segments:

```text
<layer>.<owner-domain>.<operation>.<outcome-branch>
```

The generator owns the declared vocabulary for all four segments and rejects
unknown symbols, wrong segment counts, and source-location-shaped IDs.
`semantic_class`, `owner`, `target_carrier`, and `backend_policy` are closed
enums. Free-form text is diagnostic evidence only. A `compatibility_only` site
must have an explicit profile.

Pending `missing_argument_zero` rows must be counted in the manifest and
compared with the previous/baseline count. The guard rejects an increase; an
unchanged count after a classification batch becomes a focused consultation
stop rather than an indefinite pending queue.

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
semantic site IDs are closed four-segment identities
compatibility_only rows always name an explicit profile
missing_argument_zero pending count is visible and non-increasing
known conflict ledger is complete
Canonical null and catch registry rows remain unchanged
all existing fast gates remain green
```

## Acceptance Commands

```text
python3 tools/docs/failure_outcome_site_inventory.py --check --strict
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
python3 -m unittest tools/docs/test_failure_outcome_semantic_site_graph.py
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```

The first implementation commit may add only the relation document,
inventory generator/manifest, checker, and conflict ledger. It must not alter
`src/parser`, `src/backend`, `src/mir`, `VMValue`, `ConstValue`, or grammar
profile behavior.

## S0 Closeout

`docs/reference/language/failure-outcome-relations.md` is now the normative
relation document for this card. It records the accepted 3504 vocabulary,
forbidden implicit conversions, cleanup/control outcomes, foreign boundary
policy, and activation flags without changing existing behavior.

## S1 Evidence Queue Closeout

`tools/docs/failure_outcome_site_inventory.py` and
`tools/checks/manifests/failure_outcome_site_inventory_v0.json` now provide a
deterministic 602-row evidence queue over `src` and `docs/reference`. The rows
retain source location and evidence kind, but semantic fields remain pending;
this is intentional and does not claim exhaustive semantic classification.

## S1 Semantic Site Graph Progress

`tools/docs/failure_outcome_semantic_site_graph.py` now projects the evidence
queue into the line-independent graph
`tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json`:

```text
evidence_occurrences = 602
semantic_sites = 54
boundary_projection_sites = 10
pending_sites = 52
missing_argument_zero_pending = 0
semantic_activation = 0
```

The graph retains unresolved owner/class/target fields as pending. It does
not claim semantic migration or runtime carrier activation.

The first classification batches cover 55 explicit rows:

```text
optional_absence = 31
recoverable_failure = 24
pending_rows = 547
runtime_direct_provider_rows = 5
parser_or_test_inference = 0
```

Implementation evidence and test expectations remain pending because they do
not independently establish semantic ownership.

## Next Task

```text
LANGV1-FAILURE-OUTCOME-S5-CONFLICT-LEDGER-CLOSEOUT-001
```

Make the known Failure/Outcome contradictions queryable and run the complete
inventory closeout gates. The slice may add only docs/report tooling; it may
not change parser/runtime behavior. Any semantic owner ambiguity remains a
focused design consultation stop.

## Post-Foundation Queued Follow-Up

```text
3506 - LANGV1-FAILURE-OUTCOME-SEMANTIC-UNIFICATION-001
```

This is a queued parent task, not the current blocker. It may become active
only after 3505 S0-S5 are green and
`LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001` accepts the first
activated boundary. Its implementation must be split into one code-facing
semantic slice per owner; it must not become a global `void` replacement.

See the task card:
`docs/development/current/main/phases/phase-296x/3506-LANGV1-FAILURE-OUTCOME-SEMANTIC-UNIFICATION-001.md`.

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

Implementation claims remain zero until the corresponding artifact and gate
exist:

```text
failure_outcome_relation_spec_implemented = 1
failure_outcome_evidence_queue_implemented = 1
failure_outcome_reference_classification_implemented = 1
failure_outcome_site_inventory_implemented = 1
failure_outcome_semantic_classification_complete = 0
failure_outcome_exhaustiveness_checker_implemented = 0
failure_outcome_conflict_ledger_complete = 0
```

## Next Design Stop

After S0-S5 are green, stop before any semantic migration at:

```text
LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001
```

The queued 3506 task remains inactive until this stop names the first
semantic owner, carrier, fixture boundary, and unsupported-backend policy.

That stop must decide the first activated relation boundary and its backend
fail-fast policy. It must not be opened by the inventory card itself.

## Current Design Stop

```text
LANGV1-FAILURE-OUTCOME-S1-VMVALUE-VOID-OWNER-DESIGN-STOP
```

The `VMValue::Void` cluster is intentionally held pending until site
granularity and owner boundaries are decided. See the focused consultation
document in `docs/development/current/main/investigations/`.
