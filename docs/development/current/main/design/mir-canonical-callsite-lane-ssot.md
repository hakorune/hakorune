---
Status: SSOT
Scope: MIR Call target authority, compatibility ingress, and core retirement order
Decision: finite typed structural Global and exact MIR JSON v2 accepted
Updated: 2026-09-02
Related:
- docs/development/current/main/CURRENT_STATE.toml
- docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
- docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
- docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1-manifest-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-final-shape-and-ingress-boundary-design-2026-08-25.md
- docs/development/current/main/design/archive/mir-canonical-callsite-lane-history-2026-08-25.md
---

# MIR Canonical Callsite Lane

## Current Capsule

- **Current decision:** canonical Global is `Builtin(Print)` or same-module
  `FreeFunction`/`StaticBoxMethod`; canonical MIR JSON is exact v2.0.
- **Current implementation status:** the B1 structural Global carrier is landed
  and the explicit `vm-reference` feature check is green. R6 Group A now
  separates canonical `MirInstruction::Call(MirCall)` from the explicit
  compatibility `LegacyCallV0` shape; the remaining writer/backend migration
  and R7 deletion are not complete. Transitional optional Method receivers and
  compatibility readers still exist at the outer boundary. Observer facts,
  forest-wide package admission, FreeStatic
  pre-effect handoff, Main identity/catalog co-seal I0, and the receiver
  crosswalk are landed. DeclaredInstance relation, effect, result/Completion,
  full signature, and receiver crosswalk are ready through the package boundary.
  The accepted future direct-method identity is the existing source-catalog
  `CanonicalSameModuleCallableKeyV1`; it is not yet retained as an exact
  published Method target/definition relation. A lossless product-backend
  family is also absent. The daily ny-llvmc Boundary remains a
  live JSON/name/registry compatibility consumer, not a semantic issuer or an
  exact DeclaredInstance physical owner. Direct package-to-Hako emission remains
  forbidden; backend admission begins only after canonical module publication.
- **Current bounded task:**
  `MIR-CALL-R6-CURRENT-HEAD-RECENSUS-C0` is the current design-stop row. Group A
  is closed at `45c6759962`; perform one finite read-only census of remaining
  Call writers/readers and compatibility boundaries before opening R6 Group B.
  No new issuer, receipt, adapter, fixture, fallback, resolver, schema variant,
  or production switch is permitted during this census. The readiness
  projection is `SemanticPackageReady=yes`, `ReceiverCrosswalkReady=yes`,
  `PublishedCanonicalCallReady=group-dependent`, and `EndToEndVerticalReady=no`.
  selected-C is a downstream typed consumer/RetireAfterReplacement lane, not a
  source semantic issuer or prerequisite. Do not create a selected-C source
  issuer, second resolver, family loan, fixture, receipt, adapter, or fallback.
- **Production stop line:** no formatter, hidden registry, physical symbol,
  second traversal, post-argument resolver, methodize, or backend repair may
  issue a canonical target.
- **Retirement finish line:** target is decided before argument effects, every
  selected terminal consumes the stored typed target, and `func`, `None`,
  sentinels, retry, and impossible-state guards are absent.

## Accepted canonical publication-spine rebuild (2026-09-02)

The remaining family audits exposed one shared loss point rather than a need
for more source receipts: the normal catalog admission already owns
`CanonicalSameModuleCallableKeyV1`, but converts it to `LegacySymbol`, and the
normal collector then publishes only a String-keyed function. A single
branch/worktree-isolated in-place rebuild repairs that spine. It is not a
second MirBuilder or a second source-to-physical pipeline.

The former gate required a complete typed backend before opening the task that
creates that backend. That gate was self-blocking and is retired. I0/I1 start
from one exact source half plus named publication, runner, and backend seams.
R0 alone requires the newly completed typed backend tuple and exact cutover
edge. Failure aborts the bounded branch; it does not create another D0.

The first source proof is StaticCurrentOwner `me.method`, while the canonical
backend cohort is the published `StaticBoxMethod` key namespace. Source
spelling is intentionally absent after publication: `me.target()` and an
already-verified qualified static call that select the same key are one
backend family, not two target authorities.

