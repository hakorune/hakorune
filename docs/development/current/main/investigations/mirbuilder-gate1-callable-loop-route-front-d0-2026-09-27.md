# MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-D0

Status: closed — Decision accepted 2026-09-27 (bounded S0 slice emitted)
Parent: workstream row H / Gate-1 internal owner series; follows
`MIRBUILDER-GATE1-ORDINARY-NEW-BIRTH-EDGE-AUTHORITY-D0` (closed — stale
terminal corrected; existing claim authority already emits the typed
`BirthConstructor` edge).

Selected by the family-local action scheduler: with no concrete
`fast`-mode row and no unclassified reds, the only observable frontier is
the parked callable-loop family at
`[freeze:contract][callable-loop/route-not-front-selected]`
— `LoopCondRouteRejected(SourceCallOutsideSelectedFamily)` —
function `BoxTorrentManifest.chunkListText/0`,
site `[Body(4), LoopBody(1), Value, Rhs]` (apps/boxtorrent-mini/main.hako:173,
`out = out + ids.get(i)`).

## Observed terminal and family inventory

`SourceCallOutsideSelectedFamily`
(`normal_callable_loop_source_route.rs:60-66`) is the **designed
unsupported-family terminal**, not a missing-evidence one — the enum
doc says so verbatim. Emission point:
`CallableLoopSourceTargetProbeV1::into_selected_relation`
(`normal_callable_loop_source_route_items.rs:292-345`), which admits a
loop-item batch iff covered by exactly one of:

- `SelectedStatic` — exactly one selected static-publication relation
  covering all `source_items`; requires `ExactI64`/`ExactBool` result
  (items.rs:195-205, 313-316).
- `CoreMethod` — resolver-issued `CoreMethod` rows covering **every**
  `source_items` site (items.rs:317-334).

Uncovered exact static targets → `SourceTargetUnselected`; both arms
empty → `SourceCallOutsideSelectedFamily` with all item sites. The
reject reaches the surface via
`CallableLoopSourceRouteTokenV1::issue_with_source_relations`
(`normal_callable_loop_source_route.rs:257-296`) → `LoopCondRouteRejected`
(`loop_cond.rs:571-582`) → freeze render
(`raw_loop_child_entry.rs:581-585`). `NonGenericOrOverlapping` alone is
the one arm that still delegates to `lower_non_callable_loop_route_v1`
(`raw_loop_child_entry.rs:564-570`) — an armed callable lane never
retries through legacy.

`source_items` enumeration: `ledger.method_calls()` filtered to sites
under the loop's parent site
(`normal_callable_loop_source_route.rs:301-316`).

### Loop-body call census (apps/boxtorrent-mini/main.hako)

| function | loop items | receiver class | family |
|---|---|---|---|
| `chunkListText` (164-177) | `ids.get(i)` | `Lexical(Local)` | builtin `ArrayBox.get/1` — **this D0** |
| `materialize` (179-194) | `ids.get(i)`, `store.readData(cid)` | `Lexical(Local)` ×2 | ArrayGet (this) + user-instance call (parked) |
| `releaseFrom` (196-204) | `ids.get(i)` nested in `store.release(...)` | mixed | ArrayGet (this) + user-instance call (parked) |
| `ingest` (208+) | `data.substring(...)` ✓armed, `store.put(...)`, `manifest.addChunk(...)` | mixed | StringSubstring armed + user-instance calls (parked) |
| `ContentHash.digest` | `text.substring` ✓armed, `alphabet.indexOf(ch)` | `Lexical(Local)` ×2 | StringSubstring armed + `StringIndexOf` (manifest row, not in `allowed_placements` — parked) |

`store.*`/`manifest.*` are user-box instance calls on parameter
receivers — the DeclaredInstance user-call lineage parked by the
owner-selection D5 census; a separate family, not this slice.

## Census — receiver classification

`ids` is `local ids = me.chunk_ids` → resolver receiver shape
`Variable{resolved: Local(binding)}` →
`ResolvedMethodCallReceiverSourceV1::Lexical(Local)`
(`body_shape.rs:517-547` mapping). `Lexical(Local)` is already inside
the admitted receiver vocabulary: the contract issuer accepts it on
owner + `variable_ref` agreement
(`resolver_core_method_callable_contract.rs:217-237`), and the physical
consumer `take_source_core_method_call`
(`source_call_publication.rs:36-120`) accepts `Lexical(Local)` receivers
under `named_array_requirement().is_none()`. **Receiver class is not
the gate; the op family is.**

