---
Status: answered and accepted with bounded implementation refinements
Date: 2026-07-16
Decision: Candidate B; S0 -> P0 -> atomic CUT0 -> G0
Baseline: f07af7070d
Parent: mirbuilder-p0c-mr-callable-scc-task-2026-07-16.md
Current blocker: P0c-MR-R0-D0 one-function self-call authority retirement
---

# P0c-MR-R0 One-Function Self-Call Retirement Consultation

## Accepted answer

Candidate B is accepted. The sole final callable authority is the owned
function-only Program route. Production cutover and old semantic-authority
deletion land atomically in one `CUT0` commit:

```text
P0c-MR-R0-S0
  -> P0c-MR-R0-P0
  -> P0c-MR-R0-CUT0
  -> P0c-MR-R0-G0
```

The next code-facing row is `P0c-MR-R0-S0`.

Two implementation refinements are required by current source evidence:

1. `CallableFunctionSyntaxViewV1` is retained. It is not an old one-entry
   authority: `CallableCatalogResolutionSourceV1::located_function` and
   `locate_catalog_function_v1` use it in the canonical CAT0/MP0 Program route
   to keep one declaration's header/body pairing exact. Removing it would add
   unrelated module-route churn. Only the old RootCallable constructor,
   sidecar, one-entry index facades, and exact-one admission authority retire.
2. After exact-one removal, ordinary body-only `compile_resolved` must select
   an explicit call-forbidden admission. The final policy vocabulary is
   `Forbidden | FiniteOneOrMore`, not an implicit default. A call-free body
   uses `Forbidden`; callable Program plans use `FiniteOneOrMore`.

These refinements do not change Candidate B, CUT0 atomicity, marker law,
fixtures, non-claims, or source-breaking removal decision.

## Question

P0c-MR-I1 and its runtime frame-restoration proof tail are closed. The
compiler now has two ways to represent exact static `i64` self recursion:

```text
old one-function route:
  bare FunctionDeclaration
  -> VerifiedResolvedSourceUnitV1::RootCallable
  -> one-entry VerifiedResolvedCallableForestV1/index
  -> ordinary compile_resolved ingress

new module route:
  function-only Program
  -> CAT0 catalog + MP0 resolved callable module
  -> shared graph inventory + SCC partition
  -> VerifiedRecursiveCallableModulePlanV1
  -> explicit compile_resolved_recursive_callable_module ingress
```

Which authority should remain after R0?

Local evidence favors **B — Program-only canonical authority**, with removal
performed only after one-function Program parity is green. Please confirm or
replace that recommendation, and define the exact task order.

## Current evidence

Baseline commit `f07af7070d` proves:

```text
multi-function recursive Program activation/proof fixtures: debug 7/7
multi-function recursive Program activation/proof fixtures: release 7/7
complete interpreter frame transaction fixtures: 6/6
quick gate: 66/66
recursive module publication: atomic
recursive module capability: exactly one module marker
route retry/fallback: 0
ownership operations: 0
```

Repository caller census:

```text
resolve_function_with_root_callable non-definition callers: 7
all 7 callers: cfg(test) modules
non-test production callers: 0

RootCallable semantic variant production constructor:
  resolve_function_with_root_callable only

recursive module minimum-function restriction:
  one check in VerifiedRecursiveCallableModulePlanV1::verify
  current law: functions_by_key.len() < 2 -> FunctionCardinality

CAT0/MP0 one-function Program structural blocker:
  none found outside that recursive-plan admission check
```

The ordinary `compile_resolved` entry cannot be deleted wholesale because
body-only canonical function families use it. R0 concerns only its
`RootCallable` self-call subfamily and the parallel one-entry callable
authority.

## Source authority that must remain fixed

The desired final source/module authority is:

```text
one owned function-only ASTNode::Program
one co-sealed Program/catalog source unit
one canonical-keyed resolved function map
one graph inventory
one SCC partition
one recursive typed plan
one unpublished-draft atomic transaction
```

Top-level functions remain separate single-root semantic owner forests. R0
must not turn `VerifiedSemanticOwnerForestV1` into a multi-root product.

## Non-authorities

R0 must not use any of these as identity or route truth:

```text
raw FunctionCall.name in Lower
physical MIR symbol spelling
MIR module function table
declaration order
FunctionOwnerId slot/order
runtime graph or SCC discovery
backend capability scanning as source admission
failure-driven probing of another route
```

## Candidate A — compatibility source facade

Keep a public bare-function entry temporarily, but make it a source adapter
that constructs/owns one function-only Program before semantic resolution.
After that conversion it must use the same CAT0/MP0/inventory/SCC/plan/module
transaction as every recursive Program.

```text
bare FunctionDeclaration
  -> explicit one-function Program source adapter
  -> canonical module authority
```

Requirements:

```text
old RootCallable semantic variant is not retained behind the facade
old one-entry activation witness is retired
adapter failure never retries old compile_resolved behavior
adapter owns the syntax conversion before any Builder effect
one-function Program and adapter produce normalized parity
```

Benefit: source/API compatibility.

Risk: a compatibility surface with no repository production caller may become
permanent and preserve unnecessary entry complexity.

## Candidate B — Program-only retirement

Admit singleton recursive SCCs through the canonical Program/module route,
convert all retained fixtures to one-function Programs, then remove:

```text
resolve_function_with_root_callable
ResolvedSourceUnitSemanticsV1::RootCallable
VerifiedResolvedCallableForestV1 production use
one-entry self-call activation/profile authority
old self-call-specific tests/guards after parity migration
```

