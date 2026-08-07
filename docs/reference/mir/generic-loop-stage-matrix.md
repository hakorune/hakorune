# Generic Loop V0/V1 Stage Matrix

Status: inspection-only reference
Date: 2026-08-07

This page documents the current test-only evidence boundary for Generic Loop
V0/V1 post-effect debt. It is not a production route policy, Recipe contract,
PHI owner, scheduler, or backend lowering specification.

## Authorities

The design authority is
`docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`.
The DirectAccum S1 observation boundary is specified by
`docs/development/current/main/design/loop-family-observation-policy-ssot.md`.
The executable task and acceptance evidence are
`docs/development/current/main/investigations/joinir-generic-structural-grammar-census-d2-a3-s1-execution-task-2026-08-04.md`;
the closed ledger is
`docs/development/current/main/investigations/joinir-generic-post-effect-debt-classification-d0-s1-execution-task-2026-08-04.md`.
The closed overlap parity evidence is recorded in
`docs/development/current/main/investigations/joinir-generic-overlap-semantic-parity-d2-b2-execution-task-2026-08-04.md`.
The closed bounded continuation (implementation row S1) is
`docs/development/current/main/investigations/joinir-generic-nested-carrier-winner-d2-b4-d0-design-2026-08-05.md`.
The former “next accepted design stop” pointer to D2-B4-S2 is historical and
superseded. Current Generic source-bridge work is tracked by
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-shared-source-bridge-d3-s2-d4-design-2026-08-05.md`;
this inspection-only matrix does not become its selector or Recipe authority.
The machine-readable test observer is
`src/mir/builder/control_flow/joinir/route_entry/registry/generic_stage_matrix_tests.rs`.

Implementation receipt (`CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, 2026-08-07):
the shared canonical function finish boundary is implemented for the three
V2 profiles only. Generic G0 remains caller-zero observation/Recipe evidence;
no physical selection, fallback removal, or production claim follows.

Production selection remains the ordered registry in
`src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs` and
`predicates.rs`. The `loop_route_policy::generic_g0` subtree now owns a
caller-zero AST-free observation only; it is not a Generic winner oracle.

## Callable source/facts issuer S0 receipt

The callable S0 promotion compiles the neutral SyntaxFacts, source-shape, and
SourceMap issuers in production scope while keeping fixture constructors and
mutation helpers test-only. The resolver site seam is
`CallableSemanticSourceLedgerView::only_loop_site()`; the source navigation
seam is branded `FunctionSourceViewV1::stmt_at(membership)`. The production
issuer entry uses those seams, and the SourceFacts -> SourceMap parity fixture
preserves resolver Loop/frame/Scope-Region identity. Zero/multiple Loop sites
and cross-brand memberships are typed rejects. This is still
source/facts transport only: Recipe/JoinSig, Prepared physicalization,
selection, retry/fallback, Generic substitution, and production caller
activation remain closed.

## Current common physicalizer boundary

`LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` is closed as a caller-zero,
test-only topology/After canary. It consumes the shared move-only demand and
session-local entry receipt to construct recursive logical
child/header/body/step/After topology, but it does not emit operation MIR.
The passive `LOOP-RECIPE-OPERATION-EFFECT-S0` product and both profile
adapters are caller-zero evidence. The neutral contract defines an item-keyed
exact source/effect ledger for `ReadBinding`, `WriteBinding`, constants,
comparisons, and arithmetic, but it does not emit operation MIR. Generic G0
remains a separate profile and is not relabeled as the callable physicalizer.
Cross-profile callable/G0 evidence parity is now closed as a diagnostic-only
receipt. Reviewed Decision B also closes: complete demand/preflight and private
leaf emission are separate proofs. No selector, retry/fallback, production
caller, or legacy deletion is implied. The next boundary is Builder-free
`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0`.

The P0 test canary is now landed: `VerifiedLoopPhysicalBoundaryV1` and the
private `ReadyLoopEntryV1` are consumed by one recursive topology probe, which
allocates child/root header-body-step-After blocks and returns one After
receipt. Owner, exact input coverage, parent topology, and preheader placement
are checked before allocation. Operation MIR is still closed until the
item-keyed product is consumed; this remains disconnected evidence and does
not activate Generic G0 or any production route.

The callable After-closure canary is now also landed as a separate
caller-zero boundary. It uses the real Prelude receipt and complete seven-row
Callable dispatch (`Pure=4`, `Read=2`, `Write=1`) before emitting the fixed CFG
edges and sealing CFG/identity. A provisional unsealed PHI may be typed only
by its verified Recipe class; concrete or missing type facts reject as
`ResultTypeMismatch`. This does not activate Generic G0, Tail, Completion,
DraftSeal, selector, retry/fallback, or legacy retirement.

## Operation/effect product S0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-S0` is now landed as a passive, AST-free
caller-zero product. The non-`Clone` product moves the verified Core once and
checks one profile-issued source-evidence row for each of the 19 nested
fixture operation items. Binding operations must match the Core's exact
source-read/source-write relation and class; pure operations reject fabricated
binding evidence. Duplicate, missing, foreign-owner, wrong-placement, and
invalid-source cases are typed rejects.

The product has no Builder/MIR, operation physicalizer, Return, DraftSeal,
selector, retry/fallback, production caller, or legacy-deletion authority.

## Callable operation/effect adapter S0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-CALLABLE-ADAPTER-S0` is now closed as
caller-zero evidence. The adapter consumes the callable co-seal once, proves
each transient operation view against the sealed Recipe item, derives exact
block/loop placement, and connects binding operations to the Core's exact
read/write effect relation before issuing the neutral product. Prelude, Tail,
input, semantic context, and continuation remain in one thin profile wrapper;
no operation/effect/Recipe truth is copied.

No Builder/MIR, selector, retry/fallback, production caller, or legacy
deletion is opened. The Generic G0 anchor row is now also closed: its producer
issues the exact 15 item keys before source facts are dropped, with item 3 as
the existing child-entry `DerivedCarrierEntry` for carrier 2. Item 4, C0/C1
carriers, and Generic tail reads remain outside this product. Cross-profile
parity is now closed as a diagnostic-only receipt: Callable has seven item rows
and Generic G0 has fifteen, but the receipt compares neither counts nor source
order. Both products use the same item-keyed neutral schema, owner/source
provenance, Recipe placement, Core effect matching, and common typed rejection
family. Operation physicalization remains closed.

## Full operation demand P0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0` is implemented as a Builder-free
full-program preflight. The common move-only demand carries the resolver
semantic context, complete item-keyed operation/effect product, and one
logical After continuation. `prepare_all` derives Recipe order and prepares
all seven Callable rows and all fifteen Generic G0 rows with zero MIR/Builder
effect. Context/continuation issuers are neutral transport wrappers; they do
not reissue or clone source/JoinSig evidence.

This receipt opens no physical block mapping, operation instruction emission,
function session, Return/Completion/DraftSeal, selector, retry/fallback, or
legacy deletion. The behavior-neutral physicalizer module split is now closed;
the physical block receipt is now closed, followed by the current ConstI64
leaf-emitter proof.

## Physical block receipt P0 (2026-08-07)

The topology canary issues one private logical Loop/role to physical
`BasicBlockId` receipt from the existing canonical CFG allocation. Callable and
Generic G0 operation demand remain untouched; the receipt adds no operation
shape, emitter, session, selector, fallback, or legacy authority.

## ConstI64 leaf-emitter S0 (2026-08-07)

The private prepared ConstI64 emitter is now closed as a disconnected canary.
It binds one prepared operation to the canonical physical block receipt,
emits exactly one typed `Const` instruction through the existing Builder
owner, rejects foreign/mismatched placement before emission, and proves
whole-session discard after a harness-only late failure followed by fresh
session repeat. It does not consume or partially extract a full Callable/G0
demand, and it opens no continuation, SSA/PHI, Return/DraftSeal, selector,
fallback, production, or legacy-deletion authority.

## ReadBinding leaf-emitter I0 (2026-08-07)

The bounded `LOOP-RECIPE-OPERATION-EMITTER-READ-I0` implementation is closed
as a private test-only leaf. The complete prepared program projects all
`Expr`/`SourceRead` rows with exact operation/effect/source/placement checks;
`DerivedCarrierEntry` remains a typed `CarrierSeedUnavailable` reject. The
leaf claims the source through the canonical BindingSSA/PHI seam and returns
distinct logical/physical placement receipts with explicit `PreheaderSeed`
and `CanonicalLive` entry requirements. No single-operation demand extraction
was added.

The production replacement row remains open: full operation integration,
carrier seeds, continuation/Tail, Return/DraftSeal, selector, retry/fallback
retirement, and legacy deletion are still closed.

## Callable operation-emitter preparation receipt (2026-08-07)

The caller-zero preparation slice adds a private move-only Prepared-product
handoff and a complete WriteBinding projection. Completion is moved exactly
once; no clone or second terminal owner is introduced. Typed leaf bridges now
cover pure `ConstI64`, `BinaryI64`, and `CompareI64` operations, with a focused
Const -> Binary -> Compare fixture and source-bound Write projection tests.

This remains preparation evidence, not full physicalization. The common
full Recipe-order prepare plus bounded Read/Const/Compare/Binary/Write dispatch
seam now issues one exact logical-to-physical target receipt per row and
separates semantic preflight from post-claim physical failure. The exact
Prelude receipt is now landed: the resolver-backed Prelude result and the
distinct Loop initializer are published separately into one session-local
`ReadyLoopEntryV1`; the two bindings are never conflated. The callable
`CALLABLE-LOOP-AFTER-CLOSURE-P0` slice now seals CFG/identity and issues one
`ReadyLoopAfterContinuationV1` after the complete operation dispatch.
Tail-to-ValueId, Completion claim, fresh callable session completion, and
DraftSeal remain later slices. Generic G0 parity, production selection,
retry/fallback, and legacy deletion remain closed.

