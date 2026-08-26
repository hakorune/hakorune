---
Status: SSOT
Scope: MIR Call target authority, compatibility ingress, and core retirement order
Decision: typed structural Global target selected; implementation remains design-gated
Updated: 2026-08-26
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

- **Current decision:** canonical MIR uses a typed structural Global identity;
  String is accepted only by an owner-private compatibility ingress that
  resolves it once.
- **Current implementation status:** core still stores `Global(String)`,
  `func`, `Option<Callee>`, and optional Method receivers. Ordinary
  `FunctionCall` package completion and Global-family ownership remain open.
- **Next ordered task:** `MIR-CALL-GLOBAL-TARGET-B0-FINITE-IDENTITY-DECISION`;
  it is census/design only and does not authorize a schema or producer change.
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

`CanonicalGlobalTargetV1` is a self-describing, wire-stable structural value,
not a process-local ID. The minimum candidate families are `Builtin` and
`SameModuleStatic { owner, name, arity }`. Imported/free static and runtime
helper families are admitted only if the B0 production census proves that they
need distinct canonical states and names their issuers.

The following are forbidden final states:

```text
GlobalTarget::Legacy(String)
Unknown / Default / sentinel target
ModuleInvocationBrand stored as MIR identity
process-global or consumer-only target registry
physical symbol used as semantic identity
consumer-side StaticMethodId parse, methodize, lookup, or retry
```

JSON-v0 and other retained compatibility owners may parse legacy text, but
must resolve it exactly once to `CanonicalGlobalTargetV1` before constructing
canonical MIR. Invalid explicit v1 input never retries through v0.

## Authority map

| Boundary | Sole authority | Explicit non-authority |
| --- | --- | --- |
| source static call | exact source-site/declaration relation | name/arity formatting, collector order |
| builtin call | finite builtin owner | arbitrary Global String |
| compatibility ingress | owner-private schema plus exact catalog | core legacy variant, fallback parser |
| physical issuer | `MirInstruction::call` storing four fields | classification, lookup, retry |
| optimizer | stored `Callee` and operand remap | `Const(String)` inference, Global→Method |
| interpreter/backend | typed dispatch or projection | register/name/metadata/registry recovery |
| wire egress | projection from stored target | re-creating target class from printed text |

`Callee` owns target ValueId enumeration and rewrite. Call operands are target
operands first, then `args` in stored order, with duplicates preserved. Escape,
ownership, query, and backend ABI reuse this enumeration but retain their own
policy authority.

## Current B0 design stop

Decision:
  Choose typed structural B as the final canonical representation. B0 closes
  its finite family/issuer/wire/projector matrix; it does not change code.

Source authority + canonical issuer:
  declaration/builtin/typed-ingress owners issue a structural target only
  after an exact source-site relation. The exact issuer per Global family is
  still the missing boundary.

Non-authority:
  `CanonicalSameModuleCallableKeyV1` by itself, `mir_symbol_projection`, raw
  name/arity, `ModuleInvocationBrand`, physical symbol, registry lookup,
  `EffectMask`, and existing `Global(String)` consumers.

Fail-fast boundary:
  missing, foreign, duplicate, ambiguous, wrong-namespace, wrong-arity, or
  unsupported Global family terminates before arguments, MIR mutation, wire,
  or backend effects.

Smallest next slice:
  finite read-only census from every production `Callee::Global` issuer through
  wire/optimizer/all compiled core-schema consumers, with selected VM/native
  terminals as semantic parity owners, followed by one accepted B0 matrix.
  Implementation remains false.

Non-claims:
  typed schema implementation, D1B loan, cross-module calls, new builtin
  semantics, PyVM revival, JSON-v0 retirement, or backend expansion.

Census boundary:
  production `Callee::Global` producers -> optimizer/wire/all compiled
  core-schema consumers; includes builtin, same-module static, runtime/helper,
  typed ingress, compatibility projections, and non-selected/WASM mechanical
  adaptation/isolation/retirement. Tests and non-selected backends are not
  semantic authority or new parity targets. PyVM/reference production activation
  and independently typed Extern/Method/Value routes are excluded.

Finite B0 states:

```text
BuiltinReady
SameModuleStaticReady
AdditionalFamilyObserved
CompatibilityTextReady
MissingSourceRelation
ForeignModule
DuplicateOrCollision
AliasUnresolved
WrongNamespace
WrongArity
UnsupportedForWireOrCompiledConsumer
TypedRejectBeforeEffect
ParkedSealedOutsideSelectedBoundary
```

B0 closes only when each in-boundary family has one source authority, one
issuer, one wire owner, one backend projector, and an old-edge disposition.
If a `Legacy(String)` variant, opaque registry, or reparse is required, the row
returns to `NoSafeSlice`.

## Observer/package prerequisite

The Global Decision alone cannot open D1B. Current selected shadow traversal
rejects ordinary `FunctionCall` at the profile gate; a Deferred owner tree does
not issue the semantic package. An external sink after that point would float
without an installable package.

The next design prerequisite is therefore:

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
typed target identity and its source issuers
  -> observer/package completion contract
  -> prerequisite shelves and live-transport disposition
  -> strict schema selection with invalid-v1 retry = 0
  -> typed Global common-core plus all compiled-consumer cutover
  -> observer/package completion implementation and install gate
  -> pre-effect target handoff plus direct-payload deletion
  -> source-owned effect authority for promoted target families
  -> source-backed late recovery retirement
  -> receiver / remaining wire / construction / selected-terminal closure
  -> current-HEAD consumer census
  -> mandatory-Callee schema
  -> impossible-state guard closeout
  -> finite post-Call integration cleanup
```

Producer identity always closes before consumer/schema cleanup. A touched
760+ source is split behavior-neutrally before semantic work. `MirCall` and
reader-zero `CallFlags` retire only through a live-terminal replacement;
isolated JoinIR consumers receive an explicit retire/cutover disposition.

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
- Wire selection: one payload selects one schema. Malformed explicit v1 is a
  terminal error and cannot be silently reinterpreted as v0.
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
```

The 778-line transport must be split behavior-neutrally before it is touched.
`builder.rs` and `unified_emitter.rs` are deletion/delegation-only. New target,
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
