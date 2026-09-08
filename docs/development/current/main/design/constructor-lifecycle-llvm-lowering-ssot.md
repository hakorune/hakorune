---
Status: Active contract; selected Pair execution and transport/owner retirement verified
Scope: source-owned constructor requirements through final physical input, LLVM object and linked EXE.
Related:
  - constructor-birth-new-lifecycle-ssot.md
  - mirbuilder-final-pipeline-ssot.md
  - ../../../../reference/language/lifecycle.md
---

# Constructor lifecycle LLVM lowering

## Current capsule

- **Current decision:** source owners issue meaning; existing emission/finalization binds it; one completed physical input and explicit runtime session feed V4.
- **Current implementation status:** selected Pair executes through actual OBJ/EXE callers with default source materialization. V2/V3 pending transports, JSON reparse, view-owned handoff/profile and duplicate terminal storage are retired.
- **Next ordered task:** resume canonical/compatibility separation in the existing backend/runtime order; storage wire-tag naming is complete.
- **Production stop line:** constructor Unit remains `physical_abi.rs`'s `root-result-unavailable`. Local actuals, unsupported declarations/uses and unissued result families retain their own pre-artifact boundaries. Script Array Unit is a different admitted cohort.
- **Retirement finish line:** the selected Pair transport/ownership series is closed; broader source coverage, compatibility migration and Call R7 are not complete.

Current selection belongs to `CURRENT_STATE.toml` and the rolling workstream.
The [lifecycle](../../../../reference/language/lifecycle.md) and
[type](../../../../reference/language/types.md) references own language policy.
This document owns durable lowering contracts and the remaining obligations. Git owns
superseded D0/I0 negotiations, temporary build failures and resource logs.
Missing implementation calls for a bounded issuer/consumer design; it does not
justify permanent waiting, guessed defaults or a second MirBuilder.

## Responsibility and lifetime

| Boundary | Sole responsibility | Existing owner |
| --- | --- | --- |
| Source -> Facts/Recipe | Exact object, receiver/formals, expression requirements, stores, Home obligations and exit origins | `instance_constructor_semantic`, `instance_construction`, `ordinary_new_coseal`, terminal/read/Home source owners |
| Recipe -> physical emission | Bind those requirements to emitted values and operations once | `ordinary_new_admission/selected`, `normal_callable_construction_state`, existing root/New completion ledger |
| Finishing -> final view | Verify complete retained bindings against final MIR and borrow them | `ordinary_new_local_commit`, normal pipeline final validation, `published_backend_view/lifecycle` |
| Final view -> LLVM | Project completed physical bindings; backend realizes their representation categories under explicit runtime/target ABI | `published_backend_view/physical_abi`, dedicated lifecycle C consumer |
| LLVM -> object/link | Use the same selected target and matching runtime library | selected host invocation, call-local target/tool session with LLVM layout evidence, EXE/OBJ linker owner |

Extend existing plans with the information they currently discard. Aggregate
handoff co-seals completed products and issues no new source meaning. Do not
build a second source requirement table synchronized with existing plans, or
attach a public receipt to every instruction. Any physical binding storage
belongs to the existing emission lifetime, is completed once, and moves with
the existing root/Birth handoff. Generic module clones cannot recreate it.

## Value representations

Source owners determine semantic classes from the same resolved source loan,
declaration field contracts, exact bindings, and selected operation/result
relations. Birth lane order alone is not a type contract. Preserve the class
of each admitted formal and source value before physical emission; never use
an absent annotation as an i64 default. Existing accepted argument and Birth
body relations must be reconciled together, including unused formals.

The physical owner maps those classes and runtime operation ABIs to physical
lanes. Keep i64 scalar and object handle provenance distinct even when both
use an LLVM i64. A frame reference is a synthetic ABI value, not a source Box.
Unit is an explicit no-result contract; an absent row is not Unit.

Copy propagates an already established representation. PHI records exact
incoming edges and values, then solves equality constraints using established
incoming representations, including cyclic graphs. Reserve identities before
binding backedges; reject contradictory or unseeded cycles at finalization.
This is physical consistency checking, not source type inference. Do not use
`Phi.type_hint` or optional `FunctionMetadata.value_types` as authority.
InvokeNormalResult is bound to its exact operation and usable only on Normal;
Unit operations cannot acquire a result slot.

