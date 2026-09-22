---
Status: fast__B2_typed_verifier_split
Task: MIR-CALL-PARSER-RECURSIVE-STRING-RESULT-AUTHORITY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B2-I0 (same card; source relation before D)
Implementation permission: behavior-preserving typed V2 operation-verifier split only; preserve D WIP and source admission
---

# Parser String result authority: corrected decision and task queue

## Current Capsule

- **Current decision:** B2 Array retention/discharge is implemented; D exposed a source GenericLoop carrier synchronization gap. The scoped carrier/origin design and ordered tasks below replace the induction-only proposal.
- **Current implementation status:** B1 catalog 92, publisher 5, Loop route 20, source Bool Call-to-Branch 1, and merged dependency/inventory 2 tests pass. Complete parser acceptance is still open.
- **Next ordered task:** split the 764-line typed V2 verifier, then conditional NamedArray class/common Core co-seal. ConstText logical vocabulary is verified; D/branch/runtime evidence remains open.
- **Production stop line:** no ArrayPush activation from manifest metadata alone; no Bool/String runtime ABI completion claim from MIR evidence.
- **Retirement finish line:** source statement caller switch and removal of its selected name-dependent edge in the same bounded series; shared legacy retirement remains T5/R0.

## Accepted S1 six-line brief

```text
Decision: accept S1 String normal-result propagation through the existing source-to-MIR publication chain.
Source authority + canonical issuer: VerifiedSameModuleCallableResultCatalogV1 -> VerifiedStaticCallResultPublicationOwnerV1 -> PreparedStaticCallResultPublicationV1.
Non-authority: method names, MIR inference, Compatibility probes, new recursion solver, or physical i64 interpreted as semantic String.
Fail-fast boundary: exact source coverage before handoff; MIR emission failure produces no publication; unsupported runtime ABI remains a backend rejection.
Smallest next slice: ExactString in existing facts/dispositions/call rows and sole destination publisher, preserving RHS evaluation and stable call coverage.
Non-claims: no runtime success/termination/purity, String ABI activation, merged-parser completion, production cutover or physical legacy deletion from S1 alone.
```

## Premise correction and evidence boundary

User supplied a Pro audit based on `3038bfb1`, reporting unpushed commit
`1a67ee4532b33d05b6a7eef5bf4ec825c9851163`. That object/patch is not available
in this checkout. Its reported guard/mutation results are external evidence,
not locally executed receipts or an imported commit.

Local readback confirms `src/mir/source_core_receiver/README.md` explicitly
excludes success, termination and purity from `ExactStringOnSuccess`.
`callable_result_representation/README.md` likewise defines a representation-only
proof. The former requirement for a recursive measure/termination proof was
incorrect and is superseded. The physical contract is now split at its actual boundary: S1 publishes MIR
representation; the later executable row must co-seal runtime ABI and ownership.

| Entry | Current boundary | Consequence |
| --- | --- | --- |
| StringBox direct probe / VM import | Compatibility, no source publication owner; `legacy-fallback-retired` | Dependency evidence only; changing parser selection does not establish source-backed admission. |
| Canonical merged parser | `TargetOnly/RecursiveDependency` for earlier dependencies | Loop/source handoff exists; selected `parse/2 -> starts_with/3` is `ExactI64`, required ordinals `[]`. |

Prior local tests recorded in I3 prove the selected tuple and focused consumer,
not completion of the full merged parser. No compiler probe was rerun here.

## Finite scope and existing owners

This census covers: source declarations of `ParserStringUtilsBox.i2s/1` and
`StringHelpers.int_to_str/1` -> existing result catalog -> publication demand ->
physical Call/result publication. Includes their source call sites, String
literal/left-String Add, substring, local and loop-merge result propagation as
required by the unchanged bodies. Excludes general recursive type inference,
unanchored mutual-cycle inference, VM repair, all-String/backend support, and Map
OwnedText T3 promotion. Inventory rows identify source witnesses, not name-based
acceptance policy; exact target/site/catalog brands remain authoritative.

| Responsibility | Existing owner / decision needed |
| --- | --- |
| Normal-return String fact | `src/mir/source_core_receiver/`; reuse exact source proof, preserve right operand evaluation. |
| Result propagation and call coverage | `src/mir/callable_result_representation/` (`expression_proof`, `call_proof`, `function_proof`, `solver`, `disposition`); separate result dependency from nested-call traversal. |
| Exact one-shot handoff | existing `VerifiedStaticCallResultPublicationOwnerV1`; retain source identity, duplicate/residual rejection. |
| Physical result commit | `src/mir/builder/calls/static_result_publication.rs` and `static_result_publication_physical_bridge.rs`; S1 selects the MIR destination/type mapping below; executable return/lifetime remains D2/I2. |

## Accepted mapping: source -> MIR, not runtime completion

Decision (2026-09-22): reuse the existing monotone solver and general-result
publication. Add one exact String normal-result alternative to the existing
fact/outcome/disposition/representation family; no parallel solver, registry,
semantic receipt family, or helper-name special case is introduced.

| Layer | Accepted mapping / owner |
| --- | --- |
| Source membership | Same branded declarations, target catalog and canonical SourceExprSiteV1. Helpers are witnesses, never name-based selection rules. |
| Result proof | Existing expression/function proof and worklist issue `ExactString` when every normal value return agrees. No termination/purity claim. |
| Portable handoff | Existing general call row, demand and one-shot publication owner carry String without AST, ValueId or a physical ABI guess. |
| Physical Call | `static_result_publication_physical_bridge` preserves ordered argument descent, arity checks and the existing generic unified Call terminal. |
| Result publication | `PreparedStaticCallResultPublicationV1` maps exact String to `MirType::String` at the emitted destination; ordinary and external destinations use the same mapping. |
| Runtime | Separate admission/exit/ownership owner. No new retain/release, pointer, tag, OwnedText carrier or C ABI is minted by S1. |

`CompletedUnifiedValueCallEmissionV1` is constructed after
`builder.emit_instruction` in `calls/unified_emitter/physical_terminal.rs`.
It proves compiler emission, not execution of the callee. A lowering error
propagates without result publication and disposes the failed compilation
through the existing caller. At runtime, ordinary evaluation/Fault behavior
must remain intact; S1 cannot convert a faulting/diverging RHS into a value.

The existing general publisher is extended in place. The I64-only activation
and Loop source requirement owners keep their present admission boundaries;
do not broaden `has_exact_i64_result` just because the catalog can describe
String. A newly encountered unsupported consumer is reported at its exact
boundary, not silently projected to I64 or counted as parser completion.

## Finite proof transfer decisions

| Source/result shape | S1 decision |
| --- | --- |
| String literal | ExactString. |
| Add with proved String left operand | Visit both children first; result is String on normal return, independently of RHS result dependency. Keep RHS call sites and all errors. |
| Subtract/multiply/divide/modulo/unary minus | Numeric-only result proof; never pass a String fact through unary minus. |
| Exact generated String Core row | Known String receiver plus generated result-kind/arity row supplies String; Bool/NoValue/Dynamic stay distinct. |
| Local/assignment | Existing environment transports String; receiver evidence is a derived projection. Pending local receivers remain pending, never an early permanent rejection. |
| Branch/loop merge and returns | String with String merges; mixed String/I64/Box or unknown incoming path does not become exact. Keep loop entry, backedge, break and continue coverage. |
| Same-module String call | Same branded target/call row carries ExactString. Result has no I64-result ordinal requirement; all physical arguments still undergo ordinary validation/evaluation. |
| Unproved direct/mutual cycle | Existing Pending -> RecursiveDependency remains; no SCC guesses or synthetic base result. |
| Missing return, unsupported syntax/annotation | Existing named unavailable/reject rules remain. |

`call_row::result_representation` currently assumes every CoreStringMethod row
is I64. S1 must derive its projection from the already-validated generated
result kind when String rows are added; it must not relabel substring as I64.
The final stable solver pass must retain the recursive call row after the
normal-result proof closes. A known enclosing String never licenses dropping
unknown/unsupported nested targets from package coverage.

## Runtime contract boundary and followup

The selected C non-expanded call branch
`lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc::emit_published_i64_call`
currently emits `call i64` and `set_type(T_I64)`; that is not a String ownership
contract. `FinalizedRootResultAbiV1` in
`normal_callable_semantic_package/ordinary_new_local_commit.rs` has no general
String return alternative. These are concrete followup boundaries, not proof
that all runtime String support is absent.

