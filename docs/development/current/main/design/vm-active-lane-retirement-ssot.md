---
Status: SSOT
Decision: accepted
Date: 2026-08-27
Scope: Rust MIR interpreter retirement and `.hako` semantic-reference interpreter migration.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1075-VM-ACTIVE-LANE-RETIRE-001.md
  - apps/rust-subset-to-hako/probes/README.md
---

# VM Active Lane Retirement SSOT

## Current Capsule

- **Current decision:** complete Rust `MirInterpreter` retirement is the final
  target. Public/default route retirement, temporary reference-engine use,
  current `vm-hako` retirement, and the future independent `.hako` artifact
  are four separate lifecycle stages.
- **Current implementation status:** `VM-RUNTIME-RETIREMENT-FATE-D0` and the
  six-row `CORE-DIRECT-RETIRE-D0` census are closed as design-only decisions.
  No route or engine deletion is authorized; the MirBuilder Call spine remains
  the serial implementation authority.
- **Next ordered task:** `CORE-DIRECT-RETIRE-R0` is a design stop until its one
  ProductAot substring successor and decoded CoreDirect terminal fixture exist.
- **Production stop line:** existing VM code or tests never authorize product
  parity, fallback, or a new compatibility route; retirement must not redirect
  incompatible inputs to another parser or executor.
- **Retirement finish line:** product execution has one LLVM/AOT owner,
  semantic reference has one explicitly selected owner, fallback/retry is zero,
  and each retired implementation reaches exact caller zero before deletion.

## Accepted fate Decision — `VM-RUNTIME-RETIREMENT-FATE-D0`

Two independent read-only audits closed the route/engine premise census at
HEAD `16f9beb86489a4a69043bf4494e75c11f737173a`. The result accepts full Rust
engine retirement, but rejects treating every surface called “VM” as one
deletion unit.

```text
Decision:
  Retire current vm-hako and every broad/default Rust execution route; retain
  one explicit Rust semantic-reference subset only until an independent AOT
  .hako interpreter cuts over, then physically delete MirInterpreter.
Source authority + canonical issuer:
  Product policy selects LLVM/AOT. Each named consumer owns whether it needs a
  product proof, semantic oracle, Stage1 proof, compile-only artifact, or no
  successor; a CLI/env selector only issues the selected terminal.
Non-authority:
  Code existence, old backend labels, smoke counts, current vm-hako, and future
  interpreter intent cannot issue semantics or keep a route alive.
Fail-fast boundary:
  A retired selector has one stable terminal and retry/fallback zero. An HMI
  capability miss fails before effects and never retries in Rust.
Smallest next slice:
  CORE-DIRECT-RETIRE-R0-D0: freeze the one ProductAot successor and one decoded
  family-terminal fixture; no implementation or old-script deletion yet.
Non-claims:
  No broad VM/default-mir/engine deletion, no vm-hako enhancement, no HMI
  carrier selection, no PyVM archive deletion, and no llvmlite G3 approval.
```

Census boundary: non-archive public CLI/env/startup selectors and external
`MirInterpreter` constructors -> selected terminal. It includes CoreDirect,
force-hv1, `vm-compat-fallback`, broad `--backend vm`, default `mir`, direct
JSON/pipe, REPL, current `vm-hako`, the three explicit reference selectors,
PyVM product hook, and engine-external constructors. It excludes interpreter
internals, archive files, LLVM codegen internals, and llvmlite G3.

Observed engine boundary: `src/backend/mir_interpreter/**` is 91 Rust files /
18,239 lines. Outside that tree, the non-archive exact
`MirInterpreter::new(` census is 16 files / 25 hits: five
production/reference files with six hits and 11 test files with 19 hits. This
is the reproducible direct-constructor boundary only; `VM`/`NyashVm` aliases,
borrowed engine users, and transitive route callers are separately inventoried
before R1/R2.

In this SSOT, engine `caller zero` means zero compiled callers plus zero
non-archive source/test/reference callers. Archive rows are a separate finite
inventory: each must be marked `HistoricalSourceOnly` or deleted, and none may
be reachable from Cargo, checks, generated includes, or active fixtures.

## Finite route disposition

