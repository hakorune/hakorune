---
Status: SITE0-R0-EXPR0-M0-V0-P0 closed; V0-G0 is next
Date: 2026-07-18
Decision: Candidate A owned single-use activation plan plus located legacy input
Baseline: fe2d61baa0
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Scope: exact-site same-module exact-i64 call-result activation before Builder effects
---

# Callable-result I0 activation design stop

## Current finding

Three independent worker audits and local source review agree that the closed
result catalog cannot be connected to the legacy Builder honestly by a small
emitter patch. The declaration, source-target, result, and normalized P0
products are green. The missing boundary is the production activation owner.

The exact gaps are:

1. `MirBuilder::lower_root` is the only complete pre-body hook, but it installs
   the declaration catalog into live `CompilationContext` before later
   indexing and body-lowering effects.
2. `VerifiedSourceStaticCallTargetCatalogV1` borrows that declaration catalog,
   and `VerifiedSameModuleCallableResultCatalogV1` borrows both catalogs.
   Storing all three beside the owner creates self-reference or a forbidden
   cloned authority.
3. Legacy `build_expression -> build_method_call` carries neither a canonical
   caller key nor `SourceExprSiteV1`. Exact rows cannot be claimed at emission
   without a site-aware lowering boundary.
4. The selected caller `ParserBox.static_const_parse_add/2` is an
   `InstanceBoxMethod`; the result solver proves static target declarations
   and currently rejects non-static callers during target pairing.
5. Generic call-result annotation uses function-name/module lookup after
   emission. It is not the sealed exact-site authority.

This was a new authority/lifetime contradiction discovered after P0. A second
three-worker audit has now selected one mechanically unique implementation
shape under `LocalMechanicalSelectorAuthorityV1`; external consultation is no
longer required.

## Local selection evidence

All three worker audits converge on these facts:

```text
Candidate A:
  selected

Candidate B stack-borrowed Builder session:
  rejected

exact site producer:
  existing resolved_semantics::shadow traversal

exact lowering carrier:
  located body/statement/expression input

mutable path cursor or encounter-order ledger:
  rejected
```

`MirBuilder` and `CompilationContext` are lifetime-free owned state, and the
existing `CanonicalModuleLoweringSessionV1` already isolates an owned
candidate. Candidate B would require lifetime-parameterizing the Builder or
threading borrowed catalogs across nearly every legacy lowering API. Candidate
A instead normalizes the borrowed proofs in one lexical preflight, drops them,
then moves the sole declaration catalog and owned rows into the candidate.

The existing shadow resolver is the sole complete owner-local structural
traversal that already produces `SourcePathSegmentV1` and lexical qualified-
receiver facts. It can observe all MethodCall sites without a second walker.
Legacy lowering currently has 81 expression-entry calls across 33 files and
also creates synthetic nodes, so a mutable push/pop cursor would be fragile.
Located inputs co-seal caller, exact path, and the actual moved/borrowed node;
synthetic nodes remain explicitly unlocated and cannot claim activation rows.

## Source authority

```text
owned function/program root
  -> complete declaration catalog
  -> immutable explicit-import view
  -> exact caller + SourceExprSite MethodCall inventory
  -> canonical static target rows
  -> exact-i64 result rows
  -> owned single-use plan or one scoped lowering session
  -> exact-site consumption during ordinary lowering
```

Static result inference may remain target-contract-only. Instance caller
identity is needed for source-site ownership, not as a new instance-method
result-inference family.

## Non-authorities

```text
mutable using_import_boxes
method/function/MIR symbol parsing
AST equality, source span, or encounter-order matching
current_module function signatures
final MirFunction metadata
callee-first lowering or re-lowering
legacy name-based result annotation
runtime tags or GenericLoop inference
retry or fallback
```

## Candidate A — owned single-use activation plan (selected)

Preflight borrows declaration/target/result products only while deriving one
non-Clone owned plan. The borrowed proofs are dropped before the plan and
declaration catalog are installed into an isolated candidate Builder session.

Suggested shape:

```rust
pub(crate) struct VerifiedCallableResultActivationPlanV1 {
    declaration_catalog:
        VerifiedSameModuleCallableDeclarationCatalogV1,
    rows_by_caller:
        BTreeMap<CanonicalSameModuleCallableKeyV1,
                 Box<[VerifiedCallableResultActivationSiteV1]>>,
}

pub(crate) struct VerifiedCallableResultActivationSiteV1 {
    site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    required_i64_arguments: Box<[u32]>,
    result: CallableResultActivationDispositionV1,
}
```

The exact private fields are implementation-owned under the laws below. The
plan owns no duplicate AST body, import map, target/result catalog clone,
ABI/effect table, or second callable index.
It is a normalized consumption product, not another semantic solver.

Advantages:

```text
no self-reference in CompilationContext
borrowed proof lifetime ends before Builder mutation
candidate Builder owns one non-Clone plan
failure-before-live-effects boundary is explicit
```

Recursive AST lowering receives located body/statement/expression inputs that
carry the caller key, exact `SourceExprSiteV1`, and syntax node. Child inputs
are constructed only through one shared child-role-to-path-segment law. A
caller-scoped ledger consumes rows by exact site, never by execution ordinal.

## Candidate B — stack-scoped borrowed lowering session

Keep all products on the stack and borrow them through a
`VerifiedCallableResultLoweringSessionV1<'root>` while ordinary body lowering
runs. Thread the caller key and exact source path through recursive expression
and statement lowering.

