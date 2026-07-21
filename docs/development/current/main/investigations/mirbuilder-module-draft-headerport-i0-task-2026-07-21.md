---
Status: `HEADERPORT0-RAWPORT0-SELECT` is closed; `HEADERPORT0-RAWPORT0-S0` is next
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
The raw recursive re-entrancy audit below is a prerequisite selection.  It is
now closed; the first code-facing row is the passive `RAWPORT0-S0` vocabulary.

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

## Exact task order

```text
HEADERPORT0-RAWPORT0-S0
  Passive `ModuleLoweringPortV1`, pending child-terminal, and truthful
  collector-key vocabulary.  Fix the disconnected main/condition_fn fixture
  spellings.  Production consumers = 0.

-> HEADERPORT0-RAWPORT0-M0
   Complete port-preserving raw recursive descent and resolved child-terminal
   seams.  Reuse descent traits; do not clone lowering trees.  Existing V1
   routes remain production.

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

Only after `HEADERPORT0-I0-G0`:

```text
FACTSESSION0-ACTIVEBIND0-S0/P0
-> FACTSESSION0-I0/G0
-> REMATFACT0-P0
```

The later fact-session cutover takes and seals the eight existing function
fact lanes with the collected draft.  It does not reopen this collector or
HeaderPort ownership decision.