## Generic G0 operation/effect anchor ledger S0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-GENERIC-G0-ANCHOR-S0` is closed as a caller-zero
passive adapter. The exact item keys are
`0,1,2,3,5,6,7,8,9,10,11,12,13,14,15`; the neutral verifier confirms
Recipe-derived block/loop placement, owner/source provenance, and exact Core
binding effects. Duplicate, missing, foreign-owner, and wrong-placement
rejection coverage remains green. This receipt opens no operation MIR, parity
selection, physicalizer, retry/fallback, production caller, or legacy
deletion.

## Cross-profile operation/effect parity receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-CROSS-PROFILE-PARITY-S0` is closed as
caller-zero diagnostic evidence. The common verifier, not the parity receipt,
owns duplicate, missing, foreign, wrong-placement, and pure-binding rejection.
The receipt does not relabel or select profile items and does not fuse
Tail/After. Reviewed Decision B closes with one complete move-only operation
physical demand that exposes only `prepare_all`; leaf emission borrows session
owners later and owns no continuation. The next implementation row preflights
all seven Callable and fifteen Generic G0 operations with Builder effect zero.

## S0A implementation receipt

`GENERIC-G0-STRUCTURE-S0A` is now landed as a disconnected, caller-zero
structural witness. `src/mir/compiler/generic_g0_projection/mod.rs` performs
the exact natural-source navigation; `src/mir/loop_structural_facts/generic_g0/mod.rs`
issues the move-only AST-free product. The row verifies nested body order,
resolver `BindingRefV1` relations, owner/source/frame identity, and complete
duplicate-free coverage. Focused positive/negative tests and the shared
MirBuilder replacement guard are green. No type/numeric policy, candidate,
selector, Recipe, Builder/MIR, retry/fallback, or production support claim is
made; `GENERIC-G0-SOURCE-TYPE-S0B` is the next row.

## S0B implementation receipt

`GENERIC-G0-SOURCE-TYPE-S0B` is now landed as a disconnected, caller-zero
source-type witness. The compiler projector derives one callable header view
from the natural function root and emits exact owner-branded parameter/result
sites plus the four S0A literal role/context rows. The sole AST-free issuer in
`src/mir/resolved_semantics/generic_g0/` validates parameter binding origin,
raw type spelling, annotation presence, literal cardinality, and owner
relations, then moves the result with S0A into
`loop_structural_facts::generic_g0::VerifiedGenericSourceBundleG0`. The
compiler projector issues this move-only product but does not own a second
aggregate wrapper.

Focused natural-source tests cover explicit `i64` headers, missing parameter
and result annotations, and a known non-`i64` parameter. The shared replacement
guard covers the recursive semantic directory, source/test line cap, and
caller-zero issuer boundary. S0B does not infer types, retag literals, choose
numeric representation, issue policy/Recipe keys, or enter Builder/MIR/
production; S0C and the owner refactor are now landed; Generic row
normalization is now landed caller-zero and the remaining competing-profile
observations are next.

## S0C implementation receipt

`GENERIC-G0-NUMERIC-REPRESENTATION-S0C` is now landed as a disconnected,
caller-zero numeric representation witness. The compiler adapter consumes the
move-only S0B bundle once, maps its four role rows to an AST-free scalar view,
and retains the original neutral source bundle while adding one
`VerifiedGenericNumericFactLeaseG0` plus the existing exact `i64` return ABI.
The sole numeric issuer is `src/mir/numeric_substrate/generic_g0/`; it owns
target/signed-width/range classification and imports no compiler, resolver,
Recipe, Builder, or MIR authority. Natural plain contextual literals are
accepted; typed suffixes are a known out-of-profile rejection, while neutral
opaque context/target and out-of-range boundaries remain typed
Unresolved/Rejected evidence. Focused natural and substrate tests plus the
shared caller-zero/recursive line guard are green. S0C does not prove
positivity, recurrence progression, candidate selection, Recipe, physical
lowering, or production support; the landed S1 policy observation consumes
this sealed product, and Generic row normalization is now landed caller-zero.

## S1 candidate-policy implementation receipt

`GENERIC-G0-CANDIDATE-S1` is now landed as a caller-zero, AST-free policy
observation. `src/mir/loop_route_policy/generic_g0.rs` consumes the sealed S0C
typed bundle and a sealed owner/profile/mode/coverage context exactly once.
Both `Less` conditions plus both positive `Add` steps produce one move-only
`VerifiedGenericFamilyObservationG0`. Unsupported comparison/update syntax or
non-progressing steps are `Unresolved`; contradictory direction or foreign
context is `Rejected`. Missing roles remain defensive typed rejects.

The source projector now carries neutral `Less`/`LessEqual`/`Greater`-family
and `Add`/`Subtract` syntax facts; S1 owns the admission matrix and does not
recheck S0A BindingRef relations. Seven policy tests plus the existing Generic
projection tests are green. S1 does not select a winner, issue Recipe keys,
touch Builder/MIR, retry/fallback, or open a production caller; the next row is
`LOOP-FAMILY-DIRECT-OBSERVATION-S1`.

## Generic G0 row-normalization S1 implementation receipt

`GENERIC-G0-ROW-NORMALIZATION-S1` is landed as a disconnected, caller-zero
source-to-observation bridge. The `cfg(test)` compiler adapter consumes the
existing S0A/S0B/S0C products once with an explicit numeric target and maps
typed source outcomes into a neutral C/D/U/R attempt. The route observer
rechecks owner/origin/source-kind/site/frame identity, mode, coverage, and the
candidate's structural identity before invoking the existing Generic policy
issuer. Twelve adapter tests and seven policy tests are green, together with
the shared caller-zero/line guard and `cargo check --lib`.

Known non-G0 syntax declines; missing or opaque source/type/numeric facts stay
unresolved; foreign or conflicting facts reject. Ambiguous `ForestShape` and
`BindingLookup` evidence remains conservative `Unresolved` until a separate
resolver-side split. No admission assembler, selector, Recipe/JoinSig,
Builder/MIR, physical lowering, retry/fallback, production caller, or legacy
retirement is claimed. The next ordered cell is FAMILY-ROW-CONTEXT-RETENTION-R0.

## Generic G0 policy-handoff I0/R0 implementation receipt

`GENERIC-SELECTION-POLICY-HANDOFF-I0-R0` is landed as a disconnected,
caller-zero source-to-policy witness. The test-only compiler projector
`generic_g0_projection::handoff` is the sole issuer of one move-only
`VerifiedGenericG0PolicyHandoffV1`. It co-seals a private resolver/source brand
borrowed from the selector's canonical window lease, the existing typed S0C
bundle, its exact role `BindingRef` relations, the numeric target, and the
exact post-loop return-expression relation. The handoff does not retain a
second window lease. The product retains no AST or `FunctionSyntaxViewV1`, and
policy consumes it by value without downgrading to a bare bundle or re-pairing
source facts by owner/site/name.

The old `VerifiedGenericCandidateEnvelopeV1` remains a cfg(test)-only source
lease witness; it is not wrapped, promoted, or used as a second policy
authority. Existing Generic observation, Ready assembly, and selector tests
remain unchanged and no production caller is opened. The shared loop-family
guard and focused G0 suite are green. This receipt claims no demand, Recipe,
JoinSig, Builder/MIR, physical, backend, retry/fallback, public language, or
legacy-retirement support. The next ordered boundary is the accepted
`GENERIC-G0-DEMAND-S3` design.

## Generic G0 demand S3 I0/R0 implementation receipt

`GENERIC-G0-DEMAND-S3-I0-R0` is landed as a disconnected, `cfg(test)`
caller-zero consuming witness. The Ready/Selected(Generic) path moves one
canonical `VerifiedLoopFamilyWindowLeaseV1` into
`VerifiedGenericRecipeDemandG0`; the Generic handoff contributes only its
private borrowed brand, typed S0C bundle, post-loop return read, and exact
role/provenance relations. Candidate evidence and policy profile/mode/
coverage are checked against the canonical selector window, while a sealed
zero-sized role lease proves the existing condition/update/tail relations
without copying source rows or issuing Recipe keys.

The positive natural nested G0 fixture and selected-other-family negative are
green. The demand has no AST/source-view lifetime, Recipe key, JoinSig, Core,
After, `LoopBindingKeyV1`, `ValueId`, PHI, Builder/MIR, physical, retry,
fallback, production caller, or legacy-retirement authority. The exact next
boundary is the worker-reviewed S4 Recipe/JoinSig/Core/After producer; public
activation remains zero.

## Generic G0 Recipe S4 design receipt

`GENERIC-G0-RECIPE-S4-D0` is closed on 2026-08-07 after independent worker
review. One Generic producer consumes `VerifiedGenericRecipeDemandG0` once and
owns the deterministic dense Recipe key map, `GenericG0` provenance, exact
source/effect relations, and the final `VerifiedGenericRecipeProductG0`.
Common Recipe verification, JoinSig elaboration, the single
`require_after_binding` call, and source-bound Core co-sealing remain their
respective common owners; S4 does not reimplement them.

The After envelope owns `L0.After/b1`, the moved post-loop read, owner/frame
relation, and `ExactTrivialReturnAbiV1`. P0 owns executable completion and
DraftSeal; S4 does not add a function tail or physical Return writer. The S4
implementation must prove the three carrier rows and the exact ten-row
source/effect matrix, with typed rejects for stale/foreign/duplicate/missing
relations and wrong After/ABI pairing. No production caller, physicalizer,
retry/fallback, or legacy deletion is opened by this design row.

## Generic G0 Recipe S4 implementation receipt

`GENERIC-G0-RECIPE-S4-I0-R0` landed on 2026-08-07 as a disconnected,
`cfg(test)` caller-zero producer. It consumes one
`VerifiedGenericRecipeDemandG0`, binds the resolved source forest once, emits
the deterministic G0 key map, and delegates Recipe verification, JoinSig,
source-bound Core, and After binding to their common owners. The product seals
the exact three carrier rows, ten source/effect relations, moved post-loop
read, `L0.After/b1`, owner/frame, and `ExactTrivialReturnAbiV1`.

The focused Generic suite is green (42 tests). The producer has no Builder/MIR,
physical, completion, retry/fallback, or production authority. The next
ordered boundary is `GENERIC-LEGACY-CORPUS-UNIVERSE-P0`; production selection
and legacy deletion remain closed.