This retains the original proofs directly, but introduces wide lifetime/API
threading and still needs candidate/live Builder isolation. Existing ownership
and transaction structure mechanically reject it for this row.

## Rejected shapes

```text
store borrowed catalogs beside their owner in CompilationContext
Clone/Arc declaration, target, or result authority
rebuild target/result facts during emission
select by target name, symbol, AST equality, span, or ordinal
use mutable import state as the seal
publish through legacy name/module result annotation
lower callee first or retry a failed route
```

## Resolved local decisions

1. Consume the borrowed proof chain into one owned, non-Clone, single-use
   activation plan before opening the candidate Builder.
2. Reuse the existing shadow traversal for complete MethodCall inventory and
   lexical facts. Share one neutral child-role-to-`SourcePathSegmentV1` policy
   with compiler source views and located legacy lowering.
3. Use located legacy inputs, not a mutable cursor. Raw legacy subtree
   delegation is allowed only when the ledger proves that no activation row
   exists under the subtree prefix.
4. Admit `InstanceBoxMethod` only as exact caller/site ownership. Targets and
   result inference remain `StaticBoxMethod`; no ParserBox result is inferred.
5. Preflight the root plus explicit imports before live mutation, lower once in
   `CanonicalModuleLoweringSessionV1`, and commit the completed candidate once.

## Task order after the decision

```text
R0-CALLABLE-RESULT-I64-CATALOG0-I0-ACTIVATION-D0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-PATH0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-A0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-L0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-R0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-C0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-CUT0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-G0
```

### PATH0 — one structural path policy and MethodCall inventory

```text
production behavior delta: 0
production result consumers: 0
```

Extract one neutral child-role-to-`SourcePathSegmentV1` policy from the
existing compiler source-view/shadow rules. Extend the sole shadow traversal
with a read-only all-MethodCall observation mode that emits exact caller/site
rows plus existing Bound/ProvenUnbound lexical dispositions. It must cover the
actual ParserBox syntax, nested/repeated calls, current-owner calls, qualified
calls, and dynamic/bound exclusions. No second AST walker is added.

### A0 — disconnected owned activation product

```text
production behavior delta: 0
production consumers: 0
```

Consume the borrowed proof chain into one owned non-Clone plan. Prove
brand correspondence, instance-caller/static-target separation, site
multiplicity, required-argument bounds, and reorder parity. Retain no AST and
no duplicate target/result authority. Every observed MethodCall receives an
explicit `SelectedExactI64` or `Unselected` disposition so missing rows never
mean legacy fallback. Borrowed target/result products are dropped before the
declaration catalog moves into the plan.

### SITE0-L0 — located legacy carrier vocabulary

```text
production behavior delta: 0
production result consumers: 0
```

Add privately constructed located body/statement/expression inputs carrying
caller key, exact structural site, and syntax node. Child construction consumes
only the neutral PATH0 role policy. Synthetic nodes are `Unlocated` and cannot
claim rows.

### SITE0-R0 — behavior-neutral located recursive descent

Use Refactor Series Mode. Begin one caller ledger from the activation plan,
lower the caller body through located inputs, and finish with exact coverage.
Raw legacy subtree delegation is allowed only for a prefix with zero active
rows. Wrong, duplicate, missing, foreign, and drifting rows reject typed. This
row changes no accepted program or result publication.

The behavior-neutral series is fixed before implementation:

```text
SITE0-R0-LDG0
  disconnected caller ledger
  exact source-order claim + prefix-zero proof + exact finish
  production consumers = 0

SITE0-R0-BLK0
  one behavior-neutral legacy block orchestration driver
  raw Vec<ASTNode> port remains the sole selected implementation
  located/ledger imports and production consumers remain zero
  production result publication = 0

SITE0-R0-EXPR0-E0
  one behavior-neutral body/statement/expression child-lowering port
  raw legacy port remains the sole selected implementation
  located/ledger imports and production consumers remain zero

SITE0-R0-EXPR0-M0-ARG0
  one behavior-neutral associated-input call-argument descent port
  preserve preflight, left-to-right lowering, diagnostics, and failure order
  raw AST port remains the sole selected implementation
  located/ledger imports and production result publication remain zero

SITE0-R0-EXPR0-M0-ROUTE0
  S0: one disconnected associated-input MethodCall child port
      raw port remains the sole implementation; production consumers = 0
  GUARD0: restore one exact recursion-depth guard around every raw expression descent
      public and nested child entries share one guard owner without double counting
  R0: thread exact reserved-route child demand
  M0: thread TypeOp/static/env/me/standard child demand
      preserve evaluation and special-preflight order

SITE0-R0-EXPR0-M0-V0
  split value-level terminal emission after route-specific syntax preflight
  preserve effects, destination allocation, type publication, and diagnostics
  located/ledger imports and production consumers remain zero

SITE0-R0-EXPR0-L0
  disconnected stack-scoped located lowering session
  claim MethodCall rows at entry and descend receiver/arguments in PATH0 order
  raw delegation requires an exact inactive-prefix proof
  production callers and result publication remain zero

SITE0-R0-EXPR0-C0
  connect the located session to the BLK0 driver port
  preserve location through active If/body/statement recursion
  consume the caller ledger only after successful body completion
  production callers and result publication remain zero

SITE0-R0-P0
  exact caller-row coverage and behavior-parity closeout
  accepted grammar/result publication delta = 0
```

