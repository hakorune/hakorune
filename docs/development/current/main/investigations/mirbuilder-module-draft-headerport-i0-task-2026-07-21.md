---
Status: `HEADERPORT0-RAWPORT0-SELECT/S0/M0-T0/R0` and
  `HEADERPORT0-RAWPORT0-LEGACYTERM0-S0/P0/I0/G0` and
  `HEADERPORT0-RAWPORT0-LOOPBRIDGE0-SELECT/S0/P0/I0/G0` are closed;
  `FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0` is paused at the re-entrant raw
  child-terminal consultation
Date: 2026-07-21
Scope: one atomic `FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0` cutover
Parent: docs/development/current/main/investigations/mirbuilder-finalize0-p0-production-census-task-2026-07-20.md
Decision: Candidate A-prime plus HeaderPort Candidate H-prime
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
---

# MODULEDRAFT0-HEADERPORT0-I0: invocation-owned draft collection cutover

## Outcome

Candidate A-prime remains selected.  `MODULEDRAFT0` is the prerequisite for
`FACTSESSION0`: every physical function must first complete into one
unpublished draft collected by one module-invocation-owned collector.  The
existing fact lanes stay in `FunctionLoweringStateV1`; later
`FACTSESSION0-ACTIVEBIND0` adds only a generation and receipt ledger, not a
second `TypeContext`.

Candidate H-prime fixes the required lowering-time header access: one
`ModuleLoweringInvocationV1` owns the same collector and loans an explicit,
short-lived, read-only `LoweringHeaderPortV1`.  It is not a Builder or
CompilationContext field, TLS value, cache, second draft store, or fallback.

The one semantic activation is the final atomic cutover.  Earlier I0 steps
are buildable BoxShape preparation only and keep production consumers at zero.
The raw recursive re-entrancy audit, passive S0 vocabulary, and port-owned
  resolved terminal, raw recursive threading, and the disconnected legacy
 terminal proof are closed. `LOOPBRIDGE0-S0/P0/I0/G0` is also closed. The next
 code-facing row is `FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0`.

## I0 source revalidation pause

Before the atomic cutover, the production source audit found that the current
`complete_legacy_child` API holds the invocation port mutably while its child
closure runs. A child body can itself open another raw Box child, so that shape
cannot reborrow the same port. The raw `BoxDeclaration` constructor loop also
still calls the restore-then-publish legacy facade directly. The detailed
source evidence and Candidate R-prime are in:

```text
docs/development/current/main/investigations/
  mirbuilder-headerport-reentrant-terminal-consultation-2026-07-21.md
```

Do not activate I0 until that terminal shape is selected. The existing
collector, header-port, raw-loop quarantine, and canonical callable catalog
laws remain unchanged.

## Fixed terminal law

For a successful child function:

```text
header-port-borrowed lowering and completion
-> validate_before_restore(success = true)
-> end header read borrow
-> prepare admission
-> seal unpublished draft
-> infallible collector admission
-> restore parent
```

All admission failure points close before collection.  A primary, cleanup,
admission, or unwind failure leaves the collector unchanged and restores the
parent exactly once.  `collect -> restore` is never reversed.

The final main path has the analogous form:

```text
complete main unpublished under the port
-> end header borrow
-> collect Main
-> collect synthetic condition_fn when absent
-> consume the one collector into the module shell
-> retain the existing downstream metadata/derived/PHI schedule
```

This row does not redesign TypePipeline, Call/Await facts, PHI repair,
finalization repair, JoinIR, MODULETX, or CUT0.

## Source facts that constrain the implementation

1. `CanonicalFunctionLoweringSessionV1` currently performs
   `close_unpublished -> restore -> publish`; the existing test-only
   pre-restore observer proves the needed ordering can be extracted, but it is
   not a production terminal owner.
2. `finalize_function_draft` itself reads completed headers for the Call/Await
   compatibility hint.  The port must therefore live through child completion,
   then end before mutable admission.
3. The raw reader closure is recursive, not an eight-call patch.  The audited
   reader families are Call annotation/finalizer hint, method signature,
   known rewrite, method index, tail resolver, materializer, and constructor
   birth checks.  Existing raw child ports erase their identity before
   `build_expression_impl`; direct expression callers require a port-preserving
   closure.
