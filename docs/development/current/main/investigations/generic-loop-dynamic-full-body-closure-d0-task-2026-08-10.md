---
Status: Dynamic dispatch D0 closed; neutral MethodCall source relation I0 next
Date: 2026-08-10
Row: `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-D0`
Parent: `generic-loop-source-backed-dynamic-carrier-d0-task-2026-08-09.md`
Mode: BoxShape / compiler acceptance repair
---

# Generic Dynamic Loop full-body closure

## Decision

The former next row, `GENERIC-LOOP-DYNAMIC-VM-CANARY-I0`, is
`NoSafeSlice` as currently worded.  P2A/P1R/P2B/P2C prove one canonical
Dynamic carrier cycle and whole-session discard, but they do not lower or
publish the complete unchanged source method:

```hako
skip_while(src, pos, end, pred_chars) {
    local i = pos
    loop(i < end) {
        local ch = src.substring(i, i + 1)
        if pred_chars.indexOf(ch) < 0 { return i }
        i = i + 1
    }
    return i
}
```

The compiler acceptance boundary must be widened.  The source must not be
annotated, copied into a smaller fixture, or rewritten to fit the existing
carrier proof.

## Exact gap

The closed P2 rows own only:

```text
source-backed Dynamic Enter
-> Header provisional PHI
-> Dynamic Compare / Add
-> terminal Backedge
-> canonical Header seal and PHI patch
-> whole unpublished-session discard
```

They deliberately leave unowned:

```text
substring call
local ch
indexOf call
inner comparison and If
inner early return
zero-iteration After
fallthrough After
final return
Completion
DraftSeal / CompletedFunctionDraft
collector / module publication
callable reachability and VM execution
```

Therefore a backend capability marker cannot make P2 executable.  There is no
completed draft or module to mark, collect, or run.

## Authority correction

The final compiler path is:

```text
exact unchanged callable source
-> complete source/body relation set
-> complete Dynamic Loop Recipe
-> Recipe verify / JoinSig / source co-seal
-> full-program physical demand
-> one canonical function session
-> all Call / local / If / exit / carrier operations exactly once
-> one sealed Loop After
-> all source returns merge at one physical function exit
-> Completion consumes one merged explicit result
-> finish_for_draft_seal
-> DraftSeal prepare / commit
-> CompletedFunctionDraft
-> ModuleDraftCollector
-> passive Dynamic backend capability
-> shared backend preflight
-> VM canary
```

No profile-specific whole-body lowerer, route-local PHI writer, raw Loop
retry, or post-effect fallback may be introduced.

## Existing owners to reuse

Reuse, without duplicating their meaning:

```text
Loop After:
  resolved_lowering/loop_recipe_physicalizer/recursive_after.rs
  prepare_recursive_after_v1(...).emit_and_seal

function terminal:
  CanonicalSsaFunctionSessionV2::finish_for_draft_seal

DraftSeal:
  draft_seal_owner.rs open -> prepare -> commit

publication:
  ModuleDraftCollectorV1::prepare_callable_batch
  PreparedCallableCollectorBatchV1::collect_all[_branded]
  duplicate policy = CanonicalRejectDuplicate
```

The current `tail_completion.rs::consume_callable_tail_completion_v1` is
precedent, not a reusable terminal for this method.  It owns one Tail and one
explicit return.  `skip_while/4` has an inner return and a final return.

## Full source authority inventory

The top-down census closes the D0 question as follows:

| Source meaning | Existing authority | Missing compiler authority |
| --- | --- | --- |
| four formals and `pos -> i` initialization | callable source ledger + Dynamic source/origin products | none for identity/origin |
| Loop membership/frame | resolver Loop membership and semantic context | none |
| `i < end`, `i = i + 1` | bounded Dynamic operation source products | full Recipe-neutral operation coverage |
| `src.substring(...)` / `pred_chars.indexOf(ch)` | exact syntax sites and BindingRefs | source-bound Dynamic dispatch contract |
| local `ch` | declaration/initializer site, BindingRef, iteration-local classification | exact call-result-to-local initialization relation |
| inner If | resolved If region/control inventory | Loop-owned AST-free If Facts and JoinSig transfer capability |
| inner/final Return sites | `VerifiedFunctionCompletionV1::ExplicitReturns` | physical two-site merge and merged consumption receipt |
| complete body structure | `ResolvedFunctionBodyShapeProductV1` | one atomic full-body semantic coverage product |
| After / DraftSeal / collector | common canonical owners listed above | only their full-program ingress/transfer prerequisites |