Existing kernel `handle_abi_borrowed_owned_conformance` tests the reusable rule:
borrowed arguments remain borrowed; escaping a borrowed handle as an owned
return requires an independently releasable handle, and the caller releases
that owned return. Those primitives do not prove either helper's source exit
co-seal. The runtime followup must choose the existing selected Text/handle
carrier from its semantic exit owner, bind caller/callee return agreement,
and reject unsupported representations before artifact execution. Do not
route a handle through integer arithmetic or infer ownership from MirType.

Before executable activation, the same runtime row must name creation/escape,
callee cleanup, caller release, and Fault cleanup for every selected return.
It must verify zero/positive/negative returned content, survival across callee
cleanup, release exactly once, and no published value on Fault. If a selected
backend gap is exposed, taskify that exact owner; neither VM fallback nor
whole OwnedText/serializer work is a prerequisite for S1.

## Ordered executable tasks

| Order | Task / owner | Completion and retirement boundary |
| --- | --- | --- |
| G0 — closed | Existing ingress guard recovery | Four ingress states, both rejecting consumers and split transport paths checked; local PASS plus 13/13 rejected mutations, bash syntax and diff checks green. No compiler semantics change. |
| S1 — closed, source-to-MIR only | MIR-CALL-PARSER-STRING-RESULT-S1; existing catalog + publisher | Implement the mapping above as one coherent result-family change; natural helper source yields String, final call rows are present, generic Call destination publishes String once. Remove superseded String-as-KnownNonI64 classification only for proved shapes. This is no shared legacy deletion credit. |
| A1 — dependency observed, acceptance open | Canonical source-to-MIR recheck | Pin current binary, use real merged parser, observe whether RecursiveDependency is removed, take real parse/2 -> starts_with/3 once and finish empty. Classify any new terminal; no fixture shrinking. |
| D2/I2 | Selected runtime String call/return contract and implementation | Co-seal semantic exit, carrier, caller/callee ABI, ownership and Fault in existing package/C owners; add executable content/lifetime/Fault evidence. Never use S1 as ABI authority. |
| T5 | Existing I3 selected caller cutover | After required acceptance, switch a remaining actual caller, or prove it already uses the selected owner; distinguish existing handoff from completed execution. |
| R0 | Existing selected legacy retirement | Enumerate exact remaining caller and retained shared users, prove selected edge zero, delete exclusive code/tests/guards and add re-entry protection. |