## Generic legacy corpus universe P0 receipt

`GENERIC-LEGACY-CORPUS-UNIVERSE-P0` is landed on 2026-08-07 as a checked,
inventory-only universe. The 25-column union manifest
`docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv`
normalizes 179 active phase29bq rows, 198 planner-required selfhost rows, four
Generic fixture-inventory records, four canonical Generic smoke scripts, and four
compatibility script aliases. Source line provenance, mode/profile identity,
canonical fixture paths, and alias targets are checked by
`tools/checks/lib/generic_legacy_corpus_universe_guard.py` and the shared
MirBuilder replacement guard.

All 389 case records remain `unobserved`/`unknown` with
`nonproduction-future-evidence`; no runtime route, disposition, Recipe,
Builder/MIR, physical, retry/fallback, production caller, or deletion claim is
opened. The manifest contains zero edge records by design; the edge columns
are sealed for the later dependency row. The next ordered boundary is
`GENERIC-LEGACY-OBSERVATION-FRONT-G0`.

## Generic legacy observation-front G0 receipt

`GENERIC-LEGACY-OBSERVATION-FRONT-G0` is closed on 2026-08-07 with a named
pre-Loop failure rather than a manufactured green result. The fixed direct VM
invocation of canonical case
`generic_loop_continue_strict_shadow_vm` exits `1` in the prelude
`StringifyOperator.apply/1` `Body(1)/IfCondition` expression
`value.stringify != null`. The first owner is the `BinaryOp` arm of
`raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1` while
lowering that expression.
The immutable receipt is
`docs/development/current/main/design/fixtures/generic-legacy-observation-front-g0-v1.json`;
its shared guard verifies exact P0 identity/profile, direct fixture invocation,
stable diagnostic source, and observation-only claims.

The S0-I0 transactional completion repair and S1-I0 Dynamic FieldAccess
receiver receipt transport are also closed. The immutable S1 receipt is
`docs/development/current/main/design/fixtures/generic-raw-structured-field-receiver-receipt-s1-i0-v1.json`;
it exposed the next MethodCall receiver receipt boundary. S2-D0/I0 is also
closed: `RawLegacyMethodCallInputV1` carries an optional prepared Receiver
receipt and only the raw receiver descent consumes it. The immutable S2
receipt is
`docs/development/current/main/design/fixtures/generic-raw-structured-method-receiver-receipt-s2-i0-v1.json`.
Its probe reaches the next body-item source-path mismatch, where raw lowering
retains `IfThenBody` but the resolver's canonical item site is rootless. This
was the previous design stop; no Loop production or Generic route claim was
opened. S3-D0/I0 is now closed: the
dedicated raw item-site policy strips only the accepted nested rootless
body-kind roots (`Scope`, `TaskScope`, `FastMem`, `IfThen`, `IfElse`, `Loop`,
and `BlockExprPrelude`); `Program` remains rootful, and `Function` remains
direct `Body(index)`. The immutable S3 receipt is
`docs/development/current/main/design/fixtures/generic-raw-structured-body-item-source-canonicalization-s3-i0-v1.json`.
The fresh release probe now reaches `generic_loop_v1` carrier representation
and fails with `MissingTransientType { init: ValueId(3) }`. The current design
stop is
`GENERIC-RAW-STRUCTURED-GENERIC-LOOP-CARRIER-REPRESENTATION-D0`; no Generic
production or physical cutover claim is opened. That carrier audit is closed;
the next design task is
`docs/development/current/main/investigations/generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md`.

This is evidence about the shared raw structured-child owner only. No Generic
route, Recipe, physical, disposition, production, retry, or legacy deletion
claim is opened. S0-D0 and S0-I0 are now closed: exact-demand completion is
transactional and preserves the first child error. The fresh I0 receipt is
`docs/development/current/main/design/fixtures/generic-raw-structured-demands-repair-s0-i0-v1.json`;
it records the primary
`[freeze:contract][callable-semantic-lowering/missing-variable-site]` and the
absence of the old masking diagnostic. Worker audit corrected the next owner:
resolver variable admission is already complete, while the Dynamic
FieldAccess read must transport the existing `Receiver` source receipt. The
current implementation row is
`GENERIC-RAW-STRUCTURED-FIELD-RECEIVER-RECEIPT-S1-I0`.

## DirectAccum S1 observation receipt

`LOOP-FAMILY-DIRECT-OBSERVATION-S1` is now landed as a caller-zero,
AST-free family observation. The test-only compiler adapter
`src/mir/compiler/direct_accum_observation.rs` translates the existing
DirectAccum projector's typed source errors into neutral source-attempt
reasons. `src/mir/loop_route_policy/direct_accum_observation.rs` consumes that
attempt together with one sealed owner/source/frame/mode/coverage context and
issues only `Candidate`, `Declined`, `Unresolved`, or `Rejected`.

The exact matrix is fixed: complete canonical Less plus two positive Add
assignments is a Candidate in Release, Strict, and StrictPlannerRequired;
known non-Direct shapes decline; incomplete or unsealed source windows are
Unresolved; and foreign identity, frame/source-kind, upvar/non-binding target,
or BindingRef conflicts are Rejected. The seven focused tests and shared
recursive guard are green. No legacy schedule/cursor/winner, selector,
Recipe/JoinSig/BindingKey, Builder/MIR/PHI, retry/fallback, or production
caller is introduced. The next row is
`LOOP-FAMILY-NESTED-OBSERVATION-S1`; this page remains inspection-only.

## NestedPredicate S1 design boundary

The design stop for `LOOP-FAMILY-NESTED-OBSERVATION-S1` is closed in
`docs/development/current/main/design/loop-family-observation-policy-ssot.md`.
The caller-zero implementation is now landed. It adapts only the existing
`issue_nested_predicate_source_projection_v1` product into a neutral AST-free
`Candidate`/`Declined`/`Unresolved`/`Rejected` observation. Its Candidate means
exact bounded source projection, not Recipe or physical admission. Forest
lookup failures and resolved-forest invariant conflicts remain distinct:
missing/opaque source is `Unresolved`, while a known non-Nested shape declines
and a malformed/foreign forest rejects. Producer-only
initializer/recurrence checks remain later and are not duplicated in the
observer. Seven policy tests, eight projection tests, and the shared recursive
guard are green. No selector, Recipe/JoinSig, Builder/MIR, retry/fallback,
route ID, or production caller is authorized by this design row.

## LoopTrue S1 implementation receipt

`LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1` is landed as a caller-zero, AST-free
source observation. The sole source authority remains
`compiler/loop_true_break_continue_projection.rs`; its projection now exposes
the complete owner/origin/kind/site/frame identity. A test-only adapter maps
typed source outcomes into the neutral attempt DTO, and the pure policy
observer rechecks identity, mode, and coverage before issuing only Candidate,
Declined, Unresolved, or Rejected. Candidate still means only the exact bounded
`loop(true)` plus explicit Break/Continue source projection.

Nine policy tests, eight projection tests, and the shared recursive guard are
green. The legacy FrozenLoopRouteSchedule policy demand remains outside S1.
No selector, Recipe/JoinSig, Builder/MIR, physical route, retry/fallback,
production caller, or legacy deletion is open; the next boundary is common
five-family selection/admission design.

## Common admission D0 worker-reviewed design receipt

The canonical window has exactly five semantic rows:
`DirectAccum`, `NestedPredicate`, `LoopTrueBreakContinue`,
`LoopCondBreakContinue`, and `GenericG0`. It is not a four-row pilot. A
resolver-issued AST-free window identity brand is co-sealed with one typed
`Candidate|Declined|Unresolved|Rejected` row per tag; legacy `Blocked` remains
schedule vocabulary only. The assembler validates identity/mode/coverage but
does not select. LoopCond S1, Generic normalization, and
FAMILY-ROW-CONTEXT-RETENTION-R0 are now landed caller-zero rows. The
resolver-owned `LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` source-brand prerequisite
is also landed; the common assembler is the next cell, while selector
promotion and production remain closed.

## Window lease issuer S0 receipt

`LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` is landed as a caller-zero resolver
source-brand product. `VerifiedResolvedFunctionV1` issues one non-`Clone`/
non-`Copy` `VerifiedLoopFamilyWindowLeaseV1` from an exact
`VerifiedResolvedLoopSourceV1` lookup. The lease keeps only owner and the
resolver-branded origin/source-kind/site/frame token; it contains no AST,
forest, policy mode/coverage, route, Recipe, Builder, or MIR data. Three
focused issuer tests and the in-place replacement guard are green. The route-
policy common assembler is the next separate cell; selector and production
remain closed.

## LoopCond S1 implementation receipt

`LOOP-FAMILY-LOOPCOND-OBSERVATION-S1` is landed as a caller-zero source
observation for exactly one non-true loop with one explicit-else direct
Break/Continue branch. The projection retains resolver-owned sites, typed
direct-exit origin/target evidence, and owner/origin/kind/site/frame identity;
it does not claim condition type/effect, carrier/update, return, nested-loop,
Recipe, or physical semantics. The test-only adapter and pure policy observer
preserve the C/D/U/R matrix. Nine policy tests, five projection tests, and the
shared family observer guard are green. The legacy LoopCond variants remain
migration-only; no selector, Recipe/Builder/MIR, retry/fallback, production
caller, or legacy deletion is open.

## Family row context-retention R0 receipt

`FAMILY-ROW-CONTEXT-RETENTION-R0` is landed as a behavior-neutral, caller-zero
BoxShape refactor. DirectAccum, NestedPredicate, LoopTrue, LoopCond, and
Generic G0 now retain expected/observed identity, mode, and coverage evidence
on all four dispositions, with typed reasons/payloads preserved. The focused
observation suite has 89 passing tests and the shared guard is green. This
receipt does not open the assembler, selector, Recipe, Builder/MIR, production
caller, or legacy retirement; the common assembler is next.

## Current source-to-selection evidence