4. Main currently adds `main` directly after completion.  Its exact admission
   is symbol `main`, arity `0`.  The synthetic function is symbol
   `condition_fn`, arity `1`; fixture spellings such as `main/0` and
   `condition_fn/1` are diagnostics only and must not become collector keys.
5. A-plus functions own `FunctionOwnerIdV1`, not `CanonicalCallableKeyV1`.
   The collector must add an exact `CanonicalResolvedOwner(FunctionOwnerIdV1)`
   key (or an equivalently precise named variant); using `LegacySymbol` under
   canonical-reject policy is forbidden.  Binding-SSA module children retain
   their existing canonical callable keys.
6. Binding-SSA acyclic/recursive lowering currently builds siblings against
   its sealed callable catalog while the module is empty.  It must retain that
   source-header authority; it must not start observing previously collected
   sibling headers merely because the collector now has a prefix.
7. The collector's final `collect_sealed` integrity checks must occur before
   mutation when the atomic cutover claims collector-unchanged failure.
8. Raw AST lowering can open a child while the parent expression is still
   lowering: `ASTNode::BoxDeclaration` reaches static/instance child lowering
   under generic expression descent.  The current raw child port recreates a
   builder-only port before `build_expression_impl`, so the outer invocation's
   collector is not available to that nested child.  A short
   `with_header_port` closure is therefore insufficient for raw production.

## Raw recursive module port — Candidate A-prime selected

The selected invocation collector remains outside `MirBuilder`,
`CompilationContext`, and TLS.  Yet a nested raw child must collect before its
parent is restored.  Candidate A-prime is selected:

```text
ModuleLoweringPortV1
```

It is one explicit stack-owned raw-recursion capability that carries the
invocation-owned collector through every descent which may open a child:

```text
ModuleLoweringPortV1<'collector>
  borrows the one ModuleDraftCollectorV1
  is threaded explicitly from module root through raw expression/body descent
  loans a short read-only LoweringHeaderPortV1 for header queries
  owns the child completion transition after that read loan ends
```

The concrete name is `ModuleLoweringPortV1`; it has the same shape shown above
and is not limited to raw callers.  It does not own or store a Builder.  Its
only permitted services are a short `with_headers` loan and explicit legacy or
resolved child completion using the same collector.  Header observations return
owned copies or end inside the closure before Builder mutation.  Read and
mutable phases alternate.  The port is stack-owned by the invocation and
passed explicitly through re-entrant lowering; it is never stored in Builder,
CompilationContext, TLS, an AST node, or a closure cache.

Rejected candidates:

```text
top-level HeaderPort only, while nested raw children keep old publication
  // violates one-collector and collect-before-restore laws

collector lookup through Builder / CompilationContext / TLS
  // ambient authority and re-entrancy hazard

preload or clone completed headers into current_module
  // second draft/header truth

long-lived read port combined with mutable child collector access
  // aliases the collector across read and collect phases
```

`RAWPORT0-S0` owns the private trait/API vocabulary and exact recursive-closure
test seam, including `ASTNode::BoxDeclaration`.  If that closure cannot be
made explicit without a second authority or an unbounded duplicate lowering
tree, the series stops rather than partially activating I0.

## RAWPORT0-M0-R0 checkpoint — legacy child terminal and Loop boundary

Status: **the raw expression descendants are being ported, but M0-R0 may not
claim closure yet.**  The direct raw control audit established two independent
frontiers:

```text
BoxDeclaration child function
  needs a legacy child completion terminal before it can collect before restore

Loop body
  reaches JoinIR route -> composer -> PlanLowerer and is not a thin raw
  expression descent adapter
```

### Selected legacy-terminal shape

`RAWPORT0-LEGACYTERM0` is selected as the next code-facing owner.  It extends
the existing pending-session pattern without rebranding a raw child as a
canonical resolved child:

```text
Legacy child lowering success
-> existing cleanup / terminal validation
-> capture LegacyFunctionPendingSessionV1
-> prepare exact LegacySymbol admission with LegacyReplaceWholePair
-> seal unpublished draft
-> ModuleLoweringPortV1::complete_legacy_child
-> infallible whole-pair collection
-> restore parent
```

The prepared request owns only the existing legacy function identity, expected
symbol/arity, and `LegacyReplaceWholePair` policy.  It must not fabricate a
`CanonicalResolvedOwner`, a callable key, a second collector, a temporary
module/header cache, or a Builder/CompilationContext/TLS port field.  All
fallible cleanup and admission checks close before collection; primary,
cleanup, admission, and unwind failures leave the collector unchanged and
restore the parent exactly once.  This is a disconnected M0 terminal: raw V1
facades remain the sole production route.