```text
source/catalog Facts and Recipe (existing authority)
  -> existing mandatory typed target before arguments
  -> normal collector preserves key and atomically publishes its definition
  -> PublishedMirBackendView<'m> (borrow-only, no source/package access)
  -> versioned typed C view -> one selected-C physical consumer
```

`MirModule` owns the one key-to-definition relation. The backend view borrows
it and owns no AST, resolver, catalog clone, name index, registry, retry state,
or independently issued meaning. At the FFI boundary Rust layout is never
exposed; a versioned `repr(C)` flat arena is valid for one call only. Physical
`EffectMask` may be transported as an existing MIR field, but this cohort does
not claim to issue a new source-backed semantic effect.

The stored repository-wide Call schema is now split into canonical
`MirInstruction::Call(MirCall)` and explicit `LegacyCallV0`; this Group-A shape
change did not delete the legacy family. The published view still admits both
shapes for compatibility while selected family reachability is reduced.
Global `LegacyCallV0` deletion remains R7 after the remaining writers/readers
are migrated; changing every compatibility consumer at once would mix the
vertical cutover with an unrelated mass compile break.

Before any backend attempt, the facade chooses exactly one state:

```text
selected StaticBoxMethod present + whole module supported -> CanonicalTyped
selected StaticBoxMethod present + any unsupported shape -> UnsupportedBeforeObject
selected StaticBoxMethod absent                         -> ExplicitCompatibility
```

Typed failure never retries JSON. Environment flags cannot choose two routes
for the same selected module. The compatibility implementation may remain for
unselected families, but the switched family has zero reachability to JSON,
name/registry lookup, and `args[0]` repair.

### Temporary-red contract

The fixed rewrite may be compile/test-red only in its isolated branch or
worktree. The accepted parent receipt remains immutable (`7553/7386/138/29`
plus the failure-name and inventory hashes). The branch records its immutable
parent SHA before code and never merges or rebases. Every temporary failure is
an exact test-name delta with owner, reason, successor, expiry, and a finite
state: `ExpectedCompileBreak`, `ExpectedTestMigration`, `Unexpected`, or
`Resolved`. At most one consecutive compile-red commit is allowed. The window
ends at the earlier of five rewrite commits or seven calendar days; expiry or
one `Unexpected` failure aborts instead of extending the design. `main` is not
updated while an unclassified migration-red remains. `#[ignore]`, deleting a
red test, or rewriting the parent baseline is forbidden.

### Product completion definition

The first cohort is complete only when `NyashRunner::execute_mir_mode
--emit-exe` uses the typed facade for every selected module and the same series
deletes the selected family's reachability to its old edges:

```text
published key -> exact definition            = 1
borrowed mandatory-call view + typed C consumer = 1 / 1
callee=None / Method(None) in selected view  = 0 / 0
name/registry/header target lookup           = 0
JSON/name/args[0] selected-family reachability = 0
fallback/retry from the canonical caller     = 0
named caller switched                         = 1
```

Shared old helpers earn no deletion credit while compatibility callers remain.
Temporary cohort-only adapters, tests, docs, and guards are repaid in-series;
the existing `apps/tests/me_method_call.hako` proof is simplified/reused rather
than adding another fixture file.

### Fixed task series

`MIR-CALL-CANONICAL-PUBLICATION-SPINE-STATIC-BOX-METHOD-I0` is a fixed series:
(1) preserve the neutral key through Atomic Publish; (2) add the borrow-only
view and finite coverage admission; (3) add the versioned flat C ABI and one
minimal scalar typed consumer; (4) switch the named `--emit-exe` edge and make
the old route unreachable for selected modules; (5) prove source-to-EXE,
compare exact red deltas, repay temporary surface, and close out. Commit five
cannot contain a semantic fix. Commit four not switching the production edge,
or any need for JSON/name repair, aborts the branch without another D0.

