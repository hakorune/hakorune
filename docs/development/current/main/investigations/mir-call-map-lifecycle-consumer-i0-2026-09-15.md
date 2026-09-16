Task: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0
Parent: mir-call-map-local-entry-source-i0-2026-09-15.md
NextCard: C5c-1 owner exit/cleanup co-seal — review of
0027c56b (2026-09-16) found C5b admits owners whose terminal
evidence failed; C6-3 (C v2 nested-call lane) is implemented
in flight and lands first as the already-open slice
Route note (2026-09-15): the call-arg and contained-descendant +
array-element cards landed — merged loop1 now passes entirely and the
first failure is `map_install_owners` (`Err(())`), this card's
territory.
Implementation permission: pending six-line brief acceptance
---

# Map lifecycle consumer I0

## Six-line brief

```text
Decision: pending — the merged route now stops at
[callable-semantic-package/install] MapLifecycleConsumerMissing; census
which Map lifecycle consumer the install stage requires and which merged
declaration fails to provide it.
Source authority + canonical issuer: pending census of the install
issuer (normal_callable_semantic_package install path).
Non-authority: pending.
Fail-fast boundary: pending.
Smallest next slice: name the issuer, the required consumer row, and the
failing merged declaration; then bound the fix.
Non-claims: production caller switch, physicalization, legacy retirement.
```

`Census boundary: merged entry program -> callable-semantic-package
install stage; pending first investigation.`

## Entry contract

The previous card normalized 15 malformed `birth ... return 0` corpus
sites to `return void` (the accepted Unit contract — `Unit Birth has no
result`). The merged route advanced past
`OrdinaryNew/BirthCompletionNotUnit` and now stops at
`MapLifecycleConsumerMissing` during package install.

## Acceptance

Pending: the merged entry advances past the install-stage
MapLifecycleConsumerMissing to the next named terminal.

## Entry investigation (2026-09-15)

Issuer: `install_map_preflight.rs` — `preflight_map_install` runs at
package install, before catalog mutation.

Arm analysis (`map_install_owners`, ordinary_new_terminal_access.rs):

- `requires_map_lifecycle_consumer` is true in the merged route: 40
  `new MapBox()` initializers produce map rows in completion root_flow
  (no `{k:v}` MapLiteral syntax exists in merged — every `{` is inside
  JSON string literals; MapLiteral preflight loop is vacuous).
  **Correction (superseded by Arm pinning below):** `new MapBox()`
  produces no `flow.maps()` row — the map rows come from 31 `%{...}`
  MapLiteral sites (the `%{` sigil, distinct from JSON `{`), and
  `app_main_identity` is `Some` on the merged route. `new MapBox()`
  sites travel OrdinaryNew claims; their describe arm is a separate
  bounded row.
- `map_install_owners` then requires `app_main_identity.is_some()` —
  the merged route is a library compile without an AppMain anchor, so
  the check fails closed at `Err(())` -> `MapLifecycleConsumerMissing`.
  (Owners-vs-root and per-owner initializer-chain arms sit behind this.)

Census boundary: merged entry program -> `new MapBox()` initializers;
includes every resolved body; result 40 sites across ~10 functions
(result/m/def/info/m/v locals in parser/emit boxes).

Open design question for the Decision: the Map lifecycle consumer
contract is AppMain-scoped today. The merged route needs either (a) an
explicit non-AppMain map-lifecycle admission row, or (b) the consumer
contract extended to the merged batch's actual root identity — never a
silent bypass of `app_main_identity`.

## External design consultation (2026-09-15, ChatGPT Pro review of 7618c185)

**Recommendation: (b) generalized per-function-owner lifecycle contract.**
The missing piece is not owner identity — every function already carries
`FunctionOwnerIdV1` + completion + root_flow. What is missing is the
contract that the physical route undertakes that function's lifecycle.
The batch must NOT become the runtime Map owner: 899 compiled functions
are not 899 simultaneously-live ownership scopes.

Responsibility split:

| unit            | responsibility                                        |
| --------------- | ----------------------------------------------------- |
| each callable   | own Homes, normal/Fault cleanup, arg/result transfer  |
| batch/package   | complete callable-to-consumer correspondence          |
| execution entry | Fault frame start/end, final report, process exit     |

Removing the AppMain condition alone does not finish the work —
downstream keeps the same assumption:

- install checks the AppMain direct-call loan structure and fixed
  i64 call counts (install_map_preflight.rs)
- backend admission requires a retained root and collects its direct
  calls (lifecycle_admission.rs)
- the C consumer requires `functions[0]` as process root
  (hako_llvmc_ffi_published_lifecycle_physical_v2.inc)

Ordinary callables already have the borrowed-Fault-frame + result-output
+ status ABI; emit functions from the sealed callable set and bind a
process adapter only for products that need an execution entry.

Minimum sealed facts (not five new wrappers): identity, complete target
scope, exit+cleanup order, boundary transfer, physical consumer.
`prepare_install` atomicity (return package on failure, stop before
catalog mutation) must be preserved.