The EXPR0 split follows a read-only worker audit plus local source review which
found that a located wrapper around the current
`build_expression` entry cannot preserve the exact site law: legacy child
lowering is distributed across body, statement, and expression helpers, and
the current MethodCall entry re-lowers receiver and arguments from AST. E0
therefore introduces one neutral recursive child-lowering port while retaining
the raw legacy implementation as the sole selected port. It changes no
accepted grammar, route, MIR, runtime behavior, or result publication, and it
imports no located or ledger authority.

The EXPR0 series is fixed as E0 -> M0-ARG0 -> M0-ROUTE0-S0 -> M0-GUARD0 ->
M0-ROUTE0-R0 -> M0-ROUTE0-M0 -> M0-V0 -> L0 -> C0.
ARG0 first centralizes argument preflight, exact left-to-right child descent,
and the existing undefined-value observation behind one associated-input port.
ROUTE0 then lets each existing route request only the children it evaluates:
static/env/me/reserved receiver syntax remains unevaluated, `__mir__` labels
and TypeOp type strings remain syntax-only, and standard receiver evaluation
remains before its arguments. V0 finally extracts only post-preflight
value-level terminal emission. L0 may then create a
stack-scoped located session separate from `MirBuilder`; the activation plan,
source view, and ledger must never be stored in Builder state, cloned, shared,
or reconstructed by a second AST walk. L0 claims a MethodCall before lowering
its receiver and arguments in PATH0 order, and raw delegation is legal only
after an inactive-prefix proof. C0 connects this session through the BLK0 port
and finishes the ledger after the complete caller body succeeds. CUT0 remains
the only production activation point.

The existing LDG0 ledger borrows one
catalog-owned caller row slice from the A0 plan and is non-Clone. It may expose
a disposition only after an exact located expression claim. Claims are
source-order exact: an already consumed row is `Duplicate`, a later known row
is `WrongOrder`, a row outside the caller inventory is `Unexpected`, and
`finish` rejects the first missing row. A located body/statement/expression
prefix may additionally receive one immutable proof that its complete subtree
contains zero activation rows; unlocated or foreign-plan inputs cannot receive
that proof. The LDG0 slice has no Builder, MIR, runtime, backend, or production
lowering consumer.

### SITE0-C0 — preserve location through call planning

Carry the exact site token through any `CoreEffectPlan`/immediate MethodCall
normalization that consumes a located call. Provenance must never be recovered
after plan creation. Production result publication remains zero.

### CUT0 — atomic production activation

```text
production behavior delta:
  selected same-module exact-i64 call results become available during lowering
production result publisher: exactly 1
```

Required order:

```text
1. preflight root plus explicit imports before live Builder mutation
2. seal declaration, site/target, result, and activation products once
3. open the existing isolated candidate Builder session
4. install declaration catalog plus activation plan once
5. lower each body once with its caller/site ledger
6. claim the exact row before selected call-route mutation
7. lower arguments in source order
8. require exact Integer at every required ordinal
9. emit sealed Callee::Global(target.mir_symbol_projection())
   through a no-resolver/no-fallback entry
10. only after successful Call emission publish dst as MirType::Integer
11. finish and commit the candidate once
```

The selected path must not invoke legacy name/module result annotation.

### G0 — guards and closeout

Use a small I0 checker or extend the existing small same-module inventory. Do
not extend the roughly 765-line S0 checker.

```text
declaration catalog preflight producer/install = 1
immutable import view = 1
structural AST traversal owner = 1
child-role path policy owner = 1
exact source-site inventory = 1
source target producer = 1
result solver = 1
activation plan producer = 1
located caller ledger consumer = 1
same-module result publisher = 1
candidate commit = 1
instance result inference = 0

bare target lookup on selected path = 0
legacy result annotation on selected path = 0
final metadata reads = 0
physical/name/span/AST/encounter-order inference = 0
retry/re-lowering/fallback = 0
second catalog/type/owner map = 0
source/check files >= 800 lines = 0
```

## Failure-before-effects law

```text
preflight/brand/route/result mismatch:
  candidate/live Builder instruction and type effects = 0

site/target/required-argument mismatch:
  Call emission = 0
  result type publication = 0

Call emission failure:
  result type publication = 0

later candidate lowering failure:
  live compiler/module publication = 0

rejected compile followed by valid compile:
  pass on the same compiler instance
```

Argument-expression effects may precede a required-representation mismatch.
CUT0 must not claim whole-call rollback; whole-compilation discard belongs to
the isolated candidate session.

## PATH0 closeout

PATH0 now owns one neutral `ExprChildRoleV1` / `BodyChildRoleV1` /
`SourceBodyKindV1` policy in `resolved_semantics`. Compiler located views,
shadow traversal, and callable-result expression/function proof consume that
one policy; their prior child-segment decision tables are gone.

The existing shadow resolver traversal has one disconnected all-MethodCall
observation mode. Exact call and receiver sites classify receivers as
`CurrentOwner`, lexical `Bound`, positive `ProvenUnbound`, or `Dynamic`. The
product carries no method spelling, arity, AST, target, ABI, or result fact.
The existing requested qualified-receiver observer remains unchanged.

