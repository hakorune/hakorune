---
Status: SSOT
Date: 2026-09-03
Decision: MIRBUILDER-FINAL-PIPELINE-v1
Scope: canonical source ingressからatomic MIR publicationまでの唯一のglobal pipeline-order authority。Parser grammar、language semantics、Backend loweringの詳細は隣接ownerへ委譲する。
Related:
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/investigations/function-exit-f1-draft-seal0-s0-execution-task-2026-07-25.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - src/mir/builder/README.md
---

# MirBuilder Final Pipeline

## Current Capsule

- **Current decision:** the final pipeline remains one-way, and canonical MIR
  calls converge on a typed structural target before argument or MIR effects.
- **Current implementation status:** the typed publication spine is landed for
  StaticBoxMethod, FreeStatic, FreeFunction, Builtin Print, and one root-lexical
  DeclaredInstance source family. The exact key survives Atomic Publish and the
  selected static/free/Print backend rows are typed. Group A now separates the
  canonical `Call(MirCall)` shape from the explicit `LegacyCallV0` outer shape;
  broad compatibility consumers remain reachable, and Hako has no borrow-only
  published-view ingress for SameModuleInstance.
- **Next ordered task:** execute one canonical Print reader slice in the VM
  reference consumer, then continue the finite R6 reader/producer migration.
  Do not reopen landed source families or add another precursor D0.
- **Production stop line:** no String formatter, opaque registry, second AST
  walk, post-argument resolver, optional/empty loan, or backend repair may fill
  a missing semantic target.
- **Retirement finish line:** canonical source families use the one pipeline,
  Call fallback/retry is zero, transitional integration hubs are reduced by
  finite owner, and each switched family repays its exclusive legacy
  code/tests/guards/docs before the next family. Closed detail lives in Git,
  not a copied current-tree archive.

## Decision

この文書は、canonical source ingressからatomic MIR publicationまでの
**唯一のglobal pipeline-order authority**である。Parser grammar／source AST
schema／language semanticsと、published `MirModule`を受け取る各Backendの
lowering詳細は隣接ownerが持つ。この文書はその詳細を吸収せず、受渡し境界と
authorityの向きだけを固定する。

MirBuilder再構築の最終目標は、replacement cell数、pack消化、Rust LOC、または
ファイル数ではない。

最終目標は、source semanticsの決定からfunction draftとmodule公開までを、
次の一方向のproduction authorityへ収束させることである。

```text
Hakorune AST
  -> Resolver
  -> VerifiedResolvedFunction
  -> Control-flow Observation
  -> Facts
  -> RoutePolicy
  -> RecipeComposer
  -> RecipeVerifier
  -> Verified Lowering Plan
  -> Plan / Body Lowering
  -> CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> ReadyFunctionDraftSealV1
  -> OpenFunctionDraftSealV1::prepare
  -> PreparedFunctionDraftSealV1
  -> one infallible commit
  -> CompletedFunctionDraftV1
  -> ModuleDraftCollectorV1
  -> atomic module transaction
```

短縮形は次で固定する。

```text
Resolve
-> Observe
-> Facts
-> Recipe
-> Verify
-> Lower
-> Seal
-> Collect
-> Atomic Publish
```

人間向けの七段projectionは次で固定する。これは上のnormative chainを
並べ替える第二pipelineではない。

```text
1. Frontend
   Source -> AST -> Resolve

2. Semantic Observation
   Resolve -> Observe -> Facts

3. Verified Recipe
   Recipe -> Verify

4. Function Lowering Session
   Lower -> function-local finish

5. DraftSeal
   Ready -> prepare -> infallible commit

6. Module Transaction
   Completed drafts -> Collect -> Atomic Publish

7. Backend Boundary
   published MirModule -> VM / AOT / LLVM / other selected backend
```

`Verify`をRecipeより前へ移したり、Backendをsource semanticsのrepair ownerに
したりしない。七段projectionで省略された内部edgeのauthorityは、常に上の
normative chainが優先する。

`MIRBUILDER-INPLACE-REPLACEMENT0`は、この最終形へ現在のproduction
MirBuilderを移す方法である。replacement cellやstructural measurementsは
移行の観測手段であって、最終architectureの代わりではない。

## Current convergence spine

現在のCall seamは全体pipelineから独立した第二roadmapではない。`Resolve`
から`Lower`へauthorityを一方向にするための、次の直列 prerequisite である。

```text
typed Global target family / issuer Decision
  -> reusable ingress lifecycle guard
  -> shared-runner Wpre profile/root/decoder contract
  -> finite explicit-CLI arbitration and outside-fate closure
  -> reference child isolation and CoreDirect typed terminal policy
  -> force-hv1 finite census and selected retirement
  -> strict recursive root owner and by-value decoder seams
  -> strict one-shot schema selection with invalid-v1 retry = 0
  -> MirCall/CallFlags transport retirement
  -> source-owned effect authority for every promoted target family
  -> preserve exact free/static/import/compatibility target relations
  -> JoinIR declaration co-seal and false GC Global retirement
  -> observer-only FunctionCall completion contract and package install gate
  -> builtin/Extern disposition and finite preflight lineage classification
  -> package-owned pre-effect site/target disposition
  -> one affine raw handoff and direct-payload deletion
  -> exact Method(Some) receiver/argument ABI preservation
  -> named compatibility late-recovery retirement and overlapping Method(None) retirement
  -> remaining static/CorePlan/operator producer dispositions
  -> exact touched-owner shelf and finite current-HEAD B1 census
  -> typed Global common-core and all compiled-consumer cutover
  -> remaining wire / construction / selected-terminal closure
  -> current-HEAD consumer census
  -> mandatory-Callee Call schema + impossible-state guard retirement
  -> post-Call integration-hub and source-shelf cleanup
  -> remaining selected pipeline cutovers
  -> final repository convergence audit
```

Backend retirement is a sibling program, not another semantic pipeline. Only
CoreDirect and force-hv1 bypass closure precede Wpre. Broad/default Rust route
retirement, current vm-hako retirement, independent AOT HMI construction, and
Rust `MirInterpreter` physical deletion follow Call R7 and post-Call integration
in the order owned by `vm-active-lane-retirement-ssot.md`.

## Call completion and retirement program (2026-09-03)

This is the authoritative Call projection of the global pipeline above. It is
not a second MirBuilder, a second task ledger, or permission to recreate a
landed family. The three completion levels are deliberately separate:

```text
MS1-M  MirBuilder core
       every production source family issues one mandatory typed target or a
       named pre-effect rejection; target selection precedes arguments; the
       module is published atomically; resolver/recovery/fallback/retry are 0

MS1-B  product backend
       each selected product family consumes the borrowed published module;
       unsupported profiles stop before object emission without semantic repair

MS1-D  physical retirement
       old Call schema, compatibility readers, family-only tests/guards/docs,
       selected-C/VM legacy consumers, and disconnected Builder surfaces are 0
```

