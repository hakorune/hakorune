---
Status: the initializer producer-family split is accepted and closed. The
bounded Dynamic V2 producer and installed-package final semantic program are
already live. The current design stop is the missing selected-callable
lowering relation between that package-loaned program, the exact completed
local materialization, and the located Loop admission. The parser-only product
WIP remains parked; GenericLoop stays an exact-MirType verifier.
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`
Predecessor: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R0` closed
Mode: BoxShape / behavior-neutral in-place expression traversal
PipelineSSOT: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1

## Compiler-flow position

The global compiler order is owned by
`mirbuilder-final-pipeline-ssot.md`:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower
  -> Seal -> Collect -> Atomic Publish
```

This card is limited to a parser-side observation product. Its current
`NoSafeSlice` boundary is before a complete source/owner handoff into the
existing GenericLoop admission; it does not open a new Facts, Recipe, Lower,
or publication authority.

## Goal

Make the existing expression precedence traversal return one parser-private
rich result while keeping every current string-returning API as a projection.
The traversal may retain one exact unsuffixed integer lexical witness; every
operator or non-integer shape deterministically reduces it to CompatOnly.

This is an in-place refactor of the existing grammar owner, not a parallel
expression parser.

## Product

```text
ParserExpressionParseProductV1
  branch = ExactInteger | CompatOnly | ParseError
  compatibility fragment
  exact next position
  ExactInteger -> ParserNumberLexicalPartsV1
  ParseError   -> parser-private issue
```

It owns lexical/parser shape only. It does not issue a semantic type,
`ParserNodeProductV1`, source-carrier builder, Return, SourceBody, method,
resolver, Home, Recipe, MIR, or runtime meaning.

## One traversal

```text
ParserExprBox.parse_number_product2
  -> ParserNumberScanBox.scan_parts once
  -> ParserNumberScanBox.project_compat(outcome)
  -> ExactInteger / CompatOnly / ParseError

ParserExprBox.parse_factor_product_in_context2
  numeric FIRST -> parse_number_product2
  other FIRST   -> existing factor owner once -> CompatOnly

ParserExprPrecedenceBox product traversal
  exact leaf and no operator -> preserve ExactInteger
  unary/infix/ternary/group/postfix/other -> CompatOnly
  child ParseError -> propagate ParseError

legacy parse_*2 API
  -> product traversal
  -> compatibility fragment only
```

Do not pre-scan and then call the old numeric parser. Do not add an ambient
`last_typed_expr`, JSON decoding, source substring rescan, or duplicated
precedence loops. The product-returning functions are the actual traversal;
legacy string methods are thin projections.

## Exact disposition

```text
ExactInteger:
  Ready Integer
  leading_digit_count > 0
  suffix absent
  no unary/infix/ternary/group/postfix wrapper

CompatOnly:
  Float
  any valid non-integer expression
  exact integer participating in any larger expression

ParseError:
  InvalidStart
  Ready Missing at a numeric parse position
  current-profile suffixed integer rejection
  existing malformed/freeze result
```

`scan_int` compatibility remains byte-for-byte unchanged. If a product carries
a ParseError, its compatibility fragment may preserve the legacy parser/freeze
surface, but that fragment is never typed evidence.

## Structure and line limits

- put the product model in a small expression-owned file;
- refactor `parser_expr_precedence_box.hako` in place;
- add only the one factor-product callback needed by the `ParserBox` facade;
- keep `ParserBox` below 760 lines and every other touched Hako source below
  800 lines;
- avoid a new generic Plan/Recipe or public selection/filter API.

## Acceptance matrix

```text
ExactInteger:
  0
  42
  offset x42

CompatOnly:
  1.5
  .5
  -1
  1 + 2
  1 * 2
  1 < 2
  1 && 2
  1 ? 2 : 3
  (1)
  variable/call

ParseError:
  invalid start
  missing numeric token
  1usize under current profile

regression:
  legacy expression JSON and gpos unchanged
  scan_parts called once on the numeric product route
  string APIs contain no independent precedence traversal
  Return/SourceBody/parser-node connection = 0
```

## Verification (historical R1 product WIP)

The originally planned `hako_parser_rich_body_h2_s2_s1_r1_guard.sh` was never
registered and must not be treated as a live gate. The implemented handoff
surface is guarded by `h2_compile_front_owner_handoff_i0_guard.sh`; the
following existing predecessor/parity guards remain the only runnable checks
for this parked WIP:

```bash
bash tools/checks/h2_compile_front_owner_handoff_i0_guard.sh
bash tools/checks/hako_parser_source_carrier_p0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_s0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_r0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
ParserNodeProduct or SourceCarrierBuilder issuance
Return statement product
SourceBody/list/root seal
method/H3 connection
grammar expansion
Home, resolver, Recipe, MIR, runtime
```

## Closeout

The reopen audit was attempted with the existing parser-only expression product
WIP and a fixture that contains no `loop` statement. The focused guard still
reaches the existing GenericLoop representation failure while compiling the
imported parser surface:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(113) }
```

This is a predecessor/compiler capability blocker, not evidence that the
expression product is accepted. GenericLoop repair, a fixture workaround that
changes the accepted shape, and compatibility fallback are explicitly out of
scope. The product WIP, guard, and fixture remain parked as recoverable WIP;
the row must not advance to `H2-S2-S1-I0` until the blocker is resolved by its
own owner and the reopen audit is rerun with predecessor/parity gates green.

## Dependency-owner audit (2026-08-11)

The blocker is the existing transient-result publication family, not a new
GenericLoop semantic rule and not an R1 parser defect. The canonical owner
chain is already documented by:

```text
exact source call/result contract
  + successful CompletedUnifiedValueCallEmissionV1
  -> one non-Clone lowering-time result-publication receipt
  -> type_ctx[final destination]
  -> existing GenericLoop verifier
```

The GenericLoop carrier consumer remains verifier-only. The next design work
must follow the existing Dynamic owner chain, not create a new parser-specific
result-type task:

1. `generic-raw-structured-generic-loop-carrier-representation-d0-task-2026-08-07.md`
   confirms the consumer boundary and exact missing transient-type contract.
2. `generic-loop-source-backed-dynamic-carrier-d0-task-2026-08-09.md`
   owns the untyped formal/local/carrier relation and the Dynamic ingress,
   operation-result, and PHI authorization split. It explicitly blocks this
   Hako R1 row until its source-backed coverage is ready.
3. `generic-loop-dynamic-full-body-closure-d0-task-2026-08-10.md`
   owns the complete source inventory and the next Dynamic body coverage row.
   The existing static-call publication I0 is already closed and must not be
   reopened for this different parser carrier failure without a separate exact
   source-site census.