Pitfalls named: static-owner vs runtime-scope confusion, mid-transfer
Fault, cleanup order derived from batch order, GC-root confusion.

Pre-production concerns to fix before the source-result issuer connects
to a production caller (review-verified, taskified below):

1. source-admission witness is attached after SourcePlan, not consumed
   as an admission condition — missing/foreign lineage does not reject
   pre-package (normal_callable.rs, A3 tests pass without witness).
2. source-result issuer correspondence verification relies on arg
   name/count; a different declaration's resolved input with the same
   arg shape could seal a wrong-source result — needs direct
   declaration↔resolved-owner identity binding. Test-only today.
3. the same issuer treats every BlockExpr as a transparent tail
   wrapper; a nested block with preprocessing (rebinding then tail
   read) could seal a stale value class — smallest fix is rejecting
   blocks with preprocessing until the bounded family needs them.
4. `: void` functions mixing `return null` (Value) and `return void`
   (Void) fail set-uniformity — the language contract allows both;
   handling must be unified at the declared-result boundary. — resolved by
   C4: the declared `: void` contract now normalizes `return null` to
   `(Void, ExplicitNull)` at classification, so mixed Unit spellings seal
   one `ExplicitUnitSet`; unannotated `return null` remains an explicit
   value return (the `T|Null` idiom), where a Value/Unit mix still
   rejects `ReturnClassificationInvariant`.

Suggested next completion unit: compile two small non-AppMain functions
through the same consumer and verify normal+Fault cleanup on actual
calls; classify the 40 `new MapBox()` sites into local-only / returned /
stored / arg-transferred in parallel. "Advanced to the next terminal"
does not count as consumer completion.

## Decision (accepted 2026-09-15)

**Option (b): generalize to a per-`FunctionOwnerIdV1` lifecycle-undertaking
contract.** The missing piece is the contract that the physical route
undertakes each function's lifecycle — every owner already carries
completion + root_flow + terminal relation + claims. The batch must never
become the runtime Map owner. Option (a) survives only as the
implementation scope (first non-AppMain application of the shared
contract); pseudo-AppMain (c) is rejected.

Caveat from worker verification: `/tmp/merged_entry.hako` DOES contain
`static box Main { main() }` (L19038), so `app_main_identity` is likely
`Some` and the actual failing arm is probably the loan/target/owner
mismatch (install_map_preflight.rs:31-62) or the no-loan
owners-vs-root arm — pin the exact arm first when card 5 starts.

## External design consultation #2 (2026-09-15, ChatGPT Pro review of
94309d2a — post-Facts-completion)

Recommendation verified against code: **existing-receipt co-seal +
capability matching against an implemented lowering consumer.** C5's
contract definition can land before F3/F4; admitting a Map-returning
owner still requires them.

**Q1 — co-seal, not a new independent receipt.** `PreparedNormalCallable
SemanticPackageInstallV1` (install.rs:355-404) already owns the package
and commits once after prepare succeeds — extend it to carry the
`selected_lowering_consumer` that was verified. If a
`MapLifecycleUndertakingV1` row exists it must be private
references/indexes over existing products — never a re-copied
owner/entry list (aggregate issues no new meaning). The sealed relation
the aggregate proves: *every owner's obligation for this source/package
is fully described AND the selected lowering consumer can undertake all
of it.*

**Q2 — requirement definition and satisfaction split; class names are
not a registry.** C5 may describe unmet requirements, but owner install
success requires the needed lowering capability to be already
implemented. Converge entry classes onto operation contracts — value
create/read, entry store, ownership transfer/share/borrow, return
handoff, Normal/Fault cleanup — since `MapLocal` and `NestedMap` share
the physical Map handle but differ in borrow/create/transfer
responsibility (same representation ≠ same operation contract).
Pre-install must NOT demand generated MIR/OBJ — the package is
pre-Builder; backend admission is out of scope (package README).
`BuilderInstallConsumerV1._private` (bridge.rs:19-40) is a one-shot
token, not Map-capability evidence — the capability check must connect
at the existing boundary. Stages: `prepare_install` (meaning fixed +
implemented consumer covers it) / Lower + completion seals (operations
actually generated and consumed) / physical+backend admission /
C8 runtime proof.

**Q3 — F3 is an independent Facts extension; the transfer guarantee
built on it belongs to the lifecycle contract.** F3 supplies which
return site returns which source value/binding; Recipe/Verify decides
post-return responsibility; the undertaking asks whether the consumer
can execute that verified relation. F3 precedes admitting Map-return
owners, not C5's contract definition. `return m` is not automatically
an exclusive move — a shared Map may transfer "one release
responsibility"; the language's existing ownership rule decides, F4
does not redefine it. F3 and cleanup must reference the same return
site, same value, same ownership state — a bare terminal variant is
insufficient.