The fixed order is:

```text
M0  MIR-CALL-PUBLICATION-SPINE-INTEGRATION-R0
    closed checkpointを検証して統合。semantic changeなし
M1  MIR-CALL-R6-PRODUCER-CONSUMER-CENSUS-R0
    current-HEADのwriter/reader/optional target/repair/backendを一度だけ分類
M2  MIR-CALL-REMAINING-FAMILY-DISPOSITION-R0
    残familyを4状態へ閉じ、zero/multiple tupleは追加D0なしでpark
M3  MIR-CALL-COMPATIBILITY-QUARANTINE-R0
    legacy/JSON/Unified-OFF/name repairをcore外へ隔離。retryなし
M4  MIR-CALL-MANDATORY-CALLEE-R6
    mandatory Calleeへatomic cutoverし、canonical func/optional stateを削除
M5  MIR-CALL-HAKO-PUBLISHED-VIEW-INGRESS-I0
    real caller + borrow-only lossless ingressがexactly oneの時だけ再開
M6  MIR-CALL-BACKEND-FAMILY-CUTOVER-R0
    一familyを切替え、旧edgeと専有temporary assetsを同じseriesで削除
M7  MIR-CALL-COMPATIBILITY-RETIRE-R7
    caller-zero後に旧Call/Method(None)/repair/fallback/retryを削除
M8  MIRBUILDER-PHYSICAL-THINNING-R0
    barrel/raw port/variable_map bypass/stale wrapper/dead proofをleaf-first削除
M9  MIRBACKEND-LEGACY-RETIRE-R0
    replacement coverageとcaller-zero後にselected-C/Rust-VM consumerを退役
```

## S-class completion gates (post-M9, non-executable navigation)

The M0--M9 order is the MirBuilder/product completion program. A stronger
release claim is allowed only after these five independent gates are observed;
they add no alternate pipeline and do not authorize work in the current R6
slice.

```text
S1 Enforce
   the critical Resolve→...→Atomic Publish boundaries have private
   constructors/capabilities or equivalent negative guards; an invalid
   pre-publish or backend-repair transition is not representable through the
   production API.

S2 Prove
   VM/reference and LLVM/AOT compare observable results, failures, and side
   effect order for the selected language corpus; optimized MIR is checked
   against the same witness.

S3 Delete
   LegacyCallV0 has zero production writers, reissuers, and readers; the
   compatibility ingress is explicit and caller-zero before physical removal.

S4 Bootstrap
   fixed stage0→stage1→stage2 selfhost output is reproducible and its
   semantic/identity comparison is machine-checked.

S5 Release
   a clean checkout with pinned tools builds and runs the documented sample
   ladder, records limitations, and publishes regression evidence for a
   third-party user.
```

These are completion gates, not current implementation rows. The active
`CURRENT_STATE.toml` and the linked owner documents remain the only execution
authority.

After M0 and before the next semantic implementation family, the independent
`DOCS-HISTORY-RETIRE-R1` repayment may remove its four pre-classified closed
cards. It is skipped on any census drift and does not block M1.

### M1 census contract — `MIR-CALL-R6-PRODUCER-CONSUMER-CENSUS-R0`

```text
status = `accepted_design_stop`
implementation permission = false
```

This is one finite, read-only census before Call R6. It starts at all
`MirInstruction::Call writers` and target-bearing construction owners in the
MirBuilder, CorePlan, compatibility emitter, published-module, object, C/JSON,
Hako, VM, verifier, optimizer, serializer, and rewrite surfaces. It ends at
the canonical `MirModule` publish boundary and every downstream Call reader,
legacy fallback/retry/repair terminal, and object-emission decision. The
inventory records each production-reachable site once; test/reference-only
occurrences are a separate non-production bucket.

The only accepted dispositions are `Canonical`, `CompatibilityOuterIngress`,
`ExplicitUnsupported`, and `DeadDeleteCandidate`. The census must name the
source authority, canonical producer, consumer, and finite old-edge delete set
for every production site. `callee=None`, `Method(None)`, string/name/header or
registry lookup, `args[0]` or `ValueId(0)` receiver repair, and
`fallback/retry` are never canonical dispositions. A zero/multiple authority
tuple is `NoSafeSlice`/`ParkedSealed`; it does not authorize a new receipt,
adapter, fixture, guard, or implementation row. This row grants **no
implementation permission** and does not reopen the landed StaticBoxMethod,
FreeStatic, FreeFunction, Builtin Print, or root-lexical SameModuleInstance
families.

M1 has one finite state vocabulary:

| disposition | meaning | next action |
| --- | --- | --- |
| `Canonical` | exact source authority and mandatory typed target exist | retain and include in R6 |
| `CompatibilityOuterIngress` | public/fixture compatibility still owns the shape | isolate outside canonical core |
| `ExplicitUnsupported` | language/profile combination is intentionally unsupported | typed reject before effect/object |
| `DeadDeleteCandidate` | structural asset has caller zero and no public contract | delete with its exclusive evidence |

### M1 observed production census (2026-09-03)

The first pass is now mapped by owner rather than by raw `Call` occurrence.
The raw search includes structural readers and test fixtures; those are not
independent semantic producers.  The following owner groups cover the
production-reachable boundary from source publication through backend/object
admission.  Each group has one disposition; no group is silently counted in
two rows.

This is an observed owner-group map, not a completion claim: M1 remains open
until the direct production sites and their reissue/reader edges are
cross-checked against this grouping exactly once.  No implementation row is
authorized by this map alone.

