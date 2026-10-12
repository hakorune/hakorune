# MirBuilder app bundle — mimalloc-lite completion D0 (2026-09-30)

Status: active bundle — Static validation scope S0 accepted; required Object forward D0 active.
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

Claim-only qualified Static/I64 source rows; no emission/lifecycle change. Six claim tests pass; module batch763PASS/3 documented baseline reds. Original app remained emission-binding-drift; no app/retirement claim.
Full implementation/negative tests/receipt history: this card at `7171048755`.
The accepted D0 above and current owner README retain the contract; this historical
receipt grants no additional execution or retirement permission.

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

ExactTrivial(I64) lexical instance arguments and sole I64 Invoke gate accepted. Six local-call tests and source emission pin pass; manifest reds reproduce at D0. Nullable allocate remained outside this scope; original app emission-binding-drift.
Full implementation/negative tests/receipt history: this card at `7171048755`.
The accepted D0 above and current owner README retain the contract; this historical
receipt grants no additional execution or retirement permission.

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

Nested CallResult reuses the outer binding group and recursively seals/emits each Invoke once. Focused10/10; serial8130/126/56 with unchanged deterministic failures and documented nullable flake. NoValue push/nullable allocate remain separate contracts.
Full implementation/negative tests/receipt history: this card at `7171048755`.
The accepted D0 above and current owner README retain the contract; this historical
receipt grants no additional execution or retirement permission.

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

Declared field type is sole authority for direct-initializer Scalar/Alias; staged one-shot ObjectFieldGet, no borrowed alias Home/transfer. Nine claim tests plus two emission pins pass; original app emission-binding-drift, baseline failures unchanged, array batch-only flake retained.
Full implementation/negative tests/receipt history: this card at `7171048755`.
The accepted D0 above and current owner README retain the contract; this historical
receipt grants no additional execution or retirement permission.

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


### Task 4 handoff — opaque borrowed operations

The focused [opaque-formal card](mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md)
owns ingress/operation decisions and receipts; bundle tasks10-12 and original
app EXE remain required. No physical deletion or whole-app completion claim.

## Accepted input preparation slices (2026-10-12)

`96624eb74e` owns sibling S0 (51PASS/1 baseline) and corrected batch/owner mapping;
`c82ee43752` + `4d202191a8` own qualified input D0/S0 (53PASS/1 baseline).
Generic discovery borrows passive SourceObject input through SourceInstance;
dedicated qualification, execution/Normal/result/ABI gates remain. No source rewrite
or shared-owner physical deletion. Full receipts, times and negatives in those
commits; current contract is in owner README. Original unchanged app SHA256
`33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685` now stops
at LexicalInstanceCall/source-only-object-actuals,0.050s/no EXE. Test/CLI snapshots
match. Parent two-exit/document S0 `2f6f0a8a1a` remains accepted; its JSON/readback/
native1/2/73 evidence is not whole app or global retirement proof.

## Static final-incoming D0 — current boundary (2026-10-12)

Parent: `MIRBUILDER-APP-MIMALLOC-LITE-STATIC-FINAL-INCOMING-D0`.
Original app, current tested CLI bytes, pure-first/replay-none. Read-only trace:
- Requested Static: LayoutBox.class_id(owner17/batch13), Body(0)/Initializer(0)
  -> SizeClassBox.good_size(owner11/batch7).
- Rejecting SourceObject: HakoAllocHeap.allocateResult(owner34/batch30),
  Body(1)/Initializer(0) -> allocate(owner33/batch29).
- Different callee cohorts, same source graph. Sealed contract rows provide
  batch-slot joins; header enumeration alone is not owner identity.
Evidence: `/tmp/mimalloc-static-final-incoming-site.log`, `...-contract.log`,
`/tmp/mimalloc-static-final-current-map.log`; current-binary disassembly validates
memory offsets. Those offsets are diagnostic only, not authority/portable ABI.