The first code slice is:

```text
HEADERPORT0-RAWPORT0-LEGACYTERM0-S0
  passive LegacyFunctionPendingSessionV1 / prepared admission vocabulary
  and port completion API; production consumers = 0

-> HEADERPORT0-RAWPORT0-LEGACYTERM0-P0
   success, cleanup, admission, unwind, replace-whole-pair, and raw body-port
   proofs; no resolved identity fabrication

-> HEADERPORT0-RAWPORT0-LEGACYTERM0-I0
   disconnected raw static/instance Box child body terminal wiring only

-> HEADERPORT0-RAWPORT0-LEGACYTERM0-G0
   one legacy pending-terminal owner; post-restore raw child publication = 0
   for the disconnected path
```

### Loop bridge selection — Candidate C-prime accepted

`cf_loop` cannot be parameterized as another M0-R0 thin sibling.  Its active
route reaches JoinIR routing, recipe composition, and `PlanLowerer`, where
plan construction already mutates the Builder and normalized shadow may clone
or reconstruct syntax.  Passing `ModuleLoweringPortV1` into that stack now
would mix raw child transport with Loop semantic/transaction authority.

The selection is now closed. Candidate C-prime is a scoped **raw Loop
child-open quarantine**, not a JoinIR port bridge:

```text
Raw invocation Loop
-> pure NoChildFunctionEntry preflight over the exact raw syntax
-> NoChildFunctionEntry
     -> unchanged cf_loop
   | ReachableBoxDeclaration
     -> typed freeze before cf_loop
```

The preflight reads the original raw Loop syntax only. It does not clone,
canonicalize, reconstruct, or assign source identity to AST nodes. It treats a
Lambda body as deferred ownership, recursively scans executable nested Loop /
If / statement and expression bodies, and rejects any reachable
`BoxDeclaration` before a Builder effect, header loan, collector admission, or
child session. This preserves the current accepted JoinIR loop behavior: the
audited generic Loop normalizer and recipe/Parts vocabulary do not admit a
`BoxDeclaration` or open a child function session. It does not claim that all
future Loop profiles have this property.

Candidate A, passing `ModuleLoweringPortV1` through JoinIR, is rejected:
JoinIR route selection, recipe composers, normalized shadow, and `PlanLowerer`
already own Builder, CFG, variable-map, PHI, and plan transactions. Candidate
B, reusing `VerifiedLocatedCoreLoopPlanV1`, is also rejected: it is a
callable-result claim product, its composer still mutates Builder, and it is
not a module-draft transport boundary.

The following remains fixed:

```text
ModuleLoweringPortV1 parameter in JoinIR router / RecipeComposer / PlanLowerer = 0
raw Loop clone/reconstruction for this port = 0
ModuleLoweringPortV1 parameter in normalized shadow / StepTree = 0
collector/header capability stored in Builder / CompilationContext / TLS = 0
```

`cf_loop` remains the only Loop semantic/transaction owner. The quarantine is
not a raw Loop rewrite, a second `cf_loop_with_port`, a general located Loop
bridge, or a fallback to a different route.

#### Exact continuation order

```text
HEADERPORT0-RAWPORT0-LOOPBRIDGE0-S0
  one private pure RawLoopChildEntryDispositionV1 and exact-syntax scanner;
  production consumers = 0; no Builder, port, collector, or JoinIR parameter

-> HEADERPORT0-RAWPORT0-LOOPBRIDGE0-P0
   source/route matrix: accepted generic/recipe shapes are NoChild;
   direct/nested executable BoxDeclaration rejects before effects; Lambda is
   deferred; normalized shadow does not receive the port; existing no-child
   cf_loop parity is fixed

-> HEADERPORT0-RAWPORT0-LOOPBRIDGE0-I0
   wire only the raw invocation Loop boundary:
   NoChild -> existing cf_loop, child-opening -> typed pre-effect freeze.
   Legacy raw facade and JoinIR internals remain unchanged.

-> HEADERPORT0-RAWPORT0-LOOPBRIDGE0-G0
   one scanner/boundary owner; JoinIR/plan/normalization port consumers = 0;
   existing successful Loop child-session openers = 0
```