| fixture class | source witness | current generic schedule | status |
| --- | --- | --- | --- |
| V0-only | `v0-additive` | no proven V0-only result | `UnresolvedStop` |
| V1-only | `v1-only` | `GenericLoopV1` | observed |
| Both | `both` | release/strict: `GenericLoopV0, GenericLoopV1`; planner-required: `GenericLoopV1` | observed overlap; precedence unresolved |
| Neither | `neither` | empty | `PreEffectDeclined` before Builder effects |

The `Both` fixture's nested inner Loop is also observed through the actual
depth-1 handoff: release, strict, and strict+planner-required all reach
`NestedDepth1Fastpath = Succeeded` with a Builder delta. The subsequent
`NestedGenericFallback` is `NotYetObserved` because the fastpath succeeds;
fresh-candidate repeats are identical. The matrix's nested `GenericLoopV1`
route label is trace metadata only; it is not a V1 selector or winner claim.

`contract_present = false` is an ordinary current Generic input for release
and strict modes. It is recorded in the matrix; it is not silently converted
to a Generic pre-effect decline. The pure nested-carrier policy probe may still
return `UnresolvedStop` when that contract receipt is absent.

## D4-S4 Generic Recipe handoff boundary

D4-S4-D0 is closed as a design-only authority decision. The current
`SelectedFamilyV1` is a marker without source/window/`BindingRef` provenance,
and the current Generic V0/V1 facts plus P2 snapshot are AST/Builder-derived;
none is a portable Recipe input. A future `Selected(Generic)` must retain one
resolver-issued source lease/window, exact mode/coverage, a sealed Generic
candidate envelope, and role-level `BindingRef` provenance. Window `V1Only`,
`Both`, `Neither`, `NoStandaloneRow`, or planner-unsealed evidence cannot issue
that selection.

The future Generic-specific demand must be distinct from the legacy
`VerifiedSelectedLoopRecipeDemandV1`. Only the dedicated Generic Recipe
producer may issue `LoopBindingKeyV1` and seal the internal
`BindingRef`/recipe-key/source-role effect relation. Binding SSA remains the
sole `BindingRef` -> `ValueId`/`PHI` owner. Recipe/effect failure is terminal;
legacy route reconstruction, retry, fallback, and Generic-as-DirectAccum or
NestedPredicate aliases are forbidden. The following paragraph is retained as
historical handoff context: D4-S4-D0 through D4-S4-S3-S0 subsequently closed
as design/test-only evidence, without a public semantic row or production
caller. D4-S4-S3-D1 and S1-S1 have since closed their authority boundaries.
The current blocker is the shallow `GENERIC-SELECTION-OPEN-D0` promotion gate
design in `CURRENT_STATE.toml`; deep D4 substrate/policy evidence is closed.
No `Selected(Generic)` or Recipe claim is implied here.

```text
resolver SourceLease -> AST-free Generic shape/candidate envelope
  -> policy mode/coverage observation -> selector -> Generic demand
  -> Recipe producer (sole key/effect owner) -> Binding SSA (sole PHI owner)
```

Any later implementation cell must update this reference, the active/current
mirrors, and affected support READMEs in the same commit. A cfg(test)-only
numeric/policy witness still does not create a public reference row; only a
public semantic contract or production consumer may do so.

## Stage and disposition contract

The matrix records these stage arms separately:

```text
facts absent/non-match
composer precondition with no candidate delta
composer first allocation/body/pipeline delta
composer error after candidate delta
strict shadow Some/None/Err
release verifier Ok/Err
release lower Some/Ok(None)/Err
nested fastpath and nested Generic fallback
```

The closed debt vocabulary is:

```text
PreEffectDeclined   facts/policy miss with no Builder effect
PreEffectBlocked    source/policy precondition unavailable before mutation
TerminalFreezeTarget candidate was effected; retry would reuse dirty state
ImpossibleEdge      closed invariant proves the arm cannot occur
UnresolvedStop      evidence is insufficient to choose the above
```

An effectful composer/verifier/lowerer failure is never labelled
`PreEffectDeclined`. Unobserved natural arms are retained as
`NotYetObserved`/`UnresolvedStop` rows; no failure injection is used.

The accepted-body re-observation still finds no natural strict shadow `Err`,
release verifier `Err`, or release lower `Err`. Those rows remain explicit
`UnresolvedStop` evidence; strict shadow `None` and release lower `Ok(None)`
retain the valid-Generic completion `ImpossibleEdge` invariant.

The nested diagnostic calls the raw `lower_nested_loop_depth1_any` helper to
preserve an `Err` outcome; production wraps that helper with `.ok()` before its
fallback. This keeps the observer aligned with production order without
creating a second route authority.

D2-A3-S1 has now closed its bounded natural strict/release failure-arm and
nested-depth observation. It preserved the lower-`None` `ImpossibleEdge`
invariant and changed no grammar or IR semantics. This page was synchronized
as the required post-implementation closeout surface; deeper failure arms and
V0/V1 winner equivalence remain parent design-stop work.

## D2-B2 overlap parity evidence

The test-only parity matrix joins the shared production frame, fresh direct
V0/V1 stage rows, semantic digests, and the real witness trace. Release and
strict retain `[GenericLoopV0, GenericLoopV1]`; both direct plans reach
`LowerSome`, but their nested-carrier digests differ. The witness terminates
at V0 with no debt receipt and no V1 attempt. Planner-required suppresses V0
before effect and reaches V1 separately. The pure probe and final comparison
remain `UnresolvedStop`; no winner or retry policy follows from this evidence.
The matrix is closed as deterministic evidence;
`ParityDispositionV1::UnresolvedStop` is a classification, not a policy
evaluator or winner. The next bounded design stop is D2-B4: a test-only
certificate candidate for complete recursive-carrier observations with a
natural V1 stage; all other classes remain unresolved.

## Snapshot ownership

The matrix compares `before_compose`, `before_lower`, and `after_lower`
snapshots containing block count, next ValueId, typed-value count, and variable
map size. Variable-map restoration is not candidate rollback: the composer can
leave block/value/type counters changed. Therefore `GenericComposer` is the
first effect owner whenever the compose delta changes those counters, even if a
later verifier is pure.

## Non-claims

This reference does not claim:

* V0/V1 semantic precedence or winner equivalence;
* a debt-to-later-winner trace;
* a portable Generic Recipe producer or consumer;
* shared JoinSig/PHI/physicalizer ownership;
* retry/fallback removal or JoinIR deletion;
* any language grammar or source syntax change.

## D2-B4-S1 certificate snapshot

The test-only S1 matrix uses the existing `Both` fixture. In release and strict
mode the frozen raw schedule is `[GenericLoopV0, GenericLoopV1]`; the recursive
facts label is `["j"]`, the natural V1 stage is `LowerSome` with
`GenericComposer` as its first effect owner, and a fresh repeat is stable. The
V1 outer final-value list is `["i", "j"]`; the carrier-projected subset is
`["j"]`, selected by the required `loop_carrier_j` and `loop_step_in_j` tags.
The legacy witness still attempts/terminates at V0 with no debt receipt, so the
parent D2 disposition remains `UnresolvedStop`.

Planner-required is a separate row with raw schedule `[GenericLoopV1]`; it does
not issue an overlap certificate. The certificate DTO and all five focused
tests live under `cfg(test)`; production selection, Recipe/JoinSig/PHI,
physicalization, Retry, and scheduler authority remain unchanged.

Those claims remain blocked until the parent M4 design stop closes with a
complete matrix, precedence/disjointness proof, and witness equivalence.

## D2-B4-S2 BindingRef disjointness witness

The bounded S2 witness is green with:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
```

Its source authority is one parsed outer-`j` function, the sealed resolved
loop forest, resolver-issued assignment/read `BindingRefV1`s, the shared
function/frame identity, and canonical `GenericLoopV1Facts` observation. The
positive Release/Strict row captures `[GenericLoopV0, GenericLoopV1]`; the
shadowing row resolves the inner `local j` to a different binding and remains
`UnresolvedStop`. The strict planner-required row records V0 as
`SuppressedByPlannerRequired`, captures `[GenericLoopV1]` under the same mode
scope, and remains unresolved. V0/V1 final-value and PHI tags from the older
S1 observer are corroborating only and are not used as BindingRef authority.

This is cfg(test)-only evidence (443-line sibling, no production caller). It
does not claim runtime-result parity, V0/V1 precedence, a winner, a Generic
Recipe/JoinSig/PHI/physicalizer consumer, Retry/fallback removal, or any
Builder/MIR/backend route change. The exact source and typed suppression
boundary are recorded in the closed S2/D3 checkpoints and the active handoff
design card.

S2 and scoped D3 are closed as bounded evidence. The projector coverage row is
also closed as test-only evidence in the co-sealed source-to-selection handoff card
`investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`.
Its five focused tests include one parsed S2A nested-`IfThen` source-view path,
resolver/source/frame/facts-only co-seal, and typed cross-invocation mismatch.
It does not authorize a production selector, Recipe/JoinSig/PHI/physicalizer
caller, or Retry/fallback change. The parent Generic D2 disposition remains
unresolved.
The cfg(test)-only source-backed handoff bridge
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-BRIDGE0-D1` is closed. It connects one
parsed S2A projector receipt to actual facts/raw schedule/frame flags for
Release/Strict natural Both, and rejects a cross-invocation pairing before
selection. It adds no neutral issuer or production selector. The proposed
V0-only/CompleteNoRecursive subrow was rejected by premise audit because the
existing additive matrix is synthetic and does not establish a parsed source
row; no natural V0-only witness is proven. The planner-suppression row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-PLANNER-SUPPRESSION0-D2-S1` is
closed as cfg(test)-only evidence: the existing parsed S2A source runs under
actual Strict+planner-required mode, co-seals resolver/facts/frame/mode
evidence, and proves raw `[V1]` with typed
`UnresolvedStop(PlannerRequiredV0Suppression)`. No Legacy, eligibility, winner,
or production selector is implied; the parent source-to-selection boundary
remains a design stop.

The D3-S2 P1 source-projection packaging is also closed as inspection-only.
The existing non-`Clone` resolver provenance product and projector/source
bridge remain the sole evidence owners. The machine-readable witness is
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-source-projection-d3-s2-p1-matrix-2026-08-05.tsv`;
it records exact source paths, resolver owner brands, `BindingRefV1` role
relations, strict-ancestor results, and typed pre-effect mismatch reasons.
This does not promote Generic facts, select a route, issue a Recipe key, or
add a production caller. The selected next row is the neutral AST-free facts
snapshot design/test task
`JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`.