| Input/consumer state | Sole disposition | Terminal rule |
|---|---|---|
| Product execution or performance | LLVM/EXE/AOT | VM/PyVM/llvmlite retry zero |
| Compile/emit/inspect only | compiler-owned artifact entry | execution zero |
| Named semantic reference | temporary explicit Rust reference, then HMI | unsupported is typed fail-fast |
| Bootstrap/selfhost proof | explicit Stage1 or AOT proof owner | ambient VM selection zero |
| Retired legacy selector | family tombstone during grace period | one tag/rc, retry zero |
| Implementation-specific historical probe | archive fixture or delete | no production caller |
| Unclassified live caller | `CutoverBlockerOpen` | no route/code deletion |
| Physical engine deletion | HMI cut over and every direct/transitive caller zero | delete once, no compatibility stub |

The following are different families and must not share a synthetic successor:

```text
current vm-hako
  = CanonicalV1 projection + private transport + same executable --backend vm
  = retirement target, not the future interpreter

temporary Rust semantic reference
  = raw-vm-reference
  + normal-file-vm-reference
  + normal-file-canonical-core-vm-reference

future HMI
  = independently built ny-llvmc/AOT artifact
  = no current-process child route, env payload, or Rust fallback
```

## Integrated serial task spine

The VM program does not replace the MirBuilder Call program. Only Wpre bypass
removal runs before the Call spine; broad route and engine retirement wait.

```text
0. VM-RUNTIME-RETIREMENT-FATE-D0                         closed here

1. CORE-DIRECT-RETIRE-D0/R0
   D0 is landed: ProductAot=1, SemanticReference=0, HistoricalDelete=5.
   R0 first lands the named EXE/AOT substring successor and a decoded-family
   terminal fixture, then emits [core-direct/retired|unavailable], rc=1 and
   deletes the six old scripts, raw probe, child, in-proc retry, and VM fallback

2. FORCE-HV1-CALLER-MIGRATION-R0..Rn
   close 33 direct + 36 helper rows across the finite 107-file boundary
   -> delete helper retry and startup bypass; never reinterpret as standard v1

3. WPRE-S0/I0
   one strict root -> one selected decoder -> one terminal; parse/retry once/zero

4. MIRBUILDER CALL ROWS
   MirCall/CallFlags -> effect/source/import/lineage/affine ownership
   -> method corridor -> typed Global B1 -> wire/construction closure
   -> R6 mandatory Callee -> R7 -> MIRBUILDER-POST-CALL-INTEGRATION-R0

5. VM-FAMILY-SELECTION-DEAMBIENT-R0
   one token = one owner; split runner fallback request from Stage1/kernel policy;
   remove implicit vm-hako selection from --backend vm

6. LEGACY-EXECUTOR-ROUTE-RETIREMENT
   retire vm-compat-fallback, the seven live PyVM-product-hook scripts, and
   current vm-hako after owner-local caller migration

7. RUST-VM-REFERENCE-ONLY-CUTOVER
   product/default mir, broad --backend vm, direct JSON/pipe, Stage1/selfhost,
   plugin, REPL, and tests -> AOT / compile-only / explicit Stage1 /
   temporary explicit reference / archive; broad source execution reaches zero

8. HMI-AOT-F0/CARRIER-D0/A0/P0/S0/S1/I0/B0/P1/C0/X0
   refresh the stale inventory; choose one post-R6 carrier; define an independent
   executable; build it with ny-llvmc; prove normalized parity; cut over the
   three explicit reference selectors with Rust fallback zero

9. HMI-AOT-R1a/R1b/R2
   external constructors/tests zero -> public/transitive roots zero -> delete
   aliases, feature surface, and 91-file Rust engine

10. REPOSITORY CONVERGENCE
    tombstone expiry, docs/guards/shelves, performance, and separately approved
    llvmlite G3; VMValue/VMError remain while independent consumers exist
```

Rows 5-10 cannot be pulled ahead of Call R7. Read-only freshness/caller
manifests may be prepared earlier, but they grant no implementation permission.

## Current blockers and exact next row

`CORE-DIRECT-RETIRE-D0` is closed. Its six active owners are array out-of-bounds
set, map bad key, string bounds, `charAt` bounds, replace success, and substring
success. The dispositions are:

```text
ProductAot = core_direct_string_substring_ok_vm.sh
SemanticReference = none
HistoricalDelete = the other five rc-only/unsupported/incorrect-contract scripts
```