Stop and reopen a dedicated pure-plan/function-session bridge design if a
currently successful Loop profile opens a child function, if a route requires
the module port in JoinIR, if the scanner needs a cloned/reconstructed AST as
authority, or if a collector/header capability must enter Builder/TLS. Do not
weaken this to a broad raw-Loop rejection unless a production census proves the
rejected profile is absent and that behavior change is explicitly selected.

#### LOOPBRIDGE0-S0 closeout — disconnected syntax disposition

`LOOPBRIDGE0-S0` closes one private pure
`RawLoopChildEntryDispositionV1` with no production consumer. It classifies
the exact raw Loop condition/body using `ASTNode::for_each_child`, the generic
AST child-topology SSOT. A direct, expression-nested, or executable nested-Loop
`BoxDeclaration` yields `ReachableBoxDeclaration`; an ordinary Loop yields
`NoChildFunctionEntry`. Lambda and function-declaration bodies are deferred
ownership surfaces and are not scanned as surrounding Loop execution. The
product has no Builder, invocation port, collector, header, JoinIR, source-site
identity, AST clone, or mutation authority. Four focused fixtures are green.

`HEADERPORT0-RAWPORT0-LOOPBRIDGE0-P0` is next. It must prove the source and
route matrix plus pre-effect rejection/parity before I0 wires the one raw
invocation boundary. No production dispatcher, `cf_loop`, JoinIR, or collector
consumer changed in S0.

#### LOOPBRIDGE0-P0 closeout — route isolation proof

`LOOPBRIDGE0-P0` extends the existing reusable HeaderPort guard rather than
adding a row-only shell check. It proves that the sole raw Loop dispatcher still
has exactly one unchanged `cf_loop` delegate and zero classifier consumers;
the disconnected classifier retains the direct Box, deferred Lambda/function,
and generic-child-topology laws. The guard also proves that no
`ModuleLoweringPortV1` or `RawInvocationChildPortV1` has entered any control
flow source, and that the plan subtree contains no `BoxDeclaration` or legacy
method-function opener. Thus the current accepted plan/recipe routes have zero
known child-session opener. This is a static route proof, not a claim that the
production dispatcher has begun rejecting anything.

`HEADERPORT0-RAWPORT0-LOOPBRIDGE0-I0` is next. It alone may make the pure
disposition observable at the raw invocation Loop boundary, before `cf_loop`.
It must preserve the legacy raw facade, keep the one dispatch match tree, and
prove zero Builder/collector/header delta on `ReachableBoxDeclaration`.

#### LOOPBRIDGE0-I0/G0 closeout — one pre-JoinIR boundary

I0 adds one `RawLoopChildEntryPortV1` to the existing raw dispatcher
capability set. The single Loop match now calls one `port.lower_loop`; the
legacy port delegates directly to the unchanged `cf_loop`, while the invocation
port classifies the original syntax and delegates only `NoChildFunctionEntry`.
`ReachableBoxDeclaration` returns one typed contract freeze before JoinIR,
Builder emission, collector admission, or header observation. Focused fixtures
prove both the zero-delta rejection and legacy/invocation no-child result plus
instruction-set parity.

G0 extends the reusable HeaderPort guard: it fixes one raw Loop dispatch, one
raw Loop boundary trait, exactly two port implementations, no direct dispatcher
`cf_loop` bypass, and zero control-flow/plan module-port or child-opener
consumers. The next row is the already-selected atomic
`FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0` cutover; LOOPBRIDGE0 does not add a
second production collector or authorize FACTSESSION0.

## Exact task order

