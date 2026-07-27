---
Status: active execution task map
Date: 2026-07-27
Decision: PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-prime-r1
Series: PRELOOP-STAGEB-PRODUCTION-CARRIER0
First executable row:
  - PRELOOP-STAGEB-SOURCE-INVENTORY0-P0
Related:
  - preloop-stageb-source-producer-selection0-d0-design-question-2026-07-27.md
  - preloop-stageb-carrier-handoff0-d0-design-question-2026-07-27.md
  - src/mir/source_call_target/
  - src/mir/preloop_stageb_carrier/
  - src/mir/compiler/mod.rs
Sunsets:
  - PRELOOP-STAGEB-LEGACY-ALIAS-MUTATION-SUNSET-001
  - PRELOOP-INNER-TYPE-PROOF-SUNSET-001
  - PRELOOP-STAGEB-LEGACY-SOURCE-PRODUCER-SUNSET-001 (long-term parked)
---

# Pre-loop Stage-B Production Carrier Task Map

## Outcome

This series gives one bounded Legacy source family one exact production
consumer without making Builder a source-policy owner:

```text
LegacyWholeSourceCompileRequestV1
  -> one pre-Builder source inventory
  -> Ordinary | Selected | typed rejection
  -> Selected consumes the same catalog allocation
  -> one selected instance-method function ingress
  -> exact inner Call receipt
  -> exact outer carrier Call receipt
  -> exact assignment completion receipt
  -> success-only outer Integer fact
  -> existing GenericLoop consumer
```

The series is a short sequence of buildable refactor/activation umbrellas.
Small cells inside an umbrella do not reopen design consultation. Stop and
open a new D0 only when one of the explicit evidence contradictions at the end
of this card is observed.

## Fixed authorities

```text
whole-source producer:
  PreloopStageBWholeSourceProducerV1

sole production consumer:
  MirCompiler::compile_request
  / MirLoweringRequestV1::Legacy arm

candidate law:
  zero  -> explicit Ordinary
  one   -> Selected
  many  -> typed pre-Builder rejection
  incomplete inventory or invalid authority
        -> typed pre-Builder rejection

ordinary boundary:
  complete inventory + bounded capability not proven
        -> Ordinary(BoundedProofUnavailable)

alias authority:
  CompilerSuppliedStaticImportSnapshotV1
  non-Clone owned None | Explicit(sorted aliases)
  retained by both Ordinary and Selected

selected catalog:
  sealed once for source proof
  installed from the same allocation
  never resealed in lower_root

ordinary catalog:
  existing lower_root seal/install remains unchanged

fallback:
  zero
```

## Umbrella A — whole-source inventory

### `PRELOOP-STAGEB-SOURCE-INVENTORY0-P0`

Purpose:

```text
one boxed declaration catalog allocation
+ one typed alias snapshot
+ existing source/result proof owners
-> exact complete candidate inventory
```

Suggested neutral layout:

```text
src/mir/source_call_target/
  whole_source_inventory.rs
  whole_source_inventory_error.rs
  whole_source_inventory_tests.rs

src/mir/preloop_stageb_carrier/
  source_producer.rs
```

`source_call_target` owns complete source-call observation and neutral target
facts only. `source_producer` alone composes Stage-B nested/outer contracts and
the 0/1/many policy.

Reuse:

```text
existing SourcePath / SourceExprSite vocabulary
existing method-call observation
VerifiedSourceMethodCallSiteV1
VerifiedStaticImportAliasViewV1
existing static target/result evidence
existing nested-instance Integer contract
existing outer-carrier Integer contract
```

Do not add:

```text
new AST walker
callee-name or Box-name selection
source ordinal reconstruction
Builder/MIR/runtime inspection
production caller
```

Focused matrix:

```text
no candidate                         -> complete inventory, count 0
one direct-owner exact candidate     -> count 1
one alias-selected exact candidate   -> count 1
two complete exact candidates        -> count 2
unrelated nested call                -> count 0
receiver has no supplied/direct owner-> complete noncandidate
supplied alias names missing owner    -> InvalidAliasSnapshot rejection
foreign equal-looking catalog        -> typed identity rejection
source declaration reorder           -> stable semantic classification
```

Acceptance:

```text
boxed catalog producer = 1
complete source-call inventory authority = 1
complete candidate count authority = 1
inventory incompleteness -> Ordinary = 0
Builder reference = 0
production consumer = 0
```

Use two buildable commits inside this row:

```text
A1 neutral complete inventory                         = closed
A2 Stage-B classification and the 0/1/many matrix    = closed
```

A1 closeout:

```text
complete declaration coverage receipt              = 1
complete MethodCall inventory authority             = 1
same-catalog alias brand check                      = 1
stored lexical observation -> existing V1 facts     = 1
existing static-target catalog projection           = 1
second source-navigation engine                     = 0
Builder / MIR / runtime reference                   = 0
production consumer                                 = 0
```

A2 closeout:

```text
Stage-B candidate inventory producer               = 1
existing callable-result solver reuse              = 1
existing nested / located / outer co-seal reuse    = 1
stable owned candidate identity                    = 1
consuming Zero / One / Many authority              = 1
zero / direct / alias / many / unrelated matrix    = green
existing general-result authority remains unclaimed= green
foreign catalog branding                           = rejected
declaration reorder semantic identity              = stable
second result inference or source traversal        = 0
Builder / MIR / runtime reference                  = 0
production consumer                                = 0
```

Carry into B2:

```text
the lifetime-free candidate inventory is not a durable source owner alone
the exact Box<declaration catalog> must remain owned beside it
One must immediately co-seal its paired row through
  VerifiedPreloopStageBCarrierActivationPlanV1::seal
Many rejection retains the complete ambiguous inventory
numeric catalog identity is never published or persisted
```

## Umbrella B — owned request and disconnected selection

### Commit B1 — `PRELOOP-STAGEB-LEGACY-REQUEST0-S0`

Status: closed.

Add:

```text
LegacyWholeSourceCompileRequestV1
CompilerSuppliedStaticImportSnapshotV1
```

Suggested files:

```text
src/mir/compiler/
  legacy_static_import_snapshot.rs
  legacy_whole_source_request.rs
```

Rules:

```text
constructor = private MirCompiler::compile_legacy_request only
source_file = diagnostic hint only
snapshot = non-Clone None | Explicit(sorted/deduplicated owned aliases)
raw HashMap / mutable / into_hash_map accessor = 0
BareAst = eligible
ProgramV0Compatibility = explicit Ordinary(ProfileExcluded)
ReplCompatibility = explicit Ordinary(ProfileExcluded)
Builder alias mutation before selection = 0
```

`compiler/mod.rs` is already close to 800 lines. It receives declarations and
thin delegation only.

This commit is disconnected. The production Legacy arm still follows its
current path.

B1 closeout:

```text
private non-Clone whole-source request              = 1
private None / Explicit typed alias snapshot        = 1
sorted aliases / duplicate rejection                = green
same-catalog verified alias projection              = 1
diagnostic-only source hint                         = retained
raw map / mutable / into-map accessor               = 0
Builder install / mutation                          = 0
production constructor caller                      = 0
compile_request behavior delta                      = 0
```

### Commit B2 — `PRELOOP-STAGEB-SOURCE-SELECTION0-S0`

Status: closed.

Add:

```text
PreparedOrdinaryLegacyWholeSourceV1
PreparedSelectedPreloopStageBWholeSourceV1
PreloopStageBWholeSourceDispositionV1
RejectedPreloopStageBWholeSourceSelectionV1
```

Exact law:

```text
0 complete rows:
  Ordinary(NoExactCandidate or bounded proof-unavailable reason)

1 complete row:
  Selected

2+ complete rows:
  AmbiguousCandidates rejection

selected identity drift:
  SelectedCandidateDrift rejection
```

The boundary is strict:

```text
complete inventory + bounded Stage-B proof unavailable
  -> Ordinary(BoundedProofUnavailable)

inventory incomplete
invalid alias snapshot
catalog identity/brand invalid
  -> typed rejection
```

A real candidate must never disappear behind `Ordinary`.

Both dispositions retain the same owned alias snapshot exactly once:

```text
PreparedOrdinaryLegacyWholeSourceV1
  owns complete request + snapshot + explicit reason

PreparedSelectedPreloopStageBWholeSourceV1
  owns complete request + snapshot + activation plan
```

