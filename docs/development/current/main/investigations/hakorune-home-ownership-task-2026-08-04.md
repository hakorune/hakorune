# Hakorune Home ownership — parked task order

Status: Parked design/execution board; no current execution authority

Date: 2026-08-05

Decision state:

- Home model direction: accepted;
- C′ last-Home finalization target: accepted; production activation 0;
- explicit early Home release: contextual statement `release root` accepted
  for one verified whole-root Home; ordinary/generic `release(value)` has no
  Home authority, `drop` alias rejected, production activation 0;
- exact HomeV1 grammar and physical Shared representation: provisional/D0;
- generic/composite whole-root support under the same `release root` statement:
  provisional/D0; no generic wrapper callable;
- production activation: 0;
- resume checkpoint: `MIRBUILDER-INPLACE-REPLACEMENT0` final-pipeline
  completion;
- current row remains the row named by `CURRENT_STATE.toml`.

Parser cleanliness forward-order correction (2026-08-10):

- reconciliation authority:
  `own-home-parser-cleanliness-reconciliation-2026-08-10.md`;
- before Take/Share activation, close contextual HTRIVIA parity, recut the
  direct-method parser observation transaction, then replace the Hako raw
  parameter transfer tag/builder token with the accepted typed seal;
- nested Release exact paths and Dynamic-local slot indices remain P2 polish;
- this parked correction never overrides the active row in
  `CURRENT_STATE.toml`.

Authorities:

- source semantics: `docs/reference/language/ownership.md`
- cross-layer boundary:
  `docs/development/current/main/design/ownership-home-model-ssot.md`
- terminal finalization:
  `docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md`
- failure/exit transaction:
  `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`
- language workstream:
  `docs/development/current/main/workstreams/language-v1-convergence-current.md`

Supersedes as execution order:

- `hakorune-sparse-ownership-surface-task-2026-07-15.md`
- `hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md`
- `ownership-view-missing-grammar-inventory-2026-07-28.md`
- `ownership-view-performance-compatibility-design-2026-08-04.md`

Those files remain historical evidence. They do not select work or restore
the former `move/share/view` target.

## Goal

Deliver a small source model:

```text
ordinary use        -> non-owning handle
Home-demand edge    -> one Home is transferred
share expression    -> one independent owner is added
release root         -> one verified whole-root Home ends now
terminal Home end   -> one C′ fini/field/native DropPlan
```

while retaining precise compiler products, fail-fast boundaries, and a
measured C-speed physical path.

## Non-goals for the first program

- Rust-style general lifetime syntax;
- a new immutable ownership IR;
- hidden retain/release or profile fallback;
- source-level arena/region allocation;
- general field move-out, consuming receiver, or alias PHI;
- general generic/composite/field/projection explicit release;
- all generics, dynamic dispatch, FFI, or concurrency in the first production
  slice;
- a nominal `shared box` decision before its representation D0.

## Dependency graph

```text
RESUME
  -> CENSUS
  -> TAXONOMY
     -> COMPOSITE/TRIVIAL
     -> REPRESENTATION
     -> STORAGE DESTINATIONS
     -> CALLABLE ABI
     -> TRANSFER/FAILURE TIMING
     -> BIRTH/CONSTRUCTION
     -> RESULT/EXIT D0
     -> LAST-HOME FINALIZATION C′ D0
     -> EXPLICIT WHOLE-ROOT HOME RELEASE D0
  -> SURFACE DECISION
  -> PASSIVE RELATION + ABI + BOUNDARY
  -> STRAIGHT HOME FLOW
  -> CFG HOME FLOW + DIAGNOSTICS
  -> GRAMMAR CARRIERS
  -> PASSIVE EXPLICIT-RELEASE PLAN
  -> UNIQUE CLOSED-CALL + LOCAL TERMINAL-FINALIZATION PROTOTYPE
  -> STORAGE DESTINATION ADOPTION
  -> CONTRACT BOUNDARIES
  -> SHARE MATERIALIZATION + SHARED TERMINAL WINNER
  -> C-SPEED PHYSICAL GATE
  -> PROFILE CUTOVER / RETIREMENT
```

No grammar or production row may jump over the D0 fan-out.

## Milestone 0 — safe resume

### `OWNERSHIP-HOME-RESUME-D0`

Revalidate after the MirBuilder checkpoint:

- live parser/AST/registry ownership surface;
- old inactive-syntax reject guards;
- current SharedV1 producers and fallback edges;
- Binding SSA, Ownership SSA, callable catalog, lifecycle, ObjectStoragePlan,
  and backend owners;
- current `.hako` corpus candidates.

Done:

- current authority versus historical evidence is named;
- no stale row is resumed from the old sparse/View boards;
- worktree is clean and the current-state pointer guard is green;
- the next row is a design/census row, not parser implementation.

## Milestone 1 — bounded evidence

### `OWN-HOME-CENSUS0`

Read-only bounded census of:

- return origins;
- owning stores and container insertions;
- call arguments and captures;
- field/array/map/global/registry storage;
- generic/`Any`/record/enum/Option/Result occurrences;
- function values, interface/dynamic calls, plugin/FFI boundaries;
- places where current code implicitly adds or drops a strong owner;
- existing `fini` declarations classified as ordinary method, scope-cleanup
  alias, runtime/plugin hook, or generated/native adapter;
- direct receiver `obj.fini()` calls and callable-catalog/delegate/interface
  exposure;
- manual parent-to-child `fini` cascades and their intended order;
- last-owner/Arc/Drop/global-finalizer/plugin/native routes that currently
  dispatch or bypass user finalization.

Use static search first, then one case and a small sample before any complete
corpus pass. This is evidence, not syntax authority.

Deliverable: one source-kind × destination-kind × representation matrix.

The matrix is not complete until it emits the decision inputs consumed by the
next D0 rows. These are semantic counts, not raw lexical hit totals. Each
count must include its classification rule, a small sample of source paths,
and an explicit `unknown/unresolved` bucket:

| Next D0 | Required census input |
| --- | --- |
| `OWN-HOME-REPRESENTATION-D0` | distinct nominal types that need an independent Shared owner; distinct type symbols observed in both Unique-only and potential-Shared contexts |
| `OWN-COMPOSITE-TRIVIAL-D0` | owner-bearing record/enum declarations or instantiations; `Option`/`Result` wrapping an owner-bearing payload; unresolved generic `T`/`Any`/recursive sites |
| `OWN-HOME-TAKE-EXPR0-D0` | local Home rename, explicit rebinding, or lifetime-narrowing sites that cannot use an existing destination; ordinary `local x = y` aliases are excluded |
| `OWN-HOME-FIELD-TAKE0-D0` | field/container reads that remove or replace an owner-bearing Home; ordinary field reads and stores are excluded |
| `OWN-HOME-BIRTH-D0` | `new` sites by target, birth hook declarations/parameters, declaration initializers and explicit override stores, and fallible construction paths after a prior Home store |
| `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` | `fini` declarations/calls/catalog exposure, manual child cascades, last-owner drop and plugin/native routes, each with one migration disposition: ordinary fallible `close`/domain method, non-callable automatic hook, structural-only drop, or delete/reject |

The census must state `0` when a row has no corpus consumer. A raw `rg`
count such as every `new` token is an inventory hint only; it cannot by itself
close a representation, composite, or transfer decision.

## Milestone 2 — semantic decisions before grammar

Close `OWN-HOME-TAXONOMY-D0` first. After its vocabulary is fixed, evidence
work for the remaining D0 rows may run in parallel. Every D0 must close and
then converge through `OWN-HOME-SURFACE-D0` before passive products or grammar
activation.

### `OWN-HOME-TAXONOMY-D0`

Fix the exact meanings of:

- Home/place;
- Handle;
- transferable HomeValue;
- Unique, Shared, Weak, Trivial, Unknown;
- destination Home demand;
- terminal Home forward;
- candidate HomeV1 syntax versus parked syntax.

Acceptance includes a no-second-authority rule: the old `move/view/shared`
surface cannot remain accepted beside HomeV1.

Keep five notions distinct:

```text
object identity
Home token (one independent lifetime)
Home slot/place (storage for a token)
non-owning handle
runtime ObjectHandle/carrier
```

One Shared object identity may be supported by multiple Home tokens. Do not
describe this as one physical Home object per Box.

### `OWN-COMPOSITE-TRIVIAL-D0`

Define the recursive capability algebra for:

- primitives and identity-free leaves;
- records;
- enums and variant-sensitive payloads;
- `Option<T>` and `Result<T, E>`;
- containers;
- recursive types and cycles;
- generic `T`, constraints, monomorphization, and `Any`.

Unknown never defaults to Trivial or Shared. Decide exactly which facts are
available before Builder effects.

### `OWN-HOME-REPRESENTATION-D0`

Compare without preselection:

- per-instance Unique-to-Shared promotion;
- nominal `shared box` or another type-level Shared capability;
- direct Unique physical ownership;
- Shared control-cell/registry requirements;
- weak identity/generation coupling.
- whether a Shared HomeValue satisfies a general `take`/Home demand or needs a
  distinct declared Shared demand/type.

Done only with source law, runtime layout owner, backend capability, and
failure boundary. This row must not choose from current `Arc` convenience.

### `OWN-FIELD-CONTAINER-DEST-D0`

Seal the destination matrix for:

- local Home;
- object field;
- array/map/packed storage;
- global/static/registry storage;
- parameter and return;
- weak storage;
- replacement, empty-slot, and destruction behavior.

Also decide ordinary local initialization and reassignment:

```hako
local b = a
b = c
```

It must be exactly handle rebinding, Home replacement, or rejected for the
first profile; assignment context may not guess per runtime value. Cover
uninitialized/`null` locals separately.

Field move-out remains parked unless this row names a separate exact witness.

#### Map-slot dependency of the selected compiler cutover