| owner group | disposition | source/target authority | downstream consumer | finite old-edge set |
| --- | --- | --- | --- | --- |
| `UnifiedCallEmitterBox::emit_canonical_instance_value_terminal_v1`, `physical_terminal::emit_finalized_generic_call_v1`, `MirBuilder::emit_prepared_cataloged_call_v1`, and the published-key branch of `VerifiedCanonicalDirectCallEmissionV1::materialize` | `Canonical` | existing source/package handoff, selected `Callee`, and catalog key | `MirInstruction::call` then `MirModule` publication | none for landed families; R6 must preserve the typed callee |
| `MirBuilder::emit_legacy_call`, unified-OFF `compat_entrypoints`, `exprs_call` unified-OFF, and ordinary-new fallback | `CompatibilityOuterIngress` | legacy `CallTarget`/value ingress only | BoxCall/NewBox/legacy Call consumers | legacy emitter, name carrier, unified-OFF fallback, and ordinary-new fallback |
| JoinIR `call_generator`/`convert`/block handlers plus JSON v0/v1 parsers | `CompatibilityOuterIngress` | JoinIR or wire payload; not source semantic authority | compatibility MIR module/JSON consumers | Const+Call pair, `func` carrier, `args[0]` receiver convention, wire-name reconstruction |
| `ssot::method_call`, callsite canonicalization, generic/global method route repair, and string-corridor Call rewriters | `CompatibilityOuterIngress` | existing MIR callee or explicit runtime helper contract; no new source target | optimizer/route metadata and runtime compatibility consumers | string `Method` shape, optional receiver, post-MIR name/box repair, and helper reissuance |
| `builder_emit` receiver materialization, PHI edge rematerialization, instruction/value/SSA/optimizer/verifier/printer readers | `Canonical` (structural) | already-issued callee and ValueId only; no target lookup | canonical MIR structural consumers | none; adapt mechanically during R6, without semantic re-resolution |
| published backend view/object admission for landed Global/Print rows | `Canonical` | published module definition table and typed row | selected-C typed transport/object emitter | no name lookup; retain the existing published row |
| selected-C JSON/name compatibility path, Hako published ingress, and VM reference `SameModuleInstance` arm for arbitrary UserBox | `ExplicitUnsupported` | published-module/profile boundary | object emission or reference executor | no new arbitrary-UserBox admission; reject before object/VM execution |
| `ssa::phi_input_materializer::legacy_candidate::prepare_legacy_phi_repair_candidate_v1` | `DeadDeleteCandidate` | no production caller; test-only candidate API | exclusive legacy-candidate tests | candidate module plus its exclusive test surface, after caller-zero guard |

The census confirms that the remaining `Call` work is not a new semantic
resolver: it is compatibility quarantine plus a mechanical R6 schema change.
The legacy groups remain live until their named caller-zero conditions are
met.  The PHI candidate is the only currently observed delete candidate; its
tests are not deleted by this design-stop row.

The direct-site cross-check index for this pass is finite and is kept here so
the owner groups can be audited without treating every structural match as a
new task:

* canonical source/direct owners: `calls/build.rs::emit_prepared_cataloged_call_v1`,
  `canonical_direct_call.rs::VerifiedCanonicalDirectCallEmissionV1::materialize`,
  `calls/unified_emitter/physical_terminal.rs::emit_finalized_generic_call_v1`,
  and the claimed `Birth` branch of `ordinary_new_admission.rs`;
* compatibility ingress owners: `calls/unified_emitter/compat_entrypoints.rs`,
  `builder/exprs_call.rs` unified-off, `join_ir_to_mir/call_generator.rs`,
  `join_ir_to_mir/convert.rs`,
  `join_ir_to_mir/joinir_block_converter/handlers.rs`,
  `runner/mir_json_v0/{call.rs,module.rs}`,
  `runner/json_v0_bridge/lowering/expr/call_ops.rs`, and
  `runner/json_v1_bridge/parse/mir_call.rs`;
* explicit runtime/projection owners: `ssot/method_call.rs`,
  `ssot/extern_call.rs`, `array_element_write.rs::canonicalize_legacy_array_write_calls`,
  and the string-corridor sink/concat rewriters;
* structural reissuers: `builder_emit.rs`,
  `ssa/phi_input_materializer/edge_rematerialization.rs`, and the
  call-rewrite passes.  They copy an already-issued callee and never select a
  target;
* parked physical owner: `normal_module_transaction/physical_thunk.rs`,
  whose source issuer is still `RelationPresentIssuerMissing`;
* downstream readers: MIR instruction/value/SSA/optimizer/verifier/printer
  modules, published-view/object admission, JSON emitters, C/wasm/LLVM
  consumers, and the VM reference handler.  These are readers or explicit
  profile rejects, not semantic issuers.

Occurrences under `#[cfg(test)]`, test-only modules, archives, and diagnostic
fixtures are retained in the separate test/reference bucket.  This index is
the cross-check input for M1; it does not turn a structural reader into a
producer and does not grant R6 implementation permission.

#### M1 closeout (read-only census)

The direct-site cross-check found no unclassified production-reachable Call
owner inside the stated boundary.  The canonical writer set is the four
source/package owners above plus the claimed ordinary-new `Birth` branch; the
compatibility writer set is the named unified-off, JoinIR, JSON, explicit
runtime, and ordinary-new fallback ingress; the remaining production matches
are structural readers/reissuers, explicit profile rejects, or the one parked
physical-thunk owner.  Test/reference-only occurrences remain outside this
result.  M1 is therefore closed as a census, not as an implementation or
schema-cutover claim.

### M2 remaining-family disposition — `MIR-CALL-REMAINING-FAMILY-DISPOSITION-R0`

```text
status = `accepted_design_stop`
implementation permission = false
```

M2 consumes the closed M1 owner map and assigns each remaining family one
fate before compatibility quarantine or R6.  A family with zero or multiple
authority tuples is `ParkedSealed`; it must not receive another D0, receipt,
adapter, fixture, guard, fallback, or retry.  A family is not reopened merely
because a legacy reader or backend still contains a matching string shape.

| remaining family | disposition | authority/consumer decision | next action |
| --- | --- | --- | --- |
| explicit `Extern` provider calls and runtime helper reissuers | `CompatibilityOuterIngress` | explicit provider contract is retained at the outer boundary; no source target is reconstructed from the string | isolate during M3, preserve the typed `Extern` callee in R6 |
| indirect `Value`/closure invocation | `ExplicitUnsupported` | no single published target/definition consumer is available for the selected product profile | reject before selected object/VM execution; reopen only with an exact tuple |
| explicit receiver, nested, and upvar instance methods | `ParkedSealed` | receiver/source ingress and a lossless selected consumer are not a single existing owner | no new family asset; reopen trigger is one exact carrier and finite delete set |
| ordinary-new unclaimed/builtin fallback | `CompatibilityOuterIngress` | claimed `Birth` is canonical; unclaimed builtin/plugin lowering remains an outer compatibility route | quarantine after caller census; no new semantic issuer |
| physical normal-main thunk | `ParkedSealed__RelationPresentIssuerMissing` | physical owner exists but no source-backed issuer can legally supply its target | leave parked; do not synthesize a symbol or target |
| arbitrary UserBox on selected-C / Hako published ingress | `ExplicitUnsupported` | current profiles have no lossless published-view consumer for this family | fail before object emission; language `me.method` remains retained |
| constructor/closure creation (`NewBox`/`NewClosure`) | `Canonical` (outside Call) | these are construction terminators, not Call target producers in this census | keep their existing owners; do not widen the Call schema row |

M2 is complete only when every M1 group is represented by exactly one of the
four dispositions or an explicitly recorded `ParkedSealed` boundary above.
The next executable row remains M3 compatibility quarantine; no semantic
family is reopened by this table.

M2 closeout is accepted: the worker premise audit cross-checked the named
UnifiedCall, ordinary-new, JoinIR/JSON, explicit-runtime, structural, and
parked physical owners against this table. No second semantic issuer or
unclassified production-reachable owner was found. M2 is therefore closed as
a finite disposition, not as an implementation or Call-schema claim.

