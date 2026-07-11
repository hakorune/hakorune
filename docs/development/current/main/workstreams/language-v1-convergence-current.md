---
Status: Active Taskboard
Date: 2026-07-10
Scope: Hakorune language v1 semantic convergence before selfhost migration.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/types.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
---

# Language v1 Convergence Current Taskboard

## Priority Decision

Language coherence is the prerequisite. Selfhost migration is a proof consumer
of the language contract, not authority to preserve ambiguous syntax or
semantics.

```text
language_contract_priority_over_selfhost_migration = 1
selfhost_migration_parked = 1
mirbuilder_3456_parked = 1
one_active_language_card = 1
```

Resume the parked MirBuilder route at 3456 only after this workstream closes or
an explicit user decision changes the priority.

## Target State

```text
one semantic kernel
one language contract
two independent parsers
one canonical normal form
zero implicit compatibility
```

## Language Constitution

Every row in this workstream must preserve these laws:

1. The same syntax carries the same guarantee at every use site.
2. Semantic meaning and storage/planning representation are separate.
3. Absence, recoverable failure, and language/runtime fault are separate.
4. Object identity and object lifetime are separate.
5. Sugar evaluates each source sub-expression exactly once and preserves
   source order.
6. Compatibility is available only through an explicit named profile.
7. Unsupported behavior fails before user-visible side effects.

## Management Contract

- This taskboard owns the complete order. Do not create a card for inventory,
  fixture refresh, rerun, or consultation notes.
- Open one numbered card only for the current macro row.
- Inside a macro row, use focused implementation commits and shared fixtures.
- A specification-changing row must record `Decision: accepted` before code.
- Rust and Hako parsers stay independently implemented. Shared generation may
  produce contracts, witnesses, fixtures, and documentation, not parser code.
- VM is the narrow semantic oracle; EXE/AOT must prove parity for each live
  semantic guarantee. Unsupported backends fail fast.
- No row advances from counts, source paths, or generated agreement alone.

## Authority Map

```text
language laws:
  docs/reference/language/semantic-contract-charter.md (accepted)

semantic kernel:
  docs/reference/language/semantic-kernel.md (accepted)

canonical grammar:
  docs/reference/language/grammar-contract.md (accepted)
  grammar registry -> generated EBNF/support views

parser implementation:
  Rust parser and Hako parser, independent

parser comparison:
  shared ParseWitness projection and golden corpus

type/failure/ownership semantics:
  topic SSOT plus semantic-kernel rules

runtime oracle:
  VM semantic-reference subset

product proof:
  EXE/AOT plus fail-fast backend preflight
```

The charter, semantic kernel, and grammar contract are current truth. Card 3478
closes the grammar registry, dual-parser recursive corpus, Canonical source
migration, and bounded differential composition. Type guarantee design is the
active macro row.

## Confirmed Starting Gaps

```text
compound assignment:
  Rust parser clones the lvalue AST into target and value expression

grammar:
  EBNF, Rust parser, Hako parser, topic docs, and v1 freeze disagree on
  guard/try/peek/from status

type annotations:
  metadata-only and checked boundaries share the same : T spelling

failure vocabulary:
  null and void are runtime-equivalent while Option::None is distinct
  2026-07-10 null migration baseline = 764 .hako files under lang/src + apps

ownership:
  ordinary shared strong fields and strong-owned cascade fini have no source
  distinction

capabilities:
  EffectSummary is metadata-only; CapabilityPlan starts verified=false;
  unknown declared uses are currently ignored
```

These are implementation/inventory evidence, not independent semantic
authority. Rerun mutable counts when their owning row starts.

## Ordered Macro Rows

### 0. LANGV1-CONSTITUTION-001

Status: landed as 3457.

Deliverables:

1. Create `docs/reference/language/semantic-contract-charter.md` containing
   the seven laws, normative document precedence, compatibility rule, and
   change protocol.
2. State that syntax/meaning changes require accepted Decision, positive and
   negative fixtures, and an unsupported-backend rule.
3. Point the language index, v1 freeze SSOT, and this taskboard at the charter.
4. Do not change parser, runtime, type, lifecycle, or backend behavior.

Acceptance:

```text
language_constitution_owner_count = 1
constitutional_law_count = 7
normative_precedence_defined = 1
compatibility_requires_explicit_profile = 1
runtime_behavior_changed = 0
```

Next: `LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001`.

### 1. LANGV1-SEMANTIC-KERNEL-001

Status: complete through 3459. Grammar decision consultation active as 3460.

Decision surface:

```text
Outcome =
  Normal(value_or_unit)
  Return(value_or_unit)
  Break
  Continue
  Fault(reason)

Place =
  Local(slot)
  Field(base_once, field)
  Index(base_once, index_once)
```

Deliverables:

1. Define source-order and exactly-once evaluation for expressions, calls,
   assignment, compound assignment, short circuit, guard, match, and cleanup.
2. Define cleanup outcome precedence for every incoming Outcome and cleanup
   Fault. Cleanup always runs where registered.
3. Define `NoFallthrough` as the guard-let else contract. Do not require a
   broad static `Never` type merely to express control termination.
4. Define the canonical normal form as semantic operations over evaluated
   values and Places, not AST text substitution.
5. Add side-effecting fixtures proving `array[nextIndex()] += makeValue()`
   evaluates receiver/index/RHS once in source order.
6. Replace the current compound-assignment AST clone lowering with one Place
   read-modify-write implementation. This is the first code-facing slice after
   the semantic basis.

Acceptance:

```text
semantic_kernel_owner_count = 1
outcome_variant_count = 5
place_evaluation_once = 1
source_evaluation_order_fixed = 1
sugar_observational_equivalence = 1
compound_assignment_ast_clone = 0
guard_else_no_fallthrough = 1
```

Closeout: `LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001` implements the
evaluated-Place slice. Grammar work advances through
`LANGV1-GRAMMAR-CONTRACT-SUBSTRATE-001`.

### 2. LANGV1-GRAMMAR-001

Status: complete through 3478. Contract basis accepted as 3460; substrate landed as 3461; Rust
grammar-profile owner accepted as 3462; profile plumbing plus statement-try
landed as 3463; Rust peek Compat2025 alias landed as 3464; Rust from
transport-only decision accepted as 3465; Rust from transport boundary landed
as 3466; Hako profile/witness ownership is accepted as 3467; adapter health
lands as 3468; explicit Hako profile facade plus statement-try lands as 3469;
Hako peek-to-Match alias lands as 3470. The ordered parser/MIR corrective card
3472 closes its correctness, compile-cost, fixture, and source-layout
prerequisites. Card 3471 accepts explicit Hako compatibility-transport
exclusion, delimiter-aware Match context, route-family convergence graph
ownership, and scoped test-config ownership. Card 3473 opens the first
code-facing slice.

Deliverables:

1. Define one grammar registry row schema:
   `row_id`, production, status, normalized shape, reject tag, Stage0/Stage1
   support, positive fixtures, and negative fixtures.
2. Classify every spelling as `canonical`, `compatibility_only`, `reserved`,
   or `rejected`.
3. Seed guard/guard-let, match/peek, postfix catch/cleanup/try, delegation/from,
   loops, weak, records, and current literal surfaces.
4. Generate EBNF tables, support matrix, keyword policy, and fixture index from
   the registry. Do not generate parser implementations.
5. Define `Canonical` default and explicit `Compat2025`; compatibility aliases
   normalize immediately to canonical shape.
6. Define a span-free `ParseWitness` and run one golden corpus through the
   independent Rust and Hako parsers under both profiles.
7. Missing, extra, accept/reject, reject-tag, and normalized-shape drift fail
   fast.

Acceptance:

```text
canonical_grammar_registry_count = 1
default_profile = Canonical
implicit_compatibility_count = 0
parser_implementation_count = 2
shared_parser_implementation_count = 0
parse_witness_conformance = 1
```

Post-3472 ordered follow-up queue:

```text
Q0 complete: 3471 accepts Decisions A-D
Q1 complete: 3473 Hako CompatibilityTransport explicit exclusion
Q2 complete: 3474 LANGV1-HAKO-MATCH-RECORD-DELIMITER-OWNER-001
  owner = delimiter-aware ExprContext / BlockDelimitedHeadStopsBeforeTopLevelBrace
Q3 complete: 3475 MIR-CONVERGENCE-ROUTE-FAMILY-GRAPH-SHADOW-001
  owner = route-family dependency graph
  changed-function worklist and local invalidation = subordinate mechanisms
Q4 complete: 3476 TEST-PROCESS-STATE-SCOPED-CONFIG-OWNER-001
  owner = scoped config injection
  subprocess execution = classification oracle only
Q5 audit complete in 3476:
  current 22-row corpus = green
  LANGV1-GRAMMAR-001 closeout = disproved by missing loops/weak/records/literals
Q5 decision complete in 3477:
  remaining surfaces and registry normalization contract accepted
Q5 implementation complete in 3478:
  one registry/corpus/adapter refactor-and-expansion series
  106/106 recursive corpus green
  bounded differential composition green
  Canonical rejected source occurrences = 0
```

Q2-Q4 may be reviewed in the same consultation packet, but their code deltas
must remain separate. Do not create inventory-only or rerun-only numbered
cards for this queue.

Next: `LANGV1-TYPE-GUARANTEE-001`.

### 3. LANGV1-TYPE-GUARANTEE-001

Status: design accepted as 3479; exact-numeric Box field first slice complete
as 3480; parameter-entry contract complete as 3482; return-exit design accepted
as 3483; return-exit implementation complete as 3484; local exact-numeric
owner/boundary consultation accepted as 3485; BindingId prerequisite complete
as 3486; exact-numeric local semantic implementation complete as 3487; the
representation/contract-refresh owner decision is accepted as 3488; the
single-owner BoxShape implementation is complete as 3489. Record, static table,
typed Array, and Weak field slices are complete through 3502; 3503 is the
active product-backend selection stop for Weak fields.

Target contract:

```text
annotation omitted -> Any
x: T -> gradual semantic contract T
representation/planner hint -> MIR facts, Plan, or Rune; never : T
```

Deliverables:

1. Close the type relation for primitives, exact integers, Box identity types,
   records, enums/generics, `Array<T>`, WeakRef, `Any`, and `void`/Unit.
2. Publish a guarantee matrix for local initialization/reassignment, parameter
   entry, return exit, Box field initialization/write, record fields,
   collection elements, Weak fields, FFI, and backend boundaries.
3. Choose one owner per check: callee entry for parameters, callee exit for
   returns, mutation boundary for fields/elements, and constructor/update
   boundary for records.
4. Elide a runtime check only from verifier-backed proof. Unknown proof retains
   the check; unsupported lowering fails before execution.
5. Inventory current metadata-only `: T` uses and migrate representation-only
   hints before enabling the contract by default.
6. Land boundaries in this order within the same macro row: locals and params,
   returns, Box fields, records, arrays/collections, FFI/backend preflight.

Current executable queue:

```text
T0 complete:
  3484 exact-numeric return-exit implementation

T1 complete:
  3485 local exact-numeric contract design stop
  inventory init/reassignment/PHI/loop/Any/proof invalidation
  BindingId/LocalSlotId + W1 + U1 + VM-only decision accepted

T2a complete:
  3486 behavior-preserving local BindingId owner completion
  LocalSlotId wrapper, one declaration API, CorePlan/snapshot parity
  LocalContractWrite / contract activation = 0

T2b complete:
  3487 LocalContractWrite exact-numeric semantic implementation
  init/reassignment + U1 + PHI/loop evidence + JSON/backend boundary
  no broad static checker; one LocalSlotContractOwner

T3 complete:
  3488 accepts semantic_refresh as the sole refresh-and-validate owner
  3489 routes verifier/JSON/VM/backend/tool boundaries through that owner
  representation-only :T consumer audit and migration queue
  remove direct storage/layout/planner authority from source annotations
  split types.md normative semantics from implementation-status ledger
  keep generated/current support status outside the normative type law

T4 complete:
  3490 exact-numeric annotation-island closeout audit
  exact-numeric annotation-island closeout
  parameter + return + Box field + local owner/exhaustiveness audit
  shared value-check semantics, distinct boundary timing

T5 complete through reference semantics:
  3493 record construction/update contract
  3495 readonly U16 static-table closeout
  3497 ArrayElementWrite convergence
  3499 typed Array exact-numeric state contract
  3501 WeakRef value-law corrective
  3502 WeakFieldContractOwner and VM reference consumer

T6 complete:
  3503 selects defer-with-failfast; no product backend or helper ABI selected
  declaration-level capability rejects read-only and dynamic-only obligations
  VM remains semantic-reference only and cannot satisfy product parity
  stop-the-line gate repair removes the obsolete syntax-3 comment token and
  joins naming_charter_guard to the Language v1 FULL sensitive-path gate

Queued residue after the 3503 decision:
  LANGV1-UNTYPED-FIELD-PHI-FAILFAST-CORRECTIVE-001
  replace debug panic/release Unknown fallback with one Result-bearing owner,
  stable type/* rejection, and an untyped-field method-return fixture
  all active carriers covered by unsupported-backend pre-effect rejection
  no new type-family activation

T5 active:
  3491 selects record construction/update and RecordValueContractOwner
  typed Array remains unselected without a source-owned element contract
  3492 parameter BindingId corrective and FULL 12/12 gate are complete
  3493 record carrier/VM/JSON/backend-gate implementation is active
  require producer + owner + runtime consumer + JSON carrier + backend gate
  do not open FFI/backend lowering or all-types activation implicitly
```