The `ValueId(113)` number is diagnostic only and is not a source identity.
The exact first parser callable/loop site must be proven by a read-only census
owned by the Dynamic ladder before any R1 reopen; `ParserScanLoopBox` and
`ParserNumberScanBox` are candidates, not assumptions. It may not use method
names, inferred Box types, GenericLoop backfill, retry, or a second
publication owner.

## Bounded compile-front census (2026-08-11)

The read-only probe was rerun with the guard's default environment before
classifying the source boundary:

```text
empty_script.hako                         -> RC: 0
parser_rich_body_h2_s2_r0_v1.hako         -> RC: 0
parser_rich_body_h2_s2_s0_v1.hako         -> GenericLoop MissingTransientType { init: ValueId(210) }
parked H2-S2-S1-R1 guard run               -> GenericLoop MissingTransientType { init: ValueId(113) }
```

This establishes that the current default compile front is not a universal
empty-input failure: the R0 surface remains green, while the S0/R1 parser
surface reaches the existing GenericLoop representation boundary. A separate
probe with `HAKO_JOINIR_DEBUG=1` made even an empty input reach
`UnknownTransientType { init: ValueId(31) }`; that debug-mode perturbation is
not a source-site baseline and must not be used to identify an owner. All
ValueId numbers are diagnostic allocation state only.

The exact first parser callable/loop is still unproven. The evidence does not
justify promoting `ParserStringUtilsBox`, `ParserNumberScanBox`, or
`ParserScanLoopBox` to the owner. Do not promote a candidate method name,
narrow the fixture, or add parser-specific logging/repair in this row. The
next safe diagnostic owner is a bounded compile-front source/owner inventory
that can attach a function/source site to the first GenericLoop admission;
until that exists, the existing Dynamic carrier/full-body ladder remains the
only semantic owner and this R1 product stays parked.

### Read-only call-trace census (2026-08-11)

The existing `NYASH_STATIC_CALL_TRACE=1` toggle was used without changing the
fixture or compiler route. It confirms that the failing compile front reaches
the imported parser surface, including:

```text
ParserCommonUtilsBox.is_digit/1
ParserNumberScanBox.scan_parts/2
ParserNumberScanBox.scan_int/2
```

and then stops at the unchanged:

```text
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(210) }
```

This trace is only a call-routing observation. It does not carry the exact
function owner, Loop source site, initializer source row, successful physical
producer receipt, or the `ValueId -> MirType` publication event for the failing
admission. The trace therefore cannot select `ParserNumberScanBox`,
`ParserScanLoopBox`, or any dependency as the semantic owner. The existing
`NoSafeSlice` decision and the exact-one producer-publication census remain in
force; no fixture narrowing, by-name repair, or second publication owner is
authorized.

### Dynamic-owner caller census (read-only, 2026-08-11)

The existing source-backed Dynamic products were checked separately from the
static-call result publication family:

```text
SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input
  -> CallableSemanticLoweringState::from_exact_source
  -> package-backed callable scope

CallableSemanticLoweringState::prepare_source_backed_dynamic_loop_ingress
  -> declaration + focused Dynamic tests only
  -> no non-test raw Loop production caller
```

This proves a missing production bridge, not which source site owns the
reported `ValueId(210)`. The Dynamic issuer owns source-backed Dynamic
lineage and ingress authorization; it must not invent a `MirType` from a
runtime tag or selector. The physical producer/publication owner remains the
only authority allowed to publish an exact transient type after a successful
emission receipt. The next audit must classify the exact admission as either
an existing exact-result publication row or a source-backed Dynamic lineage
row before opening an implementation task. These two families must not be
merged by name, ValueId, route, or fallback.

## Compile-front owner census D0 (2026-08-11)

Task: `H2-S2-S1-R1-COMPILE-FRONT-OWNER-CENSUS-D0`

Decision: design-only and read-only. This row shortens the diagnostic distance
to the already-existing transient-result publication owner; it does not open
the Hako expression product, GenericLoop representation, Dynamic carrier, or
static-call result framework.

The census may consume only existing compile-front observations and must emit
one stable, default-off diagnostic receipt for the first failing admission:

```text
compile front
  -> function owner
  -> method source site
  -> loop source site
  -> initializer producer
  -> TypeContext at GenericLoop entry
  -> first failing admission index
```

The receipt is diagnostic evidence, not a semantic authority. It must not be
used to select a method by name, assign a source identity from `ValueId`, infer
a transient type, or create a parser/Dynamic `Verified*` product. The existing
GenericLoop consumer remains verifier-only, and the existing Dynamic/static
publication owners remain the only semantic owners.

### Allowed boundary

```text
existing FunctionState / route context
  -> diagnostic-only owner/site observation
  -> stable one-line trace or test receipt
```

The diagnostic path must be default-off, use an existing trace toggle or a
documented dev-only toggle, and update the debug contract before adding a new
tag. It must not alter route selection, strictness, planner admission, ValueId
allocation, type publication, fallback, retry, or error disposition.

### Acceptance

```text
default empty baseline remains green
default R0 baseline remains green
default S0/R1 reproduce the same GenericLoop boundary
the first failing admission includes the complete evidence row above
debug-only perturbations are recorded separately and never treated as baseline
no candidate parser method is promoted without exact source-site evidence
```

If the existing compile front cannot provide the complete row without changing
semantic behavior, this task returns `NoSafeSlice` and records the missing
owner boundary. It must not be replaced with a parser-specific log, fixture
narrowing, GenericLoop backfill, or by-name repair.

### D0 audit receipt: `NoSafeSlice` (2026-08-11)

The existing API was audited without code changes. It cannot produce the
complete evidence row. The observable boundary is limited to:

```text
MirBuilder.current_function.signature.name / LoopRouteContext.func_name
variable_map[loop_var] -> ValueId
type_ctx.get_type(ValueId) -> Option<MirType>
alloc_generic_loop_v0_skeleton
  -> prepare_generic_loop_carrier_representation_v1
  -> typed Missing/Unknown failure
```

The missing handoff is specific:

```text
RawInvocationChildPortV1.active_source
  -> PreparedLocatedRawLoopChildEntryV1
      (SourceNodeSite + callable schedule)
  -> lower_with_existing_route_v1
      (condition/body only)
  -> GenericLoop admission
      (source/site/schedule absent)
```

The GenericLoop route therefore has no method source site, opaque
`FunctionOwnerId`, initializer producer, or first-admission path/index. The
AST loop span is discarded at the raw-loop-child boundary, and the
representation error can be reached from multiple composers. `value_origins`
is diagnostic span/caller metadata, not a source-producer authority. Existing
resolved callable/Dynamic schedule products are downstream owners and cannot
be used to reconstruct this compile-front baseline.