Body-only `VerifiedResolvedSourceUnitV1::resolve_function` and ordinary
`compile_resolved` remain unchanged for call-free canonical functions.

Benefit: one source/module/callable authority and no compatibility seam.

Risk: an external caller not visible in the repository could depend on the
public bare-function constructor. If this API is considered supported, A may
be required for one deprecation window.

## Candidate C — permanent dual authority

Keep both the bare-function RootCallable route and Program/module SCC route as
permanent semantic owners.

This is locally rejected unless a non-overlapping source contract is proven.
It preserves duplicate callable indexing, admission, capability, tests, and
maintenance without current production evidence.

## Questions requiring explicit answers

1. Is Candidate B the correct final architecture, or does public API
   compatibility require Candidate A first?
2. Should `VerifiedRecursiveCallableModulePlanV1` admit `function count >= 1`
   exactly when its partition contains at least one recursive component?
3. Should a one-function recursive Program install both:
   - one per-calling-function direct-static capability; and
   - one module-level recursive capability?
4. If A is selected, what exact removal condition prevents the facade from
   becoming a second semantic authority?
5. Which old types may be deleted immediately, and which must remain as
   test-only or compatibility facades?
6. Should the old direct-call fixture set be migrated wholesale to Program
   parity, or retained only as lower-level profile/materializer unit tests?
7. What exact normalized parity excludes invocation-local owner IDs and source
   sites while still proving equivalent call target, ABI, MIR, runtime result,
   backend rejection, and no ownership operations?

## Proposed task order if B is accepted

```text
P0c-MR-R0-S0
  disconnected one-function Program admission
  change recursive plan minimum from 2 to 1
  require a recursive singleton SCC, one call site, and exact existing grammar
  production ingress delta = 0

P0c-MR-R0-P0
  one-function Program parity fixtures
  target/signature/call-row/Binding-SSA/MIR/runtime/backend parity
  exact rejection parity and declaration/source-site normalization
  production ingress delta = 0

P0c-MR-R0-I1
  activate one-function Program through the existing explicit recursive ingress
  one module marker, one per-calling-function direct-static marker
  no new route and no fallback

P0c-MR-R0-RET0
  migrate retained old-route fixtures
  remove RootCallable production constructor/variant and one-entry activation
  preserve body-only compile_resolved families

P0c-MR-R0-G0
  repository caller/producer zero guards
  old route retry strings/callers = 0
  final current-doc closeout
```

If A is required, insert `R0-A0` between P0 and I1. It may add only the
bare-function-to-Program source adapter; it must not retain or recreate the
old semantic route.

## Required pass fixtures

```text
one-function terminating self recursion
one-function nonterminating recursion -> stable MAX_CALL_DEPTH error
post-depth-error interpreter reuse
parameter/return contract validation on recursive frames
call result as local / assignment / binary operand / final return
post-If Binding SSA argument
nested/repeated self calls where finite-call grammar already allows them
VM result parity with retained old fixtures
one recursive module marker
one direct-static marker for the calling function
CopyOwned/DestroyOwned/ReleaseStrong = 0
```

## Required reject fixtures

```text
one-function acyclic Program on recursive ingress
zero call sites
unknown target / wrong arity
zero parameters / non-i64 signature
MethodCall / receiver / Loop / early Return / Lambda
non-VM backend before backend effects
old bare route failure followed by any route retry
recursive route failure followed by bare/acyclic/legacy retry
```

## Removal counters

```text
ResolvedSourceUnitSemanticsV1::RootCallable production constructors = 0
resolve_function_with_root_callable production callers = 0
VerifiedResolvedCallableForestV1 production consumers = 0
old exact-one self-call activation witnesses = 0
old self-call route retries/fallbacks = 0

raw Lower FunctionCall.name reads = 0
MIR-table source resolution = 0
second BindingRef -> ValueId map = 0
incremental publication = 0
unsupported backend fallback = 0
ownership operations = 0
```

## Implementation may claim after R0

```text
one canonical Program/module authority handles singleton and multi-function SCCs
one-function exact-i64 self recursion uses the same graph/SCC/plan/transaction
old one-entry self-call semantic activation authority is retired
body-only canonical compile_resolved behavior is preserved
all backend and ownership non-claims remain unchanged
```

## Implementation must not claim

```text
general callable support
termination or deep-recursion support
tail-call optimization
effect fixed point or purity
MethodCall / receiver / Lambda / Loop / early Return
Box/View/Shared ABI or ownership operations
imports / plugins / FFI
LLVM / Wasm / PyVM support
source compatibility unless A is explicitly selected and verified
```

## Stop conditions

Stop if R0 requires:

1. a second Program/catalog/graph/SCC authority;
2. target re-resolution from names, symbols, MIR, or runtime state;
3. changing `VerifiedSemanticOwnerForestV1` to multi-root;
4. retaining the old activation witness behind a compatibility facade;
5. backend marker inference from emitted MIR;
6. fallback or canonical route retry;
7. incremental/callee-first publication;
8. ownership, expression, signature, backend, or runtime widening;
9. deleting body-only `compile_resolved` support;
10. touching a source/check file at or above 800 lines.

## Requested final decision lock

Please return:

```text
selected candidate: A or B (C only with a new proven invariant)
final source authority
one-function recursive admission law
backend marker law
exact type/constructor retirement list
task order
pass/reject fixtures
counters/guards
implementation may/must-not claim
stop conditions
```

The key decision is:

> Is the bare-function entry worth one temporary source-only compatibility
> facade, or should Hakorune immediately make the function-only Program/module
> route the sole callable authority now that repository production callers of
> `resolve_function_with_root_callable` are zero?