S1's source witnesses are the two unchanged helper declarations and their
recursive Return/Add call sites. The downstream caller is the already-recorded
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` source tuple.
R0 must resolve the exact old caller function/branch from that production
switch; its physical delete set is not yet proven here. It is an explicit
required output of T5, not a fabricated deletion promise or prerequisite to
implement result proof. Shared GenericLoop/LegacyCallV0 and retained I64 owners
cannot be deleted by this row.

## S1 execution brief

**Change:** add one exact normal String result across the existing solver,
call-row transport and sole publisher. Replace the bounded old String
classification atomically; preserve unproved-cycle rejection.

**Contract:** same canonical source/site/target identity, complete nested-call
coverage, full argument evaluation, unchanged effects, one-shot handoff and
residual checks. Generic Call emission and destination representation are the
physical scope; runtime ABI/ownership activation is separate.

**Done:** both natural helper bodies prove String; literal/Add/substring/local/
loop positive rows and mixed/unary-minus/unknown receiver/direct-cycle negatives;
recursive RHS row survives final seal; duplicate/foreign/residual and both
destination publisher tests; lowering failure leaves no published type. Retain
existing I64/nominal Box guards and update the owner README/reference. Run
focused quick-profile tests from one Cargo process (up to 4 jobs), reusing the
same binary; no platform CI needed for this compiler boundary.

**Stop:** a missing source-to-result mapping, dropped nested row, defaulted
Core kind, new source authority, or competing publisher returns to this card's
specific decision. A downstream runtime capability gap does not invalidate
proved MIR representation, and it does not authorize runtime activation.

## Evidence and checkpoint

Read-only worker and local source audit agreed on this mapping at the design
checkpoint. The resumed goal now implements S1; verification receipts follow
actual focused runs. Runtime acceptance and cutover remain separate.

At `3038bfb1e6`, the ingress guard returned exit 1 because of deleted `Absent`
and pre-split paths; G0 retains that known baseline debt. Pro's unpushed patch
and reported mutation checks remain external evidence, not imported local work.
Windows lifecycle stays user-deferred and warning cleanup stays paused at I147.

G0 local receipt (2026-09-22): stale guard recovered from baseline `2f066b149b`;
normal copied-tree check passes and 13 mutations reject. Local commit `5cdf6b2e9d`
is not yet pushed. S1 remains active with the integration blocker below.


## S1 integration decision and next bounded task (2026-09-22)

Read-only worker review and local owner readback agree on the following
extension. This closes the design question, not the failing implementation.

```text
Decision: project unconditional ExactString at an exact source target inside the existing publication owner when no general call-result row exists.
Source authority + canonical issuer: same branded declaration/target/result catalogs -> VerifiedStaticCallResultPublicationOwnerV1 -> existing handoff.
Non-authority: caller-result success, fabricated general rows, method names, AST rescan, or runtime ownership inferred from String.
Fail-fast boundary: reject foreign catalogs, missing caller/site, non-static target, non-ExactString result and duplicate/general-row issuance.
Smallest next slice: internal handoff constructor and owner branch after general-row selection, before the retained I64 projection.
Non-claims: no nominal-Box widening, nested-target suppression, runtime ABI admission, parser completion or legacy retirement.
```

The current owner already supports exact I64 projection without a general
caller row. Apply that same responsibility to unconditional String results:
validate target/result brands, canonical caller membership, exact source site,
static target namespace, callee `ExactString`, and absence of a general row.
Return the existing handoff with `declarations.brand().identity()`; introduce
no additional semantic receipt. Required I64 ordinals are `[]` because this
result proof is unconditional, not because missing evidence is defaulted.
Argument evaluation and nested target rejection remain independently required.

Keep the complete `targets.rows()` inventory, `insert_selected`, one-shot take
and `finish_empty` unchanged. A proved outer String result must not suppress
an unproved nested call's TargetOnly row. Do not convert ExactString back to
Unavailable to make preflight pass. Shared I64 and nominal Box contracts stay
outside this change.

Completion checklist for this same S1 row:

1. Implement the validated constructor in `static_call_result_publication.rs`
   and connect it in `static_call_result_publication_owner.rs`.
2. Test a caller with no general row and an exact String callee, plus foreign
   catalogs, missing source site/caller, wrong result/namespace, general-row
   precedence, repeated take and residual finish. Include an unproved nested
   target to demonstrate that its rejection remains intact.
3. Rerun the focused catalog/publisher/ingress tests and the two exact merged
   tests below with a nonzero executed count. Preserve the actual merged
   source; classify any later terminal before changing expected outcomes.
4. Update module README/reference with this projection boundary and record
   the tested commit. Only then close S1 and proceed to A1. D2/I2 -> T5 -> R0
   remain required in the ordered queue above; no extra platform CI is needed
   for this source-to-MIR repair.

### Superseded S1 working-tree evidence

Pre-fix failures at `5cdf6b2e9d` and subsequent projection receipts are retained
in Git at `b75332b524`. The landed S1/B1 receipts below supersede that historical
queue; none was full parser/runtime acceptance.

## B1 design: exact Bool result through composite source publication

Accepted mapping after read-only worker review and local consumer readback:

```text
Decision: extend existing composite source publication with exact Bool normal results; preserve direct I64-only contracts.
Source authority + canonical issuer: same source result catalog and publication owner -> source item relation -> existing composite source port.
Non-authority: helper names, physical i64 values, generic truthiness, or new Bool execution/receipt owner.
Fail-fast boundary: unsupported result/operator, mixed returns, foreign or residual source rows, and String/Box at composite scalar admission.
Smallest next slice: ExactBool -> existing handoff -> composite relation/state -> ExactSourceMethodCallV1(Bool) -> existing Call publisher/If consumer.
Non-claims: no String runtime ownership, Bool C return ABI acceptance, direct singleton Loop widening, parser completion or old-edge removal from proof alone.
```

B1 begins after S1 closeout. The finite witness is unchanged
`StringHelpers.skip_ws/2 -> is_space/1` at
`[Body(4), LoopBody(0), IfCondition]`. The implementation accepts the existing
composite exact scalar publication family (I64/Bool), not a name/site special
case. This is source-to-MIR BoxCount; no unrelated refactor is bundled.

| Existing owner | B1 change |
| --- | --- |
| Result expression/function proof, solver, disposition and call rows | ExactBool from Bool literals, explicit six comparisons and And/Or; Not must traverse its operand. Preserve both source child inventories and mixed-return rejection. Do not classify bitwise/shift as Bool. |
| Static handoff and publisher | Same branded exact target projection when no general row exists; ExactBool maps to MirType::Bool, including external destination. |
| `normal_callable_loop_source_route::into_item_dispositions` | Composite relation accepts exact scalar I64/Bool; direct I64 routes retain their requirement. |
| `normal_callable_loop_source_facts/composite_physical.rs` | Validate that same composite scalar contract before consumption. |
| `normal_callable_semantic_lowering_state` install/take | Validate I64/Bool and project Integer/Bool to the existing ExactSourceMethodCallV1. Keep current effects and one-shot/residual rules. |
| `helpers_value/lower.rs` -> `lowerer/emission_port.rs` | Reuse result_type destination, GlobalCall, SourcePublication and external-destination publisher; no separate Bool emitter. |
| Existing condition lowering ports | Keep branch and short-circuit ownership. Proof traversal of both children does not force runtime evaluation of the RHS. |

Required B1 evidence: original source witness reaches an actual Bool Call
destination used by the existing If condition; `starts_with` retains Integer
and empty ordinal requirements; Bool/String and Bool/I64 returns reject;
arithmetic/bitwise/shift are not Bool proofs; direct I64-only contracts reject
Bool; composite rejects String/Box; foreign/duplicate/residual and mismatched
external destination reject. Preserve source coverage and short-circuit
behavior. Recheck the real merged parser and classify its next terminal.

B1 shares this card. Its result-family extension replaces the bounded old
KnownNonI64 classification for proved Bool shapes and composite I64-only
admission, but does not earn shared legacy-retirement credit. T5/R0 remain
explicit production switch/deletion work after required acceptance.


### Landed S1/B1 checkpoint history

S1 catalog90/publisher5/ingress4/merged2 passed; this was MIR dependency evidence,
not full parser/runtime acceptance. B1's interim reds were superseded by its
closeout below. Source publication extraction `2f2f683122` preserved behavior.
Detailed interim experiments and receipts remain in Git at `231ef5f9e0`.

### B1 integration / next-owner audit (2026-09-22)

The diagnostic rebuilt binary identifies the next full-parser boundary as
`StringHelpers.split_lines/1`, loop `[Body(5)]`, method item
`[Body(5), LoopBody(1), IfThen(0)]`: `arr.push(s.substring(last, i))`.
Catalog 91, publisher 5 (now String and Bool destination/mismatch cases), and
Loop route 20 (including composite scalar/direct I64 rejection) tests pass.
The first focused-source command matched zero tests; it is discarded as
receipt. Its corrected exact name is
`mir::builder::normal_default_root_catalog_lifecycle::loop_scope_tests::source_bool_call_feeds_existing_composite_condition`.
It completes source-backed lowering and confirms the is_space Call destination
has Bool type, but its initial direct-ValueId Branch assertion failed. The
revised test follows only Copy/Compare dependencies into Branch; result pending.
Do not claim physical condition completion before that test passes.

Read-only next-owner audit: `source_call_target/core_method.rs`, the CoreMethod
instance issuer and resolver currently project StringBoxText/PureRead only.
ArrayPush manifest metadata is NoValue/MutatesShape/ColdFallback/Unprojected;
this is not an accepted source semantic law. Existing method-call statement
lowering also bypasses exact_source_method_call and emits a name-based
`dst: None` call. Issuer-only extension would leave source rows unconsumed.

Next design after B1: co-seal exact `new ArrayBox` -> binding -> receiver,
push/1 argument sites and loop/frame with Text argument retention, mutation
and Fault. Connect NoValue statement consume/emit/finish through the existing
statement physical owner; replace the selected name-dependent edge in the
same slice. No invented ValueId, name/MIR-derived receiver proof or wholesale
manifest activation. Required negatives: foreign/reassigned binding, arity,
value-use of push, duplicate/residual rows, and argument failure before
mutation. Keep nested substring evaluation/coverage. Loop-external tail push
and substring/1 are distinct remaining boundaries, not covered by this tuple.
This is a design task, not implementation permission for ArrayPush.


### B1 closeout and B2-D0 selection (2026-09-22)

Final quick-profile catalog **92/92**, publisher **5/5**, Loop route **20/20**,
focused source Bool Call-to-Branch **1/1**, merged dependency/inventory **2/2**.
Logs: `/tmp/bool-b1-closeout.log` and `/tmp/bool-b1-closeout-{0,1,2,3}.log`.
Each direct test invocation had a nonzero executed count. The focused source
uses unchanged is_space/skip_ws bodies, completes lowering, verifies Bool
Call destination and reaches Branch through existing Copy/Compare condition
operations. Catalog tests retain calls under Not/short-circuit source syntax;
runtime condition lowering is unchanged. The full merged fixture is retained
and pins ArrayPush's precise dependency, not successful parser compilation.

The prior direct-ValueId Branch assertion was a test assumption error, replaced
by explicit Copy/Compare dataflow verification. The former Bool stop assertion
was superseded by the observed source-identified ArrayPush boundary. Neither
red is counted as baseline debt. No unresolved current-change red remains in
the focused B1 verification scope; full parser/runtime acceptance stays open.
Formatting, diff, pointer and publication-ingress guard pass. All changed Rust
files are below 760 lines after the separate state-operation split. B1 uses
one source catalog, one publication owner and existing condition/Call consumers;
no shared legacy deletion credit is claimed.

B2-D0 is now selected in design_stop:

```text
Decision: resolve ArrayPush source mutation and NoValue statement mapping before activation.
Source authority + canonical issuer: existing source construction/binding and CoreMethod owners; exact mutation/argument-retention issuance must be settled.
Non-authority: ArrayPush manifest presence, arr name, physical MIR type, or ColdFallback execution.
Fail-fast boundary: unsupported source relation must reject before argument/mutation effects; no synthetic value for NoValue.
Smallest next slice: close exact Array receiver/binding + push/1 argument/Fault contract + source statement consume/emit/finish and selected name-edge deletion.
Non-claims: no ArrayPush implementation permission, tail-push/substring-1 coverage, full split_lines/parser acceptance or runtime String ABI completion yet.
```


### B2 decision and ordered queue (2026-09-22)

```text
Decision: preserve constructor selection semantics; do not silently make explicit new ArrayBox intrinsic.
Source authority + canonical issuer: resolver construction sites/binding ledger and ordinary/brand declaration inventory; source constructor classification must co-seal with CoreMethod issuance and selected consumer capability.
Non-authority: ArrayBox spelling, CoreBoxId alone, generated manifest, physical receiver type, or a second environment read inside the source issuer.
Fail-fast boundary: unresolved/foreign constructor policy, shadowed or reassigned receiver, value-use of NoValue, duplicate or residual source consumption.
Smallest next slice: passive construction source transport, then source constructor classification plus selected capability and the bounded ArrayPush statement cutover.
Non-claims: this design does not activate ArrayPush, prove runtime Text/Fault ABI, complete parser acceptance, or delete shared legacy callers.
```

Read-only worker and local consumer audit agree on the following boundary.
`ordinary_new_candidate.rs::resolve` checks ordinary source declarations first
(including duplicate rejection), then leaves builtin/plugin construction to
compatibility. `CoreBoxId::from_name` supplies identity, not provider selection.
`box_factory/registry.rs::rebuild_cache` uses provider policy on runtime factory
lanes; this registry is not used by the selected C Array allocation consumer. `ConstructionTarget` explicitly retains
`Named` for explicit new. Thus the existing source inventory excludes user
shadowing, but cannot yet prove the selected builtin constructor.

This census covers: the loop-local split_lines construction/binding -> source
method issuance -> statement Recipe -> ArrayElementWrite emission. Includes
constructor shadowing/policy, receiver reassignment, nested argument coverage,
one-shot consumption and residual rejection. Excludes loop-external tail push,
substring/1, other Array methods and executable Text ABI acceptance.

| Order | Task / existing owner | Observable completion |
| --- | --- | --- |
| 1 | Source constructor classification: existing semantic package, resolver New site + initializer BindingRef, ordinary/brand inventory | Passive source transport first; then settle selected named-Array capability without deriving source meaning from C dispatch. Distinguish ordinary user ownership, proved builtin construction, unsupported provider/capability and missing evidence. Preserve named override behavior where supported; no runtime registry capture for selected C. |
| 2 | Source Array receiver and push contract: existing resolver expression inventory / CoreMethod issuer | Zero-argument construction site joins the exact initializer/receiver binding and loop frame under the same issuance. Reject reassignment conservatively; no alias inference. Co-seal push/1, Text argument site, NoValue and mutation semantics. Nested substring remains covered. |
| 3 | Statement cutover: existing expression port / statement normalizer / effect Recipe / ArrayElementWrite writer | Explicit NoValue consumes once without fabricated ValueId; typed Push reaches emit_array_element_write(dst=None, index=None). Remove selected routing through the method-name/physical-type classifier in the same bounded series; finish rejects remaining rows. |
| 4 | Focused source-to-MIR acceptance and deletion proof | Natural source witness emits the selected ArrayElementWrite; shadow/override/foreign/reassigned receiver, arity, value-use and duplicate/residual negatives pass. Guard proves the selected row cannot re-enter name-based dispatch. Recheck unchanged merged parser and classify its next terminal. |
| 5 | Executable Text mutation contract and existing canonical acceptance queue | Verify durable Text retention, alias/lifetime behavior, argument evaluation before mutation, and supported mutation failure propagation in the selected runtime. Then continue canonical parser acceptance, T5 caller cutover and R0 retirement. |

Rows 2–4 form one bounded migration series after row 1 closes; isolated issuer
activation is not completion. The physical writer already exists in
`builder/array_element_write.rs`. The statement normalizer currently bypasses
exact source method consumption, and `lowerer/effect_emission.rs` reselects
push by name and physical receiver type. Merely adding a CoreMethod row or
passing another string MethodCall would leave those two gaps open.

The selected delete-set is that source-owned statement's admission to the old
name/type dispatch, not every shared Array writer or compatibility caller.
Shared code is physically removable only after its remaining caller inventory
is zero. Report edge retirement separately from file/line deletion.

Runtime evidence remains distinct: the existing Array surface specifies Void
and WriteHeap, and the sole append owner retains an owned element. The checked
Array ABI currently covers numeric values; raw Text append status is not proof
of semantic Fault delivery. Source-to-MIR design must preserve argument effects
and mutation order but need not wait for executable ABI acceptance. Do not add
a second append owner or reinterpret an unchecked status as a verified Fault.

Row 1's semantic classification remains an internal design dependency, not
platform/CI wait. Only the passive source observation prerequisite below has
a closed implementation mapping; no Array semantic receipt is authorized. Required owner
README/reference updates belong to the corresponding implementation slice.


### B2-SOURCE landed prerequisite

Resolver New observations retain class syntax and ordered child sites without
issuing constructor/provider semantics; duplicate rows reject. New tests3 and
ledger20 passed (quick5m24s); detail remains in Git at `9887d6213a` and logs
`/tmp/b2-source-test.log`, `/tmp/b2-ledger-test.log`.
Selected C allocation uses `nyash.array.birth_h`, not UnifiedBoxRegistry;
runtime registry capture is not required, but source/consumer co-seal still is.

### B2 remaining contract: conditional Named Array requirement

Accepted direction after the selected-consumer premise audit: preserve
`ConstructionTarget::Named`. The existing semantic package/source CoreMethod
issuer may specialize push only under a retained, exact-source-site requirement
that the named constructor realizes Core Array semantics. This is a requirement,
not an unconditional builtin receiver proof. Same-name ordinary/brand ownership
excludes this specialization; missing or foreign source evidence rejects.
Zero-argument, no-field-override construction and exact unreassigned binding
form the bounded source shape. Other shapes do not acquire builtin authority.

The existing source-to-finalized-artifact handoff must retain this requirement
without reconstructing it from a MIR name. At
`backend_capability::enforce_published_backend_supported` / selected C preflight,
the selected constructor consumer and options must discharge it before artifact
publication. A provider may satisfy the requirement only with an equivalent
contract; unknown override, incompatible carrier or missing/foreign/duplicate
evidence rejects without fallback. The selected C allocation observer can supply
capability evidence, never source admission (`published_backend_view/map_named_allocations.rs`
already enforces this responsibility split for its own inventory).

The artifact retention mapping is accepted below. The selected C discharge
refinement below names the existing invocation/query owner. Activation still
requires retained source plus exact physical correspondence; neither metadata
nor allocation observations establish source semantics or executable Fault proof.


### MIR-CALL-PARSER-ARRAY-PUSH-B2-I0 — accepted bounded series

Change: replace the selected loop-local push statement's name/type-based
MethodCall dispatch with source-owned typed ArrayElementWrite; keep Named
construction and its conditional Core Array obligation through finalization.
Contract: one source CoreMethod issuer, existing resolver binding/Loop frame,
existing Recipe producer and ArrayElementWrite site issuer/writer. No provider
selection from names, no new append implementation, no fake root or result.
Done: natural source reaches the typed write and selected C preflight with exact
retained construction/write correspondence; positive and rejection coverage
below passes, selected legacy dispatch is unreachable, unchanged merged parser
is rechecked. Runtime Text/Fault and full parser acceptance remain explicit.
Stop: missing/foreign source obligation, unavailable Text Home contract, any
unretained egress, or a physical consumer that cannot discharge the obligation.
These are implementation failures to resolve in this series, not permission
for fallback, provisional receipts or a different source witness.

| Order | Single owner transition | Required boundary |
| --- | --- | --- |
| 1 | Existing semantic package/CoreMethod contract -> existing callable lowering state | Co-seal construction site, ordinary/brand exclusions, initializer BindingRef, exact unreassigned receiver, push site, loop/frame and exact Text argument producer. Extend existing target/Home schema with Array receiver, Text retained by receiver, MutatesShape and NoValue; this is conditional on the retained Named requirement. Existing selected substring result supplies the nested Text witness; no physical-type proof. |
| 2 | Source construction/write emission -> existing finalized artifact handoff | At the existing prepared New source port record the emitted allocation destination with source owner/site, retaining Named. Add module-only retention to FinalizedRootHandoffV1 for RootValidation::Absent and Script without an Array root. Bind without fabricating root/Birth. Finalization checks exact canonical function, allocation and dependent writes after compiler finishing. |
| 3 | Exact source statement port -> typed effect Recipe -> sole writer / backend preflight | Represent Value versus NoValue explicitly in the existing port; value demand rejects before child effects or result allocation. Statement consumes once, evaluates nested argument once, emits typed Push with dst=None/index=None, and finishes without residual rows. Backend checks the retained obligation against actual selected constructor/carrier before object publication; DirectArray is not accepted for Text without its own evidence. |

The existing `PreparedRawNewExpressionV1` / source claim port is the construction
attachment point. A selected requirement must reject a Core13 extern or other
route that bypasses the chosen Named emission; it may not disappear when the
ordinary user-Box claim is absent. The effect Recipe carries typed operation
identity, not a string that asks `effect_emission.rs` to reselect push.

Retention chain: `CompletedNormalDefaultRootCatalogLifecycleV1` ->
`into_artifact_parts` -> `FinalizedRootHandoffV1` ->
`compile_normal_with_published` -> `bind_finalized_root_handoff` ->
`PublishedMirBackendView.retained_handoff`. The current Absent arm drops
callables, and Script may return None; both must retain module obligations.
`into_parts` and ExplicitCompatibility's early return must reject obligations
that they cannot retain. Their diagnostic output is not parser acceptance.

Use the existing function metadata boundary for a passive detection projection
(allocation destination plus dependent write identities), while authoritative
source rows stay in the finalized handoff. It survives clone/semantic refresh;
it never grants receiver semantics. Following typed-array claim coverage,
preflight requires exact one-to-one write-marker/retained-row/physical-write
coverage. Several push rows may share one exact construction/binding; that
shared allocation is validated once, not rejected as duplicate source issuance.
Bare-module backend and the shared JSON root builder reject undischarged rows
in both profiles. Selected C internal body projection may use the retained
published view during preflight; that projection is not an executable artifact
or a generic allow flag. Reject missing, extra, duplicate, foreign-function,
post-finishing drift and scrubbed-authority clones before external output.

Focused coverage must include module without root, ordinary and Script roots,
value-use rejection, nested substring coverage, duplicate/residual consumption,
user/brand shadowing, constructor arguments/field overrides/reassignment,
unsupported selected carrier, and all egress cases above. Required tests use
the published source-backed entry; an earlier diagnostic stop is dependency
evidence only. No whole-crate/Windows CI wait is introduced. Update owning
README/reference and reuse lane guards; close the selected old edge in this
same bounded series. The original five-implementation-commit forecast is superseded
by the discovered source-loop dependency queue below; four stages are landed, D
is still open. No per-case cards or unrelated cleanup are added.


B2 stage 1 receipt: `5f328559de`, conditional source ArrayTextAppend/NoValue
contracts; 44 focused checks pass (7 new plus 7+7+20+3 existing), quick 5m24s.
Production activation was intentionally deferred. Source contract details live
in code/reference; Git retains the original closeout.

### B2 next implementation boundary — canonical retention decision

Decision: bind the canonical caller to its existing source contract inside the
semantic package, before lending it to Builder. `selected_mapping` owns the
canonical key -> declaration -> batch-slot relation; `core_method_source.rs`
already issues contracts from that slot's ledger. This is the co-seal point.
An emission draft is move-only transport of this association, not another
semantic issuer. Its constructor stays private to the semantic package.
Builder may record allocation/write observations, never supply a replacement
caller key. Owner equality or a crate-visible `new(key, requirement)` is not
sufficient provenance. Preserve the existing loop/frame/target contract too.

The read-only worker recommendation was checked against the existing package
issuer and selected lowering port. Both ordinary selected and Main static-child
loans take CoreMethod rows; both must transport the same package-bound relation.
Raw `take_source_core_method_calls` is storage access, not authority to mint it.
Unsupported TopLevel admission must reject before issuance, not silently drop
an obligation. Finalization resolves the retained canonical key through the
existing canonical definition lookup; it does not reconstruct source identity
from a physical function name.

| Next | Existing owner / deliverable | Completion evidence |
| --- | --- | --- |
| A | Package co-seal -> selected loan -> lowering state -> finalized handoff | Foreign key/ledger and cross-function substitution reject. Ordinary, Script-empty and rootless module retain the obligation; no fabricated root. Source/physical write coverage is exact, including shared construction and residual rows. |
| B | Source statement -> typed Recipe -> existing ArrayElementWrite writer | Natural loop-local literal and nested-substring push emit once with NoValue. Value demand rejects before effects. Replace the temporary retention-required stop with the real consumer only after A is connected. |
| C | Retained published view -> selected C capability discharge | Exact Named allocation consumer/options satisfy the retained obligation before object publication. Missing/duplicate/drift/unsupported carrier reject. Raw MIR, both JSON profiles, diagnostic and compatibility exits cannot shed it; internal preflight projection borrows the retained view. |
| D | Production package selection and selected old-edge retirement | Activate the extended issuer only with A-C wired; remove this source statement's route through method-name/type dispatch in normalizer and effect emission. Guard selected caller-zero at the old dispatch and recheck unchanged merged parser; report the next actual terminal. |

A-D remain tasks inside B2-I0, not new cards or independent completion claims.
B2 stage 2 established canonical/source transport, module-only handoff, passive
marker coverage and raw refusal. Stages 3/4 below connect collection/discharge;
the current D counterexample governs production readiness.

Stage 2 receipt: `6740988fa4`, 51 focused tests pass after quick 5m20s;
548 warnings. Package caller/source association, rootless retention and raw
rejection are verified, without production activation. Detailed cohort logs:
`/tmp/b2-retention-final-test.log`, `/tmp/b2-retention-regression-{0..9}.log`;
Git retains the implementation and original closeout inventory.

Executable Text lifetime/alias/Fault evidence remains the following acceptance
task, with real selected runtime behavior required. B2 source-to-MIR completion
does not claim serializer/parser completion, tail-push/substring-1 support, or
retirement of shared compatibility code whose other callers remain live.

### B2 C/D refinement — selected C invocation and remaining execution contract

Consultation reason: discharging a retained source requirement at C publication
changes a capability boundary; a read-only worker inspected the actual query,
serializer and append consumer before this Decision. No new receipt is needed.

Decision: use the existing static V2 invocation and its actual compile options.
Source authority + canonical issuer: existing package CoreMethod source co-seal; retained handoff preserves its owner/canonical/site relation.
Non-authority: C allocation query and MIR markers observe physical correspondence only; names and host registry do not grant source semantics.
Fail-fast boundary: reject mismatched or unsupported obligations before C compile/object publication; raw/JSON/non-supported lifecycle exits remain closed.
Smallest next slice: finish A/B evidence, then bind retained obligations to the existing static V2 allocation query and switch the selected caller in this series.
Non-claims: allocation compatibility is not Text lifetime, mutation Fault, full parser acceptance, or deletion of the shared MethodCall branch.

| Order | Concrete owner transition | Done / rejection evidence |
| --- | --- | --- |
| A/B (stage 3 verified) | Package collector -> callable finish -> finalized handoff; source statement -> NamedArrayPush -> ArrayElementWrite | Rebuild latest tree once; residual, foreign compilation, one-shot clones, missing allocation and same-construction multiple-write tests. Natural source coverage is mandatory at activation; manually assembled physical tests do not substitute. |
| C1 | published ingress + internal body projection | Validate retained source/marker/physical coverage. Only `emit_published_view_body(view)` borrows authority for internal serialization; both public JSON profiles and raw module still reject. No generic allow flag or marker removal. Explicitly preserve unsupported lifecycle rejection. |
| C2 | `compile_published_static_v2` -> `from_view_with_query` -> `PublishedStaticMethodCFrameV2::from_index` | Use the same C document/options for query and compile. Join retained canonical allocation to exact function/block/instruction query through existing MapBodyIndex. Require actual Array consumer; reject missing/duplicate/foreign/drift and DirectArray Text before compile. Existing invocation Drop handles failure. |
| D | package extended issuer -> selected statement producer/writer | Activate only after A-C connect. Literal Text and substring2 loop push evaluate once, use NoValue, and bypass no source obligation. Value-use rejects before child effects. Guard selected-site non-reentry into generic MethodCall and recheck unchanged merged parser to its next actual terminal. |
| Execution contract next | Existing append runtime owner and selected C writer | Establish Text contents and retention after original handle release, alias visibility, mutation failure and cleanup behavior. Resolve failure propagation before any checked-Fault or full executable acceptance claim. |

Deletion inventory: the selected source statement's edge to generic MethodCall
and name/type dispatch is retired by D. The shared `push | set | insert` branch
in `effect_emission.rs` still serves other callers; its whole physical deletion
requires their migration/Stop and caller-zero. No B2-exclusive old function was
identified for immediate removal. Record caller-edge retirement separately from
physical source deletion; neither local green nor the new typed arm proves both.

Execution-contract gap: current C typed append calls `nyash.array.slot_append_hh`
and discards its result; the kernel can return zero on failure. Array's existing
StringLike/keep storage is an implementation path, not checked-Fault evidence.
The current physical validator rejects checked Invoke writes. The next execution
task must choose the existing failure owner and close that mapping before changing
this ABI; do not silently reinterpret NoValue as successful mutation. DirectArray
Text stays rejected. This internal gap does not reopen VM or platform work.

B2 stage 3 receipt: `d657e6db71`, expected package inventory, one-shot handback,
module retention and typed write transport; **73 tests pass**, quick **4m18s**,
543 warnings. Five emission tests plus retention 51, Loop 3, Map 8, direct 6.
Logs: `/tmp/b2-emission-final-test.log`, retention regression logs and
`/tmp/b2-emission-{loop,map,direct}-regression.log`. README/reference and guards
pass; changed Rust <=738 lines. Physical witnesses do not prove natural-source
cutover. A zero-match helper filter was discarded and actual consumers rerun.

B2-I0 stage 4: published ingress/internal body projection now verify retained
source correspondence; raw JSON and unsupported lifecycle still reject. Static
V2 frame construction validates exact allocation observations from its existing
same-invocation query before compile. Array passes; incompatible/missing/foreign/
duplicate observations reject. No new C API or source authority was introduced.
Quick build **4m18s**, warnings **543**. Named allocation/frame **13**, internal
JSON **8**, raw JSON **3**, backend **3**, emission **5**, Script **2+3**:
**37 passing tests**. One pre-existing private C-driver test remains ignored;
this is not real C-query or runtime execution evidence. Logs:
`/tmp/b2-capability-test.log`, `/tmp/b2-capability-regression-{0..6}.log`.
README/reference, formatting, pointer/ingress guards and diff checks pass.
Next D: select the extended package issuer, verify natural literal/substring2
source-to-published handoff and C frame, retire selected generic dispatch ingress,
and recheck unchanged merged parser. Runtime Text/Fault remains separate.

### MIR-CALL-PARSER-ARRAY-PUSH-B2-D0 — source GenericLoop carrier synchronization

Decision: retain the existing GenericLoop/Select/PHI owners; bind exact source identities to their publication slots, including branch state and dynamic origins.
Source authority + canonical issuer: resolver local/variable/assignment sites -> pre_effect -> CallableGenericLoopV1SemanticRecipeIssuerV1 co-seal; existing carrier and join owners allocate/publish physical values.
Non-authority: names, Builder caches, MIR-inferred binding/origin, arbitrary BindingRef/ValueId setters, or restored consumption records.
Fail-fast boundary: validate complete source/slot correspondence before skeleton allocation; reject stale/foreign publication and residual consumption before finalized publication.
Smallest next slice: preserve resolver local declarations through the body-only partition, then co-seal exact source carrier relations in the existing Recipe issuer.
Non-claims: uncommitted D remains unverified; no runtime acceptance or retirement credit before the listed gates.

Current-change failure: natural `Scan.run(s)` with local Named Array, `i=0`,
`loop(i<1){arr.push("text");i=i+1}` and `return i` fails strict dominance.
Dump proves body bb3 `%11 = 0 + 1`, header bb2 reads `%11`, exit bb5 returns
`%11`; the loop header carrier is unused. The typed Array write itself is present.
The read-only worker traced `generic_loop_composer` body rebind -> restored name
cache only -> `exact_source_variable_value` reading the mutated callable ledger.
Existing `CallableGenericLoopSourceRelationViewV1.pre_effect()` is currently
unused by the physical adapter. Its source rows and the existing carrier owner
must be co-bound; do not repair this with name-first reads or an extra PHI solver.

The induction-only repair is rejected: additional carriers and both source If
owners share the ledger. The value-demand negative already passes; the initial
test-helper Debug compile error was fixed. Keep both current-change reds open.
Logs: `/tmp/b2-cutover-test.log` (5m03s; 1 pass/1 fail),
`/tmp/b2-cutover-diagnostic.log` (intentional diagnostic panic, not a gate; probe
removed after readback). No unresolved red is claimed as baseline or green.
The unchanged merged-parser probe now reaches `static-result-ingress/no-exact-static-target`
after the old loop-push boundary. Its old-terminal assertion is a current-change
failure (`/tmp/b2-cutover-merged.log`); identify the exact later source site before
updating that receipt. Preserve the uncommitted issuer/test changes; stage 4
`319a2d7102` remains the last verified pushed checkpoint.

#### Proposed scope and corrected selection boundary

This census covers: `CallableGenericLoopV1SemanticRecipeIssuerV1 -> source-port
composer -> existing carrier/join owners -> callable ledger publication`;
the rows below describe required consumer behavior, not proof of selected source admission. Excludes raw/Compatibility, LoopTrue, new
nested-loop support, runtime append/Fault and platform work. Read-only worker
`b2_cutover_ssa_audit` supplied the full admitted-shape and origin-state audit;
main readback confirmed those consumers. The later selector audit below supersedes the claim that both If forms are admitted through GenericLoop.

| Relation / transition (selection must be established) | Required treatment |
| --- | --- |
| Unique induction Carrier | Exact increment AssignmentTarget site -> BodyRebind -> BindingRef must match the unique source Carrier. |
| BodyOnlyRebind of pre-loop binding | Allocate an extra slot through the existing carrier owner; do not discard because it is absent from the condition. |
| Iteration / branch local | Retain resolver declaration site in the existing pre_effect product before outside-row partition discards it; never publish outside its source scope. |
| ReadOnlyOperand | Borrow existing exact value; no new loop carrier. |
| Local, assignment, call, Return | Existing source-site consumption remains once-only, including argument evaluation and publication/write receipts. |
| If: conditional-update and ordinary join | Both owners participate in branch entry/reset/join publication; ordinary join uses reaching predecessors, Select retains its existing evaluation/effect contract. |
| Conditional tail break / continue | Publish through existing exit/continuation payloads; do not treat a terminated branch as a normal successor. |
| Nested loop, missing increment placement, non-RecipeOnly | Preserve current explicit rejection; do not manufacture a binding to admit them. |

At the canonical Recipe issuer, co-seal pre_effect with located body and exact
assignment-target sites, recursively only for branches proved reachable through the selected route. Consumer support alone does not prove admission.
Group by BindingRef, retaining declaration scope and all target memberships.
The Recipe producer issues opaque slots; Facts keep source identity/roles only,
without ValueId or Recipe keys. Extend the existing carrier-target product so
source consumers do not reconstruct BindingRef from its current Vec<String>.
Names may label emission caches; colliding identities must not collapse.
`CallableGenericLoopV1SemanticRecipeV1` remains the existing source/structural
product; its emitted `CorePlan::Loop`/Select/CoreIfJoin and CorePhiInfo remain
physical composition products. This repair does not claim the final AST-free
portable migration is complete or introduce a second selector/PHI solver.

One scope owned by the existing callable ledger and borrowed by the source port
publishes: entry -> header carriers -> body/branch values -> existing join result
-> header condition -> final carrier/exit. Header reads must never see body-final
SSA. Zero iterations retain entry incoming; backedges and exits use the existing
carrier/JoinSig contract, with no new edge inference. Missing/duplicate slots,
foreign owner/site, induction mismatch and unclassified rebind reject before
allocation. Reuse, stale physical value and branch-local escape reject at their
publication boundary. Finish checks all source rows and closes the scope once.

The second audit found `rebind -> invalidate_rebind` removes active dynamic
origin. A values-only branch restore is therefore invalid. Save/restore values
and active origins together, filtered to the source-visible scope. Never rewind
consumed sites, completed locals, append/write receipts, affine/construction
consumption, or the historical value-origin index. An active origin, when present,
must name the current exact value. Ordinary assignment keeps its existing origin
invalidation; physical carrier publication is not a second source assignment.

Selected origin policy: retain provenance only when the existing dynamic-origin
owner proves the same formal origin for every reaching incoming and their exact
values. Missing/different or invalidated incoming means origin unavailable, not
an unconditional loop rejection; a consumer requiring origin retains its named
failure. Foreign/stale evidence rejects. Do not infer cyclic header provenance
from PHIs or mechanically retag an old origin with a new ValueId.

Task 1 accepted Decision (2026-09-22): co-seal an identity-or-kill transfer per
exact source assignment and existing Recipe continuation. In current GenericLoop,
`exact_source_assignment_rebind` always uses ordinary `rebind` (kill); no source
preserve receipt is consumed there. An unchanged binding is identity. Sequence
composes these transfers; each branch uses its own entry snapshot. Meet only
incoming paths authorized by the existing continuation/exit owner. Return/break
paths do not enter the backedge meet; continue does. No new CFG reachability
solver or AST-name classifier is introduced.

Before body lowering, header origin is entry origin iff every reaching backedge
preserves it; if a kill reaches any backedge, header origin is unavailable.
With no backedge, entry alone determines header origin. This finite source
transfer solves bootstrap without consulting future MIR. Apply the same rule to
zero-iteration/header-false and break exit values; Return has no loop successor.
An origin-dependent use after a kill is an intentional missing-capability stop,
not permission to resurrect the initial formal from the historical value index.
The repair must classify any newly observed rejection against this contract.

The existing dynamic-loop prepare/open/operation/close family provides the
Enter/Backedge and exact-assignment verification pattern, but is not connected as
a second GenericLoop physical owner. Do not promote its dedicated preserving-add
contract from similar syntax. Any future preserve transfer requires that exact
source/operation owner and is outside this ordinary-rebind repair.

The existing dynamic-origin owner will expose scoped entry/header/branch/exit
projection plus finish: validate owner/slot/site and exact entry, install the
precomputed header disposition, record observed identity/kill transitions, then
check every reaching backedge/exit and residual row before publication. Values
and active origins move together; historical entries and consumption remain
monotonic. A transfer/execution mismatch is a named reject, never a retry.
The identity/kill policy remains accepted; the later selector correction reopens
source-to-Recipe correspondence. No runtime evidence follows from this policy.

#### Ordered tasks and closeout

| Order | Task / existing owner | Observable completion |
| --- | --- | --- |
| 1 (design accepted above) | Close dynamic-origin transfer in `normal_callable_dynamic_origin` with the source Recipe issuer | Specify entry/header/backedge/join/exit transfer for unchanged origin, ordinary invalidation, one-sided termination and missing origin. Classify any changed rejection before implementation; no newly accepted origin or silently narrowed source cohort. Identity/kill and continuation-aware meet are accepted above; implementation evidence remains tasks 2-3. |
| 2 (after route mapping) | Co-seal declaration scope, target memberships and Recipe slots in `normal_callable_loop_handoff` / `normal_callable_loop_source_facts/generic` | Every admitted assignment belongs once; induction, extra carriers and scoped locals stay distinct. Missing/foreign/duplicate/shadowing evidence rejects before physical allocation. |
| 3 (after route mapping) | Connect scoped publication in ledger/source port, `generic_loop_composer`, carriers and selected branch owners | Natural zero/one/multiple iterations, extra carrier, branch read-after-write, one/both terminal branches and local shadowing preserve exact values and origin disposition. Consumption never rolls back. Existing Select/PHI/exit owners stay sole. |
| 4 | Finish D via production package issuer and published-view tests | Literal/substring2/multiple push with optimization off/on passes strict MIR verification and retained C-frame checks; value demand remains rejected. Identify the exact next merged-parser source terminal before updating its receipt. Retire selected source push -> generic MethodCall ingress in the same change. |
| 5 (next execution contract) | Existing append runtime/C owner | Verify Text contents, retained lifetime, alias visibility, mutation Fault and cleanup through selected C execution. NoValue alone proves no successful mutation; no full parser/runtime claim before this evidence. |

Tasks 2-4 replace the unsynchronized source-ledger publication path; they do not
open a refactor lane. `conditional_update.rs` is currently 759 lines: keep added
source-state hooks in its existing responsibility modules or split that owner
before exceeding 760/800; do not compress lines. Update GenericLoop module README
and `docs/reference/mir/loop-recipe-contract.md` with the implemented contract.
Reuse focused source/carrier tests and existing ingress/pointer guards, one quick
Cargo process at jobs<=4; no full CI wait or new per-row guard.

Delete accounting remains exact: task 4 removes the selected caller edge into
name/type MethodCall dispatch. Shared `effect_emission.rs` push/set/insert code
remains until its other callers switch/Stop and caller-zero is established.
Do not count this dependency repair as whole-function or LegacyCallV0 deletion.
The design-only checkpoint is b0e3215f97; task 2 WIP is preserved at the design stop below. Preserve D WIP until its current-change reds are resolved.


Task 2 remaining implementation boundary:
Change: co-seal exact assignment target membership, declaration coverage and
BindingRef -> Recipe-local slot in the existing semantic Recipe issuer.
Contract: the source induction must match the planner increment site; pre-loop
rebinds receive slots, local rebinds retain scope without becoming carriers.
Done: natural-source induction/extra/local/shadow witnesses and coverage/owner
negatives pass before physical allocation. Slot publication remains task 3.
Stop: do not resolve binding identity from a diagnostic name or physical value.


Task 2 declaration retention landed at `d7c6648d7b`: exact local declaration
sites survive Ready/body-only partition; unread locals retain scope. Handoff9,
source/lease16 and raw-child12 passed (37 total, quick4m32s, warnings542).
This receipt does not close slot co-seal or D; detailed changes live in Git.

Task 3 connection audit: one read-only worker confirmed the existing name-map
collision sites. Key the existing state/collection owners by BindingRef for source
and String for raw; opaque slots identify loop carriers only, not every local.
Migrate these three tuples without adding a PHI/Select owner:
- `direct_associated` Local/Assignment -> definition publication: retain the
  exact declaration/target BindingRef with its value.
- `associated_source/direct_if -> dispatch/if_join -> steps/join_payload`:
  exact pre/then/else maps and key-preserving join results; source snapshots must
  not merge Builder name caches. Project joins onto pre-scope BindingRefs to
  exclude branch locals while preserving same-name outer bindings.
- `conditional_update` branch updates -> existing Select emitter: preserve the
  update key through result publication. Reuse the existing per-row PHI and
  Select emission; diagnostic labels never become binding lookup keys.
The existing carrier allocator/finalizer receives the co-sealed slot relation;
retire source `post_body_map.get(label)` and name-only carrier discovery as its
callers switch. No synthetic renaming, new shadowing rejection or second solver.


#### Selector correction and bounded task 2 acceptance (2026-09-22)

Decision: reopen task 2 co-seal for naturally selected Generic inputs; branch publication stays task 3 design.
Source authority + canonical issuer: resolver BindingRef/sites -> source Facts issuer -> existing selector -> Generic semantic Recipe issuer.
Non-authority: composer support, names, forced Generic selection, raw Select support, or missing LoopCond projection.
Fail-fast boundary: exact source declaration/target/induction coverage before any physical allocation.
Smallest next slice: direct Local/Assignment induction, repeated extra-carrier updates and same-name local; retain original If as a separate admission negative.
Non-claims: no branch/SSA synchronization, LoopCond admission extension, D cutover or runtime completion.

Read-only worker `b2_cutover_ssa_audit` and main readback establish these tuples:

| Source boundary | Existing owner / disposition |
| --- | --- |
| Direct local + variable assignment + last induction step | Generic extractor's Local/Assignment validators, explicit increment index and RecipeOnly source composer; source sites map to retained pre_effect and Recipe slots. Extra-carrier/shadow variants require dynamic confirmation. |
| Pure update If | `break_continue_item` -> ConditionalUpdate -> `loop_builder` retained owner -> selector excludes Generic. Consumer support never proves Generic admission. |
| Test payload without source identity | `loop_cond::issue` first rejects SourceIdentityMissing; the earlier ProjectionMissing prediction is withdrawn. Observe exact disposition in the retained original fixture. |
| Production pure update If | `source_loop_bridge::from_input` -> forest projection issuer -> `issue_with_source_relations`; method-call inventory is empty, so SourceItemsMissing precedes projection validation. This is static evidence, not a production execution receipt. |
| Admitted source LoopCond | `into_physical_input` -> source preflight -> `lower_loop_cond_break_continue_source` -> PlanVerifier/PlanLowerer. Uses its own PhiMaterializer and the shared callable source port/ledger. |
| Branch consumer distinction | Generic direct_if and LoopCond associated-source join converge on `dispatch/if_join::lower_if_join_state_core`. RecipeOnly source item dispatch lacks ConditionalUpdateIf; raw Select support is not source support. ExitAllowed is a separate associated-block path. |

Task 2 may repair its tests from this mapping without changing the selector or
removing If from the task queue. Keep the original If shape as an explicit named
LoopCond ingress negative; it cannot substitute for branch publication evidence.
The direct source fixture exercises two updates to the same extra binding,
loop-local rebind, unique induction and same-name shadowing without new syntax.
Reject missing/foreign/duplicate membership before slot issuance. The same
semantic Recipe owns the relation; no detached authority or physical allocation.

Task 3 must next bind selected LoopCond carriers to BindingRef and establish
header/body/exit plus shared join values/active-origins and local scope. Use an
actually admitted source caller; pure-items-empty admission and a new source
ConditionalUpdateIf arm are separate semantic Decisions, not hidden in this fix.
Missing increment, non-RecipeOnly and nested Generic restrictions stay unchanged.
Task 4 still requires natural D strict SSA, exact merged-parser terminal, caller
switch and old ingress retirement. Task 5 requires selected-C Text/Fault evidence.

Task 2 landed at `231ef5f9e0`: source21 + child12 = 33 pass, quick4m08s,
warnings542; owner/coverage/induction/shadow and retained If negative covered.
Four fixture reds resolved; If terminal observed as SourceIdentityMissing.
Historical failures/logs live in that commit. D SSA/merged-parser WIP remains
current-change red, excluded from task 2 evidence.

#### Task 3 landed physical prerequisites

`09a70b4864`: exclusive Raw/Source final-values preserve owner/site/slot/binding
through remapping; raw completion rejects Source, and its publication hook is
after final Lower verification. 94 focused tests passed, quick5m00s, warnings542.
`9460ee7b68`: shared skeleton allocation, keyed carrier/PHI closure and Continue
preserve keys independently of labels; missing incoming rejects. 112 focused
tests passed, quick5m26s, warnings542. Detailed transport audit/cohorts/logs live
in those commits. Neither receipt activates source-ledger publication or closes D.

Retained contract: source context borrows the same active ledger/scope; validate
all final rows before writes, publish once after typed-value verification, and
never restore consumed receipts or infer origins from PHIs. Raw name lookup
remains in raw facades only; source entry validation and branch correspondence
remain tasks below. Plan cloning copies transport, not publication permission.

#### Task 3 remaining design and execution queue (2026-09-22)

Decision: retain the accepted identity/kill policy and keyed physical owners;
close its missing source-continuation mapping before implementation resumes.
Source authority + canonical issuer: resolver sites/BindingRef and source-bound
Core continuation evidence, co-sealed by the existing semantic Recipe issuer.
Non-authority: diagnostic names, assignment union, future MIR reachability,
raw Select support, and historical origins.
Fail-fast boundary: missing source/continuation correspondence is Design stop,
not a new runtime rejection or narrower source admission.
Smallest next slice: settle the exact Generic operation/class input to the common Core owner below; no code or fixtures.
Non-claims: existing 112-test receipt does not establish scoped SSA or cutover.

Change:
  Replace source GenericLoop name-map restoration with exact scoped ledger
  publication. The current carrier relation holds assignment/declaration
  membership, but no continuation-indexed origin transfer. The adapter context
  has no source-completion capability; shared physical helpers alone cannot
  activate it. This corrects the preceding broad implementation permission.
Contract:
  Same callable ledger owns values and active_origins; consumption, completed
  locals and historical origins never roll back. Existing Recipe/JoinSig owns
  continuations, existing PHI/Select owns physical values, and completion commits
  once after final Lower verification. A cloned plan grants no publication right.
Done (ordered tasks; existing tasks 3-5 remain one bounded series):
  1. Close the common-Core input gaps in the table below, then co-seal exact
     assignment identity/kill and reaching backedge/exit rows in the same
     Recipe issuance. Include zero-iteration, continue, break, return and joins;
     classify non-admitted shapes explicitly. Pure update If remains LoopCond,
     never forced Generic; missing admission is a separate semantic Decision.
  2. Connect exact entry validation and scoped entry/header/body/exit state in
     ledger + dynamic-origin owner + source port. Scope locals by BindingRef;
     validate all slots before allocation and transfer observations at finish.
  3. Wire composer/context to shared keyed carriers and exclusive Source final
     values; validate the whole completion before any publication. Close natural
     SSA, extra-carrier/shadowing, admitted branch, wrong-owner, duplicate finish,
     missing-slot and origin-kill evidence; preserve raw parity.
  4. Close D current-change reds, select package issuer, and remove the selected
     source push -> name-dependent MethodCall edge in the same cutover. Identify
     the exact merged-parser terminal. Shared legacy code awaits its caller-zero.
  5. Close selected-C Text contents/lifetime/alias/mutation-Fault/cleanup evidence.
     Update owner README and reference in the implementing slice; use existing
     focused tests and ingress/pointer guards, one quick build, no CI waiting.
Stop:
  If a route lacks source-owned continuation evidence, resolve that exact mapping
  in this card before code. Do not infer it from physical plans, narrow fixtures,
  resurrect killed origins, or declare production completion from local green.

Read-only worker and main source inspection establish this design gap; no new
Rust/runtime evidence is claimed. Existing three-file D WIP is preserved.

Task 3 source mapping decision (worker + main): reuse common source-bound Core
and the neutral JoinSig engine. Exact resolver String literal -> ConstText/Text;
actual substring2 -> CallSlot/Text; actual push1 -> CallSlot/no result. Co-seal
item/site/argument/result keys with existing selected source call contracts.
NamedArrayReference is conditional on the retained construction requirement,
not a physical handle or Dynamic. The package retains sole one-shot discharge.
ConstText Decision: NonFaulting means no returned language-Fault successor for
verified literal text, not Pure/no allocation/OOM recovery. Exact argument site
and bytes come from `ledger.literal_source(requirement.argument())`. Existing
`hako_llvmc_ffi_static_v2_emit.inc` retains length/nonnull/zero->trap checks;
`nyrt_c_abi_v0.md` and `runtime-data-dispatch.md` retain contract/host-fatal policy.
Substring remains a real call with its own exact result contract.

Remaining source-program mapping constraints: V1 common Core needs typed input;
Dynamic's fixed envelope is not Generic authority; original increment/source
ordinals must survive filtering; two-exit branch pairs beyond Break/Continue
need actual selected-route coverage. These still block Generic activation.
Boundary: Generic semantic Recipe -> common logical transfer; no runtime claim.

Root closure prerequisite landed at `aea1f4a290`: shared issuance validates all
root After rows; typed single/collection payloads retain checked sole admission.
No Generic activation; detailed design and logical evidence live in that commit.

Closure prerequisite baseline classification:
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib 's6c_'` on the changed tree
and clean parent `b2f55c7ac5` both yield 64 pass / 3 fail, all with
`ObjectDefinitionsNotConsumed` (parent quick5m52s, warnings542):
- `mir::builder::pinned_text_invocation_binding::tests::s6c_loan_and_session_target_prepare_one_planless_frame_ingress`
- `mir::builder::resolved_lowering::common_v2_s6c_substring_callout_admission::tests::admission_projects_one_checked_substring_site_without_effect`
- `mir::builder::resolved_lowering::common_v2_session::s6c_substring_v9_issuer::tests::issuer_adopts_checked_v9_and_lends_only_inside_callback`
Classification: known baseline debt for this BoxShape, not ignored green.
Logs: `/tmp/b2-closure-s6c.log`, `/tmp/b2-closure-parent-s6c.log`; parent used an
isolated clean worktree with the same lockfile. Existing D WIP was untouched.

