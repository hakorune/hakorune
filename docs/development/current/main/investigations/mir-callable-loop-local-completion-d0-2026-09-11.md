---
Status: closed__CallableLoopLocalCompletionHandoff__2026-09-11
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-D0
Parent: mir-callable-loop-phi-generic-rewire-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-D0

Implementation row: `MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-R0`.

## Six-line brief

```text
Decision: source-bound Loop locals publish completion once before the next
  source read; the logical current_bindings map remains observation only.
Source authority + canonical issuer: resolver-issued SourceNodeSiteV1 and
  BindingRefV1 are owned by CallableSemanticLoweringState; its existing
  record_completed_local consumes CompletedLocalStatementV1 exactly once.
Non-authority: current_bindings, variable_map, publish_defined_binding, names,
  manual install_single_local_for_test, and Composer carrier maps.
Fail-fast boundary: direct_associated.rs Local handling must complete the
  source ledger immediately after initializer lowering; missing/duplicate or
  foreign site/value rows reject before the next source variable read.
Smallest next slice: add one source-port completion capability and connect the
  existing Local arm for one source site with one binding; keep raw behavior.
Non-claims: broad GenericLoop promotion, nested/Dynamic, new semantic receipt,
  Composer deletion, backend/OBJ/EXE, process exit, or runtime re-publication.
```

## Census boundary and exact caller

```text
statement_surface::try_build_with_port_v1 Loop arm
  -> RawLoopChildEntryPortV1::lower_loop
  -> PreparedLocatedRawLoopChildEntryV1::lower_v1...
  -> CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> claim_all().into_semantic_recipe()
  -> CallableGenericLoopV1PhysicalAdapterV1::lower
  -> CallableLoopSourceExpressionPortV1
  -> RecipeComposer::compose_source_generic_loop_v1_recipe_with_port
  -> generic_loop_body::lower_direct_body_input_with_policy
  -> direct_associated::lower_direct_statement_input Local arm
```

The current Local arm lowers initializer expressions and then publishes their
logical values into `current_bindings`; it does not call the existing
`CallableSemanticLoweringState::record_completed_local`. A later source
variable read therefore sees the named downstream failure
`variable-before-materialization` unless a test manually installs a local.
That test-only installation is not production evidence and is excluded from
this row.

## Ownership decision

The source port is the only allowed bridge because it already holds the
invocation-borrowed `Rc<RefCell<CallableSemanticLoweringState>>` and exact
source child context. It may expose one default-false capability for raw
ports; the callable implementation must derive the exact source site from the
statement, build the existing `CompletedLocalStatementV1` in source ordinal
order, and call `record_completed_local` once. The local initializer result is
the current plan value; no name lookup, second ledger, physical session, or
new `Verified*` receipt is introduced.

`current_bindings` remains a logical path observation needed by the existing
CorePlan normalizer. It is updated only after the source completion call
succeeds. Raw and compatibility ports retain their existing map-only behavior.

## Ordered implementation and acceptance

1. Add the smallest source-port hook for a completed Local value sequence;
   default raw behavior returns `false` and does not touch any ledger.
2. Let the callable port use the exact statement site and the existing
   `record_completed_local` publisher. Preserve initializer effect order and
   source ordinal order; fail on empty or mismatched completion shape.
3. Connect only `direct_associated.rs`'s Local arm. Do not alter assignments,
   nested loops, conditional exits, or compatibility routes.
4. Add production-shaped positive evidence with no manual local installation;
   the next same-BindingRef read must succeed for bounds 0, 1, and 3.
5. Add a mutation or focused helper proving that omitting the publisher fails
   at the named local-completion boundary before a physical effect; classify
   any unrelated baseline red separately.

The exclusive future delete set is only the source Callable GenericLoop Local
arm's map-only publication. Shared local descent, the existing completion
publisher, raw ports, and compatibility routes are not deletable here. This
row does not reopen the closed Generic rewire family or authorize Composer
retirement.

## Worker premise audit (2026-09-11)

One read-only worker independently confirmed the caller chain, the missing
production completion, the manual-test limitation, and the one-site bounded
slice. Its result is advisory; this card's ownership decision and acceptance
boundary are the controlling design authority.

## R0 implementation receipt (2026-09-11)

The accepted slice is implemented through the default-false
`LoopPlanExpressionPortV1::exact_source_local_completion` capability. Only
`generic_loop_body/direct_associated.rs` invokes it. The callable source port
derives the exact located statement site, reuses the existing
`CompletedLocalStatementV1` with source-ordinal rows, and calls
`CallableSemanticLoweringState::record_completed_local` before publishing the
observation-only `current_bindings` update. Raw and compatibility ports are
unchanged. The existing plan value is used as the local value because this
slice does not own a physical Local copy.

Acceptance evidence:

- `source_aware_adapter_consumes_real_callable_ledger_once` passes for literal
  bounds `0`, `1`, and `3` without `install_single_local_for_test`.
- `callable_loop_local_completion_missing_publication_rejects_before_read`
  proves the named `variable-before-materialization` boundary when publication
  is omitted.
- The focused Rust compile succeeds; the 492 whole-library warnings are known
  baseline debt and are not caused by this slice.

No production switch beyond the selected source-aware adapter, package/OBJ/EXE
acceptance, nested/Dynamic support, fallback/retry, Composer retirement, or
legacy deletion is claimed.