The actual `ParserBox.static_const_parse_add/2` source is guarded at 15
MethodCall receivers: nine current-owner `me`, four bound `text`, and two
proven-unbound `ParserStringUtilsBox.skip_ws` sites. Focused shadow tests cover
exact nested/repeated sites and every receiver class. The PATH0 checker fixes
one policy owner, one traversal, zero production observers, no direct child
path bypass in the selected proof/traversal surfaces, and the 800-line cap.
Production behavior, result publication, Builder, MIR, runtime, and backend
deltas remain zero. A0 is next.

## A0 closeout

`I0-A0` is closed. One boxed declaration catalog remains at a stable private
brand while borrowed source-target and static-result products are built and
normalized. After those borrows are dropped, one non-Clone plan owns the
catalog plus opaque caller-keyed activation rows. The plan retains no AST,
import view, target catalog, result catalog, ABI/effect table, or second
callable index.

The result catalog now accepts an exact instance declaration key only as a
source-target caller/site owner; target membership and result solving remain
strictly static. No instance declaration enters `prove_function`. A0 joins
each PATH0 MethodCall observation with its exact static target and that
target's static result disposition. Exact-i64 selections retain the callee's
required call-argument ordinals, checked against target arity. Every other
observed MethodCall receives an explicit unit `Unselected` row, so absence is
never a fallback signal.

The actual `ParserBox.static_const_parse_add/2` body seals 15 rows: exactly two
`ParserStringUtilsBox.skip_ws/2` selections with required argument `[1]`, and
13 explicit unselected rows. Instance-caller/static-target separation,
declaration reorder parity, foreign equal-catalog brand rejection, opaque
single-use decomposition, result/source-target regression suites, one A0
structural checker, cargo check, and the 800-line cap are green. Production
consumers and Builder/MIR/runtime/backend behavior remain zero. `SITE0-L0` is
next.

## SITE0-L0 closeout

`I0-SITE0-L0` is closed. One private non-Clone legacy source view can be
constructed only from the sealed A0 activation plan plus one exact catalog
caller. It borrows the catalog-owned declaration/key and constructs located
body, statement, and expression inputs carrying exact function-relative sites.
Equal-looking carriers from a foreign activation plan reject by private brand.

PATH0 now resolves each child role into its structural segment/kind together
with the borrowed child node/body in one neutral decision. Existing compiler
navigation remains a thin consumer and preserves its prior missing-child error
priority. Grouped-assignment targets are explicitly synthetic path-only
children; missing optional/indexed children are distinct from role mismatch.
No second AST walker or segment table was added.

Body, statement, and expression inputs each preserve `Located` versus
`Unlocated`. Children of unlocated syntax remain unlocated, and only a located
expression exposes `(caller, SourceExprSiteV1)` for a future activation claim.
Root/local/nested argument/If-body/reorder/foreign/wrong-role/unlocated fixtures,
the existing source-view suite, A0 and SITE0-L0 guards, cargo check, and line
caps are green. Production constructors/consumers, Builder/MIR/runtime/backend
behavior, and result publication remain zero. `SITE0-R0` is next.

## SITE0-R0-LDG0 closeout

`I0-SITE0-R0-LDG0` is closed. One non-Clone caller ledger borrows exactly one
catalog-owned caller row slice from the A0 plan. Located MethodCall inputs must
carry the same private plan brand and caller pointer before an exact row may be
claimed. Selected and explicit Unselected rows share one source-order law;
duplicate, wrong-order, wrong-node, foreign-plan, missing, and prefix-active
cases reject typed. The ledger retains only a claimed-site set and creates no
second target, result, or source-path authority.

A located body, statement, or expression can receive one immutable inactive-
prefix proof only when every A0 row lies outside its exact structural prefix.
Both Selected and Unselected rows count, root-body means the empty prefix, and
unlocated inputs cannot prove raw delegation safe. The PATH0 guard now
classifies A0's one intended disconnected observer separately from unexpected
runtime consumers. Focused ledger 4/4, callable-result 43/43, PATH0/A0/L0/LDG0
guards, cargo check, pointer and line guards are green. Production ledger
consumers, Builder/MIR/runtime/backend behavior, accepted grammar, and result
publication remain zero. `SITE0-R0-BLK0` is next.

## SITE0-R0-BLK0 closeout

`I0-SITE0-R0-BLK0` is closed. One private legacy block descent driver now
owns scope lifetime, the existing suffix-router sequencing, last-value and
empty-Block Void publication, and both fallible post-statement termination and
scope-leave termination observations. One raw `Vec<ASTNode>` port remains the
sole selected implementation. It owns source navigation and legacy statement
lowering only; located inputs, the caller ledger, activation dispositions, and
callable-result publication remain absent from the production boundary.

The extraction deliberately preserves the legacy suffix behavior, including
the separately tracked final-consumed-suffix index defect: after a suffix
reports `consumed`, the same iteration still lowers the resulting index. BLK0
adds no `continue`, bounds repair, retry, or fallback. Five focused fixtures
fix empty-block exactly-one Void publication, source order and last value,
termination before an invalid trailing statement, and lexical-scope restoration
on success and failure. Focused driver 5/5, callable-result 43/43, and the
PATH0/A0/L0/LDG0/BLK0 guards are green. The archived Phase-142 suffix smoke
still stops before this driver at `MissingTransientType { init: ValueId(3) }`;
the exact same failure occurs on baseline `61d40b26d2`, so it is recorded as a
pre-existing non-gate rather than a BLK0 regression. Builder/MIR/runtime/backend
behavior, accepted grammar, and result publication remain unchanged.
`SITE0-R0-EXPR0` is next.