This task does not claim DeclaredInstance support, selected-C arbitrary
UserBox support, source semantic-effect issuance, whole Call-schema cutover,
all-backend parity, VM retirement, selfhost, or a green whole library. It is
the finite route to the first product vertical. After it lands, remaining
Global/Method/Value/Extern/Terminal families migrate in bounded family batches,
then compatibility quarantine and R6/R7 delete the physical legacy schema.

This file is the compact current Call owner. Landed chronology and the full
writer inventories live in Git history, the linked archive, and finite
investigation manifests.

## Final core decision

The final physical issuer is deliberately thin:

```text
typed source producer or owner-private compatibility ingress
  -> exact Callee
  -> MirInstruction::call(dst, callee, args, effects)
  -> Atomic Publish of canonical MirModule
  -> selected backend projection / execution
```

No backend may receive source/package products around Atomic Publish. A future
Hako emitter may replace the selected backend only through a borrowed,
lossless view of the published canonical MirModule; it may not become a second
source-to-physical pipeline.

The target and Call shapes converge to:

```rust
enum Callee {
    Global(CanonicalGlobalTargetV1),
    Method {
        receiver: ValueId,
        box_name: String,
        method: String,
        certainty: TypeCertainty,
        box_kind: CalleeBoxKind,
    },
    Value(ValueId),
    Extern(String),
}

Call {
    dst: Option<ValueId>,
    callee: Callee,
    args: Vec<ValueId>,
    effects: EffectMask,
}
```

For DeclaredInstance, the current `box_name`/`method` fields are a transitional
carrier and physical projection, not a published callable-definition identity.
The accepted backend-neutral exact direct-method key is the existing
source-catalog-issued `CanonicalSameModuleCallableKeyV1`:
`InstanceBoxMethod + owner + method + source arity`. Call/R6 must retain that
same key and atomically relate it to the published function definition. Moving
the type to a shared MIR vocabulary location is representation neutral; it does
not create a second issuer. `resolved_semantics::CanonicalCallableKeyV1` is
FreeStatic-only, `FunctionDraftKeyV1` is routing vocabulary, and a module-local
opaque/backend ID is not introduced.

The 2026-09-02 finite audit also closes the tempting key-only intermediate.
The normal cataloged path currently drops the source key to `LegacySymbol`, and
the collector can validate `CatalogedBoxMethod` before publication, but
`MirModule` stores functions by physical `String` and has no post-publish key
consumer. Therefore key retention by itself is consumer-zero and may not land.
The next implementation requires the key/definition relation, one named
callsite consumer, one lossless backend family, and its finite old-edge delete
set in the same bounded series.

`CanonicalGlobalTargetV1` is a self-describing structural value in the
serde-free `hakorune_mir_defs` crate:

```rust
enum CanonicalGlobalTargetV1 {
    Builtin(CanonicalBuiltinGlobalV1),
    SameModule(CanonicalSameModuleGlobalTargetV1),
}

enum CanonicalBuiltinGlobalV1 { Print }

enum CanonicalSameModuleGlobalTargetV1 {
    FreeFunction { name: Box<str>, arity: u32 },
    StaticBoxMethod { owner: Box<str>, method: Box<str>, arity: u32 },
}
```

`Print` means exact `print/1`. No `Imported`, `RuntimeHelper`, `Generated`, or
`Legacy` variant exists. An imported alias becomes SameModule only after an
exact declaration exists in the final module; otherwise it rejects before
arguments. JoinIR will co-seal its generated declaration and call as a
same-module free function. Exact bare `panic/1` is outside the Call lane: its
accepted target is the semantic-reserved terminal Fault route through the
common exit transaction, and production activation remains 0. Bare `exit/1`
has no accepted issuer in this Decision and is not inferred as Extern. Bare
`error` and `now` reject; only an explicit declared
`env`/`nyash.console.error` provider may issue Extern. Math is Method,
construction is `NewBox`/`NewClosure`, and the current bounded GC Global
producer retires without activating GC semantics.

The following are forbidden final states:

```text
GlobalTarget::Legacy(String)
Unknown / Default / sentinel target
ModuleInvocationBrand stored as MIR identity
process-global or consumer-only target registry
physical symbol used as semantic identity
consumer-side StaticMethodId parse, methodize, lookup, or retry
```

