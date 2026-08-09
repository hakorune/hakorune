---
Status: active design stop
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

## Multi-return rule

`ResolvedFunctionCompletionConsumptionV1` currently consumes one explicit
return.  Do not emit multiple physical `Return` instructions and do not let a
legacy early-return writer bypass DraftSeal.

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

Dynamic Recipe work moves before VM activation.  The current V2 wire already
has `CallSlot`, `If`, `Exit(Return)`, `Text`, and structural verification, but
it has no Dynamic value class and no production physical consumer.  The D0
must decide the smallest profile-neutral extension and complete source-bound
relations before I0.

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

`CallSlot` targets must come from an exact source-bound callable target
authority.  Box/method-name matching inside the Recipe producer or physical
emitter is forbidden.  If the canonical target issuer cannot express either
call, that row stops `NoSafeSlice`; it does not fall back to the legacy
method-call writer.

## Exact task order

### 1. `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-D0`

Inventory every source item and identify its existing source, target, control,
effect, and result owner.  Fix compiler-side missing observations.  Decide
whether `ch` is a Recipe-local value or an explicit binding relation.  No
Builder effect.

Done when the positive and negative coverage matrix and every `NoSafeSlice`
issuer are explicit.  This is the current row.

### 2. `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-I0`

Issue one non-`Clone`, AST-free complete body product for the unchanged
method.  It covers all calls, locals, branch/returns, carrier reads/writes,
and outer Tail.  Missing/duplicate/foreign rows reject with zero Builder
effect.

### 3. `GENERIC-LOOP-DYNAMIC-RECIPE-D0`

Fix the profile-neutral Dynamic value/operation semantics, exact CallSlot
target relations, If/Return exits, source roles, JoinSig transfers, and
continuation compatibility.  Do not widen V1 ad hoc or infer Dynamic from raw
`MirType::Unknown`.

### 4. `GENERIC-LOOP-DYNAMIC-RECIPE-I0`

Produce and verify the complete Recipe and all source/input/item/carrier/Loop
relations.  Preflight the whole program; expose no first/select/filter API.

### 5. `GENERIC-LOOP-DYNAMIC-AFTER-CLOSE-P2D`

Physically consume the whole verified program in one canonical session and
reuse the common recursive After owner.  Prove zero-iteration, nonzero
fallthrough, and early-return bypass.  A partial carrier-only session is not
an accepted input.

### 6. `GENERIC-LOOP-DYNAMIC-MULTI-RETURN-COMPLETION-D0`

Define the common source-return set, single-exit merge, result PHI/class
contract, and sole Completion consumption.  Keep Loop continuation, Callable
Tail, Return ABI, and Completion distinct.

### 7. `GENERIC-LOOP-DYNAMIC-MULTI-RETURN-COMPLETION-I0`

Merge the inner and final `return i` paths into one exit/result and consume
Completion once.  Prove exactly one eventual physical Return and reject all
partial/foreign return sets.

### 8. `GENERIC-LOOP-DYNAMIC-DRAFT-COLLECT-P2E`

Run `finish_for_draft_seal`, DraftSeal prepare/commit, and canonical collector
batch admission.  Produce one completed callable draft with the exact
symbol/arity and no direct module insertion.

### 9. `GENERIC-LOOP-DYNAMIC-BACKEND-CAPABILITY-I0`

Install passive per-function/module capability only from the verified Dynamic
physical completion.  Register it in the shared backend gate.  Absence is
neutral, `mir-interpreter` is accepted, and unsupported backends return one
stable error with `silent_fallback_allowed=false`.  Do not infer capability by
scanning MIR and do not pass backend policy into the Builder.

### 10. `GENERIC-LOOP-DYNAMIC-VM-CANARY-I0`

Switch the one exact canary callable at the method-level production edge,
collect it, import it from a tracked Main wrapper, and execute the unchanged
production source on the MIR interpreter.  Test zero-iteration, inner-return,
and normal fallthrough cases.

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

## Nonclaims at the current D0

```text
no source rewrite or narrowing
no copied simplified skip_while
no Dynamic Recipe implementation
no CallSlot target invention
no Builder / MIR mutation
no After / Completion / DraftSeal / collector
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