### M3-A compatibility quarantine — `MIR-CALL-COMPATIBILITY-QUARANTINE-M3-A`

```text
status = `accepted_fast`
implementation permission = true
scope = `UnifiedCallEmitterBox` environment-disabled fallback only
```

```text
Decision:
  Fence the existing env-disabled legacy fallback outside the canonical
  UnifiedCallEmitterBox core. Keep the explicit compatibility owner intact.

Source authority + canonical issuer:
  Existing source/package issuer and `emit_unified_call_required_v1` issue
  the typed call. The compatibility outer caller owns legacy emission.

Non-authority:
  env reads inside the core, JSON, JoinIR names, registry/header lookup,
  args[0], backend success, fallback results, and a new receipt or adapter.

Fail-fast boundary:
  Required ingress must never reach `emit_legacy_call`. If canonical and
  compatibility callers cannot be separated, stop before changing code.

Smallest next slice:
  Remove the `!unified_call_enabled -> emit_legacy_call` branch from
  `UnifiedCallEmitterBox`; route explicit compatibility through its existing
  outer owner without changing Call schema or ordinary-new/JoinIR/JSON paths.

Non-claims:
  No M3-B ordinary-new change, no M3-C JoinIR/JSON change, no mandatory-Callee
  R6, no backend/VM change, and no new semantic target authority.
```

Census boundary: `UnifiedCallEmitterBox::emit_unified_call` entry -> its
`emit_legacy_call` branch and required-ingress terminal; includes the env
disabled branch and `RequireGenericReceipt`, excludes `exprs_call.rs`,
`boxcall_emit.rs`, ordinary-new, JoinIR, JSON, VM, and backend consumers.

M3-A acceptance is mechanical:

```text
UnifiedCallEmitter core env reads                         = 0
UnifiedCallEmitter core `emit_legacy_call` calls          = 0
required ingress -> legacy route                         = 0
explicit compatibility outer callers                    = preserved
Call schema / target authority                            = unchanged
```

At least one focused positive required-ingress test and one negative
legacy-reachability test must be recorded. Existing known-red baseline
comparison remains mandatory; an unclassified red aborts the row.

M3-A closeout is accepted at `474e8518b0`:

```text
UnifiedCallEmitter core env reads                         = 0
UnifiedCallEmitter core `emit_legacy_call` calls          = 0
required ingress -> legacy route                         = 0
explicit compatibility outer callers                    = preserved
Call schema / target authority                            = unchanged
```

The outer `MirBuilder` facade now owns the profile decision; the typed core
is configuration-free, and required/receipt ingress fails closed when the
profile is disabled. Focused evidence is `physical_receipt` 23/23,
`method_call_terminal` 8/8, and
`normal_script_direct_static_physical_publication` 3/3. The fixed baseline
runner was observed three times with identical
`7578 total / 7411 passed / 138 failed / 29 ignored`, inventory SHA
`0632d98fe396207747dd7b597563f08e81b1dfaf4054340b4cb411edc2ac12dd`, and
failure SHA `29569949bacd86b39af4f122dad137ae4d476185363d667722a0b87cf56d4ba1`.
The inventory grew by one passing focused test; the 138-name known-red set
did not change. The manifest refresh is therefore an explicit baseline
receipt update, not a green-suite claim. Python comparator tests (15/15),
pointer guard, and diff check pass. No Call-schema, ordinary-new, JoinIR,
JSON, backend, VM, fallback, or retry semantics changed.

`Method(None)`, `callee=None`, a string target, backend success, JSON, a
registry/header lookup, `args[0]`, and `ValueId(0)` are never canonical target
authority. Construction terminators such as NewBox/NewClosure remain outside
the Call schema census unless they consume or recreate a Call target.

The currently landed source families are not reopened by M1/M2:
StaticBoxMethod, FreeStatic, FreeFunction, Builtin Print, and the root-lexical
SameModuleInstance semantic vertical. Broad Extern, Value/closure invocation,
explicit object method ingress, nested/upvar receiver, ordinary-new no-claim,
generic CorePlan GlobalCall, and physical thunk stay parked unless the census
finds exactly one existing source authority, one canonical issuer, one live
caller, one lossless selected consumer, and one finite old-edge delete set.

MirBuilder core completion is reached at MS1-M, not after repository cleanup:

```text
supported source family -> Facts/Recipe -> mandatory typed target
                        -> arguments once -> canonical Call -> Atomic Publish
unsupported family      -> named typed reject before source/object effect

production callee=None                         = 0
production Method(None)                        = 0
target repair by AST/name/header/registry      = 0
receiver inference from args[0]/ValueId(0)     = 0
fallback / retry / profile reselection         = 0
partial module publication                     = 0
```

### M3-B ordinary-new outer quarantine — `MIR-CALL-COMPATIBILITY-QUARANTINE-M3-B`

`M3-B` is closed as
`ParkedSealed__OrdinaryNewUnclaimedCompatibilityMultiWriterSharedOwner`.
The existing `Birth` route is the sole canonical ordinary-new issuer, while
two unclaimed compatibility writers share the outer legacy APIs; therefore
there is no exclusive delete-set and no implementation slice. Name/registry
reissue, fallback/retry, new receipt/adapter, and ordinary-new widening remain
forbidden. Reopen only when one live outer owner and one finite delete-set are
proven; otherwise keep this family parked.

### M3-C JoinIR/JSON outer quarantine — `MIR-CALL-COMPATIBILITY-QUARANTINE-M3-C`

```text
status = `accepted_design_stop`
implementation permission = false
scope = JoinIR/JSON outer ingress -> first MIR Call/module publication
Decision = ParkedSealed__JoinIrAndJsonHaveMultipleIngressOwners
```
Canonical source/package publication remains the only semantic issuer (the sole semantic issuer).
JoinIR bridge, MIR JSON v0/v1, Program JSON v0, and JSON egress are separate
compatibility owners; wire names, `func`/Const, `args[0]`, registry lookup,
parser retry, and backend success are non-authority. Each ingress fail-closes
without canonical or alternate-parser fallback.
Four read-only outer-ingress censuses are complete: JoinIR is a test-only
`DeadDeleteCandidate` pending API/evidence/merge independence; MIR JSON,
Program JSON, and JSON egress each have multiple caller series and shared
fallback/reparse edges. No JoinIR/JSON deletion, Call R6, backend/VM change,
or new receipt/adapter is authorized until one owner, one live caller, and an
exclusive finite delete-set are proven for one ingress. Initial roots are
`src/mir/join_ir_to_mir`, `src/runner/mir_json_v0`, `json_v1_bridge`,
`json_v0_bridge`, and `mir_json_emit`.

### M4 mandatory-Callee R6 — `MIR-CALL-MANDATORY-CALLEE-R6`