Validate every formal, definition, use and return in the selected function
inventory after finishing. Missing/duplicate/foreign bindings, extra producers,
representation drift and uncovered siblings reject. Physical compiler
temporaries require a named emitter rule; no catch-all temporary admission.
Metadata may remain an observation for other consumers; deleting or changing
it must not silently change selected lifecycle admission or lowering meaning.

## Source retention and consumption

`issue_ordinary_new_claims_v1` co-seals the same AppMain parser identity,
Completion, Home prefix, exact New arguments and terminal relation under one
resolved-source loan. Identity is not reduced to a batch slot or physical root
key. Physical root lookup follows the root captured by module creation; names,
metadata and another pipeline's main-thunk product cannot restore membership.
Entry receiver/parameter/capture demands keep `EntryDemandMissing`.

A declaration object and an allocation instance are distinct. Birth retains
its exact Box/Birth source, object, owner, receiver binding and canonical target.
Every New retains its own site, checked Normal receiver and ordered actuals;
deduplicating equal Birth definitions cannot deduplicate call actuals. Unequal
relations for one definition, swapped receivers, foreign objects or missing
actual membership reject before transport.

### Selected stores and arguments

The construction plan owns each exact assignment, canonical field, receiver
source use and RHS. Selected stores accept the existing integer literal or
resolved Birth-parameter RHS. Take consumes the receiver source site; emission
consumes the parameter use through exact binding lookup and the existing source
observer. No assignment-target/RHS AST replay remains on this branch.
Missing/duplicate/foreign stores, unavailable construction and emitted
base/value/block/literal drift reject before publication.

The existing New argument walk retains ordered Integer, Bool or Local(binding)
observations with exact owner/New/ordinal/child site. Bool payload comes from
the resolver inventory. The package alone co-seals claim rows; source helpers
never depend on package products. Selected emission consumes rows before raw
child-demand creation and retains `(source row, emitted ValueId)` in the existing
ledger. Finalization checks cardinality, literal definitions and Birth argument
order. Generic raw children and compatibility loops retain their other callers.

Local source validity does not prove executable scalar kind. `Trivial` merges
Integer/Bool and is not initializer-to-return or initializer-to-actual authority.
Local actuals remain physically unavailable; no MIR constant inference, handle
lookup, first-caller specialization or anonymous i64 default may discharge them.

### Formal declarations and physical binding

The constructor semantic issuer retains each `BirthFormalContractV1`: exact
ordinal/binding, declaration class and complete use sites. Declaration and use
are different: storing into an i64 field does not declare an unannotated
parameter i64. Do not borrow the direct-callable catalog from another session.
Compiled-entry retains the complete contract and derives disposition on read.

The selected physical cohort admits retained Unannotated formals with exact
I64FieldStores or NoUse. Integer/Bool actual kinds come from issued source rows;
checked emission supplies their payload values. One unspecialized Birth body
receives kind/payload lanes, with receiver separately typed HANDLE. NoUse still
evaluates and passes its actual. ExactI64, ExactText, unsupported declarations,
uncovered/conflicting uses and Local actuals retain their named physical Stops.
Those Stops are not new source type errors.