The original snapshot may be borrowed to create
`VerifiedStaticImportAliasViewV1` in the proof scope. That borrowed view must
end before the catalog and snapshot move into the Selected product.

Every rejection retains the complete source request and exposes only:

```text
stage()
cause()
bounded_report()
discard(self)
```

Production consumer remains zero through B2.

B2 closeout:

```text
whole-source selection owner                       = 1
Builder-compatible seal_root authority             = existing 1
private cardinality consumer                       = 1
zero -> explicit Ordinary                          = green
one -> immediate exact-catalog activation seal     = green
many -> retained typed ambiguity rejection         = green
ProgramV0 / REPL -> ProfileExcluded Ordinary       = green
direct / supplied-alias selection                  = green
invalid alias / foreign catalog                    = rejected
prepared row / numeric catalog identity exposure   = 0
production caller / Builder mutation               = 0
fallback / retry                                   = 0
```

## Umbrella C — consuming module activation

### Commit C1 — `PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-A`

Status: closed.

Add the sole consuming preparation terminal:

```text
PreparedSelectedPreloopStageBWholeSourceV1
  -> PreparedPreloopStageBModuleActivationV1
```

The terminal owns:

```text
same-allocation boxed catalog
typed aliases
one armed activation row
complete retained selected source owner
```

Preflight before any install:

```text
candidate module and physical main exist
callable catalog lane is vacant
alias lane is vacant or exactly compatible
complete immutable activation plan remains retained
```

Do not expose a general public `into_parts()` tuple. Catalog and row leave the
activation plan only through this consuming preparation terminal.

Any preparation rejection is:

```text
RejectedPreloopStageBModuleActivationV1
  retains complete selected request + catalog + aliases + row
```

Landed refinement:

```text
candidate-shell observer                        = read-only 1
physical main/0 + entry-block readiness         = sealed
catalog lane vacant                             = required
alias lane vacant or exact compatible           = required
caller/target membership re-inference            = 0
ledger construction                              = deferred to C3
install / lower / production caller              = 0
```

Caller/target membership and the armed row already live inside the immutable
activation plan. C1 therefore does not reopen those source truths; it proves
only that the already-open Builder shell is safe to receive the plan later.

### Commit C2 — `LOWER-ROOT-POST-INSTALL-KERNEL0-S0`

Status: closed.

One behavior-neutral private kernel now owns the existing root work after
callable-catalog install. The ordinary facade still owns the sole root clone,
catalog seal, and catalog install, then delegates the owned root plus its
snapshot to:

```text
MirBuilder::lower_root_after_callable_catalog_install_v1
```

The seam remains in `module_lifecycle.rs`: extracting a nested file would
split the existing lifecycle guard authority without reducing policy. The
kernel preserves the existing declaration indexing, static-data preparation,
App/Script decision, and root lowering order. Selected callers remain zero.

Closeout:

```text
root AST clone                                      = existing 1
ordinary catalog seal/install                      = existing 1
post-install root kernel                           = 1
second AST walker / source policy                   = 0
selected catalog/alias install                     = 0
selected caller                                    = 0
MIR / error behavior delta                         = 0
```

### Commit C3 — `PRELOOP-STAGEB-ATOMIC-CONTEXT-INSTALL0-S0`

Status: closed.

One Builder-owned preparation/commit owner now carries the selected callable
catalog plus its typed-alias projection. It rechecks both lanes immediately
before mutation, so a C1 readiness receipt cannot authorize a stale commit.
Every vacancy and exact-alias compatibility check happens before mutation;
the private CompilationContext commit contains only infallible field moves.

This row does not create the function ledger and does not call the
post-install root kernel. Splitting the context transaction from the
function activation ledger keeps partial catalog/alias install impossible
without combining two state machines in one commit.

Acceptance:

```text
catalog + alias preflight owner                    = 1
infallible context commit                         = 1
partial catalog/alias install                     = 0
stale readiness alias conflict                    = typed reject
occupied catalog leaves aliases unchanged         = green
alias conflict leaves catalog vacant              = green
ledger / root lowering / production caller        = 0
```

### Commit C4 — `PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-B`

Status: closed.

Landed shape:

```text
complete selected Legacy owner
  -> named source/install projections
  -> exact AST + catalog + typed aliases + row
  -> one atomic CompilationContext commit
  -> receipt-gated shared post-install root kernel
  -> Armed(row)
  -> retained SelectedCallerNotObserved rejection
```

The root wrapper performs the existing single snapshot clone; the shared
kernel now borrows that snapshot. No second clone, catalog reseal, alternate
root orchestration, or fabricated source owner is present. C4 deliberately
has no successful function-activation product: D1 is the first row that can
add one because it is the first row with a real exact-function producer.

Add:

```text
PreparedPreloopStageBActivationInstallPartsV1
PreparedPreloopStageBSourceInstallPartsV1
PreparedPreloopStageBPreinstalledRootV1
InstalledPreloopStageBModuleActivationV1
PreloopStageBFunctionActivationLedgerV1
lower_root_with_preinstalled_catalog_v1
```

Only narrowly named consuming projections may expose the catalog, aliases,
AST, and row to this transaction. Generic `into_parts()` tuples and raw alias
maps are forbidden.

Owner chain:

```text
PreparedPreloopStageBModuleActivationV1
  -> named source + activation install projections
  -> AST + exact catalog + typed aliases + Armed(row)
  -> existing atomic context commit
  -> InstalledPreloopStageBModuleActivationV1
  -> receipt-gated preinstalled-root wrapper
  -> existing post-install root kernel
```

After all fallible checks, context commit is an infallible move:

```text
catalog -> candidate Builder CompilationContext
aliases -> candidate Builder CompilationContext
row     -> stack-owned single-use ledger
```

The ledger is not a Builder field. Ordinary and selected routes use the same
post-install root kernel. Catalog+alias commit must be one atomic
CompilationContext operation; do not call a fallible catalog install followed
by an alias setter.

C4 has no exact-function ingress yet. Its ledger therefore implements only
the state that has a real producer:

```text
Armed(row)
  -> finish()
  -> Rejected { retained row, SelectedCallerNotObserved }
```

`InFlight`, `Completed`, and selected-function `Rejected` are added only in D1
with their exact production transitions. Creating those variants in C4 would
be dead typestate and a false completion claim. There is no payloadless
`Consumed`/`Poisoned`, `take`, `reset`, `rearm`, or row escape accessor.

Acceptance:

```text
selected catalog seal = 1
selected catalog install = 1
selected catalog reseal = 0
partial catalog/alias install = 0
preinstalled-root wrapper clone = exact 1
post-install root kernel consumer = exact 1 disconnected
ledger state = Armed only
dead InFlight / Completed producer = 0
Builder source-site registry = 0
second root-lowering orchestration = 0
production consumer = 0
```

## Umbrella D — exact production ingress

### Commit D1 — `PRELOOP-STAGEB-FUNCTION-INGRESS0-I0`

Execution status: superseded before production activation by
`PRELOOP-STAGEB-OWNED-LOCATED-AUTHORITY0-prime-r1`.

The C4 audit proved that the owned activation row no longer contains the
borrowed nested-result contract required by the existing located Port, while
the installed catalog is held inside the Builder that the Port must mutate.
Do not execute the direct D1 sequence below. Follow:

```text
docs/development/current/main/investigations/
  preloop-stageb-owned-located-authority0-prime-r1-task-map-2026-07-27.md
```

This is the first production behavior change.

Connect exactly:

```text
MirCompiler::compile_request
  / MirLoweringRequestV1::Legacy arm
```

Flow:

```text
LegacyWholeSourceCompileRequestV1
  -> select
  -> Ordinary:
       install typed aliases at the existing candidate boundary
       use unchanged compile_with_source_internal/build_module behavior
  -> Selected:
       consume prepared module activation
       use preinstalled-catalog root lowering
       capture exact caller once
```

`compile_with_source_and_imports()` must move a typed import snapshot into the
request instead of mutating Builder before selection. `compile_legacy()` emits
the `None` snapshot instead of clearing ambient Builder aliases.

Selected function law:

```text
exact canonical key observed once -> consume row
selected key never observed       -> typed rejection
selected key observed twice       -> typed rejection
selected function failure         -> no ordinary retry
```