Do not create inventory-only, fixture-only, or rerun-only numbered cards for
T1-T5. Materialize only the currently selected design or implementation card.

Acceptance:

```text
annotation_semantic_contract = 1
unannotated_value_contract = Any
annotation_site_set_closed = 1
contract_check_owner_count_per_boundary = 1
metadata_hint_spelled_as_type_annotation = 0
unsupported_backend_fail_fast_before_effect = 1
normative_type_law_status_ledger_separated = 1
```

Next: `LANGV1-FAILURE-OUTCOME-001`.

Fable5 review routing (2026-07-11):

```text
already owned by Failure/Outcome:
  null / void / Option::None terminal relation
  local-without-initializer default migration
  failed WeakRef upgrade result migration
  grammar literal_null Canonical -> Compat2025 decision and source migration

new semantic row after Failure/Outcome:
  truthiness, compatibility equality, broad String concatenation, type tests

already owned by Ownership/Identity:
  ordinary strong-field cascade fini removal
  identity/lifetime split

parked compiler BoxShape, not Language-v1 blockers:
  nested-loop depth1 feature-product decomposition
  route-family graph shadow -> authority promotion
```

Parked compiler follow-up contracts:

```text
nested-loop depth1 decomposition:
  owner = plan/REGISTRY.md + coreplan skeleton/feature SSOT
  trigger = four feature-product planners already exist
  shape = one NestedLoopDepth1Skeleton + typed FeatureSet
  accepted-shape expansion = 0
  release-default change = 0

route-family convergence promotion:
  owner = route_dependency_graph + route_fixpoint facade
  prerequisite = fresh current-corpus shadow/full-refresh parity
  prerequisite = deterministic dirty worklist and zero stale metadata reads
  prerequisite = measured recompute reduction without semantic drift
  next step = one authority-switch design card, not an optimization patch
  helper-name cache / fixture cache / wall-clock termination = forbidden
```

Neither compiler follow-up may preempt the current Language v1 blocker without
an explicit lane-priority decision.

The current registry already classifies `literal_null` as Canonical in both
profiles. Moving it to Compat2025 therefore overturns an accepted 3477/3478
decision and must happen only through the Failure/Outcome decision plus source
migration; it is not a grammar typo cleanup.

### 4. LANGV1-FAILURE-OUTCOME-001

Status: active design decision as 3504 after Type Guarantee closeout.

Target vocabulary:

```text
Option::None = ordinary value absence
Result::Err  = recoverable failure returned as a value
Fault        = violated language/runtime contract
Normal(Unit) = successful computation with no useful result
null         = Compat2025 migration surface only
```

Deliverables:

1. Define `void` as the Unit spelling/value and keep it distinct from
   NoFallthrough and Fault.
2. Define Fault categories, propagation, top-level diagnostics, and cleanup
   execution. Fault must not convert implicitly to Result.
3. Make canonical recoverable failure use Result. Restrict catch to an explicit
   finite FFI/compat boundary or reject it in Canonical mode.