## SITE0-R0-EXPR0-E0 closeout

`I0-SITE0-R0-EXPR0-E0` is closed. One associated-input
`RecursiveChildLoweringPortV1` owns the neutral body, statement, and expression
entry boundary. One private raw implementation is selected synchronously by
exactly three legacy facades and is never stored in `MirBuilder`, cloned,
shared, retried, or selected by input probing. FastMem now enters through the
same body facade; the existing BLK0 driver remains the raw body leaf.

Existing helper recursion, including direct `build_expression_impl` calls,
remains an explicit inactive raw leaf. E0 imports no located carrier,
activation plan, caller ledger, MethodCall route, or result authority. Five
focused associated-input/exact-once/raw-parity/reuse fixtures, BLK0 5/5,
callable-result 43/43, E0/BLK0/LDG0 guards, release check, formatting, and line
caps are green. The broad FastMem filter retains two pre-existing assertion
drifts; both reproduce with the old direct block leaf and are not E0
regressions. Accepted grammar, MIR/runtime behavior, and result publication
remain unchanged. `SITE0-R0-EXPR0-M0-ARG0` is next.

## SITE0-R0-EXPR0-M0-ARG0 closeout

`I0-SITE0-R0-EXPR0-M0-ARG0` is closed. One associated-input
`CallArgumentDescentPortV1` extends the existing E0 recursive child port, so
there is still one expression-lowering authority. The driver owns the exact
legacy order: whole-list moved-state preflight before child effects, then
per-index record-value precheck, associated expression input construction,
one left-to-right E0 descent, and the existing undefined-value observation
immediately after each successful child. The first failure preserves earlier
effects, lowers no later argument, and never retries.

One raw AST implementation remains selected by the existing
`build_call_args` facade; its ten production callers are unchanged. Receiver
evaluation, reserved/static/env/me/standard route policy, TypeOp strings,
`__mir__` labels, target/effect/result publication, located inputs, and the
caller ledger remain outside ARG0. Five focused associated-input/order/empty/
failure/reuse/raw-MIR fixtures, the existing moved-state 3/3 and E0 5/5
regressions, callable-result 43/43, PATH0/A0/L0/LDG0/BLK0/E0/ARG0 guards, and
the release check are green. Accepted grammar, MIR/runtime/backend behavior,
and result publication remain unchanged. `SITE0-R0-EXPR0-M0-ROUTE0` is next.

## SITE0-R0-EXPR0-M0-ROUTE0-S0 closeout

`I0-SITE0-R0-EXPR0-M0-ROUTE0-S0` is closed. One private
`MethodCallDescentPortV1` adds an associated MethodCall input above the existing
E0 expression and ARG0 argument ports. It exposes one borrowed syntax view and
separate receiver-only and arguments-only descent primitives. A distinct-input
fixture proves that either primitive descends only the child family it requests;
the raw AST implementation remains the sole implementation and production
consumers remain zero.

S0 deliberately publishes no speculative route-demand or completed-value
product. Route classification, syntax-only operands, reserved preflight,
terminal emission, effects, types, results, located inputs, and the caller
ledger remain outside this port. Inactive raw terminal delegation will still
require the existing ledger's inactive-prefix proof in located lowering.

The closeout audit found one earlier ARG0 substrate drift before any ROUTE0
production wiring: nested raw expression descent now reaches
`build_expression_impl` without the legacy recursion-depth guard. R0 is
therefore not authorized yet. `SITE0-R0-EXPR0-M0-GUARD0` must first centralize
one exact guard owner shared by the public and nested raw expression entries,
preserving maximum depth, diagnostic, restoration, and failure order without
double counting. This is behavior restoration only; it may add no route,
grammar, result, located, or ledger authority.

## SITE0-R0-EXPR0-M0-GUARD0 closeout

`I0-SITE0-R0-EXPR0-M0-GUARD0` is closed. The raw implementation of the
existing `RecursiveChildLoweringPortV1` is now the sole recursion-depth guard
owner for raw expression descent. The public `build_expression` facade only
selects that raw port, so public and nested ARG0/MethodCall child descent reach
the same guard exactly once. The limit remains 200 and the existing fatal
diagnostic fields and error text remain unchanged.

Ordinary lowering failure and limit failure both restore the entry depth before
the Builder is reused. The overflow fixture fixes the prior leaked-depth bug;
no later child effects occur. E0, ARG0, and ROUTE0 focused regressions plus the
callable-result suite remain green. No route, accepted grammar, MIR result,
located input, ledger, runtime, backend, or ownership authority is added.
`SITE0-R0-EXPR0-M0-ROUTE0-R0` is next.

## SITE0-R0-EXPR0-M0-ROUTE0-R0 closeout

`I0-SITE0-R0-EXPR0-M0-ROUTE0-R0` is closed. One private non-Clone
`PreparedReservedMethodCallV1` consumes the neutral reserved-route decision
once and co-seals the active FastMem region when required. The production
MethodCall entry constructs one stack-scoped raw MethodCall input and port,
selects the reserved driver once, and reuses the same owned input for the
ordinary member route. Reserved receiver descent remains zero.

