---
Status: Design consultation
Date: 2026-07-20
Scope: FINALIZE0 final responsibility boundary and producer-first retirement order
Parent: docs/development/current/main/investigations/mirbuilder-finalize0-census-task-2026-07-20.md
Basis: 5220d7a307
---

# FINALIZE0 boundary consultation

## Request

Please select the final responsibility boundary for MirBuilder/module
finalization before any `FINALIZE0-CUT0` implementation begins.

The recommended answer is **Candidate A-prime: producer-first pure
finalization with an explicitly temporary legacy-repair quarantine**.

The selected final form would be:

```text
lowering-time producers
  -> complete all facts needed by later lowering
  -> complete every PHI edge and return contract
  -> VerifiedMirFunctionDraft

finalization
  -> NormalizeRepresentation
  -> VerifyCompletedDraft
  -> PublishDerivedArtifact
  -> lifecycle commit outside the pass taxonomy

final repair/inference counts
  RepairMissingLoweringFact = 0
  LegacySemanticInference = 0
```

The question is not whether the current repair passes are useful. They are.
The question is whether they remain part of the permanent finalization model,
or are treated as migration debt whose facts move back to their physical
lowering producers.

## Why the current 51-row census cannot close

Three independent read-only audits found that the current fixture proves
source-anchor existence, not responsibility correctness.

The validator currently checks only:

```text
row count = 51
row ids are unique
class/status vocabulary is known
source file exists
anchor substring exists
```

It does not verify:

```text
all production call sites are present
one semantic operation per row
invocation count
input authority
output and first publisher
mutation class
failure timing/atomicity
downstream consumers
retirement owner
```

Its printed `production_consumers=0` is a fixed string, not a measured claim.

The source census also found three missing operations:

```text
function.metadata_type_snapshot
module.call_await_annotation
module.metadata_origin_snapshot
```

Several existing rows have an incorrect single classification.

### Mixed function/module operations

`verify_typed_values_are_defined` both verifies references and prunes stale
type/origin rows. It is therefore:

```text
VerifyCompletedDraft
+ NormalizeRepresentation
```

`materialize_all_phi_inputs` is explicitly a legacy whole-function repair. It
can:

```text
delete unused PHIs
invent missing self-carried predecessor rows
clone pure instructions into predecessor blocks
allocate new ValueIds
rewrite PHI inputs
update CFG state
```

It is not `VerifyCompletedDraft`. Its current body combines normalization,
repair, and edge validation.

`annotate_missing_result_types_from_calls_and_await` combines:

```text
known Await/Call/Constructor result transfer
name/catalog-based recovery
Unknown insertion for unresolved result shapes
constructor origin publication
```

It is therefore both repair and legacy semantic inference.

### Mutation and ordering hazards

The PHI materializer mutates the function before every later operation is
known to succeed. A later failure can leave deleted PHIs, inserted
instructions, incremented ValueIds, or partially rewritten inputs.

The main-module schedule currently includes:

```text
type repair
-> call/await repair
-> metadata.value_types snapshot
-> PHI return inference
-> PHI input materialization
-> module insertion
-> all-function PHI input materialization
```

PHI edge rematerialization can create new Values after the metadata snapshot.
The snapshot may therefore omit Values present in the final MIR.

The function path has the inverse stale-state problem:

```text
metadata snapshot
-> typed-value verification
-> stale transient-row pruning
```

Rows removed from the transient store may already have been copied into
function metadata.

### First-publication and semantic-recovery hazards

`TypePropagationPipeline` scans completed MIR and can overwrite concrete
Copy/BinOp/PHI type mismatches. It is correctness repair, not normalization.

`infer_return_type_from_phi` reads MIR and name-based policies to publish a
return signature. Its error behavior also differs by build mode:

```text
debug:
  panic on unresolved PHI return

release:
  warn and publish/retain Unknown
```

The post-module schedule also contains mixed facades:

```text
optimizer
contract refresh
semantic refresh
callsite canonicalization
extern route refresh
all-functions semantic stage
```

In particular, callsite/weak/array/extern canonicalization can recover semantic
meaning from string constants, function names, box/field names, arity, and
existing value types, then rewrite MIR or first-publish result facts. These
are not representation-only normalization.

## Repository-wide caller census

The cleanup cannot be proven by only editing `finalize_function_draft` and
`finalize_module`.

```text
TypePropagationPipeline production sites = 3
  function finalization
  main/module finalization
  JoinIR VM conversion

materialize_all_phi_inputs production sites = 3
  main PHI repair
  all-functions PHI repair
  JoinIR rewrite application
```

Semantic refresh and contract refresh also have compiler, optimizer, runner,
host-provider, VM, JSON, and backend consumers. Repository-wide zero must be a
terminal gate, not inferred from one Builder lifecycle.

