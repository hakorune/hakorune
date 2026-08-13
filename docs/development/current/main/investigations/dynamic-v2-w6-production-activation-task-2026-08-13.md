---
Status: active design task
Date: 2026-08-13
Scope: selected Dynamic W6 production activation, CatalogedBoxMethod drain,
  Boundary C-ABI CheckedCallOut lowering, static post-link artifact receipt,
  and atomic selected-edge replacement
Exception: the 900-line predecessor mixes closed D0/E chronology with the new
  backend/publication boundary; this bounded successor replaces that stale
  restart surface instead of appending another ordinary row to it
ParentCurrentCard: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
Predecessor: docs/development/current/main/investigations/dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md
ParentHistory: docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
---

# Dynamic V2 W6 production activation task

## Current capsule

The selected Dynamic semantic program, A-prime demand, catalog physical
header, TextScan admission, strict runtime entries, checked CallOut ABI,
generation-aware lease owner, canonical CheckedCallOut MIR vocabulary,
site-id transport, full unpublished physical session, exact-two DraftSeal,
and branded `CatalogedBoxMethod` collector canary are landed.

Production remains closed:

```text
selected old raw AST/JoinIR edge                         = 1
selected Dynamic production caller                      = 0
Boundary C-ABI CheckedCallOut production physicalizer   = 0
static post-link artifact receipt production issuer     = 0
Rust-VM DynamicV2 production consumer                   = 0
```

The remaining work is not another semantic redesign. It is the final
authority-preserving connection from the completed MIR lane to the actual
default AOT backend and the retirement of the old selected edge. Two physical
publication owners are still missing from the implemented graph:

1. the existing normal collector drain does not admit
   `FunctionDraftKeyV1::CatalogedBoxMethod` in its mixed legacy/cataloged
   mode;
2. no production owner links to a temporary executable, observes the static
   artifact, issues a link receipt, and publishes the verified executable.

The existing test-only `RuntimeExecutablePlanV1` is not that owner. It requires
runtime function addresses, while the Boundary AOT lane emits direct symbol
calls into a statically linked executable. ASLR-era runtime addresses and
provider-image lifetime are not part of this W6.

Until those two boundaries are fixed, the complete W6 main landing is
`NoSafeSlice`.

## `DYNAMIC-V2-W6-PUBLICATION-BOUNDARY-D0`

```text
Decision:
  Keep compiler MIR-module publication and backend executable publication as
  two ordered transactions. Fix the CatalogedBoxMethod mixed normal drain, use a
  static link receipt rather than RuntimeExecutablePlan, and name one
  temporary-artifact observation/publication owner before W6 code starts.
Source authority + canonical issuer:
  The existing selected package/admission loan, canonical DraftSeal,
  ModuleDraftCollector/ModuleBuilderInvocationSession, published MirModule,
  Boundary C-ABI driver, explicit link observation, and one static artifact
  receipt issuer in that order.
Non-authority:
  The test-only W6 canary, Python/llvmlite, native replay driver, Rust VM,
  LegacySymbol/FreeStatic conversion, selector/name lookup, and a
  RuntimeExecutablePlan, runtime function addresses, `dlopen`/`dlsym`, and a
  caller-copied PlanStamp presented as an observed artifact fact.
Fail-fast boundary:
  Catalog key/brand/symbol/arity drift rejects the MIR candidate; malformed
  site/ABI/wire/PlanStamp, link failure, digest/symbol/descriptor drift, or a
  missing/duplicate defined strict symbol rejects executable publication.
  Neither failure may enter fallback.
Smallest next slice:
  Accept this publication split, the exact mixed normal collector drain, and the
  explicit link ABI plus static artifact observation/publication owner; then
  execute the BoxShape source split.
Non-claims:
  No production caller, LLVM/C physicalizer, executable publication, old-edge
  deletion, fallback/retry, or VM parity is authorized by this D0.
```

### Why the publication boundary is two transactions

The global pipeline SSOT is normative:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower
        -> Seal -> Collect -> Atomic MIR Publish
        -> Backend Boundary
```

Therefore W6 must not invent one cross-process transaction in which the
backend links an executable before the compiler is allowed to publish its
`MirModule`. The clean end-to-end order is:

```text
Compiler transaction
  selected package loan
  -> full unpublished physical session
  -> exact-two DraftSeal
  -> CatalogedBoxMethod collector preflight
  -> mixed normal collector drain
  -> candidate MirModule
  -> atomic MirModule publish once

