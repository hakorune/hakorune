---
Status: Decision accepted
Decision: Candidate A-prime with five final-boundary clarifications
Date: 2026-07-20
Scope: FINALIZE0 final responsibility boundary and producer-first retirement order
Parent: docs/development/current/main/investigations/mirbuilder-finalize0-census-task-2026-07-20.md
Repository: https://github.com/hakorune/hakorune
Branch: public-main
Source basis: 5220d7a307d7de5796a2f66d9239914c8fd92e7f
Initial consultation commit: f3be911a1f09ab7f08631f0c348387db20792891
Decision-response basis: c2d7e28dfc11d82de1e919b33ce48acf1baefe09
---

# FINALIZE0 boundary decision

## External reproduction

The source census was performed against the exact source-basis commit above.
The consultation document was first published by the initial consultation
commit and remains current on `public-main`.

```bash
git clone --branch public-main https://github.com/hakorune/hakorune.git
cd hakorune
git cat-file -e 5220d7a307d7de5796a2f66d9239914c8fd92e7f^{commit}
git show 5220d7a307d7de5796a2f66d9239914c8fd92e7f:src/mir/builder/module_lifecycle.rs
git show public-main:docs/development/current/main/investigations/mirbuilder-finalize0-boundary-consultation-2026-07-20.md
```

To inspect the public branch without cloning, query the ref rather than passing
a commit hash as an `ls-remote` pattern:

```bash
git ls-remote https://github.com/hakorune/hakorune.git refs/heads/public-main
```

`git ls-remote <url> <commit-sha>` does not prove commit reachability; the final
argument is treated as a ref-name pattern. Fetch the branch and use
`git cat-file -e <sha>^{commit}` or `git merge-base --is-ancestor` for that
check.

## Decision

**Candidate A-prime is selected:** producer-first pure finalization with an
explicitly temporary legacy-repair quarantine.

The selected final form would be:

```text
lowering-time producers
  -> seal facts or issue explicit Pending completion tokens
  -> consume every Pending token
  -> seal every PHI edge and the all-exit return contract
  -> CompletedMirFunctionDraft

Builder finalization on an owned candidate
  -> VerifyNormalizationPreconditions
  -> NormalizeRepresentation
  -> PublishSealedArtifacts inside the candidate
  -> VerifyPublishedDraft
  -> AggregateDraftIntoCandidateModule

Builder-lifecycle repair/inference counts
  RepairMissingLoweringFact = 0
  LegacySemanticInference = 0
```

The question is not whether the current repair passes are useful. They are.
The question is whether they remain part of the permanent finalization model,
or are treated as migration debt whose facts move back to their physical
lowering producers.

The accepted decision adds five clarifications:

```text
1. the final verifier runs after candidate artifact publication
2. lifecycle transitions and diagnostic observations are outside the semantic taxonomy
3. every function/module transient fact session owns a fresh generation
4. return signatures project deterministically from all sealed exits
5. normalization preserves or atomically rewrites every identity and artifact reference
```

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

## Candidate A-prime — producer-first pure finalization (selected)

### Permanent boundary

Finalization permanently admits only:

```text
NormalizeRepresentation
PublishSealedArtifact
VerifyPublishedDraft
```

Primary function/module insertion is a lifecycle commit and is recorded
outside this three-class pass taxonomy. It does not become a fourth semantic
inference class.

Diagnostic observation is also outside the semantic taxonomy. A warning-only
or environment-gated scan cannot be counted as `VerifyPublishedDraft`.

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

Forward and cyclic completion use an explicit state machine rather than an
early `Unknown` publication:

```text
Reserved
  -> EmittedPending(completion token)
  -> Completed(Exact | ExplicitUnknownAllowed | NoValue)
  OR Aborted
```

Pending facts are invisible to dependent lowering consumers. Every token is
consumed before `CompletedMirFunctionDraft` is constructed.

### Transient fact session law

Every module/function fact session owns one generation.

```text
generation N ValueIds/facts visible in generation N+1:
  0

successful finalization:
  consumes or discards the transient session

failed finalization:
  restores or discards the candidate session without contaminating reuse
```