Unaliased `me.chunk_ids.get(i)` would be receiver `FieldAccess` →
`Other` → `UnsupportedReceiver` — stays outside.

## Census — which existing authority can own `ids.get(i)`

The route-neutral catalog has exactly three target arms
(`VerifiedSourceCallTargetV1::{Static, DynamicMember, CoreMethod}`):

- **Static** — covers `QualifiedStatic`/`CurrentOwnerStatic` spellings
  only. `ids.get` is neither → structurally unreachable.
- **DynamicMember** — requires `Lexical(Local)` receiver **with a
  dynamic origin** (`dynamic_member.rs:180-193`). `origin_for_binding`
  (`normal_callable_dynamic_source.rs:135-143`) answers only for
  *formals* and *locals initialized from a formal*. `ids` is
  initialized from a `FieldAccess` (`me.chunk_ids`), not a formal copy
  → **no DynamicMember row can ever exist for `ids.get`** — structural
  non-answer, not a policy gap.
- **CoreMethod** — `ArrayBox.get/1` **has** a generated manifest row:
  `core_op: ArrayGet`, `result_kind: Dynamic`, `effect: pure_read`,
  `lowering_tier: warm_direct_abi`,
  `cold_lowering: nyash.array.slot_load_hi`. The vocabulary exists;
  the issuer family excludes the op today (below).

So the only family that can semantically own `ids.get(i)` is
**CoreMethod**. The freeze is the bounded-arm boundary, not a missing
product category.

## Census — why the CoreMethod arm drops `ArrayBox.get/1` today

One issuance chain (`issue_source_bound_core_method_calls_with_named_arrays_v1`,
`named_array_method.rs:44-158`) produces every CoreMethod row:

1. Text rows first via `issue_source_bound_core_method_calls_v1`
   (`core_method.rs:67-152`) — manifest lookup **hardcodes `"StringBox"`**
   (line 77); `allowed_placements` admits only `(StringLen,0)→[Condition]`
   and `(StringSubstring,2)→[Body,Condition]` (lines 161-170).
   `ArrayBox.get` never reaches the lookup.
2. ArrayBox rows — loops `method_calls()` with `"ArrayBox"` manifest
   lookup but `if manifest.op != CoreMethodOp::ArrayPush { continue; }`
   (lines 58-66). `ArrayGet` is dropped unconditionally.

Downstream mirrors also lack the op: `allowed_target_placements` +
result-relation mirror in `resolver_core_method_callable_contract.rs`
cover StringLen/StringSubstring/ArrayPush only;
`CoreMethodInstanceTargetIssuerV1` schema arms are `string_box_text` /
`array_text_append` / `array_integer_append`
(`core_method_instance_target.rs:97-113, 156-232`); the consumer's
`result_type` mapping knows `I64ToCaller`/`TextToCaller`/`NoValue`, not
`Dynamic`.

## Census — the receiver-box evidence requirement

`get/1` is **ambiguous**: both `ArrayBox.get/1` (`ArrayGet`) and
`MapBox.get/1` (`MapGet`) exist in the manifest at arity 1. Selector +
arity cannot pick the op — minting a contract by selector alone would
be guessing semantic meaning, which is prohibited. A `get` contract
requires **receiver-box source evidence**.

The honest authority already exists in the ArrayBox issuer's inputs:

- `detect_field_residence_claim` (`named_array_residence.rs:152-234`)
  proves `local a = <object>.<field>` alias claims: single
  `initializer_relations` row, initializer shape `FieldAccess`,
  `me`-rooted object → `NamedArrayFieldResidenceObjectV1::Receiver`.
  `local ids = me.chunk_ids` satisfies exactly this.
- `constructors.field_declaration` (named_array_method.rs:194-205):
  declared field type must be `None` or `"ArrayBox"` — `chunk_ids` is
  an untyped `init` field → `None` → admitted for provider proof.
- `resolve_birth_provider` (named_array_method.rs:242-244): the owning
  box's `birth` ledger must contain the provider site
  `me.chunk_ids = new ArrayBox()` (main.hako:134) — **the construction
  evidence that proves the field is an ArrayBox field** (boxtorrent
  fields carry no declared types; the birth store is the proof).
- `verify_field_residence_relations` (named_array_residence.rs:240-267):
  `ReassignedReceiver` (no `BindingRebind` of `ids`), `ValueDemand`,
  single `arity==1` argument, `argument_is_integer_source` — `i`
  qualifies (`local i = 0` integer literal + all rebinds `i = i + 1`
  are integer-source arithmetic, named_array_residence.rs:353-454).