```text
HEADERPORT0-RAWPORT0-S0
  Passive `ModuleLoweringPortV1`, pending child-terminal, and truthful
  collector-key vocabulary.  Fix the disconnected main/condition_fn fixture
  spellings.  Production consumers = 0.

-> HEADERPORT0-RAWPORT0-M0
   M0 is one BoxShape series, not an eight-call patch:

   ```text
   M0-T0  port-owned resolved-child terminal.  One request carries only
          resolved identity/symbol/arity; the port itself performs
          capture-pending -> prepare -> seal -> collect -> restore.
          Production consumers = 0.

   M0-R0  introduce the one raw invocation child port and move every existing
          raw child request through the existing recursive-descent traits or
          thin port-aware siblings.  The existing expression dispatcher is
          parameterized once; a parallel AST match tree is forbidden.

   M0-G0  source-derived direct-child census: every production raw child edge
          is port-preserving, or the row fails before I0.  Legacy V1 facades
          remain the production route through this whole M0 series.
   ```

   The audit finds 21 direct production `build_expression*` families.  The
   existing Binary, short-circuit, CallArgument, MethodCall, Local,
   Assignment, Return, and statement-If descent traits are reusable, but each
   raw facade currently recreates `RawLegacyChildLoweringPortV1`; Program,
   ScopeBox, unary, field/index, collection, indirect-call, match/check,
   record, async, print, exception, and control-flow helpers still bypass the
   trait boundary.  Therefore partial `BoxDeclaration` wiring is forbidden.
   Existing V1 routes remain production.

-> HEADERPORT0-RAWPORT0-P0
   Add disconnected port-aware main completion, synthetic condition_fn
   collection, and sole collector-to-module-shell aggregation.  Close nested
   child success/error/cleanup/panic/header-lifetime proofs for all five roots,
   exact canonical sibling visibility, and collector-unchanged failure before
   activation.

-> HEADERPORT0-I0
   One atomic production cutover: compiler legacy, host AST JSON, runtime
   emit, canonical A-plus, Binding-SSA trivial/acyclic/recursive, child
   terminals, main, condition_fn, and batch aggregation all use the one
   invocation-owned collector and explicit port.

-> HEADERPORT0-I0-G0
   Guard the retirement boundary, then resume FACTSESSION0-ACTIVEBIND0.
```

## Structural placement

Files already near the source-size limit must not receive duplicate V2 bodies:

```text
calls/lowering.rs        647 lines
calls/unified_emitter.rs 640 lines
module_lifecycle.rs      594 lines
```

Put the extracted pending terminal and invocation orchestration in small
sibling modules.  Keep every modified source/check file below 800 lines.

## CUT0 acceptance

```text
one collector per module invocation
production HeaderPort reader fallback = 0
post-restore bare child publication = 0
direct production MirModule insertion for function completion = 0
collector aggregation consumers = 1
lowering-time current_module.functions header readers = 0
Builder/CompilationContext HeaderPort fields = 0
TLS/header cache/second draft store = 0
legacy replacement still replaces the whole collected pair
canonical duplicate still rejects before collection
Binding-SSA sibling-header authority remains the sealed callable catalog
```

## Stop conditions

Stop the series and return to design selection if any requirement needs:

```text
a current_module-to-port adapter or fallback
a raw nested child which cannot receive the same explicit invocation port
a cloned/header-only MirModule cache
a Builder/CompilationContext/TLS port field
a second collector or a draft clone
body or metadata access through the header port
collector mutation before every admission failure point is closed
CanonicalCallableKey fabricated from an A-plus FunctionOwnerId
collector headers substituted for Binding-SSA callable-catalog authority
PHI/TypePipeline/Call-Await/JoinIR semantic repair in this row
```

## Downstream task boundary

Only after the remaining `HEADERPORT0-I0-G0` cutover is green:

```text
FACTSESSION0-ACTIVEBIND0-S0/P0
-> FACTSESSION0-I0/G0
-> REMATFACT0-P0
```

### FACTSESSION0 continuation contract — Candidate A-prime

This is a downstream contract, not an authorization to begin FACTSESSION0
while the remaining HeaderPort cutover is active. Candidate A-prime is fixed
as follows:

```text
MODULEDRAFT0
  one invocation-owned unpublished-draft collector
-> FACTSESSION0-ACTIVEBIND0
  bind generation + receipt ledger to the existing live fact lanes
-> FACTSESSION0-I0
  seal the collected draft and all eight lanes as one product
```

The production active fact owner remains the already-live
`FunctionLoweringStateV1` storage:

```text
FunctionLoweringStateV1
  type_ctx                 // six existing TypeContext lanes
  value_origins            // two diagnostic-origin lanes
  active_fact_binding      // generation + open receipt ledger only
```

`ActiveFunctionFactBindingV1` must not own, mirror, or synthesize a second
`TypeContext`.  On successful completion the same existing lanes are taken,
sealed, and paired with the one unpublished draft.  On abort they are consumed
or discarded under the existing session law.  The legacy BoxCompilation
three-clear/three-retain behavior is retired only at this later cutover.

#### One collector, prepared admission, and terminal ordering

`MODULEDRAFT0`'s collector is the only physical collector.  FACTSESSION0
upgrades that same ownership surface from:

```text
UnpublishedFunctionDraftV1
```

to:

```text
CompletedFunctionDraftWithFactsV1
```

It must not add a parallel facts map or a second bare-draft map.  Function
identity and fact generation remain distinct:

```text
FunctionDraftKeyV1
  Main | LegacySymbol | CanonicalResolvedOwner | CanonicalCallableKey
  | SyntheticConditionFn

FunctionFactGenerationV1
  fact-session lifetime brand only
```

Every child completion must close all fallible work before mutating the
collector:

```text
physical lowering success
-> validate_before_restore(success = true)
-> prepare admission
     expected symbol / arity
     module brand
     duplicate policy and key collision
     collector capacity
-> seal draft (+ later facts)
-> infallible collect
-> restore parent
```

The required failure behavior is:

```text
operation success + cleanup failure
  -> collector unchanged -> restore -> Cleanup error

operation failure + cleanup failure
  -> collector unchanged -> restore -> DuringCleanup

admission failure
  -> collector unchanged -> restore -> Publication error

unwind
  -> abort -> restore -> resume_unwind
```

Neither `collect -> validate` nor `restore -> bare publish` is allowed.  A
legacy duplicate replaces the complete paired product; a canonical duplicate
rejects before collection.  Duplicate policy is not normalized in this row.

#### Lowering-time header read view

Moving an unfinished child out of `current_module.functions` must not silently
remove the existing Call/Await compatibility signature read.  Before the I0
cutover, every such reader is classified as signature/header-only,
declaration-only, metadata-required, body-required, or lifecycle-only.

Only signature/header-only readers may use a temporary
`CompletedDraftSignatureViewV1` borrowed from the same collector-owned draft.
The view never clones a draft/header, exposes a body or `FunctionMetadata`, or
becomes a semantic cache.  A body- or metadata-required lowering-time reader
is a stop condition for this series.  This compatibility view is temporary;
its retirement owner is `FINALIZE0-CALLAWAIT-CLOSE0` after physical producer
receipts close result disposition.

Binding-SSA acyclic and recursive batches retain their sealed callable catalog
as sibling-header authority.  A prefix of collector headers must never replace
that catalog.

#### Root, synthetic, and issuer law

Main is not an exception:

```text
open module invocation
-> open Main attempt
-> complete and collect Main
-> aggregate only after all collection
```

The existing synthetic `condition_fn` receives a real empty-fact disposition
and uses the same collector port.  The collector checks its symbol inventory:
it creates the synthetic function only when `condition_fn` is absent.  Its
eventual removal remains `FINALIZE0-CONDITIONFN-RET0` work.

`FactSessionIssuerV1` belongs to `MirCompiler` lifetime, outside a replaceable
candidate `MirBuilder`.  Each `ModuleFactSessionV1` is opened explicitly by
that issuer and passed to the live or candidate builder.  Production code may
not create a local issuer inside a bare `MirBuilder::build_module` fallback;
that API must take an explicit session or become a proven test-only adapter.

#### Fixed downstream order and stop conditions

Once `HEADERPORT0-I0-G0` is green, the fixed order is:

```text
FACTSESSION0-ACTIVEBIND0-S0
-> FACTSESSION0-ACTIVEBIND0-P0
-> FACTSESSION0-I0
-> FACTSESSION0-G0
-> REMATFACT0-P0 / individual producer receipt closures
-> MODULETX0-P0
```

Stop and return to design selection if implementation requires any of:

```text
a second session-owned TypeContext
an independently keyed fact or draft collector
post-collection cleanup/admission validation
a cloned/current_module header cache
body or metadata access through the signature view
generation used as function identity
a local Builder-owned issuer fallback
PHI repair, TypePipeline/Call-Await redesign, JoinIR, MODULETX, or CUT0
```

The later fact-session cutover takes and seals the eight existing function
fact lanes with the collected draft.  It does not reopen this collector or
HeaderPort ownership decision.

## RAWPORT0-S0 closeout (2026-07-21)

`RAWPORT0-S0` closes with production consumers still at zero.

```text
ModuleLoweringPortV1
  owns only the invocation collector borrow
  loans headers through an HRTB-scoped read callback
  has no Builder, CompilationContext, TLS, cache, or second collector field

PendingFunctionSessionCloseV1
  captures a successful canonical child before parent restore
  exposes abort/drop restoration only in S0
  has no admission or collector API, preventing foreign admission pairing
```