## Candidate A-prime — producer-first pure finalization (recommended)

### Permanent boundary

Finalization permanently admits only:

```text
VerifyCompletedDraft
NormalizeRepresentation
PublishDerivedArtifact
```

Primary function/module insertion is a lifecycle commit and is recorded
outside this three-class pass taxonomy. It does not become a fourth semantic
inference class.

Every mixed facade is decomposed into semantic child operations before it is
classified. A facade may remain as orchestration, but it cannot be the unit of
authority accounting.

### Producer law

Every fact disposition required by another lowering operation must be sealed
by the successful physical producer before that producer returns.

The disposition vocabulary must preserve legitimate dynamic behavior:

```text
Exact(MirType)
ExplicitUnknownAllowed(reason)
NoValue
```

An origin obligation is present only for the applicable producer family, such
as constructors or another already-sealed box-result route. `Unknown` is not
silently promoted to an exact fact, but a source shape that is intentionally
dynamic is not rejected merely to close FINALIZE0.

```text
FieldGet:
  declared/result type at successful FieldGet emission

Call/Await:
  sealed result disposition at successful emission
  constructor/known-box origin only when the selected disposition requires it

Copy/BinOp:
  exact result fact or explicit unresolved disposition at successful emission

Phi:
  exact input completion and type publication at successful completion

Return:
  sealed completion/return contract projected without MIR/name scanning
```

Missing facts fail at the producer boundary. Finalization never makes an
earlier lowering decision valid after the fact.

### Temporary legacy quarantine

Repair/inference passes may remain temporarily while their production caller
count is nonzero, but they are branded as migration debt:

```text
canonical consumer count = 0
legacy caller inventory is explicit
no Builder consumer treats repaired output as source authority
no retry/fallback is introduced
removal dependency is named per row
```

Before the quarantine is accepted, its canonical consumer count must be zero.
This is an admission condition, not a claim about the current code.

The PHI repair path is first split into:

```text
read-only completed-PHI edge verifier
unused-PHI representation normalization
missing-edge/rematerialization legacy repair
```

If the repair path must remain across more than the immediate producer
cutover, its mutation runs on a candidate function and commits atomically.
This transaction is temporary containment, not the final PHI lifecycle.

### Derived publication law

The Builder transient type/origin snapshot occurs once after Builder-owned
structural closure and before the first post-Builder consumer. It is a one-way
publication from already sealed facts. This law also covers every previously
published non-main function that can currently be mutated by the later
all-functions PHI repair.

Later optimizer/canonicalizer transformations are outside this one-snapshot
claim. They must either maintain metadata coherently with each successful
mutation or consume a distinct sealed input artifact and republish a derived
artifact at their own natural boundary. Recomputable rune/semantic/backend
products are not counted as repeated Builder transient snapshots.

Derived record/layout/typed-object/direct-state/rune/contract products may
remain, but must declare their exact inputs and must not backfeed source or
lowering facts.

Semantic-refresh responsibilities move to their natural retirement owners:

```text
type/origin/string/map/record/route transport and fixpoint:
  METAPROP0

callsite/weak/array/extern legacy canonicalization:
  PLAN0 / RAWADAPT0

ReleaseStrong/ownership materialization:
  Ownership SSA retirement rows

final validation barrier and duplicate rebuild detection:
  FINALIZE0
```

### Benefits

```text
one fact has one producer
finalization timing cannot rescue invalid lowering
debug/release semantics converge
the Builder snapshot describes the exact Builder draft at handoff
canonical and legacy routes do not acquire separate finalizers
the future child FunctionLoweringSession needs no repair backfeed
```

### Cost

The migration requires several producer closures before CUT0. It cannot be
implemented as one broad deletion commit.

## Candidate B — permanent transactional repair finalizer

This candidate keeps repair as a supported finalization responsibility, but
runs it on a candidate function/module and commits atomically.

```text
lowering may leave incomplete type/PHI/return facts
-> finalization repairs the completed graph
-> verifier checks repaired graph
-> metadata snapshot
-> atomic publish
```

Benefits:

```text
smaller initial producer migration
legacy route behavior remains centralized
failure partial-mutation can be fixed directly
```

Costs:

```text
lowering-time and finalized truth remain different
facts needed during lowering still require parallel transient rules
MIR-to-source inference remains tempting
future FunctionLoweringSession must understand repair timing
producer ownership remains ambiguous
```

This is not recommended as the final form. It solves transaction safety but
does not solve authority duplication.

## Candidate C — canonical pure finalizer plus permanent legacy finalizer

This candidate gives canonical D-prime lowering a pure finalizer and retains a
separate repair finalizer for raw/legacy/JoinIR routes.

Benefits:

```text
canonical route can advance sooner
legacy compatibility remains available
```

Costs:

```text
two completion semantics
two parity surfaces
legacy repair becomes difficult to retire
route selection determines correctness timing
future bugs can hide in the less strict finalizer
```

This is rejected unless a separately approved compatibility lifetime and hard
deletion date exist. The current clean-architecture goal should not establish
two permanent finalization models.

## Exact decisions requested

Please confirm or revise these five decisions together.

1. **Final responsibility**

   Finalization permanently admits only representation-preserving Normalize,
   final Verify, and one-way derived Publish. An optional read-only precondition
   verifier may run before Normalize, but the published draft must be verified
   after its final mutation. Repair and legacy inference must reach zero at the
   finalization boundary; repository-wide retirement is a later terminal gate.

2. **Lifecycle publication**

   Function/module insertion is recorded as lifecycle commit outside the
   semantic pass taxonomy, rather than forcing it into
   `PublishDerivedArtifact`.

3. **Mixed-pass decomposition**

   Inventory rows describe semantic child operations, not facade function
   names. A facade with multiple classes must be split before CUT0.

4. **Legacy PHI containment**

   Split verifier/normalizer/repair first. Use a temporary candidate-clone
   transaction only if repair must survive the immediate producer migration;
   do not make transactional repair the permanent lifecycle.

5. **Owner routing**

   Keep FINALIZE0 focused on the Builder completion boundary. Send metadata
   fixpoints to METAPROP0, legacy call/weak/array/extern reconstruction to
   PLAN0/RAWADAPT0, and RC insertion to Ownership SSA retirement.

## Recommended task order after acceptance

### `FINALIZE0-CENSUS0-SCHEMA0` — sole next code-facing row

```text
behavior delta:
  0

artifact delta:
  schema v2 inventory
  validator derives its output from rows

required fields per semantic operation:
  owner
  production invocation sites/count
  input authority
  outputs
  first publication site
  mutation class
  failure timing/atomicity
  lowering consumers
  downstream consumers
  disposition
  retirement owner/dependency
```

Required corrections include:

```text
add the three missing operations
split mixed function/module operations
split optimizer/contract/semantic facades into children
remove fixed production_consumers=0 output
permit retain / split / retire-after / external-owner dispositions
count all three pipeline and all three PHI-materializer production sites
```

### `FINALIZE0-CENSUS0-P0`

Freeze the corrected repository-wide inventory and prove that every production
site belongs to exactly one semantic row.

### `FINALIZE0-VERIFY-SPLIT0`

Split `verify_typed_values_are_defined` into one read-only completed-draft
verifier and one explicit transient stale-row normalization owner. Run the
normalization before the Builder type/origin snapshot, then run the final
verifier over the exact draft that will be published.

### `FINALIZE0-TYPEPIPE-SPLIT0`

Split the current FieldGet, Copy, BinOp, and PHI repair facade into separately
counted compatibility children without changing their behavior. This is a
BoxShape refactor series and admits no new type fact.

### `FINALIZE0-CALLAWAIT-SPLIT0`

Split exact transfer, explicit-Unknown compatibility, name/catalog inference,
and constructor/known-box origin publication. Preserve exact failure/order
behavior while giving every child one removal dependency.

### `FINALIZE0-PHI-SPLIT0`

Create one read-only PHI edge verifier, one unused-PHI normalization owner, and
one isolated legacy repair owner. No new PHI admission is added.

The read-only completion verifier runs before unused-PHI pruning so that
normalization cannot erase a malformed completed PHI and create a false green.

If legacy repair cannot be removed immediately, a subordinate
`FINALIZE0-PHI-REPAIR-TX0` must prove failure leaves current MIR, facts, CFG,
and metadata unchanged.

### Producer closure series

Proceed in dependency order, reusing existing FACT0/PHI0 owners rather than
opening a second fact system:

```text
FINALIZE0-FIELD-CLOSE0
  every typed FieldGet publishes its result before return

FINALIZE0-CALLAWAIT-CLOSE0
  each Callee/Await family seals Exact/ExplicitUnknownAllowed/NoValue and any
  applicable origin obligation

FINALIZE0-COPY-CLOSE0
  every physical Copy producer seals its result disposition

FINALIZE0-BINOP-CLOSE0
  every physical BinOp producer seals its result disposition

FINALIZE0-PHI-CLOSE0
  every canonical PHI is physically complete before finalization
```

Each row removes only its own repair dependency after producer/failure/parity
proof. It does not wait for one all-or-nothing CUT0.

### `FINALIZE0-RETURN0`

Project function and main return signatures from sealed completion/return
contracts. Delete MIR/name inference only after typed, Void, fallthrough, PHI,
and error parity is green in debug and release.

### `FINALIZE0-DERIVED0`