`CORE-DIRECT-RETIRE-R0` remains a design stop because the ProductAot successor
does not yet have exact output/rc evidence and the family terminal must be
proven after decoded-family authority, not by a raw substring or second parser.
Once those two blockers are closed, one bounded R0 may delete all six scripts
and the old CoreDirect implementation with retry/fallback zero.

## Decision

The VM is no longer a product-level app execution target for current compiler
construction work.

```text
rust_vm_active_product_target=0
rust_vm_semantic_reference_subset=temporary
rust_vm_semantic_reference_final_owner=independent_hmi_aot_after_c0
hako_vm_active_product_target=0
current_vm_hako_retirement_target=1
hako_mir_interpreter_semantic_reference_target=independent_aot_artifact
primary_app_validation_route=exe_aot
```

The Rust `MirInterpreter` remains temporarily useful for small semantic smoke
tests and focused MIR reference checks. It is not the final semantic-reference
owner and is not the route where new product apps, JSON-heavy converter apps,
or selfhost compiler fronts must prove full runtime behavior.

The final direction is:

```text
canonical MIR contract
  -> independently built `.hako` MIR interpreter artifact
  -> semantic-reference owner

canonical MIR JSON/object lowering
  -> EXE/AOT as the production execution owner

Rust MirInterpreter
  -> exact caller-zero retirement after `.hako` cutover
```

## Non-negotiable cutover and retirement law

The temporary Rust interpreter is not a permanent compatibility fallback. The
only valid end state is:

```text
canonical MIR semantic-reference subset
  -> `.hako` MIR interpreter

product/app execution
  -> EXE/AOT

Rust MirInterpreter semantic callers
  -> zero
  -> source deletion
```

The transition is deliberately staged:

```text
HMI-AOT-F0/CARRIER-D0/A0
  -> fresh inventory + one post-R6 carrier + independent entry contract

HMI-AOT-P0/S0/S1/I0/B0/P1
  -> sealed portable subset + AOT build + independent normalized parity

HMI-AOT-C0
  -> `.hako` becomes the default semantic-reference runner for that closed
     subset; Rust fallback is forbidden

HMI-AOT-X0
  -> expand one named MIR instruction family at a time, each with parity and
     fail-fast unsupported-instruction behavior

HMI-AOT-R1/R2
  -> classify every remaining Rust caller, move or delete it, prove Rust
     semantic-reference caller count is zero, then delete the retired code
```

No row may keep a hidden Rust retry after HMI-AOT-C0. A `.hako` capability gap is a
typed fail-fast result until its named HMI-AOT-X0 expansion closes; it is never a
reason to execute the same module in Rust.

The current `raw-vm-reference`, `normal-file-vm-reference`, and
`normal-file-canonical-core-vm-reference` selectors are intentionally not this
cutover. They use a fresh Rust `MirInterpreter` only as the temporary reference
backend while sealing compiler, source-entry, and process-result semantics.
All three belong to HMI-AOT-R1. They may target `.hako` only after the HMI subset
that covers them has completed HMI-AOT-C0/P1 parity. This prevents an implicit
backend switch or a second entry/result authority.

This is an implementation-owner migration, not an AST-interpreter decision.
The `.hako` interpreter consumes canonical MIR and must not recover source
semantics from AST, ProgramV0, names, or backend behavior.

## Rationale

The RustSubset JSON -> `.hako` converter reached runtime through the VM route:

```text
filebox_read_enabled=1
json_tokenizer_probe_green=1
joinir_acceptance_blocker_cleared=1
global_mir_call_payload_normalized=1
```

The blocker moved from compiler acceptance to runtime collection semantics:

```text
mapbox_primitive_roundtrip=1
mapbox_user_box_roundtrip=0
arraybox_user_box_roundtrip=0
json_tree_parse_result=null
```

Keeping the VM as a product-level route would require continuing to grow:

```text
rust_vm_collection_user_box_semantics
hako_vm_collection_user_box_semantics
json_native_tree_runtime_surface
product_app_runtime_parity
```

That would split effort across too many execution engines while the current
goal is compiler construction and selfhost progress.

## Target Execution Ownership