Decision: keep the design stop and return `NoSafeSlice`. Do not add a
GenericLoop repair, parser-specific `Verified*`, candidate-method assumption,
fixture narrowing, debug-mode baseline, or retry/fallback. A future diagnostic
I0 may add one default-off, semantic-neutral owner/site handoff at the existing
route boundary; until that handoff is designed and accepted, the parser R1 WIP
remains parked and the Dynamic/static publication owners remain unchanged.

## Next design boundary: source-handoff to GenericLoop admission

Task: `H2-S2-S1-R1-COMPILE-FRONT-OWNER-HANDOFF-D0`

Decision: keep this as a design stop. The existing source transport is not
missing; it is consumed too early:

```text
RawInvocationChildPortV1.active_source
  -> PreparedLocatedRawLoopChildEntryV1
  -> exact source/owner/loop schedule validation
  -> source consumed
  -> lower_with_existing_route_v1
      -> condition/body only
  -> GenericLoop admission
```

The future diagnostic-only boundary may borrow, exactly once and without
semantic mutation:

```text
source context / callable schedule
  + current function/route context
  + loop_var / init ValueId / transient type
  -> first GenericLoop admission diagnostic
```

It must preserve `FunctionOwnerIdV1`, `SourceNodeSite`, and
`VerifiedCallableSemanticLoopBindingScheduleV1` until the admission observation
is complete, then release them. It must not make those products GenericLoop
semantic inputs, create a new source observer, or infer a type. The legacy raw
child path has no source context and remains outside this diagnostic cohort.

Acceptance for the future I0 is limited to:

```text
default-off diagnostic handoff
no route/planner/ValueId/type/publication behavior change
complete owner/site/initializer/TypeContext/admission evidence row
default empty/R0/S0/R1 baseline comparison remains reproducible
source/schedule borrow is one-shot and cannot escape the route call
```

### Handoff D0 refinement (worker audit, 2026-08-11)

The smallest future diagnostic seam is a short-lived, non-semantic seed made
at `PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1`:

```text
GenericLoopAdmissionObservationSeedV1   // diagnostic-only; not Verified*/Prepared*
  existing FunctionOwnerId / loop site
  parent / condition / body SourceNodeSite
  loop_var / carrier role
  init ValueId + transient TypeContext view
  initializer producer observation (when the source-aware hook can provide it)
  one route-local first-admission index
```

The seed is borrowed into one default-off observer at the existing
`prepare_generic_loop_carrier_representation_v1` admission and then dropped.
The callable schedule is still consumed exactly once by the existing
`consume_pre_effect` path. Legacy/unlocated raw-child paths produce no row.
No AST rescan, name/ordinal/ValueId repair, type inference, semantic state
mutation, fallback, or retry is allowed.

The audit found two missing canonical inputs, so this remains `NoSafeSlice`:

```text
exact method declaration source site
initializer producer relation
```

The current `RawInvocationRootLineage` does not provide the former, and the
current local descent exposes only a ValueId/site without a producer relation.
The future I0 may open only after both source-aware hooks and their
missing/foreign/duplicate fail-fast cases are fixed in a design Decision.

The source census is concrete:

```text
parser/source_resolver_handoff.rs::ResolverBoxMethodSourceRowV1
  already owns the parser-issued box/member source site

parser/callable_source_anchor.rs and the final callable syntax loan
  already own the opaque declaration anchor

raw_invocation_source_transport.rs::RawInvocationRootLineageV1
  Cataloged(key) carries only a callable key; it does not transfer the
  existing exact source site/anchor into the compile-front route

normal_callable_catalog_owner_link.rs
  verifies catalog/owner pairing but does not transfer a method-site receipt

stmts/local_statement_descent.rs::CompletedLocalStatementV1
  retains result/binding ValueIds only

normal_callable_semantic_lowering_state.rs::record_completed_local
  matches an existing local site to bindings, but does not retain the
  initializer expression producer relation
```

Therefore `H2-CALLABLE-METHOD-SOURCE-SITE-HANDOFF-D0` is a transport/co-seal
decision, not permission to create a second parser or resolver site issuer.

Until this D0 is accepted, `ValueId`, method name, import order, AST shape,
and `TypeContext` alone remain non-authorities. The GenericLoop carrier stays
verifier-only, and the existing Dynamic/transient-publication owners remain
unchanged.

The census closeout must be one bounded evidence row, not a new semantic
product:

```text
compile front
  -> function owner
  -> method source site
  -> loop source site
  -> initializer producer
  -> TypeContext at GenericLoop entry
  -> first failing admission index
```

It must compare the default empty/R0 baseline with the S0/R1 parser front and
record the exact delta. A debug-only failure is not an acceptable baseline.
Until that row exists, `ValueId`, method name, import order, and shape
similarity remain diagnostics only and cannot select a Dynamic or parser
owner.

This is a prerequisite consultation boundary, not an authorization to create
a generic Hako result-type framework. Once the exact owner closes, rerun the
same R1 guard and fixture from the parked WIP; do not change the accepted
expression-product shape merely to avoid the compiler boundary.

## Next design tasks (parked under this card)

The missing inputs are split by authority. They are not two new semantic
products and must not be implemented as GenericLoop fields.

### D0 decision closeout (2026-08-11)

The worker audit accepts both prerequisite designs with one canonical-pairing
condition. The parser source owner, not the Raw port, issues one co-sealed
method observation containing:

```text
same parser provenance
+ CallableDeclarationIdentityV1
+ exact ResolverBoxMethodSourceSiteV1
```

The Raw port transports and compares this existing observation; it never joins
an anchor and a site from independent fields. The observation is a
comparison-only diagnostic carrier, not a `Verified*`/`Prepared*` semantic
product, GenericLoop input, lookup key, or ownership authority.

The local initializer observation uses the existing source-aware local descent
owner. It reports one short-lived `LocalInitializer(index)` relation after the
initializer has produced its evaluated `ValueId`, then joins that relation once
with `CompletedLocalStatementV1` to obtain the local `ValueId`. It remains
default-off diagnostic state; `CompletedLocalStatementV1` does not gain producer
semantics.

Both designs reject missing, foreign, duplicate, wrong-index, and missing
completion relations before GenericLoop admission. Unlocated compatibility
routes produce no observation. The existing GenericLoop error/category,
route, ValueId, and type behavior must remain unchanged.

Decision: **accepted design; open the bounded I0 below**.

### `H2-CALLABLE-METHOD-SOURCE-SITE-HANDOFF-D0`

