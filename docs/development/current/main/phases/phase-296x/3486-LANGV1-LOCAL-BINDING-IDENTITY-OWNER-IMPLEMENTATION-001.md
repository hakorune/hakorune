# 3486 - LANGV1-LOCAL-BINDING-IDENTITY-OWNER-IMPLEMENTATION-001

## Status

Active code-facing BoxShape prerequisite after 3485 accepts BindingId as the
single local-slot identity owner.

Decision: accepted by 3485.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
accepted_shape_expansion = 0
local_contract_activation = 0
```

## Objective

Make existing `BindingId` complete and locally verifiable across every
accepted local declaration, assignment, branch snapshot, PHI preparation, and
loop/CorePlan publication path. Add a domain wrapper without a second
allocator, then close every direct identity bypass before
`LocalContractWrite` semantic activation begins.

## Structural Owner

```text
allocator:
  existing CoreContext::next_binding only

lexical identity:
  BindingId

contract-domain view:
  LocalSlotId(BindingId)

declaration entry:
  one MirBuilder local declaration API

current runtime ValueId:
  VariableContext, never identity authority

current lexical BindingId:
  BindingContext
```

`LocalSlotId` is a transparent typed wrapper with conversion/accessors only.
It must not allocate, renumber, infer from names, or persist a second ID.

## Ordered Tasks

### 1. Boundary documentation and wrapper

- update the builder variable/scope README boundary before code movement;
- add `LocalSlotId(BindingId)` in the neutral MIR-core vocabulary;
- expose no second counter or generator;
- add compile-time/unit tests proving one-to-one conversion and ordering;
- keep source AST and Program JSON free of compiler-local IDs.

### 2. One declaration API

- make the existing lexical declaration entry return `LocalSlotId`;
- register ValueId and BindingId atomically after initializer evaluation;
- retain current shadowing and same-scope redeclaration behavior;
- diagnostics may retain source names, but lookup returns existing identity;
- reject declaration outside a lexical scope with the existing structural
  boundary rather than allocating a detached identity.

### 3. Snapshot/restore parity

- define one typed snapshot containing both VariableContext and
  BindingContext views;
- migrate branch/CorePlan helpers that currently save only `variable_map`;
- restore ValueId and BindingId together on success and error paths;
- do not let branch-local shadow identities escape scope;
- assignments to an outer local retain its existing BindingId.

### 4. CorePlan and branch publication

- inventory accepted `ASTNode::Local` lowering entries under normal and
  CorePlan/Recipe paths;
- route declarations through the one declaration API;
- route assignments/publications through an existing-identity resolver;
- prohibit post-hoc name-derived IDs and direct declaration-time
  `variable_map`-only publication;
- keep logical `branch_bindings` and the builder emission cache distinct, but
  require identity evidence whenever a binding is published back.

### 5. PHI/loop prerequisite evidence

- add behavior-preserving identity observations for one if-PHI and one loop
  carrier/backedge;
- prove reassignment retains a slot while shadow declaration changes it;
- prove scope exit restores the outer slot;
- do not add runtime checks, contract carriers, PHI contract policy, or a new
  accepted loop shape in this card.

### 6. Structural guard

- add reusable unit/structural checks for missing or duplicate identity;
- ensure accepted local declaration APIs cannot return success without a
  `LocalSlotId`;
- keep logs stable, one-line, opt-in, and add no new environment switch;
- prefer Rust type/API enforcement over source-token count guards.

## Stable Fail-Fast Tags

This prerequisite owns binding-boundary tags only:

```text
type/local_contract_binding_missing
type/local_contract_binding_duplicate
type/local_contract_binding_ctx_bypass_forbidden
type/local_contract_binding_remap_drift
```

Do not define semantic contract violation/backend tags until the next slice
owns `LocalContractWrite`.

## Fixture Matrix

```text
ordinary local declaration:
  exactly one BindingId / LocalSlotId

ordinary reassignment:
  same LocalSlotId, new/current ValueId allowed

same-scope redeclaration:
  existing fail-fast retained, no second detached identity

nested lexical shadow:
  inner LocalSlotId differs; outer restored on exit

branch success/error snapshot:
  ValueId and BindingId maps restore together

CorePlan local declaration:
  same declaration API and identity law as ordinary lowering

if-PHI publication:
  outer local identity retained

loop backedge/break/continue exit publication:
  carrier identity retained; no accepted shape added

missing identity injection:
  stable fail-fast, no name-derived recovery
```

## Explicit Non-Claims

```text
LocalContractWrite = 0
FunctionMetadata.local_slot_contracts = 0
local_exact_numeric_contract_activation = 0
local_uninitialized_policy_activation = 0
local_runtime_check = 0
local_proof_elision = 0
PHI_contract_evidence = 0
loop_contract_evidence = 0
MIR_JSON_local_contract = 0
backend_capability_expansion = 0
accepted_shape_expansion = 0
release_default_changed = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Acceptance

```text
local_slot_wrapper_count = 1
local_slot_allocator_count = 0
binding_allocator_count = 1
local_declaration_identity_owner_count = 1
accepted_local_declaration_binding_coverage = complete
snapshot_value_binding_parity = 1
shadow_new_identity = 1
scope_exit_outer_identity_restore = 1
assignment_identity_retained = 1
coreplan_binding_ctx_bypass_count = 0
local_contract_activation = 0
changed_production_source_over_800_lines = 0
```

## Verification

```text
focused MIR-core LocalSlotId tests
lexical scope / BindingContext unit tests
ordinary local and reassignment builder tests
CorePlan local/branch identity tests
if-PHI and loop-carrier identity fixtures
existing focused CorePlan gates for touched paths
cargo check -q --all-targets --features vm-reference
cargo build --release --bin hakorune
current-state pointer guard
git diff --check
```

## Next Gate

Only after every acceptance item is green, open one semantic implementation
card for `LocalContractWrite`, exact-numeric init/reassignment, U1 rejection,
PHI/loop evidence, MIR JSON, VM support, and non-VM backend preflight.