```text
EXE/AOT:
  primary product/app validation route
  primary selfhost app-front validation route
  primary performance route

Rust VM:
  temporary small semantic reference subset
  focused MIR smoke tests
  payload normalization reference tests
  no broad runtime parity expansion
  exact caller-zero retirement target

current vm-hako:
  CanonicalV1 compatibility projection over a Rust VM child
  retirement target; never the future HMI artifact

.hako MIR interpreter:
  independently AOT-built final small semantic-reference owner
  canonical MIR instruction semantics and normalized event oracle
  expanded only for a named compiler-semantic consumer
  never a product-app parity target
```

## Migration architecture

```text
canonical MIR
  stable opcode and ownership semantics
        │
        ├─ LLVM/object consumer
        │    production EXE/AOT
        │
        ├─ temporary Rust MirInterpreter adapter
        │    migration oracle only
        │
        └─ standalone AOT `.hako` MirInterpreter artifact
             final semantic-reference owner
             normalized outcome + ownership/control events
```

MIR owns meanings such as:

```text
CopyOwned:
  produce a fresh independently consumable owner for the same object without
  consuming the source

DestroyOwned:
  consume exactly the named owned value and no same-object alias

Copy:
  value/representation copy, not an implicit ownership-policy authority

ReleaseStrong:
  legacy lifecycle vocabulary; canonical callers forbidden
```

`Arc`, `VMValue`, Rust register storage, runtime handle numbers, and backend
helper names are implementation details. They cannot appear in the portable
MIR semantic contract or normalized parity authority.

The old HMI-P0 selected strict MIR JSON V1 for a disconnected, Call-free
prototype. That decision is not authority over the final mandatory-Callee
Call corridor, whose canonical wire is exact V2. After Call R6/R7,
`HMI-AOT-CARRIER-D0` must select exactly one of these outcomes:

```text
Call-free HMI subset
  -> retain exact V1 only for that sealed subset
  -> every Call semantic consumer moves to AOT or structural verification

Call-bearing HMI subset
  -> adopt one exact V2 Call profile
  -> mandatory typed Callee; no V1 retry or dual automatic decoder
```

Raw Rust `MirModule` access, source AST, reconstructed ProgramV0, MIR JSON v0,
compact compatibility payloads, and JSON-to-Rust-MirModule reconstruction are
forbidden HMI authorities in either outcome. The public producer must include
semantic refresh, emit one selected carrier, and never expose the private root
builder or inherit environment-controlled schema selection. The historical
V1 inventory remains at:

```text
../investigations/hmi-p0-mir-json-v1-strict-ingress-inventory-task-2026-07-16.md
```

## Allowed VM Work

VM work is allowed only when it is narrow and directly protects a semantic
reference contract:

```text
allowed=payload_normalization_unit_tests
allowed=small_mir_semantic_smoke
allowed=fail_fast_diagnostic_for_unsupported_vm_surface
allowed=regression_test_for_already_supported_subset
allowed=bounded_rust_hako_mir_semantic_parity
allowed=caller_inventory_and_retirement_guard
```

## Disallowed VM Work

```text
disallowed=product_json_app_runtime_parity
disallowed=full_user_box_collection_semantics_for_app_execution
disallowed=feature_work_required_only_by_vm_product_route
disallowed=simultaneous_rust_vm_and_hako_vm_product_development
disallowed=silent_vm_fallback_to_hide_aot_gap
disallowed=whole_rust_vm_translation_before_subset_inventory
disallowed=ast_or_programv0_interpreter_as_mir_reference
```

## Required migration tasks

The migration is mandatory for the selfhost final form but is not allowed to
widen the active compiler row. It starts only after the MIR ownership
vocabulary and its supported-backend boundary are stable.

The completed strict-V1 prototype receipts retain their old `HMI-P0/S0/...`
IDs as history. The post-R6 standalone-artifact program uses the distinct
`HMI-AOT-*` family; old IDs are never reissued for new acceptance.

### HMI-AOT-F0 — current-HEAD inventory refresh

The existing inventory is stale and cannot authorize deletion. Its JSON lists
seven caller rows, the generated Markdown lists eight, an older closeout claims
nine, and the static checker is missing eight current instruction variants:

```text
CheckedCallOut / CheckedCallOutEnd / CheckedCallOutFault
CheckedCallOutNormalResult
PinnedTextOp / PinnedTextResidenceEnter
PinnedTextResidenceFinish / PinnedTextResidenceTrap
```

Regenerate instruction, caller, fixture, and transport inventories from the
same HEAD and make JSON, generated view, and guard agree. This is read-only
evidence and changes no execution owner.