D1 extends the C4 ledger only when these transitions gain real producers:

```text
Armed(row)
  -> InFlight(selected ingress)
  -> Completed(receipt)
  |  Rejected { retained row/evidence, cause }
```

Direct `MirBuilder::build_module`, AST JSON, Program(JSON v0), REPL, and Raw
routes have zero caller delta.

### Commit D2 — `PRELOOP-STAGEB-FUNCTION-INGRESS0-P0/G0`

Focused matrix:

```text
ordinary + no imports
ordinary + explicit imports
selected direct-owner target
selected alias target
zero candidate -> ordinary
many candidates -> pre-Builder reject
selected identity drift
catalog or alias duplicate install
selected caller missing
selected caller double consumption
selected function failure -> no retry
ProgramV0 / REPL -> Ordinary(ProfileExcluded)
failure -> fresh compiler success
```

Fold structural assertions into the existing source-entry/Stage-B guard
family. Do not create one shell guard per commit.

Milestone verification:

```bash
cargo check --lib
cargo test -q --lib preloop_stageb
cargo test -q --lib source_call_target
cargo test -q --lib callable_result_representation
cargo test -q --lib recursive_child_lowering_rawport
cargo test -q --lib module_lowering_invocation_reentrant
bash tools/checks/current_state_pointer_guard.sh
```

Use the exact focused test filters present at implementation time. Run
`tools/checks/dev_gate.sh quick` at I0/G0 milestones, not after every
mechanical commit.

## Umbrella E — outer carrier completion

These rows reuse already accepted authorities. Do not reopen their design
unless implementation evidence contradicts this map.

```text
UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
  source-neutral receipt at actual generic Call success only

PRELOOP-OUTER-CARRIER-RECEIPT0-I0
  exact outer receipt
  inner destination is never used as outer authority

PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0
  CompletedVariableAssignmentV1
  CompletedPreloopCarrierAssignmentV1
  outer final destination == assignment RHS == assigned carrier
  do not infer the carrier from variable_map after the fact

PRELOOP-OUTER-CARRIER-RECEIPT0-P0/G0
  outer/inner/assignment correspondence matrix

PRELOOP-OUTER-CARRIER-TYPE-I0-S0
  prepare with existing TypeFactDecisionV1

PRELOOP-OUTER-CARRIER-TYPE-I0-I0/P0/G0
  existing TypeFactDecisionV1
  existing TypeContext::set_type
  success-only outer Integer publication
  GenericLoop remains consumer-only

CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
  rerun the real Stage-B progression guard
```

The final progression proof runs:

```bash
bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

Do not change its expected frontier before the implementation is green.

After the outer path is green:

```text
PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
```

retires or explicitly parks the proof-only inner TYPE-I0 consumer. It must not
remain a production outer-carrier authority.

## Task order

```text
PRELOOP-STAGEB-SOURCE-INVENTORY0-P0

-> PRELOOP-STAGEB-LEGACY-REQUEST0-S0
-> PRELOOP-STAGEB-SOURCE-SELECTION0-S0

-> PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-A
-> LOWER-ROOT-POST-INSTALL-KERNEL0-S0
-> PRELOOP-STAGEB-ATOMIC-CONTEXT-INSTALL0-S0
-> PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-B

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-I0
-> PRELOOP-STAGEB-FUNCTION-INGRESS0-P0/G0

-> UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-I0
-> PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-P0/G0

-> PRELOOP-OUTER-CARRIER-TYPE-I0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-I0/P0/G0

-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
```

This is approximately 10–13 buildable commits. Use Refactor Series Mode
inside each umbrella, with one purpose per series and all behavior changes at
the end of their umbrella.

## Minimal test and guard inventory

Create about four focused test families across the whole series:

```text
src/mir/source_call_target/whole_source_inventory_tests.rs
src/mir/preloop_stageb_carrier/source_selection_tests.rs
src/mir/compiler/preloop_stageb_production_tests.rs
src/mir/builder/calls/preloop_outer_carrier_tests.rs
```

Module-activation tests may remain private beside their owner. Reuse:

```text
actual Parser same-allocation Stage-B fixture
callable catalog/install transaction tests
instance-method draft parity tests
unified physical receipt tests
TypeFactDecisionV1 tests
```

Do not add a shell/Python guard per row. At final G0, extend the existing
`tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py` structural guard
once. Do not grow the nearly-full `mirbuilder_type_fact_partition_guard.py`.

File pressure is already high:

```text
src/mir/compiler/mod.rs                  ~759 lines
src/mir/builder/module_lifecycle.rs      ~601 lines
src/mir/builder/calls/unified_emitter.rs ~789 lines
```

Keep these as thin facades. New request, activation, receipt, and test owners
belong in small sibling modules; do not push an existing source/check file over
800 lines.

## Structural gate

```text
whole-source producer                              = 1
whole-source production consumer                   = 1
consumer location                                  = compile_request Legacy arm