Current checker deliberately validates whole-final-incoming before lending only
requested owner rows. Static packet checks original qualified Rc/call/parameters/
Completion first. README's opaque Object/entry global execution contract remains.
Decision: do not skip SourceObject, filter the shared checker, grant execution or
turn rejection into missing evidence. Explore a Static-specific split inside the
same owner: global source/inventory validation plus selected-callee execution
closure and original forward dependencies. Other opaque entry/Object contracts
must remain unchanged. SourceObject is retained, not promoted.

Initial D0 questions (resolved by the selected-S0 Decision below):
1. Loan T0 below binds the canonical catalog; Static source validation must
   borrow it, not copy authority or use executable ordered checks for SourceObject.
2. Define shared global uniqueness/definition/cardinality/error/missing/snapshot
   checks; all selected callee callers and required forward dependencies still
   require executable proof. Audit existing object-view agreement obligations.
3. Keep original qualification/Completion and validate-all-before-mark ordering.
4. Positive: unrelated healthy SourceObject retained, selected Static closes.
   Negative: same-callee later caller/forward missing, unrelated Err/missing or
   changed source snapshot, foreign original, missing Completion, partial mark.
5. Focused entry/packet/Object regression and unchanged original app first-stop;
   no physical deletion permission or Object activation inferred from passage.
Read-only worker supports the split as an explicit owner contract change, not
an already-authorized weakening. Canonical catalog loan and validation domains
are resolved by the selected-S0 Decision below; no external wait or completion.

## Original catalog loan T0 — accepted (2026-10-12)

The existing Owned parameter catalog is one immutable Rc allocation shared by
Verified/Installed package, completed context and ordinary ledger. Issuer attaches
its same loan once before finish/co-seal; missing/duplicate attachment grants no
permission. Owned records remain non-Clone, no copied authority or new receipt.
Test-only malformed descriptors preserve all earlier rejection assertions.

Evidence: quick test build/link494.796s; 95 focused PASS/1 pre-change NoIncoming
baseline. Same-allocation/context lifetime, foreign/missing/duplicate/ordinal,
entry/Static packet/Object input and all adapted mutation families covered.
`/tmp/mimalloc-original-catalog-t0-v2-runtime-receipt.json`.
CLI build/link232.574s; unchanged original app0.050s/rc1/no EXE, identical
LexicalInstanceCall/source-only-object-actuals first-stop. All13 changed Rust
bytes match test/CLI/current snapshots; CLI receipt:
`/tmp/mimalloc-original-catalog-t0-cli-v1-runtime-receipt.json`.
Pointer/diff/new-hunk format PASS; coseal/install whole-file format debt matches
parent (`/tmp/mimalloc-original-catalog-t0-format-baseline.json`), not cleaned here.
Warning signatures unchanged (812 recorded in both test builds); no suppression.
No phase/selection/global execution change, old-edge retirement or app success.

Parent Static final-incoming D0 borrows this canonical catalog. The Decision
below closes its global source versus selected-callee execution domains and
reverse formal forward dependencies. Preserve unrelated class-view agreement: only view-free
SourceObject may be passively revalidated; required dependency stays executable.
Pre-T0 same-source read-only view lookup for allocate formal(owner33/binding1)
is None (`/tmp/mimalloc-static-final-object-view.log`); diagnostic old-binary
provenance, not fresh T0/S0 execution or permission. No automatic promotion.

## Static final-incoming D0 Decision — selected S0 (2026-10-12)

Decision: one existing entry owner validates the complete final incoming source
graph, but qualified opaque Static packet execution obligations cover the selected
callee and its reverse original-formal forwarding dependencies. This changes only
the Static packet contract; ordinary entry/Object whole-final execution stays.

Source authority + canonical issuer: the SAME retained original parameter catalog,
source incoming inventory, final definition/origin drafts, original forward rows,
qualified Static Rc and SAME callee Completion. No new authority/receipt.
Non-authority: pending phase alone, method names, header numbering, runtime tags,
old diagnostic memory offsets, missing class-view rows as execution evidence.
Fail-fast boundary: global duplicate/definition/cardinality/Err/missing checks;
required executable phase, exact source identity, forward coverage and class-view
agreement remain mandatory. SourceObject passive validation lends no executable
actual/result/Normal/ABI/artifact permission. Validate all before any route mark.