The exact child matrix is preserved. MIR-debug labels are syntax-only;
`mark` descends no arguments and `log` descends only source indices one and
later through one indexed E0 primitive with the existing suffix-relative
undefined-value observation. REPL alone uses the full ARG0 boundary. FastMem
function and method facades share one intrinsic/preflight core, while each
supplies its prior indexed expression descent; arity and table/range syntax
preflight still precede child effects. Ordinary and reserved-failure decisions
descend no children. Terminal emission, effects, destination/type/result
publication, located inputs, ledger authorization, and accepted grammar remain
unchanged.

The actual A0 Parser fixture has zero reserved-route sites: its 15 rows remain
9 current-owner `me` calls, 2 selected static `skip_ws` calls, and 4 standard
String calls. A future located cutover must stop if a non-evaluated MIR-debug
label/`mark` child contains an activation row, because A0 currently inventories
all MethodCall children while the legacy route intentionally evaluates fewer.
R0 does not weaken the existing inactive-prefix proof or ledger.

Reserved 11/11, MethodCall port 4/4, ARG0 5/5, E0 6/6, callable-result 43/43,
reserved policy 5/5, moved-state 3/3, FastMem non-drift 72/72, all structural
guards, release check, pointer guard, formatting, and line caps are green. The
two broad FastMem assertion drifts documented at E0 remain reproducible and are
not R0 regressions. `SITE0-R0-EXPR0-M0-ROUTE0-M0` is next.

## SITE0-R0-EXPR0-M0-ROUTE0-M0-TYPEOP-GUARD0 closeout

The M0 diff audit found one pre-existing raw recursion-depth bypass in the
source TypeOp shortcut. Its receiver used `build_expression_impl` directly,
while every other selected recursive child enters through the E0 raw
expression guard. This is closed as a prerequisite row rather than hidden in
the behavior-neutral route refactor.

The TypeOp source shortcut remains the selected production owner through this
row. Only its receiver descent changes from the unguarded raw implementation
call to the existing guarded expression facade. The type spelling remains
syntax-only, the raw depth limit remains 200, MethodCall's separate depth limit
still does not apply, and TypeOp emission occurs only after receiver success.

The boundary fixture fixes the public-source accounting: an entry depth of 198
can lower the outer MethodCall and its receiver at depth 200; an entry depth of
199 rejects the receiver attempt at depth 201, restores the entry depth,
publishes no TypeOp, and leaves the Builder reusable. This row claims only
normalization onto the existing recursion resource law. Route/result/located/
ledger/grammar/backend authority remains unchanged.

## SITE0-R0-EXPR0-M0-ROUTE0-M0 task lock

Three independent audits agree that M0 needs no new design consultation. After
the TypeOp guard prerequisite above, it is one behavior-neutral Refactor Series
with this exact order:

```text
M0-S0  generic associated-input MethodCall driver
M0-H0  record-helper test split and indexed child boundary
M0-I0  TypeOp/static/env/me/standard route wiring
M0-P0  child-demand and normalized MIR parity
M0-G0  structural guards and closeout
-> SITE0-R0-EXPR0-M0-V0
```

The existing non-Clone `MemberCallRoutePlan` remains the sole prepared member
route product. The production entry constructs one raw MethodCall input and one
stack-scoped port, selects the reserved route once, and gives the same input to
the ordinary member driver without `into_parts`, AST retry, or route
reclassification.

The exact demand matrix is:

```text
TypeOp is/as:
  receiver E0 exactly once
  syntax-only type argument descent 0

StaticReceiver / StaticThis:
  receiver descent 0
  scalar/record-helper preflight before child effects
  ordinary arguments ARG0 left-to-right

EnvMethod:
  receiver/iface syntax descent 0
  sealed spec before child effects
  arguments ARG0 left-to-right

MeCall:
  source receiver descent 0
  existing me-binding/helper/module probes keep their order
  lowered-function arguments ARG0 before the existing strict arity check

Standard:
  receiver E0 exactly once
  weak/helper/setter preflight after the receiver and before arguments
  ordinary arguments ARG0 left-to-right
```

Record-helper scalarization remains a distinct terminal authority. A
record-local argument binds its existing `ValueId` without descent; only
non-record arguments descend through the indexed E0 primitive in source-index
order. The full ARG0 preflight must not be imposed on this custom terminal.
Helper body lowering is not MethodCall child descent and remains parked outside
M0. Tests are physically split from `record_helper_args.rs` before adding the
port adapter so every touched source/check file remains below 800 lines.

`property_reads.rs` is an already-materialized receiver consumer. M0 must not
fabricate a MethodCall AST or re-run source preflight for it. Its existing
value-level standard handler remains available while the source MethodCall
path gets a thin associated-input wrapper. A need for duplicate preflight or a
fake source carrier is a stop condition for M0.

The actual A0 inventory remains exactly 15 entry claims: 9 current-owner calls,
2 selected static `skip_ws` calls, and 4 unselected standard String calls.
Selected and unselected rows are both active. Each outer `skip_ws` entry is
claimed before its nested argument row. Env, TypeOp, and reserved rows are zero
in that corpus, so route fixtures must prove those dormant demands separately.

M0 may claim only exact child-demand threading and raw behavior/MIR parity. It
may not add a completed-value terminal, result/type/effect authority, located
input, caller-ledger rule, fallback, retry, accepted grammar, or backend/runtime
behavior. Stop if route order changes; static/env/me syntax receivers or a
TypeOp type string become evaluated; standard arguments run before the
receiver; helper terminals require full ARG0; me arity moves before argument
effects; property reads need a fake AST; or any touched source/check file
reaches 800 lines.