For `push` this same chain then seals
`NamedArrayRequirementV1::FieldResidence` — a **write obligation**
product (artifact retention/provider identity) consumed by the
retention arm. `get` is `pure_read`: it needs the *receiver evidence*
but minting the write requirement would misstate semantics and route
the consumer to `named-array-artifact-retention-required`
(`source_call_publication.rs:90`). The read arm therefore issues a
**plain contract** (`ResolverCoreMethodCallableContractIssuerV1::issue`,
same as the StringBox arm) gated by the residence evidence at issue
time.

## Decision

`ids.get(i)` is a CoreMethod-family call — builtin `ArrayBox.get/1` —
admitted through the **named-array ArrayBox issuer arm** extended with
a bounded read branch, using field-residence receiver evidence (the
same authority chain `push` uses) without minting a write-requirement
product.

```text
Decision:
  Admit ArrayBox.get/1 (pure_read, LoopBody placement only) as a plain
  CoreMethod contract inside the existing
  issue_source_bound_core_method_calls_with_named_arrays_v1 ArrayBox
  arm, gated by field-residence receiver evidence; clears chunkListText.
Source authority + canonical issuer:
  ledger.method_calls() exact site + detect_field_residence_claim
  (single FieldAccess initializer, me-receiver) + field_declaration
  (declared type None-or-ArrayBox, non-weak) + resolve_birth_provider
  (me.chunk_ids = new ArrayBox() construction proof) +
  verify_field_residence_relations (no rebind, integer-source arg)
  -> CoreMethodInstanceTargetIssuerV1 ArrayGet schema arm ->
  ResolverCoreMethodCallableContractIssuerV1::issue -> CoreMethod
  catalog row -> CallableLoopSourceItemDispositionV1::CoreMethod.
Non-authority:
  DynamicMember (ids has no formal-derived origin — structural);
  Static (spelling is neither QualifiedStatic nor CurrentOwnerStatic);
  selector-only lookup (get/1 is ambiguous vs MapBox.get/1 — never mint
  without receiver-box evidence); NamedArrayRequirementV1 products
  (write obligations — pure_read must not mint them); declared-type
  lookup as sole evidence (fields are untyped — birth construction is
  the proof); RawLegacyChildLoweringPortV1 (no claim authority).
Fail-fast boundary:
  Missing residence evidence / non-Receiver object / non-Body
  placement / non-Lexical receiver -> continue ->
  SourceCallOutsideSelectedFamily (existing terminal). Never fall back.
Smallest next slice (S0):
  MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-S0 —
  single op (ArrayGet,1): schema arm + ABI profile + DynamicToCaller
  result relation in core_method_instance_target.rs; verify-mirror
  placement + result arms in resolver_core_method_callable_contract.rs;
  bounded ArrayGet branch in named_array_method.rs issuing a plain
  contract; Dynamic result-type mapping in take_source_core_method_call.
Non-claims:
  No MapBox.get; no user-instance calls (store.*/manifest.* — parked
  DeclaredInstance family); no StringIndexOf; no DynamicMember
  consumer; no Condition placement; no Construction-alias receivers
  (local a = new ArrayBox(); a.get); no unaliased me.chunk_ids.get;
  no Gate-1 or app completion claim.
```

## Consequences recorded for S0

- `chunkListText` loop clears completely (single item `ids.get(i)`).
- `materialize`/`releaseFrom` keep stopping — `store.readData` /
  `store.release` user-instance calls remain `OutsideSelectedFamily`.
- `ingest` keeps stopping on `store.put`/`manifest.addChunk`.
- `ContentHash.digest` keeps stopping on `alphabet.indexOf`
  (`StringIndexOf` — manifest row exists, placement arm absent).
- `take_source_core_method_call` needs a `DynamicToCaller` → MIR type
  arm (`MirType` choice is an S0 detail; the consumer's unconditional
  arm is already correct for `named_array_requirement().is_none()`).
- `ids` itself is a field-residence alias — `read_variable(receiver_site)`
  at the call site reads the current local; `ReassignedReceiver` keeps
  the residence evidence honest.

## Boundary of this D0

起点: `local ids = me.chunk_ids` + `loop(i<n){ ... ids.get(i) ... }`
→ 終点: `VerifiedSourceCallTargetV1::CoreMethod` row consumption in
`LoopCond` physical lowering. Includes: probe arms, receiver classes,
issuer/mirror/consumer chain, boxtorrent loop-call inventory.
Excludes: user-instance call family design (parked), StringIndexOf
placement (parked), DynamicMember loop consumers (structurally
unreachable for field-alias receivers), Main/top-level lane.