At an exact i64 store, valid Integer reaches raw storage; valid Bool records
FieldTypeMismatch reason103 and follows the issued FieldSet Fault edge without
committing the failed slot. Unknown kind or invalid Bool payload is
InvalidContract. Copy retains both tagged lanes; HANDLE never becomes scalar.
The wire vocabulary and revision belong to the
[ABI reference](../../../../reference/abi/nyrt_c_abi_v0.md#selected-lifecycle-physical-program-v2),
not this scheduling document.

### Terminal relation and result boundary

`home_new_prefix` issues one internal `TerminalRelationV1` at the existing
four sites: direct two-field I64Add, bare Unit Return, Integer literal, or one
initialized i64 FieldRead. The same relation travels through Completion and
the ledger. Only not-yet-issued state uses Option; finalized source contains
one nonoptional relation. Finalization derives the existing result projection;
there is no separately stored root-result tag or view synchronization matrix.
Backend category projection folds the three i64 families into I64, retaining
Unit separately. It does not classify a new source expression.

Keep exact owner/Return/value/Add/read sites, same-Completion agreement,
Pending/Reserved/Emitted progression, duplicate reservation/emission checks and
complete physical-read validation. Structural exclusivity replaces only the
cross-family collision checks. Neither a conflicting production terminal nor
a causal link to the historical Unit defect was established by that audit.

Selected bare Return emits `Return(None)` after Home cleanup; it never passes
through `emit_void` into `Return(Some(void))`. Source/final-MIR Unit support is
real, but constructor physical input still rejects Unit before serialization.
Retired V2 ROOT_UNIT validation is not executable evidence. The admitted Script
Array I64/Unit path is owned by the
[collection contract](collection-literal-construction-ssot.md#accepted-runtime-to-c-task-order).

## Source origins and physical diagnostics

Source-owned New sites, constructor assignment/field sites, Home exit obligations
and construction-fault Reclaim origins remain in the existing plans and emission
progress. Cleanup need not have a source statement; its issued exit obligation
is its origin. Allocation coordinates or a bare Birth branch cannot invent it.

Eligible Birth reclaim retains exact New/constructor/owner/object identity.
NoBirth has no such constructed-object reclaim; unavailable/override construction
keeps its admission fence. Selected Birth-fault cleanup consumes the retained
origin once. Missing, duplicate or drifted concrete Reclaim fails final validation.
Home release preserves exact binding/exit and source order.

After those source-to-physical bindings are checked, `PhysicalAbiInput` issues
nonzero u64 **runtime diagnostic ordinals** in deterministic function/block/
instruction order for each checked operation, plus a distinct required root
`process_result_site`. These are physical diagnostics, not source IDs or a
cross-build stable dictionary. C consumes supplied ordinals unchanged. Reject
missing, duplicate, overflowing, colliding or non-operation site rows. Birth
propagation and ObjectFieldGet do not invent checked-operation diagnostics.
Earlier source-order diagnostic-encoding proposals are superseded by this
implemented physical projection; source origins themselves remain source-owned.

## Finishing and final ownership

The existing root/New/Birth validators check complete retained bindings before
and after finishing. Generic module clones cannot reconstruct a handoff.
`FinalizedRootHandoffV1` is owned by the normal finalization invocation; the
published view borrows it and the selected profile. Lifecycle activation/route
mutation and owned lifecycle/Return coordinate Vecs have been removed. Presence
is a boolean physical observation using the shared exact predicate, not admission.
Ordinary Return alone is not lifecycle presence.

Finalization still checks lifecycle-bearing namespaces, Return-only retained
Birth signatures and direct Birth receiver/namespace/definition. Root exemption
and mandatory numeric/type checks retain their scope and order. Admission holds
borrowed functions instead of rebuilding them from stored coordinates. Static,
free-function, Print and ordinary Array rows remain live view projections.

For single-Home cleanup, existing `RootHomeExitProgress` retains the emitted
graph, entry prefix and outside incoming boundary. Only a recorded Jump into a
deleted empty internal sole-predecessor node without edge arguments may contract.
Consume original nodes exactly once; compare full resulting terminators/prefixes
and ingress. Reject cycles, bypass/new ingress, changed releases/frame/edges,
prefix changes or extra instructions. Surviving targets retain their Jump.
A temporary validated projection feeds artifact lifecycle coverage; there is no
second persistent receipt or optimizer-validation skip.

Multiple-Home constructor cleanup retains its existing exact-placement checks;
optimized contraction coverage is not claimed. Script Array's separately
validated multiple-residence behavior does not grant constructor parity.
New/Birth obligations and whole-function artifact lifecycle coverage remain
mandatory in addition to the graph-local check. Checked becomes FinishingChecked
only after all required checks succeed; no mutable passes escape final commit.

## Runtime and target ABI

The selected lifecycle target is explicitly x86_64 GNU/Linux LP64. Runtime
compilation owns the fixed-width Fault descriptor in `.nyash.runtime_abi.v1`;
target Rust layout supplies size/alignment/offsets. The host reads the named
archive section without executing target code, rejecting absent, duplicate,
truncated, unsupported or inconsistent descriptors. Path/package version,
sidecars, host sizeof and pinned-Text capability are not target proof.

One `LifecycleRuntimeSessionV1` binds the selected archive, target and entry
record before serialization or artifact creation. The neutral C session checks
LLVM target-data layout against the descriptor and target-compiled C header,
and retains its machine/data/triple/layout through emission and publication.
The module preamble and explicit llc target use that same session; generic
ambient tool flags and late runtime-directory reselection do not select this lane.
Resources and temporary files are released on every failure.

Two explicit link artifacts share runtime core: `nyash_kernel` retains legacy
entry behavior; `nyash_lifecycle_kernel` owns normalized-status main and its
`.nyash.entry_abi.v1` record. Core rejects combined legacy-entry/lifecycle-core
features. Generic wildcard builds exclude the lifecycle package; its explicit
build uses `target/lifecycle-kernel`. Renaming a legacy archive does not satisfy
entry ABI admission. EXE links the archive returned by its bound invocation;
standalone OBJ must link an ABI-compatible runtime. Artifact selection is not
source-result authority, and legacy runtime entry is not globally retired.

### Entry outcomes and cleanup

Root owns one aligned initialized FaultFrame, body and cleanup; Birth only
borrows its hidden pointer. Root reports a pending Fault after cleanup and
disposes once, including report failure. InvalidContract is not a source Fault
successor and must not touch an untrusted frame; disposal is allowed only after
successful root initialization. Both report failure and InvalidContract use the
existing final status contract. C emits normalized `ny_main() -> i64`; kernel
owns startup/flush and checked OS adaptation, not handle decoding of that result.

Normal=0, Fault=1 and InvalidContract=2 are runtime operation statuses.
Process policy is distinct: Unit0, Integer0..255 unchanged, range/unsupported
result Fault70, final program Fault70. Out-of-range I64 records reason102 and
`{actual_i64, 0}` at the supplied process-result site after Home cleanup.
Bool remains Bool; its specified unsupported-process Fault is not an i64 cast
or source rejection. Selected root Bool execution still needs an exact source
relation and physical consumer. Policy authority remains
[function exit and entry result](../../../../reference/language/function-exit-and-entry-result.md).

## Direct input, V4 and retirement boundary

`PublishedLifecyclePhysicalAbiInputV1` contains `CompiledEntryContractV1`, its
referenced layouts, representations, sites and runtime requirements. Host
`LifecycleInvocationInputV1` owns that completed input and borrows the selected
session. The wrapper derives both C arguments from this bound owner. MIR does
not depend on host modules; no sibling Array/constructor semantic product exists.

JSON Value is built once and serialized once; the intermediate String/reparse
and `program-parse` branch are gone. V4 parses the physical-v2 document once,
validates the same document, admits its finite cohort, emits LLVM text and
invokes explicitly targeted llc-18 for PIC object output. An independent parser
entry remains validation-only. Parser success is never execution admission.

Validate exact keys/revision, functions/definitions/uses, CFG/PHI predecessors
and available edge operands, terminator-only Invoke, exact Normal result origin,
representations, layouts, diagnostics and target/session. Reject unsupported
siblings, HANDLE-as-scalar, unknown tags and unseeded/conflicting values before
artifact exposure. Failures preserve existing output and clean temporary files.
No source-name resolver, generic retry or C-supplied missing default is permitted.

Selected host OBJ and EXE share `compile_published_view_object`, exact input
borrow identity and capability checks. Each retained numeric obligation matches
its exact FieldSet/value/field/layout once; diagnostic strings only check the
canonical declaration's projection. Missing/duplicate/residual or foreign input
rejects; no metadata clearing or `skip_numeric` flag is used.

The selected old generic lifecycle JSON, Rust V2 frames/wrappers, C V2/V3 pending
exports/delegation and exclusive probes were retired at `7c8041c075`. Shared
physical-v2 parser, V4, target/session, runtime descriptors and generic/static
call consumers remain. The sole surviving static C transport lives under
`published_backend_view/c_transport.rs`; its external path mount is removed.
Caller-zero C cleanup removed the unused `hako_physical_values_exist` helper,
not live parser/SSA checks or the whole C implementation.

Boundary for the closed retirement claim: selected host -> Rust transport/
reexports -> C header/translation-unit/exports -> tracked direct/manifest/
aggregate/directory build/test discovery. Excludes historical docs, external
consumers, parked branches, generic compatibility and other source families.
No blanket compiler/runtime legacy-zero or complete view-thinning claim follows.

## Default derive and actual source ingress

Default Equals/ToString are produced once by `macro/default_derive.rs` inside
the existing normal parser open postpass, after cohort/delegate selection and
before initial source co-seal. One captured macro policy reaches parser and
transform; parser reads no macro environment. AST-only compatibility shares the
pure generator through its explicit caller. Static exclusion, explicit-method
precedence and ordered public fields are preserved.

Generated origin binds Box identity, derive kind, placement and exact declaration/
receiver/parameter coverage. Equals has one unannotated other binding; ToString
has explicit empty coverage, never missing coverage or an invented source-member
ordinal. Existing resolver, parameter, Completion and signature owners consume
this origin. Complete method batches publish all methods, called or not.
Original/generated drift and missing/extra/foreign coverage reject before commit.

Final `finish_exact` remains exact and never regenerates defaults. Registered
macros, test tails and unknown changes retain their rejection. Nonempty public-
field derives still need source-owned dynamic field/Text-conversion relations
and stop before raw repair; generic MacroOrImport is not newly admitted.
Pair's plain fields are not public-field declarations. Its unchanged generated
bodies and normal macro settings now reach actual CLI/host execution; the old
macro-disabled-only checkpoint is superseded by `59e6c5c34b` and later host evidence.

## Finite inventory and acceptance boundary

Closed bounded execution: selected Pair source -> retained root/Birth and exact
actuals -> completed physical input -> bound runtime V4 -> OBJ/EXE terminal.
The unchanged Pair returns30. Bool in either Birth argument yields Fault103/70;
the failed slot is not stored, prior stores survive, reclaim precedes report,
and dispose occurs once. Physical probes additionally cover range/report/
InvalidContract, schema/site/session/tool failure and artifact atomicity.
They do not grant new source families.

Latest terminal BoxShape receipt is `a1d86db262`: exact handoff5/control33 and
Pair EXE/linked OBJ/probes pass; source max730, net Rust-32. Array execution is
separately closed at `feaaa5d5e8`/`b61aef93ec` in its own boundary. This prose
reconciliation runs no builds or runtime tests and does not replace those receipts.

The **broader source inventory is still open**: constructor Unit execution,
Bool/local/alias/typed-integer/general-Add result relations, root formals/SSA,
unsupported actual/formal classes, nonempty-public-field derives and required
multiple-Home optimization coverage retain their obligations. The broader
inventory retains these CutoverBlockerOpen obligations; none is silently
resolved by Pair success or moved to another backend.

## Ordered tasks

1. Resume the [rolling backend/runtime order](../workstreams/mirbuilder-inplace-replacement-current.md#backendruntime-feedback-and-task-order-2026-09-06):
   canonical/compatibility -> compile-call state/
   options -> runtime-hook single storage -> runtime dependency reduction order.
   Published Global-required-row and Extern Stop are already landed; do not
   restart them from an old checkpoint. A demonstrated selected dependency
   failure reopens its owner immediately.
2. Broader source-result/actual families require their own existing issuer and
   physical mapping: Bool, then local/alias, typed integer and general Add after
   operand relations exist. No new family is authorized by this document cleanup.

### Post-cutover physical backend investigation

`C-LLVM-COMPILE-SESSION-INPROCESS-D0` remains parked. Current V4 emits textual
LLVM and invokes tools; target-layout observation does not imply in-process
code generation. Choose an embedded LLVM distribution or the retained tool
owner only with target/toolchain/link evidence, a finite invocation inventory,
object/diagnostic/temp-cleanup equivalence and explicit failure behavior.
Delete command builders only after a selected equivalent successor exists.
The separate Rust `ll_tool_driver.rs` seam is not retired by implication.
No runtime-speed or concurrent-compile claim is made.

### Feedback reconciliation queue (2026-09-07)

This stable entry now records dispositions rather than repeated task cards:

| Finding | Current owner / disposition |
| --- | --- |
| Unit incorrectly labeled RootI64 | Physical root role carries I64/Unit. Constructor Unit still rejects; retained Script Unit executes. |
| Physical input bypasses compiled entry | Input contains the contract; bound invocation owns input/session together. |
| Three competing transports / reparse | Selected V4 physical-v2 only; V2/V3 and generic lifecycle companion retired. Shared static-call transport remains live. |
| View owns handoff/admission/profile | Finalization owns products/profile/admission; view borrows. Generic observation rows remain. |
| External path mounts / unused C helper / semantic README diary | Closed by the existing placement/C/README rows; no repeated cleanup task. |
| Selected-normal Outside raw source lowering | Still outside the bounded constructor cutover; existing Facts/Recipe owner must replace one finite family and its raw edge. Reopen if it bypasses a selected obligation. |
| Global/options/hooks/crate dependency | Existing backend/runtime queue, not constructor reimplementation. |
| LLVM tool subprocesses | Parked investigation above. |

### Feedback reconciliation follow-ups (2026-09-08)

1. **V4 feature repair** closed at `27a4e1ed78`: existing plugins cfg/stub pair
   removes the unguarded loader/libloading reference. Parent `efbc22b14b` versus
   repaired code, identical locked no-default-features check:13 errors ->11;
   only V4 E0425/E0433 removed. Remaining nonplugins baseline belongs to
   `plugin_loader_unified.rs` config/libraries/load_plugin_direct/ingest_box_specs,
   resolve_method_id and method_returns_result, plus
   `runtime/semantics.rs::PluginBoxV2::instance_id`. No whole-nonplugins green.
2. **Terminal relation BoxShape** closed at `a1d86db262`; source-to-finalization
   representation and preserved progress checks are specified above. No new
   public receipt, source classifier or accepted family.
3. **Physical storage wire tag BoxShape** is implemented: Rust physical ABI
   and the shared C header/parser/admission name I64 storage1 with
   `HAKO_LLVMC_LIFECYCLE_STORAGE_I64`. Three anonymous production literals are
   retired; wire value1, unknown-tag rejection and layout checks remain.
   No Bool/handle/storage redesign; current verification lives in the rolling card.

## Classified baseline evidence

At parent `b61aef93ec` and terminal closeout `a1d86db262`, identical lock/quick
package filter has116pass/2fail and pipeline21pass/2fail. Exact current receipt
and reopen owners remain in the rolling card; no whole-package green claim.
The two source-classification tests are:

- `brand_catalog_tests::normal_home_completion_observes_suffix_and_does_not_reuse_last_new_prefix`: mixed field+Bool cleanup unexpectedly available.
- `ordinary_new_coseal::field_reads::tests::terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression`: mixed field+Bool retains1read, expected0.

The pipeline tests are `published_consumer_runs_once_and_propagates_failure_without_retry`
and `published_consumer_does_not_consume_explicit_compatibility`; the latter
reports `artifact-root-completion-unavailable`. Source classifier/read inventory
and normal-default callable/compatibility finalization own the respective debts.
No expectation was weakened and no ignored test was introduced for this cleanup.

Older broad lifecycle evidence at parent `910b92d99b` was114pass/8fail versus
115pass/8fail after one added test, same lock/settings/cwd and serial
`lifecycle --include-ignored --test-threads=1`. The eight historical failures
below were not rerun by this documentation task. They remain classified evidence,
not claims that the latest whole suite has exactly eight failures:

- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::artifact_validation_rejects_exact_read_drift_and_birth_reentry`
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::artifact_validation_rejects_terminal_add_operand_drift`
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::parser_scan_package_passes_callable_source_handoff_without_fallback`
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::source_backed_package_failure_is_terminal_before_builder_effects`
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::source_bound_static_result_owner_reaches_the_raw_terminal`
- `mir::builder::program_root_work_plan::tests::selected_nonplain_script_retains_constructor_source_for_full_runtime_lifecycle`
- `runtime::weak_handles::tests::test_weak_handle_lifecycle`


The earlier missing relative runtime archive in a temporary checkout was a
setup failure, excluded after same-cwd replay. Earlier obsolete synthetic
multi-Home/field-read records were repaired in the cleanup-contraction series;
they do not license weakening current source, finishing or runtime checks.