### ROUTE0-M0 closeout

M0 now owns one generic associated-input ordinary MethodCall driver over the
existing route plan. TypeOp and Standard request receiver E0 exactly once;
Static, Env, Me/This request no source-receiver descent; ordinary arguments
use ARG0 only after the existing route preflight. The TypeOp type string stays
syntax-only. Record-helper scalarization binds record-local values directly
and uses indexed E0 only for non-record slots, without pretending to be a full
ARG0 consumer. The already-materialized property-read entry remains a thin
value-level compatibility facade and creates no fake MethodCall source.

Env, Me, TypeOp, Static, Standard, failure-stop/reuse, malformed TypeOp, and
materialized-property fixtures are green. The actual Env proof still observes
its nested argument exactly once. Record-helper 5/5, MethodCall port 4/4,
reserved 11/11, property 3/3, recursive child 7/7, callable-result 43/43,
structural guards, PHI boundary guard, release build, quick gate, pointer guard,
formatting, and line caps are green. Result/type/effect publication, located
input, caller-ledger authorization, accepted grammar, backend, and runtime
authority remain unchanged. `SITE0-R0-EXPR0-M0-V0` is next.

## SITE0-R0-EXPR0-M0-V0 task lock

Three worker audits and local source review select one behavior-neutral
BoxShape series. External design consultation is not required. V0 owns only
the handoff from route-specific syntax/preflight plus completed child descent
to the existing value-level emitters and their completed `ValueId`.

The durable boundary is a private, stack-scoped terminal port associated with
the existing `MethodCallDescentPortV1` input. It exposes distinct terminal
methods for exactly these current ordinary paths:

```text
TypeOp
qualified static global
current-owner lowered global
Env extern
Standard method
```

It does not introduce a stored prepared-terminal enum or a second route table.
`MemberCallRoutePlan` and `ReceiverNormalizationPlan` remain the only ordinary
route decisions. `EnvMethodSpec`, `CallTarget`, `TypeOpKind`, `MirType`, and the
existing emitters retain target/effect/representation/publication truth. A
static/global terminal receives semantic owner, source method, checked arity,
and already-lowered values; it must not issue a callable key or recover identity
from a physical MIR symbol. The raw port is the only V0 implementation.

Property read remains an already-materialized value-level compatibility
consumer. It may reuse the same raw Standard terminal helper but must not
fabricate a MethodCall input. Static scalar facts, weak-load, record-helper and
setter scalarization, FastMem, MIR-debug, and REPL remain explicit early/custom
terminal owners. V0 inventories them and proves they do not silently enter the
generic ordinary terminal boundary.

The series is fixed as:

```text
V0-S0
  README boundary + private terminal port/raw adapter
  disconnected exact terminal fixtures
  selected production terminal consumers = 0

V0-I0
  thread TypeOp/static/me-lowered/env/standard ordinary completions
  through the raw adapter after existing preflight and child descent
  accepted grammar/MIR/effect/dst/type/diagnostic delta = 0

V0-P0
  normalized terminal parity and early/custom-terminal inventory
  failure-before-publication, no retry, and Builder reuse

V0-G0
  one structural guard, docs/current closeout, line caps
  -> SITE0-R0-EXPR0-L0
```

### V0-S0 closeout

V0-S0 is closed with one private `MethodCallValueTerminalPortV1` and one raw
implementation. The disconnected fixtures cover TypeOp Check/Cast,
qualified/current-owner globals, Env returning/no-result, and Standard method
emission. Operand comparison normalizes the existing LocalSSA Copy/Const
rematerialization instead of treating raw `ValueId` equality as semantic
identity. One dedicated structural guard fixes the port/raw-helper owners,
forbids syntax/route/located/ledger/result authority, and proves production
terminal consumers remain zero. V0-I0 is next.

### V0-I0 closeout

V0-I0 is closed with exactly five ordinary source completion sites. TypeOp,
qualified static, current-owner lowered global, Env, and Standard all retain
their existing route-specific preflight and child-effect order, then finish
through the same per-call associated terminal adapter. Lowered `me` keeps its
receiver-prefixed physical arguments and source arity. Materialized property
shares the one Standard preflight owner and calls the raw value helper without
constructing a MethodCall source input. Scalar facts, weak load, record/helper
setter, FastMem, MIR-debug, and REPL remain explicit custom terminals. Route
event fixtures prove children-before-terminal, lowered-Me receiver prefix,
terminal failure exact-once/no-retry, and Builder reuse. V0-P0 is next.

### V0-P0 closeout

V0-P0 is closed as an evidence-only slice. The disconnected terminal fixtures
now normalize destination allocation, target/effects, argument order,
returning/no-result Env behavior, and existing type/origin publication for
TypeOp, static/current-owner global, Env, and Standard. Materialized property
fixes the returned Call destination plus normalized receiver without creating a
source carrier. Static-scalar, weak-load/upgrade, helper-setter, FastMem,
MIR-debug, and REPL fixtures plus structural counts prove those owners remain
outside the generic terminal port. Receiver, argument, and terminal failures
enter no later ordinary terminal publication, do not retry, and leave the
Builder reusable. Production behavior and authority deltas remain zero. V0-G0
is next.

### Exact terminal laws