4. Inventory every live `null` use by meaning: optional absence, no-result,
   parser sentinel, foreign null, or compatibility.
5. Migrate by meaning to Option, Result, Unit, or explicit FFI carrier. Do not
   globally replace text.
6. Keep `null` available only under explicit `Compat2025` after migration.
7. Explicitly classify and migrate `local x` default initialization,
   `WeakRef.weak_to_strong()` failure, `NullBox`, `VoidBox`, `MissingBox`, and
   dropped-WeakRef observations. No runtime representation may silently define
   the source absence relation.
8. Change the `literal_null` registry profile only after Canonical source and
   API fixtures no longer rely on it. Canonical rejection must not retry the
   compatibility profile.
9. Publish one VM/EXE matrix for Unit, Option::None, Result::Err, Fault,
   foreign null, and Compat2025 null, including cleanup precedence.

Acceptance:

```text
absence_failure_fault_relation_count = 1
fault_implicit_result_conversion = 0
unit_fault_nofallthrough_distinct = 1
canonical_null_surface = 0
compat2025_null_surface = 1
catchable_fault_set_closed = 1
local_default_absence_owner_count = 1
weak_upgrade_absence_owner_count = 1
```

Next: `LANGV1-COERCION-EQUALITY-COMPAT-001`.

### 4A. LANGV1-COERCION-EQUALITY-COMPAT-001

Status: design decision required after Failure/Outcome.

Purpose: remove dynamic-language compatibility residue from Canonical
truthiness, equality, concatenation, and type tests without silently changing
the already accepted runtime rules.

Decision surface:

```text
truthiness Canonical candidate set:
  Bool only
  or Bool + Integer

Compat2025 candidates:
  Float truthiness
  String / StringBox truthiness

canonical equality:
  primitive same-kind plus accepted numeric relation
  Box equality follows BoxIdentity
  absence follows the landed Failure/Outcome relation

compatibility equality candidates:
  Void == NullBox / VoidBox / MissingBox
  dropped WeakRef == Void

canonical concatenation candidate:
  String + String only

compatibility concatenation candidate:
  String + any implicit toString coercion

canonical type test:
  typed type-reference surface, exact spelling selected by consultation

compatibility type test candidate:
  x.is("TypeName") / x.as("TypeName")
```

Deliverables:

1. Define one closed profile-aware semantic matrix for truthiness, equality,
   concatenation, and type test/cast. Grammar acceptance alone is not semantic
   authority; decide one semantic compatibility registry or equivalent SSOT.
2. Select the Canonical truthiness set explicitly. Treat the current
   Bool/Integer/ExactNumeric/Float/String behavior as implementation evidence,
   not an automatic v1 decision.
3. Remove compatibility equality from the Canonical relation. Inventory
   `Void`, `NullBox`, `VoidBox`, `MissingBox`, dropped WeakRef, and pointer
   identity together so absence and identity are not mixed.
4. Decide whether Canonical concatenation is `String + String` only. Any broad
   implicit stringify rule retained for migration must require explicit
   Compat2025 and reject Void/Fault. Inventory both operand orders in VM
   `eval_binop`, `StringBox` dynamic-add dispatch, and the generic `AddBox`
   stringify fallback; none may independently define Canonical semantics.
5. Replace stringly typed `x.is("TypeName")` / `x.as("TypeName")` with a
   type-reference-based Canonical surface. Typos and unsupported backend paths
   fail before user effects; string spellings remain compatibility-only if
   retained.
6. Generate positive/negative/profile fixtures for independent parsers where
   syntax changes, plus VM and EXE semantic fixtures. Do not use environment
   flags or Canonical-to-Compat retry.
7. Inventory source migration before changing defaults. Selfhost usage is
   migration evidence, not authority to keep a rule Canonical.

Acceptance:

```text
truthiness_semantic_owner_count = 1
truthiness_canonical_value_set_closed = 1
implicit_string_truthiness_canonical = 0
compatibility_equality_in_canonical = 0
box_equality_uses_identity_relation = 1
broad_string_concat_canonical = 0
string_type_name_test_canonical = 0
semantic_compatibility_requires_explicit_profile = 1
runtime_backend_fallback = 0
```

Next: `LANGV1-OWNERSHIP-IDENTITY-001`.

