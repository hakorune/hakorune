Task: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0
Parent: mir-call-map-local-entry-source-i0-2026-09-15.md
NextCard: F3 terminal relation for non-i64 returns / F4 physical Map
return-argument ABI (ordered after this contract lands)
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
| F3  | `TerminalRelationV1` for non-i64 value returns (Facts extension)   | —            |
| F4  | Map return/argument physical ABI + entry-class operation contracts | C5a, F3      |
| C5b | admission connect: preflight matches full sealed membership + implemented consumer capability | C5a, F3, F4 |
| C6  | downstream contract split (admit_lifecycle retained-root removal, physical doc, C v2) | C5b |
| C8  | two-function non-AppMain consumer acceptance (normal+Fault cleanup, refined above) | C5b, C6 |

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
2. **Facts**: `TerminalRelationV1` has only Call/I64Add/Unit/
   IntegerLiteral/I64Field — `return <local-or-map>` gets
   `ReturnValueNotCovered`, no relation.
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