Smallest next slice: `MIRBUILDER-APP-MIMALLOC-LITE-STATIC-FINAL-INCOMING-S0`.
Production mapping: existing borrowed_static_packet_actuals_v1 -> entry owner
common core, for LayoutBox.class_id -> SizeClassBox.good_size. Replace that
Static call's whole-final execution obligation; preserve the normal/Object wrapper.
Shared owner and other callers stay; no physical deletion permission in this S0.

1. Keep checked_borrowed_entry_incoming as unchanged default wrapper. A private
   core accepts optional execution-owner scope; only the qualified opaque Static
   branch selects scope. Put closure validation in existing entry_owner_view child
   and SourceObject revalidation in existing formal_actuals owner. Do not mint a
   second checker/receipt or broaden field visibility. Source760/800 still apply.
2. Seed selected callee. Compute finite reverse closure from ORIGINAL forwards
   whose callee_formal.owner is included, never from pending actual variants. Match
   exactly one final incoming/opaque actual by call/target/site/ordinal/binding/
   source_formal/callee_formal; corroborate both drafts' origins. Require the
   pending actual's original Forwarded variant; reject orphan pending forwards.
   Add source_formal.owner, repeat to fixed point. Dependencies require final
   definitions and nonempty incoming, not invented source-only completion.
3. Project the SAME original inventory over that execution set, preserving its
   global/selected-owner vetoes. Compare one-to-one with final calls, including
   only the existing requested target_static/owner_view extension. Compare call,
   callee, full arguments, Static Rc pointer or full Instance target identity.
   Missing/extra/duplicate required calls reject. Sanctioned omitted Static
   callers were excluded at issuance and stay excluded: this proves inventory
   coverage, not a new all-source-callable census.
4. Unselected SourceObject may remain passive only when exact original call/
   callee/arguments/Instance target agree, SAME catalog regeneration equals the
   entire pending snapshot, and its opaque callee formals have no object_views
   membership. Existing view membership rejects rather than skipping agreement.
   Required/dependency SourceObject still rejects. All other phases use existing
   executable ordered checks; qualification/Completion/mark ordering stays.
5. Existing packet tests: unrelated healthy SourceObject+selected Static positive;
   source phase/candidate/ordered/original drift, Err/missing, class-view member,
   selected source-only and default global rejection negatives. Forward negatives:
   pending Forwarded->Integer, orphan/duplicate/root/site/ordinal/target drift,
   dependency missing definition/incoming, required original call removed. Check
   failed co-seal leaves every route unmarked. Reuse entry/packet/Object regressions.
6. Build once, focused positive/negative filters nonzero, classify reds and record
   source/binary snapshots. Fresh unchanged original app first-stop before/after.
   Required pointer/diff checks; qualified-route guard's unchanged raw_loop caller
   count debt remains reported, never waived. No full suite/CI required here.

Non-claims: S0 acceptance, production cutover closeout, caller-zero/old-edge
retirement, whole SizeClassBox/app/MirBuilder completion remain unproved.
Read-only worker confirmed original-inventory projection and sanctioned omissions;
no remaining external dependency or required future-green construction condition.

### STATIC-FINAL-INCOMING-S0 receipt — accepted (2026-10-12)

Landed `7901d434fa`; full acceptance/provenance is retained in that commit.
Only qualified opaque Static uses original-forward reverse execution closure;
view-free unrelated SourceObject requires SAME catalog/full-snapshot revalidation.
Default entry/Object and required inputs keep whole-final executable checks.
No new authority/receipt/source rewrite/ABI/result/artifact grant or deletion credit.

- V7 focused82PASS/1 original NoIncoming baseline: packet15, entry18, Object
  identity9/port2/slot12, actual26; no rebaseline. Exact runtime receipt and snapshot:
  `/tmp/mimalloc-static-scoped-s0-v7-runtime-receipt.json` and `...-snapshot-v7.json`.
  Forward/phase/catalog/view/coverage corruptions reject before partial route marks.