Decision: an intrinsic Map value slot owns its admitted value obligation.
Observe candidates without consumption and describe both install successors in
one source issuance. Successful install transfers once; detached-old cleanup
Fault never restores the candidate's previous owner. The
[intrinsic Map slot target](../../../../reference/language/ownership.md#intrinsic-map-slot-destination-target)
owns the language law; the
[Map construction owner](../design/collection-literal-construction-ssot.md#runtime-escape-ownership--next-design-boundary)
owns physical/runtime follow-up. Other Home destination families remain parked.
Source authority + canonical issuer: existing resolver source identity and
candidate capability/obligation owners, joined at the matching root's pre-effect
semantic issuance. Map candidate/commit issuance is still missing.
Non-authority: ordinary Handle support, literal syntax alone, MIR i64, runtime
tags/Arc/clone, declaration-only I64UnitTrivial ABI and matching owner numbers.
Fail-fast boundary: reject missing/foreign source relation, unsupported candidate
or stale obligation before effects/artifact; retain existing escape Stops until
actual source/physical consumers close. No implicit share or carrier-to-Home cast.
Smallest next slice: connect same-issuance Map source flow to Completion and
its install/Fault/end consumers. Read/clone/observer migration gates owned-slot
intake, not this source/physical implementation work; insertion repair is verified.
The one-entry transfer law is fixed; do not repeat that policy census.
Non-claims: source Map execution, runtime retention, general Home Flow, full
key/residence/cleanup matrix or public cutover.

Boundary of this design: exact resolved Map child -> candidate evidence -> root
co-seal -> selected raw/Core write -> detached cleanup result. Includes Script
and callable roots, all candidate categories below, duplicate keys and
pre/post-commit failure. Excludes general CFG Home joins and other containers.
It is not an Exhausted census of the full Map cutover; included open dependencies
are listed below and cannot be replaced by scalar-only completion.

**Existing evidence and missing source obligations**

| Candidate | Reusable evidence / required boundary |
| --- | --- |
| Trivial | `callable_parameter_contract/issuer.rs` binds explicit i64 to its exact parameter; `home_prefix_local_flow.rs` observes bounded prefix Integer/Bool/TrivialLocal. Neither independently issues a general Map child. |
| Direct available Home | `home_new_prefix::issue_new_home_prefixes_v1` proves selected Normal local installation/unwind. Map still needs exact availability and removal of the prior cleanup responsibility at commit. |
| Fresh acquisition | The exact producer Normal must issue an evaluation-owned obligation. Before acquisition Normal there is no child obligation. Do not treat a prior local as evaluation-owned. |
| Existing Shared / explicit share result | Passive Home vocabulary is not an admitted producer. Consume separately issued compatible obligations only; missing acquisition remains unsupported. |
| SelfContainedDynamicCarrier | `dynamic_invocation_contract/catalog.rs` supplies the exact target envelope. A specific Normal result needs Live/Forwarded flow; the two-call LoopBodyLocal lifecycle is not a Map input. |
| Ordinary borrow | Exact binding/parameter contract provides borrow evidence only. Alias support cannot recover a root Home for transfer. |
| Unknown/composite | Unsupported until its source capability owner issues evidence; no type/tag/default repair. |

Reuse `OWN-HOME-FLOW0-S0` for source availability instead of duplicating the
prefix classifier. Stored Home state and ordinary Handle observation remain
separate. `node=Owns(A)` and `alias=Borrows(A)` both read as handles, but only
exact binding `node` can supply A to an owning demand. This inventory does not
authorize placeholder variants or independent public proof factories for absent
Shared/carrier producers.

**Root ownership and private API**

Script uses `NormalScriptPreEffectSourceObservationIssuerV1::issue` under its
checked parser loan, then `VerifiedScriptSemanticSourceV1::seal_ast_with_forest`;
retain the joined state through `ScriptSemanticSourcePreEffectPartsV1` across
install. The callable package precedes this Script forest, so its ordinary-New
ledger cannot own Script state. Callable roots use the existing package issuer's
resolved batch/parameter contracts; do not add CFG/Completion ownership to that
package. Each root joins its own evidence before Recipe; independent products
cannot be paired later through equal owner keys.


The existing common source is `VerifiedResolvedOwnerCoreV1` in
`resolved_semantics/product.rs`; Function, Script and forest products already
lend it. Literal/variable observations can borrow this core directly. No new
root trait, adapter, registry or separate core-only extraction task is needed.
When the real flow consumer is connected, remove its unnecessary
ResolvedFunctionLoweringInput retention as part of that same change.

Map membership additionally requires the root's body shape, which is not owned
by the common core. Callable entry uses
`VerifiedResolvedCallableSemanticBatchV1::with_declaration_semantics` and its
existing `VerifiedResolvedCallableSemanticRowRefV1`: one row retains the verified
function/forest and same-issuance body-shape sidecar. The plain
`with_lowering_input` loan currently supplies no body shape and is not this Map
input. Script instead lends core and inline body shape from the same
`VerifiedResolvedScriptV1` at its pre-effect seal. Do not accept caller-supplied
core/shape tuples merely because their owner numbers match.

The following names describe private source-flow responsibilities, not current
callable APIs or new public receipts:

| Operation | Required input and successor |
| --- | --- |
| `observe_candidate` | Exact child site and current source flow; borrow existing evidence, consume nothing. |
| `commit_install` | Same candidate, exact Map parent/EntryValue and successful-install successor; remove the prior obligation once, install slot ownership and expose detached-old responsibility. |
| `precommit_failure` | Exact Fault continuation; preserve prior local/evaluation/carrier ownership until its prescribed cleanup. |
| `finish_displaced_cleanup` | Already-committed successor and cleanup Normal/Fault; retain new-slot ownership and consumed/forwarded predecessor state. |

These describe both semantic successors together, not a compile-time prediction
of runtime success. Use FunctionOwnerIdV1, OwnedExprSiteV1, BindingRefV1 and verified
Map parent/EntryValue identities. Ordinal, name or supporting object identity is
insufficient. Preparation must not become permanent transfer permission: if an
intervening effect changes the referenced obligation, the candidate is stale.
General rebinding/CFG joins stay unsupported until their source flow proves the
condition; no unowned generation counter or runtime lookup fills that gap.

**Construction and compatibility prerequisite**

Accepted compatibility: an ordinary-New Home whose exact source declaration
issues `ObjectDestructionDispositionV1::PlainI64NoHook` may move into the
intrinsic Map slot with its existing outer-storage end obligation unchanged.
It is still a Home, not Trivial. `ConstructionEligibilityV1::Ok(plan)` must
refer to the same branded object definition. This bounded profile needs neither
a generic Unique/Shared classifier nor a new MapCompatibleHome receipt.
Other acquisition/destruction profiles keep their existing unavailable boundary.

The source authority is `instance_constructor_semantic/object_definition.rs`,
which checks the whole declaration, including field and member restrictions.
`destruction_for` and `construction_for` lend that source-backed evidence;
`OrdinaryNewAdmissionClaimV1` retains exact Box source, New site, declaration,
destination, construction and destruction. Preserve this combination at the
root co-seal; object ID, class text and `prior_homes contains binding` alone are
not compatibility. The language end law is
[Map construction/end](../../../../reference/language/lifecycle.md#intrinsic-map-construction-and-end).

Source availability must come from the exact acquisition/local-install Normal
successor. `try_take` creates `NewLocalCommitV1::Pending` before emission;
neither claim presence, a local_commits row nor try_take success issues an
available Home. Physical Pending/Emitted state is a consumer check, never the
source issuer. Existing aliases retain only ordinary Handle support.

**Next source-flow connection and actual coverage**

Boundary: selected callable/App Main source loan -> ordinary-New candidate
and descriptor retention -> root source flow -> Completion cleanup. Script is
still included in full Map cutover but has a different pre-effect root owner;
its numeric Array lifecycle is not an ordinary-New/Map acquisition issuer.
This is not an all-root coverage claim or a new narrower cutover finish line.

Static counterexample: `local node = new Pair(10,20); local m =
%{"x" => node}; return 30`. In `ordinary_new_coseal.rs`, candidates now retain
construction/destruction/Birth descriptors before source scanning. The scanner
saves node's prefix before Map makes terminal cleanup unavailable. Completion
retains that unavailable result; the co-seal moves the existing New descriptor
into its claim but discards terminal relation/field rows.
A New after the Map likewise retains its descriptor with unavailable prefix.
Thus there is no descriptor-issuance cycle to solve by a second registry or
by weakening terminal checks. No execution of this example is claimed.

The existing `with_lowering_input_and_method_source` now borrows body shape
from the same semantic row as forest/projection, inside the same parser loan.
`with_body_shape` preserves the existing product; no second loan is paired by
owner number. This removes information loss, not Map admission or raw lowering.
The Option-reader audit covers current callable-loan consumers: Completion and
ordinary-New do not yet read it; constructor issuance already attaches its own
shape, and generic-G0's shape-dependent callers are test paths. S6C reads the
separate typed declaration row. New consumers must not infer admission from
shape presence alone.

The next implementation contract must connect the existing source-flow owner
(`home_prefix_local_flow` / `scan_new_home_flow`, under OWN-HOME-FLOW0-S0)
to these retained descriptors and the same-issuance Map EntryValue relation.
Close the whole selected root transition, not an accessor-only extraction:

| Source successor | Responsibility state consumed by cleanup |
| --- | --- |
| New acquisition before Normal | No newly acquired child responsibility. |
| New and local install Normal | Exact direct binding owns the original obligation. |
| Map allocation Normal, before entry commit | Map construction owns empty/previous slots; direct local still owns candidate. |
| Entry install Normal | Same obligation belongs to exact Map entry; remove local cleanup once. |
| Detached-old cleanup Fault | New slot stays committed; continue with committed Map cleanup state. |
| Map construction Normal | Same Map responsibility reaches destination; no new Home. |
| Root return/Fault | End only remaining locals plus Map live slots under their own ordered plans. |

The source owner must describe both Normal/Fault successors before Recipe;
physical progress cannot reconstruct the missing state. Keep descriptor source
issuance and root flow co-sealed in the existing package owner, with no Map
state table added to the physical claim ledger. Before code, fix the actual
Completion/Recipe consumer for the changed cleanup state, including Map end;
a source-only Compatible receipt or green prefix test does not close that seam.
Do not repeat the closed descriptor compatibility or root-loan census.

Ordered remaining work in this same series: lend retained New construction/end
evidence into that source walk (its current NewSite-to-Binding input is not such
evidence); issue install Normal/Fault and displaced-end continuations through
the existing control owner; consume them in Completion/root end and physical
operations. Then close native residence/read/observer/end before enabling intake
and the source/host switch. A binding list alone cannot represent precommit and
postcommit Fault cleanup; do not remove the old local from every successor.

**Completion and physical consumer Decision**

`ResolvedCleanupObligationsV1` owns one source flow result containing terminal
order and exact Map operation successors. `function_control_new_homes` currently
attaches only `explicit_empty().with_terminal_homes(homes)`; replace that lossy
attachment, not the Completion owner. `terminal_homes()` may remain a projection
of this result, with no second stored terminal list. Transferred children leave
the committed successor's list and the Map binding remains. Do not delete the
original `local_commits` acquisition/Birth
validation evidence. Do not flatten Map children back into ordinary local end
rows. Map runtime entry order and root lexical order are different authorities.

The source walk issues the following finite operation mapping, with exact Map
site, EntryValue where applicable, and source scope/outward target. No physical
block identity or phase number supplies meaning:

| Source operation | Normal responsibility | Fault responsibility |
| --- | --- | --- |
| Map allocation | Empty construction Map acquired | Outer locals; no Map acquired |
| Key/child evaluation and install before commit | Candidate retains its exact prior owner until install | Acquired active evaluation, construction Map, outer locals; never clean an unacquired child |
| Entry install | Candidate transferred once; detached old value separate | Uncommitted candidate remains with prior owner |
| Detached old end | Committed Map, next entry | Same committed Map; old end attempt consumed; no candidate revival |
| Construction completion | Same Map obligation reaches destination | Cleanup follows the actual preceding fallible operation |
| Root terminal | Remaining locals and each Map's own live-slot end plan | Existing first-Fault/best-effort suffix |

`NewFaultContinuationV1` must retain its direct-local-New membership check.
Map control issuance reuses exact scope/target validation in the control owner,
with Map membership established from the same body shape. The source flow and
these control relations attach once; package co-seals existing descriptors,
and root progress consumes the selected end operation. Neither a Map-only
sibling receipt nor a binding-list diff is an implementation of this mapping.

**Candidate descriptor ownership: implemented connection**

Change: `ordinary_new_coseal` replaces the existing candidate tuple with a
private candidate that owns existing construction/destruction/Birth descriptors
before the source walk. It issues no availability or Map compatibility. The
existing field callback borrows that candidate; its repeated `construction_for`
lookup and tuple reclassification are removed. Final claims and Birth
handoffs receive the same products by move, once, after the source scan.

Validation order becomes candidate source order. Supported success remains
unchanged; multiple-invalid-candidate rejection priority is not claimed
byte-for-byte stable. Previously a field callback could report
`ConstructorLookup(ParentSourceMismatch)` before the general
`ConstructorRelationMismatch`, or inspect a later candidate before an earlier
Birth error. Do not preserve this duplication as a second validation authority.

Keep `ConstructionEligibilityV1::Err` and destruction `Unavailable` as retained
descriptors, distinct from failed lookup. Preserve overrides, builtin exclusion,
selected membership, no-Birth arity, and Birth target/completion/effect checks.
No candidate is inserted into the physical claim ledger before final co-seal.
Existing ordinary-New/field/constructor rejection tests cover the consumers;
package tests additionally cover unavailable-descriptor retention and candidate
source-order rejection. The module README owns this internal loan/transfer
contract; no language meaning or public ABI changes. Source files remain below800.
This deletes a real repeated source lookup, but does not close Map transfer,
phase emission, runtime intake or the full source/host cutover.

**Implemented source flow through the install Stop**

Change: extend the existing source walk and Completion cleanup result for
direct-local Map construction whose entries transfer prior direct ordinary-New
Homes. Preserve ordered entries and duplicate-key replacement, empty Map
construction, exact allocation/install/displaced-end Fault successors and the
remaining terminal order. This is one candidate family in the full Map series;
fresh children, nested Maps, Shared/carrier, other value families and Script
remain included unfinished work, not a reduced cutover finish line.

Source authority/issuer: the same lowering loan supplies initializer, Map and
EntryValue membership. `PrefixLocalFlow` retains the exact acquisition site in
its existing Home payload, not another site map. The scanner alone establishes
Normal-before-use and Available/Consumed. A scoped callback taking exact New
site and destination binding checks the existing candidate's construction and
PlainI64NoHook destruction. It returns compatibility only; foreign/duplicate
evidence rejects, unavailable descriptors return unsupported. It issues no
availability, new receipt or package-owned type into resolved semantics.
Ordinary observations still return Handles, never transfer evidence. After
transfer, direct reuse and aliases rooted in the consumed binding cannot recover
Home or a valid selected field-read/argument observation from that old local.

Map outward-control membership uses exact initializer/destination, body shape,
`exact_scope_containing`, body/function pairs and the region parent. Preserve
New's existing membership checks. Attach the whole root flow once through
`function_control_new_homes`; package co-seal retains it in Completion rather
than saving another Map-only receipt or reconstructing cleanup from list diffs.

Consumer/Stop: retain the successful source package, then reject its sealed Map
lowering requirement at `prepare_install`, before catalog vacancy validation.
Keep `Result<Prepared, Self>` and return that same package. The existing
`with_normal_callable_install_once` maps this immutable requirement to a named
`MapLifecycleConsumerMissing` install reason; it does not rescan source. Testing
only that wrapper is insufficient: direct `prepare_install().commit()` callers
must also be unable to obtain a prepared package. Prepared fields remain private.
Do not put this backend-unconnected reason in source issuance or add a rich
error product carrying Completion. Map requirement takes precedence over a
catalog-occupied error for the same package.

The completed scanner retains both complete and unavailable direct-local Map
observations. An earlier unavailable prefix, implicit exit, empty Map or first
rejected entry cannot erase the install requirement. A rejected Map publishes
no partial transfer and consumes no candidate. Coverage is verified-Completion
root bodies; nested-statement Maps, Completion rejection and Script are not
claimed by this boundary. The prior prefix-count and absent-terminal early exits
are removed because both could hide a later Map from the common install Stop.

Acceptance: inspect actual package Completion for prior-Home transfer,
precommit retention, committed replacement/old-end Fault, ordered Map/root
cleanup and empty construction. Alias/reuse/foreign/fresh-child cases must not
produce accepted transfer. Existing New/field/Birth positives and refusals stay
covered. The normal root catalog lifecycle harness must reach `CatalogInstall`
with the named reason, no catalog installed, no New/Map body allocation and no
compat retry. Module preparation already precedes this boundary, so
`current_module.is_none()` is not its acceptance condition. A failed direct
prepare must return the same source package with its Completion intact.

Retirement: replace the selected Map `PrefixNotCovered` fall-through with this
source result and mandatory install Stop; consumed locals no longer survive in
the committed cleanup/ordinary-observation state. Delete the Stop only with
actual Map lifecycle lowering. Current `MapLiteralEntryWrite` discards results,
the static-V2 body index recognizes plain NewBox/Write, and `InvokeOperation`
has no Map variant: none can substitute for detached-old ownership or the
accepted phase mapping. No source/LLVM/EXE or owned-slot-intake completion is
claimed by the source package or the Stop test.

Reuse `ordinary_new_admission/selected.rs::emit_root_home_exit_payload` for
first-Fault/best-effort suffixes. Its current origins and validators force every
binding through ordinary-New `local_commits` and `InvokeOperation::HomeRelease`.
The existing root progress owner must instead consume a finite ordinary-object
end / intrinsic-Map end operation, with the same operation retained for graph
validation. No parallel Map root cleanup ledger. The physical cleanup checkpoint
below removes the single-origin projection restriction for recorded HomeRelease
graphs; Map origins and their failure suffixes still need explicit coverage.

Install and old cleanup are separate fallible operations: install Fault means
uncommitted; install Normal supplies committed state and detached-old/no-old
result; detached-old end Fault keeps the commit. FieldSet/ArrayElementWrite Unit
results cannot encode this. Map allocation and Map builtin end need their own
physical operations; no fabricated CanonicalObjectId or generic Call hides them.
`NewFaultContinuationV1` is source-checked direct-local New only, so a Map Fault
needs its own exact source scope/target relation from the control owner.

The final handoff must retain checked Map source/end evidence; Map is not an
unverified NoBirth case. Whole-root Map forwarding also needs an outgoing Home
and cleanup-Fault contract, not the current scalar return-value slot. These are
included cutover blockers, not permission to open unsupported Home returns.

**Source/install validation checkpoint**

Commands: `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib`
with filters `normal_callable_semantic_package` (126 pass),
`resolved_control_flow` (33 pass), and
`normal_default_root_catalog_lifecycle_tests` (11 pass, six baseline failures).
The added actual-root Map test reaches CatalogInstall without catalog/body
installation. Source-flow tests cover transfer/replacement successors, empty and
implicit-exit Maps, unavailable-prefix/entry retention, alias/reuse/fresh/nested
refusal, consumed observation refusal and direct prepare package retention.

The same root filter at isolated parent `3661a9a907` gives 10 pass and the same
six failures with identical deterministic freeze/assertion signatures. These
are known baseline debt, not a Map acceptance waiver. Prefix for all names:
`mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::`.

| Test suffix | Parent/current failure |
| --- | --- |
| `actual_string_helpers_general_result_row_reaches_its_first_loop_carrier` | `raw-compat/runtime-box-fate-retired/static` |
| `source_bound_static_result_owner_reaches_the_raw_terminal` | `raw-compat/runtime-box-fate-retired/static` |
| `source_backed_package_failure_is_terminal_before_builder_effects` | actual RootExpansion, expected CallableSemanticSeal |
| `artifact_validation_rejects_terminal_add_operand_drift` | expected add-binding-drift diagnostic absent |
| `artifact_validation_rejects_exact_read_drift_and_birth_reentry` | `ordinary-new/local-commit/root-cleanup-graph/finished-node` |
| `parser_scan_package_passes_callable_source_handoff_without_fallback` | `static-result-ingress/no-exact-static-target` |

Local logs: `/tmp/hakorune-map-home-{package,completion,root}-tests.log` and
`/tmp/hakorune-map-home-parent-root-tests.log`. Comparison used names plus
freeze/assertion signatures, not unordered AST/config debug serialization.
No fixture refresh, baseline repair or C execution is included in this slice.
Pointer/canonical-corridor guards and diff checks pass; touched Rust max737.

**Accepted physical outcome and ordered implementation**

The [detached outcome contract](../design/collection-literal-construction-ssot.md#detached-install-outcome-physical-contract)
now fixes caller-owned opaque storage, four live states plus disposal,
commit-before-old-end, operation-derived projection and immediate matching end.
Independent MIR and runtime audits confirmed the existing one-result projection
can support this contract; current C handle classification cannot. Descriptor
V1 is fixed-size and must be revised with its readers, never padded silently.

Order within the existing Map series:
1. Native remove/clear lock-held teardown is removed at the existing Map owner;
   focused Map tests8 pass, including reentry/once-only Drop/capacity retention.
2. Close the checked intrinsic Map identity/allocation/lifetime contract below,
   then use the common storage owner through Native-only public and non-NyashBox
   checked facades. Preserve one table per Map and ordered end. Root imports no
   kernel FaultFrame or registry; no Owned promotion of a native-visible Map.
3. Implement target descriptor and checked install/outcome/end primitives,
   then connect existing lifecycle Invoke, projection and root cleanup consumers.
   Tests must exercise real storage/end; schema-only acceptance is insufficient.
4. Close remaining candidate/Script/observer obligations, execute selected
   source OBJ/EXE, then delete matching raw set/MapLiteralEntryWrite and common
   install Stop. Do not claim production caller-zero before that switch.

**Implemented native Map teardown dependency**

Change: `MapBox::remove_key_str` and `clear_entries` detach under their existing
write lock and drop detached native values after unlocking. BoxShape: no new
source acceptance, ownership issuer, runtime ABI or owned-value variant.
Contract: preserve key law, remove Bool, clear's empty-state visibility and table
capacity. Reentrant child Drop sees the committed removal/clear and runs once.
Callers: existing Map delete/clear and kernel substrate routes. Delete-set:
these two lock-held native Drop edges; no new wrapper or second teardown owner.
Done: existing Map focused tests plus real reentry probes for remove and clear,
missing-key/empty-clear behavior, unchanged capacity and one-time teardown;
existing pointer/corridor guards and `src/boxes/README.md` receipt.
Stop: any need for source widening, owned intake or different failure policy.
The ordered physical execution work above remains unfinished after this repair;
this dependency does not replace the full Map cutover finish line.

Validation: `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib map_box`
passes8; log `/tmp/hakorune-map-native-teardown-tests.log`. The two new reentry
probes use the real storage lock and verify committed removal/empty state,
missing/empty operations, capacity and one-time Drop. Pointer/corridor/diff
checks pass; touched Rust max611. Runtime reference and native boxes README
are synchronized. No C/OBJ/EXE or owned-intake evidence is claimed.

**Implemented native GC observation to metrics**

Change: replace GC raw Map access/skip and module-root empty-on-error with
owner-projected native children and typed failure through actual RcDiagnostic
and kernel metrics. The [finite observer contract](../design/collection-literal-construction-ssot.md#native-map-diagnostic-observer-transition)
owns the exact boundary inventory and last-completed-result policy.
Contract: no owned variant, clone-to-share conversion, graph-wide snapshot
claim or semantic receipt. Same native success behavior; one finite last result
replaces separate counters, with no callback under its mutex. Incomplete replaces
old Complete; actual JSON/text sink identifies nonComplete and emits no numeric
trial counts for it. Existing trigger/attempt/duration policy stays.
Done: real native Map graph projection and controller success/failure transition;
poisoned Map and module snapshot produce explicit errors; prior Complete cannot
survive failed observation; kernel output tests distinguish null from numeric0.
Focused root GC/Map and kernel entry tests/check, pointer/corridor guards,
native boxes/kernel README and runtime reference. Sources stay below800.
Stop: any need for owned publication, source family expansion, native clone
semantic change or a new GC collection policy. The subsequent JSON transition
below removes get_data; public read/clone/owned-end migration remains open.

Validation: jobs4/locked/quick root lib filters `gc_controller`7,
`modules_registry`1, `map_box`8 pass (`--test-threads=1` for env-sensitive
controller tests); kernel `-p nyash_kernel --lib entry::gc_metrics_tests`1 passes.
Logs `/tmp/hakorune-map-gc-{gc_controller,modules_registry,map_box}.log` and
`/tmp/hakorune-map-gc-kernel-entry.log`. Pointer/corridor/diff checks pass;
touched Rust max626. Native boxes/kernel README and runtime GC reference updated.
The actual RcDiagnostic root snapshot sees a registered native Map, then a real
poisoned Map replaces its previous successful result. A nested native Map test
separates clone from share. Module-root evidence is local-registry real poison
plus controller result replacement, not a production-global poisoning test.
Kernel tests execute the formatter used by real JSON/text output, not an EXE.

**Implemented dependency: native JSON Result terminal**

Change: existing JSONBox::set returns Result; Map owner lends native entries,
conversion propagates nested errors, and get_data is removed after JSON/GC tests
use private owner poisoning. Delete the non-object Error String arm.
Contract: [native JSON Result](../design/collection-literal-construction-ssot.md#native-json-observation-result-terminal)
defines the finite errors and explicit Rust API signature change. Borrowed
children are not cloned; input disposal precedes destination locking/commit.
No source dispatch, owned variant or replacement panic wrapper.
Done: focused JSON observation, Map and GC tests; actual set Err retains the
destination for nested Map failure, distinguishes invalid destination, and
reentrant top-input Drop sees an unlocked destination. Pointer/corridor/diff
checks, native boxes README and runtime reference. No broad JSON filter with
known unrelated baseline debt is required by this native boundary.
Stop: need for new source dispatch, owned projection or altered native
Array/fallback/clone semantics. External Rust client compatibility is not proven.
Validation: `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib`
with filters `boxes::json::observation_tests`5, `map_box`8 and `gc_controller`7
passes, serial `--test-threads=1`. Logs `/tmp/hakorune-map-json-{json,map,gc}.log`.
Nested source failure retains the destination; source failure precedes poisoned
destination refusal. Drop observes the destination unlocked and not yet committed
on success and non-object refusal. No Rust get_data callers remain in src/crates.
Pointer/corridor/diff checks pass; touched Rust max634. Native boxes README and
runtime reference describe the Result boundary and explicit Rust API change.

**Accepted premise reset: checked intrinsic Map publication**

Decision: the [native/checked facade boundary](../design/collection-literal-construction-ssot.md#map-publication-premise-reset-native-facade-and-checked-intrinsic-facade)
replaces Owned admission into public NyashBox MapBox. An independent read-only
worker confirmed infallible Clone/clone_box/clone_arc cannot preserve copied-table
semantics for non-Clone Owned entries. Native host Result/status capabilities do
not solve this. A retained native alias is the decisive counterexample.

Source authority + canonical issuer: existing exact Map source flow and lifecycle
law issue allocation/transfer/end meaning; common storage remains the sole payload
owner. The physical decision below fixes checked placement; source-to-ABI
reference publication remains to be implemented.
Non-authority: native host handles, legacy bare-i64 reads, VM error capability,
public Clone and helper-only Result tests cannot admit Owned values.
Fail-fast boundary: keep common MapLifecycleConsumerMissing and runtime escape
Stops; no checked facade may enter NyashBox or native host publication.
Runtime dependency: common Map storage, the checked non-NyashBox lifecycle
facade and real SafeMutex indexed-residence end implement the
[accepted physical lifetime](../design/collection-literal-construction-ssot.md#checked-intrinsic-map-physical-lifetime-decision).
Non-claims: Owned source intake, ABI/LLVM execution, Dynamic escape, OBJ/EXE and
complete Map retirement remain unproven. Existing common install Stop stays.

**Accepted checked identity and runtime implementation boundary**

Change: one actual common storage implementation, Native-only public facade,
checked non-Clone facade; caller-owned opaque Map region with Normal-only physical
reference. Runtime owns layout and lifetime transitions; source Map flow owns
acquisition/transfer/end meaning. The descriptor and C connection are subsequent
steps of this same series, not permission to expose a host handle meanwhile.
Contract: native key/clone/iteration semantics retained. Checked install validates
and reserves before mutation, returns candidate unchanged on failure, commits
before detaching old, and ends outside locks. Terminal end marks Ending before
callbacks, ends reverse live-install order and retains first Fault while trying
the suffix. SafeMutex exact indexed payload remains the real child residence;
TLS and unknown profiles reject before admission. No second payload table.
Done: actual native Map regression tests plus checked storage/residence tests
cover real indexed invalidation once, duplicate replacement, precommit retention,
old-end failure after commit, reverse C/B cleanup, Ending re-entry refusal and
native publication exclusion. Focused root/kernel gates, existing pointer and
corridor guards, owner README/runtime reference; every source below800.
Stop: needing a new source family, dynamic carrier, broad NyashBox clone change,
implicit share, second registry or a fabricated physical-to-semantic authority.
Runtime local evidence does not retire the source install Stop.

Independent source/MIR/C audit confirmed the placement mapping and corrected two
scope assumptions: unused aliases can be Complete and acquire no new residence;
Map followed by New requires Map end in that New's prior-home Fault suffix.
Mixed-origin cleanup and retained optimized-graph validation are mandatory in the
consumer step, not root-terminal-only follow-ups. Current C's universal HANDLE/
i64 projection and intrinsic birth_h paths must change at cutover.

Deletion target: selected intrinsic Map allocation/publication through
nyash.map.birth_h, old set/MapLiteralEntryWrite and common install Stop at actual
source switch. Native compatibility export remains Native-only. Acceptance of
the full series covers empty/multiple Maps/unused aliases/Map-before-New across
source to EXE, plus all allocation/install/end Fault paths. This supersedes
repeated get/values Result-only slices as a proposed Owned-intake solution.
The preceding Decision came from read-only audit. The runtime implementation
checkpoint below is separate evidence; no source/ABI/C activation follows from it.

**Runtime storage/residence checkpoint**

Implemented common MapTable, Native-only native facade, checked non-NyashBox
facade and private SafeMutex indexed residence. Table/end-buffer reservation is
precommit; no payload is duplicated. Root tests cover same-candidate refusal,
order exhaustion, missing versus unavailable projection, reverse end, old-end
failure, first/suppressed reports, real lock poison and end-time reentry refusal.
Trait ambiguity checks compile only while checked Map has neither Clone nor
NyashBox and the detached outcome has no Clone. No source fixture was widened.

Validation: serial `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib`
filters `map_box`13, `boxes::json::observation_tests`5, `gc_controller`7 pass;
`-p nyash_kernel --lib checked_map_residence`2 passes. All use `--test-threads=1`.
Logs `/tmp/hakorune-checked-map-{map,json,gc,kernel}.log`. Kernel tests use actual
SafeMutex indexed storage: profile/type refusal and failed precommit retain the
payload; successful displacement/end invalidates it once. Stale old-end failure
does not undo new ownership. Root callback tests prove reverse order/reentry;
those callbacks are physical probes, not source Home or user-fini execution.
Pointer/corridor/diff checks pass; touched sources max698. Boxes/kernel README
and runtime reference updated. No new ABI export, C build or EXE claim.

Next: implement opaque Map, prepared-key and detached-outcome lifetime wrappers
plus the target descriptor/session/required-symbol revision, using the accepted
placement states. Disposal must reject live Map or Ready outcome; no mutable
Map/outcome/FaultFrame borrow spans callbacks. Only then connect the existing
operation-derived C projection and mixed-origin cleanup, and execute source
cutover with selected old-edge deletion. The source install Stop remains.

**Opaque ABI key/order audit**

Read-only worker confirmed a missing native temporary in the wire inventory:
MapKeyDomain preparation allocates/owns text, and old C length-aware String handles
do not transfer that residence. The [opaque ABI contract](../../../../reference/runtime/runtime-data-dispatch.md#checked-map-opaque-abi-contract)
now fixes Map/key/outcome states, key-before-child order, install consumption,
status/Fault publication, callback borrows and disposal. No new semantic issuer
or source shape is needed. Map/outcome-only layout revision is superseded by one
revision carrying all three opaque layouts; no ABI implementation landed yet.

Concrete implementation order within the same row: runtime placement wrappers
and real key/child/install/end cases; target descriptor issuer+archive decoder+
session C row+validator+required symbols/driver; actual C placement/projection and
mixed-origin cleanup; selected source switch and exclusive old-edge deletion.
The worker inventory includes launcher descriptor-symbol assertion, which must
move with section/export renaming. Key cleanup executes on child Fault, install
Fault and success. Preflight InvalidContract preserves Ready; every post-move
returned result leaves Consumed. Raw init freshness stays an unsafe compiler
lifetime obligation, not a runtime read of uninitialized storage. No stale native
String-key handle/cache path may serve the checked consumer.

**Opaque runtime ABI and target descriptor checkpoint**

Implemented eleven Map/key/outcome C exports with native preparation before child,
preflight state preservation, consumed key/outcome states and disposal rejection.
Short frame validation ends before callbacks; returned end diagnostics merge into
the existing primary/suppressed frame afterward. Indexed preparation now preserves
storage-unavailable versus missing identity instead of flattening the two errors.
Root canonical numeric key classification no longer formats an intermediate String.

The one descriptor is V2, 236 bytes; old section/export/decoder names are retired.
Rust archive/session/symbol checks, C row/revision/geometry and launcher assertion
are updated together. The actual x86_64 archive reports Map104/align8, key40/align8,
outcome40/align8, each contract1. These are observed target facts, not new constants
for consumers. The C emitter receives the extended session but does not yet emit
Map operations or allocate unused regions. Source install Stop remains unchanged.

Validation: serial jobs4/locked/quick kernel `fault::checked_map`5 and
`map_identity_tests`1; root `runtime_abi_descriptor`8 (two ignored),
`map_key_domain`3 and `map_box`13 pass. Build `-p nyash_kernel` succeeds; explicitly
running the ignored actual-archive descriptor test adds1 pass and checks all11
symbols. Logs `/tmp/hakorune-map-opaque-{abi,poison,descriptor,keys,map,archive,actual-descriptor}.log`.
The poison case uses an isolated real store. C build passes; actual C runtime
harness passes native-key/indexed install/end/disposal; production C session owner
accepts the actual extracted descriptor and rejects10 malformed sessions.
C test files are `checked_map_runtime_abi_test.c` and `lifecycle_opaque_session_test.c`.
The latter stops at target admission, not object emission. Launcher assertion was
updated, but the full launcher suite was not rerun. Max touched source703;
pointer/corridor/diff guards pass. No source Map OBJ/EXE or R7 completion claim.

Next execution: connect source Map flow to operation-derived MIR/key/Map/outcome
physical projection and every mixed-origin cleanup suffix, then selected C Map
emission using the existing session layouts. Preserve unused aliases without a
new Home; later New Fault must release earlier Maps. Validate both retained and
optimized graphs before source switch and selected old-edge/install-Stop deletion.

**Mixed cleanup connection audit (2026-09-09)**

Decision: existing source flow is sufficient for the direct-local cohort; bind
ordinary/Map Home origins once in the current Completion ledger. No new source
issuer, semantic receipt, object-ID stand-in or parallel Map cleanup ledger.
Boundary: Completion -> claim preflight -> emission progress -> original/finished
validation -> compiled-entry cleanup coordinates; includes root and later-New/
Map-failure suffixes, excludes Script/general Map escape and native compatibility.

Read-only worker identified required consumers previously implicit in the queue:
ordinary_new_coseal::try_take rejects prior Maps before selected.rs; prepare/begin
New emission flattens prior origins to object/value pairs; root_home is ordinary
only; emission_validation pins later-New failure bindings to original blocks;
root_validation coverage omits Map operations; finalized_root_observation treats
an empty ordinary local_commits table as NoSelectedLocalNew (empty Map root must
participate); compiled_entry_contract::issue_cleanup_coordinates omits Map cleanup.
Use the same finite bound-origin lookup at claim, preparation and root exit.
Finishing must account for every recorded suffix, not just terminal cleanup.

Physical consumer audit: Invoke currently has no Map/key/outcome operation;
normal projection is a boolean value/no-value distinction in Rust. C parser
restricts projections to new_box/array_new; admission calls all such values HANDLE
and emission loads i64. Add operation-derived Map/key/outcome roles with exact
Invoke correspondence at these existing owners. Do not transport opaque pointers
through generic handles or ptrtoint. Key prepare precedes child; cancellation and
disposal are physical temporaries, with no synthetic source Home. Map install
Normal must project the detached result directly into its matching end; reject
escape/missing/double end in both original and retained graph.

Ordered connection: remove the existing single-Home root projection restriction;
then finite bound-origin/progress with Map operations and all failure suffixes;
then physical transport/C emission and source switch with exclusive old edges
removed. Preserve prior Map on later-New Fault, unused alias Home count, empty
Map root observation, precommit versus committed cleanup and first-Fault order.
Schema/parser tests alone cannot close this cutover series.

Physical cleanup checkpoint: root capture/finishing now consumes every nonempty
recorded Home origin list. It requires 2N-1 release Invokes from the existing
emitter, instead of a universal one-release limit; DAG, external ingress, prefix,
full Invoke tuple and sole-predecessor contraction checks remain. Source origin
correspondence still belongs to root_home, not graph reachability. Five focused
root_cleanup_graph tests pass, including two-Home clean/pending-Fault contraction
and Fault-to-clean drift rejection; existing source-backed Pair diagnostic
finishing test also passes. Commands use jobs4/locked/quick/--lib and serial tests;
logs /tmp/hakorune-multi-home-{graph,finishing}.log. Initial two test callsites
missed the new count argument; corrected before successful rerun. Corridor/diff
guards pass; touched source max326. No Map-specific end vocabulary, later-New
suffix finishing or multi-Home source-to-EXE completion is claimed.

Installed-Home owner consolidation: claim preflight, prior-New preparation and
root exit now use one installed_home lookup over the existing physical ledger.
Missing/duplicate bindings reject; source prior/terminal order remains unchanged.
The ledger emits the end operation once, and both Builder cleanup paths consume
it directly. Root origin validation compares the same operation instead of
reconstructing object/value fields. No Map value or emitted progress is invented;
Map remains stopped until its actual operation/progress consumer is connected.

An existing synthetic order test exposed a parent failure at 32dc673170:
`brand_catalog_tests::new_completion_tests::ordinary_new_home_prefix_retains_order_and_requires_prior_installation`
failed root-cleanup-graph/release-count at parent and current. Parent reproduction
used a detached worktree, the same ignored Cargo.lock and serial locked quick
command. Its old three-Home fixture omitted pending-Fault suffixes/entry boundary.
The fixture now matches the existing emitter's 3+2 nodes, keeping ordering and drift
checks. Extra Invoke ingress now rejects at internal-incoming before unowned-site
coverage; an isolated extra ReturnFault still rejects at unowned-site coverage.
This is a test migration missed by the preceding graph change, not permission to
weaken graph validation. Logs: /tmp/hakorune-home-end-{parent,order-fixed}.log.

Consolidation validation: order test1, source-backed two-New completion1,
root_cleanup_graph5 and diagnostic Pair finishing1 all pass after the fixture
repair. Serial CARGO_BUILD_JOBS=4 / --locked --profile quick --lib filters with
--test-threads=1; logs /tmp/hakorune-home-end-{order-fixed,owner,graph,finishing}.log.
Pointer/corridor/diff guards pass; touched source max643. Package README updated.
The parent red above is resolved, not carried forward as a baseline waiver.
Next: actual Map Invoke/key/outcome emission progress and operation-derived
projection at the now-shared ledger/cleanup consumers. Ordinary lookup alone does
not establish a Map physical binding; keep the common source install Stop.

**Map Invoke dependency checkpoint (2026-09-09)**

MIR now carries Map New/PrepareKey/InstallIndexed/EndOutcome/End operations;
Normal result roles derive from the operation (Map/key/outcome, never Handle).
The finite verifier rejects opaque escape, temporary misuse, missing/double end
and inconsistent live-Map joins. It uses iterative traversal for the acyclic
cohort. Key/outcome projections immediately feed install/end: accepted entries
read already-acquired direct locals, so no fallible child evaluation intervenes.
Fresh/nested child cancellation is not admitted by this dependency. Native
post-operation disposal precedes either successor; invalid-contract status traps
rather than selecting source Fault cleanup. MIR reference owns these rules.

Validation: cargo check --locked --profile quick --lib passes; serial jobs4
quick lib filters verification::invoke (12 pass) and physical_program_json
(7 pass). Logs: /tmp/hakorune-map-invoke-{check,tests,transport}.log.
JSON projects the operation vocabulary and referenced object layouts. C parser/
emission, source Map progress/binding and source-to-OBJ/EXE remain unconnected;
no C build or runtime execution is claimed. Common install Stop remains.

Next source entry is RawInvocationChildPortV1::lower_expression, before raw
fallback, using its exact source site and existing callable ledger/state.
Consume MapHomeFlow from the same co-sealed Completion exactly once. Existing
state.read_variable(entry.site().node()) must record each variable consumption;
compare the retained binding/acquisition with installed Home progress. Return
through existing callable-local completion, preserving unused aliases and empty
Map observation. No child AST re-lowering or new semantic receipt. Before this
consumer binds to snapshot APIs, complete the bounded storage task below.

**Map physical binding and local placement Decision (2026-09-09)**

Decision: one local_commits enum and shared Local Copy/ReuseInitializer placement; no second Map registry, manager or proof layer.
Source authority + canonical issuer: existing ResolvedInitializerRelationV1, lexical Variable relations and Completion-owned MapHomeFlow from observe_map.
Non-authority: placement, physical progress, type names and ValueId equality do not issue source membership, transfer, replacement or Map escape.
Fail-fast boundary: declaration/initializer/owner and completed Map result agree before local allocation/registration; reject without Copy retry. Common install Stop remains.
Smallest next slice: actual Map emitter, enum progress and exact Local placement together, including empty Map and unused aliases; then mixed cleanup/C cutover.
Non-claims: read-only design evidence, not execution; source OBJ/EXE, full lifecycle consumption and old-edge retirement remain open.

Boundary: selected callable Local initializer/alias -> existing Local driver ->
shared installed-Home lookup -> original/finished root validation. Includes empty
and direct-prior-ordinary-Home Map, unused aliases, and prior Map on later New
Fault. Excludes Script, fresh/nested entry acquisition, Map escape/read/field/Birth
widening, and compatibility Local behavior. No source syntax or result ABI change.

Retain the existing initializer relations in CallableSemanticLoweringState from
input.function().expression_source().initializers(), checking their declaration
statement/ordinal/binding against existing locals once. These are existing source
rows retained for initializer correspondence, not a new binding authority; locals
continues to own materialized declaration bindings. Never repair either table
from the other after a mismatch. A private exact lookup returns the retained row.
No MapHomeFlow declaration field is needed and no Body/Initializer path slicing
may manufacture one. beginMap compares relation.binding to flow.destination,
relation.initializer_site to flow.site, and the current declaration/owner. The
Map physical row may retain that declaration locator but not keys/outer snapshots.

Use LocalCommit::Ordinary(NewLocalCommitV1) / Map(MapLocalProgress) in the one
existing local_commits table. Map source stays borrowed by exact table key from
root_completion.cleanup.root_flow; reject absent/unavailable/duplicate flows.
Map row owns only emission state/result/recorded physical associations and local
progress. Shared installed_home serves claim preflight, prior-New preparation
and root exit; it returns end operation and physical availability for either
kind. Map Installed is sufficient before Checked. Field-read admission and Birth
handoff remain explicitly Ordinary-only. is_empty, completion and artifact
coverage must account for all Map rows and unconsumed Completion Map demand;
zero ordinary claims cannot erase an empty Map root's obligation.

Local placement uses the callable-specific lower_callable_local_v1 caller:
pass a fallible private placement callback to the existing shared Local driver.
Existing callers use Copy. Evaluate initializers with the existing child port,
then preflight ALL placements before the existing from-values owner allocates
or registers local destinations. That owner retains name/slot registration,
metadata and CompletedLocalBinding issuance. ReuseInitializer takes the same
ValueId with no allocation, Copy or LocalContractWrite; it is not post-emission
Copy deletion. No RecursiveChildLoweringPort/RawStructuredChildScope/package
adapter forwarding is needed. Ordinary distinct Copy checks stay unchanged.

Reuse has only these source-backed cases:
- Map initializer: exact retained initializer relation -> Complete Map flow ->
  destination -> same row's expression-completed result.
- Unused alias: exact declaration -> initializer Variable site -> lexical binding
  -> current Map result. Existing read_variable must have consumed the exact site.
  Alias chains use the same source binding chain, never a value-only shortcut.
  Alias completion updates callable bindings only, not a new Home/progress row.
Exact numeric or typed Array Local contracts cannot take opaque reuse; incompatible
annotation rejects before materialization. Merely matching MapBox text or a
physical ValueId is insufficient. Pending Map/foreign site also rejects.

Physical emission consumes the same source order: Map New, then each PrepareKey,
exact prior-local read, InstallIndexed and EndOutcome. Normal result kinds come
from operations. Key/outcome consumers are immediate in this cohort. Allocation
Fault cleans initial outer Homes; key/install Fault ends Map then precommit outer;
detached-old end Fault ends Map then committed outer. Shared pending-Fault chains
preserve first Fault. C disposal is part of operation completion before either
successor; status2/unknown traps, not status1 cleanup. Do not transport opaque
Map/key/outcome through generic Handle, ptrtoint or public MapBox.

Ordered implementation and acceptance:
1. Same slice: retained initializer relations + actual Map emitter/progress +
   shared Local placement. Verify empty/populated Map initializer==local, opaque
   Copy zero, unused alias/chain with one Map Home and consumed Variable sites;
   reject foreign/same-spelling wrong binding/site, pending result, incompatible
   annotation and arbitrary equal ValueId. Ordinary Local Copy stays. Placement
   failure may follow initializer effects but must not partially register locals.
2. Shared lookup/local batch/original validation: Map Installed-before-Checked,
   later-New cleanup, duplicate install/result drift, unavailable/unconsumed Map,
   and empty-Map-only root. No fake ordinary object/destruction fields for Map.
3. Recorded mixed cleanup finishing and compiled-entry coordinates: extend the
   existing Home end vocabulary to MapEnd, preserving graph/prefix/ingress and
   actual normal-result associations. Share terminal dispatch at this edit while
   retaining draft/finishing stages and failure precedence. No MIR reconstruction
   of source origins or ordinary-only filtering that drops Map obligations.
4. C parser/admission/emission and actual source OBJ/EXE acceptance, then remove
   common install Stop and selected Named MapBox/birth/set edges in the cutover.
   Replaced Map/unused-alias fresh local IDs and Copy are part of this delete-set;
   generic/ordinary compatibility Copy remains. Earlier slices are dependencies,
   never evidence that the series or R7 has closed.

Worker audit verified the Local from-values owner currently allocates/copies
unconditionally; there is no existing identity-preserving hook to activate.
Existing initializer rows close the missing source locator, so no user language
policy decision or new semantic issuer is required before this bounded work.

Map Local/MIR dependency checkpoint (2026-09-09):
actual Map emission now joins the common LocalCommit table, retained exact
initializer relations and shared Copy/ReuseInitializer driver. The same source
input drives empty Map, populated Map, unused alias chain and later New through
the actual callable port; all four cases pass strict MIR and original-root
validation, with result-projection mutation rejected and opaque Copy absent.
New coseal11, emission2, two-New1, prefix-order1, Pair finishing1 and Map source6
pass (23 tests including the four-case dependency test). Commands use jobs4,
locked quick library filters; logs are /tmp/hakorune-map-connect-ordinary.log,
-emission.log, -regression-{0,1,2,3}.log and
/tmp/hakorune-map-physical-dependency.log. Pointer/corridor and diff checks pass;
changed source maximum713. Initial extraction/import and test helper privacy
compile errors were current-change failures, fixed before these successful runs.
Next: placement foreign/pending/annotation/batch rejection matrix, optimized
mixed failure suffixes and compiled-entry coordinates, shared terminal dispatch,
then selected C/source artifacts and old-edge retirement. This checkpoint does
not close the implementation series or lift MapLifecycleConsumerMissing.

Dependency validation entry (read-only worker audit, 2026-09-09):
issue one branded package, select its actual AppMain identity and matching batch
slot, and borrow the body from that slot's same-source input. A cfg(test)-only
helper under the existing callable loan port may run the real source scope,
Local child port, frame selection and ledger completion. It does not construct
an installed package, reparse source, or change prepare_install. This proves
callable-to-MIR dependencies only; installed production and C/OBJ/EXE remain
separate acceptance. Include empty/populated Map, unused alias chain and later
New, with opaque Copy rejection and source/physical drift checks.

Placement rejection checkpoint (2026-09-09):
source foreign-site/wrong-binding preflight rejects without preventing a later
correct begin; duplicate begin, pending equal-value install and unconsumed Map
remain rejected. Shared Local tests reject the second callback or i64/Array<i64>
opaque contract before registering either local. Focused placement2, Map
dependency2 and existing local-contract4 pass with jobs4 locked quick library
filters; logs /tmp/hakorune-map-placement-reject.log,
/tmp/hakorune-map-map_physical_dependency_tests.log and
/tmp/hakorune-map-local_contract_tests.log. No production switch. Initial test
field-path compile error was corrected before the green run.
Next is the mixed finishing Decision below; no further source-family expansion.

Mixed finishing physical Decision (read-only worker audit at 037b3f0ce9):
the existing local-commit owner captures validated original New/Map/root
associations at draft validation, then produces one temporary finished projection
for both validation and artifact coverage. Source meaning remains in Completion;
there is no new semantic issuer. Remove old-block fixed comparisons and artifact
rereads of original checked_bindings together. Do not search the CFG for an equal
instruction as a replacement proof.
The existing root contraction comparison must handle instruction sequences:
Map bindings include frame/projection instructions and multiple rows per block,
so root-only one-terminator/empty-prefix assumptions cannot be applied wholesale.
Only recorded Jump contraction through an originally sole-predecessor deleted
node is permitted; concatenate recorded instruction order and update moved
InvokeNormalResult origins using the same mapping. Preserve Normal/Fault slots,
operands, frame, external ingress, reachability and acyclicity.
Compiled-entry coordinates add Map End and EndOutcome, projected from the
validated finished physical program; New/PrepareKey/Install are not cleanup.
These coordinates do not implement C execution.
Acceptance: actual simplify_cfg on populated Map failure suffixes, later-New
allocation/Birth failure after Map, mixed/multiple/empty Map root; exact artifact
site coverage and optimized cleanup coordinates. Reject swapped successors,
changed frame/operands/order, omitted/duplicate cleanup, external reentry and
arbitrary instruction relocation. Share terminal dispatch without changing the
draft/finishing stages or their failure precedence. Source install Stop remains.

Finishing premise correction (2026-09-09, worker reviewed): the binding-only
new_completion fixture omitted two prefix terminators. Completing those Jump
connections preserves its original diagnostic assertions; the test now passes.
Production finalization already terminates its root before this validation.
Actual optimized Pair additionally removes three unreferenced Const instructions
from the New prefix. The physical comparison permits only omission of original
unrecorded Const with a unique dst definition and zero original uses; finished
uses of that dst reject. Retained instructions remain exact and ordered. No
name-based metadata exception, general pure-instruction filter, liveness fixpoint,
source reissuance or arbitrary rewrite is authorized. This replaces the overly
strict complete-prefix assumption without weakening recorded lifecycle bindings.
The dedicated negative set includes recorded/used/duplicate-dst Const omission,
new use, rewrite, reorder and insertion. Full compiler verification is required;
direct simplify_cfg-only Map evidence cannot close this checkpoint alone.

Mixed finishing checkpoint (2026-09-09): original New/Map/root bindings now share
one local-commit physical boundary and temporary finished mapping. Removed the
separate root projection state/algorithm, original-block artifact rereads and
duplicated draft/finishing dispatch list (quality row9). Source checks and their
diagnostic order remain intact. Map End/EndOutcome are included in the existing
compiled-entry cleanup coordinate issuer; the coordinate-only test does not
claim full Map compiled-entry admission.
Focused tests pass33: physical boundary1, root graph5, Map dependency2 (six source
bodies x Birth/no-Birth x optimize/no-opt =24 cases), cleanup coordinates1,
coseal11, emission2, unavailable field1, completion/order1, per-New actuals1,
physical JSON/full compiler7 and direct field return1. The full compiler group
includes optimized Pair, tagged Bool actuals, Unit rejection and exact annotation
rejection. Commands use jobs4 locked quick library filters, one Cargo at a time;
logs `/tmp/hakorune-map-final-*.log`. The earlier 0-test compiled-entry filter was
discarded and both real test names were run successfully. No selected C changes
or EXE result claim; next is selected C input/emission then source OBJ/EXE and
old-edge cutover. The common Map install Stop remains.

Known baseline debt from the broad `map_` filter: with
`CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib map_ -- --test-threads=1`,
parent 61076eb17b reports 217 passed / 4 failed / 1 ignored and the finishing
worktree reports 218 / 4 / 1. Both runs have the same four assertion failures:
`refresh_module_global_call_routes_accepts_map_handle_child_field_get_string_body`,
`refresh_module_global_call_routes_rejects_arbitrary_unknown_map_return_as_box_type_inspector`,
`map_value_get_missing_key_stays_unknown_after_typed_write`, and
`map_value_get_mixed_value_results_stay_unknown`.
The first two belong to mir::global_call_route_plan::tests; the last two to
tests::mir::mir_corebox_router_unified::map. Actual/expected values match across
parent and current runs. Logs `/tmp/hakorune-map-finished-parent.log` and
`/tmp/hakorune-map-finished-focused.log`; this is no whole-library green claim.

**Bounded callable Map closeout and next deletion (2026-09-09)**

map_home_flow_tests10/10 now includes three ordinary numeric Local relations
beside Map: root numeric value/alias and a foreign callable's same-name Local.
Their BindingRefs remain distinct from the Map destination and prepare_install
accepts without catalog mutation. Same-scope redeclaration remains the resolver's
typed refusal; nested shadowing is not promoted by this test. Log
`/tmp/hakorune-map-binding-boundary.log`. Together with source13/EXE26, source
completion probes12 and the finite route census, this closes the selected
Complete callable AppMain direct-local I64 Map cohort only.

Broader MAP-LITERAL-COMPILER-CUTOVER-I0 remains open: raw/Script/Core issuers,
all selected static hosts, V1 ingress, the six literal edges and natural nested/
mixed-formal acceptance still belong to the collection owner. No R7 completion
or deletion permission for those callers follows from this bounded closeout.
The preflight getter also does not close queue11's physical Local lookup scans.

Additional thinning queue14 is closed: the duplicate record propagation and
sole with_types forwarder are deleted. Raw-root from-values and callable
receipt_v1 retain the same private publication owner and preclaims entry.
Existing local_statement_parity_tests6/6 and local_placement_tests2/2 pass,
including record/typed Array snapshots and failed-batch registration. Logs:
`/tmp/hakorune-local-thinning-parity.log` and
`/tmp/hakorune-local-thinning-placement.log`. No language/ABI contract changed.
Queue11 is also closed: exact declaration lookup plus BindingRef check replaces
the placement scan. Lookup2/2, parity6/6 and source13/EXE26/completion12 pass;
logs `/tmp/hakorune-local-lookup{,-parity,-source}.log`. Alias-origin and
Map initializer-site scans remain separate. Next is the existing static V2
host invocation connection in the collection SSOT, not unrelated tuning.

**Source-object Fault checkpoint (2026-09-09)**

The same actual13 source/26 EXE executions now include source-object probes on
both duplicate-key bodies (NoBirth and empty Birth). Six completion modes each
cover normal, Map allocation Fault, key prepare Fault, install identity Fault,
outcome-end Fault and Map-end Fault. Runtime operations execute before observation;
Fault injections respect their existing postconditions. Expected outer Home ends
are0/2/2/2/1/0. Map/key/outcome disposal is complete before report, and report
precedes frame disposal; normal reports zero times. Install identity failure is
reason101, while injected storage/end failures use100. This is12 source-object
completion observations, not just synthetic physical JSON evidence.

The initial probe link found the archive's strong main in the same linked code;
observation-only --wrap=main resolves that conflict without changing production
entry/archive or allowing duplicate definitions. Final host test passes in67.88s.
Malformed Array<bogus> retains the existing Local diagnostic before installation;
map_home_flow_tests now9/9. Logs `/tmp/hakorune-map-source-{fault,annotation}.log`.
The bounded retirement census below identifies no selected Named retry. Remaining
closeout is distinct-binding/non-Map annotation acceptance and reconciliation of
that finite cohort with the next active retirement row. No all-Map/R7 claim.

**Selected Map retirement census (2026-09-09, read-only at 6a99587934)**

This census covers admitted Complete callable AppMain direct-local Map -> scoped
callable child -> semantic Map emitter -> published physical input -> V4.
Includes source routing, failed-consumer retry and physical publication. Excludes
Script, nested/fresh candidates, compatibility, normalizer-owned other cohorts
and non-selected backends. This is not an all-Map/R7 census.

The exact scoped child and package adapter both return to
RawInvocationChildPortV1::lower_expression. Its callable Map arm directly returns
lower_callable_map_v1 -> selected/map::emit; failure cannot fall into raw dispatch.
The emitter consumes Completion, not entry AST. Published host lifecycle dispatch
returns V4's result directly, without a Named retry. The former selected route
through collection_literals Named MapBox New/birth/set is no longer entered.
No whole Stop-exclusive test remains to delete: annotation refusal replaced the
blanket test, while unavailable prefix/implicit exit tests retain live contracts.

Boundary-outside retained edges: raw_expression_dispatch/mod.rs ->
collection_literals.rs remains the one raw Map caller; normalizer/helpers_value/
lower.rs retains its own Named MapBox sequence. Their owners are respectively
raw expression lowering and normalizer value lowering. They are non-authorities
for this selected lifecycle cohort. Reopen when those source cohorts are selected
for lifecycle migration, or a scoped callable Map caller bypasses the direct arm.
Their raw parity/child-failure tests remain needed and are not exclusive deletions
of this cutover. Stale blanket-Stop/C-unimplemented prose is removed in owner docs.

**Source Map execution checkpoint (2026-09-09)**

The selected source cohort now passes common install through the existing
Completion/root-claim/Local annotation preflight. The blanket selected Map Stop
is replaced; unavailable cohort and annotated Map/alias refusal remain before
catalog/body mutation. Existing initializer storage supplies declaration-key
lookup through a borrowing getter, without a second index. prepare_install
returns the original package plus its existing issue; no repeated classifier.

The actual published host test covers13 sources: box-free empty Map and six
bodies under NoBirth/empty-Birth definitions. Direct EXE and independently linked
OBJ return30 for every case (26 executions), including unused alias chains,
later New, two live Maps and duplicate-key transfer. Empty input carries the
legitimate empty definition/layout product. The first run found a C-only frame
assumption: empty Birth has no borrowed-frame projection when it has no frame
uses. Admission now permits zero or one projection for Birth, while root still
requires one and SSA rejects undefined uses. No source schema or Fault ABI change.
The rerun passes; Pair C execution regression also passes.

Final focused checks: map_home_flow_tests8 and pre-effect root rejection1 pass;
pointer/corridor guards and diff whitespace checks pass. Changed source max753.
Logs `/tmp/hakorune-map-{source-execution,preflight-tests,preflight-root,source-pair-regression}.log`.
Remaining series acceptance: actual source-object Fault/cleanup observations,
finite selected old-edge retirement evidence, malformed annotation and distinct
binding refusal coverage, then full selected cutover closeout. Physical synthetic
Fault tests already pass, but do not substitute them for source-object evidence.
No whole-library or R7 completion claim.

**Source install preflight Decision (2026-09-09, audited at bb74fa8078)**

Decision: retain the common Map Stop until existing-owner source preflight is closed; Map Complete alone is insufficient.
Source authority + canonical issuer: Completion and co-sealed root ordinary claims; existing Local initializer/variable relations and annotation policy.
Non-authority: C consumer green, Map Complete and unrelated helper claims cannot establish readiness for the entire root or Local annotations.
Fail-fast boundary: retain rejection before initializer MIR effects; do not silently move the existing Stop to a later emitter failure.
Smallest next slice: audit and connect the existing Local annotation owner's preflight over retained initializer/alias relations, then select the same-owner ready Map cohort at common install.
Non-claims: no source switch, new receipt, fixture workaround, Unit root support or R7 closure.

Independent read-only audit names the finite install requirements: successful
root Completion plus AppMain identity; nonempty all-Complete Map flows;
successful terminal_homes; existing IntegerLiteral/I64Add/I64Field relation;
and all claims owned by that Completion having successful construction,
home_prefix and argument_rows with PlainI64NoHook destruction. Helper-owner
claims remain under their own admission. The Map candidate compatibility check
only covers transferred entry candidates; unrelated prior/later New readiness
must not be inferred from it. Counterexample to the weaker rule: empty Map
followed by an unavailable ordinary New.

One preflight connection remains unresolved: local_placement currently rejects
numeric/typed-Array annotations on Map initializers or aliases after evaluating
the initializer. Complete Map does not certify that policy. The existing Local
owner must expose/reuse the same policy before emission using retained relations;
do not copy a new annotation classifier into the ledger or issue a parallel
readiness receipt. Audit exact direct/alias callers before implementing this
connection. Acceptance retains unsupported annotation/alias rejection before
catalog/body mutation, unsupported New rejection, and existing valid aliases.
Accepted connection after independent Local-owner audit (2026-09-09): extract
map_local.rs annotation condition into one pure Local-owner function. Preflight
borrows the same admitted root lowering input from the existing batch, checks
its existing initializer relations, and follows exact variable Local bindings
to a Complete Map site/destination. Apply the policy to the original Local or
alias annotation; no names, AST classification, ValueId or saved alias index.
Only walk after Complete flow/terminal/root claim readiness, so this traversal
cannot re-admit rebind or unavailable ownership shapes. Keep numeric-first and
typed-Array parser error precedence, including malformed annotations; do not
broaden the rejected annotation vocabulary. The physical placement consumer
retains value/progress checks and calls the same policy. No speed claim for
this source-only traversal; existing lookup thinning remains queue11.

prepare_install remains the sole precommit boundary. Return the unchanged
package with the existing install issue on refusal, rather than re-running
preflight in the outer caller to infer a reason. Tests cover direct/alias/alias
chain rejection before catalog/body mutation, unsupported unrelated New,
malformed annotation, distinct same-name bindings and unchanged non-Map Locals.
No new semantic receipt or preflight cache is authorized. This closes the design
question and opens the bounded existing-owner preflight implementation, followed
by the audited source OBJ/EXE/old-edge cutover in the same series.

**Selected C Map consumer Decision (2026-09-09, audited at 33c0c10c60)**

C consumer verification checkpoint (2026-09-09; not source cutover): C build,
existing Pair execution suite and standalone physical parser test pass. The
new `published_map_physical_execution_test.py target/quick/libnyash_kernel.a`
uses that archive's actual V2 descriptor: six synthetic physical programs and
a reversed-block-order input and two live Maps with later ordinary allocations
link and return30. Six normal/Fault completion
paths check Map/key/outcome init/dispose counts; status2/unknown trap. Thirteen
malformed inputs preserve the existing object, including missing/double Map end
reaching the indexed admission rejection rather than a schema error. Read-only
worker audit found no concrete lifetime acceptance hole in the inspected index,
flow and emission. Transferred markers are conservative reuse exclusions, not
an inventory of surviving runtime entries. Indexed Unit roots remain unsupported.
Parser definition/dominance scans still precede the consumer index; queue13 is
not closed. NativeArray regression passes: the ignored retained Script host
test runs 39 source cases, returned-allocation Fault probes and malformed-input
checks. Source publication/OBJ/EXE and old-edge retirement remain open. Logs:
`/tmp/hakorune-map-c-{build,pair,execution}.log`.
The first NativeArray host regression stopped before C because the local
`target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a` lacks the V2
descriptor section. This is an observed stale-artifact prerequisite, not a
passing regression or a classified source baseline. After rebuilding that
selected runtime, `retained_script_inputs_reach_native_c_and_reject_physical_mutations`
with `--ignored --test-threads=1` passed (1 test, 209.95s). Logs
`/tmp/hakorune-map-lifecycle-runtime-build.log` and
`/tmp/hakorune-map-native-regression.log`. Pointer/corridor guards pass; changed
C source maximum721 lines. No Rust source or source acceptance changed.

Next source integration audit: common install.rs prepare_install owns the
blanket Map Stop; select only the already-proved Complete cohort and retain
effect-before rejection for unsupported source, prefix and terminal coverage.
Use the existing published host OBJ/EXE test pattern, not the dependency lowerer
as production evidence. A box-free `%{}` with return30 must carry the legitimate
empty definition publication: instance_constructor_semantic issues Some([]),
program_root_lowering transfers it and the collector publishes it. Verify this
through actual source before changing the physical input's definition contract.
Keep the existing Birth/no-Birth, alias, later New, two-Map and duplicate-key
bodies; prove direct EXE/linked OBJ30 and no selected Named MapBox New/birth/set.
Unit source execution remains outside this indexed I64 cutover.

Decision: replace the existing indexed V4 scalar Pair protocol with exact-origin physical lifetime validation and matching emission; no new semantic receipt.
Source authority + canonical issuer: existing Completion/local-commit finalization and CompiledEntryContract; physical program supplies exact operations, values, targets and layouts.
Non-authority: C roles, storage liveness, absent Birth calls and zero-field layouts cannot issue NoBirth, completed Home or destruction permission.
Fail-fast boundary: malformed physical input, role/origin/consumption/join drift and unsupported layout reject before temporary LLVM/object output; runtime InvalidContract/unknown status traps without Fault retry.
Smallest next slice: one selected C consumer cutover, covering parser/index/admission/emission together and actual runtime-linked execution; preserve the common source install Stop until source acceptance.
Non-claims: this is a read-only accepted design; C execution, source OBJ/EXE and R7 are not completed by this Decision or parser-only tests.

Boundary: final physical JSON + existing target session -> indexed V4 C consumer
-> atomic OBJ publication. Includes the five Map operations, ordinary indexed
storage, Birth calls and mixed cleanup needed by the accepted source cohort.
Excludes source classification, native Array widening, legacy/static dispatch,
Map read/escape, fresh/nested child evaluation and runtime API redesign.

Two disjoint worker audits confirmed runtime mapping and C lifetime requirements.
The existing C parser excludes Map operation kinds and site census, constrains
Normal projections to new_box/array_new, requires >=2 indexed functions and
nonempty layouts. V4 then fixes two functions, one two-field layout, Birth target1
and arity2; all Normal results become Handle/load-i64. These are the concrete
replacement targets, not general source limitations. Empty Map requires a root
with no Birth and no referenced object layout; zero-field Page remains valid.
The Rust physical input must not demand object definitions when its referenced
object set is empty; nonempty sets still require exact issued definitions.

Existing hako_lv4_input owns one document-lifetime index for functions/blocks,
SSA producers, object/layout slots and operation-derived result roles. Parser,
admission and emitter consume it; no global cache or second operand graph.
Map/key/outcome Normal results are opaque storage pointers, not i64 out slots.
Ordinary Copy resolves the same indexed allocation origin; opaque Copy/Phi/Call/
return remain forbidden. SSA/dominance and unique Normal landing/projection
checks remain. Shared index construction also addresses queue13's repeated
lookups; separate structure/SSA/ABI checks are not deleted.

Physical state is per allocation origin: indexed storage Absent/Live/Transferred/
Consumed, Map Absent/Live/Ended, key/outcome Absent/Ready/Consumed, plus pending
Fault. New Normal establishes LiveStorage, never source Home. Supplied Birth
calls validate receiver origin/layout, arguments, target and field accesses;
source-required Birth presence stays with Rust finalization. C cannot prove a
deleted source-required Birth from this JSON and must not claim it does.
This follows the existing checked object ABI's published-caller permission
contract; no absent-call heuristic or new NoBirth transport authority is added.

Install Normal transfers its candidate; returned Fault preserves candidate
liveness. Both consume key; only Normal produces Ready outcome. End operations
consume on either returned status and preserve a previously pending Fault.
Cleanup Invokes remain valid on pending-Fault paths; acquisition/install do not.
Exact states agree at joins, the bounded graph is acyclic, and terminals retain
no caller-owned live storage or Ready temporary. Transferred storage is owned
by its Map, whose end remains required. Do not reuse native Array's blanket
Fault-path Invoke rejection for mixed cleanup.

| MIR operation | Existing runtime operation completion | Normal result |
| --- | --- | --- |
| Map New | storage_init -> checked_new; Fault disposes Unissued storage | Map pointer |
| PrepareKey | key_init -> key_prepare_utf8; Fault disposes Empty key | key pointer |
| InstallIndexed | outcome_init -> checked_install_indexed; dispose consumed key on Normal/Fault, Unissued outcome on Fault | outcome pointer |
| EndOutcome | outcome_end -> outcome_dispose on Normal/Fault | none |
| End | checked_end -> storage_dispose on Normal/Fault | none |

Use descriptor V2/session2 size/align and per-storage contract1, not measured
104/40-byte constants. Each acyclic operation initializes fresh nonoverlapping
storage; stack lifetime does not substitute for dispose. All init/dispose statuses
must be0. Checked status0/1 dispatch occurs only after required disposal;
status2/unknown does not assume consumed key or inspect unknown region state.
Key bytes use exact UTF-8 length including embedded NUL, never strlen/host handles.
Indexed candidate is the sole i64 handle; type_id comes from its exact layout.

Ordered implementation within the existing Map series:
1. Replace indexed parser/admission/emission together: operation roles, index,
   lifetime states, arbitrary supplied Birth ordinals/layout slots, zero-Birth/
   zero-layout cases and opaque runtime calls. Remove scalar lv4_flow, fixed
   hako_lifecycle_birth_1/2-field declarations and universal result load together.
   Keep one V4 entry, target session and atomic publication. Split real C owner
   responsibilities before760/800; no parallel Map compiler or pending-only API.
2. Existing physical execution harness plus source-issued Map dependency input:
   empty/populated/duplicate-key/unused-alias/multiple-Home/later-New cases,
   injected Prepare/Install/End faults, disposal counts and invalid-status trap.
   Malformed role/origin/site/layout, double/missing consumption, Fault projection
   and join drift must preserve any preexisting output artifact. Preserve Pair
   and native Array execution; parser green alone is dependency evidence.
3. Connect actual source publication/host OBJ and linked EXE, observe result and
   cleanup, then delete the common Map install Stop and selected old literal
   Named MapBox/birth/set edges atomically. No public source acceptance from
   test-only dependency issuance. This is the existing row's finish line.

Post-cutover Map performance task — ParkedSealed (2026-09-09):
owner is the existing MapTable/checked install and selected backend/runtime,
under perf-owner-first-optimization-ssot.md. Current code confirms a shared
MapTable<V>, distinct native-copy/checked-transfer contracts, and checked install
contains_key followed by reservation and insert. This is a candidate duplicate
lookup, not measured cost or permission to alter failure order.
Reopen only after selected source Map OBJ/EXE cutover and a bounded profile plus
generated-code inspection identifies this path as hot. Compare operations with
the same resolved target, ownership, alias/escape and known key/count information;
literal syntax alone is neither a speed guarantee nor equivalence to named
new MapBox(), whose provider contract remains distinct.
Ordered tasks: measure lookup/key conversion/handle/lock/call costs; choose one
demonstrated duplicate operation for removal; then evaluate inlining/link-time
visibility or capacity/batch specialization only if measured costs justify it.
Preserve allocation/key/child/install/displaced-end/Fault order and precommit
candidate retention; reserve failure cannot be moved across effects by assumption.
Counterexamples include duplicate keys with faulting old-end and effectful child
evaluation. No new syntax/MapManager/semantic receipt, no unmeasured speed claim,
and no implication that static target resolution removes a machine-code call.
Non-authority: syntax spelling, fixed-HEAD review and table sharing do not prove
equivalent semantics, inlining or runtime speed.

Additional thinning queue (verified against 61076eb17b and current code, 2026-09-09):
keep finishing -> selected C -> source OBJ/EXE -> old-edge retirement as the
completion order. These rows do not authorize source-family widening or replace
cutover with optimization work.
Status (2026-09-09, feedback rechecked at HEAD 41912dd3bc plus working tree):
rows11 and14 are closed at 0ae4f899af and 34f8ae8506 respectively, with their focused acceptance.
Rows10 and12 remain queued. Row13 is partially implemented: invocation-owned
function/block/value/layout lookups, memoized physical kinds and one Copy query.
Parser definition scans, dominance walks and unused dominance `seen` remain;
there is no measured speed claim or whole-row13 closure.
The active execution pointer remains CURRENT_STATE.toml; this queue does not
select a new implementation row. Row10 needs multi-ingress cleanup
correspondence proof; row12 belongs to session integration; remaining row13 work
belongs to its validator. Keep broader Map cutover ahead of unrelated tuning.
Deletion targets are repeated work, not independent source/ownership/FFI checks.
Repeated feedback maps here; no sibling cards or new guard family are needed.
Review boundary: the five reported production owners below; includes their
current repeated generation/read/lookup sites, excludes whole-program cost,
runtime speed and completion of the in-progress static V2 changes. No new
build or performance measurement was run for this feedback reconciliation.
Scheduling: finish the selected Map cutover first; take row10 at the next
mixed-cleanup owner edit, row12 at runtime-session integration, and the remaining
row13 at physical-validator work. None requires reopening closed rows11/14.

10. Map emitter cleanup sharing: selected/map.rs separately emits each pre/post
    failure chain; post(i) equals pre(i+1) for the same issued ownership state,
    frame and outward target. For n distinct prior Homes transferred once,
    allocation plus these chains emit n*n+n HomeRelease operations, excluding
    preceding New emission and Map End; each cleanup operation adds three blocks.
    Owner: existing selected emitter and local-commit physical correspondence.
    First share adjacent identical entries, then evaluate shared tails using
    source-issued operations. Preserve disposal, first Fault and all ingress
    states; do not infer ownership from CFG or claim arbitrary-order linearity.
    Acceptance: exact operation/block counts, duplicate-key/failure order,
    multi-predecessor joins and actual simplify_cfg + artifact validation.
    Handle at the same owner's next boundary after current finishing closes.

11. Closed: Exact Local initializer lookup. map_local.rs uses its retained
    declaration-keyed BTreeMap with the exact statement/ordinal locator, then
    verifies BindingRef. The former per-Local values scan is deleted. SourceStmtSiteV1::from_node is only a lookup-key
    projection of the existing statement, never source membership issuance or
    AST-path reconstruction. Keep insertion uniqueness and foreign/ordinal
    refusal; no second index or ledger. Alias-chain lookup is separately
    measured, not implicitly claimed fixed by this one change.
    Acceptance: lookup2/2 (missing locator/binding drift), parity6/6 and real
    source13/EXE26/completion12 pass. No language or ABI contract changed.

12. Runtime archive I/O sharing: LifecycleRuntimeSessionV1::select calls Fault
    descriptor and entry ABI readers, both traversing/extracting every archive
    member. Checked Map symbol validation invokes nm; NativeArray requirements
    invoke it again. Share one session-local member read and symbol inventory,
    while retaining distinct descriptor/entry/symbol validators and malformed/
    missing/duplicate rejection. No process-global cache or RuntimeManager.
    Acceptance: actual archive plus malformed/duplicate/missing inputs, observed
    ar/nm invocation counts and unchanged required-symbol checks. Queue with
    selected runtime/session integration, before treating inspection as cheap.

13. Selected C physical lookup sharing: physical_v2 value_def_block still scans
    rows and dominance revisits blocks. The reported two Copy type queries are
    already replaced by one lv4_input_type query with a retained local kind;
    invocation-owned indexes and memoized kinds also exist. Reuse those owners
    where their validation/lifetime contract permits, rather than adding a
    second index. Retire remaining repeated definition/block scans and the
    unused seen allocation. Preserve separate structure, SSA/dominance and ABI checks,
    duplicate/cycle/range rejection, and disposal on error. No semantic reissuer.
    The dominance seen allocation is confirmed unused except allocation checks
    and frees; remove that independently when this function is touched.
    Acceptance: existing negative physical-input and emitter tests, allocation
    failure cleanup, and measured lookup/scan counts. Align with C consumer
    edits; no new global state or general optimizer project.

14. Closed: Local metadata/forwarder thinning. Removed the second record
    clone/registration after common propagate and the sole with_types forwarder.
    Real raw-root from-values and callable receipt callers still use the same
    private publication owner. Preclaims, completed-local correspondence and
    contract preflight remain. Existing parity6/6 and placement2/2 pass;
    no independent caller of the deleted forwarder remains. Row11 is separate.

**Map quality queue (2026-09-09, measured at 7cc63ab9ea)**

Boundary: current Map physical-frame construction and the shared owners touched
by its MIR opcode cutover; includes derived-analysis reuse, effect consumers and
source budgets, excludes a repository-wide thinning or performance claim.
Runtime opaque ABI validation does not close this compiler queue.

1. Correct the rolling source budget now: remove the absent raw invocation
   transport and replace historical counts with measured live paths. Confirmed
   local.rs774, flow.rs759, instruction.rs744 and raw dispatch754. This repairs
   current documentation, not the outstanding source splits.
2. Flow rewrite-owner split completed: flow.rs414, value_uses.rs237 and
   value_uses_tests.rs116. CFG/PHI decisions stay in flow; only two private
   substitution entrypoints are exposed. Function bodies and three existing
   tests match the parent modulo module wrapping/visibility. All16 simplify_cfg
   tests pass (`CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib
   simplify_cfg -- --test-threads=1`, /tmp/hakorune-flow-owner-split.log).
   Corridor guard follows the new code/test owners and passes; no new guard,
   accepted form or instruction contract. Module README updated.
3. Bound-frame analysis consolidation completed. After Named binding,
   map_frame_projection computes demand once, domain closure/operation selection
   once and original demand once, returning actions plus original demand to the
   frame. Coverage, unresolved and original-Float refusals are preserved. Domain
   selection still receives no provisional external domain map. Prior inspection
   entrypoints are test-only; no cross-graph/global cache or semantic receipt.
   Validation: `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib
   published_backend_view::map_ -- --test-threads=1`:27 pass,1 ignored (actual C
   query requires its dedicated instrumented driver); includes recursive/mixed
   formals, seeded Phi/Copy, missing Named and partial frame/action refusals.
   Log /tmp/hakorune-map-frame-analysis.log; corridor/diff checks pass, touched
   source max355. Compiler README updated. No elapsed-time speedup or source/C
   cutover claim. The previous five demand/two domain/two original computations
   are removed from the production frame path; tests may inspect them separately.
4. Split local.rs's 618-line materialize_local_v1 by existing ordered phases
   before extending it. Preserve cache, forwarding, failure-policy and emission
   order; one materialization owner, no second Facts classification. Done:
   both parent/child below760, focused checked/legacy materialization tests and
   reusable guard; classify any red against the parent before widening scope.
   Instruction/raw-dispatch remain deletion/delegation-only until a bounded
   split creates room. Merely moving the entire large function is insufficient.
5. At the planned source/write cutover, retire raw Map set (MUT) and normalizer
   set (PURE+Io) in favor of the selected write contract (MUT+Io). The normalizer
   mask at helpers_value/lower.rs:603 is inconsistent, but effect.rs:is_pure
   returns false for Io; DCE elimination, CSE and memory-effect consumers retain
   it as effectful. No mutation erasure/reordering was established by this audit.
   Done: selected old edges absent and mutation retained through optimization.
   If the old set path is edited before retirement, correct its physical mask
   with a focused check then; do not introduce a second source effect authority.

6. Map source snapshot BoxShape completed before the new emitter's API intake.
   Owner resolved_semantics/home_map_flow.rs::observe_map formerly retained
   precommit_outer/committed_outer/live_before/live_after per entry: 2n² elements
   plus allocation_fault's n for n distinct transfers. These arrays are deleted.
   One reverse initial Home list now records transfer points; entries retain
   exact displaced/replacement points. outer_after_installs borrows the issued
   prefix; out-of-range rejects. Live projection is test-only. No elapsed-time
   or peak-memory measurement is claimed.
   Former direct snapshot accessor consumers were map_home_flow_tests only; production
   home_new_prefix consumes entry bindings and remaining Normal state, while
   Completion retains the flow and common install checks Map presence.
   Keep the same source issuer: one initial outer order plus exact entry transfer
   and displaced relations. Expose required cleanup through owner-side borrowed
   iteration/projection; backend must not reclassify keys, replacement or Home
   ownership. Replacing Box with Arc alone does not remove changing snapshots.
   Delete all four per-entry arrays and full-list collect/clone construction.
   Acceptance preserves a→A,b→B,a→C (displaced A; final C,B), precommit versus
   committed Fault order, allocation/key/install/displaced-end cases, alias and
   duplicate-binding rejection, prior Maps and later-New cleanup suffixes.
   Fresh/nested child refusal remains; do not claim newly supported child Fault.
   Retained structure is proportional to initial outer+entries. Expanded MIR
   cleanup code size and total compile complexity are separate measurements.
   Validation: serial jobs4 cargo test --locked --profile quick --lib
   map_home_flow_tests -- --test-threads=1:6 pass, including untouched Home/prior
   Map/later-New Fault ordering and out-of-range prefix rejection. Existing alias,
   duplicate, fresh/nested and unavailable-prefix install Stops remain covered.
   Log /tmp/hakorune-map-source-deltas.log; pointer/corridor/diff checks pass,
   touched source max251. Module README/reference updated. No source/C cutover.
7. Static Call invocation state is already task3 in lang/c-abi/README.md; keep
   that owner rather than creating a duplicate task. Reconfirmed Rust transport
   environment save/change/restore and C published-call static rows/count/used.
   The two route forward-link helpers mutate HAKO_AOT_USE_FFI via the common
   set_env_value helper; include them in the existing ingress inventory. Owner:
   capi_transport plus C published-call/route consumers. Acceptance remains
   overlapping distinct rows/options, failure cleanup and unchanged environment,
   followed by deletion of globals/mutation. No race or serialization proof.

8. New ledger progress BoxShape completed before adding Map progress.
   NewEmissionProgress/NewLocalCommitV1 remain the sole physical owner. Worker
   audit confirms Emitted.result and initializer store the same accepted value;
   local is a distinct Copy destination and must remain distinct. Move the
   independent expression/local completion states into the existing progress
   representation; delete duplicate value storage and synchronization checks.
   Preserve Unprepared/Prepared/Emitting/Emitted/expression-complete/Installed/
   Checked transitions and the separate RetainedUnavailable completion path.
   installed_home must accept Installed-before-Checked for later-New cleanup.
   Keep is_complete versus installs distinct, all-local preflight before updates,
   all-emission validation before marking Checked, original/finishing MIR checks
   and argument/reclaim/source correspondence. No new manager or proof wrapper.
   Callers: selected admission -> callable/raw expression completion -> semantic
   observation local batch -> child port complete_new_emissions. Acceptance:
   existing coseal order/duplicate/foreign/value drift tests, later-New completion,
   failed batch leaves every row unchanged, unavailable never becomes Checked.
   This was representational debt; no reachable corrupt state was demonstrated.
   Private progress.rs now owns expression/install/check transitions. Emitted
   retains result once, distinct local lives in Installed/Checked; independent
   initializer/local Options and checked bool are deleted. Unavailable has no
   Checked variant. Original/finishing graph validation remains at its owner.
   Validation: serial jobs4 locked quick lib filters ordinary_new_coseal::tests
   (11 pass), ordinary_new_emission_validation_tests (2), source-backed two-New
   completion (1), prior Home order (1), diagnostic Pair finishing (1). Includes
   Installed-before-Checked lookup, bad-Copy validation leaving progress intact,
   checked graph revalidation, and two-local rejected batch without partial
   installation. Logs /tmp/hakorune-new-progress-{coseal,emission,two-new,order,finishing}.log.
   Initial test placement could not access the private owner; moved it under
   that owner and reran successfully without widening production visibility.
   Pointer/corridor/diff guards pass, source max566. Module README updated;
   ownership reference meaning is unchanged. No new accepted shape/C execution.
9. Terminal validation dispatch BoxShape is closed by the mixed finishing
   checkpoint above: one private validation order serves draft and finishing.
   Unit still precedes cleanup/field checks and scalar checks follow them;
   both stages, source checks and artifact-only coverage remain. The duplicated
   dispatch list is deleted without adding a semantic receipt.

Next selection is owned by CURRENT_STATE.toml and the active workstream.
Callable dependency emission, Local placement, mixed finishing, source host
execution and source-object Fault observations have bounded evidence above.
Source snapshot storage and New ledger progress are closed.
The LocalSSA split is queued before that owner grows; static Call state follows its existing lane
selection and does not block unrelated Map work. Corrected counts and these
registrations are not evidence that outstanding source changes have landed.

**Physical residence prerequisite (source compatibility is not storage compatibility)**

The [Map construction owner](../design/collection-literal-construction-ssot.md#map-owned-canonical-object-residence)
owns the storage Decision. Current checked object allocation returns a negative
indexed-store identity; Map Handle5 accepts a positive host handle and clones
its NyashBox. PlainI64NoHook proves end semantics, not interoperability. Keep
`named-non-host-handle` and boxed-object escape Stops until the actual Map entry
can accept and discharge that exact obligation. The root-defined non-Clone residence/kernel SafeMutex end interface is now
accepted; TLS remains unsupported. The checked facade must close identity,
read/end and native-publication exclusion before production owned-slot intake. Source/Completion and physical consumer work can
proceed toward that gate; source-only accepted rows with a pending physical
consumer do not count as cutover. A no-read source is not an observer exclusion:
`nyash.map.birth_h` registers MapBox in host handles, whose snapshot is traced by
`GcController::run_trial_collection` in `RcDiagnostic` mode. This is diagnostic
reachability, not cycle reclamation, and is inactive in `Off` mode. Public native
access and this trace must be supported or actually proven unreachable before
intake. Non-Clone residence can coexist with an Arc-backed Send+Sync Map wrapper;
the real conflict is value-cloning public operations, not those marker traits.

**Transaction and acceptance**

Precommit failure transfers nothing, but cleanup is origin-specific: an existing
local keeps its obligation until ordinary root cleanup; an acquired fresh child
is cleaned by evaluation; a Live carrier follows its exact producer continuation.
Map allocation responsibility is separate from each child. At install Normal the
prior owner relinquishes once. Replacement installs the new slot before ending
the detached old obligation outside the storage lock. Old cleanup Fault does not
roll back installation or resurrect the old candidate owner.

Required acceptance covers direct-owner transfer and alias refusal; duplicate
use of the same Home (second use rejected even for equal keys); two independent
fresh acquisitions at an equal key (distinct obligations); acquisition-before/
after and install-before/after Fault; stale/foreign candidate refusal; Normal-only
carrier publication; and postcommit cleanup Fault preserving new ownership.
These are target cases, not claims that those programs compile today. Existing
Map deletion returns Bool, not an ownership-returning value.

Ordered closure: accepted Map end law and plain-object compatibility -> source
Normal/transfer flow and root Completion/Recipe cleanup co-seal;
add consuming physical transaction; connect runtime storage/cleanup; switch
source/host and retire the old edges with OBJ/EXE evidence. `InvokeOperation`
currently has no Map install/detached-old result. Array write and FieldSet promise
no mutation on Fault and cannot represent postcommit cleanup Fault. Raw
`collection_literals::build_map_literal_with_port_v1` and Core
`helpers_value/lower.rs` still emit generic set. Runtime
`nyash_map_literal_store_v1`/`MapBox::insert_key_str` return no detached obligation;
a status change alone cannot issue source transfer.

Included open work: candidate capability/availability, implementation of key
residence and Map/root cleanup under the fixed end order, borrow invalidation, Dynamic read
publication, cycles/self-insertion, representation compatibility and runtime
profile/finalizer affinity. Do not bypass these with a scalar-only cutover.
No additional user policy is needed for the one-entry law; actual producer and
consumer mapping must close before implementation is authorized.

**Landed prerequisites / evidence**

- Native JSON observation: Array/Map child clone edges removed; stored-child/clone-zero/independent-result tests pass. Command `CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib json`: current444 pass/4 fail/2 ignored, isolated parent `a3ee23938d`442/4/2; same four failure bodies. Logs `/tmp/hakorune-json-observation-tests.log` and `/tmp/hakorune-json-observation-parent-tests.log` (temporary). Parent used the same locked dependencies and shared target, serial Cargo; temporary worktree removed.
  Baseline debt (no waiver):
  `runner::json_artifact::program_json_v0_loader::tests::typed_program_v0_import_compile_uses_published_program_pipeline`;
  `runner::mir_json_emit::tests::hmi_t0_fixtures::ownership_transport_matches_checked_in_hmi_t0_fixture`;
  `runner::mir_json_emit::tests::hmi_t0_fixtures::scalar_suite_matches_checked_in_hmi_t0_fixture`;
  `stage1::program_json_v0::tests::records_and_metadata::metadata_and_annotations::source_to_program_json_v0_transports_local_type_annotation_metadata`.
- Native Map replacement: displaced Drop runs after unlock; Map6 verifies reentry/commit/once and existing key behavior. Pointer/corridor pass; source max597. No owned-slot or remove/clear completion claim.

- `aa4222541c`: ordered Map keys and exact EntryValue structure, explicit Script call Stop.
- `a3ee41cfeb`: prefix stored state separated from ordinary observation; duplicate classifiers removed.
- `9f190cba9a`: unsupported OtherTrivial-to-integer Add edge removed; package119 and pointer/corridor pass, changed source max628. Both earlier field+Bool baseline failures are closed, with expectations preserved; 12 rejected-expression cases retain no terminal/field rows.

These prove their bounded source responsibilities only. Historical parent
comparisons and implementation briefs are in Git, not additional active steps.


### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0`

Decision: accepted by
`box-lifecycle-cprime-terminal-home-finalization-ssot.md`; implementation stays
parked. This row replaces the former `OWN-SHARE-FINI-CLEANUP-D0` question and
must not retain B′ as a parallel authority.

Fix one responsibility split:

```text
share = explicit independent Home acquisition
cleanup = standalone lexical exit registration
fini { } = non-callable terminal Home hook
close()/shutdown() = ordinary domain methods
```

Seal the local/field/return/take/share/weak/birth matrix, parent-hook-before-
field-release ordering, reverse declaration-order owning-field release,
terminal-child-only hook dispatch, field replacement transaction, exactly-once
Shared winner, cycle non-guarantee, finalizer non-escape/no-suspension rules,
and pre-effect rejection for cross-thread/plugin/FFI families without an exact
contract.

Dependencies:

```text
LANGUAGE-RESULT-EXIT-C-PRIME0-D0
OWN-HOME-TAXONOMY-D0
OWN-COMPOSITE-TRIVIAL-D0
OWN-HOME-REPRESENTATION-D0
OWN-FIELD-CONTAINER-DEST-D0
OWN-HOME-TRANSFER-FAILURE-D0
OWN-HOME-BIRTH-D0
```

Done only when terminal Home release has one proposed DropPlan authority,
direct `obj.fini()` and B′ Dead-with-live-Home are rejected targets, and no
ordinary handle or runtime refcount observation can dispatch the hook.

### `OWN-EXPLICIT-HOME-RELEASE-STMT0-D0`

Decision: accepted semantic target; implementation remains parked.

**Change**: supersede the earlier ordinary-call spelling and select contextual
statement `release root` as the sole explicit early Home-end spelling.
Ordinary/generic `release(value)`, `drop root`, `drop(value)`, direct
`obj.fini()`, and identifier-based compiler magic have no Home authority.

**Contract**: the first profile accepts only a verified whole-root owning local
or owning parameter with exactly one available Home. It consumes that root at
the source point, invalidates dependent handles without hidden re-rooting, and
enters the existing C′ DropPlan only when the release is terminal. It has no
Result channel; `close()`/`shutdown()` remain ordinary domain methods. Trivial
roots reject rather than silently no-op. Generic/composite capability and
field/projection/container release are not claimed.

**Done**: the exact Unique/Shared non-terminal/terminal matrix, cleanup-capture
conflict, Fault chronology, diagnostics, contextual grammar carrier, resolved
root, sealed Home Flow plan, and post-implementation reference receipts are
named without adding production parser, Builder, runtime, or backend callers.

**Stop**: return to design if the implementation needs generic capability
guessing, more than one Home, field move-out, consuming receiver, cleanup/hook
Home consumption, cross-thread affinity, a `drop` alias, or fallback.

### `OWN-HOME-CALLABLE-ABI-D0`

Fix before schema/grammar:

- exact ClosedCallable classifier, including direct body-known recursion/SCC;
- exact ContractBoundary classifier;
- declaration-side parameter/receiver demand authority: body inference may
  verify explicit demand and infer result relation, but never invent a
  consuming plain parameter;
- public/export declaration stability;
- bodyless Shared result contract;
- whether Home ABI participates in callable identity, overload selection, and
  interface compatibility (the first profile should not overload only by Home
  demand unless explicitly decided);
- generic instantiation and unknown behavior;
- result-origin substitution at call sites;
- named stable anchors versus temporary receivers.

The first profile rejects a borrowed result rooted in a temporary such as
`makeTree().getRoot()` unless a separate lifetime-extension rule is sealed.

### `OWN-HOME-TRANSFER-FAILURE-D0`

Fix the exact transfer point relative to:

- left-to-right argument evaluation;
- failure while evaluating a later argument;
- callee entry;
- cleanup on caller/callee failure;
- terminal return and publication.

The source must retain a coherent Home when argument preparation fails before
the call boundary. Select one effect-free preflight/commit rule; do not let
individual lowerers choose when consumption becomes visible.

### `OWN-HOME-BIRTH-D0`

Dependencies: `OWN-FIELD-CONTAINER-DEST-D0` and `OWN-HOME-TRANSFER-FAILURE-D0`.
The 2026-09-05 [failed-construction integration decision](../design/constructor-birth-new-lifecycle-ssot.md#failed-construction-minimal-integration-contract)
owns the phase matrix, native handoff, Fault exit and tasks; constructor syntax is unchanged.
Initialized fields release in reverse declaration order, not history order.
Design is accepted; source/exit products and physical activation remain open.
This dependency may be designed before MirBuilder completion; it does not
activate the whole Home program or waive declaration/transfer prerequisites.

### `OWN-HOME-SURFACE-D0`

Convergence row after every Milestone 2 D0 above. Select the exact HomeV1
source grammar and hard rejects, including:

- declaration-side Home demand spelling;
- plain parameters remain Handle and cannot become consuming from body shape;
- opaque result relation and Shared result spelling;
- expression-side share;
- caller-side transfer omission versus optional lint;
- local reassignment and terminal return rules;
- parked `take` expression/field/receiver forms;
- contextual statement `release root` with one identifier root;
- contextual-keyword disambiguation.

This row converges the remaining candidate spellings. It carries the already
accepted `release root` spelling unchanged and cannot reopen or replace it.

## Milestone 3 — passive compiler products

These rows begin only after `OWN-HOME-SURFACE-D0` and add types/verifiers with
production callers zero.

### `OWN-HOME-RELATION0-S0`

Introduce branded, non-forgeable relation vocabulary for Home roots,
destinations, result origins, and typed rejection reasons.

Implementation task:
`docs/development/current/main/investigations/own-home-relation0-s0-implementation-task-2026-08-09.md`

The bounded S0 module is passive and caller-zero. It issues only a fresh
relation brand, opaque source ordinals, exhaustive demand/result vocabulary,
and typed foreign/duplicate rejection reasons. It does not classify types or
issue `VerifiedHomeAbi`; those remain the next `OWN-HOME-ABI0-S0` boundary.

### `OWN-HOME-ABI0-S0`

Design/implementation boundary:
[`own-home-abi0-s0-design-task-2026-08-09.md`](own-home-abi0-s0-design-task-2026-08-09.md).
The design stop is closed; the bounded implementation task is
[`own-home-abi0-s0-implementation-task-2026-08-09.md`](own-home-abi0-s0-implementation-task-2026-08-09.md).

The design stop fixes one `CallableHomeAbiIssuerV1`, one same-resolver-brand
capability environment, and one non-`Clone` `VerifiedHomeAbiV1` catalog. The
passive relation brand is batch provenance only, never nominal type identity.
The later implementation introduces receiver/parameter demands and result
relation for the explicit I64/Unit cohort. Parameter/receiver demands come
from the resolved declaration; ClosedCallable body analysis may infer result
relation and verify local flow, but cannot invent a consuming demand. Only the
canonical issuer seals the product. The bounded ABI0 implementation is
caller-zero and does not open Query behavior, target, Recipe, or Home Flow.

The next design stop is
[`own-home-query-behavior-d0-design-task-2026-08-09.md`](own-home-query-behavior-d0-design-task-2026-08-09.md),
which must co-seal typed Query behavior without reissuing Home relations.

### `OWN-HOME-BOUNDARY0-S0`

Classify ClosedCallable and ContractBoundary. ContractBoundary includes
export, separate compilation, interface/dynamic call, callback/function value,
plugin/FFI, and unresolved generic cases. Require exact declaration/manifest
ABI and compiled-artifact schema/profile/dependency fingerprints.

No user-maintained lock file becomes semantic authority.

## Milestone 4 — Home Flow and diagnostics

### `OWN-HOME-FLOW0-S0`

Straight-line, caller-zero availability verifier:

```text
Available -> Consumed
Available -> Shared acquisition + Available
Handle -> Home demand = reject
Unknown -> ownership-changing edge = reject
```

Binding SSA supplies identities; Home Flow does not remap values.

Consumption becomes visible only at the transfer point selected by
`OWN-HOME-TRANSFER-FAILURE-D0`; argument evaluation cannot partially consume a
caller Home and then retry another route.

### `OWN-HOME-FLOW-CFG0-S0`

Add joins, branches, and loop backedges:

- reject use after conditional consume;
- report the branch that consumed the Home;
- reject a consumed Home reaching a backedge without replacement;
- permit only separately proven loop-local fresh, consume+break, and
  consume+replenish shapes;
- no hidden PHI owner synthesis.

### `OWN-HOME-ARG-MATRIX0-S0`

Freeze argument/destination behavior:

- handle alias to Home-demand parameter: reject with root fix;
- `share` to handle-only parameter: reject as redundant paid owner;
- fresh Home temporary to handle-only parameter: exact scoped lifetime rule;
- Home to ordinary handle input: no consume;
- Home to Home-demand input: consume exactly once.

### `OWN-HOME-DIAG0-S0`

Golden-test typed diagnostics for branch/backedge availability, boundary ABI,
destination mismatch, redundant share, unknown capability, and result-origin
conflict. Every hint is filtered by an actual capability witness.

## Milestone 5 — grammar carriers, still production-zero

Grammar begins only after Milestones 2–4 are closed.

Within the release/finalization subfamily, the exact order is:

```text
OWN-GRAM-RELEASE0
-> OWN-GRAM-FINI-HOOK0
-> OWN-FINI-HOOK-PLAN0-S0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0
-> OWN-EXPLICIT-HOME-RELEASE0-S0
```

### `OWN-GRAM-HOME-PARAM0`

Candidate contextual form:

```hako
adopt(take node: Node)
```

Land registry, Rust parser, Hako parser, AST/schema, formatter, reject tests,
and one shared grammar guard together. It declares a destination Home demand;
call-site `take` is not part of this row.

### `OWN-GRAM-HOME-RESULT0`

Candidate ContractBoundary form:

```hako
getRoot(): Node from me
```

Resolve exact anchor grammar, multi-return consistency, generic restrictions,
whether ClosedCallable source may omit it, Shared result spelling, and
temporary-root rejection. No multi-anchor result PHI.

### `OWN-GRAM-SHARE0`

Candidate contextual expression:

```hako
adopt(share service)
```

`share(...)` remains an ordinary call. Parser transport is not permission to
materialize a Shared owner.

### `OWN-GRAM-RELEASE0`

Accepted contextual statement target:

```hako
release file
```

Land the registry, Rust parser, Hako parser, dedicated AST/schema carrier,
formatter, exact-source receipt, and positive/negative grammar guard together.
The guard proves `release root` is contextual, not a globally reserved token:
`release(value)`, `obj.release()`, a callable/binding named `release`, and
`Build.release` remain ordinary, while `unbox root` has zero Home production.
The same implementation commit updates exact `docs/reference/**` support and
examples; later FIRST/FINAL closeout rows are audits, not deferred updates.
V1 accepts one identifier root only. `release(value)`, `obj.release()`, and a
binding named `release` remain ordinary source forms; parser acceptance grants
no Home consume authority before `VerifiedExplicitHomeReleasePlanV1`. The same
implementation commit updates EBNF, ownership, lifecycle, and quick-reference
pages, plus the language status index and stage-profile matrix, to the exact
parser-live surface.

### `OWN-GRAM-FINI-HOOK0`

After `LANGUAGE-RESULT-EXIT-C-PRIME0-R0` has retired scope-position `fini`,
land one unambiguous Box-member carrier:

```hako
box Resource {
    fini {
        me.closeBestEffort()
    }
}
```

The same bounded row updates the grammar registry/corpus, Rust parser, Hako
parser, AST/schema, formatter, resolver lifecycle-declaration catalog, and
shared parse witness. The lifecycle declaration is deliberately absent from
the ordinary callable catalog. Resolution produces a typed
`LifecycleHookDeclId`, not a method-name exception; member-call resolution
rejects that declaration kind before dynamic fallback and before Builder
effects. Unknown calls may not retry a runtime method table. Negative
witnesses also reject parameters/result annotations, ordinary `fini() {}` declarations,
alias/delegate/interface exposure, and scope-position `fini`.

### `OWN-FINI-HOOK-PLAN0-S0`

Seal one passive `VerifiedFinalizerHookPlanV1` before lifecycle effects. It
proves the exact hook body, FinalizerLease non-escape, no resurrection/re-entry,
no `return`/`break`/`continue`/`?`/suspension, no `share me`, exact field/native
capability, and one exact hook descriptor for later DropPlan composition. It
does not require or dispatch a TerminalHomeDropPlan. Unknown plugin/FFI/thread
affinity rejects before Builder effects. Builder caller count stays zero in
this row.

### `OWN-TERMINAL-HOME-DROP-PLAN0-S0`

Seal one passive `VerifiedTerminalHomeDropPlanV1` as the sole lifecycle
product. It contains:

```text
verified terminal-transition strategy (StaticUnique or admitted Shared)
optional VerifiedFinalizerHookPlanV1
verified owning/weak field release descriptor and reverse order
native structural-drop capability
weak tombstone/reclaim disposition
first-Fault and suppressed-teardown receipt
```

The plan is not a hook-only product. `I0/U`, `I0/F`, and `I0/S` consume this
same schema and never re-infer hook presence, field order, native capability,
or terminal strategy. Unknown representation, plugin/FFI, or thread affinity
freezes before Builder effects. The schema row is followed by exact
`OWN-TERMINAL-HOME-DROP-PLAN0-S0/U`, `/F`, and `/S` plan receipts after each
profile's facts exist; these are sealed instances of the same product, not
three policy owners. Physical consumer count remains zero in the schema row.

### `OWN-EXPLICIT-HOME-RELEASE0-S0`

After ABI, straight/CFG Flow, diagnostics, terminal DropPlan schema, and
`OWN-GRAM-RELEASE0` are sealed, produce one caller-zero
`VerifiedExplicitHomeReleasePlanV1`. It binds the parsed release carrier, exact
resolved whole-root place, available Home, path-sensitive consume,
dependent-handle invalidation, cleanup-capture exclusion, and C′ terminal
disposition. It never derives authority from the identifier spelling and
publishes no Builder/MIR, runtime, backend, generic, field, or Shared physical
capability.

Each grammar acceptance is one BoxCount row with one fixture, shared grammar
gate, and one commit. Do not mix grammar activation with lowering.

The shared negative fixture set keeps `move`, source `view/owned/shared`,
call-site `take`, `take place_expr`, consuming receiver, and field take
rejected until their own accepted rows exist.

## Milestone 6 — first Unique production slice

### `OWN-HOME-UNIQUE0-P0`

Select one exact Unique Box representation and one closed source shape. Prove:

- no RC/control-cell/registry owner work on the selected route;
- one Home creation, use, transfer, and terminal destruction;
- compile failure leaves the unpublished candidate discarded;
- fresh compiler reuse succeeds;
- unsupported routes fail before Builder effects.

### `OWN-HOME-CLOSED-CALL0-I0`

Activate one direct ClosedCallable call with one Home-demand parameter and/or
one terminal result. The caller consumes only `VerifiedHomeAbi`; body
re-inference and fallback are zero.

Add one destination family per later BoxCount row. Do not widen to fields,
containers, dynamic calls, or generic boundaries in the same commit.

## Milestone 7 — storage destination adoption

### `OWN-HOME-STORAGE0-I0`

Activate destinations one family at a time, each with its own fixture and
physical witness:

1. local Home initialization/terminal destruction;
2. local reassignment or its selected hard reject;
3. one object field store/replacement shape;
4. one array/map/container insertion shape;
5. global/registry storage;
6. weak storage/upgrade in its separate lifecycle row.

The first child cell is named `OWN-HOME-STORAGE0-I0/L`. It activates only
local Home initialization, forwarding, and terminal destruction; completion
of `/L` does not claim field/container/global storage. The first owning-field
cell is `OWN-HOME-STORAGE0-I0/F` and owns only one exact field store/replace
shape.

Do not generalize one field proof into every container. Field move-out and
consuming receiver remain parked unless their own D0 and storage receipts
land.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U`

This cell starts only after all of the following are green:

```text
OWN-HOME-FLOW-CFG0-S0
OWN-HOME-CLOSED-CALL0-I0
OWN-HOME-STORAGE0-I0/L
OWN-TERMINAL-HOME-DROP-PLAN0-S0/U
```

It activates one owning-local terminal release by consuming the sealed
`VerifiedTerminalHomeDropPlanV1` unchanged. The physical witness must prove:

- ordinary handles, `take`, terminal return, and non-terminal Home release
  dispatch no hook;
- declared hook plus terminal Home release dispatches the hook exactly once;
- no declared hook dispatches zero user hooks while structural teardown still
  completes;
- local cleanup runs before the terminal release;
- the FinalizerLease cannot escape or resurrect the object;
- a body/cleanup/hook Fault chronology witness preserves the first Fault,
  suppresses later teardown Faults, and still releases remaining local/native
  payload best effort;
- the Unique route performs no RC, control-cell, or global-finalizer work;
- unsupported storage, Shared, backend, plugin, and FFI routes reject before
  effects without fallback.

### `OWN-EXPLICIT-HOME-RELEASE0-I0/U`

After the Unique local, closed-call, DropPlan `/U`, and passive release plan
are green, activate one exact owning-local `release root` route. The same
implementation commit updates the exact reference pages and examples. Prove
source-point synchronous release, dependent-handle invalidation, no cleanup
capture, terminal hook exactly once, `drop` alias zero, RC/control-cell zero,
and no retry/fallback. Owning parameter, generic, composite, field, projection,
container, Shared, plugin, and FFI cases remain rejected.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F`

After `OWN-HOME-STORAGE0-I0/L`, `OWN-HOME-STORAGE0-I0/F`, and the `/F` terminal
DropPlan product are green, activate one field replacement and parent teardown
profile. It consumes `VerifiedTerminalHomeDropPlanV1` without re-inferring
hook presence, release order, or native capability:

```text
RHS evaluate/preflight
-> commit new field Home
-> release old field Home
-> old hook only if terminal

parent hook
-> verified owning fields in reverse declaration order
-> native payload drop
```

Include partial `birth` rollback: the unpublished outer hook is zero while
already-complete child Home releases may finalize terminal children. Manual
parent-to-child `fini` calls and field-order re-inference are zero.

## Milestone 8 — contract boundaries

### `OWN-HOME-CONTRACT-BOUNDARY0`

Land exact declaration/metadata consumption for, in order:

1. exported separately compiled direct call;
2. interface/dynamic dispatch parity;
3. callback/function value;
4. resolved generic instantiation;
5. plugin/FFI manifest.

Each boundary requires exact ABI match and fail-fast unknown behavior. No
whole-program body inference crosses the boundary.

## Milestone 9 — Shared materialization

### `OWN-HOME-SHARE0-I0`

Only after `OWN-HOME-REPRESENTATION-D0`:

- map one explicit `share` site to one verified physical acquisition;
- preserve source identity and source availability;
- reject weak/trivial/handle/unknown operands;
- prove no implicit owner producer;
- prove cleanup and terminal lifecycle remain separate owners.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S`

After Shared materialization and `OWN-TERMINAL-HOME-DROP-PLAN0-S0/S` is
sealed, activate one exact same-thread Shared terminal winner and weak fence.
It consumes `VerifiedTerminalHomeDropPlanV1` without re-inferring field/native
or hook policy:

- non-last Home release dispatches no hook;
- exactly one zero-owner winner enters Finalizing;
- weak upgrade/new lease fails once Finalizing starts;
- hook, reverse field release, native drop, and weak tombstone publish once;
- cross-thread affinity, cycles, plugin, and FFI remain rejected or explicitly
  outside the admitted profile.

### `OWN-EXPLICIT-HOME-RELEASE0-I0/S`

After same-thread Shared acquisition and the terminal winner are green, extend
the same release plan to one exact Shared Home. Prove non-last release runs no
hook, the last release enters the same DropPlan exactly once, aliases are never
silently re-rooted, and the implementation/reference commit claims no
cross-thread, cycle, generic, composite, field, plugin, or FFI support.

## Milestone 10 — C-speed physical proof

### `OWN-HOME-C-SPEED0-G0`

For a selected exact front:

- collect perf top report before code changes;
- inspect assembly at the hot symbol;
- prove Unique alias/call/return adds no RC, control-cell, handle-registry, or
  Box birth work;
- compare exact-front instructions and whole-program behavior;
- include the admitted Unique `release root` front and prove it adds no
  generic dispatch, runtime lookup, RC, or global finalizer registry;
- include the ordinary scope-end terminal path and prove automatic `fini` plus
  reverse owning-field teardown through a focused assembly/performance witness;
- keep Shared owner accounting as a separate measured profile and never credit
  it as a Unique zero-cost result;
- keep only evidence-backed representation changes.

Grammar completion is not a performance gate.

## Milestone 11 — readiness and retirement

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-R0`

After the admitted Unique/field/Shared profiles and C-speed gate are green,
retire the selected competing authorities:

```text
direct obj.fini() source/callable owner = 0
ordinary fini(...) declaration = 0
B′ Dead-with-live-Home state = 0
manual parent -> child fini cascade = 0
global Box finalizer authority = 0
terminal structural drop bypassing a declared hook = 0
canonical B′ fallback/retry = 0
```

Plugin/FFI routes either migrate in a separate bounded series or reject before
effects; a host Drop route may not silently stand in for verified Home.

### `OWNERSHIP-HOME-PRODUCT-READINESS-D0`

Required before default/profile cutover:

- admitted source units have one Home authority;
- production callable Home ABI consumer count is exact;
- Home Flow covers every admitted CFG shape;
- Unknown/opaque boundaries fail before effects;
- implicit-share producers and profile retries are zero or named blockers;
- old sparse/View target docs are historical only;
- SharedV1 corpus migration and rollback-free cutover are planned.

### `OWNERSHIP-HOME-CUTOVER0-I0-R0`

One whole-unit profile cutover. In the same series, retire selected old
production authority and forbid HomeV1 failure -> SharedV1 retry. Keep legacy
profile support only when explicitly selected by project/source-unit policy.

## Milestone 12 — normative reference closeout

### `OWN-HOME-REFERENCE-CLOSEOUT0-DOC0`

This parent row has two mandatory execution cells. It is a documentation
contract, not a grammar or lowering shortcut:

```text
OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U
-> OWN-EXPLICIT-HOME-RELEASE0-I0/U
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST
-> OWN-HOME-STORAGE0-I0/F
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/F
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F
-> OWN-HOME-SHARE0-I0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/S
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S
-> OWN-EXPLICIT-HOME-RELEASE0-I0/S
-> OWN-HOME-C-SPEED0-G0
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-R0
-> OWNERSHIP-HOME-PRODUCT-READINESS-D0
-> OWNERSHIP-HOME-CUTOVER0-I0-R0
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL
```

`/FIRST` reports only the exact first production slice and must not claim
field/Shared/default-profile support. `/FINAL` reconciles the final cutover and
is the only cell that marks the parent DOC0 complete.

Both cells include the named receipt
`LIFECYCLE-LAST-HOME-FINI-REFERENCE-CLOSEOUT0-DOC0` with
`slice = first | final`; the Home closeout cannot be marked complete without
both lifecycle/reference proofs. They also include
`OWN-EXPLICIT-HOME-RELEASE-REFERENCE-CLOSEOUT0-DOC0` with the same slice.
Every grammar, passive-plan, implementation, and retirement cell updates its
exact live reference/support status and examples in the same commit;
FIRST/FINAL are audits, not permission to leave references stale between
implementation and closeout.

Update the normative and derived reference surfaces from provisional/parked
language to the exact implementation that actually landed:

* `docs/reference/language/ownership.md` — Home/Handle rules, accepted
  `take`/`share`/`release` surface, destination-side Home demand, rejected
  forms, diagnostics, and the exact profile/fallback policy;
* `docs/reference/language/EBNF.md` and its grammar registry — the exact
  parser-live contextual `release root` statement, with contrasting examples
  showing that `release(value)` and `obj.release()` remain ordinary calls;
* `docs/reference/language/README.md`, variables/scope, lifecycle, cleanup,
  and constructor/birth references — ownership, Box-member `fini {}` as a
  terminal hook, direct-`fini` rejection, ordinary `close()` methods,
  canonical `release root`, zero `drop` alias, standalone cleanup, `new`,
  field initializer, `birth`, and
  partial-construction boundaries must point to their separate owners;
* `docs/reference/boxes-system/memory-finalization.md`,
  `docs/reference/boxes-system/README.md`,
  `docs/reference/architecture/rust-to-hako-lifecycle-projection.md`, both
  plugin lifecycle references, and VM plugin integration — replace B′/direct
  fini/manual child cascades with the exact implemented C′ and plugin/FFI
  capability boundary;
* deprecated `docs/reference/plugin-system/plugin-system.md` and any other
  historical Box/plugin page carrying callable `fini()` or Arc-only lifecycle
  wording — retain historical status where appropriate, but remove stale live
  claims from indexes and closeout views;
* `docs/reference/ir/json_v0.md`, ownership/exit MIR references,
  callable/interface/FFI ABI references, and generated support views — exact
  Home ABI/profile metadata, one resolved release plan, no body re-inference
  at a boundary, and no hidden strong-owner producer;
* active language workstream dashboards, examples, migration notes, and
  environment-variable documentation — no stale `move/view/owned/shared`
  target or SharedV1 retry claim remains presented as the live Home surface;
* historical proposals and parked design cards — label them as evidence and
  link the accepted reference page instead of silently rewriting history.

C′ closeout evidence is mandatory after the first C′ production slice and
again after final cutover. The two cells are time-bounded and must not borrow
future-slice evidence.

`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST` requires exactly:

```text
fini hook grammar == Rust parser == Hako parser == lifecycle descriptor = 1
direct obj.fini() accepted examples                                 = 0
Unique local terminal hook dispatch exactly once                     = 1
close/shutdown reserved language syntax                              = 0
release contextual grammar/parser/AST parity                         = 1
release identifier/MIR-name authority                                = 0
resolved whole-root release plan for admitted slice                  = 1
ordinary/generic release wrapper Call as Home authority              = 0
drop root / drop(value) accepted alias                               = 0
generic/composite release support claimed by FIRST                    = 0
first-slice B′ live reference claim                                  = 0
field/Shared/default-profile support claimed by FIRST                 = 0
```

`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL` requires every `/FIRST` item plus:

```text
parent hook before reverse verified-owning-field release             = 1
Shared non-last hook dispatch                                        = 0
Shared terminal hook dispatch exactly once                           = 1
Shared release non-last/terminal parity                              = 1
final cutover/reference parity                                       = 1
B′ live reference claim across all live reference pages              = 0
```

Each closeout must compare the reference grammar with both parser registries,
the resolver/Home Flow verifier, callable ABI metadata, and only the physical
route selected at that slice. A mismatch that reflects real implementation
behavior reopens the owning code row; documentation must not hide it as
historical prose.

Required evidence:

```text
reference grammar == parser-live grammar for this slice             = 1
reference Home ABI == sealed compiler product for this slice        = 1
accepted examples compile on this slice's selected profile          = 1
rejected examples fail before Builder effects                       = 1
old claims within this slice's live reference surface               = 0
implementation-backed reference closeout before final completion    = 1
docs-only premature or future-slice reference claim                 = 0
```

This row does not add `region`, field-take, consuming receivers, multi-anchor
views, generic/dynamic/FFI ownership, or any other parked capability. It also
does not make the `fini` hook a direct-call, physical-free, or transfer
operation.

## Parked follow-ups

- `OWN-HOME-TAKE-EXPR0-D0`: `take place_expr` for lifetime narrowing/local
  renaming, with a real corpus consumer;
- `OWN-HOME-FIELD-TAKE0-D0`: field extraction, empty slot, replacement;
- `OWN-HOME-CONSUMING-RECEIVER0-D0`;
- `OWN-EXPLICIT-HOME-RELEASE-COMPOSITE-ROOT0-D0`: generic/composite whole-root
  support under the same statement after exact Home-bundle classification and
  a real consumer; no `release<T>` wrapper callable;
- `OWN-HOME-MULTI-ANCHOR0-D0` and result PHIs;
- capture, `await`/`yield`, task/channel, and cross-thread flow;
- explicit `region` after a real arena allocation/free substrate exists;
- closed-graph promotion and bulk reclaim;
- unsafe raw ownership lane.

## Guard policy

Do not create one shell script per row. Extend one reusable Home contract guard
only when code/schema lands. It should eventually check:

- one `VerifiedHomeAbi` consumer path;
- no old `move/view` production grammar authority;
- no HomeV1 -> SharedV1 fallback;
- no hidden strong-owner producer outside explicit `share`/boundary witnesses;
- exact production caller counts for selected physicalizers.
Until then, use the current-state pointer guard and document-only checks.