This is a correctness law, not only a reuse fixture. It prevents an old
same-numbered ValueId fact from causing `contains_key`-style false greens in a
later compilation using the same compiler.

### Return completion law

The return signature is projected from one sealed all-exit contract:

```text
ReturnCompletionContract
  explicit source return declaration
  every reachable Return exit disposition
  explicit return-void exits
  return-without-value exits
  implicit fallthrough disposition
```

Function and main implicit Return insertion belongs to completion producers,
not verification. No HashMap/MIR scan may select the first observed Return.

### Origin law

The census keeps separate rows for:

```text
semantic lowering origin:
  value_origin_newbox

diagnostic origin:
  value_origin_callers

published post-Builder semantic origin:
  exact metadata field, or explicit none
```

A finalization-time origin write with no consumer is retired rather than moved
to a new producer merely to preserve a dead transient fact.

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

`PublishSealedArtifacts` operates only on the owned candidate. Until every
publication constructor is proven pure and infallible,
`VerifyPublishedDraft` runs after publication and before aggregation/commit.
It checks MIR/metadata coherence, return-contract projection, artifact
revision/fingerprint, and transient-to-snapshot equality.

### Normalize identity and freshness law

A permanent normalization must prove all of:

```text
runtime semantics preserved
ValueId and BasicBlockId preserved, or all references rewritten atomically
instruction-site identity preserved, or all side tables rewritten atomically
no new lowering fact
no source/name/runtime-tag inference
derived artifact revision/freshness preserved
```

For temporary PHI repair, the candidate transaction covers the complete state,
not only `MirFunction`:

```text
MirFunction
transient type/kind/origin facts
CFG-derived state
function metadata
instruction spans and site-indexed artifacts
ValueId allocator state
```

The read-only PHI verifier builds its CFG view from terminators and does not
mutate a shared CFG cache.

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

## Accepted boundary decisions

The five consultation decisions are accepted with the clarifications below.

1. **Final responsibility**

   Finalization consumes `CompletedMirFunctionDraft`, not an already-verified
   draft. It runs an optional read-only precondition verifier, normalization on
   an owned candidate, sealed-artifact publication inside that candidate, and
   a mandatory final read-only verifier after the final publication mutation.
   Repair and legacy inference reach zero at the Builder boundary;
   repository-wide retirement is a later terminal gate.

2. **Lifecycle publication**

   Candidate aggregation, external commit, caller-context restore, region/slot/
   debug-scope close are lifecycle transitions outside the semantic taxonomy.
   Warning-only and environment-gated scans are diagnostic observations outside
   the taxonomy. Candidate aggregation and external publication remain distinct.

3. **Mixed-pass decomposition**

   Inventory rows describe child operations, not facade names. Schema v2 records
   operation domain, route reachability, publication kind, identity/freshness,
   invalidated artifacts, session generation, and bidirectionally verified
   production source sites.

4. **Legacy PHI containment**

   Split verifier/normalizer/repair first. The verifier uses a pure CFG view.
   A temporary repair transaction, if needed, owns MIR, facts, CFG state,
   metadata, spans/site artifacts, and allocator state; it is not permanent.

5. **Owner routing**

   Keep FINALIZE0 focused on the Builder completion/freshness boundary. Send metadata
   fixpoints to METAPROP0, legacy call/weak/array/extern reconstruction to
   PLAN0/RAWADAPT0, and RC insertion to Ownership SSA retirement.

## Selected task order

### `FINALIZE0-CENSUS0-SCHEMA0` — sole next code-facing row

```text
behavior delta:
  0

artifact delta:
  schema v2 inventory
  validator derives its output from rows

required fields per semantic operation:
  operation domain
  owner
  production invocation sites/count
  route/profile reachability
  source sites with path/symbol/operation/ordinal/cfg domain
  input authority
  outputs
  publication kind
  first publication site
  mutation class
  identity stability
  invalidated artifacts
  session generation
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
prove source-match -> one row and row-site -> one source match
```

`operation_domain` is one of:

```text
semantic_pass
lifecycle_transition
diagnostic_observation
```

Only `semantic_pass` requires a semantic class. Publication subkinds are:

```text
SealedFactSnapshot
PureProjection
RecomputableDerivedArtifact
DiagnosticArtifact
```

### `FINALIZE0-CENSUS0-P0`

Freeze the corrected repository-wide inventory and prove that every production
site belongs to exactly one semantic row.

### `FINALIZE0-VERIFY-SPLIT0`

Split `verify_typed_values_are_defined` into one read-only completed-draft
verifier and one explicit transient stale-row normalization owner. Run the
normalization before the Builder type/origin snapshot, then run the final
verifier over the exact draft that will be published.

The correctness verifier runs in every build mode. Strict/dev flags may add
diagnostics, but cannot select whether correctness is checked.

### `FINALIZE0-FACTSESSION0`

Introduce one fresh module/function fact generation and prove success, typed
failure, panic restoration, and same-compiler reuse. No ValueId type/kind/
origin fact from generation N may be observed in generation N+1. This row
precedes producer closure so stale facts cannot hide a missing producer.

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

Move function/main implicit Return materialization to completion producers.
Seal every reachable explicit value, explicit Void, no-value, and fallthrough
exit into `ReturnCompletionContract`; project the signature deterministically
from the whole contract. Delete MIR/name inference only after multi-exit, PHI,
typed, Void, fallthrough, and error parity is green in debug and release.

### `FINALIZE0-DERIVED0`

Move the Builder transient type/origin snapshot after Builder structural
normalization and prove one-way publication. Add an explicit optimizer
coherence row: optimizer/canonicalizer mutation either updates metadata in the
same successful transaction or consumes a distinct sealed input artifact.
Distinguish SealedFactSnapshot, PureProjection, RecomputableDerivedArtifact,
and DiagnosticArtifact; publish revision/fingerprint evidence and run the
final coherence verifier after candidate publication and before commit.
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
Pending completion tokens at draft close = 0
cross-generation transient facts = 0
debug/release semantic divergence = 0
scope = builder_finalize_function_and_module
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

## Decision lock

> **Candidate A-prime is selected with five clarifications. Builder
> finalization consumes a `CompletedMirFunctionDraft`, not an already-verified
> draft. Every lowering-visible result disposition is sealed by its physical
> producer or by an explicitly owned completion token consumed before draft
> completion. Implicit returns and the all-exit return contract are completion-
> producer responsibilities; return signatures are projected deterministically
> from all sealed exits and never selected by MIR/name scanning. Permanent
> semantic finalization admits only representation-preserving normalization,
> sealed-artifact publication, and read-only verification. Lifecycle transitions
> and diagnostic observations are outside the semantic taxonomy. Finalization
> operates on an owned candidate, verifies normalization preconditions,
> performs no repair or source-semantic reconstruction, publishes sealed
> snapshots or declared derived artifacts, verifies MIR/metadata freshness
> after the final publication mutation, and only then aggregates/commits.
> Normalize may not introduce lowering facts or invalidate ValueId, block,
> instruction-site, or metadata identities without an atomic coherent rewrite.
> Temporary PHI repair is isolated from verifier/normalizer and transactionally
> owns MIR, transient facts, CFG state, metadata, spans, site artifacts, and
> allocator state. Every module/function fact session has a fresh generation;
> no ValueId fact survives compiler reuse. Inventory rows separately record
> semantic, lifecycle, and diagnostic operations, exact production sites, CFG
> domain, route reachability, publication kind, invalidated artifacts, first
> publication, consumers, atomicity, session generation, and retirement owner.
> FINALIZE0-G0 claims zero repair/inference only inside Builder function/module
> completion; repository-wide zero belongs exclusively to the later
> `MIRBUILDER-CLEAN0-REPAIR-RET0-G0`. The sole next code-facing row is
> `FINALIZE0-CENSUS0-SCHEMA0`; repair families are removed individually only
> after producer, return-contract, freshness, failure, route, and broad parity
> proofs are green.**
