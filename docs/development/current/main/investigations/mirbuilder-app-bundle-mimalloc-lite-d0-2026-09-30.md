# MirBuilder app bundle — mimalloc-lite completion D0 (2026-09-30)

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