The collector vocabulary now distinguishes A-plus
`CanonicalResolvedOwner(FunctionOwnerIdV1)` from legacy symbols, and its
disconnected fixtures use real `main`/`condition_fn` symbols with arities
`0`/`1`. Focused port, collector, and pending-terminal tests, the
HeaderPort guard, `cargo check`, formatting, and whitespace checks are green.
`RAWPORT0-M0` alone may thread this port through the audited raw/resolved
recursive descent and construct the port-branded terminal admission.

## RAWPORT0-M0-T0 closeout (2026-07-21)

`M0-T0` closes with zero production consumers. One owned
`ResolvedChildDraftAdmissionV1` carries only resolved source identity, symbol,
and arity for the A-plus single-child family; it carries no collector borrow.
The invocation port alone turns it into a collector admission and consumes one
pending resolved child in this order:

```text
capture pending child
-> prepare collector admission
-> seal matching draft
-> infallible collect
-> restore parent
```

The pending product does not accept `PreparedFunctionDraftAdmissionV1`; a
foreign collector pairing is therefore absent from the API. M0-T0 fixtures
prove collection after a sealed resolved child and zero collection after a
symbol mismatch. The port, pending-terminal, collector, and HeaderPort guard
remain disconnected from every production route. `M0-R0` must now parameterize
the one existing raw dispatcher and propagate the port across the complete
source-derived raw child census; it may not activate a single partial route.
Binding-SSA callable batches retain their sealed catalog and receive a separate
batch adapter later; this terminal cannot be reused to sequentially collect
them.

## RAWPORT0-LEGACYTERM0-S0 closeout (2026-07-21)

`LEGACYTERM0-S0` closes one disconnected legacy child terminal with zero
production callers.

```text
LegacyFunctionPendingSessionV1
  wraps the existing pending session and preserves Legacy body capture

LegacyChildDraftAdmissionV1
  owns only exact LegacySymbol + arity
  has no Clone, canonical owner, collector, Builder, or header authority

ModuleLoweringPortV1::complete_legacy_child
  captures -> prepares LegacyReplaceWholePair -> seals -> collects -> restores
```

The port creates its own prepared collector admission, so callers cannot pair a
legacy pending child with a foreign prepared admission.  Focused tests pin one
legacy collection, legacy whole-pair replacement, admission failure with zero
collection, and legacy-versus-resolved pending authority.  The HeaderPort
guard requires the non-Clone `LegacySymbol` shape, the explicit whole-pair
policy, and zero raw-dispatch/function-session production callers.  No raw Box
body is wired in this row.

`HEADERPORT0-RAWPORT0-LEGACYTERM0-P0` is next.  It must prove cleanup failure,
primary failure, unwind restoration, exact raw static/instance Box child body
port transport, and no resolved identity fabrication while preserving collector
delta zero on every non-success terminal.  Loop remains outside that proof and
behind `RAWPORT0-LOOPBRIDGE0-SELECT`.

## RAWPORT0-LEGACYTERM0-P0 closeout (2026-07-21)

`LEGACYTERM0-P0` proves the disconnected terminal through the exact production
shape; it does not yet wire the raw Box declaration branch.

```text
lower primary failure
  -> collector delta = 0 -> parent restored

lower primary + cleanup failure
  -> collector delta = 0 -> parent restored

successful lowering + cleanup failure
  -> collector delta = 0 -> parent restored

lower panic/unwind
  -> collector delta = 0 -> parent restored -> unwind resumes
```

The fixture enters a real outer function, calls
`ModuleLoweringPortV1::complete_legacy_child`, and observes the same collector
through the header port after each terminal.  It does not use the FACTSESSION
test observer or a duplicate raw expression dispatcher.  A separate parser
fixture takes the exact `run` body from one `static box` and one instance `box`;
the legacy port installs each exact body as the child `fn_body_ast` before the
lower closure receives control.  The guard requires all four failure/body
fixtures, the parser-backed body carrier, `LegacyReplaceWholePair`, and zero
raw production callers.

`HEADERPORT0-RAWPORT0-LEGACYTERM0-I0` is next.  It may wire only raw static and
instance `BoxDeclaration` child bodies through this existing legacy terminal.
It must retain the current legacy body/snapshot behavior, never route through a
resolved owner, and leave constructors, Lambda, Loop, finalization, and every
other raw child family untouched.  Loop remains a separate design selection.