Design the canonical source-carrier handoff for the exact callable declaration
site. The parser/source session already issues the site together with the
opaque declaration identity and final source slots; this row carries that
existing relation into the compile-front route. Catalog keys, method names,
arity, inventory ordinals, AST pointer equality, and `RawInvocationRootLineage`
must remain navigation or lookup data only. The downstream catalog-owner link
may borrow the site once, but it may not repair a missing or foreign identity.

Acceptance:

```text
one parser/source owner
same-session provenance and declaration identity
exact declaration site survives to the diagnostic seam
missing/foreign/duplicate site rejects
no AST/text rescan and no GenericLoop semantic input
parser-side provenance/anchor/site co-seal is the sole issuer
```

### `H2-LOCAL-INITIALIZER-PRODUCER-OBSERVATION-D0`

Design the source-aware diagnostic observation for one
`ExprChildRoleV1::LocalInitializer(index)`. The issuer remains the existing
local/recursive descent owner and may lend a short-lived relation containing
initializer source site, evaluated `ValueId`, and the statement/local ordinal.
`CompletedLocalStatementV1` remains a completion receipt and does not gain
producer semantics; the relation is consumed by the diagnostic seed only.

Acceptance:

```text
one source-aware local descent hook
exact initializer site + evaluated ValueId relation
missing/foreign/duplicate producer rejects
one-shot borrow; no semantic state retention
no ValueId/name/order repair, type inference, fallback, or retry
wrong initializer index and missing completion reject before admission
```

The accepted I0 is now closed:

```text
H2-S2-S1-R1-COMPILE-FRONT-OWNER-HANDOFF-I0
  source/site/producer handoff
  -> one default-off first-admission observation
  -> unchanged GenericLoop failure
```

### I0 closeout receipt (2026-08-11)

The parser-issued method observation now travels through the existing
syntax-loan -> resolved semantic batch -> installed package -> raw callable
port -> located loop-entry path. It is still one comparison-only diagnostic
carrier; no GenericLoop semantic field or second source issuer was added.

The local descent records the exact initializer source relation only after
the initializer has produced its evaluated `ValueId`. The default-off
admission observer retains that relation together with the co-sealed method
provenance/anchor/site and rejects a missing or foreign pairing before the
existing route is entered.

Verified so far:

```text
parser direct-method observation test: green
cargo check --lib: green
callable semantic batch tests: green
normal callable semantic package tests: green
h2_compile_front_owner_handoff_i0_guard.sh: green
selected member-gate observation exclusion test: green
```

This row still does not authorize GenericLoop semantic changes, production
route activation, fallback/retry, H2-S3, FuncScanner, Stage-B JSON, or Dynamic
narrowing. After the I0 closeout, the parked post-Dynamic Loop BoxShape series
remains the ordered cleanup path:

```text
LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
```

### `H2-S2-S1-R1-REOPEN-AUDIT` (next design stop)

The predecessor S0/R1 fixture gate still fails at the unchanged compiler
boundary:

```text
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(...) }
```

This is not a GenericLoop admission defect. The next audit is read-only and
must census the one canonical producer-publication chain:

```text
init ValueId
  -> source initializer
  -> actual physical producer
  -> successful emission receipt
  -> transient `ValueId -> MirType` publication
  -> GenericLoop TypeContext read
```

Acceptance:

```text
one producer issuer for the selected ValueId
known successful producer has one publication receipt
missing/failed/foreign/duplicate producer or publication rejects
default route, MIR type, ValueId, and GenericLoop error remain unchanged
producer issuer remains separate from Dynamic carrier lifecycle
```

If the producer issuer cannot be identified from existing source/emission
receipts, keep this row at `NoSafeSlice`. Do not insert a default type in
GenericLoop, infer from loop_var/name/AST/ValueId/route/runtime tag, copy from
previous/PHI, publish after failure, or add debug/fixture/fallback/retry
behavior. The parser expression-product WIP remains parked until this audit
has a source-backed owner and the same predecessor/parity gate can be rerun.

### Read-only producer/publication census closeout (2026-08-11)

The reopened census is classified as **C: unowned/missing evidence**. The
correct disposition remains `NoSafeSlice`; this is not a GenericLoop defect
and it does not reopen the already-closed static result-publication row.

The two existing candidate families were checked separately:

```text
Static result publication (existing, distinct family)
  RawInvocationChildPortV1::try_emit_source_bound_static_call_result_v1
    -> exact selected source-bound handoff
    -> CompletedUnifiedValueCallEmissionV1
    -> PreparedStaticCallResultPublicationV1::commit
    -> type_ctx[final destination] = exact MirType

Dynamic source owner (existing, currently disconnected)
  SourceBackedDynamicCallableIssuerV1
    -> CallableDynamicOriginLoweringStateV1
    -> prepare_source_backed_dynamic_loop_ingress
```

The first family has a real successful-emission/publication receipt, but the
H2 fixture's static-call trace only reaches names such as
`ParserCommonUtilsBox.is_digit/1`, `ParserNumberScanBox.scan_parts/2`, and
`scan_int/2`. It does not bind the failing GenericLoop initializer to an exact
static source-site, call-emission receipt, or publication destination. The
reported `ValueId(210)` is allocation evidence only and is not a source
identity; reopening static result publication would therefore create a second
authority without evidence.

The second family is a plausible source lineage for the fixture: the imported
parser methods use untyped formals and locals such as `local j = i`, followed by
numeric loops. However, a production caller census found no non-test caller of
`prepare_source_backed_dynamic_loop_ingress`. The Dynamic issuer also does not
publish a `ValueId -> MirType` fact. It cannot be connected to this failure by
method name, loop variable, route, or runtime tag.

The only confirmed GenericLoop path is still verifier-only:

```text
variable_map[loop_var] -> init ValueId
type_ctx.get_type(init)
  None    -> MissingTransientType
  Unknown -> UnknownTransientType
  exact   -> carrier representation check
```

No producer, publication, default type, previous/PHI copy, or Dynamic
authorization is issued at that boundary. This preserves the existing route,
MIR type behavior, and failure category.

Reopen criteria are now explicit and one-to-one:

```text
(method identity, loop source, initializer source, init ValueId)
  + exact successful physical producer receipt
    or exact Dynamic ingress receipt
  + unique post-success TypeContext publication
    or authorized Dynamic representation
```

Until that relation is observed, keep `H2-S2-S1-R1-REOPEN-AUDIT` at
`NoSafeSlice`. Do not repair GenericLoop, reopen static I1, connect the
Dynamic issuer in production, infer from ValueId/name/route/AST/runtime tag,
narrow the fixture, add debug-only evidence, or introduce fallback/retry/a
second publisher. A future bounded design may be named
`H2-S2-S1-R1-EXACT-FIRST-ADMISSION-PRODUCER-CENSUS-D0`, but it is a read-only
owner census, not an implementation task, and should not be opened until a
new exact observation source exists.