### 5. LANGV1-OWNERSHIP-IDENTITY-001

Status: design decision required after Coercion/Equality compatibility.

Target laws:

```text
record = immutable identity-free value; with creates a new value
box = shared identity handle; assignment/param/return share identity
weak = non-owning BoxIdentity handle
ordinary field = shared strong
weak field = non-owning
owned field = reserved until exclusive lifecycle authority is enforceable
```

Deliverables:

1. Define record copy/update/equality and reject identity/lifecycle operations
   on records.
2. Define Box assignment, call, return, equality, hash, Dead observation, and
   no-implicit-clone rules.
3. Remove implicit cascade `fini()` through ordinary shared strong fields.
   Parent cleanup explicitly finalizes resources it owns.
4. Keep a future `owned` category reserved; do not add syntax until transfer,
   alias, overwrite, cycle, and partial-birth rules are enforceable.
5. Project BoxRef, WeakRef, host handles, plugin mapping, equality, and hash to
   one generation-aware `BoxIdentity(ObjectHandle, generation)` relation.
6. Add VM/runtime and EXE fixtures for aliases, weak upgrade, Dead/Freed tokens,
   generation mismatch, repeated fini, and parent/child finalization.
7. Replace prose-only escape rules with one closed ownership-operation
   vocabulary covering local bind/share, assignment, argument/return/outbox,
   strong-field publication, weak acquisition/upgrade, explicit fini, and
   runtime reclamation. Each operation names whether identity is shared,
   authority transfers, or no ownership change occurs.
8. Verifier/runtime/backend consume the same operation relation. They must not
   infer ownership transfer from variable names, field strength, reference
   counts, or cleanup placement independently.

Acceptance:

```text
record_box_semantic_law_count = 2
ordinary_field_ownership = shared_strong
ordinary_field_implicit_cascade_fini = 0
box_identity_relation_count = 1
weak_identity_relation_count = 1
owned_surface_enabled = 0
ownership_operation_relation_count = 1
prose_only_escape_authority = 0
```

Next: `LANGV1-CAPABILITY-EFFECT-001`.

### 6. LANGV1-CAPABILITY-EFFECT-001

Status: queued after Ownership/Identity.

Target axes:

```text
uses X = declared authority budget
EffectSummary = observed/transitive effect set
@rune Contract(...) = verifier obligation
```

Deliverables:

1. Define a closed effect vocabulary and conservative treatment of dynamic
   dispatch, FFI, unknown callees, allocation, safepoints, and publication.
2. Verify `actual effects` are a subset of declared `uses`.
3. Verify Rune promises against actual effects independently of capability
   authorization.
4. Change CapabilityPlan/EffectPlan to `verified=true` only after both checks.
5. Reject unknown declared capability names instead of ignoring them.
6. Backend consumes verified Plan only and must not rediscover effects or
   authority from source names.

Acceptance:

```text
capability_effect_contract_axis_count = 3
actual_effect_subset_declared_uses = 1
rune_contract_verified_independently = 1
unknown_declared_capability_ignored = 0
backend_unverified_plan_consumption = 0
```

Next: `LANGV1-CONFORMANCE-CLOSEOUT-001`.

### 7. LANGV1-CONFORMANCE-CLOSEOUT-001

Status: queued last.

Deliverables:

1. Run canonical grammar and ParseWitness conformance on both parsers.
2. Run semantic-kernel evaluation-order and Outcome fixtures on VM and EXE.
3. Run type, failure, ownership, identity, and capability positive/negative
   packs with unsupported-backend preflight checks.
4. Prove Canonical default has zero implicit compatibility.
5. Update reference docs from generated views and close v1 freeze.
6. Only then unpark selfhost migration and resume 3456.

Acceptance:

```text
language_v1_frozen = 1
semantic_kernel_count = 1
language_contract_count = 1
independent_parser_count = 2
canonical_normal_form_count = 1
implicit_compatibility_count = 0
vm_exe_semantic_parity = 1
selfhost_migration_unparked = 1
```

## Current Front

The live front is owned only by `CURRENT_STATE.toml`. Do not duplicate a card
token here. The parked MirBuilder resume remains recorded by the current-state
lane status and the language-v1 closeout row.