candidate zero product                             = explicit Ordinary
Option::None candidate authority                   = 0
0 -> Ordinary / 1 -> Selected / many -> reject     = exact
incomplete inventory -> Ordinary                   = 0
invalid alias snapshot -> Ordinary                 = 0

compiler-supplied alias snapshot                   = 1
ambient Builder alias read during selection        = 0
Builder alias mutation before selection            = 0
Ordinary snapshot consume                          = exactly 1
Selected snapshot consume                          = exactly 1
borrowed alias view after selection                 = 0

selected exact catalog seal                        = 1
selected catalog install                           = 1
selected catalog reseal                            = 0

ordinary build_module route                        = unchanged
new selected direct-build_module activation        = 0
AST JSON / Program(JSON v0) / REPL behavior delta  = 0

Builder source-policy owner                        = 0
Builder source-site registry                       = 0
persistent SourceExprSite -> ValueId map           = 0

selected caller consume                            = exactly 1
payloadless selected ledger terminal               = 0
selected retry through ordinary lowering           = 0
fallback / route reselection                       = 0

Raw source package / brand / ledger consumer       = 0
all modified/new source/check files                < 800 lines
```

## Stop-the-line conditions

Open a new D0 only if evidence proves one of these:

```text
1. Complete candidate inventory requires a second AST walker or name policy.
2. Typed alias snapshot cannot reproduce the exact existing target authority.
3. Same-allocation catalog cannot be installed without resealing or a partial
   catalog/alias mutation.
4. Selected lower_root cannot share the existing post-install kernel without
   creating a second root-lowering orchestration.
5. Exact selected function row cannot be consumed without a Builder field or
   persistent source-site map.
6. Existing alias behavior depends on mutation timing that cannot be
   represented by the owned typed snapshot.
```

All other unsupported cases are typed Ordinary or typed rejection according
to the fixed 0/1/many law.

## Sunset accounting

Immediate repayment:

```text
PRELOOP-STAGEB-LEGACY-ALIAS-MUTATION-SUNSET-001
  retires:
    compile_with_source_and_imports pre-selection Builder alias mutation
    compile_legacy pre-selection alias clear
    selected lower_root catalog reseal
  repay at:
    PRELOOP-STAGEB-FUNCTION-INGRESS0-I0
```

After the outer production path is green:

```text
PRELOOP-INNER-TYPE-PROOF-SUNSET-001
  row:
    PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
  retires or merges:
    proof-only inner TYPE publisher
    temporary Stage-B correspondence fixture
    redundant Emitted state when no consumer remains
```

Long-term parked:

```text
PRELOOP-STAGEB-LEGACY-SOURCE-PRODUCER-SUNSET-001
  owner:
    PRELOOP-STAGEB-SOURCE-PRODUCER-RETIRE0
  retire only after:
    canonical/normal source-plan migration
    compile_request Legacy selected consumer = 0
    proof-only producer consumers = 0
```

## Post-Stage-B frontier

The real progression guard selects the next frontier:

```text
ownership grammar
loop-refresh
another missing representation
```

Alias/View language semantics remain parked. Do not reserve a mandatory
loop-refresh series before the real Stage-B guard names it.

## Non-claims

```text
direct MirBuilder::build_module activation
Raw source/package activation
default compiler route cutover
whole port-aware Raw cutover
general instance-method selection or result inference

loop-refresh activation
GenericLoop type publisher migration
ownership grammar
Alias / View language semantics

parser / Hako / VM / LLVM / backend changes
fallback / retry
```