### `H2-S2-S1-R1-INITIALIZER-PRODUCER-COSEAL-D0` (next design task)

The independent design review separates the only two admissible producer
families. This is a read-only design/census row; it does not add a GenericLoop
field, a new type publisher, or a production route.

```text
Diagnostic-only admission observation
  method identity/provenance
  + loop source
  + initializer ordinal/source/value
  + exactly one producer family

A. selected direct static-call initializer
   LocalInitializerObservation
     -> CompletedUnifiedValueCallEmissionV1.final_destination
     -> PreparedStaticCallResultPublicationV1 demand/site/representation
     -> successful commit
     -> post-commit exact TypeContext fact

B. source-backed Dynamic formal/local lineage
   LocalInitializerObservation
     -> exact Dynamic current-origin/ingress receipt
     -> authorized Dynamic representation
```

For family A, the possible diagnostic carrier may retain only the existing
source site, initializer `ValueId`, call demand site/caller/target, emission
destination, admission index, and the exact post-success `MirType`. It must
prove `initializer.value == emission.final_destination`, exact source-site
and provenance equality, successful publication, and one-shot completion. The
normal static publication owner remains the sole semantic publisher.

Family B must not be simulated with a static receipt. The existing Dynamic
issuer recognizes formal/local/carrier lineage, but its ingress preparation has
no non-test raw-loop caller and it does not publish a `ValueId -> MirType`
fact. Opening this family therefore requires a separate source-backed Dynamic
lineage Decision; it cannot be inferred from a method name, loop variable,
route, runtime tag, or `ValueId`.

Acceptance for this D0:

```text
direct selected static path and Dynamic lineage path are distinct
one producer receipt is paired with one initializer observation
foreign/missing/duplicate/wrong-index/wrong-site rows reject
static commit failure or missing post-commit type rejects
unselected/ordinary/nested calls do not become static rows
Dynamic formal/local without an exact lineage receipt remains NoSafeSlice
GenericLoop route/error/type behavior is unchanged
```

Required negative cases include foreign method/loop provenance, initializer
ordinal mismatch, `ValueId != final_destination`, unselected publication,
double publication, failed emission, nested/non-direct static call, and a
Dynamic/local-copy initializer presented as a static receipt. No AST/MIR
rescan, ValueId/name repair, default type, fallback, retry, or second
publisher is allowed. The current H2 failure has not yet supplied either
family's exact row, so implementation remains closed until this D0 is
accepted.

### D0 design closeout (2026-08-11)

The design is accepted with an explicit implementation stop. The two
producer families remain separate and neither may be inferred from the H2
`ValueId` or method/loop name:

```text
Static:
  CompletedUnifiedValueCallEmissionV1
    -> PreparedStaticCallResultPublicationV1::commit
    -> post-success TypeContext fact

Dynamic:
  SourceBackedDynamicCallableIssuerV1
    -> CallableDynamicOriginLoweringStateV1
    -> PreparedSourceBackedDynamicLoopIngressV1
```

The existing static family is the sole lowering-time type publisher, but the
H2 failing initializer has no exact selected static call-site/emission row.
The Dynamic family has exact formal/local/Loop source lineage, but it owns no
`MirType` fact and has no non-test production ingress for this failure. Thus
the H2 initializer still has no one-to-one producer co-seal and remains
`NoSafeSlice`.

The complete Dynamic producer is already landed. The installed normal-callable
package owns the entire bounded chain through
`VerifiedDynamicExitTransactionCoSealV1`, and its selected lowering loan can
lend that exact program. The previous claim that
`LOOP-V2-DYNAMIC-FULL-PRODUCER-D0` was the next missing owner is therefore
superseded.

The actual missing relation is later and narrower:

```text
installed-package selected semantic loan
  + exact completed local materialization
  + exact located Loop source/schedule
  -> selected initializer admission
```

The current Builder adapter does not consume the package-loaned Dynamic
program. It reconstructs a request-local Dynamic source state from the
resolved input, while the located raw-Loop terminal later discards the method
and admission observations before entering the legacy route. The completed
local materialization itself is not missing: the request-local state already
retains exact initializer/local `ValueId`s and their `BindingRef` relation.
The next row must relate those existing facts to the package owner; it must not
turn diagnostic `LocalInitializerObservationV1` into semantic authority.

## `H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-COSEAL-D0` (closed 2026-08-11)

### Decision closeout (2026-08-11)

The worker-backed authority review accepts this boundary. The complete
Dynamic producer remains the package owner, `PreparedDynamicLocalEntryV1`
remains the existing local materialization owner, and the located Loop entry
is the only place allowed to relate them. The D0 itself adds no receipt or
code; the next executable row is the behavior-neutral
`H2-SELECTED-DYNAMIC-LOWERING-AUTHORITY-R0` below.

Decision question: can the selected-callable lowering boundary issue one
move-only admission from already-owned semantic and physical facts without
reissuing source semantics or modifying GenericLoop?

```text
SelectedCallableLoweringInputRefV1
  semantic = Ordinary | Dynamic(&VerifiedDynamicExitTransactionCoSealV1)
  exact resolved callable source/owner
  exact method source observation
                 |
                 | HRTB-bounded Dynamic initializer-admission view
                 v
request-local completed local materialization
  initializer ValueId
  local destination ValueId
  local declaration / BindingRef
                 +
located Loop admission
  callable identity/provenance
  owner / frame / Scope / Region
  Loop / condition / body sites
  exact binding schedule
                 v
PreparedInitializerProducerAdmissionV1<'program>
  = Static(PreparedStaticInitializerAdmissionV1)
  | Dynamic(PreparedDynamicInitializerAdmissionV1<'program>)
```

The request-local Dynamic arm reuses the existing
`PreparedDynamicLocalEntryV1`; it does not introduce another local
materialization product. The closed admission sum is scoped to the sole
consumer and must not be published as a disconnected caller-zero receipt.

The final Dynamic co-seal may lend only the narrow prelude relation needed by
this boundary: selected callable identity, owner/frame/scope provenance,
parameter #1 source binding, prelude initializer source, local induction
declaration/binding, root Loop/carrier/entry, and authorized Dynamic
representation. It does not lend raw Recipe, JoinSig, batch slot, target,
Fault, cleanup, or a freely pairable ingress product.

The request-local state may retain physical `ValueId` projection and
exactly-once consumption, but it may no longer classify the selected Dynamic
callable by rerunning `SourceBackedDynamicCallableIssuerV1`. Package semantics
is the classifier; request-local state only binds that meaning to values
created by this Lower.

