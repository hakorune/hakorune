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
  - src/mir/compiler.rs
Sunset:
  id: PRELOOP-STAGEB-LEGACY-SOURCE-PRODUCER-SUNSET-001
  owner: PRELOOP-STAGEB-SOURCE-PRODUCER-RETIRE0
  row: PRELOOP-STAGEB-SOURCE-PRODUCER-RETIRE0-S0
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

alias authority:
  CompilerSuppliedStaticImportSnapshotV1

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

Suggested layout:

```text
src/mir/source_call_target/
  whole_source_inventory.rs
  whole_source_inventory_error.rs
  whole_source_inventory_tests.rs
```

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
missing alias target                 -> incomplete proof disposition
foreign equal-looking catalog        -> typed identity rejection
source declaration reorder           -> stable semantic classification
```

Acceptance:

```text
boxed catalog producer = 1
complete candidate count authority = 1
Builder reference = 0
production consumer = 0
```

## Umbrella B — owned request and disconnected selection

### Commit B1 — `PRELOOP-STAGEB-LEGACY-REQUEST0-S0`

Add:

```text
LegacyWholeSourceCompileRequestV1
CompilerSuppliedStaticImportSnapshotV1
```

Rules:

```text
constructor = private MirCompiler::compile_legacy_request only
source_file = diagnostic hint only
BareAst = eligible
ProgramV0Compatibility = explicit Ordinary(ProfileExcluded)
ReplCompatibility = explicit Ordinary(ProfileExcluded)
Builder alias mutation before selection = 0
```

This commit is disconnected. The production Legacy arm still follows its
current path.

### Commit B2 — `PRELOOP-STAGEB-SOURCE-SELECTION0-S0`

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

An incomplete exact proof is not a new Legacy compile error. It produces an
explicit Ordinary disposition. Only ambiguity, contradictory authority, or
post-selection identity drift rejects the whole request.

Every rejection retains the complete source request and exposes only:

```text
stage()
cause()
bounded_report()
discard(self)
```

Production consumer remains zero through B2.

## Umbrella C — consuming module activation

### Commit C1 — `PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-A`

Add the sole consuming preparation terminal:

```text
VerifiedPreloopStageBCarrierActivationPlanV1
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
selected caller and target remain members of the catalog
activation ledger is Armed
```

Do not expose a general public `into_parts()` tuple. Catalog and row leave the
activation plan only through this consuming preparation terminal.

### Commit C2 — `PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-B`

Add:

```text
InstalledPreloopStageBModuleActivationV1
PreloopStageBFunctionActivationLedgerV1
lower_root_with_preinstalled_catalog_v1
```

After all fallible checks, commit is an infallible move:

```text
catalog -> candidate Builder CompilationContext
aliases -> candidate Builder CompilationContext
row     -> stack-owned single-use ledger
```

The ledger is not a Builder field. Extract one common post-install root
lowering kernel so ordinary and selected routes share orchestration after
catalog installation.

Acceptance:

```text
selected catalog seal = 1
selected catalog install = 1
selected catalog reseal = 0
partial catalog/alias install = 0
Builder source-site registry = 0
second root-lowering orchestration = 0
production consumer = 0
```

## Umbrella D — exact production ingress

### Commit D1 — `PRELOOP-STAGEB-FUNCTION-INGRESS0-I0`

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

Direct `MirBuilder::build_module`, AST JSON, Program(JSON v0), REPL, and Raw
routes have zero caller delta.

### Commit D2 — `PRELOOP-STAGEB-FUNCTION-INGRESS0-P0`

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

### Commit D3 — `PRELOOP-STAGEB-FUNCTION-INGRESS0-G0`

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
PRELOOP-OUTER-CARRIER-RECEIPT0-P0
PRELOOP-OUTER-CARRIER-RECEIPT0-G0
  exact outer receipt
  outer destination == assignment carrier
  inner destination is never used as outer authority

PRELOOP-OUTER-CARRIER-TYPE-I0-S0
PRELOOP-OUTER-CARRIER-TYPE-I0-I0
PRELOOP-OUTER-CARRIER-TYPE-I0-P0
PRELOOP-OUTER-CARRIER-TYPE-I0-G0
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
-> PRELOOP-STAGEB-MODULE-ACTIVATION0-S0-B

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-I0
-> PRELOOP-STAGEB-FUNCTION-INGRESS0-P0
-> PRELOOP-STAGEB-FUNCTION-INGRESS0-G0

-> UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-I0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-P0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-G0

-> PRELOOP-OUTER-CARRIER-TYPE-I0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-I0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-P0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-G0

-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
```

This is approximately 10–16 buildable commits. Use Refactor Series Mode
inside each umbrella, with one purpose per series and all behavior changes at
the end of their umbrella.

## Structural gate

```text
whole-source producer                              = 1
whole-source production consumer                   = 1
consumer location                                  = compile_request Legacy arm

candidate zero product                             = explicit Ordinary
Option::None candidate authority                   = 0
0 -> Ordinary / 1 -> Selected / many -> reject     = exact

compiler-supplied alias snapshot                   = 1
ambient Builder alias read during selection        = 0
Builder alias mutation before selection            = 0

selected exact catalog seal                        = 1
selected catalog install                           = 1
selected catalog reseal                            = 0

ordinary build_module route                        = unchanged
direct build_module producer                       = 0
AST JSON / Program(JSON v0) / REPL behavior delta  = 0

Builder source-policy owner                        = 0
Builder source-site registry                       = 0
persistent SourceExprSite -> ValueId map           = 0

selected caller consume                            = exactly 1
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
```

All other unsupported cases are typed Ordinary or typed rejection according
to the fixed 0/1/many law.

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