The D3-S2 P2 neutral facts snapshot is now closed as cfg(test)-only evidence.
It consumes exactly one sealed P1 resolver provenance product and adds only
the mode-neutral `NestedWriteWithPostLoopRead` disposition. It does not modify
`LoopFacts`, `LoopStructuralFactsPayloadV1`, Generic V0/V1 facts, selector,
Recipe, Builder, MIR, PHI, Home, debt, retry, fallback, or runtime ownership;
P1 typed rejects remain the sole source/owner/frame gate. No production caller
is authorized.

The P3 bounded independent-column family-overlap census is closed as
cfg(test)-only evidence:
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-OVERLAP-CENSUS0-D3-S2-P3`. It records
raw Generic mode/carrier/schedule rows separately from resolved
NestedPredicate/DirectAccum/A+ rows and an explicit canonical rejection.
Fixture labels are reporting-only because no common source/owner/frame brand
exists. Existing overlap remains precedence evidence, not an exact
disjointness proof. The only cross-authority result is
`UnresolvedStop(FamilyOverlap)`; no shared classifier, winner, selector,
Recipe, BindingKey, Builder, MIR, or production caller is added.

## D4 shared source-window witness and next route stop

`JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-WITNESS0-D3-S2-D4-S0`
is closed as a private `#[cfg(test)]` transport witness in
`src/mir/shared_loop_source_window_tests.rs`. One non-`Clone` resolver-owned receipt
lends paired raw/resolved views through a consuming `with_views` call. Four
focused tests cover the canonical nested-loop row plus foreign-owner,
non-loop, and equal-shape distinct-session rejects. This proves source
owner/site/frame/forest identity only; it does not prove family disjointness or
authorize a classifier, selector, Recipe, Builder/MIR, or production caller.

The D4-S1 DirectAccum route design is accepted: the resolver/source unit stays
the sole identity authority and the existing DirectAccum preflight probe is
the first test-only consumer. The raw Generic edge, NestedPredicate
precedence, A+ fallback, and retry/fallback boundaries remain unchanged.

### D4-S1 witness closeout and D4-S2 boundary stop

The D4-S1-S0 witness is closed as cfg(test)-only. It consumes the D4 paired
source views, confirms the exact existing DirectAccum source-unit probe admits
the canonical Local/Loop envelope, and records foreign/non-loop receipt
rejects plus a loop-body-shape terminal reject before Builder effects. It adds
no production caller or family selector.

The next active row is the docs-only
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-BOUNDARY-DESIGN0-D4-S2`. It must freeze
one owner map and one complete disposition matrix for raw Generic V0/V1 and
resolved NestedPredicate/DirectAccum/A+ observations across modes, carrier
completeness, shadowing, owner/frame mismatch, and the listed unsupported
shapes. Natural Both remains `UnresolvedStop(FamilyOverlap /
WinnerCorrectnessUnavailable)`; planner-required V0 suppression remains typed
unresolved. No selector, retry, fallback, or edge retirement is authorized
until that design is accepted.

D4-S2 owner map and boundary are now frozen as docs-only policy: resolver owns
source identity; neutral `loop_structural_facts` may own only AST-free
facts/eligibility; the Recipe producer alone issues `LoopBindingKeyV1`; one
non-`Clone` canonical plan co-seals route-affecting inputs; and
`registry/selection.rs` alone may consume that plan for policy. Its matrix is
`V0-only|V1-only|Both|Neither` × `Release|Strict|planner-required` ×
`CompleteRecursive|CompleteNoRecursive|Unavailable|Ambiguous` × source relation
(`exact|shadowing|foreign/mixed|missing`) × shape (`exact|nested-wrapper|
duplicate-write|Index|Program|CompoundAssignment`), with resolved
NestedPredicate/DirectAccum/A+/canonical-reject columns independent. No old
edge is retired here; later cutover requires one selector, duplicate caller
zero, same-commit old-edge deletion, and retry/fallback zero.

### D4-S2-S0 legacy same-source census (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-LEGACY-SAME-SOURCE-CENSUS0-D4-S2-S0` is a
private `#[cfg(test)]` retirement inventory, not canonical policy evidence. It
consumes one resolver-owned non-`Clone` source receipt for each of six rows:
`nested-predicate` and `direct-accum` × `Release`, `Strict`, and
`StrictPlannerRequired`. Each row retains resolver owner/site/frame plus
`legacy_*` raw facts status, V0/V1 presence, carrier, raw schedule, and the
existing resolved preflight family.

The measured rows are exact and mode-stable: nested-predicate is
`CompleteRecursive(["j", "sum"])` with legacy
`[NestedLoopMinimal, GenericLoopV1]` and resolved `NestedPredicate`; direct-
accum is `CompleteNoRecursive` with `[AccumConstLoop]` and resolved
`DirectAccum`. All six rows are `Available`, V0 absent, and V1 present. The
census does not issue a selector, winner, Recipe/key, Builder/MIR effect, or
retry/fallback, and does not retire an old edge. D4-S3-D0 is now closed as the
docs-only authority decision; D4-S3-S0 is the closed private observation-set
witness and D4-S3-S1 is the next matrix-only row.

### D4-S3-D0 canonical selection authority (closed design)

The future canonical product is the resolver-branded, non-`Clone`
`VerifiedLoopFamilyAdmissionWindowV1`: one source receipt/window, one exact
mode snapshot, one coverage seal, and family-tagged rows with typed
`Candidate|Declined|Unresolved|Rejected` dispositions. Legacy `Blocked` and
the old `VerifiedLoopFamilyObservationSetV1` witness name are historical only;
they are outside this common row algebra. Semantic family tags
are not route IDs. The set contains no AST, raw schedule/cursor, Recipe/key,
Builder/MIR/ValueId/PHI, retry, or fallback.

A new family-level `CanonicalLoopFamilySelectionV1` in
`mir::loop_route_policy` is the future sole selector and consumes only the
assembler's `Ready(window)` once. Its S2 outcomes are
`Selected|Rejected(Overlap)|Unresolved(OutOfWindow)` over exactly five
`Candidate|Declined` rows. `NoCandidate` requires a sealed whole-unit proof
and is not an S2 outcome; missing or foreign identity, incomplete coverage,
planner-unspecified suppression, and BindingRef/frame mismatch are assembler
failures that never reach S2. A+/Trivial stay outside this Loop-family set.
The existing 19-route evaluator and the live DirectAccum/NestedPredicate
resolved lanes are preserved as migration/live owners; Generic selection
remains caller-zero. D4-S3-S0 is a closed private observation-set witness,
not a selector or production cutover.

### Common admission assembler S1 (landed)

`LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1` is the caller-zero route-policy
assembler for the canonical `VerifiedLoopFamilyAdmissionWindowV1`. It consumes
one resolver-issued identity lease plus an arbitrary-order move-only vector of
the five typed family rows, validates exact tag coverage and co-sealed
identity/mode/coverage, and canonicalizes only after all checks pass. Failure
evidence retains the lease, every row, and typed issues with `Rejected` taking
precedence over `Unresolved`.

Candidate payloads are opaque to this assembler: it neither counts candidates
nor rejects semantic overlap or `OutOfWindow`; those remain selector-only. A
non-Ready assembler result is terminal for this boundary and is not passed to
the selector. The six focused tests and shared caller-zero/line guard are
green. No Recipe, Builder/MIR, production caller, retry/fallback, or legacy
retirement is claimed; selector design/consumer is the next bounded cell.

### Selector S2 design correction (2026-08-06)

The selector design is deliberately narrower than the row algebra carried by
the assembler. `CanonicalLoopFamilySelectionV1` will consume only
`Ready(window)` once, so its five rows are exactly `Candidate|Declined`:

```text
1 Candidate + 4 Declined -> Selected
2+ Candidates            -> Rejected(Overlap)
5 Declined               -> Unresolved(OutOfWindow)
```

Missing/duplicate tags, identity/frame, mode/coverage, and row-level
`Rejected|Unresolved` are assembler outcomes and never reach S2. `NoCandidate`
is not an S2 result; it requires a separate whole-unit no-loop-envelope proof.
The implementation belongs in a new `family_selector.rs`; the historical
Generic marker in `family_selection.rs` is not promoted. S2 remains
caller-zero and must not issue Recipe/JoinSig, Builder/MIR, physical, retry,
fallback, or production effects.

### D4-S3-S0 observation-set witness (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-OBSERVATION-SET0-D4-S3-S0` is
closed as a private `cfg(test)` witness in
`src/mir/shared_loop_source_window_tests.rs`. Each
`TestLoopFamilyObservationSetV1` owns one non-`Clone` resolver receipt, one
private Release/Strict/planner-required mode snapshot, one loop-window-only
coverage seal, and three semantic family rows. All rows are typed
`Unresolved`; this proves the transport shape without selecting a winner,
precedence, or `NoCandidate` result.

The focused test covers two existing fixtures across three modes (six sets)
and checks owner/origin/source-kind/site/frame correspondence through the
consuming paired-view seam. No route ID, raw schedule/cursor, AST, Recipe/key,
Builder/MIR/ValueId/PHI, retry/fallback, selector, or production caller is
introduced.

### D4-S3-S1 canonical matrix (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-MATRIX-CLOSE0-D4-S3-S1` is closed
as a private registry witness. It issues one resolver-branded non-`Clone`
source-window receipt for three parsed fixtures (`Both`, `V1Only`, and the
existing `NoStandaloneRow`) across `Release`, `Strict`, and
`StrictPlannerRequired`: nine sets. Each set consumes its receipt once and
records resolver identity, facts status, V0/V1 presence, carrier provenance,
and four explicit presence cells (`V0Only`, `V1Only`, `Both`, `Neither`).

