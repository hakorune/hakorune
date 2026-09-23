---
Status: design_stop__b3_source_type_effect_and_coseal_mapping_open
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2
Parent: mir-call-parser-array-push-b3-branch-continuation-d1-2026-09-23
NextCard: same-card__resolve_b3_carrier_effect_and_coseal_mapping
Implementation permission: false; design/taskification only. Do not edit compiler code, fixtures, receipts, or production selection until the source/effect/co-seal mapping is settled.
---

# LoopCond Recipe carrier relation D2

## Six-line brief

```text
Decision: continue the selected B3 migration for StringHelpers.split_lines/1 through the current LoopCond/GeneralIf owner.
Source authority + canonical issuer: imported declaration identity and same-function resolver ledger; extend source projection and one semantic-program issuance.
Non-authority: method names as selectors, post-hoc AST/MIR pairing, physical phi/name maps as source identity, and unrelated GenericLoop receipts.
Fail-fast boundary: exact i/last BindingRefs, branch join, ArrayPush/substring operation rows and complete consumption reject before physical allocation.
Smallest next slice: D2 fixes B3 operation/type/effect and Recipe/JoinSig carrier mapping; then connect it to the current sole LoopCond/GeneralIf physical owner.
Non-claims: no physical traversal proven, production switch, old-edge deletion, tail push/function-return completion, serializer, or backend parity.
```

## Corrected entry premise and target choice