- V7 reflow-only equivalence: `...-format-proof.json`; fresh CLI matches final6 Rust
  files. Existing whole-file fmt debt remains; new actuals/owner-view/scope clean.
- CLI build251.974s; original app SHA256
  `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685`.
  pure-first/replay-none probe0.050s/rc1/noEXE; same LexicalInstanceCall/
  source-only-object-actuals. `...-cli-v1-runtime-receipt.json` records provenance.
- Fresh trace: scoped=true, GoodSize11/batch7, rejecting34->33. Original17->11 and
  33->17 make allocate33/batch29 REQUIRED. SAME sealed joins in `...-contract.log`,
  `...-map.log`, `...-forward-graph.log`, `...-tuple.log`; offsets diagnostic only.
- Warning signatures test812/CLI1735 unchanged. Pointer/diff pass; unchanged
  qualified-route guard debt raw_loop_child_entry count3 vs4 is NOT PASS/waived.
  Sources below760 (entry752, test parent556/child329).

Production calls scoped contract; full cutover/caller-zero/retirement and whole
SizeClassBox/app/MirBuilder remain unproved. Shared/default owners stay.

## Required Object forward D0 — current boundary (2026-10-12)

Selected row: `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-D0`.
The internal input-closure gap is allocateResult(owner34/batch30),
Body(1)/Initializer(0), argument0 `size` -> allocate(owner33/batch29), opaque
formal1, REQUIRED by good_size <- class_id <- allocate reverse dependency.
The source has `if size <= 0 { return new HakoAllocHandleResult(0,2,null) }`
before the call. That syntax is not Integer/entry/execution authority by itself.

Smallest next design: inspect SAME source-only draft/original incoming and existing
operation/Normal proof owner for that guarded forward. Establish whether the
existing checked argument or borrowed entry finish can lend executable input at
this exact site without inventing class authority or reissuing the formal catalog.
Keep OpaqueHandle/tagged ABI, source evaluation/Fault order and true/false scopes;
Integer Normal proof and usize range checking remain distinct. Callee source-only
phase is not permission; all required callers/dependencies must still validate.

Worker checks the concrete authority gap and existing producer/consumer, then main
records one Decision and smallest implementation slice here. No source annotation
workaround, pending-phase cast, global execution waiver, new receipt chain, parked
lane reopening or whole-app claim. This is internal design work, no external wait.

### Owner-selected milestone and D0 refinement (2026-10-12)

Near milestone: unchanged mimalloc-lite, selected pure-first/replay-none physical
route, EXE generation -> execution -> exact suite output. Full retirement and
selfhost remain later REQUIRED milestones of MIRBUILDER-FINAL-PIPELINE-v1, not
removed or counted complete. Current work stays ONLY Required Object Forward D0.
After its smallest S0: focused tests, then the SAME original app; record first-stop
by terminal AND owner/site plus EXE/run result. If it does not move, audit repeated
proof construction/common owner before stacking another small patch. Do not add
unrelated improvement/audit lanes or caller-specific authority/receipts.

Read-only worker found existing Integer Normal authority: SAME CheckedIntegerGuard
Rc and BorrowedGuardedActual's exact call/ordinal/site/binding reach survive source
pruning. Current consumer uses it only as a domain seed. Compare materialization
of source-only definitions accepts completed STATIC scalar cohorts; Instance owner34
has no matching consumer. Typed Object finish requires ExactI64 scalar parameters,
so it cannot activate this OpaqueHandle input unchanged.

Decision direction: declaration-bound tagged carrier binding in existing entry
owner, checked Normal continuation in existing compare owner, and exact guarded
forward reconstruction in existing actual/input owner. Keep formal OpaqueHandle
and kind/payload ABI; use existing original catalog/signature, not caller consensus.
An unused declaration's complete-empty observed incoming may bind its DECLARED
carrier, never certify executable incoming or Integer/class/ownership/whole body.
Required callee33 incoming stays complete; no outgoing edge is omitted.

### MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0 Decision and bounded tasks (2026-10-12)

