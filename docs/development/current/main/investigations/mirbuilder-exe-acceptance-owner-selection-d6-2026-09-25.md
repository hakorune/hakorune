# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D6

Status: decision__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-COND-UPDATE-S0
  (landed — `ConditionalUpdateIf` located-source parts arm through the
  existing `try_lower_conditional_update_if_input` facade; json advanced
  past `loop-cond-item-unsupported`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D5 selection lineage.

## Problem

LOOP-COND-COND-UPDATE-S0 landed: json_stream_aggregator now lowers the
`ConditionalUpdateIf` item in `JsonStreamAggregator.ingest` and stops at
a new named terminal:

```text
[freeze:contract][callable-semantic-lowering/incomplete-consumption]
owner=FunctionOwnerIdV1 { compilation: 1, slot: 27 } entry=true
locals=3/3 variables=11/12
missing_variables=[SourceNodeSiteV1([Body(2), LoopBody(2), Receiver])]
assignments=2/2 lambdas=0/0 loop_break_transport_kind=None
```

`LoopBody(2)` is `me.ingestLine(stream.substring(start, end))` — the
`me` receiver site inside the loop body was never consumed by any
lowering product. Select the next bounded design slice.

## Fresh class map (receipt after LOOP-COND-COND-UPDATE-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`seedBlocks`), binary_trees (`iterationCheck`), mimalloc_lite (`seedBlocks`) — D5-sealed forks |
| `callable-semantic-lowering/incomplete-consumption` (Receiver site) | 1 | json_stream_aggregator (`JsonStreamAggregator.ingest`, LoopBody(2) `me.ingestLine`) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt` type error (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

## Questions

1. Which contract owns the `Receiver` site of a `me.method(...)` call
   inside a loop body — the same DeclaredInstance receiver-consumption
   lane (parked `MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0`),
   or a distinct consumption-ledger gap in
   `callable-semantic-lowering`?
2. How did the singleton-anchor route carry `me.ingestLine`? Per D5 the
   singleton contract admits one selected static call (`JsonLine.find`)
   without verifying remaining items — was the expression statement
   issued as an item (and which kind) or never issued, leaving its
   `Receiver` variable site unconsumed?
3. Does the consumption audit expect every `Receiver` site to be
   consumed by a lowering product, or is the audit itself missing a
   receiver-consumption record that the located dispatcher should emit
   when it lowers a `me.method` item?
4. Bounded candidates (order only — each needs its own census):
   - G1: receiver-site consumption for loop-body `me.method` items
     (the json blocker).
   - F3b: `UnconsumedSelected` publication consumption on the
     singleton LoopCond route.
   - F3c: `SourceItemsMissing` call-free admission.
   - coverage forks: B3-ArrayPush and ordinary-instance-call coverage
     (both D5-sealed; reopen only through their own cards).

## Boundary

- Includes: owner census for the `Receiver`-site consumption terminal;
  the item-issuance question for uncovered call statements under a
  singleton anchor; relation to the parked DeclaredInstance lane; one
  Decision with a six-line brief or sealed NoSafeSlice.
- Excludes: implementation; reopening the D5-sealed coverage forks;
  backend toolchain; inference panic family; NamedArray family.

## Exit

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.

## Census answers (main-thread source reads)

### Q1 — who owns `Receiver`-site consumption?

`CallableSemanticLoweringState::finish_with_named_arrays`
(`named_array.rs:30-60`) requires every entry in `variables` — built
from `owner.variable_refs()` Local bindings
(`normal_callable_semantic_lowering_state.rs:234-245`), which includes
each `me` read site — to appear in `consumed_variables`. Two
consumption owners exist today:

- Raw lane: `take_declared_instance_receiver_value_inner_v1`
  (`recursive_child_lowering.rs:448-493`) → locator
  `take_exact_relation` → `take_exact_receiver_value`, consuming the
  `Receiver` node site once (the binding stays reusable).
- Located path: `CallableLoopSourceExpressionPortV1::
  exact_source_variable_value`
  (`normal_callable_loop_source_port.rs:348-365`) accepts
  `Variable | Me | This` expression inputs and calls
  `ledger.read_variable(&site)`, which records consumption and returns
  the materialized value.

Neither is invoked for `Me`/`This` MethodCall *receivers* in the
located path: both the statement lowering
(`lower_method_call_statement_input`,
`loop_body_lowering_associated_input.rs:243-254`) and the value lowering
(`helpers_value/lower.rs:339-353`) carve `Me | This` receivers out to
`lower_me_this_method_effect` (`normalizer/common.rs:37-103`), which
resolves `me` by name through `variable_map`/`phi_bindings` and never
touches the site ledger. That carve-out is the gap.

### Q2 — how did `me.ingestLine` get this far?

The statement was issued as a normal `Stmt` item and lowered — all
argument variable sites (`stream`, `start`, `end` inside the nested
`substring` call) are consumed, which is why the audit reports
`variables=11/12` with exactly one `Receiver` site missing. The
singleton-anchor route (`into_selected_relation`) never verifies the
remaining items; item issuance is independent of call coverage.

### Q3 — what does the audit expect?

Every variable site consumed exactly once (`read_variable` rejects
duplicates as `duplicate-variable-consumption`; unregistered sites are
`missing-variable-site`). The located `Me`/`This` receiver carve-out is
therefore a real consumption-contract gap, not an audit bug — the same
`Me` site IS consumed when it appears as a standalone value
(`lower.rs:31-41` routes through `exact_source_variable_value`) or as a
`FieldAccess` object.

Relation to the parked DeclaredInstance lane: the parked card covers
*selected-C admission* — CanonicalInstance target-key emission plus
backend coverage. Site consumption is a smaller, separable contract:
the emitted `CoreEffectPlan::MethodCall` already carries the materialized
receiver; the audit only asks that the source site be accounted. This
slice does not reopen the parked admission lane.

Caveat for the S-slice: `This` receivers in static boxes may carry no
registered `Receiver` site (`variable_refs` only records Local
bindings). The bounded fix must keep the existing `GlobalCall` path for
unregistered `This`/static-box receivers and consume the site only when
the ledger registered one — a `source_read_binding`-style existence
check or a port method returning `Ok(None)` for unregistered sites is
required; hard-failing on `missing-variable-site` would break static
`this.method`.

### Decision

```text
Decision: select G1 — consume the located `Me`/`This` MethodCall
receiver site through the source port in the loop item lowering paths
(statement `lower_method_call_statement_input` and value-position
`lower.rs` MethodCall arm).
Source authority + canonical issuer: resolver `variable_refs` Local
bindings (Receiver site); consumption owner = the located source port's
ledger read (`read_variable`/`exact_source_variable_value` shape).
Non-authority: DeclaredInstance selected-C admission (parked), call
coverage contract, recipe issuance, backend emission kind.
Fail-fast boundary: registered site consumed via the ledger read;
unregistered `This`/static-box receivers keep the existing GlobalCall
path (no new fallback — the check is site registration, not name).
Smallest next slice: one receiver-consumption arm inside the existing
`Me | This` carve-outs, with positive (me.method statement + value in
loop) and negative (unregistered-this / duplicate-consumption) pins and
the json_stream_aggregator receipt.
Non-claims: no coverage relaxation, no DeclaredInstance admission, no
recipe/Facts change, no backend coverage claim beyond current
`MethodCall` emission.
```

Follow-on inventory recorded, not selected (unchanged from D5):

- F3b `UnconsumedSelected` publication consumption on the singleton
  LoopCond route.
- F3c `SourceItemsMissing` call-free admission.
- Coverage forks: B3-ArrayPush (sealed D2), ordinary-instance-call
  coverage (no issuer; parked DeclaredInstance lineage).
