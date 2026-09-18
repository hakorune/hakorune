Task: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0
Parent: mir-call-map-local-entry-source-i0-2026-09-15.md
NextCard: T1 readable Map argument (borrowed read-only no-escape
handoff) — first vertical slice of the accepted 2026-09-18
map-argument Decision; T1..T5 queued in the T-series work-package
table (consultation #4). C6-4 rootless cohort stays deferred;
C1-id remains pre-production homework running in parallel.
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

**C8 design decision (main-lead trace, worker audit pending integration)**:

```text
Decision: generalize the existing per-owner affine direct-call
disposition loan from "AppMain owner only" to "every selected owner
with sealed direct-call observations". The map-result receive lane
(LocalCallObservationV1{Map} -> MapLocalProgress ->
Invoke{Call{result:Map}} -> StoredLocal::Map -> terminal Map::End) is
already owner-generic below the loan: completion_index, map_call_source,
begin/record/validate_map_call_emission, prepare/validate_root_home_exit,
describe_map_lifecycle_obligations, and the Borrowed fault frame all key
on FunctionOwnerIdV1. Only loan issuance, the co-seal scan predicates,
local_call_for_owner, and the CatalogedTargeted dispatch are AppMain-bound.
Source authority + canonical issuer: resolver direct_call_observations +
direct_call_target + callable index header + selected published key,
co-sealed per owner by the existing issue/co_seal_lifecycle path. The
callee's sealed terminal relation remains the sole result-kind authority.
Non-authority: the preflight-resolved `callee` in CatalogedTargeted (never
emission authority for a sealed lifecycle site); the header annotation;
tests.
Fail-fast boundary: an issued row must be taken exactly once at lowering
(finish_empty per owner); a map-result site without its sealed
observation/completion/terminal relation rejects at seal or dispatch.
Smallest next slice: (1) issue per-owner loans in issuer.rs +
co_seal_lifecycle for every loan; (2) run the home-flow scan when the
owner's loan holds an unannotated-target site, with the local_map_call /
is_i64_call predicates resolved through that owner's loan;
(3) local_call_for_owner searches completion_index;
(4) retain_child_terminal_relation also retains when the owner's flow
carries a Map local call; (5) the port holds owner-keyed loans and
CatalogedTargeted takes the row at a sealed site, reusing emit_local /
scalar materialize. Root caller keeps terminal `return use_map(7)` as a
scalar row.
Non-claims: no rootless cohort (C6-4 parked), no `return m` of a received
map, no instance-method calls in non-AppMain owners, no scalar cataloged
emission change for sites without a sealed row.
```

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
| C6-3 | C v2 nested-call lane — landed: flow bound `fi \|\|` → `(fi && !ordinary)` admits ordinary callers at fi>0 while Birth callers keep rejecting; per-block `%call_out<b>` i64 slot for nested ordinary calls inside ordinary functions (the caller's own `%out_i64` stays reserved for its return handoff — the exact gap the review noted for scalar calls inside map-returning callees); `invoke_normal_result` loads `%call_out<invoke_block>` at fi>0, `%ordinary_out` at root. Map nested calls reuse the existing per-block `%map<b>` lane. Evidence: `published_lifecycle_v4_nested_call_test.c` — positive root→helper→inner chain compiles; negatives reject malformed invoke_block + target-role drift (`function-body`) and a structurally valid Birth caller (`unsupported-cohort`, the exact `fi && !ordinary` arm). Execution proof: the compiled object linked against `libnyash_lifecycle_kernel.a` + runtime probe returns exit 7 (`Result: 7`, `COUNTS 1 0 0 0 0 1`) — inner's const reaches the root through `%call_out0`→`%ordinary_out`. Regression: receiver-identity + parser preartifact C tests and the full `published_lifecycle_v4_execution_test.py` all green; `cc -fsyntax-only -Wall` shows only baseline v2.inc warnings | C6-2 |
| C5c-1 | owner exit/cleanup co-seal — landed: `describe_map_lifecycle_obligations` now co-seals per-owner terminal evidence independent of any AppMain loan — `OwnerTerminalHomesUnavailable` (`terminal_homes()` must be `Ok`), `OwnerTerminalRelationMissing`, and `OwnerTerminalMapUnmatched` (a sealed `Value` terminal naming a map return must match a described site: `MapLiteral` requires its `ReturnBoundary` row at the same return-site node, `MapLocal` adds `ReturnHandoff` to the map's `LocalBinding` site — `local m = %{}; return m` no longer skips the handoff). Consequences (intended): implicit exits reject (`local m = %{}` — `ReturnValueNotCovered`/`TerminalNotCovered` means no sealed exit evidence exists to co-seal; `return`/`return 30` stay admitted); call-argument map owners always stop at describe (a `%{...}` argument poisons the i64-call prefix → `ArgumentHandoff` stays unreachable vocabulary). Pins: `map_transfer_invalidates_old_local_and_alias_field_observation` extended to `prepare_install` rejection + vacant catalog; `implicit_exit_map_owner_rejects_without_terminal_evidence`; `call_argument_map_owner_stops_at_describe_without_exit_evidence`; `returned_map_local_describes_return_handoff_on_the_local_site`. Focused: 231/231 `normal_callable_semantic_package`; `map_` batch shows 4 parent-reproduced baselines (`global_call_route_plan`×2, `mir_corebox_router`×2) + `map_write_timing_tests::boxcall_delegation` order-dependent flake (green standalone + 5-test batch on this diff) | C5b |
| C5c-2 | EntryStore obligation precision — landed: `MapHomeEntry::store_class()` is the sealed row's own install predicate (`Scalar`=`InstallValue`, `Transferred`=`InstallIndexed`, `Borrowed`, `Opaque`), shared verbatim by describe and the selected emit lane — one classification, verified once. `EntryStore` now carries the class: capability declares `Scalar`+`Transferred` only, so `Opaque` stores (String/`[...]`/`%{...}` child values — the review's `%{"op" => "const"}` owner) describe the obligation but fail at verify, before catalog mutation (`map-value-consumer-missing` is never reached downstream). Borrowed entries (`MapLocal`/`BorrowedHandle`/kind-less `Local`) no longer describe a redundant `EntryStore` — `OwnershipShare` is their store obligation. `emit_flow` consumes `store_class()` instead of its own inline reclassification. Pins: `opaque_entry_classes_reject_at_preflight_before_catalog` (String/array/nested-map entries → `prepare_install` reject + vacant catalog). Focused: 232/232 `normal_callable_semantic_package`; `map_` batch shows 4 parent-reproduced baselines + `map_write_timing` order-dependent flake (serial/standalone green) | C5c-1 |
| C5c-3 | `OwnershipShare(Handle)` admission — landed: `MapEntryBorrowKindV1{Handle,MapLocal,Local}` makes `OwnershipShare` kind-specific (home_map_flow `borrowed_root()` carries kind+root; `MapLocal`/`Local` borrows keep their own kinds). `BorrowedHandle` leaves are self-rooted parameter handles, and ordinary formals all arrive LV4_I64 — so `OwnershipShare(Handle)` is declared by `map_lifecycle_capability()` and the selected emit lane lowers it through the existing scalar `InstallValue`/`I64` lane (`end()` stays a non-owning no-op; the exact root binding is still enforced via `take_exact_lexical_value`, and `validate_map_emission`/`begin_map_emission` accept `InstallValue` only for the sealed `Handle` borrow kind). Verify adds `BorrowedEntryEscape`: a site mixing borrowed entries with an outward handoff (Slot/Return/Argument/Contained) rejects even when every named operation is covered — the borrow cannot ride a boundary with no liveness contract. Test hook: `lower_map_dependency_for_test` now derives `parameter_count` from the resolved header and materializes physical param ValueIds before entry adoption. Pins: `self_rooted_handle_entry_describes_handle_ownership_share`, `declared_capability_seals_an_in_owner_handle_borrow`, `verify_rejects_a_borrowed_entry_that_escapes_through_a_handoff`, `verify_rejects_map_local_and_kindless_local_borrow_kinds`, `borrowed_formals_describe_a_handle_share_the_declared_lane_admits`, `self_rooted_handle_borrow_emits_install_value_i64` (param ValueId → `InstallValue{I64}`); `declared_root_unissued_map_sites_stop_before_install` updated — a `%{"v" => <untyped formal>}` map now admits. Focused: 252/252 `normal_callable_semantic_package`; `map_` batch shows the same 4 parent-reproduced baselines (`global_call_route_plan`×2, `mir_corebox_router`×2) + `map_write_timing_tests::boxcall_delegation` order-dependent flake (standalone + parent-parallel green). Non-claims: `OwnershipShare(MapLocal)`/`OwnershipShare(Local)`, `EntryStore(Opaque)`, `ArgumentHandoff`, `SlotHandoff`/`ContainedHandoff` stay undeclared — `_module_with_blocks` still stops at `UncoveredOperation`, now first on `EntryStore(Opaque)` at `local main` | C5c-2 |
| C9  | `EntryStore` payload representation — C9-1 landed (Text): the worker-audited Decision splits `Opaque` into payload-kind classes. `MapEntryStoreClassV1::Text` is the sealed `MapValueSource::String` row — `ResolvedLiteralSourceV1::String(Box<str>)` now carries the literal bytes at Facts issuance (same source-owned payload rule as `entry.key()`), so no consumer re-reads AST. `InstallText{map, key, utf8}` is a dedicated MIR operation (no ValueId operand; bytes ride inline like `PrepareKey`), JSON `map_install_text`, C emit embeds `private constant` bytes and calls `nyash.map.checked_install_text_v1`, which validates UTF-8 + pointer separation before key consumption and installs `CheckedMapPayload::Text(Box<str>)` — owned bytes, never an interned/`string_literal_handle` cache entry (an interned handle would leave a stale cache entry on map end). Indexed flow/admission/v2 validators treat it exactly like `InstallValue`: consume key, publish outcome, end displaced entries. `validate_map_emission` pins emitted bytes to the sealed row (`map-literal-value-drift`), wrong-family emits hit `map-install-source-drift`, and `map_install_owners`-equivalent preflight admits `Text` while `[...]`/`%{...}` stay `Opaque`. `runtime_map_symbols` inventory gains `checked_install_text_v1` plus the previously-missing `storage_move_v1`. Pins: `text_entry_describes_text_store_class`, `declared_capability_seals_a_text_entry`, `verify_still_rejects_opaque_entry_classes`, `merged_route_shape_seals_text_and_empty_array_entries` (was `..._stops_at_array_entries_not_text` — renamed in C9-2 when `[]` became a covered lane), `string_literal_entry_emits_install_text` (end-to-end `InstallText` + strict verify), `text_install_consumes_the_key_and_carries_no_value_operand`, `map_install_text_publishes_inline_utf8_without_a_value_operand`, kernel `text_abi_owns_validated_bytes_and_rejects_invalid_utf8_before_key_consumption`, host `text_payload_owns_bytes_through_rejection_detachment_and_end`; `opaque_entry_classes_reject_at_preflight_before_catalog` drops the now-covered String case. Focused: 256/256 package + 22/22 invoke::map_tests + 11/11 kernel checked_map + gcc -fsyntax-only on `hako_llvmc_ffi.c`. Non-claims (named, not implemented): `EntryStore(EmptyArray)` — `"params" => []`/`"locals" => []` remain the live residual and need a fresh-array residence or owned-empty decision (`checked_new_v1` returns a positive host handle the negative-handle indexed lane cannot carry); non-empty `[...]` elements; MapChild (`Box<CheckedMap>` residence + `checked_install_map_child_v1`, storage-move pattern — named only); `OwnershipShare(MapLocal)`; `ArgumentHandoff`; map read/projection | C5c-3 |
| C9-2 | `EntryStore(EmptyArray)` — landed: the worker-audited Decision chose the owned-empty marker over fresh-array residence (the checked lane is write-only — `observe_native` returns `ProjectionUnavailable` on present entries — so a marker is the exact ownership representation; a `checked_new_v1` positive host handle cannot ride `InstallIndexed`'s canonical indexed identity, and a new residence would only restate the weaker `drop_handle` end contract). `store_class()` maps `NestedArray{elements:[]}` → `EmptyArray` (non-empty stays `Opaque`); `InstallEmptyArray{map,key}` carries no value operand; JSON `map_install_empty_array`; physical `MapInstall` ABI; C v4 validator/indexed-flow/emit treat it exactly like `InstallText` (consume key, publish outcome); `nyash.map.checked_install_empty_array_v1` shares `install_candidate` and installs `CheckedMapPayload::EmptyArray` (trivial end, no host handle). Pins: `merged_route_shape_seals_text_and_empty_array_entries`, `entry_store_empty_array_still_requires_a_declared_operation`, `empty_array_entry_emits_install_empty_array` (end-to-end + strict verify), `empty_array_install_consumes_the_key_and_carries_no_operands`, `map_install_empty_array_publishes_no_payload_operand`, kernel `empty_array_abi_installs_an_owned_marker_through_the_shared_protocol`, host `empty_array_payload_is_an_owned_marker_with_trivial_end`, Facts `array_entry_*` `store_class` asserts (`[]` → `EmptyArray`, `[1,"s",args]` → `Opaque`). Focused: 259/259 package + 12/12 kernel checked_map + 53/53 verifier/JSON/ABI batch; `map_` batch reds are the recorded baseline set (4 parent-reproduced) + `map_write_timing` order-dependent flake (standalone green). Non-claims: non-empty `[...]` needs a real residence decision (elements carry borrows a marker cannot drop); `OwnershipShare(MapLocal)`, `ArgumentHandoff`, `to_json` body admission, map read/projection stay undeclared | C9 |
| F5  | map-result call lane — the `result()==Some(I64)`/`:i64` callee gates deferred from C5b: admit sealed map-terminal callees, issue `Call{result:Map}` rows, and let the receiving caller consume the `InvokeCallResultKind::Map` projection (F4 verifier + `ordinary_map` wire already landed). F5-1 landed: `signature().result() -> Option<I64>` (unannotated = map-result candidate, `:i64` path unchanged incl. `ZeroParameters`); `LocalCallObservationV1` + `LocalCallResultClassV1` issued at scan (map arm installs `StoredLocal::Map` + `homes.push`); co_seal splits by site (terminal needs caller `Call` relation; local needs the row — I64-class locals still require the caller's terminal-call undertaking, Map-class owns its binding) + result matrix `(None, Value(MapLiteral|MapLocal)) -> Map`; unannotated non-map targets reject (`LifecycleSourceMismatch`); cataloged walk requires `map_result_callee` proof for `result()==None` targets (`UnissuedDirectCallObservation`); receive-only owner gets `{NormalCleanup, FaultCleanup}` describe + `map_install_owners`/`requires_map_lifecycle_consumer` enumeration. Observable: `main() { local m = make_map(); return 0 }` issues `Call{result:Map}` + described obligation; `caller(seed:i64):i64 { local m = make_map(); return seed }` admits cataloged. Focused: 235/235 package (3 new pins + deferred-test boundary updates to earlier Resolver/loan rejects; 2 resolved_semantics failures are parent-reproduced baseline). F5-2 landed — physical `MapLocalProgress` + `result:"map"` emission + C v2 execution evidence; see the F5-2 row | C5c, F4 |
| C1-id | opaque source identity — pre-production homework on a test-only issuer: `verify_source_input_identity` is content comparison; content-identical foreign declarations stay indistinguishable. Bind to the batch-issued opaque source identity before any production connection | C1 |
| C6-4 | rootless cohort — deferred: upstream `uncovered-lifecycle-function` coverage, `FinalizedRootHandoffV1` library variant, doc marker (merged-route shape, not needed by C8's rooted acceptance) | C6-1 |
| C8  | two-function non-AppMain consumer acceptance — landed: per-owner `DirectCallDispositionLoansV1` replace the single AppMain loan; `issue_direct_call_loans_v1` covers app_main + selected `is_main_static_child` owners carrying resolver observations; co-seal/`local_call_for_owner`/`call_source_completion_for_owner` are owner-generic; `map_install_owners` deleted — `describe_map_lifecycle_obligations` owns the common checks plus `prior_homes` rejection at preflight; frame-drift validation accepts `Borrowed` frames (mode re-validated by `check_binding`); unconditional `borrow_fault_frame` fixed (`artifact-unowned-lifecycle-site`). Production-route evidence in `map_consumer_tests` (source-issued artifact): Normal receive + one clean release; callee-Fault invoke landing → `ReturnFault` with no result; caller-Fault — construction-fault paths and the exit pending chain release the live received lease into `ReturnFault`; release drift rejects at artifact validation. Focused: 3/3 `map_consumer_tests` + 6/6 `direct_call_owner_loan_tests` + 243/243 package; five `normal_default_root_catalog_lifecycle_tests` reds reproduce identically at parent → baseline debt. Non-claims: qualified `Main.make_map()`, terminal `return <call>` in child owners, receive with `prior_homes`, `return m`, map arguments, non-loan-role owners, rootless cohort | C5c, F5, C6 |

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

## F5 Decision (2026-09-16, worker-audited admission seams)

**Callee result class = the callee's sealed terminal relation** — the
result contract row's `terminal_relation` classified by
`call_result_kind` is the sole result-kind authority. The index
signature records only the syntax fact (`:i64` annotation → `Some(I64)`;
unannotated → `None`) and never decides result class downstream.

Audit outcome (read-only worker + primary trace): the F5 gap is not a
missing physical primitive — `emit_local`,
`InvokeCallResultKind::Map`, C v2 `ordinary_map` + `result:"map"` +
`storage_move`, and the `Map::End` cleanup-graph edge all landed in
F4/C6-3 — but a scalar-only admission chain:

1. `ExactTrivialCallableSignatureV1.result` is always `I64`;
   `validate_exact_i64_header` rejects unannotated returns
   (`ReturnTypeOutsideProfile`) and zero-param headers
   (`ZeroParameters`), so `make_map() { return %{...} }` is never an
   index row — calls resolve to `TargetMissing`.
2. `RootHomeFlow.local_calls` is issued only when the `is_i64_call`
   predicate fires; a map call falls to `PrefixNotCovered`.
3. `co_seal_lifecycle`'s map-owned arm requires the caller's own
   terminal `Call` relation (`call_source_completion_for_owner`), the
   callee `result()==Some(I64)`, and an `IntegerLiteral` callee
   terminal — `local m = make_map(); return 0` and any map-result callee
   both reject. `call_result_kind` is unreachable today (the
   `IntegerLiteral` force makes it constant-`I64`).
4. The received map's caller obligations are undescribed —
   `describe_map_lifecycle_obligations` enumerates `MapLiteral` sites
   only; a receive-only owner is not `map_owned` and has no
   `flow.maps()` row.
5. The receiving caller is the AppMain root — the disposition loan is
   root-scoped and non-root call production is the C6-4 gap, so F5's
   shape is `local m = make_map()` inside `main`.
6. Physical: `local_commits` has no call-site `Map` row, so
   `prepare_root_home_exit` cannot issue the received map's `Map::End`
   (`root-home-not-installed`) — the only missing physical piece; the
   `installed_home(binding)` exit lookup, `MapLocalProgress` row shape,
   and `terminal_homes` accounting (`install_map` + `homes.push`)
   already generalize.
7. `return m` of a *received* map is unreachable in this lane (root is
   i64-boundary; non-root call production is C6-4) — deferred, not
   fabricated.

```text
Source authority + canonical issuer: callee sealed terminal relation
  (completion + result contract row) -> `call_result_kind`; the index
  header seal records syntax facts only.
Non-authority: `signature().result()` never classifies result kind;
  `map_owned` (body MapLiteral) is membership evidence, not result
  class; declared annotations never decide the call result.
Fail-fast boundary: (contract result, terminal) drift rejects; caller
  shapes outside `local m = f()` receive-hold-release / `return f()`
  i64 terminal reject; nested non-root call production stays
  unadmitted; the physical lane stops at a named gate until F5-2.
Smallest next slice: F5-1 below.
Non-claims: map arguments; terminal `return f()` map pass-through;
  received-map `return m` handoff; `MapLocal`-terminal callees without
  an own `%{...}` literal; formal `MapCallEdgeContractV1` matching
  rows; rootless cohort (C6-4).
```

Decomposed:

| # | slice |
|---|-------|
| F5-1 | admission + Facts + Verify — signature `result -> Option<ExactTrivialScalarAbiV1>`; the unannotated static profile is admitted as a map-result candidate (zero-param allowed on the unannotated path only — the `:i64` path keeps `ZeroParameters`); `exact_formals` drops the signature-result arm (param evidence unchanged); `LocalI64CallObservationV1` gains a `LocalCallResultClassV1` marker issued at scan (predicate result, verified against the contract at co-seal); the local scan installs `StoredLocal::Map` + `homes.push` for map-class calls; `co_seal` splits the map-owned arm by site — a terminal site still needs the caller `Call` relation, a local site needs the local-call row — and the callee matrix `(Some(I64), IntegerLiteral) -> I64`, `(None, Value(MapLiteral\|MapLocal)) -> Map`, all drift rejects; `describe` + `map_install_owners`/`requires_map_lifecycle_consumer` count map-class local calls so a receive-only owner's `{NormalCleanup, FaultCleanup}` obligation is described and its exit evidence is co-sealed. Observable: `main() { local m = make_map(); return 0 }` installs with `Call{result:Map}` + a described receive obligation; the physical lane stops at `root-home-not-installed` (named) until F5-2 |
| F5-2 | physical + C v2 — landed: `MapLocalProgress` commit row (`Emitting -> Emitted{ExpressionCompleted -> Installed -> Checked}`) keyed by exact owner+site+binding; `emit_local` dispatches map rows and carries the root-owned `FaultFrameEnter` in the row (a `Plain` terminal exit has no `Call` entry); `installed_home -> end_operation = Map::End` releases the received lease; `validate_map_call_emission` enforces the sealed call source, row/binding identity, exactly one `Invoke{Call{result:Map}}` + matching `InvokeNormalResult` + root-owned frame definition, and every recorded binding at its exact physical site. Physical program publishes the `ordinary_map` callee + `result:"map"` edge on the real target ordinal. C v2 consumes unchanged: `published_map_physical_execution_test.py` compiles a `local m = make_map()` receive program through `published_lifecycle_v4_driver.c`, links it against the kernel archive + `HAKO_MAP_CALL_PROBE` (`--wrap=nyash.map.storage_move_v1`), and observes exit 30 with `30 1 2 1 1 1 1 1` — one `storage_init`, two `storage_dispose` (callee moved-from + caller `Map::End`), one key/outcome pair, one `storage_move`; the source-issued `/tmp/hako-issued-physical-v2-map-call.json` runs identically through the unchanged consumer (optional argv). Negative matrix: result-kind drift both directions (`result:"i64"`->`ordinary_map` and `result:"map"`->`ordinary_i64`) rejects `function-body`; Rust pins cover contract missing / identity+site drift / cleanup evidence missing. Focused: `physical_program_json_tests` 15/15 incl. `map_result_local_call_publishes_ordinary_map_callee_and_map_edge`; full py suite green | C5c, F4 |

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

## C8 Decision (2026-09-16, review-driven: per-owner loans + prior-Homes admission)

Review of `0bc84965` accepted the F5-2 authority chain and named two
remaining design issues plus the C8 acceptance gap. Traced at HEAD:

```text
Decision: generalize the AppMain direct-call loan to per-owner loans
  keyed by FunctionOwnerIdV1, issued for every declaration that is
  (app_main identity) OR (selected role `is_main_static_child`) and
  carries resolver `direct_call_observations`. Same-box bare calls
  (`static box Main { main / use_map / make_map }`) are the admitted
  fixture — the sealed row remains the site's sole authority so the
  FunctionCall arm's existing loan intercept consumes it before
  `bare-static-method-retired` is ever consulted.
Source authority + canonical issuer: resolver `direct_call_observations`
  + `direct_call_target` (already owner-generic — verified by probe),
  joined with callable_index header + selected published key by
  `issue_direct_call_loans_v1`; result class via existing
  `co_seal_lifecycle` + owner-generalized `local_call_for_owner`.
Non-authority: name/arity lookup, physical layout, AST re-observation,
  `is_app_main` flags on the row path (kept only for the instance-call
  arm, which is genuinely AppMain-scoped).
Fail-fast boundary: loans only for owners that lower through the raw
  child port (`is_main_static_child` + app_main); `finish_empty` per
  owner; `local-call prior_homes` rejects at install preflight
  (describe arm) until Fault cleanup for prior resources exists;
  common owner checks live in `describe_map_lifecycle_obligations`
  (completion/homes/flow/terminal/map-return matching — the old
  terminal-kind restriction and app_main requirement are removed);
  loan-specific checks (`has_taken_slot`, map-target ⊆ obligations,
  no self-target) iterate actual loan rows only.
Smallest next slice: (1) BoxShape rename of the AppMain* direct-call
  family to owner-generic names; (2) per-owner issuance + coseal gates
  + port plumbing + preflight restructure; (3) C8 runner with mandatory
  source-derived artifact and Normal/callee-Fault/caller-Fault probes.
Non-claims: qualified `Main.make_map()` MethodCall lane (import
  inventory plumbing — separate slice); terminal `return <call>` in
  non-AppMain owners (`call_source_completion_for_owner` stays
  root-scoped); prior-Homes Fault cleanup; map arguments; received-map
  `return m`; `Ordinary`-role/dynamic/S6C loan owners.
```

Verified owner-genericity already present (no change needed):
`describe_map_lifecycle_obligations` enumerates Map-class `local_calls`
per owner; `MapLocalProgress`/`root_home_exit`/`validate_finalized_child_emissions`
are keyed by `OwnedExprSiteV1`/owner; `emit_local` takes `(owner, site,
row)`; `borrow_fault_frame` is per-function; `record_root_local_call_bindings`
is owner-keyed. `retain_child_terminal_relation` must additionally retain
for loan owners (`has_map || loan_for(owner)`), since a receive-only owner
has no literal (`has_map=false`) but describe requires its terminal relation.
The `prior_homes` Facts (`LocalCallObservationV1::prior_homes()`) are
already sealed — the rejection moves to `describe_map_lifecycle_obligations`;
the lowering guards (`local-call-prior-homes-unsupported`,
`map-call-prior-homes-unsupported`) remain as unreachable defense.

## C8 acceptance evidence (2026-09-16, production route)

Landed: `DirectCallDispositionLoansV1` (owner-keyed collection,
`finish_empty` per owner) replaces the single AppMain loan;
`issue_direct_call_loans_v1` issues rows for app_main + selected
`is_main_static_child` owners carrying resolver observations;
`co_seal_lifecycle` runs per issued loan; `local_call_for_owner` /
`call_source_completion_for_owner` search the completion index for any
owner; `map_install_owners` is deleted — owner-common checks live in
`describe_map_lifecycle_obligations` (now also rejecting non-empty
`prior_homes` at install preflight), loan-specific checks iterate actual
loan rows. Ports carry owner-keyed loans through the raw child scope.

Two physical-route fixes the C8 runner exposed:

- `map-call-frame-drift` (`ordinary_new_local_commit/map.rs`) pinned
  `FaultFrameMode::RootOwned`; cataloged child owners legitimately enter
  `Borrowed` frames. The site check now requires exactly one recorded
  `FaultFrameEnter` with the recorded `dst` regardless of mode — mode is
  re-validated by `check_binding` against the finished function.
- `borrow_fault_frame` materialized a `FaultFrameEnter` on every plain
  exit, leaving an unowned lifecycle site (`artifact-unowned-lifecycle-site`)
  in owners with no homes/call. It now runs only inside the branch that
  emits Invoke/fault paths.

Production-route evidence (source-issued artifacts via
`complete_normal_default_program_root_catalog_lifecycle` +
`into_artifact_parts`, in
`mir::builder::normal_default_root_catalog_lifecycle::map_consumer_tests`):

- `source_backed_map_consumer_child_receives_and_releases_its_lease` —
  fixture `main { local r = use_map(10) return 30 }`,
  `use_map(seed): i64 { local m = make_map() return 42 }`,
  `make_map() { return %{..} }`. Observed in MIR: exactly one
  `Invoke{Call{result:Map}}` in `use_map` with `InvokeNormalResult`
  binding the received lease; the invoke's fault landing ends in
  `ReturnFault` (callee-Fault: no result installed, borrowed frame
  returned); exactly one `Map::End` of the received lease on the clean
  chain; `main`'s scalar call stays on the sealed row; artifact
  validation passes.
- `source_backed_map_consumer_fault_suffix_still_releases_pending_homes` —
  fixture adds `local n = %{"b" => 2}` after the received lease (main is
  terminal `return use_map(10)` because `use_map` becomes map-owned).
  Caller-Fault is observed directly: the literal's fallible construction
  fault paths (`Map::New/PrepareKey/InstallValue/EndOutcome`) drain the
  still-live received lease into `ReturnFault`, and the exit pending
  chain releases it again — every `End(received)` either reaches
  `return 42` exactly once (clean chain) or drains to `ReturnFault`.
  Release is LIFO (n then m); the pending chain carries the received
  lease behind the outermost clean release.
- `source_backed_map_consumer_release_drift_rejects_at_artifact_validation` —
  mutating one `End`'s map handle rejects at artifact validation,
  before catalog mutation effects.

Focused gates: 6/6 `direct_call_owner_loan_tests` (non-AppMain positive +
negatives: unsealed owner, scalar-only row on map site, prior_homes,
self-target, non-loaned map owner bystander), 243/243
`normal_callable_semantic_package`, 3/3 map_consumer_tests. The five
remaining `normal_default_root_catalog_lifecycle_tests` failures
(`actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`,
`parser_scan_package_passes_callable_source_handoff_without_fallback`,
`source_backed_app_main_direct_call_consumes_affine_loan`,
`source_backed_package_failure_is_terminal_before_builder_effects`,
`source_bound_static_result_owner_reaches_the_raw_terminal`) reproduce
identically at HEAD without this change — known baseline debt, not
current-change failures.

Admitted entry classes for this acceptance: same-box bare calls in a
`static box Main` (`main` / `use_map` / `make_map`), scalar terminal or
local call in `main`, one map-result receive per owner with zero
`prior_homes`, integer-literal terminal.

Non-claims (unchanged): qualified `Main.make_map()` MethodCall lane;
terminal `return <call>` in non-AppMain owners; receive after an earlier
home (`prior_homes` rejects at preflight); received-map `return m`;
map arguments; `Ordinary`-role/dynamic/S6C loan owners; rootless
cohorts; runtime exit-code evidence.

Merged-route probe (2026-09-16, `./target/quick/hakorune --backend mir
/tmp/merged_entry.hako`, sha256 `23b6cf89...`, temporary instrumentation
removed): the outer label stays `MapLifecycleConsumerMissing` but the
boundary moved — with `map_install_owners` deleted the describe arm now
reaches per-owner exit evidence and stops at
`OwnerTerminalHomesUnavailable` for `MirSchemaBox._module_with_blocks`
(batch slot 681):

```hako
method _module_with_blocks(blocks) {
    local main = %{"name"=> "main", ... "blocks"=> blocks}
    return MirJsonEmitBox.to_json(%{"functions"=> [main]})
}
```

The owner's terminal is `return <qualified static method call>` —
`terminal_call` covers only loan `is_i64_call` rows and AppMain zero-arg
instance calls, and its argument collection requires all-integer args
(the `%{...}` arg fails that filter). `ReturnValueNotCovered` leaves
`terminal_homes` unavailable, so the C5c-1 co-seal rejects before
catalog mutation — fail-closed as designed. This is the named residual:
a terminal relation class for `return <call>` outside the admitted
i64-direct-call family (non-claimed above). The next bounded slice names
that classification decision; map-argument handoff
(`ArgumentHandoff`/`Opaque` entries in `to_json`'s map) stays behind it.

## Review fix note (2026-09-16, review of `65fcfe05cc`)

Two defects from the external review were fixed on this branch:

1. **Verifier/consumer divergence** (`dac1e73e5b`):
   `resolved_region_flow::AnalyzerV1` authorized returns through the
   singleton `completion.explicit_site()`, which is `None` for
   `ExplicitReturns` and `ExplicitUnitSetWithImplicitEnd`, so every
   sealed multi-site completion was rejected downstream. The analyzer
   now carries the whole `explicit_sites()` set — the same convention
   `IfControlAnalyzerV1` already used — and authorizes a `Return` by
   exact site membership (`return_not_in_sealed_completion_sites` for
   any unsealed site). **Correction (2026-09-17 review):** the earlier
   note claimed single-site `ExplicitUnitSetWithImplicitEnd` completions
   lower end-to-end through the canonical route — that is not true.
   `verify_body` (`capability.rs`, `ReturnPolicyV1::FinalOnly` at the
   root body / `Forbidden` inside `if`/`else`/`BlockExpr` bodies) runs
   before `verify_function_completion_v1` and rejects every nested
   `return` with `return_not_allowed_here`, so no sealed generalized
   completion reaches the canonical analyzer or draft seal in
   production; the set-based authorization is forward groundwork for
   the lane that lifts that gate, exercised today only by direct
   analyzer tests. Downstream lanes fail closed the same way:
   multi-site value sets at draft seal
   (`MultipleExplicitReturnClaimsUnsupported`) where the exact-two
   `PreparedFunctionExitSetV1` lane is the designed consumer — its
   generic-lane wiring is a named later slice
   (`multi_site_exit.rs` documents the deferred fresh-session
   consumer) — generic_g0 requires `returns_value` + exactly one site,
   and s6c ingress requires the exact-two loop-return+tail set.
   Pins: `sealed_if_else_value_return_set_is_authorized`,
   `sealed_unit_return_with_implicit_end_is_authorized`,
   `unsealed_return_set_never_reaches_flow_analysis` in
   `if_flow_tests.rs` (13/13 `resolved_region_flow`), plus the
   boundary pin
   `sealed_unit_set_with_implicit_end_stays_outside_the_canonical_route`
   in `capability_tests.rs` (sealed completion asserted,
   preflight + `compile_resolved` reject `return_not_allowed_here`).
2. **fmt drift** (`5035bc4b77`): 15 session-touched files reformatted;
   `cargo fmt --check` clean.

Baseline classification (verified at pre-session parent `12bc76ed4d`,
not current-change): 5 `normal_default_root_catalog_lifecycle_tests`
reds (already recorded at C8) plus 10 `resolved_lowering` reds —
`DynamicCarrierMismatch` ×5 (dynamic_loop_*), `ObjectDefinitionsNotConsumed`
×2 (s6c substring), `ReturnValueTypeMissing` ×2 (tests.rs),
`AlreadyIssued` ×1 (physical_entry_lane_adoption). The five broad
`resolved_` filter reds (loop phi materializer, owner-forest receiver,
shadow vocabulary, brand constructor) are likewise unrelated modules.

## Review fix note (2026-09-17, review of `ec71673454`)

The external review of `ec71673454` named two contract defects plus a
diagnostics-flattening cost. All three are closed on this branch:

1. **A map-receiving callee fell to the scalar `Call` route.**
   `co_seal_lifecycle` gated the non-map-owned local-call arm on the
   caller's `call_source_completion`, which a Plain terminal exit does
   not have, so `use_map` — a lifecycle-bearing callee returning `i64` —
   was classified Scalar and emitted `Call{Global}`. In the lifecycle
   artifact that edge confers no membership: the callee's Invoke-bearing
   body would reject `function-not-cataloged` /
   `instruction-unsupported` at `admit_lifecycle`. The classifier now
   reads the callee's own sealed products — a terminal Call relation, or
   a root flow carrying local calls or Map rows
   (`callee_lifecycle_participant` in `direct_call_lifecycle.rs`) — and
   keeps the call on `InvokeOperation::Call` with `I64` result kind.
   `RootHomeExitEntry::Plain` gained `local_bindings` so the Plain exit
   owns the source-ordered lifecycle binding groups directly (validated
   by the same `validate_local_call_binding_groups` the Call entry
   uses), and `emit_local` records the shared `fault_frame` definition
   in every lifecycle-emitting local-call group — duplicate coverage
   under Call exits deduplicates on `(block, instruction)`.
   `non_map_local_call_with_plain_return_preserves_scalar` still pins
   the ordinary-callee scalar path, so the lift is callee-evidence-only.

2. **`finish_empty()` accepted a fully untouched loan.** Ready rows are
   now explained rows: `DirectCallDispositionLoanV1` gained
   `canonical_route_bypassed`, set only through
   `mark_canonical_route_bypass(owner)` — called exactly at the selected
   CallableSingleLoop lowering in `lower_app_main_static_child`
   (`normal_callable_semantic_loan_port.rs`) for the exact
   `program.owner()`. Untouched without the mark, or partially consumed,
   rejects `ResidualRows`; the bypassed lane drains. No blanket bypass.

3. **Typed causes no longer flatten.** `install_map_preflight` maps
   `describe_map_lifecycle_obligations` failures to
   `InstallIssue::MapObligationDescribe(MapObligationDescribeIssueV1)`
   and `verify_map_lifecycle_undertaking` failures to
   `InstallIssue::MapLifecycleUndertaking(MapLifecycleUndertakingIssueV1)`,
   preserving `OwnerTerminalHomesUnavailable`,
   `CallPriorHomesUnsupported { owner, site }`, `UncoveredOperation`,
   `EmptyUndertaking`, and siblings through `package_issue`'s `{:?}`
   boundary instead of collapsing to `MapLifecycleConsumerMissing`.

Focused: `normal_callable_semantic_package` 244/244; the three
`normal_default_root_catalog_map_consumer_tests` pins green (the C8
`main` now asserts `Invoke{Call{I64}}` reaches `use_map`, not a scalar
`Call`). Baseline reds unchanged
(`source_backed_app_main_direct_call_consumes_affine_loan`,
`main_f1_rejects_direct_call_and_nested_owner_before_lowering` —
parent-reproduced; batch-only env/flaky reds classified separately).

### Runtime acceptance (the C8 execution evidence)

`published_map_consumer_tests.rs` (feature `plugins`, `#[ignore]`d on
toolchain) materializes the same C8 family

```hako
static box Main {
    main() { return use_map(10) }
    use_map(seed: i64): i64 { local m = make_map() local n = %{"b" => 2} return 42 }
    make_map() { return %{"a" => 1} }
}
```

through `compile_normal_with_published`, asserts the issued physical
JSON carries exactly one `result:"map"` edge and one `ordinary_map`
callee, compiles the view to an object, and links it against
`libnyash_lifecycle_kernel.a` + `published_map_fault_probe.c` with
`-DHAKO_MAP_SOURCE_PROBE -DHAKO_MAP_CONSUMER_PROBE`. The v4 map storage
is opaque to read ABI by design, so the returned map's contents are
evidenced transitively: the probe captures key bytes at
`key_prepare_utf8`, records each successful `checked_install_value`
against its map storage, follows `storage_move` from the ended caller
slot back to the callee slot that received the install, and prints one
`END key=value local|moved` line per `checked_end`.

Observed (probe stdout, deterministic):

```text
=== normal ===                     42 2 3 2 2 2 2 1
END b=2 local
END a=1 moved                      <- returned map carried a=1 into the
                                      caller's end: usable after callee
                                      cleanup, freed in caller
=== value-install-fault-1 ===      70 1 1 1 1 1 1 0   REPORT 101
END empty local                    <- callee fault: lease ended in
                                      make_map cleanup, no storage_move
=== value-install-fault-2 ===      70 2 3 2 2 2 2 1   REPORT 101
END empty local
END a=1 moved                      <- caller fault: received map freed
                                      in caller while carrying a=1
=== value-outcome-fault-1/-2 ===   70, REPORT 100; committed entries
END a=1 local / END b=2 local + END a=1 moved
=== end-fault ===                  70 2 3 2 2 2 2 1   REPORT 100
END b=2 local / END a=1 moved      <- release-under-fault still drains
```

Counts are observed releases, not inferred: map storage init/dispose,
key init/dispose, outcome init/dispose, and `storage_move` are pinned
per mode (`init==dispose` in every mode; `moves` is 0 iff the callee
faulted before the return handoff). `Ordinary`-role callers,
`prior_homes`, and map arguments remain non-claims; no production
switch or legacy retirement is claimed.

## Next-slice sharpening (2026-09-16)

The `return <qualified call>` terminal class carries a second
prerequisite beyond the relation itself: `issue_direct_call_loans_v1`'s
`collect_direct_call_rows_v1` requires the callee owner to be a batch
declaration (`PublishedTargetMissing`), i.e. `MirJsonEmitBox.to_json`
must be an emitted package member before a caller's terminal can invoke
it. `to_json`'s body (recursion, `.get`, `is_array`, expression-`if`
branches, early returns, string concat) is itself outside the admitted
family — so the classification Decision must also name the
callee-admission ordering (terminal relation for the caller may land
first only if the callee cohort is already cataloged, or the caller
slice must wait behind the callee's own admission chain).

## OpaqueCall Decision (2026-09-17, worker-audited, landed)

```text
Decision:
  `return <qualified call>` (a sealed `method_calls()` row whose
  receiver is `QualifiedUnbound`) is its own terminal class —
  `TerminalRelationV1::OpaqueCall(TerminalOpaqueCallReturnV1
  {owner, return_site, call_site})`. It is not the affine `Call`
  relation (no loan row, no i64 arguments, no Invoke lane) and not a
  `Value` source (its result class is unproven; `Value` classifies
  non-i64 returned sources only).
Source authority + canonical issuer:
  The resolver-sealed `VerifiedResolvedMethodCallSourceV1` row
  (receiver disposition, selector, arity, argument sites) is the only
  authority; `scan_new_home_flow` issues the relation after
  `terminal_call`, `return_scalar`, and `terminal_returned_source`
  all decline.
Non-authority:
  `direct_call_observations`/loan rows prove nothing for qualified
  calls; `OpaqueCall` never issues Invoke, never claims callee
  catalog membership, never claims a result class, and never hands
  map arguments across the boundary.
Fail-fast boundary:
  No `method_calls()` row or a `Lexical`/`CurrentOwner`/`Other`
  receiver keeps `ReturnValueNotCovered`. `call_result_kind` is
  Option-ized so an `OpaqueCall` callee cannot fabricate
  `InvokeCallResultKind::I64`; both query sites fail closed
  (`LifecycleSourceMismatch` inside `co_seal_lifecycle`,
  `root-instance-call-result-kind-unavailable` on the AppMain lane).
  `result_abi()` derives `None`; `finalized_root_handoff` checks
  owner drift only. Describe reaches the argument map rows and names
  `ArgumentHandoff`/`EntryStore(Opaque)`/`OwnershipShare`;
  `verify_map_lifecycle_undertaking` fails `UncoveredOperation` —
  the typed blocker replacing `OwnerTerminalHomesUnavailable`.
Smallest next slice:
  The map-argument handoff capability itself (`ArgumentHandoff`,
  `EntryStore(Opaque)`, `OwnershipShare` argument entries), ordered
  behind callee-body admission (`MirJsonEmitBox.to_json`: recursion,
  `.get`, `is_array`, expression-`if`, early returns, concat) only
  where a `Call`/Invoke relation actually requires the callee in the
  batch. `OpaqueCall` itself needs no callee membership — the caller
  relation joins the caller's own sealed row only.
Non-claims:
  No `ArgumentHandoff`/`ContainedHandoff` capability, no qualified
  callee resolution (import-inventory plumbing), no qualified-call
  Invoke or physical edge, no `me.`/`obj.` receiver calls, no
  `local x = <qualified call>` initializer lane, no map-result
  qualified calls, no `_module_with_blocks` admission — its install
  still rejects, now with a named operation instead of unavailable
  terminal homes.
```

Evidence: `terminal_value_return_tests` pins `OpaqueCall` retention,
`terminal_homes = Ok`, install admit for the no-map-argument shape,
and `MapLifecycleUndertaking(UncoveredOperation)` for the map-argument
shape; `map_lifecycle_undertaking_tests` pins `ArgumentHandoff` in the
described set and keeps `OwnerTerminalHomesUnavailable` for the
non-terminal local-call shape. Package suite 247/247; the two
`resolved_semantics` reds are baseline entries in
`cargo_lib_red_baseline.failures.txt`. Changed-file line counts stay
below 800 (`home_new_prefix.rs` 744).

Remaining named residual on the merged route: after C9-2 the
`local main` literal is fully covered (`"name" => EntryStore(Text)`,
`"params"`/`"locals" => EntryStore(EmptyArray)`, `"blocks" =>
OwnershipShare(Handle)`). The first reported `UncoveredOperation` in
`CompatMirEmitBox._module_with_blocks` is now `EntryStore(Opaque)` on
the `%{"functions" => [main]}` argument literal — `[main]` is a
non-empty array whose `main` element also describes
`OwnershipShare(MapLocal)`, and `ArgumentHandoff` sorts behind both
(operation `Ord`: `EntryStore` < `OwnershipShare` < `ArgumentHandoff`).
The next bounded slice is the map-argument cluster the card already
named — `ArgumentHandoff` (call-edge promotion + wire kind +
`exact_formals` + callee map-read lane) together with the argument
literal's own obligations: `EntryStore(Opaque)` needs the non-empty
array residence decision (a marker cannot stand in — elements carry
borrows the payload must retain) and `OwnershipShare(MapLocal)` needs a
borrowed-storage contract, plus `MirJsonEmitBox.to_json`'s own body
admission.

## Review fix note (2026-09-17, review of `8161f2abb9`)

The external review named one current-change regression plus two
hygiene gaps. All three are closed on this branch:

1. **Plain-exit binding expectation counted scalar-route calls**
   (`6b8efa6d8c` follow-up). `expected_local_call_sites` enumerated
   every sealed I64 `local_calls()` row, but the seal predicate
   (`is_i64_call`) is routing-agnostic — a callee with no sealed
   lifecycle product keeps the scalar `Call` route under a Plain
   caller exit (`callee_lifecycle_participant` `continue` arm) and
   owes no binding group. `record_root_home_exit` then froze on
   `local-call-binding-sequence` for
   `main() { local first = helper(10) return 0 }` shapes that
   previously published. `co_seal_lifecycle` — the routing authority —
   now marks each site it routes to `DirectCallExecutionV1::Lifecycle`
   on the ledger (`lifecycle_local_call_sites`), and
   `expected_local_call_binding_sites` returns the marked subset of
   sealed I64 rows. Under Call-exit callers every sealed I64 local
   call is still marked or the co-seal rejects, so the strict
   group↔site ordering check is unchanged there. Pins:
   `scalar_local_then_plain_return_preserves_ordinary_call_through_publication`
   green again (deterministic, was red at `6b8efa6d8c` onward);
   `non_map_local_call_with_plain_return_preserves_scalar` still pins
   the scalar route at package level; `normal_callable_semantic_package`
   257/257.
2. **Stale README authority reference.** `normal_callable_semantic_package`
   README named the deleted `map_install_owners` and the singular
   AppMain loan; it now describes the per-owner
   `DirectCallDispositionLoansV1` evidence (unspent rows, map-target
   ⊆ described owners, no self-target) with the undertaking verify as
   the coverage proof.
3. **`source_result.rs` crossed the 760 split-design line** (783).
   Split at the responsibility boundary: `source_result.rs` keeps the
   product vocabulary, sealed-input identity checks, and issuance
   (335); `source_result_classify.rs` owns the row-driven expression
   classifier (463). `source_result` tests 19/19; `cargo fmt --check`
   clean; warning count unchanged.

Red classification for this fix round (all non-current-change):
parent-reproduced at `ec82461377` — `source_backed_app_main_direct_call_consumes_affine_loan`,
`main_f1_rejects_direct_call_and_nested_owner_before_lowering`,
`published_consumer_runs_once_and_propagates_failure_without_retry`,
`published_consumer_does_not_consume_explicit_compatibility`,
`actual_string_helpers_general_result_row_*`,
`source_bound_static_result_owner_*`, `source_backed_package_failure_*`,
`parser_scan_package_passes_callable_source_handoff_*`,
`instance_box_declaration_lifecycle_preserves_*` ×2,
`instance_method_batch_preserves_prefix_*`, `test_weak_handle_lifecycle`,
`global_call_route_plan` ×2, `mir_corebox_router` ×2. Order-dependent
flakes green under `--test-threads=1`/standalone:
`module_lifecycle`/`capture_tests` family (`mirbuilder_minimal_*`,
`verified_main_*`, `shared_root_kernel_*`,
`instance_box_declaration_lifecycle_stops_*`, `typed_array_source_*`),
`map_write_timing::boxcall_delegation`, `mir_corebox_router` extra rows.

## External design consultation #4 (2026-09-18, ChatGPT Pro — map-argument lane)

Inquiry: `CHATGPT_PRO_INQUIRY_MAP_ARGUMENT_LANE_JP.md`; full response:
`CHATGPT_PRO_RESPONSE_MAP_ARGUMENT_LANE_JP.md`. Factual claims verified
against HEAD `17edb9c766` — the int/handle `I64` tag conflation in the
`Borrowed` store, `MapInstallFailure.candidate` dropped without semantic
`end()` at the kernel boundary, `to_json`'s actual read surface
(`module.get`/`length`/`get`/`keys`/`is_map`/`is_array`/missing-sentinel +
`functions_0` fallback + `len==0` repair), and the write-only
`observe_native` → `ProjectionUnavailable` bound all check out.

**Decision (accepted 2026-09-18):** `%{"functions" => [main]}` is a
caller-owned temporary Map that owns its Array storage whose elements
borrow the existing `main` map. The first call contract is a
**synchronous read-only no-escape borrow**: the callee never takes
dispose responsibility; on Normal and Fault the caller cleans the
temporary region after the callee returns (borrow liveness ends when the
containing region's cleanup completes, not at callee return). Consume /
mutable / escape forms stay named rejections until their own contracts
exist. `BorrowedEntryEscape` stays — a borrow handoff is admitted only
when the exact formal contract, every borrow root's liveness, and both
Normal+Fault cleanup paths are proven; capability-set membership alone
never admits.

Representation prerequisites recorded by the response (close before
read lanes open):

- int vs handle: the `Borrowed` store currently conflates to
  `CheckedMapPayload::I64`; the semantic tag must survive store→read —
  no type recovery from key names, values, or registry hits.
- `EmptyArray` converges to a zero-length Array view, not a fresh
  mutable ArrayBox per projection (alias semantics of `MapBox.get`);
  unimplemented identity observation stays a named rejection.
- `install_candidate` failure paths must gain staging-ownership or
  explicit cleanup before payloads can own children — Rust Drop is not
  semantic `end()`.

Layer placement follows the existing chain (no second compiler, no
omnibus registry, no receipt chain): Facts carry owner/site/containment/
binding/ordinal; parameter-result contracts carry formal value-kind +
access(ReadOnly) + escape(NoEscape); Recipe/cleanup carries staging
owner, evaluation order, borrow window, release order; verify co-seals
exact actual↔formal, root liveness, mutation/move/end prohibition
(alias, re-entry, helper calls included), callee non-escape, and
full-member coverage; physical emit binds BindingRef→value/placement
once; runtime keeps storage state, lookup, bounds, kind tags.

Work packages T1–T5 are queued in the Ordered bounded card queue below.
T1 is the vertical slice; T2's Array staging/borrow relations may be
prepared in parallel. Deferred placements (response §9): C1-id stays
pre-production homework running in parallel (mandatory before its issuer
touches production, not a stop reason for read work); C6-4 rootless
stays an independent family unless the required artifact is
library-only; owned MapChild reuses T2's containment machinery only when
a real nested-map transfer needs it — never as a borrowed-MapLocal
substitute; received-map return / escaping borrows stay closed under
`BorrowedEntryEscape`. Production switch conflates two censuses
(response §10): compiling `CompatMirEmitBox` ≠ retiring the compat MIR
path it emits — Rust-side legacy lowering/writer/reader and .hako-side
fallback/repair are separate caller-zero deletion units.

### Ordered bounded card queue — map-argument work packages (T-series)

| #   | Work package (exit shown as running evidence) | Depends |
| --- | --------------------------------------------- | ------- |
| T1  | readable Map argument end-to-end — a small ordinary callee receives a Map and its result is decided by a real `get` (source→MIR→OBJ→link→execute; Normal + callee-read-Fault + caller-temporary-cleanup-Fault). Requires: semantic tags surviving store→read (int vs handle), read ABI (lookup/length/indexed/kind-test/scalar projection as tagged views bound to the borrow window — no clone, no fresh ownership), formal borrow contract (value kind + ReadOnly + NoEscape per slot — not a blanket exact_formals lift), call-edge co-seal, Invoke + FaultFrame for fallible callees, both cleanup paths. Negative: kernel-only green, capability-set addition alone, constant-return shortcuts, Main-only authority | C9-2 |
| T2  | `[main]` end-to-end — callee actually reads `funcs[0].name` (Text), `params` empty array, `blocks` kind/length from runtime storage. Requires: owned `Array` payload with staging owner + reverse-prefix cleanup on mid-construction fault, `BorrowedMap`/`BorrowedHandle` payload kinds (physical-side refs only — never in source Facts), transitive liveness (argument→Array→main→blocks), `[main, main]` double-borrow without double-end, fault injection at construction/install/callee/cleanup per real owner. Negative: markers, whole-map serialize, probe side-tables, blanket `EntryStore(Opaque)` admission | T1 |
| T3  | original `to_json` unmodified — `return MirJsonEmitBox.to_json(%{"functions"=>[main]})` compiles and runs; JSON verified on finite fixtures (empty/non-empty blocks, params, flags, Unicode/escapes); returned Text outlives input-map cleanup (no borrowed view returned as Text result). Requires: resolved call-graph feature census first (get/length/get/is_map/is_array/flags.keys/recursion/early-return/Text ops — enumerate before building, not discovered one-by-one), qualified-call lane, recursion closed inside the existing callable cohort, Text-result ownership ABI. `to_json` body admission is a separate job reusing T1/T2 read contracts — not folded into one commit | T2 |
| T4  | merged compiler runs — real merged source's MIR/OBJ/execution plus result agreement on a finite program the compiled compiler produces. Requires: full member coverage (unreached members don't count as covered), real ingress, publish, backend, actual use — tracked at same revision/config; missing input artifacts never count as success | T3 |
| T5  | production switch + physical deletion — the named production caller switches to the new route; the selected family's old edges are deleted after caller-zero. Requires: the production entry, backend/profile, canonical tuple, and deletion unit named at start; separate evidence for seal / finalized artifact / physical input / OBJ-link / execution / default switch / caller-zero / physical deletion — no step's success counts as the next's proof; opt-in success and comment-only deprecation don't count | T4 |

Reporting contract per package (response §7/8): changed contracts, the
source, generated artifacts, execution results, Normal/Fault owners,
named rejections, and remaining old production edges — recorded on this
card at each landing. C8's `#[ignore]`d LLVM/runtime tests run from an
acceptance runner explicitly; environment-missing skips never count as
gate success. Internal commits keep BoxCount (semantic admission) and
BoxShape (behavior-preserving split) separate; 760-line split design,
800-line hard stop.

### T1 bounded implementation plan (2026-09-18, read-only worker audit)

Read-only worker audit (agent 804f9e55) mapped the T1 surface against
HEAD `cb2adda501`. Findings: (a) `m.get("k")` seals as a neutral
`MethodCall` row — no `TerminalRelationV1` map-read variant exists;
terminal stop `ReturnValueNotCovered`, initializer stop
`PrefixNotCovered` (corrects the earlier card note: `to_json`'s
`module.get` dies at `PrefixNotCovered`, not `EntryStore(Opaque)`);
(b) the all-i64 formal restriction is three-layered —
`validate_exact_i64_header` (callable_index.rs), `exact_formals`
(direct_call_lifecycle.rs), `ExactTrivial*` ABI spelling; the carrier
`CallableParameterContractKindV1` exists but `DeclaredHandle` discards
the declared box name; (c) `MapCallEdgeContractV1` vocabulary is never
constructed — the edge-conformance plug-in point is
`preflight_map_install`; (d) no read lane at any layer — no
`MapInvokeOperation::Get`, no `nyash.map.checked_get_*`, C emit prints
`i64` per param, `CheckedMap` is deliberately write-only; (e) the
`Borrowed` store conflates into `CheckedMapPayload::I64`
(selected/map.rs).

```text
Decision: split T1 into T1-α (callee-side read contract: `: MapBox`
  formal + `return m.get("lit")` scalar lookup → i64, verified
  read-only + no-escape) and T1-β (caller-side argument handoff:
  `ArgumentHandoff` capability + call-edge co-seal + caller emission +
  the full end-to-end fixture). The checked lane's scalar-get contract:
  Present(I64)→value, Missing→0, non-scalar payload→Fault — a new lane
  contract; no VM `.get`/sentinel semantics are changed.
Source authority + canonical issuer: the `: MapBox` parameter
  declaration (existing callable-parameter-contract issuer) plus the
  callee body's sealed rows (existing terminal-relation scanner).
Non-authority: runtime payload shape, key names, registry hits; the
  `DeclaredHandle` legacy kind does not carry the map contract.
Fail-fast boundary: non-scalar payload reaching the i64 projection;
  any use of `m` outside the admitted read (store/return/argument/
  mutation stays uncovered); untyped or foreign-owner formals;
  unsupported declared types.
Smallest next slice: T1-α — contract kind + header/signature admission
  + `MapLookup` terminal relation + `ptr` param ABI + `checked_get`
  kernel op + C emit + `#[ignore]`d probe that invokes the compiled
  callee against hand-built checked storage.
Non-claims: no caller `ArgumentHandoff`, no borrowed-handle entries,
  no array/text/key-enumeration reads, no `to_json`, no production
  switch, no legacy retirement.
```

Bounded order inside T1 (each its own commit, BoxCount only):

1. **T1-α callee read contract** — new contract kind `Map` minted for
   `: MapBox` (read-only/no-escape by construction; a second mode would
   be a new variant); `validate_exact_i64_header` +
   `ExactTrivialCallableSignatureV1` carry a per-slot kind; new
   `TerminalRelationV1` variant for `return <map-formal>.get("lit")`;
   physical signature param kind (`ptr` map-storage wire);
   `MapInvokeOperation` read op + `nyash.map.checked_get_*` kernel
   export + `map_get` JSON/C emit; probe executes the compiled callee
   against storage built through existing `checked_new`/install
   symbols — no caller lane needed.
2. **T1-β caller argument handoff** — `ArgumentHandoff` in
   `map_lifecycle_capability()`; `MapCallEdgeContractV1` rows built and
   co-sealed at `preflight_map_install` (CallArgument destination ×
   callee map formal × callee evidence); caller emission constructs
   `%{"k"=>7}`, passes the `%mapN` storage pointer, cleans on Normal
   and Fault; `exact_formals` admits the map formal; end-to-end
   `main(){ return Helpers.read_k(%{"k"=>7}) }` executes.
3. **T1-γ tag coverage** — `CheckedMapPayload` borrowed-handle variant +
   tagged install op + kind-mismatch read evidence (a handle entry can
   never be misread as i64); `BorrowedEntryEscape` admission under the
   proven edge contract if a borrowed entry rides the argument map.

## T1-α landed evidence (2026-09-18, callee-side readable Map contract)

Landed: the `: MapBox` formal mints `CallableParameterContractKindV1::Map`
(callable-parameter-contract issuer, header `MapBox` spelling only — the
`DeclaredHandle` legacy kind never carries it). `StoredLocal::BorrowedMap`
separates the borrowed formal from owned map locals in the prefix flow, so
`return m` can never ride the map-local return lane. `TerminalMapGetReturnV1`
(`TerminalRelationV1::MapGet`) seals `return <map>.get("<literal>")` with
owner, sites, receiver binding/class (`OwnedLocal` | `BorrowedParameter`)
and the sealed literal key; `has_map` admits Map-formal owners into the
homes-aware walk, and `retain_child_terminal_relation` keeps the relation.
The ledger issues `PreparedTerminalMapGetReturnV1` /
`RootHomeExitEntry::MapGet`; `record_root_map_get_exit` creates the
`Invoke{CheckedGetI64}` on the function's own fault frame (Borrowed-mode
children accepted — mode is re-validated by frame validation, not by the
entry); cleanup ingress, binding preservation, key identity and local-call
binding groups are validated by `root_map_get_entry`. The builder dispatch
`emit_terminal_map_get_return` (RootExitIngress::MapGet) projects the i64
through `InvokeNormalResult`; finalization seals
`FinalizedRootResultAbiV1::MapGetReturn`.

Physical: `MapInvokeOperation::CheckedGetI64` verified in `invoke_map`
(role, no-escape, live map / map-formal liveness; `normal_result_kind` =
I64). Wire: `param_types` `"map"` representation on the physical ABI JSON
and `hako_physical_params` C admission; `map_checked_get` op carries the
sealed UTF-8 key inline. C emit: MapBox formals declare `ptr`; the op
emits a private key constant, an `i64` out-slot pre-initialized to `-1`
(Fault reads never surface uninitialized stack), calls
`nyash.map.checked_get_i64_v1(frame, site, map, key, len, out)` and
dispatches on status only — no key/outcome/storage bookkeeping. Kernel:
`CheckedMap::read_i64` (Present(I64)→value, Missing→0 written on Normal,
non-scalar→Fault reason 104 `NYRT_FAULT_REASON_MAP_NON_SCALAR_READ_V1`),
exported as `nyash.map.checked_get_i64_v1`, registered in
`runtime_map_symbols` and `include/nyrt_fault_v1.h`.

Evidence: 6/6 `map_get_terminal_tests` (owned-local root + child relations
with cleanup retained; borrowed-formal relation with empty terminal homes;
borrowed-escape, non-literal key and non-get selector all stay uncovered
at `ReturnValueNotCovered`); 265/265 `normal_callable_semantic_package`;
18/18 `physical_program`; `map_read_tests::
issued_map_read_source_exe_probe_observes_get_contract_and_end`
(`--ignored`) green — compiled object linked against
`libnyash_lifecycle_kernel.a` + `published_map_fault_probe.c`
(`HAKO_MAP_READ_PROBE`): i64 read exits 7 with `READ k 1 7 0 1 2 1`,
empty map exits 0, `%{"k" => true}` exits 70 with `REPORT 104` and out
sentinel `-1`; get runs at seq 1, `checked_end` at seq 2 exactly once —
the read leaves the owned storage live until End on Normal and Fault.
`map_`/`terminal` batch reds: recorded baseline set (global_call_route_plan
×2, mir_corebox_router type-certainty ×2, published_consumer ×2, array
route ×3, root_catalog_lifecycle ×2) + `map_write_timing`/env-race flakes —
all standalone-green or parent-reproduced; no current-change failure.

Non-claims: no caller `ArgumentHandoff` (a sealed `Call` edge carrying a
map argument remains T1-β — the uncalled cataloged callee still stops at
`admission-function-not-birth` on the published route, which is the
existing membership contract, not a regression); no borrowed-entry/tag
coverage (T1-γ); no `to_json`, no production switch, no legacy
retirement. The compiled-callee reach is root-owned storage only in this
slice; the same emit path serves `BorrowedParameter` receivers once a
call edge can hand the pointer across.

## Review fix note (2026-09-18, ordinary-call param-ABI conformance)

The review of the T1-α lane found one correctness/security hole:
`static box Main { main() { return read_k(10) } read_k(m: MapBox): i64
{ return m.get("k") } }` compiled with `verification = Ok(())` — an i64
`Const` argument rode an `Invoke{Call}` edge into a callee whose
physical formal is a `MapBox`/`ptr`. The asymmetry proved it a bug, not
a deferred slice: a *map* argument to the same formal failed closed at
admission while an *integer* argument sailed through. Root cause: the
`signature.params().any(|k| k != I64)` rejection sat behind the
`local_call_for_owner` `continue`, so the terminal `return <call>` arm
never consulted it, and `materialize_call` validated arity only. The
malformed edge passed semantic seal, MIR verification and physical
validation; only LLVM IR emission would have rejected the incompatible
signature — and a slot spelled `i64` would have dereferenced the integer
as a map pointer.

Fix — one rule at the sealed-contract boundary plus independent
fail-closed layers; no caller-side map handoff is claimed:

1. `co_seal_lifecycle` scalar arm now rejects any non-`I64` sealed
   formal **before** the `local_call_for_owner` lookup, covering the
   terminal `return <call>`, local `local x = <call>` and
   non-participant `continue` arms with one check. The `map_owned` arm
   keeps rejecting through `exact_formals`.
2. `VerifiedTrivialDirectCallV1::seal` applies the same all-`I64` gate
   to the `InlineI64` route (`DirectCallTargetMismatch`).
3. `VerifiedCanonicalDirectCallEmissionV1::materialize` /
   `materialize_call` gain `ScalarParameterAbi { index }`; every
   emission lane (`into_scalar_emission`, `lifecycle_emission`,
   trivial-SSA `emit`/`emit_resolved_header`, `root_call_entry`)
   funnels through it, so a malformed edge cannot be constructed even
   if a caller bypasses the seal.
4. MIR verifier `check_call_edge` (`invoke.rs`) resolves cataloged
   same-module callee keys from the published carrier and enforces:
   arity including instance receivers; no `Box("MapBox")` formal on the
   scalar edge (it mirrors the wire `"map"` representation exactly);
   a recorded argument type must be `Integer`/`Unknown`, never a proven
   non-i64 kind → `call-argument-type-drift`. Uncataloged callees stay
   outside this relation's authority.
5. C `hako_physical_validate_ordinary_call` requires per-argument
   `kind == "i64"` **and** callee param `representation == "i64"` — the
   edge is i64-only end to end because operands are spelled `i64 %v`;
   a `"map"` formal and a `"map"` argument kind both fail closed.
6. V4 indexed flow enforces the identical i64-only relation on live
   values (`representation == "i64"` → `LV4_I64`, else `LV4_BAD`).

Scope kept: `MapBox`→`MapBox` argument handoff remains T1-β — every
layer above rejects it rather than silently widening; the emitter's
unconditional `i64 %v` spelling is now the only reachable shape, so the
emitter needed no change.

Pins: `direct_call_lifecycle_tests` +3 — `i64_parameter_callee_accepts_scalar_call_arguments`,
`map_parameter_callee_rejects_scalar_call_arguments` (terminal + local ×
`m.get` + plain bodies), `map_owned_map_parameter_callee_still_rejects_direct_call`;
`canonical_direct_call_tests::rejects_non_scalar_parameter_abi_before_materialization`;
`invoke_call_tests` +3 — `cataloged_call_rejects_non_i64_parameter_on_the_scalar_edge`
(typed + untyped), `cataloged_call_rejects_proven_non_scalar_argument`,
`cataloged_call_accepts_i64_argument_contract`;
`published_map_physical_execution_test.py` — `i64 argument -> i64 formal`
compiles; `i64`→`map` and `map`→`i64` argument/formal drift both reject
`published-lifecycle-physical-parser/function-body`.

Suites: `normal_callable_semantic_package` 268/268, `verification`
107/107, `resolved_value_profile` 65/65, `canonical_direct_call` 6/6,
`physical_program` 18/18; `published_map_physical_execution_test.py`
full suite green, `published_lifecycle_v4_execution_test.py` full suite
green, `published_lifecycle_v4_nested_call_test.c` pass; `cc
-fsyntax-only -Wall` warning count identical to baseline (85).
`direct_call`-filtered reds `source_backed_app_main_direct_call_consumes_affine_loan`
+ `main_f1_rejects_direct_call_and_nested_owner_before_lowering` are
recorded in `cargo_lib_red_baseline.tests.txt` — known baseline debt,
unchanged.

Non-claims: no `ArgumentHandoff`, no call-edge map co-seal, no
`main(){ return Helpers.read_k(%{"k"=>7}) }` fixture (all still T1-β);
no tag coverage (T1-γ); no `to_json`, no production switch, no legacy
retirement.