### HMI-AOT-CARRIER-D0 — one post-R6 carrier

Run only after mandatory-Callee R6/R7. Choose the Call-free exact-V1 outcome or
one exact-V2 Call profile described above. Missing/unsupported family rejects
before effects. V1-to-V2 retry, V2-to-V1 retry, and dual auto-detection are
forbidden.

### HMI-AOT-A0 — independent executable entry contract

Define one standalone input/result/fault contract. The executable accepts one
sealed carrier through its own stdin or file entry. It does not spawn the
current `hakorune`, read route/payload environment variables, or call current
`vm-hako`.

### HMI-AOT-P0 — ingress, opcode, and caller inventory

Inventory:

```text
all Rust MirInterpreter instruction handlers
all semantic-reference fixtures and callers
all VM-only product/compat callers
available MIR transports and their lossiness
backend-specific values hidden behind VMValue
```

Inventory the selected carrier and its lossiness. The inventory changes no
execution owner and cannot reopen the carrier choice.

Selected order:

```text
HMI-AOT-P0-D0  exact selected-carrier/strict-profile decision lock
HMI-AOT-P0-I0  checked-in handler/caller/fixture/transport/value inventory
HMI-AOT-P0-G0  freshness, coverage, lossiness report and guards
HMI-AOT-S0-D0  strict-seal/interpreter implementation packet
```

The strict seal does not exist at P0-D0. P0 specifies and inventories it;
HMI-AOT-S0 begins with the direct whole-document reader/seal implementation.

### HMI-AOT-S0 — closed portable semantic subset

Seal the first required subset instead of translating the Rust VM wholesale:

```text
Const
Copy
CopyOwned
DestroyOwned
BinOp
Jump
Branch
Phi
Return
```

The exact list follows the accepted ownership instruction spelling. Unsupported
instructions fail before interpreter effects; there is no Rust fallback.

### HMI-AOT-S1 — normalized observation contract

Compare portable meaning only:

```text
function outcome
ordered block/edge visits
normalized scalar values
CopyOwned / DestroyOwned ownership events
typed failure category and exact source MIR instruction position
```

Do not compare:

```text
Arc identity
VMValue representation
runtime handle numbers
raw host pointers
hash iteration order
backend helper names
```

### HMI-AOT-I0 — disconnected `.hako` MIR interpreter core

Implement the closed S0 subset in small responsibility boxes:

```text
ingress/
  sealed MIR reader only

frame/
  ValueId environment and current predecessor

control/
  block entry, edge selection, Phi input selection and Owned parallel move

ownership/
  VerifiedOwnershipSsaV1 reader and CopyOwned / DestroyOwned semantic adapter

execute/
  instruction dispatch and typed outcome
```

No new or modified source/check file may reach 800 lines. The dispatch layer
does not own opcode semantics; each small box implements the sealed contract.

### HMI-AOT-B0 — independent AOT artifact

Build the `.hako` interpreter with `ny-llvmc` as a standalone artifact. Prove
that execution does not require the `vm-reference` feature, a child invocation
of the current executable, current `vm-hako`, an environment payload, or a
Rust fallback. HMI-AOT-P1 parity uses this artifact.

### HMI-AOT-P1 — independent Rust/`.hako` parity

Run the same sealed MIR fixtures through the Rust reference and the HMI-AOT-B0
artifact, then compare the S1 normalized result. Required fixtures include
straight-line arithmetic, diamond/Phi, Loop backedge/Phi, owned copy/destroy,
Owned Phi forwarding, self-assignment law, BlockExpr tail ownership, and typed
unsupported-op rejection.

Rust is a temporary comparison oracle only. Matching Rust behavior does not
override the MIR contract when the Rust implementation is wrong.

### HMI-AOT-C0 — semantic-reference owner cutover

After parity is green, make `.hako` the default semantic-reference runner for
the closed subset. EXE/AOT remains the product validation route. Rust fallback
is forbidden; unsupported `.hako` instructions fail with a typed capability
error.

### HMI-AOT-X0 — consumer-driven subset expansion

Add one MIR instruction family per capability slice, only when a named
compiler-semantic fixture requires it. Every expansion updates the sealed
subset, `.hako` implementation, normalized parity, and unsupported boundary
in one commit.