The final source product belongs in the neutral source/Facts pipeline, not in
`src/mir/builder`. The existing Builder Dynamic products remain bounded
migration evidence and later adapters.

```text
src/mir/compiler/                 exact source observer/projection
src/mir/loop_structural_facts/    atomic full-body Facts
src/mir/loop_recipe_contract/     deterministic Recipe/source co-seal
src/mir/builder/                  physical consumption only
```

## Exact `ch` rule

`ch` keeps its resolver-issued declaration/BindingRef/source-read identity,
but it is not a carrier and does not need a mutable Binding-SSA slot. The
Recipe producer may map the `substring` result to one Recipe-local SSA value
only after co-sealing the exact local-value relation:

```text
declaration BindingRef/site
+ substring CallSlot result key
+ exact lexical read site
+ zero rebind / no escape / same iteration scope
-> local-value relation
-> indexOf CallSlot argument
```

`ch` is `IterationLocal`, never a Loop carrier. Unsupported reassignment or
escape stops before Recipe issuance. Facts retain roles/sites/BindingRef;
only the producer assigns the Recipe value key.

## Next I0 product boundary

`GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-I0` issues a source inventory, not a
target-complete semantic program:

```text
VerifiedDynamicLoopFullBodySourceInventoryV1
  exact callable owner/root/provenance
  exact formal rows: src / pos / end / pred_chars
  exact prelude row: local i = pos
  exact Loop membership and condition source tree
  exact body rows in source order:
    local ch initializer MethodCall source
    inner If condition MethodCall + comparison source
    inner Return source
    i rebind source
  exact outer Return source
  exact BindingRefs for i/ch and every lexical read/write
  total statement/expression coverage receipt
```

The product is non-`Clone`, AST-free, and has no Recipe keys or physical IDs.
It preserves both MethodCall source rows without claiming a target/result
contract. It preserves both Return rows without duplicating the existing
`VerifiedFunctionCompletionV1` set authority.

I0 rejects before Builder effect:

```text
missing / duplicate / foreign source row
wrong callable or Loop root
wrong parser/resolver provenance
reordered direct body row
unknown extra statement/expression
ch declaration/initializer/read mismatch
i declaration/read/rebind mismatch
Return row not present in the canonical Completion set
```

## Multi-return rule

`VerifiedFunctionCompletionV1` already owns the exact set of two explicit
source returns. The missing boundary is physical consumption:
`ResolvedFunctionCompletionConsumptionV1` currently accepts only one site and
rejects a return set larger than one. Do not duplicate source Completion,
emit multiple physical `Return` instructions, or let a legacy early-return
writer bypass DraftSeal.

The new common contract must:

```text
all exact source Return sites
-> one function-exit block
-> one result incoming row per reachable Return
-> one merged result (PHI when required)
-> one Completion claim
-> one physical Return written by DraftSeal
```

Missing, duplicate, foreign, unreachable-as-reachable, or class-incompatible
return rows reject before publication.  Failure discards the complete
unpublished function session.

## Recipe boundary

Dynamic Recipe work moves before VM activation. The current V2 wire already
has `CallSlot`, `If`, `Exit(Return)`, `Text`, and structural verification, but
it has no Dynamic value class, Dynamic Add/Less operations, source-bound
Dynamic dispatch relation, V2 JoinSig/Core/demand, or production physical
consumer. V1 must not be widened ad hoc. The D0 must decide the smallest
profile-neutral V2 extension and complete source-bound relations before I0.

The exact full-body coverage includes:

```text
inputs: src, pos, end, pred_chars
carrier: pos -> i
condition: i < end
body call: src.substring(i, i + 1)
body local: ch
body call: pred_chars.indexOf(ch)
body branch: result < 0
branch exit: return i
body rebind: i = i + 1
outer tail: return i
```

