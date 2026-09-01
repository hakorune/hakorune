---
Status: SSOT
Scope: MIR Call target authority, compatibility ingress, and core retirement order
Decision: finite typed structural Global and exact MIR JSON v2 accepted
Updated: 2026-08-28
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
  and the explicit `vm-reference` feature check is green. The MIR Call core
  still stores `func`, `Option<Callee>`, and transitional optional Method
  receivers. Observer facts, forest-wide package admission, FreeStatic
  pre-effect handoff, Main identity/catalog co-seal I0, and the receiver
  crosswalk are landed. The selected-C DeclaredInstance boundary remains
  CoverageMissing: a named launcher chain reaches `LlTextEmitBox.emit_module`,
  but its source ingress is Hako MirBuilder/JSON and no lossless package
  admission ABI or source-backed issuer exists. Its final physical-Recipe and
  transport contract is recorded as a design-only refinement, not an issuer or
  implementation route.
- **Next ordered task:**
  `MIR-BUILDER-EXTERN-ROUTE-SPEC-CATALOG-LOOKUP-BOXSHAPE-S0`, selected by
  `CURRENT_STATE.toml` as the current behavior-neutral fast row. It moves
  only derived lookup code so the physical boundary is clean; after closeout,
  `MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0` remains the parent
  design stop. The manifest also records
  the bounded `MIR-CALL-ME-DECLARED-INSTANCE-HAKO-SEALED-PHYSICAL-CALL-D0`
  refinement: one session-local physical Recipe is permitted only as a
  design contract until an exact Hako owner/caller/ABI tuple exists. It emits
  no target, loan, or Call; Method(Some), backend, and schema cutovers remain
  closed.
- **Production stop line:** no formatter, hidden registry, physical symbol,
  second traversal, post-argument resolver, methodize, or backend repair may
  issue a canonical target.
- **Retirement finish line:** target is decided before argument effects, every
  selected terminal consumes the stored typed target, and `func`, `None`,
  sentinels, retry, and impossible-state guards are absent.

This file is the compact current Call owner. Landed chronology and the full
writer inventories live in Git history, the linked archive, and finite
investigation manifests.

## Final core decision

The final physical issuer is deliberately thin:

```text
typed source producer or owner-private compatibility ingress
  -> exact Callee
  -> MirInstruction::call(dst, callee, args, effects)
  -> stored target projection / execution
```

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
