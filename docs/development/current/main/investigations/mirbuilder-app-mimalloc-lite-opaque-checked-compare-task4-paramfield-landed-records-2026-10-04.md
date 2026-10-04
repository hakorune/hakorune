# mimalloc-lite opaque checked-compare task-4 — archived PARAMFIELD prerequisite landed records

Status: archived verbatim from
`mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-d0-2026-10-03.md`
at the post-series frontier census (2026-10-04), when the active card
crossed the 1000-line active-document boundary again. These are the
six earliest landed records of the PARAMFIELD prerequisite series —
I64RESULT-S0, ROOTSOURCE-S0, USESIZE-T0, NULLCOMPARE-S0, NULLACTUAL-S0
and the FIELDSIZE-T0 size extraction. Evidence pins,
EXE receipts and frontier notes are unchanged; the PARAMFIELD-S0,
FIELDRESULT-S0 and PARAMFIELD-ACCEPTANCE-R0 records (the current
landed-tail window) remain in the active card.

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-I64RESULT-S0)

Landed (`976257bcf0` on `codex/birth-definition-publication`):

- Scope taken: an unannotated borrowed callee whose complete explicit
  value-return set is source-proven Integer/exact-I64 corroborates the
  executable I64 projection. `corroborate_borrowed_i64_result_v1` admits
  `row.result() == None` only when the retained declared contract is
  `DeclaredFunctionResultContractV1::Unannotated` — `Void` shares `None`
  and stays rejected — and `corroborate_terminal_lexical_result_v1`
  admits the unannotated row only beside the same owner-issued I64-class
  proof with `contract_corroborated`. The declaration is never
  reannotated, no `Verified*`/`Prepared*` receipt is created, and the
  source-result/contract owners stay where they are.
- The accepted local-new actual promoted to a real positive: local
  `new Item(..)` + `return s.release(h)` and its discard form publish the
  I64 invoke through `TypedHome` kind-3 transport; the bound
  `local r = s.release(h); return r` terminal stays pinned at
  `artifact-actual-root-source-missing` for ROOTSOURCE-S0.
- `ordinary_new_borrowed_formal_result.rs`=517, owner tests=394,
  publication-new-argument parent=723 (I64RESULT witness moved to a new
  145-line child so the parent stays under the 760-line design boundary).