### Family rule

```text
Static only:
  exact selected direct-call source
  + successful emission destination
  + sole post-success TypeContext publication
  -> existing exact-MirType GenericLoop route

Dynamic only:
  package-loaned exact Dynamic program
  + exact local materialization
  + exact located Loop admission
  -> existing Dynamic V2 physical path

Static && Dynamic:
  reject as ambiguous producer

neither:
  NoSafeSlice
```

Dynamic never fabricates or publishes `MirType::Unknown`, a nominal Dynamic
type, or a TypeContext backfill. It does not enter legacy GenericLoop.

### `H2-SELECTED-DYNAMIC-LOWERING-AUTHORITY-R0` closeout (2026-08-11)

R0 is closed as a behavior-neutral BoxShape refactor. The package-side
Dynamic admission now retains the existing
`VerifiedSourceBackedDynamicCallableV1` exactly once beside the final Dynamic
exit-transaction program. `SelectedCallableSemanticRefV1::Dynamic` loans both
products through the installed package, and the selected-callable adapter
passes the retained source product into request-local origin state.

The ordinary path keeps its existing local source issuance. On the selected
Dynamic package path, `SourceBackedDynamicCallableIssuerV1` is no longer
executed by the Builder adapter; the package seed is shared immutably through
an `Rc` and consumed by the existing origin projection. No new admission
receipt, `ValueId` publication, `MirType`, GenericLoop branch, CFG/PHI/session,
fallback, or retry was added.

The package test now proves that the selected Dynamic loan carries a source
seed with the exact resolved owner, and
`h2_compile_front_owner_handoff_i0_guard.sh` enforces:

```text
selected adapter consumes input.semantic() = 1
selected adapter source reissue = 0
package Dynamic admission retains the source-backed seed = 1
GenericLoop semantic change = 0
source files below 800 lines = 1
```

Verification receipt (2026-08-11):

```text
cargo check -q --lib                                      -> pass
cargo test --lib normal_callable_semantic_package          -> 11 passed
h2_compile_front_owner_handoff_i0_guard.sh                -> pass
current_state_pointer_guard.sh                            -> pass
git diff --check                                          -> pass
```

The focused legacy-origin test
`normal_callable_scope_consumes_real_entry_and_local_terminal_receipts`
still fails with
`[freeze:contract][script-lexical/local-binding]`. The same failure is
reproduced on the parent commit, so it is a pre-existing predecessor failure,
not an R0 regression; it remains owned by the existing lexical/local-binding
boundary and is not repaired in this authority-only row.

R0 does not claim that the located Loop consumes this authority. The method
and admission observations are still outside the physical route; that is the
next design boundary.

### Required negative matrix

```text
identity:
  foreign parser provenance / declaration identity / selected callable
  wrong owner / frame / Scope / Region
  wrong Loop, condition, body, or method source

materialization:
  wrong initializer ordinal/site
  wrong local declaration or BindingRef
  wrong initializer or local destination ValueId
  missing initializer -> local relation
  stale/foreign formal origin
  duplicate local completion or double receipt consumption

family:
  Ordinary selected semantic offered as Dynamic
  foreign package/program or wrong program owner
  Dynamic local copy offered as Static
  Static result offered as Dynamic
  both families or neither family

authority guards:
  diagnostic LocalInitializerObservation as semantic input
  source reissue from the selected Builder adapter
  name/arity/ValueId/route/runtime-tag repair
  raw batch-slot getter, public from_parts/into_parts, Clone authority
  Dynamic receipt entering legacy GenericLoop
```

## `H2-SELECTED-DYNAMIC-LOOP-CONSUMER-D0` (superseded broad design stop)

R0 proved only the authority transport. This D0 must decide whether the
transport can be consumed by one real bounded Dynamic Loop physical route.
It is not a new Dynamic producer task: the complete V2 producer and the
package-held `VerifiedDynamicExitTransactionCoSealV1` are already landed.

### Exact question

```text
installed package Dynamic semantic loan
  + existing PreparedDynamicLocalEntryV1
  + located method/Loop source observation
  + exact binding schedule
        |
        v
one bounded selected Dynamic Loop consumer
```

The consumer must be named before implementation. It must either consume the
package-owned semantic program directly or prove that a source-backed result
and ABI contract already exists. A disconnected admission receipt is not a
consumer and must not be created to make the graph look complete.

### Bootstrap cycle to close explicitly

```text
H2 result carrier
  -> needs the imported Dynamic Loop to compile

Dynamic physical input
  -> needs the H2 result carrier / source-backed result contract
```

The only acceptable resolution is a bounded in-place replacement of the
selected Dynamic Loop responsibility inside the existing outer callable
terminal. Do not add a second function pipeline, a temporary GenericLoop type,
or a compatibility fallback. If the cycle has no sole consumer at this
boundary, the result is `NoSafeSlice` and the H2 row remains blocked.

### D0 acceptance

`accepted` requires all of the following:

```text
one named production consumer
package Dynamic program is the only semantic classifier
PreparedDynamicLocalEntryV1 is the only existing local materialization fact
method/Loop/frame/scope provenance has one exact relation
Static xor Dynamic selection remains fail-fast
Dynamic does not enter exact-MirType GenericLoop
the consumer can reach the existing bounded V2 physical input/demand
no source/Recipe/JoinSig reissue
no fallback/retry/bootstrap shim
```

If any item is absent, record the exact missing owner and keep this D0 at
`NoSafeSlice`; do not insert another prerequisite row between this D0 and the
cutover.

### Required negative cases

```text
foreign package/program/owner/frame/scope/region/Loop
wrong initializer site/ordinal or local BindingRef/ValueId
missing or duplicate local materialization
ordinary semantic offered as Dynamic
Dynamic offered as Static exact-type receipt
both producer families or neither family
Dynamic routed to GenericLoop
package semantic program consumed twice
source reissue by name/arity/ValueId/route/runtime tag
unlocated compatibility route or fallback/retry
```

### Non-claims

```text
no new admission receipt before a sole consumer exists
no GenericLoop change or Dynamic MirType
no CFG/PHI/function session/DraftSeal/Collector/publication
no result/ABI inference from runtime tags or ValueId
no parser fixture narrowing
no provider/runtime execution
```

### Consumer census closeout (2026-08-11): `NoSafeSlice`

The current code audit confirms that this is a real consumer gap, not a
missing Dynamic producer:

```text
package-held program:
  VerifiedDynamicExitTransactionCoSealV1
  -> SelectedCallableSemanticRefV1::Dynamic
  -> selected Builder adapter (source seed only)

physical demand:
  issue_dynamic_full_loop_operation_physical_demand_v2
  -> test-only exit-transaction callers
  -> no production physical consumer

local materialization:
  PreparedDynamicLocalEntryV1
  -> retained inside CallableDynamicOriginLoweringStateV1
  -> no located-Loop consumer surface

located Loop:
  PreparedLocatedRawLoopChildEntryV1
  -> method/admission observations are discarded
  -> legacy lower_loop_or_freeze_v1 route
```

The worker audit also found that `SelectedCallableSemanticRefV1::Dynamic`
has no production consumer of its `program` field yet: the selected adapter
currently consumes only the retained source seed. The nearest V1 canary,
`CanonicalSsaFunctionSessionV2::open_source_backed_dynamic_loop_header`, is
test-only and owns a disconnected hard-coded Dynamic operation/PHI route; it
cannot be promoted as the V2 consumer. The V2 physical demand itself has only
the semantic-program tests as callers and deliberately emits no Builder,
CFG, `ValueId`, ABI, or Completion fact.

There is also no Dynamic result/ABI/Tail owner at this boundary:
`DynamicCallableFunctionExitTargetV1` records only Value/Unit, while the
existing exact return ABI publisher is static-family-only. This makes the
bootstrap cycle a genuine authority gap, not a missing adapter. Keep the D0
at `NoSafeSlice` until one bounded consumer and its result/ABI contract are
named and co-sealed.

Therefore the missing owner is the sole selected-Dynamic physical consumer
and its bootstrap contract. The existing physical-input/demand products are
valid upstream evidence, but they do not by themselves lower a Loop or bind
the request-local initializer/local values. The D0 remains `NoSafeSlice`.

Do not make the test-only physical-demand issuer production merely to close
the graph, promote `GenericLoopAdmissionObservationV1` to semantic authority,
or add a disconnected `Prepared*` receipt. The next design question remains
exactly one: whether the existing outer callable terminal can consume the
package program, local materialization, and located Loop in one bounded V2
route without a second pipeline, fallback, or retry.

### D0 decision closeout: `NoSafeSlice` (2026-08-11)

```text
Decision:
  REVISE / NoSafeSlice; do not open an implementation row.

Source authority:
  installed package's VerifiedDynamicExitTransactionCoSealV1

Existing physical fact:
  PreparedDynamicLocalEntryV1 inside request-local origin state

Missing canonical issuer:
  selected Dynamic Loop physical consumer plus Dynamic result/ABI/Tail/
  Completion relation

Fail-fast boundary:
  no sole consumer, no exact co-seal, foreign/duplicate materialization,
  or missing result/ABI contract

Non-claims:
  no GenericLoop change, MirType backfill, V1-canary promotion, V2 demand
  production activation, fallback, retry, or disconnected admission receipt
```

This broad stop is retained as historical audit evidence. The current pointer
now moves to the narrower selected-initializer admission bridge below; the
already-landed full producer is not reopened and no hidden prerequisite row is
invented to disguise the remaining relation.

### Consumer audit refinement (2026-08-11): narrow the stop, do not reopen the producer

The follow-up worker audit confirms that the earlier consumer stop was correctly
conservative, but its wording was still broader than the actual missing seam.
The complete bounded Dynamic producer and package-held
`VerifiedDynamicExitTransactionCoSealV1` are already landed. The missing work
is not another producer or another physical-demand inventory; it is the
selected-callable lowering bridge that relates the package semantic loan to
the request-local values and the located Loop.

Existing owners were checked and are not silently reusable:

```text
VerifiedFunctionCompletionV1
  = sole logical completion owner
  = explicit return-site set / declared result relation

ResolvedFunctionCompletionConsumptionV1
  = outer physical terminal
  = currently single explicit return + physical BasicBlock/ValueId only

VerifiedCallableTerminalCompatibilityV1
ReadyCallableTailCompletionV1
VerifiedCallableTailV1
  = static or caller-zero profile-specific contracts

Dynamic exit transaction / V2 physical demand
  = logical chronology and complete semantic input only
  = no Builder ValueId, ABI, Tail, Completion, DraftSeal, or physical caller
```

The H2 `skip_while` cohort has two explicit returns and no source-backed
Dynamic result/ABI contract, so promoting the existing static terminal or the
test-only V1 Dynamic canary would create a second authority. The correct
disposition is to close the broad consumer wording as an audit result and move
the pointer to the narrower design stop below.

## `H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-BRIDGE-D0` (current design stop)

This is a successor boundary, not a new Dynamic producer task. It owns one
question only:

> Can the selected-callable lowering boundary issue exactly one move-only
> relation between the package-loaned Dynamic semantic program, the existing
> completed-local materialization, and the exact located Loop admission?

The intended shape is:

```text
SelectedCallableLoweringInputRefV1
  semantic = Ordinary | Dynamic(&VerifiedDynamicExitTransactionCoSealV1)
  exact callable/source owner and method observation
                 |
                 | narrow HRTB borrow, no raw Recipe/JoinSig exposure
                 v
DynamicInitializerAdmissionViewV1
  selected callable identity
  owner / frame / Scope / Region provenance
  parameter #1 source BindingRef
  prelude initializer source/ordinal
  local induction declaration/binding
  root Loop/carrier/entry relation
  authorized Dynamic representation
                 +
PreparedDynamicLocalEntryV1
  initializer ValueId
  local destination ValueId
  local BindingRef
                 +
PreparedLocatedRawLoopChildEntryV1
  exact method/Loop/condition/body source
  exact binding schedule
                 v
PreparedInitializerProducerAdmissionV1<'program>
  = Static(PreparedStaticInitializerAdmissionV1)
  | Dynamic(PreparedDynamicInitializerAdmissionV1<'program>)
```

The closed sum is a lowering-boundary product, not a new parser/resolver
authority. The package remains the only Dynamic semantic classifier; request-
local state only binds that meaning to Values created by this Lower. The
diagnostic `LocalInitializerObservationV1` remains non-semantic. Missing,
foreign, duplicate, or ambiguous relations fail fast as `NoSafeSlice`.

### Bridge acceptance

This D0 can move to implementation only when all of the following are named
and co-sealed:

```text
selected package program is consumed (not merely retained)
exact method/callable identity and parser provenance
exact frame / Scope / Region / Loop relation
initializer source + ordinal
local declaration / BindingRef
initializer ValueId -> local destination ValueId
exact binding schedule
Static xor Dynamic family selection
Dynamic never enters exact-MirType GenericLoop
no source/Recipe/JoinSig reissue, fallback, or retry
```

