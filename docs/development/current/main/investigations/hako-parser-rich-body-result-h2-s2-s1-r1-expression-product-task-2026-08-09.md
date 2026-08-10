---
Status: compile-front handoff I0 and the read-only GenericLoop
producer-publication audit are closed without an exact producer owner. The
active frontier is the design-only initializer producer co-seal split; the
parser-only product WIP still reaches the existing GenericLoop representation
blocker before fixture execution, so implementation remains NoSafeSlice.
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