**Q4 — per-owner receipts plus call-edge conformance.** "Owners with
map rows" under-covers: `use()` that receives a Map without creating
one still has obligations. Target set = sealed callables involved in
Map create/receive/hold/transfer/release. Batch verifies three things:
(1) every owner/site has its contract + consumer match, (2) every call
edge's callee convention matches the caller's handoff/receiver/Fault
state, (3) import/export carry explicit boundary contracts. Many owners
may share one consumer implementation. AppMain loan evidence stays for
the paths that need it but is removed as the deep-callee coverage
criterion; the process adapter binds only to explicitly selected
execution products (source containing `Main` ≠ emitting a process
entry this compile).

**Q5 — split C5 contract definition from C5 admission connection.** No
reverse dependency: C5 define → F3 → F4 → C5 admit-connect → C6
downstream connect (lifecycle_admission retained-root restriction is a
C8 prerequisite — admit_lifecycle still requires retained root +
root direct-call collection, lifecycle_admission.rs:28-66) → C8.

**New pitfalls (priority order):**

1. **Returned map containing borrowed maps** — `local child = %{..};
   return %{"child" => child}` (MapLocal non-consuming borrow): parent
   return-transfer alone leaves a dangling borrow if callee cleanup
   ends `child`. Liveness of borrowed targets inside returned values
   must be verified; borrow-only evidence → reject. (Not a runtime
   dangling observation — physical side still stops earlier.)
2. Expected set must come from sealed membership, not successful
   receipts — current owner enumeration uses
   `filter_map(...as_ref().ok())` (ordinary_new_terminal_access.rs:
   93-110), which drops failed completions silently.
3. Recursion/mutual recursion — do not require callee body completion
   first; fix the boundary contract, then verify body+edge match.
4. Callers discarding return values still owe release responsibility.
5. Same Map stored into multiple entries — prevent double-move; if
   shared, pin per-reference responsibilities.
6. Duplicate-key overwrite — old-value release plus new-value
   store-failure responsibility.
7. Post-seal internal mutation — the same package + consumer verified
   at prepare must reach commit; prevent substitution/pre-consumption.

**C8 acceptance refined**: `make_map()` + `use_map()` (receive-only).
Verify Normal (returned map usable after callee cleanup, freed in
caller), callee Fault (no result, initialized resources reclaimed),
caller Fault (received map freed in caller), missing evidence → reject
before catalog mutation. Observe release/reclaim/live handles, not only
exit code; name the admitted entry classes.

## Ordered bounded card queue