ConstText logical opcode (landed bounded BoxCount; no source activation):
Change: add logical V2 ConstText with exact stored text and Text result through
schema/verifier/JoinSig projections; no source/physical route activation.
Contract: source authority remains the exact resolver literal; common Core must
later co-seal its item/site/value. NonFaulting has the restricted meaning above.
Done: empty/Unicode/embedded-NUL text survives wire roundtrip and typed logical
verification; wrong/missing result class and duplicate definitions reject.
Existing Dynamic/S6C contracts stay unchanged; update owner README/reference.
Stop: do not synthesize a CallSlot, infer Pure or physical representation, or
claim D/source acceptance. NamedArray class/co-seal and branch coverage follow.

Closure final receipt: new3 + Dynamic33 + S6C64 = 100 distinct pass; S6C3
remain the parent-reproduced baseline debt above. Quick5m56s, warnings542.
Logs `/tmp/b2-closure-final-{build,dynamic,s6c}.log`; fmt, both existing guards
and diff check pass. Initial filename filter matched zero and was corrected to
`join_sig::v2::collection_tests::`; that zero run earns no evidence credit.
No source caller switch/retirement is claimed. Three-file D WIP stays excluded.

ConstText split boundary: typed_schema_v2.rs grows from 757 to 764 lines.
Before further verifier changes, move check_operation and its class/definition
helpers to an existing-verifier child module in a separate BoxShape commit;
keep structural control checks in the parent. No line compression or new owner.

ConstText receipt: new5 pass; schema41 + JoinSig20 + Dynamic33 + S6C65 =
159 distinct pass. Same3 S6C baseline reds/reason as `b2f55c7ac5` above;
no new red. Quick4m43s, warnings542. Logs `/tmp/b2-const-text-{build,schema,
join,dynamic,s6c}.log`. README/reference, fmt, ingress/pointer/diff checks pass.
Source literal co-seal and physical activation remain unimplemented.
Next split contract: move check_operation/expect_defined_class/define_value to
one verifier child; retain shared expect_defined_value and control checks in the
parent. Existing test paths and error precedence remain unchanged; no opcode,
source admission, new receipt, fallback or caller switch in that BoxShape.