```text
TypeOp:
  receiver already lowered
  type spelling already sealed as syntax-only
  preserve Check/Cast, parsed MirType, dst allocation, and diagnostic

qualified static global:
  scalar/record-helper preflight already complete
  arguments already lowered in source order
  preserve owner.method/arity projection, CallTarget::Global, dst, annotation

current-owner lowered global:
  helper/module/argument/arity processing already complete
  preserve receiver prefix, global target, dst, and legacy annotation timing

Env:
  EnvMethodSpec already sealed and arguments already lowered
  preserve exact iface/method/effects/returns law, including the current
  no-result allocation and Void-result behavior

Standard:
  receiver already lowered; weak/helper/setter preflight already complete
  arguments already lowered
  preserve CallTarget::Method, rewrite/resolution behavior, dst, effects,
  type/origin publication, and diagnostics
```

Destination allocation must not move before child effects. Existing concrete
type/origin/result annotation remains owned by the emitters and existing
annotation helpers; V0 neither infers nor publishes a new fact. A terminal
error is returned once with no fallback or retry. Existing allocator burn
behavior is normalized and fixed rather than opportunistically cleaned up.

### Required V0 evidence

Use a new `calls/method_call_terminal_tests.rs`; do not grow the 343-line
child-demand fixture. The minimum normalized matrix is:

```text
TypeOp is/as:
  op/value/type/returned-dst/allocation/type-origin parity

qualified static global:
  semantic owner/method/arity, argument order, target/dst/effects parity

current-owner lowered global:
  receiver prefix, target/dst, legacy annotation parity

Env returning and no-result:
  extern identity, exact effects, dst/None, Void result, allocation parity

Standard and materialized property:
  receiver/arguments/target/dst/effects/type-origin parity
  property uses no source carrier

controls:
  weak-load, static scalar, record-helper/setter, reserved routes stay custom
  receiver/argument/preflight failure enters no ordinary terminal
  terminal failure preserves exact diagnostic, publishes no later fact,
  and the Builder remains reusable
```

The V0 guard fixes one terminal-port owner, one raw implementation, exact raw
consumers after I0, custom-terminal non-consumers, property fake-AST zero,
located/ledger/activation/result-authority imports zero, route/name probing and
retry zero, required fixtures, README boundary, and every touched source/check
file below 800 lines. ROUTE0 and earlier guards remain independently green.

### V0 stop conditions

Stop before implementation broadening if any of the following is required:

1. A terminal enum/table becomes a second route/target/effect/result owner.
2. A callable key is issued in Builder or reconstructed from a MIR symbol.
3. Destination allocation or terminal mutation must precede child success.
4. Me arity moves before argument effects or legacy annotation timing changes.
5. Record-helper/property requires full ARG0 or a fake MethodCall source.
6. A selected activation row can exit through an early/custom terminal and
   cannot be excluded or represented without widening V0 authority.
7. Located source, caller ledger, activation disposition, or exact-result
   publication must enter V0.
8. Plan/view/ledger/claim state must be stored in `MirBuilder`, cloned, shared,
   or kept as a mutable `current_claim` across nested calls.
9. FastMem/debug/REPL preflight or terminal ownership changes.
10. Fallback, retry, AST rewalk, runtime tag inference, or a touched
    source/check file reaching 800 lines.

## Required fixtures

```text
forward/reverse generic proof apps both execute 6
repeated and nested exact sites consume once each
qualified imported alias and current-owner calls
instance caller -> static target call

ParserBox.static_const_parse_add/2
  -> ParserStringUtilsBox.skip_ws/2
  -> StringHelpers.skip_ws/2

required argument is exact Integer
result is exact before GenericLoop verification
actual parser fixture has no MissingTransientType

unknown/non-i64 required argument -> no Call/result publication
missing/duplicate/wrong/foreign site -> typed reject
failed Call -> result type delta 0
rejected compile -> later valid compile succeeds
```

## Implementation may claim after CUT0

```text
one pre-body exact activation producer
one exact-site, exact-once selected call consumer
instance caller identity with a static target result contract
successful sealed Call followed by exact Integer publication
declaration-order and repeated-site stability
candidate-session discard on compilation failure
```

## Implementation must not claim

```text
general callable or instance-method result inference
non-i64 result inference
general legacy Builder source-site conversion
callee-first or two-pass lowering
whole-call argument-effect rollback
GenericLoop inference
fallback or retry
```

## Stop conditions

1. A second AST walker, callable catalog, target catalog, or result solver.
2. Site reconstruction from names, symbols, spans, AST equality, or order.
3. Mutable imports as seal authority.
4. Self-referential/Clone/Arc catalog storage.
5. Widening result inference to instance bodies merely to admit a caller.
6. Current-module/final-metadata/runtime-tag result lookup.
7. Callee-first publication, retry, re-lowering, or fallback.
8. A second persistent `ValueId -> type/owner` map.
9. Unrelated route behavior changes in the candidate transaction.
10. A source/check file reaching 800 lines.

## Decision lock

P0 remains closed and production behavior remains unchanged. Candidate A plus
located legacy input is locally selected under
`LocalMechanicalSelectorAuthorityV1`; external consultation is no longer
required. `I0-PATH0`, `I0-A0`, and `I0-SITE0-L0` are closed;
`I0-SITE0-R0` is the next code-facing row. No emitter patch may bypass the
SITE0 refactor series.