Evidence pins (test profile `--lib`):
`borrowed_call_result_accepts_unannotated_complete_i64_source` (literal,
multi-exit and exact-formal positives; declared contract stays
`Unannotated`, `result()` stays `None`, proof corroborated),
`borrowed_call_result_keeps_unannotated_i64_bounded` (mixed exits,
implicit exit, `: bool`, `: void` and opaque-return negatives — the
opaque return stays outside the borrowed profile at the use-draft
frontier and never acquires the permission),
`unannotated_borrowed_i64_result_publishes_call_forms` (direct/bound/
discard/multi-exit plus typed-home positives: `ordinary_call` `i64`,
callee `ordinary_i64` role, `borrowed_kind_payload_v1` carrier, kind-1/
kind-3 actuals and `invoke_normal_result`),
`parameter_field_frontiers_stay_fail_closed` re-pins 12 named stops —
literal-null/inline-new/received-nullable actuals
(`borrowed-actual/unsupported-or-unavailable`,
`IncompleteOrdinaryNewCoverage`, `artifact-source-unavailable`) and the
bound root-source stop (`artifact-actual-root-source-missing`),
`borrowed_` sweep 207/207, package/`resolved_semantics` sweep 922/925 —
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once` and
`qualified_call_map_argument_reaches_the_named_capability_boundary`
reproduce identically without this change (baseline debt);
qualified-route scope guard PASS except `brand_catalog_tests.rs=961`
known structural debt; pointer guard PASS.

Open frontiers (deliberately out of this slice): bound-result root
source, null equality, null/received-nullable actuals, formal field
read, field result, Bool result, inline-new actuals, `.get`,
`me.`-receiver calls and the app/migration route.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ROOTSOURCE-S0
(retain verified main source for birth/local-call actuals independently
of the terminal map).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ROOTSOURCE-S0)

Landed (`93b0cdaee8` on `codex/birth-definition-publication`):

- Scope taken: `seal_finalized_root_birth_handoff` now issues the root
  source loan from the union of retained inventories — terminal
  relations, checked birth actuals and lexical local-call groups — with
  the verified main owner carried explicitly on
  `FinalizedRootSourceHandoffV1`, so presence no longer derives from
  `!terminal_relation.is_empty()` and an actually-empty relation map is
  legitimate transport data. The missing-source checks
  (`artifact-actual-root-source-missing`,
  `artifact-local-call-root-source-missing`) stay verbatim.
- The bound-i64 return class: `return_scalar` reclassifies a local bound
  to a proven-i64 source (literal store, exact formal, field-read or
  classified call result — the stored `SourceScalarKind::Integer` is the
  sole class authority) as `ReturnScalar::Integer`, and the terminal
  observer mints `TerminalI64ScalarReturnV1` (owner + exact return/value
  sites only) as `TerminalRelationV1::I64Scalar`. `result_abi` projects
  it to `FinalizedRootResultAbiV1::I64ScalarReturn` → compiled `I64`;
  `Value`/`OpaqueCall` still return `None` and a retained loan without a
  result ABI keeps failing at the unchanged
  `retained-root-result-missing` boundary.
- The card witness `local h = new Item(..); local r = s.release(h);
  return r` publishes end to end through the existing binding-group and
  generic-return lanes; bound literal/copy/trivial-add and the
  no-`new`-argument bound call publish the same way.
- File sizes: `home_terminal_relation.rs`=785,
  `home_new_prefix_terminal.rs`=632, `ordinary_new_local_commit.rs`=741,
  `finalized_root_handoff.rs`=319 — all under the 800-line stop.

Evidence pins (test profile `--lib`):
`bound_i64_scalar_issues_exact_terminal_relation` (bound literal, copy
chain and proven field-add mixes mint the exact owner/site relation),
`empty_terminal_root_source_keeps_verified_owner_without_abi` (the loan
retains owner and lexical groups with an empty terminal map; `result_abi`
honestly reports `None`),
`bound_i64_returns_publish_through_root_source` (publication witness:
`root_i64` main, `invoke_normal_result`, kind-3 typed-home actual),
`unproven_bound_returns_stay_fail_closed` (bound bool/object →
`retained-root-result-missing`; bound text → `artifact-source-unavailable`;
pure main → `artifact-root-completion-unavailable`),
`mixed_or_unproven_add_discards_terminal_and_all_staged_reads` re-pinned
(the two `local value = 1` suffixes promoted to the new positive test),
`parameter_field_frontiers_stay_fail_closed` re-pins 10 named stops,
`borrowed_` sweep 209/209, package/`resolved_semantics` sweep 924/927 —
the same 3 baseline reds reproduce identically without this change
(`birth_receiver_non_escape...`, `main_static_child_port...`,
`qualified_call_map_argument...`); qualified-route scope guard PASS
except `brand_catalog_tests.rs=961` known structural debt; pointer guard
PASS.

Open frontiers (deliberately out of this slice): pure-main bound returns
without any checked `new` (`artifact-root-completion-unavailable` is the
unchanged gate), bound bool/nullable/text/object results, null equality,
null/received-nullable actuals, formal field read, field result, Bool
result, inline-new actuals, `.get`, `me.`-receiver calls and the
app/migration route.

#### USESIZE-T0 — landed record

BoxShape extraction, no semantic change: `ordinary_new_borrowed_formal_uses.rs`
(771 → 552 lines) keeps the draft types, `draft_borrowed_incoming_calls_v1`,
`join_borrowed_forward_uses_v1` and the main use loop; the six operand
classifier/helper bodies (`compare_operand_kind`, `normal_integer_operand`,
`add_operand_kind`, `is_call_argument`, `use_dominated_by_if`,
`sequence_ordinal`) move verbatim into the new private child
`ordinary_new_borrowed_formal_use_operands.rs` (246 lines). Only private
visibility changed (`pub(super)` + one `use operands::{...}` re-export in the
parent so `array_element`/`new_argument` children resolve their existing
`super::` imports untouched). Evidence: git-verbatim diff of moved bodies
(only the `fn`→`pub(super) fn` prefix and one trailing blank line differ);
`borrowed_` sweep 209/209 identical to baseline; package sweep 575/578 —
the same 3 baseline reds; scope guard pins all moved symbol spellings and
the parent shrink, PASS except `brand_catalog_tests.rs=961` known structural
debt; pointer guard PASS. No null/field admission, no test changes.

#### NULLCOMPARE-S0 — landed record

Landed (`160cd13a8e` on `codex/birth-definition-publication`):

- Semantic envelope: `dynamic_operator_contract` gains
  `DynamicOperatorFamilyV1::Equal`, `DynamicOperatorValueClassV1::Null` and
  `DynamicOperatorSuspensionV1::NonSuspending`; the issuer seals exactly one
  new envelope `Equal(Dynamic, Null)` — `TrivialBool` result, no carrier
  lifecycle obligation, both source operand orders. Existing envelopes keep
  `MaySuspend` and are unchanged.
- Source admission (`ordinary_new_borrowed_formal_use_operands.rs::
  null_compare_operand_kind`): a formal use drafts
  `NullCompareOperand{binary}` only when exactly one binary expression
  contains the candidate site, the operator is `Equal`, the binary is a
  direct `if` condition, the sibling is the exact
  `ResolvedLiteralSourceV1::Null` literal, and the envelope issues. `!=`,
  integer `0`, Bool `false`, another binding as sibling, ambiguous multiple
  matches and equality outside a direct `if` all stay `UnsupportedUse` —
  the unchanged fail-closed artifact boundary.
- Projections: `null_compare_uses` (entry) and
  `borrowed_ordinary_null_compare_uses_v1` (finalized source projection)
  reuse the same owner/function binding checks as the existing borrowed-use
  projections.
- Use verifier: `FunctionUses.null_admissions`, `Scan.null_uses` and
  `ViewScan.null_consts` (exact `ConstValue::Null` producers). `Compare{Gt}`
  keeps the checked normal-integer compare path; `Compare{Eq}` admits a
  tracked or lent operand only beside an exact null producer; every other
  compare operator touching a tracked/view operand stays
  `borrowed-use/forbidden-operand`; per-admission coverage closes at
  `borrowed-use/null-coverage`. The null-verification block lives in the new
  `borrowed_call_uses_null_compare.rs` child so the parent stays at 785
  lines (<800).
- Physical: the JSON emitter spells admitted null equality as
  `borrowed_null_compare` (`predicate: "eq"`, original lhs/rhs order, exact
  `const_null` sibling) while ordinary integer equality keeps `compare`.
  The C parser registers `const_null`/`borrowed_null_compare`
  destinations, validates the exact five-key shape and eq-only predicate;
  the index seeds the row as `LV4_BOOL`; the flow admits exactly one
  carrier (TAGGED/HANDLE/I64/BOOL lane) beside the exact null producer and
  admits `const_null` in a function carrying the dedicated row; emission
  reads the carrier's kind/payload lane — never the Integer view — so a
  well-formed non-null kind answers false without a fault branch.
- Publication/EXE witness: `null_compare_publishes_the_dedicated_physical_row`
  emits three source programs (`handle == null`, `null == handle`, object
  argument) — `Store.check/1` keeps `borrowed_kind_payload_v1`, no `compare`
  row appears, two `borrowed_null_compare` rows (edge-port model) carry the
  exact `const_null` sibling. The C execution test compiles all three to
  .o, links the runtime probe and executes them: every non-null carrier
  answers false (exit 3, no fault); seven forged physical rows (wrong
  predicate, ordinary-compare spelling, carrier/carrier, null/null,
  undefined operand, extra key, foreign `const_null`) reject.

Evidence pins (test profile `--lib`):
`null_compare_admits_direct_if_equal_with_null_sibling`,
`null_compare_rejects_wrong_operator_sibling_and_position`,
`borrowed_use_null_compare_view_passes_in_either_operand_order`,
`borrowed_use_rejects_null_compare_operator_and_sibling_drift`,
`borrowed_use_null_compare_coverage_is_per_admission`,
`non_null_equalities_stay_fail_closed_before_publication` (`!=`, `== 0`,
`== false`, `q == handle` all stop at `artifact-source-unavailable`),
`parameter_field_frontiers_stay_fail_closed` re-pinned (`pa-eqnull-newarg`
now reaches `borrowed-actual/unsupported-or-unavailable`,
`pa-eqnull-localnewarg` reaches `admission-function-not-birth` — both
correct frontier advances), dynamic-operator contract suite,
`borrowed_` sweep 216/216, package/`resolved_semantics` sweep — the same
3 baseline reds reproduce identically without this change
(`birth_receiver_non_escape...`, `main_static_child_port...`,
`qualified_call_map_argument...`); all four existing V4 execution tests
rerun green; `published_lifecycle_v4_null_compare_execution_test.py`
PASS; qualified-route scope guard PASS except `brand_catalog_tests.rs=961`
known structural debt; pointer guard PASS.

Open frontiers (deliberately out of that slice): `null`/ReceivedNullable
actuals (NULLACTUAL-S0 — landed below), formal field reads under the
non-null successor (PARAMFIELD-S0), and everything the frontier test
still pins.

#### NULLACTUAL-S0 — landed record

Landed (`d32898b92b` on `codex/birth-definition-publication`):

- Candidate vocabulary (`home_local_call_borrowed_actuals.rs`): the exact
  `ResolvedLiteralSourceV1::Null` literal observes as its own
  `BorrowedCallActualValueV1::Null` — never Integer(0)/Bool(false) — and
  a stored `ReceivedNullable` binding
  (`PrefixLocalFlow::is_received_nullable`, no non-null narrowing mark
  required — the actual carries both runtime states) observes as
  `BorrowedCallActualValueV1::ReceivedNullable(binding)` only when the
  bound value is the exact binding itself; `local q = h` aliases stay
  `Binding` and keep their own rejections.
- Ledger (`ordinary_new_borrowed_formal_actuals.rs`):
  `BorrowedFormalActualSourceV1::Null` and
  `::ReceivedNullable{binding, class}`; the class comes solely from the
  sealed `NullableObject(C)` claim through
  `lexical::nullable_received_result_class` — a fake, foreign, consumed
  or unclaimed producer closes at
  `borrowed-actual/nullable-class-unavailable`.
- Emission: `Source::Null` emits an exact `ConstValue::Null` and rides
  `BorrowedLiteral` projection (verified against the exact producer);
  `Source::ReceivedNullable` rides the binding-read lane exactly like
  Scalar/TypedHome. `emission_validation` now counts recorded `Emitted`
  bindings (clean + fault twins) plus root-local-call binding groups for
  owed exit releases — the `actual=2/owed=1` drift `pa-trivial-handlearg`
  exposed was recorded-group bookkeeping, not a second emitted release.
- Wire: `Null` spells kind `0`, `ReceivedNullable` spells
  `nullable_typed_object` in `borrowed_kind_payload_v1` args. The C call
  transport admits numeric tags 0..3 plus the symbolic kind; indexed
  flow requires the exact `const_null` producer for tag 0 and a live
  HANDLE-origin value for `nullable_typed_object`; the dominated
  tagged-formal gate and the physical formal-transport gate now serve
  `ordinary_nullable_handle` callees as well as `ordinary_i64`; the
  callee prologue admits the (0,0) pair; the caller emits
  `payload != 0 ? 3 : 0` before the call line so the callee's own
  prologue re-proves the pair.
- Publication/EXE witness:
  `null_and_nullable_actuals_publish_their_wire_forms` — the literal-null
  2-box fixture's arg carries kind 0 beside its sole `const_null`, the
  `Handle/Store.check/Store.release` fixture's arg carries
  `nullable_typed_object` beside `invoke_normal_result`. The C execution
  test compiles both and runs three outcomes through the runtime probe:
  literal null releases HOME=1 (root Store only — the sentinel never
  owns a lease), live nullable releases HOME=2 (`s` + `h` via
  `home_release_if_live` exactly once), the null-state nullable
  transports (0,0) and releases nothing; the callee End stays zero in
  every case. Seven forged physical mutations (tag over-range, forged
  tag 0 on a `const_i64` producer, stale literal tag 3, scalar-lane
  value, extra key, result-role drift, `const_null` in a function with
  no admitted null lane) all reject before artifact or at admission.
- Frontier: `pa-trivial-handlearg` is the promoted positive — removed
  from `parameter_field_frontiers_stay_fail_closed`'s fail-closed list;
  the `local q = h` alias variant keeps the exact-binding rejection.

Evidence pins (test profile `--lib`):
`exact_null_actual_selects_the_null_source_not_a_scalar_payload`,
`null_and_nullable_actuals_publish_their_wire_forms`,
`non_null_domains_keep_their_own_tags`; borrowed-actual sweeps 219/219
and package/publication sweeps green; package/`resolved_semantics`/
`publication` sweeps — the same 6 baseline reds reproduce identically
on clean `160cd13a8e` (dynamic-loop prepare, dynamic compare+add
rebind, `birth_receiver_non_escape`, `main_static_child_port`,
`qualified_call_map_argument`,
`reserves_four_mechanical_lanes_without_builder_publication` —
`ObjectDefinitionsNotConsumed` on a plain string-scan fixture,
reproduced clean); C preartifact parser test RC=0, nested lifecycle
test RC=0, all existing V4 execution tests rerun green;
`published_lifecycle_v4_nullable_actual_execution_test.py` PASS;
`published_lifecycle_v4_execution_test.py`'s stale `bad_add` expectation
documents pre-existing `ee30614975` add-view behavior (reproduces on
clean baseline) and is untouched by this slice; qualified-route scope
guard PASS except `brand_catalog_tests.rs=961` known structural debt;
pointer guard PASS.

Open frontiers (deliberately out of this slice): guarded formal field
reads (PARAMFIELD-S0), the field-return result proof
(FIELDRESULT-S0), the bound-alias nullable variant and everything the
frontier test still pins.

#### FIELDSIZE-T0 — landed record

Landed (`8fbfffd487` on `codex/birth-definition-publication`):

- BoxShape only: `prove_local_field_read_batch` and
  `stage_local_field_read_batch` move verbatim into the new private
  `field_batch` child `ordinary_new_coseal_issue_source_field_batch.rs`
  (111 lines); `issue_source` drops 780 -> 681 lines and re-exports both
  (`pub(super) use field_batch::{..}`), so the `source_claims::` call
  sites in `ordinary_new_coseal_issue.rs` and the `use super::*`
  staged-read tests resolve unchanged. Moved functions carry
  `pub(in crate::mir::normal_callable_semantic_package)` — the only
  required visibility change; no predicate, evaluation order, error arm
  or signature changed. Verbatim diff proven
  (`git show` body comparison identical modulo the visibility keyword).
  No null/field admission moved with it.

Evidence pins (test profile `--lib`):
`field_batch` staged-drift tests 2/2, `field_read` 29/29, `coseal`
274/274 identical; qualified-route scope guard PASS (pins relocated to
the child spelling) except `brand_catalog_tests.rs=961` known structural
debt; pointer guard PASS.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-S0
(complete class view + guarded scalar field read).