`NoStandaloneRow` is never collapsed into a real `Neither` Generic presence;
`V0Only` and a parsed `Neither` source remain `NotYetObserved`. The natural
`Both` fixture is V0/V1 in Release/Strict and observes mode-local V1-only under
planner-required V0 suppression; this is unresolved evidence, not intrinsic
winner or suppression policy. A planner-required facts freeze leaves all
cells unobserved. Foreign-owner and non-Loop inputs remain typed rejects.
The witness calls the facts owner directly and introduces no legacy schedule
selection, selector/winner/precedence, Recipe/key, Builder/MIR, retry,
fallback, runtime, or production Generic caller.

The next row is the private pure selector consumer
(`...CANONICAL-SELECTOR-PURE0-D4-S3-S2`).

### D4-S3-S2 pure selector (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTOR-PURE0-D4-S3-S2` is
closed as a private `#[cfg(test)]` neutral consumer in
`src/mir/loop_route_policy/family_selection.rs`, separate from the legacy
19-route evaluator. The registry adapter passes only a neutral window-complete
Generic evidence row. It does not pass AST, LoopRouteContext, fixture labels,
owner coordinates, route IDs, raw schedules/cursors, or legacy policy
evidence.

The outcome vocabulary is typed `Selected`, `NoCandidate`, `Rejected`, and
`Unresolved`, but the current S1 input cannot construct the first two: a
window-complete seal is not a whole-unit no-Loop proof. All nine source/mode
rows therefore remain `Unresolved`, preserving overlap, V1-only,
NoStandaloneRow, and planner-mode-unsealed reasons. Foreign/non-Loop source
window rejects remain before the selector. No Recipe/key, LoopBindingKeyV1,
Builder/MIR, retry/fallback, runtime, or production Generic caller is added.

The next row is the design-only Generic Recipe handoff
(`...GENERIC-RECIPE-HANDOFF0-D4-S4-D0`).

The bounded row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-INDEX-AMBIGUOUS0-D2-S2` is
closed as cfg(test)-only evidence. One parsed S2A-shaped nested IndexWrite
(`items[j] = i`) co-seals resolver `IndexWrite`, facts
`Ambiguous("assignment target")`, exact source/forest/frame identity, actual
Release/Strict mode, and raw `[GenericLoopV0, GenericLoopV1]`. The typed result
is pre-effect `UnresolvedStop(IndexWriteAmbiguousCarrier)`; no eligibility
issuer or selector arm is implied. The bounded
`JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0` row is also
closed as cfg(test)-only evidence: actual Release/Strict natural-Both
`CompleteRecursiveCarrier` is the only test-only eligible result, while
planner, shadowing, missing-capability, and cross-invocation mismatches remain
typed unresolved. It does not close the production handoff;
Compound/Unavailable D2-S3 is now closed as the adjacent source-matrix row and
the parent D3 design stop remains current.

The scoped D3 matrix is now also green as one cfg(test) test over four typed
rows: natural Release, natural Strict, shadowing negative, and planner-required
V0 suppression. Its evaluator separates pre-effect BindingRef eligibility from
post-effect V1 corroboration. The projector coverage row is still test-only;
the source-to-selection handoff card remains the design authority
`investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`;
no production selector change is implied.

The bounded source row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`
is closed as cfg(test)-only evidence. It uses a parsed nested
`CompoundAssignment` under scoped basic sugar, actual resolver/source/frame/
BindingRef evidence, and the facts-owned
`Unavailable("CompoundAssignment")` disposition. Release/Strict measured raw
schedule is `[V0,V1]`; the only result is typed pre-effect
`UnresolvedStop(CompoundUnavailableCarrier)`. Top-level compound behavior,
eligibility, Legacy, winner/precedence, and production handoff remain outside
this row; execution returns to the parent D3 design stop.

The selected premise was
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`.
It is not a policy row: one parsed top-level `CompoundAssignment` must first
be observed through resolver/source/frame identity and the facts extractor.
The result space was open between exact `CompleteNoRecursiveCarrier`,
`Unavailable`, `Ambiguous`, and typed `NoStandaloneRow`. The implementation
observed typed `NoStandaloneRow`: the parsed resolver/BindingRef/source/frame
witness is present, but no facts product is emitted and Release/Strict both
measure raw schedule `[]`. This is cfg(test)-only evidence and does not
authorize collector widening, selection, eligibility, Legacy/winner policy,
Recipe, PHI, Builder, MIR, Retry, fallback, or production handoff. The linked
task, current mirrors, and this reference page were closed together.

The accepted implementation child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`.
It is a docs-only boundary for choosing one parsed flat Assignment shape and
its disposition. `CompleteNoRecursiveCarrier` is an observation label, not a
winner or eligibility proof; the provisional one-member result is typed
`UnresolvedStop(NonRecursiveOutOfTarget)`, while facts absence or empty raw
schedule is `NoStandaloneRow`. Simple-while, local/effect V1-only,
CompoundAssignment, selector, Legacy, Recipe, PHI, Builder, MIR, Retry,
fallback, and production handoff remain separate.