B1 installs the only semantic-to-physical projection as a one-way operation
under `resolved_semantics/callable_symbol`; `MirModule` then owns one structural
target lookup over its function storage. The cutover deletes or delegates
`CanonicalSameModuleCallableKeyV1::mir_symbol_projection`. There is no inverse
parser. A collision in physical projection rejects the module batch before
publication.

Canonical MIR JSON uses exact `schema_version = "2.0"`. Its Global target is a
family-tagged object, `args` and `effects` are required, and `func`, `flags`,
aliases, missing effects, duplicate/unknown/out-of-order effect names are
rejected. Exact v1.0 and v0 remain owner-private compatibility inputs and must
resolve text once before canonical MIR. Invalid explicit schema input never
retries through another parser. The reference child has a separate accepted
CanonicalV1-to-private-payload design; it is not a Wpre schema variant.

Wpre parses one JSON root and classifies it totally within the shared runner
family-unknown boundary. Exact `schema_version` `2.0` selects canonical v2,
exact `1.0` selects compatibility v1, schema-less MIR-v0 requires the exact
functions/blocks shape, and `version=0, kind=Program` belongs only to the
Program artifact owner. Explicit schema plus functions/blocks is validated by
that decoder, not treated as a root conflict. Mixed legacy markers, any other
version, or malformed/unsupported shape reject. Until B1 installs the v2 codec,
an explicit v2 payload reaches a typed parser-unavailable terminal, never v1/v0.

The root owner uses one recursive duplicate-aware serde parse to produce one
`serde_json::Value`; selector and selected decoders consume that value rather
than reparsing raw text. Stage1 arbitration, force-hv1, selfhost, runtime/
kernel, reference, LLVM, observer, and C-ABI paths have separate owner/fate
rows and are not silently folded into this shared Wpre boundary.

B1 v2 is a bounded Call-corridor profile, not a claim that all current emitter
ops round-trip. Its exact op set is:

```text
const copy copy_owned destroy_owned newbox field_get binop compare
branch jump phi ret mir_call
```

Any other op is an unsupported-v2 terminal. `mir_call` has one flat shape with
required `dst` (integer or null), `callee`, `args`, and `effects`; nested
`mir_call`, `func`, `flags`, aliases, and duplicate placements are rejected.
The exact Global target objects are:

```text
{type:Global,target:{family:builtin,builtin:print}}
{type:Global,target:{family:same_module,kind:free_function,name,arity}}
{type:Global,target:{family:same_module,kind:static_box_method,owner,method,arity}}
```

Effects use the exact `EffectMask::effect_names()` order from `pure` through
`barrier`; missing, duplicate, unknown, out-of-order, or unprojectable bits
reject. Full MIR-v2 vocabulary belongs to a separate future owner.

## Authority map

| Boundary | Sole authority | Explicit non-authority |
| --- | --- | --- |
| canonical type | `hakorune_mir_defs/global_target.rs` | serde, String wrapper, registry ID |
| source static call | exact source-site/final-module declaration relation | name/arity formatting, alias map, collector order |
| builtin call | exact source `print(expr)` / `Print` issuer | arbitrary Global String or classifier list |
| compatibility ingress | owner-private schema plus exact catalog | core legacy variant, fallback parser |
| physical issuer | `MirInstruction::call` storing four fields | classification, lookup, retry |
| optimizer | stored `Callee` and operand remap | `Const(String)` inference, Global→Method |
| interpreter/backend | typed dispatch or projection | register/name/metadata/registry recovery |
| module lookup | one `MirModule` structural-target lookup using one-way callable-symbol projection | per-consumer format/parse |
| wire parse/egress | one runner-owned v2 codec, defs remain serde-free | re-creating target class from printed text |

`Callee` owns target ValueId enumeration and rewrite. Call operands are target
operands first, then `args` in stored order, with duplicates preserved. Escape,
ownership, query, and backend ABI reuse this enumeration but retain their own
policy authority.

## Accepted B0 architecture

Decision:
  Typed structural B is accepted with the three shapes above. String is only
  compatibility input; B1 now carries the structural form through the
  selected MIR consumers.