An exact declared target is not honest for these source-backed Dynamic
receivers. The one source-call target catalog must gain a route-disjoint
`DynamicMemberTarget`-style variant, issued from exact call site, receiver
BindingRef/Dynamic lineage, selector spelling, and checked arity. The selector
is the source/runtime message identity, not a Box/type/result classifier.
Recipe and physical emitters never look up names. Until this issuer exists,
the row is `NoSafeSlice`; no declaration target is fabricated and no legacy
method-call writer is used as fallback.

## Exact task order

### 1. `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-D0` — closed

This census fixes every existing/missing owner, the `ch` local-value rule, the
Dynamic dispatch prerequisite, and the physical Return-consumption gap.

### 2. `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-I0` — closed

Issue one non-`Clone`, AST-free source inventory for the unchanged method:
formals, `i`, `ch`, both call sites, If, both returns, rebind, and exact total
source coverage. This row owns source identity only; target, Recipe, and
Builder remain nonclaims. Missing/duplicate/foreign rows reject with zero
Builder effect.

Implementation receipt (2026-08-10):

```text
src/mir/compiler/dynamic_full_body_source.rs
src/mir/compiler/dynamic_full_body_source_tests.rs

6 exact binding roles
28 exact statement/expression roles
2 Return sites retained through the existing Completion product
0 Recipe keys
0 Builder/MIR effects
```

The focused matrix accepts the unchanged production file, rejects an added
Loop statement instead of narrowing the source, rejects a different selector
instead of reclassifying it as this canary shape, and rejects a foreign
Completion owner. Selector spelling is source-shape evidence only; it issues
no semantic target or result type.

### 3. `SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-D0` — closed

Separate route-disjoint Dynamic message identity/source rows from the later
execution envelope. Owned by
`source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`.

### 4. `RESOLVED-METHOD-CALL-SOURCE-RELATION-I0`

Add one reusable AST-free resolved MethodCall row with exact call/receiver/
result sites, checked selector/arity, and complete ordered argument sites.
Profile-specific full-body roles are not promoted into universal authority.

### 5. `SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-I0`

Issue exact caller-zero source-call target rows for `substring/2` and
`indexOf/1`. No nominal receiver fabrication, name-based semantic classifier,
Builder lookup, retry, or fallback.

### 6. `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0/I0`

Define and issue one selector-independent conservative effect/Fault/
suspension/Home contract. The Dynamic result lifetime is the hard prerequisite;
no Pure, NonSuspending, Trivial result, or other permissive default is allowed.

### 7. `LOOP-V2-DYNAMIC-VALUE-AND-CALL-RELATION-D0`

Fix profile-neutral `Dynamic`, explicit Dynamic Add/Compare semantics,
CallSlot target/source relations, and the `ch` local-value relation. Do not
overload I64 operations, widen V1, or infer Dynamic from `MirType::Unknown`.

### 8. `LOOP-V2-DYNAMIC-VALUE-AND-CALL-RELATION-I0`

Produce and verify all Dynamic value/operation/CallSlot relations from the
complete source product and dynamic target catalog, still with zero Builder
effect.

### 9. `LOOP-V2-JOINSIG-RETURN-D0/I0`

Extend the common V2 Core/semantic-program co-seal so the inner
`If(Return i, fallthrough)` has JoinSig-authorized true-to-FunctionExit and
false-to-resume transfers; Loop predicate false reaches After, and outer Tail
reaches FunctionExit. Recipe owns structure; JoinSig alone owns transfer
meaning.

### 10. `LOOP-V2-FULL-PREFLIGHT-I0`

Produce the complete V2 Recipe plus source/input/item/carrier/Loop relations,
then preflight every item exactly once. Expose no first/select/filter API.

### 11. `LOOP-COMMON-V2-CONTROL-PHYSICAL-I0`

Bind Recipe-derived segment placement to opaque JoinSig transfers and lower
Call/Dynamic operations/If/Return through canonical CFG, Binding SSA, and
PhiTxn services. Layout never infers Return or After destinations.

