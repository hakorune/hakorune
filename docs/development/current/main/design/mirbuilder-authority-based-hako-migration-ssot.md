---
Status: SSOT
Date: 2026-07-04
Scope: MirBuilder authority-based Rust-to-Hako migration order.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-candidate-inventory-v0.json
---

# MirBuilder Authority-Based Hako Migration

## Decision

MirBuilder migration unit is authority, not Rust module, struct, or file.

Do not translate Rust structure into `.hako` by shape. Use the existing
MirBuilder authority seams as the migration boundaries:

```text
Facts -> Recipe -> Lower
plan REGISTRY rule
symbolic command producer
executor / verifier
ID allocation authority
```

The migration should reduce Rust semantic authority by contract. It should not
copy the current Rust call graph into native `.hako`.

## Current Entry

The current safe entry is still the narrow Rust-oracle parity-pilot lane:

```text
vocabulary leaf / boolean classifier
  -> Rust oracle fixture
  -> hand-authored .hako implementation
  -> .hako EXE parity gate
  -> HakoAdopted decision
```

The inventory entry for this lane is:

```text
tools/rust_lifecycle/mirbuilder_native_owner_candidate_inventory.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-candidate-inventory-v0.json
```

This inventory is a recall aid and artifact consistency check. It does not
select the next owner and does not make a Source Selfhost claim.

## Anti-Drift Operating Rule

Normal work stays in migration mode, not inventory mode.

For each `MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-N`, the
unit of progress is exactly one owner:

```text
select exactly one owner
  -> Rust oracle fixture
  -> hand-authored .hako implementation
  -> .hako EXE parity gate
  -> HakoAdopted decision
  -> advance pointer to RERUN-(N+1)
  -> stop
```

The inventory may be read only to select the next owner or to reflect a
completed adoption. Do not spend a normal migration turn improving the
inventory, broadening candidate taxonomy, or reclassifying unrelated surfaces
unless the user explicitly switches the turn to inventory work.

Candidate priority for the narrow parity-pilot lane is:

```text
near an already adopted owner
1-2 row pure formatter / classifier
scalar token input
stable string or boolean-text output
no route matching
no backend lowering
no MIR mutation
no ID allocation
```

If the next useful step would be Fact owner migration, plan construction,
symbolic command production, allocator movement, or a Source Selfhost claim,
stop in design-consultation mode instead of implementing.

## Working Rules

This is the compact memory aid for the lane. Keep it stable and read it before
choosing the next owner.

- One rerun adopts exactly one owner, then stops.
- Inventory is recall-only. Do not spend a normal migration turn on inventory
  work unless the turn is explicitly about selection or reconciliation.
- The safe owner cadence is fixed:
  `Rust oracle fixture -> hand-authored .hako implementation -> parity gate -> HakoAdopted decision`.
- After a landing, sync the restart pointers in the same turn:
  `CURRENT_STATE.toml`, `latest_card_path`, `landed_tail`, and the inventory
  fixture.
- If the next step is Fact owner migration, plan construction, symbolic command
  production, allocator movement, or a Source Selfhost claim, stop and switch
  to design-consultation mode instead of widening the slice.
- Keep `source_selfhost_claim = 0` until the later stages in this document are
  explicitly reached.

## Pointer Sync Rule

When a rerun lands, keep the restart pointers synchronized in the same turn:

```text
CURRENT_STATE.toml.latest_card
CURRENT_STATE.toml.latest_card_path
CURRENT_STATE.toml.landed_tail
mirbuilder-native-owner-candidate-inventory-v0.json
```

Do not leave `latest_card_path` pointing at a missing file. The generated
inventory fixture is the recall source for next-owner selection, but the current
state file remains the restart SSOT.

## Stages

### 1. Vocabulary Leaf

Move pure label, tag, formatter, and classifier surfaces to `.hako`.

Allowed:

```text
enum/tag/string label vocabulary
small boolean classifier
stable input -> stable output
Rust oracle parity fixture
```

Forbidden:

```text
plan construction
route collection / route selection
backend lowering
MIR mutation
ID allocation
Source Selfhost claim
```

### 2. Fact Owner

Move read-only fact owners one at a time.

Contract:

```text
input snapshot -> fact DTO
Rust oracle fixture -> .hako output diff
no MIR mutation
no route execution
no ID allocation
```

This stage establishes the reusable fast-gate template for semantic owners
larger than vocabulary leaves.

