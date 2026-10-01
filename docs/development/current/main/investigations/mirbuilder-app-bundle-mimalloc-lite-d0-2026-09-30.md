# MirBuilder app bundle — mimalloc-lite completion D0 (2026-09-30)

Status: active bundle — scalar expression S0 landed at 06cf1a1481; opaque formal ingress child selected.
Scope: mimalloc-lite completion bundle; selected source contracts and retirement only.
Related: CURRENT_STATE.toml; docs/development/RULES.md; parent Gate-1 card below.

Parent card: `mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`
(the Gate-1 thinning ledger). This card carries the one-app completion
bundle for `mimalloc_lite_exe` from the fixed `real-apps-exe-boundary`
suite.

## Decision — MIRBUILDER-APP-BUNDLE-MIMALLOC-LITE-D0 (accepted)

Decision:
  `mimalloc_lite_exe` becomes one completion bundle: ordered internal
  slices, each with its own D0/S0 + positive/negative/Focused-gate
  verification; the app closes only when `mimalloc_lite_exe` passes
  end-to-end (MIR-JSON negative pin re-worded for the drifted terminal +
  pure-first EXE parity run).

Source authority + canonical issuer:
  Empirical probe census (fail-closed ladder terminals) +
  `ObjectDestructionDispositionV1` (`object_definition.rs`),
  `issue_construction_plan` (`instance_construction.rs`), the generated
  `CORE_METHOD_CONTRACT_ROWS_V2` manifest, and the sealed resolver facts
  already used by the prefix/home machinery. No MIR/runtime inference.

Non-authority:
  The 6-FAIL count is not a task count; VM-route behavior is not
  evidence for the EXE lifecycle lane; a passing focused unit test is
  not a production claim.

Fail-fast boundary:
  Every slice keeps `RetainedUnavailable`/`PrefixNotCovered` truthful —
  no permissive arms, no fallback emission. Unsupported shapes keep
  their designed terminal.

Probe census (post-`1eba1bcb11`, `--emit-exe --backend mir pure-first`,
minimal drivers — each isolates one family):

| probe | shape | first terminal |
| --- | --- | --- |
| `new Page{v:i64}` + `return 0` | baseline | compiles (nyrt gate) |
| `v: usize` field | usize scalar field | `artifact-source-unavailable` |
| `extra: usize = 0` default | usize + default | `artifact-source-unavailable` |
| `items: ArrayBox = new ArrayBox()` | container field | `artifact-source-unavailable` |
| `c: Child = new Child(1)` field-init | object field + user `new` init | `artifact-source-unavailable` |
| `new HakoAllocPage(0,8,4)` (real `using`) | usize+ArrayBox fields | `artifact-unowned-lifecycle-site` |
| `LayoutBox.class_size(0)` | static-box call | `artifact-root-completion-unavailable` |
| `c.get(): i64` no-arg | instance call | compiles (existing claim) |
| `c.add(5)` scalar arg | argumented call | `artifact-source-unavailable` |
| `new Outer(i)` object arg | object argument | `artifact-source-unavailable` |
| `local s = i.v` | `handle.<field>` read | `artifact-source-unavailable` |
| `return false/true` in unused method | Bool literals | compiles (needs reachable-fn recheck) |

Claim internals for the `usize` probe (pending-claims census):
`home_prefix=Ok`, `construction=Ok`, `argument_rows=Ok`,
`destruction=Unavailable(FieldType)` — the only failing component is
`object_definition.rs`'s closed `declared_type_name == "i64"` profile.
The claim's object must end its Home at every exit; a non-`i64` field
makes that unprovable today, so the row stays `RetainedUnavailable`.

Bundle slice inventory (each becomes its own bounded D0/S0; order is
the dependency order, not a strict serialization):