Broad collection, JSON application, Box, plugin, or REPL parity is not an
implicit goal.

### HMI-AOT-R1 — Rust caller isolation

Classify every remaining Rust `MirInterpreter` caller as:

```text
AOT product proof
HMI semantic reference
structural verifier
route-retirement proof
historical delete
Blocked(named successor)
```

No permanent `KeepRust` disposition exists. No new Rust semantic-reference
caller is allowed after HMI-AOT-C0.

### HMI-AOT-R2 — physical Rust interpreter retirement

Delete Rust interpreter handlers and support state only after compiled and
non-archive exact caller zero, with every archive reference explicitly marked
`HistoricalSourceOnly` or deleted. A remaining executable legacy consumer is a
blocker, not a reason to claim partial physical retirement.

## Dependency order

```text
MirBuilder Call R7 + MIRBUILDER-POST-CALL-INTEGRATION-R0
  -> HMI-AOT-F0
  -> HMI-AOT-CARRIER-D0
  -> HMI-AOT-A0
  -> HMI-AOT-P0
  -> HMI-AOT-S0
  -> HMI-AOT-S1
  -> HMI-AOT-I0
  -> HMI-AOT-B0
  -> HMI-AOT-P1

HMI-AOT-P1 + first exact BoxRef SSA-I1-O1 owner
  -> HMI-AOT-C0
  -> HMI-AOT-X0
  -> HMI-AOT-R1

HMI-AOT-R1a external constructors/tests zero
  + HMI-AOT-R1b public/transitive route roots zero
  -> HMI-AOT-R2
```

The `.hako` interpreter may grow beside later If/Loop/exit/family expansion,
but it never blocks a production EXE/AOT capability merely to obtain product
VM parity. Physical Rust retirement is a selfhost completion condition, not a
hidden requirement for one D-prime compiler slice.

## Migration acceptance

```text
semantic reference owners after HMI-AOT-C0 = 1 (`.hako`)
Rust fallback after HMI-AOT-C0 = 0
product app validation remains EXE/AOT
current vm-hako caller count = 0
broad/default Rust source-execution route count = 0
external Rust MirInterpreter constructor count before R2 = 0
source AST / ProgramV0 execution authority = 0
normalized parity covers every accepted HMI opcode
unsupported opcode typed fail-fast before effects
new Rust semantic-reference callers after cutover = 0
Rust physical deletion only after compiled/non-archive caller zero and complete archive disposition
```

## Migration non-claims

```text
`.hako` compiler Lower parity
product VM parity
interactive interpreter or REPL readiness
all MIR opcodes supported at HMI-AOT-C0
Box/collection/plugin runtime parity
HMI carrier choice before Call R6/R7
Rust interpreter physically deleted before HMI-AOT-R2
```

## Interactive Interpreter / REPL Parking

Interactive interpreter and Python-like REPL product work is parked.

```text
repl_active_product_target=0
interactive_interpreter_active_product_target=0
rust_mir_interpreter_repl_extension_allowed=0
hako_mir_interpreter_required_before_python_like_repl=1
```

Existing REPL and `MirInterpreter` documentation is historical or
semantic-reference material unless a later accepted current-state card
explicitly reopens the lane.

Do not expand REPL or interpreter behavior for product/app execution. The
`.hako` migration above is a bounded semantic-reference migration, not a
reopening of the product VM or interactive interpreter lane.

Allowed work remains limited to narrow semantic-reference smoke tests,
regression tests for already-supported behavior, and fail-fast diagnostics for
unsupported VM surfaces.

## Converter Implication

The RustSubset converter should not be blocked on the Rust VM route.

```text
rust_subset_converter_primary_route=exe_aot
rust_subset_converter_vm_route=diagnostic_only
json_native_vm_collection_gap_blocks_vm_only=1
```

## Stop Lines

```text
do not fix broad VM runtime parity unless a semantic reference smoke requires it
do not require JSON/native app converter to pass on Rust VM
do not treat VM failure as compiler-construction failure when EXE/AOT is the selected route
do not add VM-specific workarounds to .hako source
do not use silent fallback to mask unsupported VM runtime surfaces
do not translate the Rust VM file-for-file
do not treat Rust parity as authority over the sealed MIR contract
do not delete Rust interpreter code before exact compiled/non-archive caller zero and complete archive disposition
```