### 3. Recipe / Plan Rule

Move plan construction by existing authority seam, not by source file.

Preferred unit:

```text
one REGISTRY rule = one responsibility = one parity gate
```

The `.hako` owner returns a recipe or plan DTO. Rust remains the executor and
verifier during this stage.

### 4. Symbolic Command Producer

Move mutation intent, not raw mutation.

Contract:

```text
.hako emits symbolic command list
Rust verifies command list
Rust executes MIR mutation
symbolic IDs only
```

This keeps mutation ordering observable before moving allocation authority.

### 5. Allocation / Executor Authority

Move ID allocation and executor authority only after command ordering is stable.

At that point allocation becomes a pure fold over a command list:

```text
symbolic command list -> allocated command list
```

This is the first point where Source Selfhost authority can be reconsidered for
the selected MirBuilder slice.

## ID Allocation Boundary

During stages 1-4, Rust is the only allocation authority.

Reason:

```text
ValueId / BlockId allocation correctness is mostly call-order correctness.
If allocation crosses the Rust/.hako boundary before command order is stable,
one off-by-one allocation turns parity diffs into noise.
```

During migration, `.hako` must use symbolic IDs only:

```text
%tmp.cond
%tmp.body_value
bb.loop_head
bb.after
```

Rust executor maps symbolic IDs to real IDs. Parity for stages 3-4 compares the
symbolic plan or symbolic command list before real ID assignment whenever
possible.

## Taskization

Use these tokens for the next durable slices. They are task identifiers, not
proof of implementation.

```text
MIRBUILDER-NATIVE-OWNER-CANDIDATE-INVENTORY-001
  Status: materialized.
  Scope: read-only inventory for existing narrow parity-pilot artifacts and
         advisory Rust source candidates.
  Non-claim: no next owner selected.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-084
  Status: historical after rerun 084 landed.
  Scope: kept as provenance for the prior narrow vocabulary pilot.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-085
  Status: historical after rerun 085 landed.
  Scope: kept as provenance for the prior narrow vocabulary pilot.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-086
  Status: historical after rerun 086 landed.
  Scope: kept as provenance for the prior narrow vocabulary pilot.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-087
  Status: historical after rerun 087 landed.
  Scope: kept as provenance for the prior narrow vocabulary pilot.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-088
  Status: historical after rerun 088 landed.
  Scope: kept as provenance for the prior narrow vocabulary pilot.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-095
  Status: historical after rerun 095 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-096
  Status: historical after rerun 096 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-097
  Status: historical after rerun 097 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-098
  Status: historical after rerun 098 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-099
  Status: historical after rerun 099 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-100
  Status: historical after rerun 100 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-101
  Status: historical after rerun 101 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-102
  Status: historical after rerun 102 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-103
  Status: historical after rerun 103 landed.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-104
  Status: current next selection.
  Scope: choose one more vocabulary_leaf, boolean_classifier, or formatter
         owner from the inventory, then proceed with the existing Rust-oracle
         parity sequence.
  Non-claim: no Fact owner migration yet.

MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001
  Status: pending after the current vocabulary-leaf cadence is intentionally
          paused or closed.
  Scope: select one small read-only fact owner and define the canonical
         Rust-oracle fixture -> .hako implementation -> output diff gate.
  Non-claim: no plan construction migration.

MIRBUILDER-REGISTRY-RULE-PLAN-PARITY-PILOT-SELECTION-001
  Status: pending after the Fact owner parity template is green.
  Scope: select one existing plan REGISTRY rule as a recipe/plan DTO parity
         pilot.
  Non-claim: Rust remains executor/verifier.

MIRBUILDER-SYMBOLIC-COMMAND-LIST-PILOT-SELECTION-001
  Status: pending after at least one REGISTRY rule plan parity pilot is green.
  Scope: emit symbolic command lists from `.hako` and execute them in Rust.
  Non-claim: Rust remains ID allocation authority.

MIRBUILDER-ID-ALLOCATION-AUTHORITY-CUTOVER-PREFLIGHT-001
  Status: blocked until symbolic command ordering is stable.
  Scope: prove allocation as a pure fold over command lists.
  Non-claim: no Source Selfhost claim before this preflight is green.
```

## Non-Claims

```text
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
mir_mutation_from_hako = 0 until symbolic command producer stage
id_allocation_from_hako = 0 until allocation cutover preflight
```