| #   | Card                                                              | Depends      |
| --- | ----------------------------------------------------------------- | ------------ |
| C1  | source-result issuer owner binding — landed: `verify_source_input_identity` binds the resolved input's declaration node to the catalog row field-by-field (name/params/param_decls/return type/body/uses/attrs); `ForeignResolvedInput` rejects before ledger consumption. Residual honestly bounded: content-identical foreign declarations are indistinguishable (parser nodes carry `Span::unknown()`), but identical content produces identical product rows — only the owner label could differ | — |
| C2  | source-admission witness — landed: `issue()` now always returns an attested witness (zero rows allowed), materialization fails merged+source-backed without one (`MissingCoverageWitness`), and `NormalRootExecutionConsumerV1::consume_once` enforces `merged lineage ⇒ witness` as `SourceAuthorityUnavailable` before package effects | — |
| C3  | BlockExpr prelude accounting — landed: `classify` rejects `NonEmptyBlockExprPrelude` when the sealed statement inventory shows `BlockExprPrelude` children of the wrapper site; empty-prelude wrappers stay transparent (folding prelude statement effects is a separate semantic slice) | — |
| C4  | `:void` mixed `return null`/`return void` — landed: `classify_return_value` now takes the declared contract and normalizes `return null` to `(Void, ExplicitNull)` under an explicit `: void` annotation (mixed `return null`/`return void` seals one `ExplicitUnitSet`); unannotated `return null` stays an explicit Value return. Supersede note added to the 7/25 exit card; `types.md` records the declared-boundary rule | — |
| C5a | contract definition — landed: `map_lifecycle_undertaking.rs` defines `MapLifecycleOperationV1` (11 operations derived from sealed `MapHomeFlow` rows), `MapEntryBorrowV1` borrow evidence, `MapCallEdgeContractV1` edge vocabulary, `MapLifecycleConsumerCapabilityV1`, `describe_map_lifecycle_obligations` (sealed-membership enumeration, named describe issues), and `verify_map_lifecycle_undertaking` (obligation ⊆ capability seal). Not yet connected — C5b wires it into `preflight_map_install` + `PreparedInstall` | — |
| C7  | merged `new MapBox()` census — landed: exact 39-site classification recorded in Census section (27 returned / 11 nested-stored into a returned container / 1 truly local / 0 arg-transferred at creation site); 38/39 maps egress through a return boundary | — |
| F3  | `TerminalRelationV1` for non-i64 value returns (Facts extension) — landed: `Value(TerminalValueReturnV1)` records the exact returned source (`MapLiteral` site / `MapLocal` / `Home{binding,acquisition}` / `Handle` root / `String`/`Null`/`Float` literal); `return void` spells the Unit terminal; returned map-local/Home bindings leave terminal cleanup. No physical ABI — a root `Value` terminal stops at `root-result-missing`, and `map_install_owners` rejects the owner at the non-i64-terminal arm (same `MapLifecycleConsumerMissing` terminal, different arm than the old `terminal_homes` error). Gate boundary: the walk runs for AppMain roots and `has_map`/`child_new_ready` children; other child owners keep no relation — fail-closed | — |
| F4  | Map return/argument physical ABI + entry-class operation contracts — landed F4a..F4d: `nyash.map.storage_move_v1` export; `InvokeCallResultKind::Map` + verifier lease-transfer (`Return{map}` consumes the live lease, `map-return-not-live` rejects a spent one); `ordinary_map` role + `"result":"map"` wire + C validator/flow/emit (caller `%map<b>` out storage, callee `storage_move` into `%out_map`); builder dispatch on the sealed flow destination (`ReturnBoundary` → `emit_return`, other destinations stay `map-destination-unsupported`); disposition rows carry the callee's terminal-relation result class; `map_install_owners` admits `Value` terminals bounded to exact map-source returns. Focused: 5/5 `map_physical_dependency` + 3/3 verifier `invoke::map_tests` additions + 228 package + 102 verification + 14 physical_program + kernel `storage_move`. Residual: call-result `Map` rows cannot yet be produced — both admitted call lanes gate callees on `result()==Some(I64)`/`:i64` index; the kind↔role mechanism is the seam C5b plugs into. Map arguments remain unadmitted (named reject) | C5a, F3      |
| C5b | admission connect — landed: `preflight_map_install` describes every sealed member's obligations (`describe_map_lifecycle_obligations` — full batch-membership `MapLiteral` enumeration, named describe issues) and verifies them against `BuilderInstallConsumerV1::map_lifecycle_capability()` — the consumer's own declared set: create / scalar+transferred entry store / displace / return handoff / Normal+Fault cleanup; OwnershipShare and slot/argument/contained handoffs stay undeclared and fail closed at verify. The sealed undertaking rides `PreparedInstall` into `InstalledNormalCallableSemanticPackageV1` (`map_lifecycle_undertaking()`), pinned by a 2-owner undertaking assertion through commit. Retired bounds: `owners ⊆ targets ∪ root`, `expected_local_calls`, the `>5` distinct-target cutoff — the undertaking replaces them as coverage proof. Retained as scoped AppMain product evidence only: unspent affine slots (`has_taken_slot`), map-target ⊆ described owners, no self-target, `map_install_owners` per-owner lane admissibility inside the AppMain lane, and the per-owner initializer/alias annotation chain. Describe correction: `Local{kind:None}` entries describe OwnershipShare + borrow evidence (a kind-less local is a live-binding store, not a scalar copy) — matching the physical lane's `map-value-consumer-missing`. Deferred honestly: `MapCallEdgeContractV1` matching (both kinds unreachable — ArgumentHandoff uncovered at per-owner verify, map-result callees still `result()==Some(I64)`-gated); `new MapBox()` claim-family describe arm; the `result()==Some(I64)`/`:i64` callee-result seam itself. Test flips (intended): non-AppMain covered owners install, repeated covered targets install, `return %{...}`/implicit/unit exits install; still rejected: unavailable/stale rows, malformed annotations, share/slot/argument/contained obligations | C5a, F3, F4 |
| C6  | downstream contract split — decomposed: C6-1 `admit_lifecycle` (landed), C6-2 physical doc per-function call edges, C6-3 C v2 nested-call lane, C6-4 rootless cohort (deferred — merged-route shape, not needed by C8's rooted acceptance) | C5b |
| C6-1 | admission contract split — landed: `admit_lifecycle` no longer requires the retained root as membership authority. Ordinary membership is every sealed `Call{I64\|Map}` edge target across the module (`ordinary_call_names` — was: root-only, I64-only); the retained root is an optional process-entry input (root-scoped result checks apply only while bound; `retained_birth_abi` defaults empty). `validate_functions` takes `Option<&str>` — `None` removes the entry exemption. Kept strict: `script-root-not-callable`, `candidate-unavailable`, `function-not-cataloged`/`function-not-birth`, `birth-call-drift`, return-only retained-Birth re-check. Non-claims: emission unchanged — `issue_lifecycle_physical_program` still requires the retained root (`root-missing`/`root-handoff-missing`) and emits only the root's direct callees (nested callers still `ordinary-call-membership` — C6-2); upstream `into_artifact_parts` `uncovered-lifecycle-function` still gates rootless ordinary lifecycle functions before admission (C6-4); C v2 `fi==0` ordinary-call restriction unchanged (C6-3). Focused: 4/4 `lifecycle_admission` (new pin collects a nested caller's `I64`/`Map` edge + `None`-root rejection); 85/87 `normal_default_pipeline` + 81/84 `published_backend_view` — all failures parent-reproduced baseline (local-commit completion, LLVM `no_lowering_variant` array routes) | C5b |
| C6-2 | physical doc per-function call edges — landed: `issue_lifecycle_physical_program` walks sealed `Call{I64\|Map}` edges transitively from the retained root (BFS over cataloged definition symbols); `call_sets` keeps each function's own `OrdinaryCallSite` multiset and `issue_function_with_module` validates the callee's nested rows instead of rejecting them (`&[]` retired). `ordinary_sites` keeps one physical result contract per callee key across all callers (`ordinary-result-contract-drift` on I64/Map mix). `CompiledEntryOrdinaryCallV1` gains `caller_function_index`; the contract collects ordinary rows from every emitted Root/Ordinary function, preserving program index relationships (`compiled-entry-ordinary-unissued` still requires every emitted ordinary to be referenced). Birth calls and function order unchanged; no root-only fallback. Positive pin: `nested_ordinary_call_chain_emits_per_function_call_rows` injects a nested edge into a module clone (the producer lane cannot yet seal a call inside an ordinary callee — same upstream gap as C6-4), re-runs `try_new`+`bind_finalized_root_handoff`+`admit_lifecycle`, and asserts 3-function emission, per-caller rows `(0,helper)`/`(helper,inner)`, and the callee's own row inside its emitted body. Non-claims: C v2 `fi==0` ordinary-call restriction unchanged (C6-3); rootless cohort unchanged (C6-4); `admit_lifecycle` unchanged since C6-1. Focused: 15/15 `physical_program` + 3/3 `compiled_entry_contract` + 4/4 `lifecycle_admission`; 86/88 `normal_default_pipeline` + 82/85 `published_backend_view` — all failures parent-reproduced baseline | C6-1 |
| C6-3 | C v2 nested-call lane — in flight: flow bound `fi \|\|` → `(fi && !ordinary)` admits ordinary callers at fi>0 while Birth callers keep rejecting; per-block `%call_out<b>` i64 slot for nested ordinary calls inside ordinary functions (the caller's own `%out_i64` stays reserved for its return handoff — the exact gap the review noted for scalar calls inside map-returning callees); `invoke_normal_result` loads `%call_out<invoke_block>` at fi>0, `%ordinary_out` at root. Map nested calls reuse the existing per-block `%map<b>` lane. Positive+negative C fixture: `published_lifecycle_v4_nested_call_test.c` | C6-2 |
| C5c-1 | owner exit/cleanup co-seal — review finding on 0027c56b: the undertaking seals from per-site Complete rows alone; `terminal_homes()` failure or a missing terminal relation still admits (`local a = new Page(7) local m = %{"a" => a} return a.value` keeps a completed map row while owner exit analysis fails — the existing `map_transfer_invalidates_old_local_and_alias_field_observation` test pins only the flow state, not install rejection). The old `map_install_owners` exit check now runs only inside the AppMain direct-call loan lane. Fix: co-seal every described owner's terminal evidence (terminal_homes success + terminal relation) regardless of loan; describe `ReturnHandoff` from the sealed terminal relation (`MapLocal` returned source) so `local m = %{}; return m` cannot skip the handoff obligation. Negative pin: extend that existing test to assert `prepare_install` rejection + vacant catalog | C5b |
| C5c-2 | EntryStore obligation precision — review finding: `EntryStore` is described unconditionally per entry, so String/NestedArray entries pass the scalar+transferred-only capability and stop only at lowering, after catalog mutation (`%{"op" => "const"}` owner with no store consumer). Fix: store obligations carry the consumer's actual value-representation/ownership precision — reuse the existing lane predicate inside the common preflight; no new generic registry | C5b |
| F5  | map-result call lane — the `result()==Some(I64)`/`:i64` callee gates deferred from C5b: admit sealed map-terminal callees, issue `Call{result:Map}` rows, and let the receiving caller consume the `InvokeCallResultKind::Map` projection (F4 verifier + `ordinary_map` wire already landed) | C5c, F4 |
| C1-id | opaque source identity — pre-production homework on a test-only issuer: `verify_source_input_identity` is content comparison; content-identical foreign declarations stay indistinguishable. Bind to the batch-issued opaque source identity before any production connection | C1 |
| C6-4 | rootless cohort — deferred: upstream `uncovered-lifecycle-function` coverage, `FinalizedRootHandoffV1` library variant, doc marker (merged-route shape, not needed by C8's rooted acceptance) | C6-1 |
| C8  | two-function non-AppMain consumer acceptance (normal+Fault cleanup, refined above) | C5c, F5, C6 |

C1–C4 are pre-production fixes on test-only issuers — cheap and
independent. C5a is this card's core deliverable; C5b/C6/C8 follow it;
C7 informs scope. Per consultation #2: C5 contract definition does not
depend on F3/F4 — admitting a Map-returning owner does. Full worker
verification evidence lives in the card audit trail.

C2 verification note (2026-09-15): the broader module run surfaced 7
unrelated reds — `published_consumer_*` ×2 (documented baseline at
`b61aef93ec`), `normal_callable_semantic_source` parity/ledger ×3
(`freeze:contract`/`runtime-box-fate-retired`), and
`production_skip_while_*` ×2 (`DynamicCarrierMismatch` / unwrap None).
Parent replay at `1ab879453d` reproduced all 7 identically → classified
known baseline debt (`ParentFailCurrentFail`), not current-change.

F4 verification note (2026-09-16): the full `--lib` run surfaced 5
failing rows plus a SIGABRT — `normal_callable_semantic_source`
parity/ledger ×3 (`mixed_nonplain_batch_*`,
`callable_parameter_and_local_*`,
`callable_entry_local_variable_*`), `direct_call` ×2
(`source_backed_app_main_direct_call_consumes_affine_loan`,
`main_f1_rejects_direct_call_and_nested_owner_before_lowering`), and
`source_backed_loop_keeps_invocation_scope_and_ledger_route` stack
overflow. Parent replay at `bd19308759` reproduced all 6 identically →
known baseline debt, not current-change.
`compatibility_loop_uses_legacy_child_terminal_without_callable_scope`
was marked FAILED in the aborted full run but passes both standalone and
in the `loop_scope` module run on this diff → order-dependent suite
flake (parallel env leakage), not current-change. The C shim TU passes
`cc -fsyntax-only -Wall` (the `storage_move` return emit initially
shipped a 12-of-14 `%u` arg mismatch, fixed; remaining warnings are
untouched baseline sites).

C5b verification note (2026-09-16): the package suite is 230/230 green
after the admission flips (9 pins updated from the old AppMain-shape
expectations to undertaking semantics — see the C5b row for the flip
classification). The full `--lib` run surfaced the same baseline set
before the `source_backed_loop_*` stack-overflow SIGABRT truncated it:
`normal_callable_semantic_source` parity/ledger ×3 and
`actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`
— the latter newly surfaced in this run and reproduced identically at
parent `1397cb2b5c` → `ParentFailCurrentFail` baseline debt, not
current-change. `compatibility_loop` was green in this run, consistent
with its order-dependent classification. Production-caller review:
all `prepare_install` callers keep their existing signature — the
capability is the builder consumer's static declaration
(`BuilderInstallConsumerV1::map_lifecycle_capability()`), not a
per-call parameter; the one-shot token remains provenance only. No
legacy AppMain-only admission path remains as a fallback — the
undertaking is the single coverage criterion and `map_install_owners`
runs only inside the loan branch as scoped evidence.

## External design consultation #3 (2026-09-16, user review of 0027c56b)

Direction accepted; two pre-catalog gaps and one deferred identity
residual raised. Both gaps are acceptance-boundary holes — known rejects
leak past `prepare_install` and stop only after catalog mutation.

1. **Owner exit/cleanup not co-sealed.** `describe_map_lifecycle_obligations`
   requires a Complete flow row per declared `MapLiteral` site but never
   checks the owner's terminal evidence: `local a = new Page(7)
   local m = %{"a" => a} return a.value` completes the map row while
   `terminal_homes()` fails — the old `map_install_owners` exit check
   now runs only inside the AppMain direct-call loan lane. Fix (C5c-1):
   co-seal every described owner's exit/cleanup evidence regardless of
   loan; `ReturnHandoff` must also derive from the sealed terminal
   relation (`return m` of a map-local), not only `ReturnBoundary`
   destinations.
2. **`EntryStore` too coarse.** Every entry describes `EntryStore`,
   which the declared capability covers — String/NestedArray entries
   pass preflight and stop only at lowering (`%{"op" => "const"}` owner
   with no store consumer). Fix (C5c-2): store obligations carry the
   consumer's actual value-representation/ownership precision, reusing
   the existing lane predicate inside common preflight — no new generic
   registry.
3. **C1 identity residual (pre-production).**
   `verify_source_input_identity` is content comparison; content-identical
   foreign declarations stay indistinguishable. The issuer is test-only
   today — bind to the batch-issued opaque source identity before any
   production connection (C1-id).

Accepted order: C5c fixes → map-result call lane (F5) → C6 → C8
normal/Fault execution. The review's C6 emitter note (an ordinary
function's scalar call result has no out-slot — `%out_i64` is the
callee's own return handoff) is exactly the C6-3 lane already in flight:
per-block `%call_out<b>` covers ordinary callers including
`ordinary_map`.

Docs hygiene: `CURRENT_STATE.toml` `latest_card_summary` predates
F4/C5b/C6-1/C6-2 — synced alongside this taskification.

## F4 Decision (2026-09-15, worker-audited physical layers)

**Map return ABI = caller-owned opaque out storage + `i32` status** — the
`%out_i64` convention generalized to checked-Map storage, no new runtime
publication:

- callee role `ordinary_map`:
  `define internal i32 @hako_lifecycle_ordinary_map_N(ptr %frame, ptr %out_map, [i64 receiver], [i64 args])`
- callee `return <map>`: the MIR `Return{value}` is the single lease-transfer
  point (verifier consumes the LV4_MAP lease instead of `map-missing-end`);
  C emit runs `nyash.map.storage_move_v1(dst=%out_map, src=%v)` +
  `storage_dispose` on the moved-from storage. Uniform for `return %{...}`
  and `return m` — no special "construct into out" path.
- caller `ordinary_call` with `"result":"map"`: `lv4_map_alloca` reserves
  `%map<block>` in the call's origin block; `invoke_normal_result` projects
  it as a live LV4_MAP lease the caller must consume (End / install /
  return) — the existing projection line already works.
- runtime: `nyash.map.storage_move_v1(dst, src)` admits src (`MAP_TAG`),
  requires `Phase::Live`, bitwise-moves `CheckedMap` into fresh dst storage
  (`Mutex`/`MapTable`/`Vec` are not self-referential), leaves src as
  `unissued` (disposable). No i64 handle — `fault_checked_map.rs` forbids
  host publication.
- Fault semantics: unchanged — callee's mid-construction fault path already
  ends/disposes its live maps; caller never reads `%out_map` on status!=0
  (projection only exists on the normal landing).

Bounded order:

- F4a runtime: `storage_move_v1` export + focused test.
- F4b MIR: `InvokeCallResultKind::Map`; verifier lease-transfer rule for
  `Return{map}` (escape exception + `map-missing-end` consumption).
- F4c wire+emit: `ordinary_map` role, `"result":"map"`, physical signature
  result kind, JSON encode, C validator/flow/emit.
- F4d builder+package: `return %{...}` (ReturnBoundary emission) and
  `return m` value return probe, `Call{result:Map}`, signature result for
  map-returning callees, `map_install_owners` Value arm bounded to
  map-source terminals.

F4d integration decision (2026-09-15, traced through the admitted lanes):

- `return %{...}`: `lower_callable_map_v1` dispatches on the sealed flow
  destination, not on the initializer index. `ReturnBoundary` → a
  dedicated `map::emit_return` that begins emission against the flow
  row's destination statement (== the completion's `explicit_site`) with
  no binding/declaration; `LocalBinding` keeps the initializer path;
  other destinations stay fail-closed. `MapLocalProgress` binding and
  declaration become `Option` — return-bound rows have neither.
- `return m` (installed `%{...}` local): the existing generic
  value-return path already emits `Return{map}`; F3 already removes the
  returned binding from `terminal_homes`, so no `Map::End` precedes the
  transfer. Pinned by test, no new code path.
- `map_install_owners`: `TerminalRelationV1::Value` is admitted bounded
  to map-source returns — `MapLiteral` requires a `Complete` flow row
  whose `ReturnBoundary` statement is the relation's own return site;
  `MapLocal` requires a `Complete` flow row whose `local_binding` is the
  returned binding. Home/Handle/String/Null/Float `Value` returns stay
  rejected (each is its own named slice).
- `Call{result:Map}`: disposition rows carry `InvokeCallResultKind`
  decided at seal/issue time from the callee's source evidence —
  `Value(MapLiteral|MapLocal)` terminal relation ⇒ `Map`, otherwise
  `I64`. Emission sites use the row kind (no hardcoded `I64`); the
  result value types as `MirType::Box("MapBox")`. Both admitted lanes
  still gate callees on `result()==Some(I64)`/the trivial `:i64` index,
  so no row can carry `Map` today — the mechanism is the seam C5b's
  admission connect plugs into, and the physical layer already
  cross-checks kind↔role consistency (F4c).
- Signature result: `Map::New` results register
  `MirType::Box("MapBox")` in `value_types`, so
  `infer_return_type_from_phi` yields `Box` → `OrdinaryMap` role —
  `Unknown`-typed returns remain accepted as a backstop.
- `mark_checked` admits `ExpressionCompleted → Checked` only for
  return-bound rows (no install step exists); local-bound rows still
  require `install()` first. `validate_map_emission` matches
  `flow.local_binding()` to the optional binding and requires an
  installed `local()` only when the flow binds a local.

Non-claims: no map argument lane yet (census: 0 arg sites — defined by the
C5a edge vocabulary, admitted in C5b when an obligation demands it); no
String/BorrowedHandle/MapLocal/NestedArray entry payload kinds (each is a
named entry-class slice); no `root_map` role (opaque storage cannot cross
the process boundary); `Handle`/`Home`/`String`/`Null`/`Float` returns keep
their own lanes.

## Arm pinning (2026-09-15, verified against 7618c185)

`app_main_identity` IS `Some` on the merged route (`static box Main` at
merged L19038 produces the app relation). The actual failing arm is the
**first loop** of `preflight_map_install`: every `MapLiteral` body-shape
site needs `map_flow(&owned)` -> a `Complete` row in that owner's
`root_flow().maps()`. But `observe_map` is reached only through the
Local-initializer walk (`home_new_prefix.rs:678-709`); a `%{...}` in
`return`-position is never observed, so `map_flow` fails
`map-source-unavailable` -> `MapLifecycleConsumerMissing`.

Root cause chain (each layer independently bounded):

1. **Facts**: `flow.maps()` rows are issued only for `local x = %{...}`
   initializer sites — `return %{...}` / arg-position map literals have
   no row at all (`map_source_outward` pins `[Body, Initializer]`).
2. **Facts** — landed (F3): `TerminalRelationV1::Value` now covers
   `return <local-or-map>` and non-i64 literals; `return void` issues the
   Unit relation. Owners whose `return <value>` stayed uncovered still get
   `ReturnValueNotCovered` with no relation (consumed/uninitialized
   bindings, arbitrary expressions, TypedInteger literals). Issuance is
   still gated on the New-home prefix walk running for the owner —
   children without `has_map`/`child_new_ready` get no scan; extending
   that admission gate is C5b.
3. **Contract**: `map_install_owners` requires map owners be inside the
   AppMain direct-call loan targets (<=5); merged map owners are deep
   ordinary functions.
4. **Physical**: Map return/argument ABI does not exist (checked Map is
   alloca-local; v4 emit supports create/limited entry write/end only).

## Census (merged entry)

`%{...}` MapLiteral: 31 live sites, dominant form `return %{...}`
(MirJsonEmitBox `make_*`/`to_json` family). `new MapBox()`: exact 39-site
classification (2026-09-15, verified line-by-line):

- **27 returned** — the map is the function's own return value:
  MirSchemaBox `m` ×20 (`i`, `inst_const*`, `inst_static_data_load`,
  `inst_ret`, `inst_compare`, `inst_binop`, `inst_copy`, `inst_branch`,
  `inst_jump`, `phi_incoming`, `inst_phi`, `inst_mir_call_*`, `block`,
  `fn_main`, `module`); plus FuncScannerHelpersBox `_read_qualified_ident`
  /`_extract_ident` `result`, BoxTypeInspectorBox `_describe` `info`,
  BoxHelpers `map_put_or_new` `obj`, LowerLoopMultiCarrierBox
  `_extract_limit_info` `info`, DefsScannerBox `extract_name` `result`,
  MirRootHydratorBox `_parse_object` `obj`.
- **11 nested-stored** — stored into a container that escapes through
  the return path: MirSchemaBox inner `v`/`callee`/`payload` ×10 (set into
  the returned `m`), FuncScannerBox `def` pushed into the returned
  `methods` array.
- **1 truly local** — JsonFragNormalizerBox `_normalize_instructions_array`
  `seen` signature-dedup map (never returned, stored, or passed on).
- **0 arg-transferred at creation site** — every site is
  `local x = new MapBox()` or a parameter rebind; arg-position maps in the
  merged route arrive via `%{...}` literals or parameters, not `new
  MapBox()` inline.

Boundary: merged entry program -> `%{` literals and `new MapBox()` in
function bodies; excludes string literals/comments. Lifecycle impact:
38/39 created maps escape through a return boundary (directly or via a
stored container) — the C5a per-owner contract must cover
return-position map egress, and F4's return ABI is the dominant physical
requirement.

## Decomposed slice order

- **F1** `MIR-CALL-MAP-RETURN-SITE-FLOW-I0`: observe return-position
  MapLiteral as a map-flow row with a ReturnBoundary destination (Facts).
- **C5** (this card's contract core): per-owner lifecycle undertaking —
  generalize `map_install_owners`/preflight from AppMain-loan membership
  to per-`FunctionOwnerIdV1` verification.
- **F3**: `TerminalRelationV1` extension for non-i64 value returns.
- **F4**: Map return/argument physical ABI (family-scale).
- **C8**: two-function non-AppMain consumer acceptance.

The merged route cannot pass `MapLifecycleConsumerMissing` until F1+F3+F4
land — the map-owning cohort pervasively returns maps. This card's
deliverable is the verified decomposition; C5 remains the contract work.

## Arm re-pinning (2026-09-15, after F1 + entry-source family landed)

With return-boundary observation (F1), entry-value sources
(String/BorrowedHandle), nested-map EntrySlot, array NestedArray, and
map-local MapLocal all landed, the first preflight loop1 failure moved
to a **call-argument position**:

```text
[tmp/preflight-loop1] site=[Body(1), Value, Argument(0)]
    err=[freeze:contract] map-source-unavailable
-> [callable-semantic-package/install] MapLifecycleConsumerMissing
```

`MirJsonEmitBox.to_json(%{"functions" => [main]})`-shaped literals have
sealed `MapLiteral` rows but `scan_new_home_flow` never walks
`Argument(ordinal)` children of call expressions — so no flow row is
issued. Entry-class coverage is now complete (zero
`MapCandidateNotCovered` on the merged route); the remaining Facts gap
is **walk coverage for argument-position maps**, ordered as
`MIR-CALL-MAP-CALL-ARG-FLOW-I0` before the C5 contract arm can be
reached on merged.