Backend transaction
  published MirModule + canonical site-id metadata
  -> default Boundary C-ABI lowering
  -> temporary object
  -> explicit --nyrt link into temporary executable
  -> executable/archive digest + defined symbol + embedded descriptor check
  -> StaticLinkedAotArtifactReceiptV1 issue
  -> atomic executable artifact publication once
```

Link failure publishes no executable and no static artifact receipt. It does
not turn a valid published `MirModule` into a partial compiler publication. If a future
product requires rollback across both processes, that requires a separately
named higher-level build invocation transaction; W6 does not fabricate one.

The phrase “same activation commit” means that the repository production
graph changes atomically:

```text
new selected compiler caller = 1
old selected raw/JoinIR edge = 0
```

It does not merge the two runtime publication authorities.

### D0 exit decisions

This D0 is accepted only when all four representation/owner choices are
explicit. Merely naming a future receipt is not enough.

#### 1. Cataloged Box-method mixed normal-drain projection

Preferred shape:

```text
NormalCatalogedBoxMethodDraftAdmissionV1
+ completed exact-two draft
+ ModuleInvocationBrandV1
  -> one move-only CatalogedBoxMethodDrainProjectionV1
     key = CanonicalSameModuleCallableKeyV1
     symbol + arity = moved admission projections
  -> existing normal collector drain lifecycle
     (mixed legacy + cataloged terminal)
```

This projection is a physical transport of the retained admission, not a new
catalog/source identity. Extend the existing
`PreparedNormalCollectorDrainLifecycleV1`/`ModuleDraftCollectorV1` path with a
mixed terminal: `LegacySymbol` rows keep `LegacyReplaceWholePair`, while the
cataloged projection uses `CanonicalRejectDuplicate` and inserts the exact
`FunctionDraftKeyV1::CatalogedBoxMethod` row. The compiler-level
`CanonicalPhysicalDrainManifestV1` remains unchanged and must not gain a
CatalogedBoxMethod variant; the normal selected package path does not own its
canonical-source continuation family.

A free-static callable row, `LegacySymbol` conversion, reconstructed
name/arity key, second collector, or a second module transaction is rejected.

#### 2. Explicit Boundary link call ABI

The current C boundary obtains the runtime archive through
`NYASH_EMIT_EXE_NYRT`. W6 must not treat a temporary environment override as
the final exact-artifact authority. Freeze a revised link call ABI that passes
the exact archive path as an argument, for example:

```text
hako_llvmc_link_obj_v2(
  object_path,
  temporary_executable_path,
  runtime_archive_path,
  extra_link_flags,
  error_out,
)
```

The CLI `--nyrt` resolution remains the caller-side source. The C linker does
not rediscover it from an environment variable or fallback directory.

#### 3. Artifact-bound descriptor

ABI revision, wire revision, and PlanStamp cannot be copied from expected
input into an “observed” struct. Select the generated-object representation;
the sidecar alternative is not part of W6:

```text
selected: generated-object descriptor symbol retained in the executable
         in a dedicated non-discarded descriptor section
rejected: digest-co-sealed sidecar emitted and renamed with the executable
```

The selected descriptor binds exact entry IDs/symbols, ABI/wire revisions, and
the compile invocation brand to the linked artifact. The link verifier reads
the retained descriptor symbol after link and rejects missing, duplicate, or
foreign descriptors. The descriptor is a compile fact; executable and runtime
archive digests remain post-link observations in the static receipt.

#### 4. Candidate cleanup and publication owner

Name one owner in `crates/nyash-llvm-compiler/src/link_driver.rs`:

```text
StaticAotArtifactPublicationTxnV1
```

It owns:

```text
temporary object
temporary executable
retained object descriptor observation
failure cleanup
final atomic rename
StaticLinkedAotArtifactReceiptV1 issue
```

No file at the requested final executable path is replaced until every
post-link observation is green. A failure leaves the prior final artifact
unchanged.

## Actual production backend owner

The daily `ny-llvmc` default is `DriverKind::Boundary`. The production chain
is:

```text
Rust MIRBuilder
  -> MIR JSON + CheckedCallOut site-id metadata
  -> ny-llvmc --driver boundary
  -> hako_llvmc_compile_json_pure_first
  -> lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  -> LLVM IR / object
  -> link_driver + explicit runtime archive
  -> executable