The implementation child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`.
It may add exactly one parsed flat Assignment witness. Exact
`CompleteNoRecursiveCarrier` plus measured `[V0,V1]` maps only to typed
`UnresolvedStop(NonRecursiveOutOfTarget)`; facts absence, empty raw schedule,
simple-route/V1-only schedules, shape drift, and identity drift return to the
D2-S5-D0 design stop.

The S1 witness is now closed as cfg(test)-only evidence. It observes exact
`CompleteNoRecursiveCarrier` with Release/Strict raw `[GenericLoopV0,
GenericLoopV1]` and maps only to typed `UnresolvedStop(NonRecursiveOutOfTarget)`
for the one-member out-of-target shape. It does not establish a winner,
eligibility, Legacy, selector, Recipe, PHI, Builder, MIR, Retry, fallback, or
production handoff.

The accepted docs-only design child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`.
It must partition source-backed rows into
`ResolvedCandidate`, `LegacyPreserveExistingSchedule`, `UnresolvedStop`,
`NoStandaloneRow`, and `NotYetObserved`, then define the winner/disjointness
proof for natural recursive Both. The current recursive Both row remains
`UnresolvedStop(WinnerCorrectnessUnavailable)`; route labels, digests, and
legacy receipts are corroboration only. Its selected cfg(test)-only child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`, now
closed with V0=false, V1=true, `CompleteNoRecursiveCarrier`,
`has_body_local=false`, actual frame flags, no recipe contract, and raw `[V1]`;
its typed result is `UnresolvedStop(V1OnlyNonRecursive)`. No selector or
neutral handoff is authorized by this reference entry.

The D3-S1-S2 candidate-stage source bridge is closed as cfg(test)-only
inspection evidence. It reuses the parsed natural-Both source for resolver
forest/BindingRef facts and fresh V0/V1 candidate plans. Release/Strict retain
raw `[V0,V1]`, direct `LowerSome`/`GenericComposer`, order-independent
snapshots, and distinct resolver owners. V0 lacks outer `j` while nested V0
retains it; V1 records outer `j` with `loop_carrier_j` and `loop_step_in_j`.
These are label-backed plan projections, not typed BindingRef provenance, and
the direct loop context does not lower the full post-loop return. Planner-
required remains `[V1]` and unresolved; the actual legacy trace is V0
terminal/no-debt. No winner, selector, issuer, Recipe, PHI, Builder, MIR,
retry, fallback, or runtime authority is added.

The next accepted design stop is
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`,
recorded in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md`.
It is docs-only: resolver-owned `BindingRefV1` provenance, an AST-free neutral
facts snapshot, a logical loop-binding relation, and one non-Clone opaque
handoff must be specified before any issuer/selector implementation. Full
scalar Return projection, natural debt-to-different-winner evidence, and Home
semantics remain deferred; label/ValueId inference and synthetic debt remain
non-authoritative.

The first selected child is the cfg(test)-only
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`,
recorded in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-provenance-observation-d3-s2-s0-task-2026-08-05.md`.
It observes resolver forest/frame and exact `BindingRefV1` role/ancestry only;
Generic snapshot/key issuer, seed/opaque input, selector, Builder/MIR, and
Return/Home/debt meaning remain unimplemented.

That observation child is closed as cfg(test)-only evidence: four focused
tests seal natural resolver forest/frame plus exact `BindingRefV1` role and
ancestry, and reject shadowing, foreign owner, forest-shape, and frame
mismatch. Production caller/import is zero and artifact is none. Generic
snapshot/key/seed ownership and winner/Return/PHI/Home/debt semantics remain
the D3-S2 design stop. A premise audit additionally found that the current
forest/frame coordinates omit a resolver owner/invocation brand, so equal
origin/site coordinates from two sessions can be mixed; this witness is not a
production capability until the cross-session brand audit is accepted.

## D2-B4-S2A nested `IfThen` carrier evidence

The bounded S2A row is closed as one parsed, `cfg(test)`-only carrier witness.
The source has an outer loop, an inner loop, a nested `IfThen` write to `j`, a
separate canonical inner `j` step, and a post-loop `j` read. Resolver-issued
`BindingRefV1` identity, strict ancestry, source/frame identity, and the exact
two-member loop forest are asserted. Release/Strict raw schedules remain
`[GenericLoopV0, GenericLoopV1]`; fresh direct V0/V1 stages are `LowerSome`
with `GenericComposer` as first effect owner and stable distinct digests. The
V1 witness records `CompleteRecursiveCarrier(["j"])`; the legacy witness still
terminates at V0 without a debt attempt.

This is inspection-only evidence. It does not select a winner or add a Generic
Recipe, JoinSig, PHI, physicalizer, Builder, MIR, backend, Retry, fallback, or
runtime consumer. Parent Generic D2 and the co-sealed source-to-selection
handoff remain unresolved; the current facts-only selector is unchanged.

## Selector S2 caller-zero implementation receipt (2026-08-06)

The canonical selector consumer is now landed in
`src/mir/loop_route_policy/family_selector.rs`, while production selection
remains closed. Its only input is the common assembler's `Ready(window)`
product, consumed once; non-Ready assembler outcomes never reach it. The
selector implements exactly:

```text
1 Candidate + 4 Declined -> Selected
2+ Candidates            -> Rejected(Overlap)
5 Declined               -> Unresolved(OutOfWindow)
```

Selected and failure products retain the resolver lease and the appropriate
typed row evidence. `NoCandidate` is not introduced here and still requires
the M8 whole-unit proof. The three focused tests cover every typed family
candidate, overlap retention, and five-row `OutOfWindow` retention. The
selector has no AST/source lookup, route/schedule, Recipe/JoinSig, Builder/MIR,
retry/fallback, or production caller. The historical `family_selection.rs`
marker remains test-only. This matrix entry is the required same-commit
post-implementation reference receipt. D4-S4-S0 remains a NoSafeSlice audit;
the next shallow gate is `GENERIC-SELECTION-OPEN-D0` for a real resolver-issued
candidate envelope. Recipe handoff, physical parity, production cutover, and
legacy retirement remain later gates.

## Static-call result publication I0/R0 receipt (2026-08-07)

`GENERIC-RAW-STRUCTURED-STATIC-CALL-RESULT-PUBLICATION-I0-R0` is landed as a
disconnected, caller-zero publication witness. An exact source demand is
sealed by `(Cataloged caller, SourceExprSite)` and carries no AST, `ValueId`,
or type authority. A successful physical
`CompletedUnifiedValueCallEmissionV1` is the sole source of the final
destination; a single-use publication consumer writes `MirType::Integer`
only after that receipt is available. Duplicate publication, failed physical
emission, foreign source/site, name lookup, and fallback are rejected.

The focused source and Builder tests are green. This row does not wire
`RawInvocationChildPortV1`, GenericLoop, production selection, retry/fallback,
or legacy deletion. The canonical VM probe still reports
`MissingTransientType { init: ValueId(3) }` because the new bridge is
intentionally disconnected; this is a pre-production evidence boundary, not
a production support claim. I1/D0 is now accepted: the existing outer module
candidate is the sole rollback owner, and the bounded I1/R0 activation must
turn post-effect failure into terminal Freeze rather than retry debt. The next
gate is the one-caller implementation; no production support claim exists
until its receipt and fresh strict probe pass. The subsequent I1/R0 caller
audit found no production owner: `RawInvocationChildPortV1` discards the
located source receipts before `route_generic_loop_v1`, while the located
claim/receipt path is test-only. The next boundary is therefore the compact
source-bound handoff design stop in
`generic-static-call-publication-source-bound-handoff-design-stop-2026-08-07.md`.
No by-name selector or direct located-path wiring is authorized.

## Source-bound static-call result publication I1/R0 bounded receipt (2026-08-07)

The bounded source-bound issuer is now wired in the normal module candidate
lifecycle. After candidate `CatalogInstall`, one move-only owner issues the
exact `(Cataloged caller, SourceExprSite, target)` handoff and lends it to the
raw static-call terminal. The terminal consumes the handoff once, emits the
existing `CompletedUnifiedValueCallEmissionV1` receipt, and publishes the
result through that receipt; no AST reread, name-based selector, retry, or
fallback is introduced. Call-argument descent also transports the exact
`Argument(i)` source path so the callable BindingRef ledger remains aligned.

The focused owner, lifecycle, and raw-terminal tests are green and all new
source/check files remain below the 800-line boundary. This is a bounded
terminal consumer only: GenericLoop remains verifier-only, Generic production
selection and legacy retirement remain closed, and a fresh strict VM receipt
is still required before any broader caller cutover.

The fresh post-rebuild strict probe was re-run with
`generic_loop_continue_strict_shadow_vm.sh` against `target/debug/hakorune`.
It still exits `1` at
`MissingTransientType { init: ValueId(3) }` after reaching the generic-loop
planner/shadow tags. The probe does not reach the canonical
`StringHelpers.int_to_str/1` source-site handoff, so it is recorded as a
negative receipt rather than a production-support claim. A dedicated
cataloged fixture is still required before Generic production selection or
physical cutover can open.

## Callable-semantic Loop handoff S0 receipt (2026-08-07)

`GENERIC-CALLABLE-SEMANTIC-LOOP-HANDOFF-S0` is now closed as a pre-effect
boundary. The selected `StringHelpers.int_to_str/1` source is projected
through a source-only migration bridge into one move-only,
AST-free `VerifiedCallableSemanticLoopBindingScheduleV1`. Its focused
receipt contains the one admitted condition-read, body-read, and assignment
rebind profile (two variable reads total):

```text
Body(2).LoopCondition.Lhs          -> ConditionRead
Body(2).LoopBody(0).Value.Lhs      -> BodyRead
Body(2).LoopBody(0).Target         -> BodyRebind
```

The located raw Loop entry consumes this schedule at the pre-effect boundary
and verifies owner, exact role coverage, duplicate-free sites, and the
non-nested profile before calling the existing Generic route. Foreign,
duplicate, partial, nested, and role-mismatched source products reject before
Builder effects. The immutable receipt is
`docs/development/current/main/design/fixtures/generic-callable-semantic-loop-handoff-s0-v1.json`.

This row does **not** claim a portable Recipe/JoinSig projector, consume
callable ledger `ValueId`s, publish PHI, select a production Generic winner,
or retire the legacy route. The current `CallableSemanticLoweringState`
source view and ValueId map remain migration bridges only; BindingSSA is
reserved as the later physical owner. The selected callable fixture is a
single-loop source profile and is not the nested two-loop `generic_g0`
profile; direct projection between them is rejected. S1 is now closed as
caller-zero evidence. The resolver syntax-facts audit found a real authority
gap, so the current design stop is `RESOLVER-SYNTAX-FACTS-D0`: operator/RHS
literal, initializer, prefix-call, and terminal-tail syntax facts must be
sealed by one AST-free observer before MAP-S1 can open. The existing S0
three-role schedule is a subset claim only; it does not close direct-call,
exit, upvar, field/index, lambda, or other callable rows. The later physical
proof and production selection remain closed, and every implementation
cutover must update this reference entry in the same commit as its code.

## Callable source ledger S1 receipt (2026-08-07)

`GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-LEDGER-S1` is now closed as caller-zero
resolver evidence. `CallableSemanticSourceLedgerView` borrows the sealed
callable forest and keeps typed queries for declaration, lexical-reference,
assignment-target, direct-call, exit, lambda/capture, and Loop-membership rows.
`VerifiedCallableLoopMembershipV1` pairs the resolver-issued Loop source token
with its derived `LoopExecutionFrameKeyV1`; no raw path can mint either value.

Four focused resolver tests cover positive typed rows and identity, exact and
missing Loop lookup, the existing capture boundary, and foreign-owner
rejection. The view has no AST, copied ValueId map, Loop policy, Recipe
producer, physicalizer, Builder/MIR caller, retry/fallback, or production
selection. Scope consumption, operation/effect correspondence, After/tail,
completion, and legacy retirement remain closed.

The next authority is the design stop
`RESOLVER-SYNTAX-FACTS-D0`, recorded in
`resolver-syntax-facts-d0-task-2026-08-07.md`. The resolver API does not own
operator/RHS literal, initializer, prefix-call, or terminal-tail syntax facts;
MAP-S1 is therefore `NoSafeSlice` until one AST-free syntax observer is sealed.

## Callable source-to-Recipe map D0 worker review (2026-08-07)

The D0 review keeps one shallow design row and fixes four outputs: the
single-loop profile envelope, the row-by-row source correspondence, common
owner/physical-input boundaries, and the implementation/`NoSafeSlice` gates.
The selected profile is `StringHelpers.int_to_str/1` with `Body(2)`
`loop(i < 1) { i = i + 1 }`, a prefix `value` boundary outside the Loop
Recipe, and terminal `return value` at `Body(3)`. It is not the nested
`generic_g0` profile.

The logical preview is `L0/K0` with `V0` initial, condition
`Read + ConstI64 + CompareLess`, step `Read + ConstI64 + Add + Write`, and
carrier `C0=(L0,K0,V0)`. The source map uses
`(typed_source_site, source_role, target_kind)` coverage because one source
expression can produce multiple operations. Resolver owner/origin/source-kind,
Loop frame, and Scope/Region are co-sealed; path/name/ordinal/AST and the
lowering-state map are not authorities. Prefix/tail and whole-callable
coverage belong to the outer plan; Loop-only completion is `NoSafeSlice`.

Current schema gaps are kept explicit: the common operation/source relation
must carry Recipe item/value keys and literal/operator anchors; the initial
carrier needs a separate source-to-input projection; and the existing nested
`VerifiedGenericAfterEffectG0` cannot be reused for this callable profile.
The next implementation candidate after the syntax observer is caller-zero
`MAP-S1`, documented in `generic-callable-single-loop-source-map-s1-task-2026-08-07.md`
with immutable fixture `generic-callable-single-loop-source-map-d0-v1.json`.
Recipe/JoinSig, physicalization, production selection, and legacy retirement
remain closed.

## Resolver syntax-facts D0 stop (2026-08-07)

The syntax observer owns only as-written shape: operator kind, literal shape,
initializer shape, call-boundary shape, and return-expression shape. Resolver
remains the authority for owner/origin/source-kind, BindingRef, Loop
frame, Scope/Region, direct-call receipt, and exit identity. Type/range/
overflow/monotonicity policy belongs to the numeric substrate/route policy;
the observer does not resolve call targets or rebuild BindingRefs.

The count is fixed as **9 syntax rows plus one separate prefix boundary**:
initial carrier (1), condition Lhs/Rhs/operator (3), step
Lhs/Rhs/operator/assignment target (4), and terminal tail shape (1). Whole-
callable declaration/reference/assignment/exit coverage is sealed by the
resolver-to-MAP join and outer canonical plan, not by the syntax observer.
After `SyntaxFacts-S1`, execution goes directly to `MAP-S1`; no row-specific
D0 suffixes or Recipe/physical/production/legacy work are opened by this stop.

## Resolver SyntaxFacts-S1 implementation receipt (2026-08-07)

`RESOLVER-SYNTAX-FACTS-S1` is closed as caller-zero evidence. The compiler-side
observer publishes a sealed `VerifiedSourceSyntaxFactsV1` containing exactly
nine syntax rows plus one separate prefix boundary. Its vocabulary is limited
to neutral as-written operator, literal, initializer, call-boundary, and
return-expression shapes; resolver identity, BindingRef, direct-call, exit,
frame, and Scope/Region facts remain separate join inputs.

The resolver loop membership handoff now preserves source/frame/Scope/Region
as one move-preserving product. Unknown root-body statements are explicit
rejects, so the observer cannot silently skip an unclassified statement.
Focused caller-zero tests cover exact rows, source-lifetime independence,
foreign context, Scope/Region retention, unknown statements, and a
non-literal condition RHS. There is no MAP, Recipe, ValueId, CFG, PHI,
Builder, production, retry/fallback, or legacy-retirement caller.

The next execution row is `MAP-S1`. Once MAP-S1 is green, work stops for one
`RECIPE-COSEAL-D0` design decision covering common Recipe/JoinSig,
operation-source/effect relation, separate Loop-continuation/callable-Tail
contracts, and co-sealed
Scope/Region/frame. This stage row is evidence-only and does not authorize
physicalization or production selection.

## Callable source-map MAP-S1 implementation receipt (2026-08-07)

`GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-MAP-S1` is closed as a caller-zero,
`cfg(test)` source-map product. `callable_single_loop_source_map.rs` joins the
resolver-owned `CallableSemanticSourceLedgerView` with the sealed
`VerifiedSourceSyntaxFactsV1` product and publishes
`VerifiedCallableSingleLoopSourceMapV1`.

The product has exactly nine loop/tail rows plus a separate prefix boundary:

```text
InitialCarrier
ConditionRead / ConditionBound / ConditionOperator
StepRead / StepDelta / StepOperator / StepWrite
TailReturnRead
PrefixBoundary
```

The map retains typed source sites and neutral syntax payloads. Resolver
`BindingRefV1`, assignment target, and terminal-return evidence are joined
exactly; condition/step/read/write rows share the initial loop carrier
binding, while the tail is an exact lexical read of the selected prefix
result binding. Resolver-issued loop source/frame/Scope/Region are reissued
and fully compared with the syntax context. The prefix optionally retains an
applicable direct-call receipt; the selected MethodCall remains an outer
callable boundary because the resolver has no canonical callable target for
that method call.

The selected MAP policy is fixed to initial `0`, bound/delta `1`, `Less`, and
`Add`; typed/other literals and operators reject before any Builder effect.
Four focused tests cover positive sealing, source lifetime independence,
foreign owner, and an out-of-profile literal. The map contains no AST,
Recipe/JoinSig, ValueId, CFG, PHI, Builder, physicalizer, production route,
retry, fallback, or legacy-retirement authority. The source file remains
under the 800-line lane limit.

`RECIPE-COSEAL-D0-r1` is now accepted as a shallow common design after
external review. It names the
move-only `VerifiedLoopRecipeCoSealV1` boundary: existing verified Core plus
profile-neutral operation-source, input-source, semantic-context, and
`VerifiedLoopContinuationContractV1` capabilities. The selected callable profile maps
`InitialCarrier` to a carrier plus explicit preheader input relation;
condition/step rows to common Read/Const/Compare/Add/Write operations;
`PrefixBoundary` to `VerifiedCallablePreludeV1`; and
`TailReturnRead` to a separate `VerifiedCallableTailV1`. The current row does
not issue exact return ABI or `VerifiedFunctionCompletionV1`; their existing
issuers are joined once by the later prepared physicalization product.
`VerifiedLoopAfterTailEnvelopeV1` is rejected. The nested Generic G0 S4
`VerifiedGenericAfterEffectG0` remains G0-only and is not a common callable
owner.

The resolver/MAP owns source sites, BindingRef, owner/origin/source-kind,
frame, and Scope/Region. The common Recipe producer alone issues logical
keys; JoinSig elaborates logical ports; CanonicalSsaFunctionSession remains the
sole ValueId/CFG/PHI owner; completion/DraftSeal remains the sole terminal
owner. Every source row is consumed once by `(typed site, role, target kind)`.
Missing, duplicate, foreign, unconsumed, cross-owner, or second-owner
evidence is `NoSafeSlice` before physical effects. No production selection,
physicalizer, retry/fallback retirement, or legacy deletion is opened.

The next row is one bounded caller-zero co-seal implementation. Its commit
must update this reference page and `docs/reference/mir/loop-recipe-contract.md`
in the same commit; implementation completion does not imply production
activation.

## Callable Recipe co-seal I0/R0 implementation receipt (2026-08-07)

`RECIPE-COSEAL-I0-R0` is closed as caller-zero evidence. The implementation
consumes `VerifiedCallableSingleLoopSourceMapV1` once and publishes the common
`VerifiedLoopRecipeCoSealV1` plus separate `VerifiedCallablePreludeV1` and
`VerifiedCallableTailV1` contracts. The selected profile is represented by
one recursive `LoopRecipeV1`, one carrier, one explicit preheader input, seven
logical operations, and one Loop After capability. The exact terminal
statement site is preserved in the MAP Tail target; no source path is rebuilt
from names or ordinals.

The new `callable_single_loop_v1` producer id is test-only provenance. Focused
tests cover positive co-seal, source-view drop, Prefix/Tail mismatch, and
Tail/Loop-After fusion rejection. No Builder/MIR/physical IDs, ABI/Completion,
physicalizer, selector, retry, fallback, production selection, or legacy
deletion was opened. The typed function-finish terminal and its bounded
prepare-design correction are now closed for the three V2 session lowerers.
The current boundary is caller-zero topology/After-only
`LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`; operation emission and physical /
production activation remain closed.

## Callable physical prepare P0 receipt (2026-08-07)

The first caller-zero prepare slice is landed in the test-only
`loop_physical_prepare` module. It brands the exact resolved callable input,
moves the common co-seal into the topology-only compatibility
`VerifiedLoopPhysicalDemandV1`, and defines the
typed prelude and Tail/ABI/Completion relation boundaries without opening a
Builder session. Focused tests cover exact catalog branding, source-view
lifetime independence, and the existing MethodCall fixture's typed
`NoSafeSlice::MissingPreludeTarget` result.

This is partial P0 evidence, not a positive physical or production claim. The
fixture's `helper.to_i64(n)` source has no resolver-issued direct callable and
the existing return declaration is unannotated; it cannot prove a positive
Prepared/ABI witness. A genuine positive requires a separately verified
static-call source profile with an exact receiver/target relation. No target
injection, name lookup, AST rematch, physical ID, selector, retry, fallback, or
production caller is allowed in this row.

This historical transport is not the current full operation demand. Operation
preflight uses `VerifiedLoopOperationPhysicalDemandV1` and cannot reuse the
topology-only boundary.

## Callable source-shape split receipt (2026-08-07)

`CALLABLE-SOURCE-SHAPE-THIN0` is closed as a caller-zero, behavior-neutral
BoxShape refactor. Neutral call/literal/operator shapes are isolated from the
syntax observer, and the observer/source-map tests are isolated in sibling
test-only modules. `Method` and `FreeStatic` are explicit source-shape kinds;
`FreeStatic` has no resolver target until the next exact fixture row. The
existing MethodCall fixture remains a natural typed negative.

No Generic/Loop Recipe, selector, physicalizer, Builder/MIR, retry, fallback,
publication, or production claim follows from this split. The next row is
`CALLABLE-STATIC-PREFIX-S0`; same-brand different-owner target validation and
declaration-derived ABI remain later prepare contracts.

## Callable static-prefix observer receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-S0` now has one exact resolver-backed top-level
catalog fixture: `int_to_str(n: i64): i64` calls `to_i64(n: i64): i64` as a
free static `FunctionCall`. The source observer emits explicit
`SourceCallKindV1::FreeStatic` evidence and the test asserts the direct target
comes from the same compilation's callable ledger with a different owner.
The prior `helper.to_i64(n)` `MethodCall` remains an explicit typed negative;
it is not target-injected or relabeled as static.