This bridge does not itself solve Dynamic result ABI, multi-return
Completion, Tail, DraftSeal, CFG, PHI, or publication. Those are later owners;
claiming them here would hide the actual bootstrap cycle.

### Required downstream ladder (fixed; no hidden prerequisite rows)

```text
1. H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-BRIDGE-D0
   close the exact selected semantic/local/Loop relation

2. H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-BRIDGE-I0
   only after D0 acceptance; one private move-only bridge, no route switch

3. H2-S2-S1-R1-REOPEN-AUDIT
   unchanged empty/R0/S0/R1 fixtures and predecessor/parity guards

4. H2-S2-S1-I0 -> H2-S3-I0 -> H2-I0 -> H3-I0 -> H5
   close parser expression/body/header/final-source/parity substrate

5. HAKO-CALLABLE-HEADER-RESULT-CARRIER-I0
   source-backed declared result contract (selected :i64 cohort first)

6. DYNAMIC-CALLABLE-RESULT-CONTRACT-I0
   selected Dynamic result relation from source contract, not runtime tags

7. PHYSICAL-INPUT-AUTHORITY-I0
   source-backed Dynamic physical input/demand consumer; no Builder fact yet

8. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   multi-return Completion adapter, Tail/ABI projection, DraftSeal Return
   exactly once; existing VerifiedFunctionCompletionV1 remains the logical
   completion owner

9. H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0
   one named production caller, fresh session, old edge deletion, no fallback

10. LOOP-UNIFICATION-AFTER-DYNAMIC-D0 series
    remove Recipe-derived transfer inference/evidence rescans, keep Callable
    profile ownership out of the common physicalizer, then retire the fixed-
    role topology only after segment callers are zero

11. MIRBUILDER-FIRST-PRODUCTION-CUTOVER
    prove one real production method through Resolve -> Observe -> Facts ->
    Recipe -> Verify -> Lower -> Seal -> Collect -> Atomic Publish

12. SELFHOST-MIRBUILDER-HANDOFF-PERF-GATE
    after the new MIRBuilder is production-green and before selfhosting,
    compile the `.hako` mimalloc implementation with the new builder and
    record C-vs-AOT performance/assembly evidence; no selfhost claim without
    this canary
```

No row in this ladder may be skipped by reopening the already-landed full
producer or by routing Dynamic through a fabricated `MirType::Unknown` or a
static exact-type receipt.

### Historical ordered execution ladder (superseded at the consumer boundary)

The selected-admission D0 and its behavior-neutral R0 are closed. This ladder
records the pre-refinement consumer stop; the successor ladder above is the
current pointer. Later rows remain fixed gates, not permission to cross an
unresolved design stop.

```text
1. H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-COSEAL-D0
   fix the HRTB view, exact co-seal identity, XOR family rule, and stop line

2. H2-SELECTED-DYNAMIC-LOWERING-AUTHORITY-R0 (closed 2026-08-11)
   package-owned Dynamic source seed is retained and consumed by the
   selected-callable adapter; package-path source reclassification is zero.
   Route, fixture, GenericLoop, CFG, and physical behavior remain unchanged.

3. H2-SELECTED-DYNAMIC-LOOP-CONSUMER-D0 (current design stop)
   close the bootstrap cycle and identify the sole consumer explicitly:
     H2 needs the imported Dynamic Loop to compile
     full Dynamic physical input needs a source-backed result/ABI contract
     the selfhost result carrier is itself downstream of H2/H3
   prove that final program + existing PreparedDynamicLocalEntryV1 + located
   Loop can enter one bounded V2 consumer; otherwise remain NoSafeSlice

4. H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0
   only after row 3 is accepted:
     issue and consume the private Static | Dynamic admission in this cell
     one named selected production caller
     fresh unpublished function session
     existing complete Dynamic physical input/demand
     same-slice deletion of the selected source-reissue and legacy Loop edge
     fallback/retry = 0

5. H2-S2-S1-R1-REOPEN-AUDIT
   rerun the unchanged empty/R0/S0/R1 fixtures and predecessor/parity guards;
   no fixture narrowing or debug-only acceptance

6. H2-S2-S1-I0 -> H2-S3-I0 -> H2-I0 -> H3-I0 -> H5
   close the parser expression/body/header/final-source/parity substrate in
   the existing order

7. HAKO-CALLABLE-HEADER-RESULT-CARRIER-I0
   connect the H3-sealed typed result row to the existing resolved batch

8. DYNAMIC-CALLABLE-RESULT-CONTRACT-I0
   issue the exact selected semantic result contract

9. PHYSICAL-INPUT-AUTHORITY-I0 -> DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   close Prelude/Tail/ABI/Completion and the durable full physical session

10. LOOP-UNIFICATION-AFTER-DYNAMIC-D0 series
    remove Recipe-derived transfer inference, repeated evidence scans,
    Callable profile ownership from the common physicalizer, and finally the
    caller-zero fixed-role topology
```

There is exactly one permitted non-cutover implementation row between the
current Decision and the consumer Decision. If R0 discovers another missing
semantic authority, it returns to design instead of inserting a new adapter
or disconnected receipt row. Rows 4 and later must name their production
caller and same-slice old-authority deletion before implementation.

### Structure and file budget for the R0/cutover series

The implementation must extend the existing owner subtrees instead of adding
logic to the already-large adapters:

```text
dynamic_full_body_recipe/coseal/semantic_program/exit_transaction/
  initializer_admission.rs       HRTB final-program view; target <= 250 lines

builder/selected_initializer_admission/
  model.rs                       move-only closed sum; target <= 220 lines
  issuer.rs                      exact co-seal; target <= 350 lines
  route.rs                       sole bounded consumer; target <= 300 lines
  tests.rs                       positive/negative matrix
```

`normal_callable_semantic_loan_port.rs`, `raw_loop_child_entry.rs`, and
`normal_callable_semantic_lowering_state.rs` remain thin integration surfaces;
do not append the co-seal implementation to them. Split before 760 lines and
keep every source file below 800 lines. Prefer focused Rust tests plus the
existing H2/normal-package lane guards; do not create a one-off shell guard
unless an existing reusable guard cannot enforce caller-zero/no-reissue.
`recursive_child_lowering.rs` is already 785 lines and is an explicit
no-addition surface; route new ownership through the dedicated child module.

### Current nonclaims

```text
no code implementation while this D0 is current
no GenericLoop change or new carrier variant
no Dynamic Recipe/source/envelope/JoinSig reimplementation
no CFG / PHI / session / DraftSeal / Collector / publication
no result/ABI inference or TypeContext backfill
no provider/runtime execution
no production cutover
no parser fixture/source narrowing
no fallback or retry
```