1. `DESTRUCTION-SCALAR-FIELD` — admit integer-scalar declared field
   types (`usize` etc., via `classify_numeric_type_name` authority) as
   trivially releasable alongside `i64`. Design Q: extend
   `PlainI64NoHook` or rename disposition; verify the physical release
   path treats `usize` identically. This is the universal first gate —
   every allocator box (`HakoAllocHandle.requested_size`,
   `HakoAllocPage`'s usize fields) trips `FieldType`.
2. `DESTRUCTION-ARRAYBOX-FIELD` — owned `ArrayBox` fields (4× on
   `HakoAllocPage`): child release plan at object end.
3. `DESTRUCTION-OBJECT-FIELD` — owned user-object fields
   (`HakoAllocHeap.small_page`/`medium_page`): nested release.
4. `FIELD-INIT-USER-NEW` — `small_page: HakoAllocPage = new
   HakoAllocPage(LayoutBox.class_size(0), …)`: the
   `ProviderConstruction` store arm admits builtin zero-arg `new` only;
   user-class field-init `new` with call arguments needs its own
   construction row. (Today → `FieldContractUnsupported`.)
5. `STATIC-CALL-CLAIM` — `LayoutBox.class_size/class_id/class_capacity`
   static-box calls: `artifact-root-completion-unavailable`.
6. `INSTANCE-CALL-SCALAR-ARG` — `heap.allocate(8)`, `heap.release(h)`:
   argumented instance calls (0-arg i64 already compiles).
7. `CALL-RESULT-ARG-POSITION` — `handles.push(heap.allocate(8))`: a
   live Invoke inside a core-method argument.
8. `HANDLE-FIELD-READ` — `handle.block_id`, `heap.small_page`,
   `small.alloc_count`: `handle.<field>` reads.
9. `OPAQUE-SCALAR-POSITION` — `requested_size` in `.set` args and
   `requested_bytes + requested_size` (allocate Body(8)+).
10. `OBJECT-FIELD-RECEIVER-CALL` — `me.small_page.seedBlocks()`,
    `me.small_page.allocate(size)`: `me.<ObjectField>.m(..)` (the
    ArrayBox lane of the nested-receiver S0 does not cover object
    fields — needs callee-claim authority, not the core manifest).
11. `RESULT-NEW-OBJECT-ARGS` — `return new HakoAllocHandle(me.page_id,
    block_id, requested_size)`: `me.<i64 field>` arg partially covered;
    BoundValue/OpaqueHandle arg positions remain.
12. `FORWARDED-RESULT` — `return me.small_page.allocate(size)`,
    `return same`, nullable `Handle`/`NullableHandle` results.
13. `SEEDBLOCKS-LOOP-ALIAS` — `local free_stack = me.free_stack` +
    `loop(i < capacity) { free_stack.push(i) }`: container-field alias
    locals + calls inside loop bodies (callable-loop domain).
14. `MIR-JSON-PIN-REWORD` — the negative leg now freezes
    `emission-binding-drift` (drifted inside the fail-closed ladder);
    re-pin when the app approaches.

Normal/Fault verification per slice: positive + negative pin tests in
`brand_catalog_*_tests.rs`, the `page_heap_fixture_result_claim_census`
frontier pin, official quick-profile serial, and the 3× emit-exe smoke.
Production switch + legacy retirement are assessed when the app passes.

Non-claims:
  No promise on ordering beyond "destruction admission is empirically
  first"; slices may merge/split at their own D0s. No claim that
  unused-by-app methods (`realloc*`, `allocateResult`, `isLiveHandle`,
  `resizeInPlace`, `HakoAllocHandleResult` births) are required — the
  reachable-set boundary is a per-slice question. Bool literals are
  provisionally unblocked (probe_s) pending a reachable-function
  recheck.

Landed slices 1-4 (DESTRUCTION-SCALAR-FIELD, DESTRUCTION-ARRAYBOX-FIELD,
DESTRUCTION-OBJECT-FIELD, FIELD-INIT-USER-NEW — D0 decisions and S0
landed receipts) moved to
`mirbuilder-app-bundle-mimalloc-lite-landed-2026-10-01.md` to keep this
active card under the 1000-line pointer-guard bound.


## Decision — MIRBUILDER-APP-MIMALLOC-LITE-STATIC-CALL-CLAIM-D0 (accepted)

```text
Decision: admit qualified static-box calls as non-lifecycle local-call
          claims. `local class_id = LayoutBox.class_id(size)` must not
          end as `PrefixNotCovered`: static calls emit the existing
          generic `Call{Callee::Global}` (StaticReceiver route →
          static-result ingress → typed receipt), never `Invoke`, so
          they owe no lifecycle site — but the homes-aware prefix scan
          classifies local-call initializers only through
          `direct_call_observations`, which qualified receivers never
          produce. The claim gap is the I64 lane of
          `issue_local_call`, not emission.
Source authority + canonical issuer: the method-call inventory row
          (`QualifiedUnbound` receiver +
          `ResolvedQualifiedReceiverIdentityV1`) resolved through the
          source-call-target catalog /
          `VerifiedStaticImportAliasViewV1` (`using … as LayoutBox` →
          canonical owner), corroborated by the callable index
          `StaticBoxMethod` header and the callable
          result-representation solver disposition
          (`ExactI64{required_i64_arguments}` /
          `ExactBool`/`ExactString`). Sole claim product:
          `LocalCallObservationV1{result: I64}` issued inside
          `scan_statement_flow`'s I64 lane through a package-injected
          membership predicate — the same injection pattern as
          `terminal_call`/`local_map_call`. The issued row installs
          `install_i64_call_result`, records no Home, no ledger row,
          no lifecycle site.
Non-authority: `direct_call_observations` for qualified receivers,
          receiver-name spelling or `Unresolved` guesses, the retired
          generic-compatibility lane (`UnissuedStaticCallRetirementV1`).
Fail-fast boundary: sites failing exact membership keep
          `PrefixNotCovered`; `new`-argument static calls keep
          `ArgumentNotCovered`/`ArgumentNotTrivial` until
          `CALL-RESULT-ARG-POSITION`; `me.class_id(..)` stays on the
          separate `resolve_me_call_with_publication_ingress` route
          (its `TargetOnly` freeze is out of scope).
Smallest next slice: STATIC-CALL-CLAIM-S0 — package-injected static
          membership predicate + I64-lane admission for
          `local x = QualifiedBox.m(args)` in ordinary non-birth
          functions; args sealed to Integer/Bool literals or scalar
          `Local`/parameter bindings; positive pin
          `local class_id = LayoutBox.class_id(size)`; negatives:
          unresolvable alias, non-static target, unproven result
          disposition, non-trivial arg.
Non-claims: no emission lane added (StaticReceiver route is the sole
          emitter), no `Invoke`/lifecycle/ledger row, no arg-position
          or `new`-arg calls, no `me.m()` static-route change, no EXE
          or app-level completion claim.
```

Then `MIRBUILDER-APP-MIMALLOC-LITE-STATIC-CALL-CLAIM-S0` lands the
claim end to end.

## S0 landed — MIRBUILDER-APP-MIMALLOC-LITE-STATIC-CALL-CLAIM-S0

Landed as designed — the claim product only; no emission, lifecycle, or
ownership semantics moved:

- `QualifiedStaticCallClaimIndexV1`
  (`normal_callable_semantic_package/qualified_static_call_claim.rs`)
  composes the already-sealed authorities into a
  `(canonical caller key, source site) -> QualifiedStaticCallClaimV1`
  map: `VerifiedStaticImportAliasViewV1::seal` (the same `using` rows the
  lifecycle passes in via `issue_normal_callable_semantic_package_with_
  brand_catalog_and_loop_policy_v1`'s new `import_rows` parameter),
  `VerifiedWholeSourceStaticCallTargetInventoryV1::verify`, and
  `VerifiedSameModuleCallableResultCatalogV1::verify`. Only
  `QualifiedStatic` targets with an `ExactI64` disposition keep a row;
  the row carries `required_i64_arguments`.
- `issue_qualified_static_local_call` (`home_local_call_flow.rs`)
  corroborates the `QualifiedUnbound` receiver shape against the
  method-call inventory and seals every argument: Integer/Bool literals
  or `TrivialLocal` scalar bindings via `PrefixLocalFlow::observe`;
  callee-required i64 ordinals require i64-class evidence. Sole product:
  `LocalCallObservationV1{result: I64}` — `install_i64_call_result`,
  `path_calls` membership, no Home, no ledger row, no lifecycle site.
- `LocalCallObservationV1.arguments` widened from `Box<[i64]>` to
  `Box<[LocalCallArgumentV1]>` (`Integer`/`Bool`/`Scalar(binding)`);
  the lexical-handle emitter consumes `Integer` only and fails closed
  on any other class.
- Predicate threading: `scan_statement_flow` / `observe_if_statement` /
  `walk_branch` / `verify_function_completion_with_new_homes_and_
  argument_observations_v1` take the package-injected predicate;
  bounded siblings (`issue_new_home_prefixes_v1`,
  `issue_new_home_prefixes_with_arguments_v1`, plain
  `verify_function_completion_with_new_homes_v1`) stub `Ok(None)`.
  `issue_new_home_prefixes_probing_fields_v1` now installs the caller's
  declared parameter contracts (was `iter::empty()`) and the same
  predicate — probe and verified walk can never diverge on scalar
  arguments again.
- Caller keys: selected rows translate through `caller_key_for_
  function`; App Main (never a selected row) falls back to the
  catalog's `source_backed_app_main` co-seal key.
- Focused tests (`qualified_static_call_claim_tests.rs`, 6 green):
  exact `Alias.m(size)` claim, scalar-parameter argument, imported
  alias resolution; fail-closed for bool-at-i64-ordinal, handle/map
  arguments, non-`QualifiedUnbound` receivers, and unregistered sites.
- Incidental reclassification: same-module qualified static calls
  (e.g. `Sizes.size(7)`) previously sat unclassified as `BoundValue`;
  the claim now installs them as `Trivial(Integer)` and `new` argument
  rows read `Local` — `selected_new_arguments_admit_inventoried_call_
  result_local` updated to the honest kind (admission unchanged).
- Gates: `cargo check --lib` clean; `cargo test --lib` over
  `normal_callable_semantic_package` + `resolved_semantics` = 763 pass,
  3 fail — all three are the documented red-baseline rows in
  `cargo_lib_red_baseline.failures.txt` (no new regressions). Real-app
  frontier check: `apps/mimalloc-lite` MIR emit still stops at the
  pinned `emission-binding-drift` gate — downstream of this slice's
  claim lane, unchanged by design (D0 non-claims: no app-level
  completion claim).

Next: `MIRBUILDER-APP-MIMALLOC-LITE-INSTANCE-CALL-SCALAR-ARG-D0` —
bundle slice 6: argumented instance calls (`heap.allocate(8)`,
`heap.release(h)`); 0-arg i64 instance calls already compile, this
slice designs the scalar-argument claim surface.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-INSTANCE-CALL-SCALAR-ARG-D0 (accepted)

```text
Decision: admit `local x = recv.m(args)` — lexical-receiver instance
          calls whose sealed callee is an `InstanceBoxMethod` with an
          `I64` result — as non-lifecycle local-call claims. Census
          (worker + fixture probes): no predicate admits an i64-result
          lexical-receiver call today — `terminal_call`'s instance arm
          is terminal-only and 0-arity, the I64 `issue_local_call` lane
          requires `direct_call_observations` (method calls are never
          there), and the site falls to
          `install_inventoried_call_result` + soft `PrefixNotCovered`,
          letting the generic/dynamic member route emit an unowned
          Invoke (debug builds then panic in return_type_strategy for
          `return r`). The semantic gap is the claim row, not physical
          machinery: `LexicalInstanceCallDispositionRowV1` already
          proves receiver class + unique InstanceBoxMethod target +
          callee result kind, and `emit_canonical_instance_call_at_v1`
          / the lexical Invoke emitters already take `Vec<ValueId>`.
Source authority + canonical issuer: `LexicalInstanceCallDispositionRowV1`
          (`ordinary_new_lexical_instance_call.rs`, minted at
          issuer.rs) is the sole membership authority — claim-local or
          parameter-proven receiver class -> unique
          `InstanceBoxMethod` target -> callee's uniform result kind
          `Some(I64)`; the row already carries `argument_sites`. The
          flow-side issuer corroborates `Lexical(Local)` receiver
          against the method-call inventory and seals every argument
          via `PrefixLocalFlow::observe` — Integer/Bool literals or
          `TrivialLocal` scalar bindings — while the callee's
          `parameter_contracts` (`ExactTrivial(I64)` formals, the same
          proof `lexical_handle_result_call` uses) supply the required
          i64 ordinals. Sole claim product:
          `LocalCallObservationV1{result: I64}` +
          `install_i64_call_result`, `path_calls` membership; no Home,
          no ledger row beyond the shared observation, no lifecycle
          site change.
Non-authority: `terminal_call`'s app-main 0-arg instance arm (keeps
          its terminal-relation duty), `direct_call_observations`,
          receiver-name spelling guesses, the dynamic member route
          (it must never re-emit an armed site once the lifecycle gate
          takes it), `me.m(..)` receiver calls (nullable route),
          `me.<field>.m(..)` (manifest lane), `QualifiedUnbound`
          (static lane).
Fail-fast boundary: non-`Lexical(Local)` receivers, non-`I64`
          dispositions, unproven receiver class, non-unique targets,
          any callee formal not `ExactTrivial(I64)`, and any argument
          that is not an Integer/Bool literal or scalar `TrivialLocal`
          all keep `PrefixNotCovered` — Handle/BoundValue/map/home/
          nested-call args are explicitly out. Emission honors the
          Handle-lane co-seal contract: caller-side observation and
          disposition row must agree on target and result class; a
          half-sealed edge freezes.
Smallest next slice: STATIC claim S0 shape reused —
          `INSTANCE-CALL-SCALAR-ARG-S0`: an issuer predicate consulting
          `lexical_instance_calls` + callee parameter contracts (all
          formals `ExactTrivial(I64)` required for the claim), an
          `issue_lexical_local_call`-style flow issuer producing
          `LocalCallObservationV1{I64}` with `LocalCallArgumentV1`
          rows, and a symmetric Standard-route emit gate
          (observation + `take_lexical_instance_call` result==I64 +
          selector agreement -> `Invoke{SameModuleInstance, I64}` with
          materialized args; the dst value is registered I64 so
          `return r` type-checks). Positive pin: `local r =
          pool.allocate(8)` inside `main`/`run`; negatives: non-I64
          callee, handle-typed formal (`release(h)`), map/home arg,
          unproven receiver.
Non-claims: `heap.release(h)` and any handle-argument call need a
          consume-vs-borrow contract from source — a separate D0 (the
          `new`-arg `moved_arguments` move is the existing precedent,
          but no call-argument analog exists). Arg-position calls
          (`handles.push(heap.allocate(8))`) stay with
          CALL-RESULT-ARG-POSITION. `return recv.m()` terminal-forwarded
          results stay with FORWARDED-RESULT. No `me.m(..)` route
          change, no new ownership product, no app-level completion
          claim.
```

Next: MIRBUILDER-APP-MIMALLOC-LITE-INSTANCE-CALL-SCALAR-ARG-S0

## S0 landed — MIRBUILDER-APP-MIMALLOC-LITE-INSTANCE-CALL-SCALAR-ARG-S0

Landed as designed — one claim lane, one disposition arm, one emit
gate; no new ownership product and no semantic scope beyond the D0:

- `lexical_i64_result_call`
  (`ordinary_new_coseal_issue_lexical.rs`) is the package-injected
  predicate: lexical `Local` receiver whose owner-local binding is
  not rebound, exactly one initializer tied to an ordinary-new
  claim, unique selected `InstanceBoxMethod`, all callee formals
  `ExactTrivial(I64)`, no callable result-class claim
  (Handle/Nullable/Object), and every verified value-return site is
  literal/scalar-compatible — `return <i64 formal>` is admitted via
  the callee's own parameter contract.
- `issue_lexical_i64_local_call` (`home_local_call_flow.rs`)
  corroborates the receiver binding against the method-call
  inventory and seals each argument via `PrefixLocalFlow::observe`
  — Integer literals or scalar local bindings only. Sole product:
  `LocalCallObservationV1{result: I64, arguments: sealed rows}`.
- Disposition issuer arm
  (`ordinary_new_lexical_instance_call.rs`): an i64 observation +
  uniform i64 callee result mints
  `Some(InvokeCallResultKind::I64)` and records the site as a
  lifecycle local-call site for root binding accounting; a
  mismatched result class fails closed.
- `emit_local_lexical_i64` (`terminal_call.rs`): requires the I64
  disposition + sealed observation; receiver must appear in prior
  Homes and is materialized from the exact lexical binding, args
  from sealed rows zipped with exact `argument_sites`; emits
  `Invoke{Call, SameModuleInstance(InstanceBoxMethod), I64}`, runs
  the newest-first prior-home unwind (shared with the Handle lane
  in `handle_call.rs`), registers the dst as integer, and records
  root local-call bindings.
- Standard-route dispatch (`direct_call_disposition_port.rs`)
  consults Handle and i64 lexical observations together, takes the
  disposition exactly once, validates selector + result kind, then
  hands off to the matching emitter — the dynamic member route
  never re-emits an armed site.
- Predicate threading mirrors the static-claim S0: verified walk
  and `probing_fields` share the same predicate, bounded siblings
  stub `Ok(None)`.
- Focused tests: `lexical_i64_local_call_tests.rs` (6 green —
  parameter receiver, claim-local receiver, unproven call-result
  arg/receiver, non-i64/mixed result, one-shot disposition) + an
  emit-level pin in `normal_default_root_catalog_lifecycle_tests.rs`
  asserting `Pool.allocate/1` emits
  `Invoke{SameModuleInstance, I64}` with one scalar argument.
- Gates: focused `normal_callable_semantic_package` +
  `resolved_semantics` + lifecycle batches green; the only reds
  observed are manifest rows 12/64 of
  `cargo_lib_red_baseline.failures.txt`
  (`literal-physical-drift`, upstream `ReceiverNonEscape` boundary)
  — both reproduce identically at the D0 HEAD, no slice regression.
  App smoke: `apps/mimalloc-lite` still stops at the pinned
  `emission-binding-drift` frontier — `heap.allocate` is
  nullable-result and correctly outside this lane (D0 non-claim).

Next: `MIRBUILDER-APP-MIMALLOC-LITE-CALL-RESULT-ARG-POSITION-D0` —
bundle slice 7: `handles.push(heap.allocate(8))`, a live Invoke
inside a core-method argument.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-CALL-RESULT-ARG-POSITION-D0 (accepted)

```text
Decision: admit a proven i64-result lexical call in DIRECT argument
          position of an already-claimed i64 lexical call — `local x =
          recv.m(recv2.m2(scalars))` — by sealing the inner site as a
          `CallResult` argument. Census (read-only worker + fixture
          probes): every argument seal today goes through
          `PrefixLocalFlow::observe` (literal / variable_ref only), so a
          nested-call arg returns `None` and the OUTER claim is withheld
          — yet the inner site already owns a
          `LexicalInstanceCallDispositionRowV1` (the issuer walks all
          `method_calls()`, arg-position included) and the raw lane
          already materializes the inner Invoke through
          `drive_call_arguments_v1`/`drive_legacy_expression_v1`. The
          observed gap is claim + type attribution: `local r =
          pool.give(pool.allocate(8))` panics in return_type_strategy
          because the dynamically emitted Invoke never registers its
          result value. App reality: every direct arg-position call in
          mimalloc-lite is handle-adjacent (`push(heap.allocate(..))` is
          a NullableObject escaping into container storage; `get(..)` is
          `Dynamic`), so the real sites still wait on the
          consume/borrow D0 — this slice lands the scalar machinery
          first, the same pattern FIELD-INIT-USER-NEW followed.
Source authority + canonical issuer: the inner site's
          `lexical_i64_result_call` predicate, reused unchanged
          (claim-local receiver + unique `InstanceBoxMethod` + all-i64
          formals + literal/i64-formal value returns). The outer issuer
          `issue_lexical_i64_local_call` gains a `CallResult` argument
          arm that looks the arg site up in `method_calls()` and runs
          the inner predicate; the claim product carries the inner
          site's sealed observation — a destination-free arg-position
          row (parallel to `LocalCallObservationV1`, which is
          binding-shaped and cannot express an arg-position call), never
          a widened `destination`. The existing arg-position disposition
          row corroborates `result == Some(I64)` at co-seal exactly like
          the statement lanes.
Non-authority: `BoundValue` (binding-shaped provenance only, never a
          bare-site record), `me.<field>.m(..)` subtree-neutrality
          (`argument_subtree_neutral` keeps rejecting `MethodCall`),
          the raw descent (it emits but never claims),
          `QualifiedUnbound` receivers, receiver-name guesses.
Fail-fast boundary: any argument that is not an Integer/Bool literal,
          a scalar `TrivialLocal`, or a call site the inner predicate
          claims; an inner callee whose exits are not uniformly I64
          (nullable/handle/dynamic/mixed); inner formals not all
          `ExactTrivial(I64)`; calls inside binary/condition subtrees
          deeper than a direct argument site; statement-position calls
          (`handles.push(x)` itself is a separate NoValue-core-method
          lane). A sealed outer row whose inner claim later fails at
          co-seal freezes — never degrades.
Smallest next slice: `CALL-RESULT-ARG-POSITION-S0` —
          `LocalCallArgumentV1::CallResult{site}` (or the sibling arg
          row), the arg-position observation mint, the disposition
          arm for arg sites, and the emit arm: `emit_local_lexical_i64`
          materializes a `CallResult` argument by recursively emitting
          the inner call (its own Invoke + projection + shared
          prior-home unwind + binding-group record) before the outer
          Invoke. Positive pin: `pool.give(pool.allocate(8))` lowers to
          two `Invoke{SameModuleInstance, I64}` with the inner result
          feeding the outer's argument slot. Negatives: nullable or
          handle-result inner calls, non-i64 inner formals, statement
          calls, deeper subtree calls.
Non-claims: `push(heap.allocate(8))` itself — a NullableObject result
          escaping into container storage — stays with the
          consume/borrow D0 together with `heap.release(h)`;
          `handles.push(x)` statement calls need the lexical-receiver
          NoValue core-method lane (own slice); `handles.get(..)`
          `Dynamic` results; calls inside `+`/`==`/`&&` subtrees;
          `me.m(..)`/`me.<field>.m(..)` receivers; `QualifiedUnbound`
          inner calls; app-level completion.
```

Next: MIRBUILDER-APP-MIMALLOC-LITE-CALL-RESULT-ARG-POSITION-S0

## S0 landed — MIRBUILDER-APP-MIMALLOC-LITE-CALL-RESULT-ARG-POSITION-S0

Landed as designed — one sealed argument row, one recursive seal, one
recursive emit; no disposition-issuer change (the arg-position row was
already minted with `Some(I64)` by the `(false, false, other)` arm) and
no second lifecycle binding group:

- `LocalCallArgumentV1::CallResult(Box<ArgumentCallObservationV1>)`
  (`home_local_call_flow.rs`) is the sole nested-call evidence: the
  inner `OwnedExprSiteV1`, the enclosing prior Homes, and recursively
  sealed argument rows. `seal_i64_call_arguments`/
  `seal_argument_call` reuse the unchanged `lexical_i64_result_call`
  predicate — an arg-site that is a proven i64 lexical call seals as
  `CallResult`, anything else leaves the outer site unclaimed.
- Emit refactor (`terminal_call.rs`): `emit_local_lexical_i64` keeps
  the single `record_root_local_call_bindings` call for the outer
  site and delegates physical emission to `emit_lexical_i64_call`,
  which the `CallResult` arm re-enters — take the inner site's
  armed disposition exactly once, recursively emit its Invoke +
  normal projection, and feed the inner result `ValueId` into the
  outer argument slot. Inner instructions append to the outer
  `bindings` vector in physical order, so the one recorded group
  covers both invocations (the inner call owns no destination
  binding and no expected lifecycle site — the strict
  source-order accounting stays exact).
- Focused tests: `lexical_i64_local_call_tests.rs` +2 (positive
  `CallResult` seal with inner site/args/prior-Home evidence and a
  one-shot armed disposition; four-case fail-closed table —
  construction-result, opaque-formal, deeper-subtree, bool-actual
  inner calls keep the outer site unclaimed) + an emit-level pin in
  `normal_default_root_catalog_lifecycle_tests.rs` asserting two
  `Invoke{SameModuleInstance, I64}` where the `give` invoke seats
  on `allocate`'s normal-landing edge and the inner projection dst
  is the outer call's argument.
- Gates: focused `lexical_i64` (10/10) +
  `normal_callable_semantic_package`/`resolved_semantics`/
  lifecycle batches green modulo manifest baseline reds
  (`main_static_child_port_consumes_all_role_rows_once`,
  `qualified_call_map_argument_reaches_the_named_capability_boundary`,
  `artifact_child_rejects_retained_unavailable_commit_before_lifecycle_coverage`,
  `birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`)
  plus one batch-only flake that passes in isolation — no slice
  regression. Serial quick-profile run: `8130/126/56` with the
  failure-name set byte-identical to the accepted receipt
  (`failure_sha256` unchanged); the manifest re-baselines the
  +3-test inventory only. One intermittent observation:
  `nullable_receiver_call_serializes_nullable_handle_and_checked_release`
  flaked red in one serial run and in isolation — it reproduces
  identically at the D0 HEAD (`ordinary-membership-drift`), so it is
  pre-existing flake debt outside this slice, not a regression.
  App smoke: `apps/mimalloc-lite` still stops at the
  pinned `emission-binding-drift` frontier — `handles.push` is a
  NoValue core-method lane and `heap.allocate` is nullable-result,
  both D0 non-claims waiting on their own slices.

Next: `MIRBUILDER-APP-MIMALLOC-LITE-HANDLE-FIELD-READ-D0` — bundle
slice 8.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-HANDLE-FIELD-READ-D0 (accepted)

```text
Decision: admit `local x = <proven receiver>.<declared field>` as a
          claimed field-read local binding — one meaning, two result
          classes dispatched by the field's DECLARED type. Numeric
          scalar fields (`is_numeric_integer_type_name`: i64/usize
          family, i64 wire) seal a Trivial scalar local; object-class
          fields seal a borrowed field-alias binding that joins NO
          Home (the field's teardown is owned by the root object's
          destruction plan) and carries the field's declared class as
          the alias's binding-class authority — so a later `x.f` /
          `x.m(..)` resolves on the field's class, never on the root
          receiver's class. Census (read-only worker): the app's
          `run()` needs `local small = heap.small_page` (handle field)
          and `small.alloc_count`/`medium.*` (usize fields, all in
          concat/if subtrees — slice 9's domain); the alias's class
          authority is the missing link for every downstream read.
Source authority + canonical issuer: `BodyExpressionShapeV1::FieldAccess`
          sites on the local-initializer position only. Receiver class:
          claim-local `new` candidate class OR `me` entry-loan box —
          the existing `binding_class`/`initializer_class` provenance.
          Field identity: `with_source_object_definition` →
          `UserBoxFieldDecl` ordinal → `CanonicalFieldRefV1`
          (declaration-ordinal authority — never a name lookup).
          Result class: `UserBoxFieldDecl.declared_type_name` — a
          numeric-integer name seals scalar, a known ordinary-box name
          seals object-alias, anything else is unclaimed. The claim
          product extends the local-initializer observation in
          `scan_statement_flow` (the FieldAccess arm currently falls
          through to `observe`=None → `PrefixNotCovered`) and installs
          a `binding_class` initializer arm for non-`me` FieldAccess
          receivers via declared type (the me-only `field_write_claim`
          arm stays untouched). Physical owner: staged `FieldRead`
          row + `ObjectFieldGet{dst, base, CanonicalFieldRefV1}`
          (claims must reach Emitted — existing
          `ordinary_new_field_reads.rs` validation); scalar results
          register `Integer`, object-alias results register the field
          class's box type so a downstream receiver/return stays typed.
Non-authority: `field_origin_by_box`/`value_origin_newbox` MIR-level
          inference (claim issuers never consult physical facts);
          `field_write_claims` for non-`me` receivers (write-claim
          provenance is not declaration authority); name-string field
          resolution; `DeclaredHandle` param contracts
          (`handle: HakoAllocHandle` stays `UnsupportedDeclaredType` —
          param-receiver reads are a separate D0).
Fail-fast boundary: receiver class unproven (no claim-local `new`, no
          entry loan, rebound/multiple initializers) → unclaimed;
          field absent/weak/undeclared-type → unclaimed; alias escape —
          returned, container-stored, passed as a call argument, or
          used where a Home join is assumed (e.g. as a lexical-i64-call
          receiver, whose prior-Homes membership check would freeze) →
          unclaimed for now; a scalar field read emits
          `ObjectFieldGet` only — never a raw `FieldGet` guess.
Smallest next slice: `HANDLE-FIELD-READ-S0` — one claim row minted by
          a new initializer-observation arm, the field-alias binding
          record (root binding + `CanonicalFieldRefV1` + declared
          class, excluded from `homes` and from unwind evidence), the
          non-`me` `binding_class` extension, staged `FieldRead` +
          `ObjectFieldGet` emit. Positive pins: `local small =
          heap.small_page` installs an alias whose class is
          `HakoAllocPage` and joins no Home; `local n = page.capacity`
          (usize) installs an Integer scalar. Negatives: param-receiver
          reads, unproven receiver, weak/undeclared field, alias
          escape (return/container/call-arg), subtree positions.
Non-claims: `handle.block_id` param-receiver reads (DeclaredHandle
          contract widening is its own D0); scalar field reads inside
          concat/`==`/`&&` subtrees or call arguments
          (`small.alloc_count` in the app's `print`/`if` — slice 9
          OPAQUE-SCALAR-POSITION); `x.m(..)` calls on an alias receiver
          (needs the alias's prior-Homes/exhaustion semantics — its
          own decision); `me.<ObjectField>.m(..)` receivers (slice
          10); `local a = me.<ArrayBox>` residence aliases (slice 13);
          field writes on non-`me` receivers; chained
          `a.b.c` reads; app-level completion — `run()` still needs
          slices 9-13.
```

Next: MIRBUILDER-APP-MIMALLOC-LITE-HANDLE-FIELD-READ-S0

## S0 landed — MIRBUILDER-APP-MIMALLOC-LITE-HANDLE-FIELD-READ-S0

Landed as designed — one field-declaration authority, one staged
ledger row, one physical `ObjectFieldGet` owner; `field_write_claims`
stay non-authority and the bounded `with_arguments` sibling keeps its
stub (local-initializer reads admit on the verified lane only):

- `StoredLocal::FieldAlias { class }` + `field_read_receiver`
  (`home_prefix_local_flow.rs`) is the sole alias record — borrowed
  storage carrying the declared class, observable as `None` so an alias
  never moves the receiver root into a `new` argument and never joins
  the Home/teardown set. Receiver provenance classifies into
  `OwnedHome` / `RootedHandle` / `Alias`; `me` receiver sites resolve
  through the `Me` shape (`BodyMeReceiverV1::Lexical`), not
  `variable_ref`.
- `home_new_prefix_field_read.rs` is the sole local-initializer
  observer — `local x = recv.field` only (the FieldAccess must be the
  statement's direct initializer; subtree positions stay
  `PrefixNotCovered`). It sits after the field-call arm and before the
  inventoried-call fallback; scalar results install `Trivial(Integer)`,
  box results install `FieldAlias`.
- `terminal_home::local_read_field` is the one declaration authority —
  three provenances each proven on their own class: claim-local `new`
  candidate (construction plan's object must equal the definition's
  object; duplicate destinations fail hard), the entry loan's `me` root
  (`home == loan.receiver()`, same owner), and a prior alias's retained
  declared class (resolved through ordinary-box coverage). Result class:
  `is_numeric_integer_type_name` → `Scalar`, `coverage.contains_box` →
  `Alias`, anything else (weak/duplicate/undeclared/MapBox) →
  `Ok(None)` unclaimed.
- Ledger (`ordinary_new_field_reads.rs`): `LocalFieldRead` rows stage
  through the coseal predicate (idempotent across observation/accounting
  passes), merge with owner/duplicate checks, take once with
  receiver-site `Receiver`-segment + progress validation, and record
  `Emitted` with `ObjectFieldGet`; completeness and physical validation
  cover the local rows the same as terminal/argument rows.
- Emit (`fields.rs` + `raw_ordinary_new_claim.rs`):
  `PreparedExactFieldReadClaimV1::{Terminal, Local}` splits the claim
  family — `Local(Scalar)` lowers `MirType::Integer`, `Local(Alias)`
  lowers `MirType::Box(class)`; the terminal take falls through to
  `take_local_field_read`, and the sealed class (not MIR inference)
  types the destination.
- Focused tests: `local_field_read_claim_tests.rs` +7 (scalar on
  selected `new`, object alias, read-on-alias, `me` entry-receiver read
  in a `new`-carrying method, alias rejected as movable `new` argument,
  missing/MapBox field fail-closed, parameter receiver unstaged) + 2
  emit-level pins in `normal_default_root_catalog_lifecycle_tests.rs`
  (single `ObjectFieldGet` typed `Integer`; chained alias read basing on
  the alias binding's materialized `Copy`, typed `Box("Page")`).
- Gates: focused `local_field_read` (9/9) +
  `normal_callable_semantic_package`/`resolved_semantics`/lifecycle
  batches green modulo manifest baseline reds (unchanged set) plus the
  known `array_source_binding` batch-only flake passing in isolation —
  no slice regression. App smoke: `apps/mimalloc-lite` still stops at
  the pinned `emission-binding-drift` frontier — `handles.push` is a
  NoValue core-method lane and `heap.allocate` is nullable-result, both
  D0 non-claims waiting on their own slices.

Next: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-SCALAR-POSITION-D0` — bundle
slice 9.

## Decision — OPAQUE-SCALAR-POSITION-D0 (2026-10-01)

Read-only worker audited `e939989660`: the unresolved premise was whether
scalar subtree projection and opaque-parameter use share an existing owner.
They do not have the same admission contract; the decision separates them.
No compiler execution or new baseline measurement was performed for this D0.

```text
Decision: reuse declaration-backed scalar field-read claims at explicit
  expression positions; keep opaque parameters opaque. First restore the
  source size boundary in a separate BoxShape T0, then implement the bounded
  scalar-expression S0. Opaque borrowing and mixed call/String roots require
  their own contract decisions inside this bundle, not a permissive observe.
Source authority + canonical issuer: exact resolver expression_source rows
  (operator and child sites), current PrefixLocalFlow scalar/receiver proofs,
  terminal_home::local_read_field and the receiver's declared numeric field.
  Existing LocalFieldRead ledger -> exact field port -> ObjectFieldGet stays
  the sole projection/physical chain. No new field-read map or evaluator.
Non-authority: parameter spelling, caller literal 8, a usize destination,
  MIR value_types, Other("BinaryOp"), VM behavior or a proved leaf alone.
Fail-fast boundary: missing receiver/field/operator evidence, opaque or
  object-valued operands, and unproved call-containing roots remain unavailable.
Smallest next slice: MIRBUILDER-APP-MIMALLOC-LITE-COSEAL-SIZE-T0; no semantic
  change. Follow with SCALAR-EXPRESSION-POSITION-S0 under the profile below.
Non-claims: no DeclaredHandle widening, nullable push, object-field method,
  String concatenation, forwarded object result, app PASS or global retirement.
```

### Finite source inventory and distinct contracts

| Selected source | Existing evidence / gap | Disposition |
| --- | --- | --- |
| `apps/mimalloc-lite/main.hako:39-40` | Eight numeric field leaves on prior `small`/`medium` aliases; surrounding print/concat also calls `freeCount` | Reuse field declaration proof; String/call root admission remains separate |
| `apps/mimalloc-lite/main.hako:44` | The same eight fields in comparisons/And; root also calls `freeCount`, `requestedBytes`, `outstandingBlocks` | Condition traversal must be explicit; field support alone does not cover the whole root |
| `page_heap_box.hako` allocate `Body(8)` and requested-bytes RHS | Unannotated `requested_size` is `OpaqueHandle`, installed as `StoredLocal::Handle`, not scalar | Keep rejection; select checked operation/borrow/Fault contract before widening |
| Existing exact scalar local in builtin argument/store RHS | Field-call and field-store walkers have different transfer rules | Preserve each context's current admission; do not merge them by bool or spelling |

There are 16 scalar field-read occurrences in the app source, not 16 new
owners or proof products. Imported allocator operands are a separate part
of the same app dependency chain. This static inventory is not an observed
runtime terminal order and does not repeat the whole-repository census.

### Ordered implementation tasks inside slice 9

1. **COSEAL-SIZE-T0 (next).** The committed issuer is 1,048 lines and ledger
   module 807. Split private source-claim preparation, per-declaration
   observation/claim assembly and ledger/error vocabulary at their ownership
   boundaries. Existing `#[path]` modules are sufficient; keep source loans,
   invocation identity, take/finish order, errors, ABI and acceptance unchanged.
   Merely moving the two bottom helpers does not make the issuer small enough.
   Every touched/extracted source must be below 800, with room below 760 where
   practical. Do not add semantic arms to either oversized parent beforehand.
2. **SCALAR-EXPRESSION-POSITION-S0.** Generalize the existing field-read site
   helper while retaining its direct-initializer adapter and Alias behavior.
   New subtree positions accept only Scalar field results on an exact Home,
   entry `me` or live prior alias. Use sealed expression source operator/child
   rows, not the generic body-shape kind, for the initial pure profile:
   Integer/Bool literals and exact scalar locals; integer Add/Subtract,
   integer Equal/NotEqual comparisons, and Bool And/Or. Require Integer
   operands for arithmetic/comparisons and Bool operands for And/Or; mixed
   Integer/Bool operands reject. A fully proved initializer installs its
   actual Integer or Bool class (existing scalar install), never a fabricated
   local for each leaf. Require Bool for this profile's if-condition root
   before branch traversal; branch
   body verification does not prove condition leaves. Reuse existing ordinary
   binary and short-circuit consumers; their evaluation/Fault semantics stay
   unchanged. Other operators and effectful/mixed-call roots are excluded.
3. **Transactional staging and physical cutover (same S0).** Prove the complete
   selected root before committing staged field-read rows. Rejected roots
   leave no successful partial claim/residual rows. Keep exact owner/site,
   receiver child path and canonical field; every accepted leaf uses the
   existing once-only take/record/finish checks and Integer ObjectFieldGet.
   Short-circuit RHS reads remain in the RHS control region; never hoist them.
   Existing core arguments/store RHS may reuse the leaf helper only while
   preserving their own whole-root, transfer and effect checks.
4. **Remaining slice-9 decisions.** For opaque allocator parameters select the
   exact source binding, checked operation/provider, normal representation,
   borrow/retain/transfer behavior, rejection/Fault and caller unwind. For
   String concat/print and call-containing conditions name each existing call
   and operation consumer before combining leaf evidence. Do not weaken an
   exact-I64 call contract or relabel Handle from its use position. Keep these
   tasks in this bundle; do not open a second interpreter or observer census.
5. **App closeout and retirement.** Re-observe the chosen app after relevant
   boundaries change; close the bundle only at its original EXE acceptance.
   Scalar-expression green alone is not permission to re-pin the app as PASS.

### Acceptance and deletion accounting

- Positive: Home numeric field in `local n = page.count + 1`; entry `me`
  numeric comparison; prior-alias numeric condition; supported scalar-local
  store/argument; repeated same-field reads have distinct exact sites.
- Negative: mixed scalar operands or non-Bool condition, unknown/rebound/
  consumed receiver, missing/weak/noninteger field,
  alias as numeric value, foreign/duplicate/wrong child site, double take,
  opaque operand, hidden call, unsupported operator and partially proved root.
- Physical: exact canonical field/base and Integer destination feed the
  actual operator/condition; short-circuit RHS stays conditional. Normal/Fault
  cleanup retains each original obligation exactly once; no borrowed alias
  enters Home teardown or escapes as a movable argument.
- T0 uses existing field-read claim/emit and touched-module regressions; S0
  adds discriminating focused positives/negatives and physical tests. Classify
  failures against the existing quick-profile baseline; never promote a flake
  into the deterministic failure manifest to make a run green.
- Retirement mapping: selected scalar FieldAccess source sites previously
  entering raw Dynamic/FieldGet must consume the exact ObjectFieldGet port.
  Assert no raw FieldGet/re-entry for those exact sites and no stranded claims.
  The shared Dynamic arm has outside readers and is retained. Physical delete
  credit is zero until an actual helper/edge has migrated callers, verified
  acceptance and caller-zero; record any such finite delete set before removal.
- Keep unrelated legacy schema/backend retirement parked. The app bundle's
  production switch and selected-edge retirement remain mandatory closeout,
  not a claim made by this design or by the BoxShape split.

Next: `MIRBUILDER-APP-MIMALLOC-LITE-COSEAL-SIZE-T0`, then
`MIRBUILDER-APP-MIMALLOC-LITE-SCALAR-EXPRESSION-POSITION-S0`.


### COSEAL-SIZE-T0 publication receipt (2026-10-01)

- Behavior-preserving ownership split: issuer 1048 -> 711 lines, source-claim
  child 405; coseal 807 -> 434, ledger child 381. Ledger method visibility
  retains the original semantic-package boundary. Existing claim order,
  invocation identity, errors and affine take/finish behavior are unchanged.
- Module README and scope-guard pins follow the moved responsibilities; both
  new modules are registered in the size boundary. No semantic arm or selected
  old-edge retirement was added; physical deletion credit remains zero.
- Focused quick/serial `local_field_read`: 9 passed, 0 failed (7 claim and
  2 emitter tests). Full lib baseline verifier: KNOWN BASELINE, exit 0;
  8139 passed / 126 failed / 56 ignored, inventory 8321, unchanged failure
  SHA256 `eed5d558e18359d7d8502c87a0bd54b42cdbcb8215f3ab505671169c34e6e497`.
- Pointer guard, touched-file rustfmt, shell syntax and diff check pass.
  Qualified-route scope guard still rejects unchanged `brand_catalog_tests.rs`
  at 961 lines: existing size debt, not a T0 regression; not waived or hidden.
- D0 and T0 worktree changes were preserved and pushed together by an external
  handoff at `0d60daa3f59155990aa216b207df3cf5da9d49f6`. This existing history
  is retained; separate D0/T0 patch snapshots remain available in `/tmp`.
  Local `.git` is now writable; the earlier publication restriction is resolved.
- Next: SCALAR-EXPRESSION-POSITION-S0 under the accepted profile above.
  App acceptance and legacy retirement remain incomplete.


### S0 transaction decision — worker audit (2026-10-01)

The existing verified callback stages each successful leaf eagerly; the probe
callback is pure. Recursing through that verified callback before whole-root
acceptance would leave a left-hand read staged when a later operand rejects.
This is an extension hazard, not evidence of a direct-initializer regression.

Decision: change the existing local-field-read callback to an all-or-none batch
at the same source/issuer boundary. Do not add a second semantic owner/map or
use a rollback after physical emission. Ordered implementation within S0:

1. Add passive exact read requests (owned read site, receiver expression site,
   receiver binding, Home root, optional alias class, declared field name).
   Direct initializer uses a singleton allow-Alias adapter; pure expression
   roots request Scalar-only. Requests are not claims or authority.
2. Preflight the whole root using sealed literal/local/operator rows. Check
   binary row identity and exact Lhs/Rhs child paths. A field is a conditional
   Integer obligation, not a proven Integer: collect its exact request while
   checking receiver provenance. Reject unsupported/mixed roots before staging.
   Preserve Integer/Bool class; obtain IfCondition through the existing located
   source port and require Bool before branch forks. Do not tighten unrelated
   previously accepted conditions without the selected-profile scope check.
3. The existing issuer proves EVERY request via terminal_home::local_read_field
   into a temporary vector. Check unique owner/sites, canonical fields and
   declared result; Scalar-only batches reject Alias/None. Repeated exact sites
   may be idempotent across passes only if receiver site/binding, Home root,
   canonical field and result match the existing row. Reject drift rather than
   answering solely from contains_key. No ledger insertion occurs on rejection
   or error. Only after all checks succeed extend local_staged_reads once.
   The source-only probe uses the same admission with no staging.
4. Keep batch preparation private to the existing issuer's source-claim child
   or a responsibility-specific child; do not grow the 711-line issuer to 800.
   Update the existing scanner/branch/completion callback wiring together.
   Exact ObjectFieldGet and one-shot ledger consumption stay the physical path;
   RHS short-circuit placement stays owned by the existing operator consumer.
5. Add focused negatives for valid-left/invalid-right, second-field Alias,
   opaque/mixed operand, wrong binary child site, non-Bool condition, duplicate
   request and cached-site drift. Check rejected roots add zero staged rows.
   Positive emitter tests prove arithmetic/comparison destinations and a
   conditional RHS field read, plus direct initializer/Alias regressions.

This refines task 3's transaction mechanism; it does not broaden the accepted
profile, claim app PASS, or authorize shared Dynamic-arm removal. S0 remains
selected; no external dependency or completed migration is inferred.


S0 lifetime refinement (read-only follow-up, 2026-10-01): the existing
field_read_receiver bypasses ordinary Handle observation's Consumed-root
check, and FieldAlias retains only class. Selected-new argument transfer calls
consume_home before later reads, with no other source owner rejecting these
stale receivers. Preserve root provenance inside existing FieldAlias state;
pass the exact request Home root into alias installation and inherit it through
alias chains. Require a live Home/self-rooted entry Handle for RootedHandle and
Alias receiver proof; include root in branch-state equality. Keep the alias as
borrowed storage with no Home obligation or movement authority. Add negatives
for direct Handle alias and FieldAlias reads after a verified selected transfer;
assert the transfer was accepted and later reads stage no rows. This closes the
already-required consumed-receiver rejection, without new assignment admission.


S0 condition-scope refinement (read-only audit, 2026-10-01): select the
whole pure morphology before semantic proof, not merely the presence of a
field or a supported outer operator. A recursive selector permits the profile's
sealed binaries, Integer/Bool literals, lexical variables, and FieldAccess with
only direct Variable/Me receivers. Nested unsupported operators, calls, String/
Null, containers and nested/effectful receivers remain outside this profile and
retain their existing condition owner, with zero new read claims. Selected pure
roots still reject mixed types, missing fields, consumed receivers and non-Bool
results. Add regression evidence for nested Greater, call and nested field
receivers; the page-heap Body(0) Greater condition must not move its existing
Body(8) opaque-operand frontier backwards. The old argument negative for a local
initialized by `1 + 2` becomes a positive exact-Local assertion; its direct-call
argument case remains negative. Do not rebaseline either discrepancy as debt.


### SCALAR-EXPRESSION-POSITION-S0 receipt (2026-10-01)

- Existing field declarations and sealed expression/operator paths now prove
  complete pure Integer/Bool initializer roots and selected field conditions.
  Every batch is checked before staging; cached-site descriptor drift and
  duplicate sites reject without partial insertion. Direct initializer Alias
  behavior remains on its singleton adapter. Borrowed aliases retain the
  original root and reject reads after its selected transfer.
- Source-backed physical tests consume the exact Integer ObjectFieldGet in
  arithmetic (including existing Copy bindings); RHS short-circuit reads stay
  in a distinct control block. Selected sites do not emit raw FieldGet. The
  shared Dynamic arm retains outside readers: physical deletion credit zero.
- Focused scalar expression 12/12; existing local_field_read 9/9. Page-heap
  fixture census and selected-new argument regression each pass. The former
  keeps Body(8), rather than regressing to the earlier Greater condition.
  The old `local x = 1 + 2` negative now checks its admitted exact Local rows;
  direct hidden-call arguments still reject.
- Full quick/serial observation: 8150 passed / 127 failed / 56 ignored,
  inventory 8333. Twelve tests added, none removed; all 126 deterministic
  baseline failures unchanged. The sole extra failure is the already-recorded
  nullable_receiver_call_serializes_nullable_handle_and_checked_release flake,
  which passes in isolation. Baseline verifier reproduced that summary drift:
  FAIL, not PASS. Its deterministic failure manifest is unchanged; inventory
  and expected passing count alone are refreshed for the twelve added tests.
- Qualified-route guard still rejects unchanged brand_catalog_tests.rs=961:
  existing debt, not waived. All touched/new Rust sources are below 800.
  Pointer, touched-file format, shell syntax and diff checks pass.
- Final post-format verifier: KNOWN BASELINE, exit 0; 8151 passed / 126 failed /
  56 ignored, inventory 8333, unchanged failure SHA256
  `eed5d558e18359d7d8502c87a0bd54b42cdbcb8215f3ab505671169c34e6e497`.
  Earlier nullable flake observations remain recorded above. No whole-app EXE
  PASS, opaque support, String/call-root admission or global retirement claim.


### Task 4 handoff — opaque borrowed operations (2026-10-01)

OPAQUE-BORROWED-OPERATIONS-D0's ingress Decision is accepted in
[the focused opaque-formal card](mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md).
Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-FORMAL-INGRESS-S0. The focused
card owns exact domain/use-closure/entry/physical acceptance and remaining
Set/Add decisions. Original app EXE completion and bundle tasks 10-12 remain.
No opaque operation activation or physical deletion claim is made here.