Source authority + canonical issuer:
  exact print/source-declaration/generated-declaration or owner-private ingress
  issues one structural target before arguments.

Non-authority:
  `CanonicalSameModuleCallableKeyV1` by itself, `mir_symbol_projection`, raw
  name/arity, `ModuleInvocationBrand`, physical symbol, registry lookup,
  `EffectMask`, and legacy text consumers.

Fail-fast boundary:
  missing, foreign, duplicate, collision, wrong namespace/arity, unsupported
  family/schema/consumer terminates before arguments or effects.

Smallest next slice:
  Reference-child private transport I0. The reusable ingress-schema guard is
  extended only for the listed child/monitor files; shared-runner selection stays
  closed until one parsed Value, strict duplicate-key ownership, decoder
  signatures, and outside fates agree.

Non-claims:
  selector implementation, typed schema implementation, D1B loan, cross-module
  calls, new builtin semantics, PyVM revival, v0 retirement, backend expansion.

Census boundary:
  production `Callee::Global` producers -> optimizer/wire/all compiled
  core-schema consumers; includes builtin, same-module static, runtime/helper,
  typed ingress, compatibility projections, and non-selected/WASM mechanical
  adaptation/isolation/retirement. Tests and non-selected backends are not
  semantic authority or new parity targets. PyVM/reference production activation
  and independently typed Extern/Method/Value routes are excluded.

Finite B0 states:

```text
BuiltinPrintReady
SameModuleFreeFunctionReady
SameModuleStaticBoxMethodReady
CompatibilityTextReady
MissingSourceRelation
ForeignModule
DuplicateOrCollision
AliasUnresolved
WrongNamespace
WrongArity
UnsupportedForWireOrCompiledConsumer
TypedRejectBeforeEffect
ExternOrMethodOrConstructionOwner
```

The architecture is accepted from six independent read-only audits plus
adversarial review at HEAD `9bff1a1ff2`. B1 readiness remains task-gated until
the finite producer and compiled-consumer rows below close. A new family,
physical parse-back, hidden registry, or legacy core variant reopens design.

## Accepted observer/package transition

The Global Decision alone cannot open D1B. Current selected shadow traversal
rejects ordinary `FunctionCall` at the profile gate; a Deferred owner tree does
not issue the semantic package. An external sink after that point would float
without an installable package.

The accepted replacement is:

```text
profile-gate-adjacent observer-only FunctionCall branch
  -> record existing source site / name / arity
  -> observe arguments in the same traversal
  -> issue no Callee and no canonical direct-call target
  -> allow owner observation to complete
  -> require total disposition before package install
```

It may not widen callable semantics, create a second AST walk, use body effects
as target evidence, or publish scratch outside the package lifecycle. Existing
brand/site/catalog identity is reused unless a concrete mispair proves it
insufficient; a new resolver-session receipt is not created speculatively.

## Durable retirement order

The exact current task tokens and their selected order belong only to the
rolling workstream. This SSOT fixes the durable dependency shape:

```text
finite typed identity and observer/package contract (accepted)
  -> reusable ingress lifecycle guard
  -> finite explicit-CLI arbitration and outside-fate closure
  -> reference child isolation and CoreDirect typed terminal policy
  -> force-hv1 finite census and selected retirement
  -> strict recursive root owner and by-value decoder seams
  -> strict one-shot schema selection with cross-parser retry = 0
  -> MirCall/CallFlags transport retirement
  -> source-owned effect authority
  -> preserve exact free/static/import/compatibility target relations
  -> JoinIR declaration co-seal and false GC Global retirement
  -> observer/package completion implementation and install gate
  -> builtin/Extern disposition and all-lineage late recovery retirement
  -> pre-effect affine handoff plus direct-payload deletion
  -> receiver ABI, Method(None), methodize, optimizer/VM recovery retirement
  -> all remaining static/CorePlan/operator producer dispositions
  -> exact touched-owner shelf and finite current-HEAD B1 census
     (new hole -> owner S0/R0 -> rerun census)
  -> typed Global common-core plus all compiled-consumer cutover
  -> remaining wire / construction / selected-terminal closure
  -> current-HEAD consumer census
  -> mandatory-Callee schema
  -> impossible-state guard closeout
  -> finite post-Call integration cleanup
```