This is still caller-zero source evidence only. The next cell,
`CALLABLE-STATIC-PREFIX-MAP-S1`, owns same-brand different-owner map acceptance
and foreign-brand rejection. ABI derivation, Prepared positive products,
physicalization, production selection, fallback/retry retirement, and legacy
deletion remain closed.

## Callable static-prefix source-map receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-MAP-S1` is closed as a caller-zero source-map cell.
The map keeps the resolver-issued `to_i64` target for same-brand,
different-owner calls and rejects an independently sealed foreign compilation
brand as `ForeignOwner`. The existing MethodCall fixture remains a typed
negative. This row adds no ABI, Prepared product, Recipe, physicalizer,
Builder/MIR, selector, retry, fallback, publication, or production authority.

The next bounded row is `CALLABLE-STATIC-PREFIX-P0`, limited to
declaration-derived ABI and Prepared evidence.

## Callable static-prefix Prepared receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-P0` is closed as a pre-effect Prepared relation. The
caller ABI is derived from the completion declaration and exact callable
header, while the callee ABI is derived from the resolver-issued target
header. The `FreeStatic` fixture produces one positive Prepared product; the
MethodCall remains a typed `MissingPreludeTarget` negative.

Physicalization, Builder/session effects, selector, retry/fallback,
publication, and production selection remain closed. The next step is a
design-only common physicalizer/session boundary review.

## Callable logical issuer D0/S0 boundary (2026-08-08)

The callable source/facts issuer S0 and bounded logical issuer S0 are closed
with exact resolver identity parity and bounded negative coverage. The logical
issuer reuses the canonical Recipe verifier, JoinSig elaborator, After
binding, and source-bound Core co-seal for the seven-operation mapping. The
profile Recipe shape is production-owned in
`callable_single_loop_recipe.rs`; `callable_recipe()` is only a test parity
wrapper. The logical product remains caller-zero: Prepared/physical/selector/
production-caller/Generic G0/retry/fallback/legacy behavior remain closed.