```

Accordingly:

- the Boundary C-ABI pure-first lowering is the sole W6 CheckedCallOut
  physicalizer;
- `src/llvm_py/**` remains a transport/test/compatibility view;
- `--driver native` and `--driver harness` remain keep/canary lanes;
- Rust VM remains an explicit DynamicV2 nonconsumer.

LLVM/C emits the physical conditional branches named by MIR. It does not
choose Normal/Fault meaning, successors, provider, selector, result class, or
cleanup chronology.

## Accepted authority graph

```text
CoreMethodContractBox generated rows
  -> retained I6/I7 call relations
  -> normalized TextScan complete-role contract
  -> ProviderAdmissionSeal
  -> PreparedAotExecutableAdmissionV1
  -> CheckedCallOut function-local site plans
  -> canonical site-id MIR/JSON metadata
  -> Boundary C-ABI physicalizer
  -> post-link observed static artifact facts
  -> StaticLinkedAotArtifactReceiptV1

VerifiedDynamicExitTransactionCoSealV1
  -> A-prime physical demand
  -> one physical operation/control/cleanup session
  -> canonical CFG/SSA/PHI/Completion close
  -> exact-two DraftSeal
  -> CatalogedBoxMethod collector
  -> mixed normal collector drain
  -> atomic MirModule publication
```

No downstream owner relooks up generated rows, selector, provider, image, or
registry. Site ID is the only backend call-site locator; old
`block + instruction_index` remains source/preflight evidence only.

## Ordered implementation DAG

The following rows are work-branch checkpoints inside one W6 series. Except
for the behavior-preserving source split, none may land on main as a selectable
production fragment. The production graph changes only at W6-E.

### W6-S — `DYNAMIC-V2-W6-SOURCE-SPLIT-R0` (BoxShape)

Split the two over-budget implementation surfaces without changing paths,
visibility, issuers, callers, instruction order, or accepted shapes.

```text
src/mir/checked_callout.rs (778)
  -> checked_callout.rs              facade/re-exports
  -> checked_callout/site_plan.rs    IDs, shape, plan, JSON projection
  -> checked_callout/census.rs       function 1:1:1 verifier
  -> checked_callout/tests.rs        focused tests

src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/callout_corridor.rs (731)
  -> callout_corridor/mod.rs         corridor model/orchestration
  -> callout_corridor/emission.rs    I0-I7 emission body
```

Acceptance:

```text
external Rust module paths and visibility unchanged
CheckedCallOut site-plan pair issuer                    = 1
function census verifier                                = 1
canonical CFG/SSA issuer                                = 1
corridor emission entry                                 = 1
each touched activation source                          < 650 target
any activation source                                   < 700 mandatory gate
all Rust source                                         < 800 hard stop
```

Observed closeout: the split is landed with checked-callout sources at
14/360/197/245 lines and corridor sources at 139/598 lines; focused
`checked_callout` (14) and `selected_dynamic_physical_emitter` (3) tests,
`cargo check --lib`, and the current-state, activation, physical-input, VM,
and pre-cutover guards are green. Production counts remain new=0, old=1.

Do not combine this BoxShape with evidence semantics, backend vocabulary, or
the production caller switch.

### W6-A1 — `DYNAMIC-V2-W6-A1-PHYSICAL-EVIDENCE-CURSOR-R0`

Close only the physical evidence already retained by the unpublished session.
This is the safe first part of W6-A; AOT backend handoff remains closed until
the static artifact owner is implemented.

```text
compare demand              = move-only, consumed by I9 exactly once
cleanup rows                = private cursor, issue=1 / claims=4 / close=1
physical census             = 15 operations + 1 If + 1 Exit, claim/close once
profile/DraftSeal before close = reject
AOT artifact/backend handoff = 0
new production caller       = 0
selected old edge           = 1
```

The cursor may only consume existing A-prime/Recipe/cleanup evidence. It may
not issue semantic rows, reissue site plans, reinterpret `RejectBeforeEffect`,
or introduce a provider, LLVM, RuntimeExecutablePlan, VM, fallback, or retry
consumer.

Focused acceptance requires exact End locations (`I7 Fault`, `ThenTerminal`,
`Continuation`) and rejects missing, duplicate, foreign, reordered, or
unclosed evidence before profile close and DraftSeal.

Observed W6-A1 closeout: the move-only compare consumer, four-row cleanup
cursor, ordered 15-operation/If/Exit cursor, and profile-before-close fence are
implemented. Focused selected-emitter tests (4), `cargo check --lib`, the
current-state/AOT/physical-input/VM/pre-cutover guards, and `git diff --check`
are green; AOT/backend handoff remains 0 and production counts remain new=0,
old=1.

### W6-A — `DYNAMIC-V2-W6-EVIDENCE-CONSUME-R0`

Close the physical session evidence that is currently retained and then
silently dropped.

#### Compare demand

Move the existing `DynamicV2CompareI64CapabilityDemandV1` into the I9
physical emitter. It must co-check the already-issued I9 row, V11/V12 producer
families, `ImmediateI64` representation, and non-faulting compare, then become
unavailable.

#### Cleanup cursor

Replace the borrowed four-row cleanup array with one session-private,
move-only consumption cursor:

```text
I6 Fault       -> consume no-End row
I7 Fault       -> consume End(V10) row
Inner Return   -> consume End(V10) row
Backedge       -> consume End(V10) row
close          -> no row remains
```

Profile close and DraftSeal are forbidden before cursor close. The terminal
must verify location as well as total count:

```text
I7 Fault / ThenTerminal / Continuation each contain End exactly once
all other blocks contain End zero
```

#### Operation/control/exit close

Co-check the existing physical projections as one complete census:

```text
15 operation rows + 1 If control + 1 Exit = 17 exact items
```

This is a physical consumption terminal, not a second semantic program.
Operation order remains Recipe-issued; If/Exit meaning remains owned by the
existing control and Completion products. Missing, duplicate, orphan, or
reordered items reject before publication.

#### Disposition and AOT handoff

`RejectBeforeEffect` must be explicitly consumed by the unpublished canary
fence. It must never be reinterpreted as executable success. After DraftSeal,
the pre-link AOT admission is moved into the backend handoff instead of being
dropped. Executable-ready state can be issued only from the static post-link
artifact owner selected by D0.

Acceptance:

```text
compare demand consumer                                 = 1
cleanup cursor issue / consume / close                  = 1 / 4 / 1
17-item physical census                                 = 1
unpublished RejectBeforeEffect fence                    = 1
AOT admission backend handoff                           = 0 (deferred to W6-D)
site-plan table reissue                                 = 0
pre-link static artifact receipt                        = 0
production caller                                      = 0
```

### W6-B — `DYNAMIC-V2-W6-CATALOGED-DRAIN-I0`

Extend the existing normal collector drain to consume the already-defined
`FunctionDraftKeyV1::CatalogedBoxMethod` row alongside existing legacy rows.
Do not add a CatalogedBoxMethod arm to the compiler-level
`CanonicalPhysicalDrainManifestV1`; do not create a second collector, module
transaction, or key family.

The drain co-checks:

```text
selected catalog key
+ physical symbol and arity
+ invocation brand
+ completed exact-two draft
+ whole-batch collision census
  -> existing mixed normal collector-drain terminal
  -> existing candidate MirModule transaction
```

The selected projection is move-only and is issued from the completed
cataloged draft/admission pair. The mixed terminal accepts exactly
`LegacySymbol` plus the cataloged row for this bounded cohort:

```text
LegacySymbol       -> LegacyReplaceWholePair
CatalogedBoxMethod -> CanonicalRejectDuplicate + exact symbol/arity
CanonicalCallable/Main/arbitrary -> reject in this terminal
```

Forbidden conversions:

```text
CatalogedBoxMethod -> LegacySymbol
CatalogedBoxMethod -> FreeStatic CanonicalCallable
CatalogedBoxMethod -> reconstructed name/arity key
```

Acceptance:

```text
CatalogedBoxMethod mixed normal-drain owner              = 1
collector handoff / normal-drain consumption             = 1 / 1
legacy/free-static conversion                           = 0 / 0
second finalization or second function session          = 0
failure before module commit leaves live Builder equal  = green
production selected caller                              = 0
```

Negative cases include foreign/unbranded receipt, key or symbol/arity drift,
duplicate key/symbol, cataloged row paired with `LegacyReplaceWholePair`,
CatalogedBoxMethod -> LegacySymbol/CanonicalCallable conversion, replacement
disposition drift, symbol-index drift, missing projection, projection-count
mismatch, and a second drain attempt.

Observed closeout (2026-08-13): the existing normal drain now accepts the
mixed `LegacySymbol` + `CatalogedBoxMethod` rows with their original policy,
brand, key, symbol, and arity. Focused lifecycle tests (7 passed), `cargo
check --lib`, current-state, AOT/physical-input, VM-fence, pre-cutover, and
in-place-replacement guards are green; production remains `new=0`, `old=1`.

### W6-C — `DYNAMIC-V2-W6-BOUNDARY-CALLOUT-I0`

Add the selected CheckedCallOut physicalizer to the actual default backend:

```text
lang/c-abi/shims/hako_llvmc_ffi_checked_callout_lowering.inc
  included once by hako_llvmc_ffi_pure_compile.inc
```

The Rust JSON call-in transports canonical `site_id` metadata once. The C
lowerer consumes the exact admitted entries and emits:

```text
I6 substring: EndAuthorizedHandle + nonzero lease on Normal
I7 indexOf:   ImmediateI64 + lease zero on Normal
semantic Fault: branch to MIR fault landing
invalid transport / malformed wire / unexpected Suspended:
  backend fail-stop or trap, no semantic successor, no fallback
```

Normal/Fault target identity comes only from MIR. The C lowerer may emit the
physical branches but cannot invent a successor or reclassify a wire failure
as semantic Fault.

Acceptance:

```text
Boundary C-ABI CheckedCallOut physicalizer               = 1
site-id -> admitted entry mapping                        = exact 2
I6/I7 ABI and normal shape                               = exact
Python/native/harness/VM production consumer             = 0
selector/name/provider/runtime registry lookup           = 0
generic method fallback / retry                          = 0 / 0
```

### W6-D — `DYNAMIC-V2-W6-STATIC-LINK-RECEIPT-D0 -> I0`

Do not promote the test-only `RuntimeExecutablePlanV1`. The static Boundary
lane directly calls linked symbols and has no runtime function-pointer table,
dynamic provider image, or `dlsym` consumer. Keep that type parked until a
real dynamic-image execution route exists.

The D0 first fixes one Rust link-invocation owner and these observation
sources:

```text
temporary output path and atomic final rename owner
runtime archive and executable digest owner
defined-symbol census owner
ABI / wire / PlanStamp artifact descriptor or paired-manifest owner
candidate cleanup on every failure
```

It also replaces the environment-mediated `hako_llvmc_link_obj` archive
selection with the explicit link ABI chosen by the D0. Compatibility callers
may remain behind a named keep boundary, but the selected W6 route has exactly
one explicit archive-path call and zero environment rediscovery.

ABI, wire, and PlanStamp must be observed from an artifact-bound descriptor or
manifest emitted by the same compilation, not copied from the expected input
and called “observed”. Once this representation is accepted, the I0 link owner
validates:

```text
temporary executable path and final destination
executable digest
exact runtime archive path and digest
required entry IDs and uniquely defined symbols
artifact symbol values as diagnostics, never runtime addresses
call ABI revision / wire revision
carried ModuleInvocationBrandV1 PlanStamp
```

`StaticAotArtifactPublicationTxnV1` is the only selected W6 owner for the
temporary object/executable paths, descriptor observation, failure cleanup,
receipt issuance, and final atomic rename. Only a complete observation issues one move-only
`StaticLinkedAotArtifactReceiptV1` and prepares executable publication.
Missing, duplicate, stale, foreign, or mismatched observations delete or
abandon the candidate and publish nothing executable.

Acceptance:

```text
post-link static artifact receipt issuer                 = 1
exact runtime archive input                              = 1
explicit Boundary link ABI / env archive rediscovery    = 1 / 0
archive/exe digest + symbol + descriptor verification    = green
runtime address/image-lifetime fabrication               = 0
RuntimeExecutablePlan / dlsym / provider reselection     = 0 / 0 / 0
link failure executable publication                      = 0
executable final-path publication before W6-E            = 0
```

### W6-E — `DYNAMIC-V2-AOT-ACTIVATION-I0-W6`

This is the only production BoxCount. In one activation commit:

```text
selected package production callback                    0 -> 1
selected old raw AST/JoinIR edge                         1 -> 0
CatalogedBoxMethod collector/mixed normal drain          = 1
atomic MirModule publication                             = 1
Boundary C-ABI CheckedCallOut physicalizer               = 1
static artifact receipt / executable publication         = 1 / 1
RuntimeExecutablePlan issuer/install                     = 0 / 0
LegacySymbol / FreeStatic conversion                     = 0 / 0
runtime lookup / generic fallthrough / fallback / retry  = 0 / 0 / 0 / 0
Rust-VM DynamicV2 production consumer                    = 0
```

The cutover callback consumes the existing package loan exactly once. It may
not re-open raw AST lowering, call `lower_loop_or_freeze_v1`, retry another
route, or retain a dual-production branch. The activation commit is merged
only when every terminal count is green. After cutover, a per-request compiler
or link failure remains fail-fast and must never revive the deleted old edge.

## Negative matrix

```text
foreign selected package/admission or invocation brand       -> RejectBeforeEffect
CatalogedBoxMethod key/symbol/arity/brand drift               -> discard MIR candidate
missing/duplicate/extra operation, If, Exit, cleanup row      -> discard session
compare demand missing, reused, or wrong representation       -> discard session
End in wrong block or duplicate/stale lease                   -> discard session
orphan/duplicate site plan, terminator, Normal projection     -> discard session
site-id role/entry/shape swap                                 -> backend reject
unknown ABI/wire/effect/status/result tag                     -> backend fail-stop
Suspended from non-suspending TextScan                        -> backend fail-stop
missing/foreign runtime archive path or env rediscovery       -> static-link reject
missing/duplicate/undefined linked symbol                      -> static-link reject
stale archive/executable digest or artifact descriptor         -> static-link reject
link failure followed by executable/plan publication          -> guard failure
legacy/free-static key conversion                             -> guard failure
old and new selected production callers both nonzero          -> guard failure
provider/selector/image runtime relookup                      -> guard failure
fallback, retry, generic String, or VM DynamicV2 consumer     -> guard failure
```

## Focused gates

Run per internal row, then run the full set before W6-E:

```bash
cargo check -q --lib
cargo test -q --lib normal_callable_semantic_package
cargo test -q --lib dynamic_full_body_recipe
cargo test -q --lib selected_dynamic_physical_emitter
cargo test -q --lib completion

bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dynamic_v2_physical_input_authority_guard.sh
bash tools/checks/dynamic_v2_callslot_wire_authority_guard.sh
bash tools/checks/dynamic_v2_vm_nonconsumer_fence_guard.sh
bash tools/checks/loop_precutover_authority_guard.sh
bash tools/checks/dynamic_v2_aot_activation_authority_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The focused `completion` suite has a known parent-baseline
`ReturnValueTypeMissing(ValueId(12))` failure. Reproduce it at the recorded
parent before classifying it as baseline debt; every new failure remains
blocking until classified.

## Guard ratchets

Extend existing guards rather than creating one guard per file.

```text
checked_callout and corridor files                       < 650 target
activation owner file                                   < 700 hard task gate
all Rust source                                          < 800 repository hard stop
site-id AOT lookup consumer                              = canonical call relation only
CheckedCallOut plan-table install                        = 1
compare/cleanup/AOT evidence terminal                    = 1 / 1 / 1
Boundary C physicalizer                                  = 1
Python/native/harness/VM production physicalizer         = 0
CatalogedBoxMethod mixed normal drain                    = 1
static artifact receipt post-link issuer                 = 1
RuntimeExecutablePlan production issuer                  = 0
selected new/old production caller before W6-E           = 0 / 1
selected new/old production caller at W6-E               = 1 / 0
```

## Post-cutover queue

Only after W6-E caller-zero evidence:

1. delete the selected raw JoinIR edge, legacy finalizer remnants, reject-only
   canary shells, and stale block/index AOT locator definitions;
2. convert the physical operation cursor into typed move-only leaf views only
   if the W6-A 17-item terminal still leaves duplicate row interpretation;
3. split `emit_instruction` private phases while keeping one public writer;
4. classify `builder.rs` modules after production caller/cfg census;
5. distinguish per-site physical effect from function-wide union before any
   provider cohort beyond the sealed pure-read TextScan pair;
6. keep `CURRENT_STATE.toml` compact and archive landed W6 receipts rather
   than appending chronology to this card.

## Closeout template

```text
Decision actually implemented:
Production new/old caller census:
Compiler publication owner and count:
Backend static artifact receipt/publication owner and count:
Evidence consume counts:
Negative matrix result:
Focused gates and classified reds:
Line counts:
Commit / push:
Remaining non-claims:
```