Producer identity always closes before consumer/schema cleanup. A touched
760+ source is split behavior-neutrally before semantic work. `MirCall` and
reader-zero `CallFlags` retire only through a live-terminal replacement;
JoinIR first deletes generated-name/alias fallback and co-seals exact generated
function identity while retaining one guarded old physical publication until
B1 types it. The current false bounded-GC Global publication, arbitrary String
issuers, late recovery, Global-to-Method repair, and authority-free formatted/
CorePlan/env-gated publishers all retire before B1. C0 may open B1 only when its
inventory is exhausted after every discovered owner-specific remediation reruns.

The active queue is:

```text
docs/development/current/main/workstreams/
  mirbuilder-inplace-replacement-current.md#ordered-frontier
```

## Cross-cutting contracts before R6

- Receiver ABI: `Callee::Method.receiver` is the semantic receiver;
  `Call.args` contains source arguments only. A backend may project its ABI
  once, but Builder and VM may not add then strip the same receiver.
- Effect authority: changing target transport must preserve the source-owned
  effect decision. Cataloged `READ` and unified Global `IO` defaults are an
  open parity conflict, not permission to choose the weaker value.
- Construction: Call is invocation. `NewBox` and `NewClosure` remain
  construction owners; invoking a closure value uses `Callee::Value`.
- Wire selection: parse the JSON root once, then select exact canonical v2,
  compatibility v1.0, MIR-v0, or Program-v0. Malformed, conflicting, and
  unsupported markers are terminal and never enter another parser.
- Affine lifecycle: owned target is taken before argument descent and its
  borrow ends first. Success requires `finish_empty`; failure uses a typed
  abort without overwriting the primary lowering error.

## Source budget and safe placement

Do not append semantic code to these current owners:

```text
src/mir/builder.rs                                      741
src/mir/builder/raw_invocation_source_transport.rs      778
src/mir/builder/normal_callable_semantic_loan_port.rs   710
src/mir/builder/raw_expression_dispatch/mod.rs          706
src/mir/builder/calls/unified_emitter.rs                 711
src/mir/string_dead_text_region_plan.rs                  790
src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs 744
```

The 778-line transport must be split behavior-neutrally before it is touched.
The 790-line dead-text owner has one exact call-shape split before B1.
`builder.rs`, the 744-line lowerer, and `unified_emitter.rs` are
deletion/delegation-only. New target,
inventory, handoff, and loan code belongs in small owner-specific siblings.
Every touched/new source remains below 760 lines; 800 is a hard stop.

## Acceptance and non-goals

Positive parity compares typed target, target operands, args order, dst,
source-owned effects, and selected execution result. String spelling, numeric
`func`, printer output, and compatibility fixtures are not semantic proof.

Return to `NoSafeSlice` if implementation needs a legacy String variant,
opaque/global registry, second traversal/resolver, post-argument target search,
optional/empty or cloneable loan, physical symbol parsing, receiver duplication,
or semantic additions to a 760+ source.

PyVM/reference production activation, non-selected backend activation/parity,
performance work, JSON-v0 removal, Loop/M8/M9 activation, broad crate splitting,
warning cleanup, and general dead-code retirement remain outside this Call
decision. Compiled Rust core-schema consumers, including WASM/non-selected
consumers, remain inside B0/B1 mechanical disposition.

## Reusable evidence

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_row_guard.sh --only mir-call-d1b-targeted-variant-split
bash tools/checks/run_row_guard.sh --only mir-call-d1b-cataloged-affine-loan-lifecycle
bash tools/checks/mir_call_d1b_selected_normal_duplicate_projection_guard.sh
git diff --check
```

Cargo gates run only in an accepted fast/closeout row. Historical Call
chronology is preserved in Git and
`design/archive/mir-canonical-callsite-lane-history-2026-08-25.md`; neither is
current scheduling authority.