The former requirement for an already-connected LoopCond issuer was circular.
The [entry policy](../design/current-docs-update-policy-ssot.md#implementation-entry-and-retirement-conditions)
allows naming owners and consumers to implement/connect; implementation,
tests, production switch and caller-zero are migration outputs. A shared
MethodCall owner may remain while the selected old responsibility is retired.

Read-only source-route audit selected B3 over the proposed call-free scalar
profile for this migration. The scalar profile would be a new callable
admission shape: it has no natural source caller in the observed static-child
cohort, and `normal_callable_loop_source_route.rs` rejects empty source-call
relations before the existing LoopCond physical consumer. It cannot prove
retirement of the live B3 source boundary. M8 S6D producer/observation remains
its own ordered workstream row; it is not a prerequisite invented for B3.

The exact natural source target is
`lang/src/shared/common/string_helpers.hako::StringHelpers.split_lines/1`.
The imported static box method batch enumerates every declared method without
runtime call-use filtering. Its production compiler path is:

```text
ProgramDeferredStaticBoxLifecycleV1::lower_normal_with_port_v1
 -> PreparedNonMainStaticBoxMethodBatchV1::lower_root_with_port_v1
 -> NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method
 -> canonical route / source lowering
 -> LoopCond source entry
```

Evidence seams: `src/mir/builder/program_root_lowering.rs`,
`nonmain_static_box_method_batch.rs`, and
`normal_callable_semantic_loan_port.rs`. The source declaration has the exact
LoopCond body: outer carriers `i` and `last`; local `ch`; `s`/`n` reads; a
nested `ArrayPush(s.substring(last, i))` in the `ch == "\n"` branch; and
`last = i + 1` in that branch. The post-loop tail push and returned `arr` are
outside the selected loop row.

This compiler caller is real even though repository search finds no runtime
call to `StringHelpers.split_lines/1`: the static method batch compiles declared
members from imported boxes. `lower_cataloged_static_box_method` has a leading
Dynamic semantic path and then the Ordinary canonical route; the exact B3
source's selected semantic disposition still needs an observed receipt and
must not be guessed from the Rust type name.

## Current terminal and planned old responsibility

The natural-import test
`normal_default_root_catalog_merged_route_tests::merged_parser_program_source_stops_at_named_publication_boundary`
asserts `SourceCallOutsideSelectedFamily` for
`function=StringHelpers.split_lines/1`, source site
`[Body(5), LoopBody(1), IfThen(0)]`. A read-only route trace identifies that
site as the nested `arr.push(...)` statement. It also finds three method-call
sites in the loop inventory: `s.substring(i, i+1)`, the push, and its nested
`s.substring(last, i)` argument. `source_core_method_items` keeps only sites
present in the package-issued CoreMethod map; if this named rejection is
observed, the uncovered row is the push, not evidence of a missing substring
contract. The test assertion is not a fresh dynamic observation: the actual
current first terminal and Dynamic-versus-Ordinary admission remain unverified.
Do not claim that the physical LoopCond consumer was reached or rewrite the
assertion as a physicalization receipt.

The intended existing physical owner remains the LoopCond/GeneralIf chain:
`raw_loop_child_entry` → `loop_cond_bc_source` → `lower_loop_cond_source_item`
→ `lower_if_join_source` / `lower_if_join_state_core` and the current loop
carrier materializer. D2 must specify how the one source-bound semantic
issuance reaches that owner; it must not create another PHI/Join solver.

Planned selected delete-set, after that connection is implemented and selected:
for `split_lines/1`'s admitted loop, remove the AST/name-based outer-carrier
discovery, branch join name-to-BindingRef recovery, and its
`SourceCallOutsideSelectedFamily` rejection/re-entry for the exact nested
ArrayPush/substring source rows. Preserve the raw/shared LoopCond and
MethodCall owners for other callers. The exact current source route and terminal
must be checked at implementation entry; whole-file/shared-writer deletion is
not implied.

## Exact source authority and mapping still required

| Relation | Authority and required treatment |
| --- | --- |
| Function/method membership | imported static declaration identity, selected ordinary callable admission, same resolver function owner and source ledger; resolve Dynamic-vs-Ordinary before claiming this route |
| `i`, `last` | exact resolver BindingRefs: `i` declaration/read/condition/increment across header and backedge; `last` declaration/read, branch write, implicit-else incoming value, loop continuation and after value |
| `ch` | exact loop-body local and comparison read; kill at iteration boundary; never export as outer carrier |
| `s`, `n`, `arr` | exact parameter/local BindingRefs and roles. `s`/`n` read-only; `arr` is mutated only by the selected exact ArrayPush row |
| substring calls | source call sites, receiver/argument relations, selected result type and Fault/effect contract; preserve evaluation and failure behavior on both `ch` read and push argument |
| ArrayPush | named-array source row tied to exact `arr`, exact nearest-loop/IfThen statement, substring text argument and operation result/effect; never select by method spelling alone |
| GeneralIf | then branch maps `last` to `i+1`; implicit else maps `last` to incoming value; one Recipe-keyed join output returns to the same `last` BindingRef |
| loop continuation | key relations for `i` and `last` at header, body, step/backedge, break/after and final state; all source rows consumed once |

D1 already fixed branch-state semantics: reset only current values and active
origins at branch entry; reads, assignments, calls, locals, consumed source rows,
emission ports and historical origins are monotonic because both branches are
compiled once. Do not roll back the whole callable ledger.

### Existing operation authority and the remaining relation gap

The exact static operation owners exist. The package issuer
`src/mir/normal_callable_semantic_package/core_method_source.rs` invokes
`src/mir/source_call_target/core_method.rs` and
`named_array_method.rs` with the brand catalog. The ArrayPush issuer ties an
`ArrayBox.push` to nearest-loop Body placement, an exact constructed named-array
requirement, retained Text argument, and the manifest ArrayPush/1 target. The
manifest projection in `src/mir/core_method_result_kind.rs` gives these source
operations:

| source operation | existing result/effect authority | placement in `split_lines` |
| --- | --- | --- |
| `s.length()` | StringLen/0: I64Value, PureRead, CodePointCount | before loop; its `n` result is a loop-invariant input, not a loop call row |
| `s.substring(i, i+1)` | StringSubstring/2: StringValue, PureRead, CodePointHalfOpenClamped | LoopBody local `ch` initializer |
| `s.substring(last, i)` | same StringSubstring/2 contract | IfThen push argument |
| `arr.push(text)` | ArrayPush/1: NoValue, MutatesShape, TextRetainedByReceiver | one nested IfThen statement, with exact `arr` construction requirement |

The named-array issuer may currently omit a row on
`NotNamedArray`, `DeclarationCollision`, `InitializerMissing`,
`ConstructionMissing`, or `UnsupportedReceiver`. Static inspection cannot say
which, if any, applies to the merged parser's live resolver ledger. The existing
straight-line issuer test proves only its own positive shape, not this nested
IfThen site. At the selected B3 boundary, the source inventory and exact `arr`
BindingRef must either co-seal the issued push row or end at a named rejection;
an optional omission must not turn into a weaker coverage check. The first
implementation-entry observation must capture the actual issuer rows and
current terminal before changing code, and reconcile the existing test
assertion if it is stale.

These contracts prove source operation meaning at their issuing owner. They do
not yet bind the loop-contained call rows, their values, `i/last/ch/arr`
BindingRefs, the branch join, and loop continuation to one portable loop
program. In particular, ArrayPush shape mutation is not a carrier rebind, and a
PureRead effect label alone is not a Fault/outcome proof. The B3 source row
must carry any exact call outcome/failure contract too; missing Fault authority
cannot be replaced by “no fault” inferred from result/effect kind.

The typed LoopRecipe V2 schema already expresses CallSlot, If, Loop,
Text/Dynamic values and externally bound call outcomes. `CallSlot` permits a
missing value result, which matches ArrayPush's NoValue shape, but the current
portable producer is profile-limited. Extend that existing Recipe/Core/JoinSig
owner to issue B3's several call sites and two outer carrier relations. The
separate DynamicFullLoop fault-cut owner requires exactly two result-bearing
calls and cannot be borrowed unchanged for the resultless mutating push. The
bounded design must say which existing call-outcome owner is extended to cover
both substring results and ArrayPush mutation/failure before issuing the
co-seal.

`SourceLoopCondPhysicalInputV1` still transports planner Facts/Recipe, forest,
source-call rows, one target relation and a plain source port; it has no
co-sealed carrier-key relation, operation/effect Core or JoinSig continuation.
The package rows are currently projected into `take_source_core_method_call`
and `take_source_array_push` by the source port. D2/D3 must define how those
same source-issued rows enter one semantic issuance and how that same issued
relation reaches the existing LoopCond/GeneralIf physical owner. No second
wire, reconstructed contract, name lookup, or detached authority in the
physical input.

Facts may carry resolver BindingRefs, exact source sites, source operation
contracts and complete coverage only. Recipe producers issue binding/carrier/
value/item keys. The semantic-program issuance must co-seal the same source
context, source-bound Core, operation/effect relations, Recipe and that Core's
JoinSig continuation before physical lowering. No name, MIR ValueId, BlockId or
EffectMask inference. Missing source type/effect/Fault issuer is an internal
design task, not an external wait or permission to mint default Pure.

## Ordered tasks and observable exits

| Task | Work | Done evidence |
| --- | --- | --- |
| D1 — caller and asserted boundary | Closed by source audit for caller only: imported `StringHelpers.split_lines/1` enters the non-Main cataloged-static-method batch. The existing natural-import test asserts a named stop at the nested push; current dynamic first terminal and Dynamic/Ordinary admission are not re-observed. | Exact declaration/caller; test assertion distinguished from an executed receipt; physicalizer success unclaimed. |
| D2 — carrier and operation mapping (next) | Fix resolver-owned roles/sites for `i`, `last`, `ch`, `s`, `n`, `arr`; exact substring and ArrayPush typed operation/effect sources; branch/loop continuation keys and residual policy. For the nested push, design required row-or-named-reject behavior across the issuer's five soft omissions; I1 will observe which actual resolver disposition occurs. Name a source-backed Fault/outcome owner for the resultless mutation; do not infer non-faulting from result/effect. | Full finite source inventory and assignment rows mapped; result/effect/Fault authority named; push coverage explicit; no implicit else, optional omission, or source row left unclassified. |
| D3 — one semantic issuance | Name existing source-bound Core and Recipe/JoinSig issuer seams; co-seal source context, carrier relations, branch join, loop continuation and operation/effect rows before any physical ID. | One owner and exact input/output tuple, duplicate/foreign/shadow/missing/residual rejection boundary, selected physical consumer relation. |
| I1 — connect selected LoopCond/GeneralIf owner | At entry, observe actual Dynamic/Ordinary admission, package-issued source rows, and first terminal on the natural imported source. Then extend current physical owner to consume Recipe keys/relations; preserve the raw facade for unrelated shapes. If the test's expected stop is stale, reconcile it before using it as a baseline. | Natural imported source reaches existing publication through current sole physical owner; positive/negative evidence; no AST/name re-discovery or weakened coverage. |
| I2 — selected cutover and retirement | After mandatory M8/M9/M10 production-entry prerequisites, switch this exact static-method loop membership to the co-sealed path; remove its old selection/rejection/re-entry edge in the same bounded series. | No selected-membership fallback; selected old responsibility caller-zero; shared owners retained only for named remaining callers. |
| C — closeout | Record dynamic receipt, guards, module README/reference and pointer; classify reds. | Terminal evidence, selected-edge deletion and required acceptance recorded; loop-only success is not full function or serializer completion. |

The concrete internal design work D2/D3 is available now. The exact downstream
function terminal and return of `arr` remain a separate owner boundary: this
card claims only the selected loop relation. The post-loop tail push is excluded
until its own source statement owner and effect contract are selected; do not
silently drop it or count it as loop completion.

## Acceptance to be specified for implementation

- Natural imported `StringHelpers.split_lines/1` source from declaration through
  the existing selected function publication terminal, with the nested push
  and branch join consumed by the exact selected loop owner.
- Compare zero, one and multiple newline paths, including implicit else
  preservation of `last`; check resulting array contents once function-tail
  handling is in the selected acceptance scope.
- Alpha-renamed locals preserve behavior; nested If/exit identities and
  `ch` iteration scope match exact resolver sites.
- Missing/duplicate/foreign/shadow binding, wrong branch/loop target, substring
  result mismatch, ArrayPush row mismatch, mixed co-seal and any unconsumed
  source/effect row reject before physical allocation.
- Selected membership failure is terminal and cannot fall back into the old
  name collector; unrelated supported LoopCond/MethodCall shapes remain live.

## Non-claims

No current physicalizer success, production switch, old-edge deletion, full
static-box/library caller-zero, tail push, function-return completion, Text or
Array serializer, backend parity, or overall MirBuilder completion is claimed.
M8 S6D, M8 S6E/S6G and later M9/M10 obligations remain under their owner SSOTs.