### 12. `GENERIC-LOOP-DYNAMIC-AFTER-TAIL-P2D`

Close normal zero/nonzero Loop After and issue the outer final-return
candidate. The inner Return bypasses After. A partial carrier-only session is
not accepted.

### 13. `GENERIC-LOOP-DYNAMIC-MULTI-RETURN-COMPLETION-D0/I0`

Project the existing exact `ExplicitReturns` set to one canonical function
exit, merge reachable result candidates (one PHI when needed), define Dynamic
return representation/ABI projection, and consume Completion exactly once.
DraftSeal remains the only physical Return writer.

### 14. `GENERIC-LOOP-DYNAMIC-DRAFT-COLLECT-P2E`

Run `finish_for_draft_seal`, DraftSeal prepare/commit, and canonical collector
batch admission. Produce one completed callable draft with exact symbol/arity
and no direct module insertion.

### 15. `GENERIC-LOOP-DYNAMIC-BACKEND-CAPABILITY-I0`

Install passive capability only from verified Dynamic physical completion.
The shared backend gate accepts `mir-interpreter`; unsupported backends return
one stable no-fallback error. No MIR scanning or Builder backend policy.

### 16. `GENERIC-LOOP-DYNAMIC-VM-CANARY-I0`

Switch one exact method-level production caller, collect it, and execute the
unchanged production source. Test zero-iteration, inner-return, and normal
fallthrough cases. In the same cutover series remove the selected legacy edge;
`Err -> legacy`, raw-loop re-entry, retry, and duplicate publication are zero.

## Production switch and retirement

The eventual exact switch is before the legacy function owner opens:

```text
RootCallableCapturePortV1::lower_cataloged_static_box_method
```

It consumes the exact semantic loan and draft admission, opens one canonical
function session, lowers the complete body, seals it, and collects it.

Do not switch at:

```text
RawLoopChildEntryPortV1::lower_loop
raw_loop_child_entry::lower_with_existing_route_v1
```

Those seams are already inside the legacy function owner and lack callable
Completion authority.

In the exact canary switch commit, remove the selected call to
`lower_normal_cataloged_static_box_method_with_source_v1`.  For that callable
there must be no `Err -> legacy` fallback, raw-loop re-entry, duplicate draft,
duplicate PHI, retry, or direct module insertion.  Other legacy GenericLoop
callers remain until their own cutover.

## Acceptance matrix

```text
coverage:
  exact unchanged source accepted
  missing / duplicate / foreign item rejected before Builder mutation

control:
  zero iterations reaches outer Tail
  inner If return bypasses Loop After
  fallthrough backedge reaches Header and later After

completion:
  two exact source returns -> one exit/result -> one Completion claim
  one physical Return only

atomicity:
  failure after any physical effect discards child, restores caller once,
  and publishes zero drafts

publication:
  exact callable key/symbol/arity collected once

backend:
  no capability is neutral
  MIR interpreter accepts Dynamic capability
  unsupported backend rejects in shared preflight with no fallback

VM:
  tracked wrapper calls the production method unchanged
  zero-iteration / inner-return / fallthrough results are correct
```

## Nonclaims after source I0

```text
no source rewrite or narrowing
no copied simplified skip_while
no Dynamic semantic target or Recipe implementation
no CallSlot target invention
no Builder / MIR mutation
no After consumption / DraftSeal / collector
no backend metadata
no VM execution
no production switch
no retry / fallback / legacy deletion
```

## Same-slice documentation rule

Each implementation row updates its owner README and public reference in the
same commit.  At minimum the series updates:

```text
src/mir/builder/README.md
src/mir/builder/resolved_lowering/README.md
src/mir/builder/resolved_lowering/canonical_cfg/README.md
src/mir/builder/ssa/binding/README.md
the GenericLoop owner README
docs/reference/mir/generic-loop-stage-matrix.md
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/metadata-facts-ssot.md
docs/tools/check-scripts-index.md when a guard is added
```

All new source files stay below 800 lines.  Split at one owner boundary before
760 lines rather than after the limit is exceeded.