Move the Builder transient type/origin snapshot after Builder structural
normalization and prove one-way publication. Add an explicit optimizer
coherence row: optimizer/canonicalizer mutation either updates metadata in the
same successful transaction or consumes a distinct sealed input artifact.
Partition semantic-refresh children by FINALIZE0, METAPROP0, PLAN0/RAWADAPT0,
and Ownership SSA owner.

### `FINALIZE0-CONDITIONFN-RET0`

Retire or re-home the synthetic `condition_fn` compatibility producer. It must
not remain an unnamed LegacySemanticInference exception at FINALIZE0-G0.

### `FINALIZE0-P0`

Required parity is broader than map equality:

```text
normalized MIR/CFG/PHI
transient versus metadata facts
function signatures
route and contract products
optimizer inputs
VM/interpreter behavior
backend/JSON-visible outputs
debug/release failure parity
fresh compiler/session reuse
```

Canonical Binding-SSA fixtures must observe zero repair changes.

### `FINALIZE0-CUT0`

Delete one repair/inference family per commit after its own producer closure.
Do not combine type, PHI, return, semantic-refresh, and JoinIR retirement in
one cut.

### `FINALIZE0-G0`

Builder lifecycle claims, scoped exactly to `finalize_function_draft` and
`finalize_module`:

```text
RepairMissingLoweringFact rows in Builder lifecycle = 0
LegacySemanticInference rows in Builder lifecycle = 0
TypePropagationPipeline Builder-lifecycle sites = 0
materialize_all_phi_inputs Builder-lifecycle repair sites = 0
Builder-lifecycle-first concrete type/origin publishers = 0
MIR/name-to-source semantic recovery in Builder lifecycle = 0
Builder transient type/origin snapshot families = exactly one
debug/release semantic divergence = 0
```

The JoinIR VM converter/apply sites, METAPROP0 fixpoints, PLAN0/RAWADAPT0
canonicalization, compiler post-build schedule, and ownership schedule are
explicit external dependencies. Their current physical placement inside
`finish_built_module` is not claimed clean by this local G0. They do not block
the Builder-lifecycle FINALIZE0-G0, but they join at:

```text
MIRBUILDER-CLEAN0-REPAIR-RET0-G0
  after METAPROP0
  after PLAN0 / RAWADAPT0
  after JoinIR D-prime succession retirement
  after Ownership SSA legacy retirement
```

Only that terminal guard may claim repository-wide
`TypePropagationPipeline = 0`, `materialize_all_phi_inputs = 0`, and global
legacy semantic inference zero.

## Stop conditions

Stop the selected implementation row if any of these is required:

1. Finalized metadata is read back as a lowering-time fallback.
2. A repaired fact is copied into a new persistent ValueId fact map.
3. A concrete type/origin or required explicit disposition is first published
   by finalization.
4. An already-concrete fact is silently overwritten.
5. Source target/field/route meaning is reconstructed from MIR spelling,
   names, spans, or runtime tags.
6. PHI repair mutates current state before all possible failures are known.
7. A Builder transient metadata snapshot occurs before later Builder-owned
   structural mutation, or a post-Builder mutator changes MIR without
   transactionally maintaining/republishing its own metadata artifact.
8. A mixed facade is accepted as one semantic classification.
9. Canonical and legacy routes receive permanent separate finalizers.
10. Failure retries another lowering/finalization route.
11. CUT0 is implemented before schema-v2 census and parity are green.
12. One commit mixes more than one repair family or mixes BoxShape with new
    source-language admission.
13. A source/check file reaches 800 lines.

## Proposed decision lock

> **Candidate A-prime is selected. Finalization is permanently a completed-
> draft normalization, final verification, and one-way
> derived-publication boundary; primary draft/module insertion is a lifecycle
> commit outside that semantic pass taxonomy. RepairMissingLoweringFact and
> LegacySemanticInference are explicit migration debt and must reach zero at
> this boundary, then join the later repository-wide retirement gate. Inventory
> rows classify semantic child operations, never mixed facade names. Every
> required result disposition is sealed by its successful physical producer,
> including explicit dynamic/Unknown and no-value cases; return signatures come from sealed
> completion contracts rather than MIR/name scans. The existing PHI repair is
> split into read-only verification, unused-PHI normalization, and isolated
> legacy repair; candidate-clone atomicity is temporary containment only when
> that repair must survive producer migration. Metadata snapshots occur once
> after Builder-owned structural normalization and never backfeed lowering;
> later mutators own coherent metadata updates or distinct derived artifacts. Metadata
> fixpoints move to METAPROP0, legacy call/field/extern reconstruction to
> PLAN0/RAWADAPT0, including array canonicalization, and RC insertion to
> Ownership SSA retirement. The sole next
> code-facing row is FINALIZE0-CENSUS0-SCHEMA0; CUT0 remains forbidden until
> schema-v2 census, producer closures, return-contract projection, and broad
> parity are green, after which repair families are removed one at a time.**