```text
status = `accepted_fast`
implementation permission = true
scope = Group A: MirInstruction Call/LegacyCallV0 type separation
Decision = ExistingMirCallCanonicalAndLegacyCallV0OuterBoundary
```
The existing source/package issuer and typed `MirCall` remain canonical, but
public `MirInstruction::Call` is now the bounded Group A implementation seam.
Promote the existing `MirCall` to canonical mandatory `Callee` `Call(MirCall)` and isolate the
old `func`/`Option<Callee>` fields as `LegacyCallV0`; no target/effect/ABI
re-resolution is allowed. `callee=None` and `Method(None)` remain explicit
legacy-only shapes during R6 and are rejected by canonical publication.
Canonical publish rejects `LegacyCallV0`, while explicit compatibility ingress
may retain it until R7. Move writers/readers mechanically, classify every
changed test and red, and keep no fallback/retry. Do not add `CallV2`, a second
resolver, or a new semantic receipt. Group A ends at compiling central APIs;
later writer/backend/compatibility migration remains in the same branch series.
Canonical coreの最終targetはopaque IDやphysical symbolではなくwire-stableなtyped
structural identityである。追加familyはproduction censusが証明した場合だけ加え、
legacy Stringはowner-private compatibility ingressで一度だけ解決しcoreへ入れない。

#### Group A closeout — `45c6759962`

`MirInstruction::Call(MirCall)` is now the canonical mandatory-callee shape for
the bounded Group A seam. The former `func`/`Option<Callee>` carrier is explicit
`LegacyCallV0`; no active-tree struct-style `MirInstruction::Call { ... }`
literal remains. Central readers, diagnostics, remappers, and published-view
admission understand both shapes without re-resolving target, effect, or ABI.
The evidence is limited to this bounded seam: `cargo test --no-run`,
`cargo build --features vm-reference`, focused instruction/physical/view/JSON
tests, pointer/active-surface guards, and `git diff --check` passed. This does
not claim producer/reader caller-zero, R7 deletion, Hako/VM retirement, or a
green whole-library suite. The next boundary is the read-only
`MIR-CALL-R6-CURRENT-HEAD-RECENSUS-C0`; Group B and R7 remain unopened.

### R6 current-HEAD re-census — `MIR-CALL-R6-CURRENT-HEAD-RECENSUS-C0`

The post-Group-A scan is bounded to active `src/`, `crates/`, and `tests/`
Rust/`inc` files (excluding `archive/` and `target/`). It starts at the two
`MirInstruction` call variants and ends at MIR structural consumers,
publication/view admission, JSON/JoinIR compatibility ingress, and compiled
backend readers. Raw occurrences are not semantic owner counts:

```text
canonical `MirInstruction::Call(...)` matches       45 lines / 19 files
canonical `MirInstruction::call(...)` constructors  31 lines / 16 files
explicit `LegacyCallV0` matches                    877 lines / 326 files
receiverless `Callee::Method` shapes                11 files
`callee: None` literal matches                      32 lines
`emit_legacy_call(` matches                          5 lines
```

The finite owner dispositions are:

| owner class | disposition | current evidence and boundary |
| --- | --- | --- |
| existing `MirInstruction::call` producers and typed tuple readers | `Canonical` | source/package owners and central MIR structural readers; no new target/effect/ABI issue |
| Builder legacy facade, JoinIR conversion, JSON v0/v1 parsing, unified-off and explicit compatibility ingress | `CompatibilityOuterIngress` | `LegacyCallV0` is retained at the outer boundary until R7; no canonical fallback into it |
| instruction methods, value/SSA/optimizer/verifier/printer, callsite rewriters, and published-view validation | `Canonical` / structural | copy or validate an issued callee; they must not select a target or infer a receiver |
| MIR interpreter, WASM, and product LLVM readers that currently match only `LegacyCallV0` | `CompatibilityOuterIngress` | reader migration is still required; canonical `Call` has no direct backend arm yet and must fail closed rather than fall back |
| `Callee::Method { receiver: None, .. }`, missing-callee guards, and legacy sentinel fixtures | `ExplicitUnsupported` / compatibility | never a canonical publication shape; keep only at the explicit outer/test boundary |

The scan found no active old struct-style `MirInstruction::Call { ... }`
literal and no new semantic issuer introduced by the enum split. It did find
three completion blockers for the next R6 slice: (1) the published backend
view still accepts both instruction shapes, (2) compiled backend readers do
not yet consume `Call(MirCall)` directly, and (3) the raw legacy carrier
remains present across compatibility and structural reissuers. Therefore this
re-census closes the Group-A inventory but does not authorize global schema
deletion. Group B must choose one existing reader/family, add no new issuer,
and prove a finite old-edge delete set before implementation permission is
opened.

### R6 Group B — `MIR-CALL-R6-GROUP-B-VM-CANONICAL-PRINT-I0`

```text
status = `accepted_fast`
implementation permission = true
```

The first Group-B candidate is deliberately one reader and one canonical
family. The existing source-backed Print issuer is the only semantic owner;
the VM reference backend is merely a typed consumer. This row does not make VM
the product backend and does not delete the legacy carrier.

```text
source authority:
  print_stmt::build_print_from_value -> unified typed Print target

canonical issuer:
  emit_unified_call_required_v1 -> physical terminal -> MirInstruction::call

consumer:
  MirInterpreter::execute_instruction -> execute_global_target(Print)

non-authority:
  func/Option<Callee>, name lookup, registry, args[0], fallback, retry,
  and the legacy handle_call entrypoint

fail-fast:
  canonical Call is accepted only for the exact Builtin(Print) target in this
  row; wrong arity is rejected before provider dispatch and every other
  canonical target remains an explicit unsupported error.

old-edge delete set:
  the single canonical Print-to-wildcard unsupported branch in the VM reader.
  LegacyCallV0 writers/readers and all other backend routes are out of scope.

acceptance:
  execute_instruction runs one canonical Print call, rejects wrong arity and
  non-Print canonical calls without legacy/name fallback, and existing VM
  feature build plus focused tests remain green.

non-claims:
  no VM product promotion, no SameModuleInstance support, no R7 deletion,
  no JSON/JoinIR migration, no Call schema redesign, no new receipt/variant.
```

The bounded reader implementation is now complete: canonical Print is handled
directly by `execute_instruction`, wrong arity and non-Print canonical calls
fail closed, and `LegacyCallV0` remains untouched as explicit outer
compatibility. Focused positive/negative tests, the existing VM feature build,
the active-surface dispatch, the current-state pointer guard, and diff checks
are the required evidence. This closeout does not claim whole-schema reader
caller-zero or any product-backend/VM retirement.

このDecisionはtyped Globalの実装許可ではない。ordinary `FunctionCall`は現在、
selected shadow profileでDeferredになりpackage発行へ到達できないため、target
loanより前に次を設計しなければならない。

```text
same traversal observer-only site/name/arity/argument observation
  -> owner tree can complete without issuing a target
  -> complete inventory is dispositioned before package install
  -> exact typed target or typed reject / ParkedSealed
```

観測scratchをpackage外へ浮遊させたり、第二AST walkで回収したりしない。既存brand
で検出できない具体的mispairが無い限り、新しいresolver-session receiptも作らない。

Call closure前の巨大integration ownerは保持する。semantic changeで触るsourceが
760行以上なら、そのrowの直前にbehavior-neutral owner-specific splitを置き、800行
到達をhard stopにする。Call closure後にだけ、`CompilationContext`、raw ports、
semantic adapter、ambient root/recursion state、`builder.rs` barrelをfinite owner単位で
縮める。全面crate splitや第二MirBuilderは作らない。

## Authority map

| Boundary | Owns | Must not own |
| --- | --- | --- |
| Resolver | `BindingId` / `ScopeId` / `RegionId` / callable target / source provenance | MIR emission、route retry |
| Observation / Facts | sourceとcontrol-flowの観測結果 | MIR mutation、hidden acceptance policy |
| RoutePolicy / Recipe | 一度だけ行うroute選択とlowering義務 | real `ValueId` / `BasicBlockId`、publication |
| RecipeVerifier | omission、duplicate、coverage、exit、carrier、merge契約 | repair、別Recipeへのfallback |
| Verified-plan Lowering | CFG、operand、Binding SSA、edge、PHI materialization | ASTからのroute再判定、別ownerへのretry |
| Function-local Finish | CFG / semantic / If / Binding SSA / PHI / resolved-binding / Completion の全closeと `ReadyFunctionDraftSealV1` 発行 | profile選択、Return書込み、draft publication |
| FunctionDraftSeal | exit、PHI closure、type/signature/metadata、session closeのprepareとcommit | Recipe再解析、source route選択 |
| Draft Collector | `CompletedFunctionDraftV1`の完全集合 | open/prepared draftの公開 |
| Module transaction | candidate moduleのsuccess-only atomic publication | partial insertion、failure後のresume |

Facts、Recipe、Verifierの詳細契約は
`recipe-first-entry-contract-ssot.md`と`recipe-tree-and-parts-ssot.md`が持つ。
Function exit semanticsは`docs/reference/language/function-exit-and-entry-result.md`
が持つ。DraftSeal、collector、module publicationの現行実装とaccepted
evidenceはRelatedに列挙したsource owner／taskにある。この文書はそれらを
結ぶ最終pipeline authorityを所有する。

## Loop specialization navigation

この文書は全compilerの順序だけを所有する。Loop固有の再帰
Facts/Recipe/JoinSig/Verify/Lower順序は
`joinir-loop-selfhost-recipe-pipeline-ssot.md`、post-Recipeのphysical
demand/session境界は`loop-common-physical-demand-and-session-ssot.md`が
所有する。現在実行中のbounded profileとexact rowは
`CURRENT_STATE.toml`の`current_execution_design`へ辿り、ここへ複製しない。

### Selected initializer materialization seam

pre-Builder semantic packageとLowerで初めて割り当てられる物理値は、互いを
再発行しない兄弟authorityである。selected callableがLoopへ入るときだけ、
次のrelationを一度co-sealする。

```text
installed-package selected semantic loan
  requires SelectedCallableSemanticRefV1::Dynamic
  + request-local completed local materialization
  + exact located Loop source/schedule
  -> one scoped selected Dynamic initializer admission
```

packageはcallable/Recipe/lifecycle意味を所有し、request-local stateは
`BindingRef -> ValueId`投影だけを所有する。located Loop boundaryは両者を
co-sealしてsole consumerへ渡すが、source semantics、Recipe、JoinSig、型を
再発行しない。Ordinary/Staticはこのcellを選択せず、既存の唯一のpost-success
TypeContext publicationとexact-MirType routeを保つ。Dynamicだけがpackage-loaned
programからbounded V2 routeへ入る。missing/foreign/duplicate relationはeffect前に
rejectする。新しいStatic/Dynamic closed sumやfamily arbitrationを作らず、
Dynamicを`MirType::Unknown`やlegacy GenericLoopで偽装しない。

selected Dynamicの最終source authorityはfinal exit-transaction co-sealから貸す
narrow initializer viewである。移行中のgeneric source seedは、cutoverでproduction
callerを0にするか、final programと一つのpackage-internal non-splittable co-seal
からだけborrow可能にする。二つのsource classifierを独立consumerへ公開しない。

admissionだけをcaller-zero productとして先行発行してはならない。最終co-seal
はnamed consumerと同じproduction replacement cellでissue/consumeし、旧selected
edgeを同時に削除する。selfhost header-result carrierとのbootstrap循環で
source-backed result/ABIが不足する場合、正本sourceへ明示result annotationを
置き、現在選択中のfrontendがnormalized header rowを一件だけ発行する。現在は
Rust final-source producer、selfhost parity後はHako producerをatomic cutoverで
選ぶ。同一compileで両方をadmitせず、frontend固有result receipt、body/Loop/MIR
inference、compatibility retry、fixture narrowingで循環を越えない。

明示result `: i64`はdeclared-result syntax authorityであって、logical class
`Dynamic`の物理carrierを`Integer`としてReturnできる証明ではない。bounded
`ParserScanLoopBox.skip_while/4`は、その変換自体を避けるA-primeを採用する。

```text
A-prime:
  pos/end: i64 source contract
  -> exact parameter transport / BindingRef relation
  -> exact local copy i = pos
  -> mixed typed Recipe
  -> I64 carrier / operations / returns
  -> ImmediateI64 AOT/LLVM physicalization
     Rust VM evidence is reference/compatibility-only and non-gating

not selected here:
  global all-values-as-handles
  language-wide tagged representation cutover
  terminal Dynamic-to-i64 helper
```

source result annotationからcarrierを逆算しない。`pos/end`のsource contract、
resolver binding、local copy、Recipe class、physical representationを一方向に
co-sealする。`src`/`pred_chars`とDynamic invocation temporariesはDynamicのまま、
induction carrierだけをI64にする。consumerがbare bits、metadata、runtime table、
TypeOp、MirType、sentinel-zero helperから欠落したprovenanceを修復してはならない。

parameter/result境界は二つの時刻へ分ける。

```text
pre-session demand:
  exact parameter contracts
  + mixed Recipe / JoinSig / Completion sites
  + required target capability
  ValueId / BasicBlockId / MIRなし

session-local realization:
  exact demand row + formal/local/PHI/return physical IDs
  -> ImmediateI64 receipts + site-keyed Completion claims
```

semantic ownerはsession IDsを持たず、session realizationはresult contract、return
site、Recipe classを再分類しない。AOT/LLVMはexact selected capabilityがあれば
`Direct`、なければ`RejectBeforeEffect`であり、A-prime I0に`Checked` helperはない。
Rust VMはproduction capability、session prerequisite、cutover gateではなく、新しい
DynamicV2 provider/receipt/representation adapterを追加しない。
Dynamic temporary Faultとcleanupのprimary/suppressed順序は既存exit transactionが
所有する。source annotationを理由に既存Dynamic ValueIdへ`MirType::Integer`を
後付けしてはならない。full tagged Dynamic corridorは将来taskとしてparkし、
A-prime失敗時に自動選択しない。

### Bounded loop unification boundary

Dynamic full-body cohortがphysical-input/demandまで閉じた後も、common
physicalizerはRecipeからtransferを再推論してはならない。統一する核は
次の二つのcomplete protocolだけである。

```text
verified Recipe placement
  + JoinSig-owned logical transfer view
  -> prepared physical layout

complete operation/source-effect ledger
  -> complete physical demand
```

`physical_layout`/`recursive_after`は`LoopConditionV1`や`as_recipe()`から
Predicate/Jump/Backedgeを再構築せず、`segment_allocator`はRecipe条件を再走査
してHeader/Bodyを再分類しない。common physicalizerのstop lineは
`ReadyLoopAfterContinuationV1`であり、Callable profile-close、Tail、ABI、
Completionはcallable ownerが持つ。V1/V2を型変換するadapter、synthetic
`ItemKey`、名前・順序によるrepair、第二JoinSig/Recipe/physical plannerは
禁止する。

このcleanupはA-prime parameter/Recipe/physical-input rowsの後に開くparked
BoxShape laneであり、実行行を先取りしない。詳細なsubtaskとcaller-zeroの
退役条件は、active Dynamic cardの
`LOOP-UNIFICATION-AFTER-DYNAMIC-D0` sectionだけを参照する。

Durable order is the exact parameter contract, atomic mixed-Recipe recut, then
Builder-free physical input. Loop authority cleanup and the AOT/LLVM
exact-I64 gate are mandatory before site-keyed Completion, DraftSeal
preparation, and session-local realization open. Rust VM is not a mandatory
sibling and cannot unlock production. One production replacement follows.
After the first production cutover, semantic parity and performance
promotion may proceed as
sibling proofs; every required sibling must be green before a selfhost
producer is activated. Exact task tokens and cleanup census remain in the
active card.

## Non-negotiable laws

### 1. Meaning is decided once

source shape、route、Recipe、exit／merge義務はVerifierより前で決める。
Verifierを通過した後に、LowerまたはDraftSealがASTを読み直して別の意味を
選んではならない。

```text
forbidden:
  Recipe
    -> Lower
    -> DraftSeal reclassification
    -> another Recipe / Legacy fallback
```

### 2. Lower consumes verified products

Lowerは`VerifiedRecipe`、verified `CorePlan`、または同じ責務を持つverified
lowering productだけを受け取る。名称が将来`LoweredRecipe`などへ縮退しても、
未検証入力をLowerが再判定しない契約は変えない。

### 3. Seal completes; it does not plan

canonical function pathでは、Body Loweringはexit operandとexact exit blockを
準備する。physical `Return`の唯一writerはDraftSealのdetached prepare projection
であり、`PreparedFunctionDraftSealV1::commit(self)`は検証済みprojectionをmove
するownership-only terminalである。

multiple source Returnでもこのownerは増えない。
`VerifiedFunctionCompletionV1::ExplicitReturns`がdeclared result分類とexact
ordered sitesのsole semantic ownerである。そのborrowed exact-result projection
から一方向に得たABI、各siteの`BindingRef` operandを
一つのmove-only setへco-sealし、既存Completion consumptionがsite-keyedな
physical claimをexactly onceで閉じる。DraftSeal prepareはdetached projectionの
各claimed exit blockへ一つのReturnを書き、全検証を完了する。commit後のfallible
workは0で、profile lowererはReturnを書かない。単に複数exitを一つへ集めるため
だけのsynthetic return-join/PHIは作らない。backend/MIR制約が別のverified owner
として要求した場合だけ、独立Decisionで開く。

`CanonicalSsaFunctionSessionV2`経路における`ReadyFunctionDraftSealV1`の
issuerは、target `finish_for_draft_seal`だけに集約する。各V2 profile
lowererがCFG／SSA／PHI／Completionのfinish順を手作業で複製して直接
`ReadyFunctionDraftSealV1::new`を呼ぶ形はreplacement debtである。非V2の
既存direct constructor callerもcompat debtとして増加禁止にし、最終退役で
production callerを0にする。
profile固有ledgerは先にprivate close receiptへ畳み、common finish terminalが
そのreceiptと全function-local ownerをconsumeして初めてReadyを発行する。

The current R0 audit is intentionally bounded: the V2 session has three
existing profile constructors (`trivial_ssa`, `direct_accum`, and
`nested_predicate`), while one non-V2 `CanonicalFunctionLowererV1` direct
constructor remains an explicit compatibility allowlist entry. R0 migrates
the three V2 paths only. A move-only profile-close receipt and sealed function
identity prevent terminal re-inference of body/site/target/current-block
facts. The guard contract is mechanical: V2 direct Ready-constructor callers
must be zero, the non-V2 allowlist must not grow, and every V2 finish order is
owned by the one terminal API. Physical Loop lowering, production selection,
retry/fallback retirement, and legacy deletion are later rows.

すべてのfallibleなexit、PHI、type、signature、metadata、verification、
session-close準備を`prepare`で終える。`commit(self)`はownership-onlyの
infallible terminalとする。

この契約へ未移行のproduction Return writerは互換完成形ではなく、
replacement debtである。

### 4. Publication is all-or-nothing

collectorが受理してよいのは`CompletedFunctionDraftV1`だけである。全draftが
揃う前にlive moduleへfunctionを直接挿入しない。

```text
success:
  completed drafts -> candidate module -> atomic publish

failure:
  discard candidate module and unpublished drafts
```

### 5. Authority never flows backward

後段は前段の決定をconsumeするだけである。

```text
Seal      -> Recipe      forbidden
Lower     -> RoutePolicy forbidden
Collector -> Lower       forbidden
Publish   -> retry       forbidden
```

## Responsibility diagram, not a file quota

上の箱は責務境界であり、各箱に専用Rust file、type、trait、guardを一つずつ
作る要求ではない。

- 一つのtypeが隣接する機械的段階を安全に表してよい。
- Plan LoweringとBody Loweringは実装上interleaveしてよい。
- ただしsemantic authorityの向きは逆流させない。
- 新しいwrapperやproof fileを作ること自体を進捗に数えない。

## JoinIR naming boundary

`JoinIR`という名前は、現在のrepositoryでactiveなBuilder
Recipe/CorePlan系とlegacy JoinModule系の両方に使われた履歴がある。

このSSOTでは、次の責務名を使う。

```text
Control-flow Observation:
  StepTree / ControlForm / CondBlockView / Loop / If / ExitLine observation

Verified-plan Lowering:
  verified Recipe/CorePlanからCFG / merge / carrier / Binding SSAをmaterialize
```

legacy JoinModuleを第二planner、第二acceptance truth、または最終pipelineの
別routeとして復活させない。

## Replacement-cell admission rule

新しいreplacement cellは、実装前に次へ答えなければならない。

```text
1. north-starのどの責務／edgeを前進させるか
2. structural / production-reachable / test-only / public-contractの各countは何か
3. censusのstart -> end、includes、excludesは何か
4. named existing production callerはどれか
5. selected new ownerはどれか
6. 同じcommitで削除するold authorityはどれか
7. cutover後のfallback / retry / reselectionが0か
```

`structural_sites > 0`を`production_reachable_callers > 0`へ読み替えない。
production reachが0ならreplacement I0ではなくcaller-zero reconciliation／RET0、
public contractが残るなら明示Decisionを選ぶ。test-only direct injectionは
production acceptanceではない。

最初のproduction replacement rowは
`H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0`である。
`MIRBUILDER-FIRST-PRODUCTION-CUTOVER`はそのrowが満たすmilestone名であって、
第二のswitch taskや別authorityではない。成功時は同じcellでselected legacy
Loop edgeを削除し、fallback/retryを0にする。

次はreplacement cellとして数えない。

```text
production caller = 0 のproof-only owner
old authorityを削除しないadapter追加
別production routeの建設
LOCだけを減らしauthority graphを変えない移動
```

判断は常に次の一問へ戻す。

> この変更は競合するauthorityを一つ消し、production経路を
> `Facts -> Recipe -> Verify -> Lower -> Seal -> Publish`
> へ近づけるか。

Noなら、cell数やLOCが良く見えても選択しない。

## Convergence credit and proof compression

MirBuilder移行の進捗単位は、新しい型・receipt・module・guardの数ではない。
次のproduction graph deltaだけを収束creditとして数える。

```text
named production caller switches to the selected owner
  + selected old edge becomes zero
  + fallback / retry / reselection become zero
```

source/Facts/Recipe/Joinの中間productは、この切替を安全にする証拠であり、
それ自体をfinish lineにしない。selected-normal、canonical、raw、compatibilityが
同じsource formを物理化できる期間はmigration windowである。新ownerのgreenだけで
閉じず、canonical consumer切替と旧consumer退役までを同じordered familyに置く。

専用のmove-only productを恒久化してよい境界は次に限定する。

```text
source authority or semantic ownership changes
physical effect becomes possible
candidate becomes commit/publication eligible
publication authority changes
lifecycle closes
```

同じtransaction内のread-only validation段階、単なるfield projection、テスト専用
canaryはprivate stateへ畳み込めるかを先に検討する。canary／migration receiptを残す
場合は、named production consumer、current caller count、`retire_when`を必須にする。
cutover後はcaller-zeroを確認してpromote、quarantine、またはretireの一つへ閉じる。

`FunctionMetadata`のfact／plan／seedも同じ規律に従う。familyごとに最低限、

```text
production producer owner/count
production consumer owner/count
backend consumer owner/count
last verified consumption revision
retire_when
```

を一つの機械可読inventoryで観測する。`producer > 0`かつproduction/backend
consumerがともに0のrowは性能成果ではなく、caller-zero migration debtである。
新しいbackend consumerはperf owner-first attributionが選んだ一familyだけに追加し、
inventoryだけを理由にoptimizerやfast pathを増やさない。

type authorityも逆流させない。

```text
semantic type
  -> verified physical representation
  -> verified ABI passing class
  -> verified storage layout
  -> one backend physical-type input
```

backendは`MirType`、metadata、ABI manifest、storage planを独立に再結合しない。
この四層を一つのbackend inputへ閉じる設計は
`value-repr-and-abi-manifest-ssot.md`が所有し、source semanticsやABIを同時に
変更するcleanup rowへ混ぜない。

## Completion authority

`MIRBUILDER-FINAL-PIPELINE-v1`全体は、次がproduction graphで成立した
ときに着地する。MirBuilder core単体のMS1-M境界は上のCall completion
programが所有し、backend coverageと物理retirementをcoreの再設計条件へ
逆流させない。

```text
accepted production source families enter one authority pipeline = all
normal/default runner enters one typed canonical source ingress   = yes
normal/default route-selection authority                          = 1
Legacy compile_with_source* production callers                    = 0
family-specific canonical entrypoints as competing prod fronts    = 0
Facts / Recipe / Verify decision authority                       = one each
unverified direct lower                                           = 0
Lower-side AST route redecision                                   = 0
DraftSeal-side Recipe / route redecision                          = 0
physical Return writer on canonical function path                 = 1
CompletedFunctionDraft-only collection                            = yes
partial module publication                                        = 0
fallback / retry / profile reselection                            = 0
canonical rejection -> Legacy retry/fallback                      = 0
selected old production owner / facade / edge                     = 0
full accepted corpus / backend parity                             = green
```

pack counters、replacement ledger、five-cell LOC、source/test measurements
は、このsemantic completionへ到達する過程の観測値である。増減だけで
implementation permissionやcompletionを決めない。

## Final repository convergence finish line

`MIRBUILDER-FINAL-PIPELINE-v1` の完了は Loop の production cutover だけで
終わらない。次の直列順を最終 finish line として固定する。

```text
CANONICAL-FUNCTION-FINISH-TERMINAL-R0
  -> LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0
  -> caller-zero LOOP-PHYSICAL-PREPARE-P0
  -> Generic G0 prepare parity
  -> common physicalizer / caller-zero canary
  -> production selection
  -> M8/M9 coverage and parity
  -> M10b activation
  -> M11/M12 legacy retirement
  -> REPO-FINAL-CONVERGENCE-AUDIT0-G0
  -> repo-physical-structure-cleanup-ssot.md final convergence acceptance
```

最後の cleanup では、pipeline SSOT の一本化、`src/mir` root facade の
durable-only 化、Rust/.hako/compat authority の分類、Context の owner 分離、
`CURRENT_STATE` と設計 registry の収束、temporary proof/receipt/adapter の
promote/quarantine/retire、旧 D4/S-series ledger の archive 化まで確認する。
cleanup は Loop cutover 前に開かず、各実装 row は owning README/reference、
guard index、current mirror を同じ commit で更新する。詳細な row と stop
条件は上記 cleanup SSOT にのみ置き、ここに第二の task ledger は作らない。

## Explicit non-goals

```text
one box = one file / type / trait
new language semantics
new runtime or backend policy
independent second MirBuilder
legacy JoinModule revival
metric-derived architecture
DraftSealでのsource re-analysis
```
