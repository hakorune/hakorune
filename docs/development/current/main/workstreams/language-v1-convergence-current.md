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

The charter and semantic kernel are current truth. The active card implements
only the evaluated-Place compound-assignment slice of that kernel.

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

Status: contract basis accepted as 3460; substrate landed as 3461; Rust
grammar-profile owner accepted as 3462; profile plumbing plus statement-try
implementation active as 3463.

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

Next: `LANGV1-TYPE-GUARANTEE-001`.

### 3. LANGV1-TYPE-GUARANTEE-001

Status: design decision required after Grammar.

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

Acceptance:

```text
annotation_semantic_contract = 1
unannotated_value_contract = Any
annotation_site_set_closed = 1
contract_check_owner_count_per_boundary = 1
metadata_hint_spelled_as_type_annotation = 0
unsupported_backend_fail_fast_before_effect = 1
```

Next: `LANGV1-FAILURE-OUTCOME-001`.

### 4. LANGV1-FAILURE-OUTCOME-001

Status: design decision required after Type Guarantee.

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

Acceptance:

```text
absence_failure_fault_relation_count = 1
fault_implicit_result_conversion = 0
unit_fault_nofallthrough_distinct = 1
canonical_null_surface = 0
compat2025_null_surface = 1
catchable_fault_set_closed = 1
```

Next: `LANGV1-OWNERSHIP-IDENTITY-001`.

### 5. LANGV1-OWNERSHIP-IDENTITY-001

Status: design decision required after Failure Outcome.

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

Acceptance:

```text
record_box_semantic_law_count = 2
ordinary_field_ownership = shared_strong
ordinary_field_implicit_cascade_fini = 0
box_identity_relation_count = 1
weak_identity_relation_count = 1
owned_surface_enabled = 0
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

```text
current = LANGV1-GRAMMAR-CONTRACT-SUBSTRATE-001
next = registry, witness, corpus, adapters, and comparator
parked_resume = MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001
```