Decision: connect the existing declaration-tagged entry, checked Compare Normal
and guarded argument owners for this required forward. No caller consensus,
class inference, source rewrite, phase cast or separate proof ledger.
Source authority + canonical issuer: SAME original parameter catalog/signature,
source use draft and CheckedIntegerGuard/BorrowedGuardedActual; existing entry,
compare and actual/input owners issue and lend their corresponding obligations.
Non-authority: source-only phase, comparison syntax, diagnostic owner numbers,
physical metadata alone, missing packets or completed-empty fabricated inventory.

Bounded profile: original source-only InstanceBoxMethod with OpaqueHandle formals;
at least one checked Compare and one exact guarded outgoing Object input. EVERY
formal use must be CompareOperand or an exact guarded UnresolvedArgument; roots,
ordinals, bindings and declaration mode must match SAME original catalog. Project
original incoming with its vetoes preserved; only complete-empty observed incoming
may bind this declaration's tagged carrier. Nonempty real incoming stays covered.
Neither case grants an executable incoming loan, Integer entry or whole-body permit.

Smallest next slice (one responsibility: guarded required input forwarding):
1. Existing entry owner lends that bounded declaration view as SelectedTagged.
   Check supplied parameter kinds as well as roots/ordinals/bindings against the
   original catalog. Receiver law comes from its original declaration mode;
   all nonempty incoming modes agree. Do not infer mode from physical params.
2. Existing guarded-actual owner lends exact call/ordinal/site/binding/formal and
   SAME Compare Rc correspondence to the draft. No retained batch/signature copy
   is added solely to repeat source validation already closed at issuance.
3. Existing Compare owner accepts that Instance declaration view; physical lowering
   still emits the original checked comparison, preserves operand order/tag Fault,
   and final use closure independently verifies its Normal CFG dominance.
4. Existing actual/input owner reconstructs the guarded outgoing input from SAME
   original target/arguments/catalog/signature. Keep callee33's complete incoming
   and original forwarding dependencies. Update SourceObject demand-time rechecks
   to recognize only this reconstructed guarded contract; never allow arbitrary
   Executable phase replacement. Preparation proves a plan; final publication
   independently owes the actual Compare/Normal path, not a preconstruction result.
5. Validate zero-call tagged DECLARATIONS independently in BOTH final consumers:
   compiled_entry_contract/borrowed_call_incoming.rs and invoke_borrowed_calls.rs.
   Published target -> handoff.callables().owner_for_canonical_key; MIR side uses
   the unique canonical definition key/symbol -> root.callables() lookup. Demand
   finished source/owner/symbol, bounded profile, complete-empty original incoming,
   recorded entry values and exact params/carriers/receiver correspondence. Reuse
   BorrowedCallUses::entry for published full-use closure. Keep incoming==original,
   real-call actual coverage and physical/witnessed coordinate equality unchanged;
   add no fake packet, call slot or coordinate. Share the existing source lender.
6. Existing-owner positives/negatives cover the guarded declaration/forward and
   zero-call final verification. Refuse foreign/missing catalog/signature/guard,
   supplied kind drift, rebinding, pre-compare/branch mismatch, omitted real calls,
   missing physical Compare/dominance and outgoing execution after guard Fault.
   Bool/object fail at the original Compare; usize range remains its original
   write boundary. Body/constructor/result/cleanup residual gates stay mandatory.
7. Focused nonzero filters and required guards, classify reds, then rerun SAME
   unchanged mimalloc-lite with fresh binary provenance. Record terminal AND
   owner/site and EXE/run/output. If unchanged, inspect common proof owner before
   another patch. Commit/push only selected files; keep this card below1000 lines.

Read-only worker independently confirmed both zero-call gaps and existing canonical
owner lookup (also used by birth_calls.rs). This closes the D0 correspondence;
S0 is selected, not accepted. Changed source beyond760 requires responsibility
split before800; put bounded entry logic in its existing owner-view child.
Non-claims: Integer/class/ownership entry, whole body completion, S0 acceptance,
EXE success, production closeout/caller-zero/retirement and full product completion.
