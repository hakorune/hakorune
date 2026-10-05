# MIRBUILDER-FIELD-ARRAY-GET-I64-RESULT-D0 — Page `me.<ArrayBox f>.get(i)` I64 result authority

Row: MIRBUILDER-FIELD-ARRAY-GET-I64-RESULT-S0
Parent card: `mirbuilder-stored-child-borrowed-call-receiver-d0-2026-10-05.md`
(its "Field Array.get I64 result integrated Decision" and "consumer-shape
refinement" sections are the accepted Decision and remain authoritative
history; this card carries the execution record).

## Integrated Decision (restated)

Read-only worker audit on the parent card established: the real app target
is `HakoAllocPage.block_used: ArrayBox` (page_heap_box.hako40; sites
110/142/170 all use the `handle.block_id` index). `HakoAllocPageModel`'s
`DirectArrayI64` fields are a different family — no `.hako` box exists for
it and `new DirectArrayI64()` providers stop at `FieldContractUnsupported`.

Decision: admit direct `me.<ArrayBox field>.get(<proven-i64 index>)` as an
armed i64 result only when the field's element-i64 is source-proven by a
whole-field integer-store census; keep manifest `Dynamic` otherwise.

- Source authority + canonical issuer: sealed MethodCall/FieldAccess/Me
  shapes, the field declaration, the sole birth provider store, and the
  owning-box integer-store census over the field (direct `me.<f>` and
  proven-alias writers). The refinement proved the named-array callable
  contract is loop-scoped (`Body`/`Condition` placement only), so the arm
  issues inside the existing prefix field-call manifest lane —
  `proven_field_call` in `home_new_prefix_field_call.rs` — as canonical
  issuer; same contract vocabulary (`CoreMethodResultKindV1::I64Value`)
  as every other manifest i64 row.
- Non-authority: manifest `Dynamic` row alone, i64 return annotations,
  MIR/ValueId tags, route-plan origin, `DirectArrayAccessPlan` metadata,
  `owned_object_residences`, class-name-only or physical-tag-only element
  claims.
- Fail-fast boundary: `PrefixNotCovered` for unproven receivers/indices;
  `source-not-i64` through `BorrowedFormalIngress` for an armed borrowed
  `return` of an unproven get source; vetoed fields keep `Dynamic`.
- Smallest slice: direct-receiver arm + whole-field census + the local and
  borrowed-result consumers of the armed contract.
- Siblings kept out: `handle.block_id` formal-field-read index admission,
  declared-DirectArrayI64 arm (blocked on the `FieldContractUnsupported`
  provider prerequisite), condition-position and nested-receiver
  admission, and the physical read owner for the `Callee::Method` get.

## Verified closeout / 2026-10-06

S0 landed. `array_i64_fields.rs` issues the census once in co-seal
preflight; the claim ledger transports `array_i64_fields` to the prefix
arm, the borrowed-result pending fold and test-only accessors. A field is
proven only when declared `ArrayBox`, exactly one birth-store
`new ArrayBox()` provides it (inline initializers reach birth through the
prologue row), every `set`/`push` value on attributed `me.<f>` or
proven-alias writers is integer-source (literals, integer locals, proven
`me.<i64>` reads and proven-field `get` results fixpoint), and no
escape/foreign selector/unattributed write/alias rebind/second provider
remains. `capture_field_array_get` in `ordinary_new_borrowed_formal_
result_pending.rs` seals a borrowed callee's direct or local-bound
`return`; vetoed fields keep manifest `Dynamic` coverage and an unproven
borrowed return fails `source-not-i64` through `BorrowedFormalIngress`.

Evidence: 11 brand-catalog pins + 4 published-view pins green — i64
install, `me.<i64>` index, chained get-as-index, borrowed direct/local
return seal; vetoes for non-integer writes, second provider, escape,
rebind, foreign selector, non-integer/null/homes index; condition-position
keeps the raw-compare boundary; non-borrowed `return me.<f>.get(..)` mints
no terminal relation; `Callee::Method` get stays unpublished until the
physical read owner. Regression battery 1158 PASS / 6 FAIL, all six
reproduced identically on baseline 36b13d8d8e — zero current-change
failures. Unchanged `apps/mimalloc-lite` `--emit-exe` stops at
`artifact-unowned-lifecycle-site`, identical to baseline (app sites use
the `handle.block_id` sibling index). Scope guard stops at the unchanged
root1351 baseline; touched sources stay under 800 (coseal_issue 793).

Staged: `array_i64_fields.rs`; `brand_catalog_array_i64_tests.rs`;
`array_i64_field_call_tests.rs`; census/issue wiring in
`ordinary_new_coseal{,_issue,_issue_lexical,_issue_source}`; ledger field
+ accessors in `ordinary_new_ledger`; predicate/re-export in
`ordinary_new_terminal_home`; profile param in
`ordinary_new_borrowed_formal_profile`; capture arm in
`ordinary_new_borrowed_formal_result_pending`; census plumbing in
`home_new_prefix{,_scan,_arguments,_branch,_field_call}` and
`function_control_new_homes`; module registrations
(`brand_catalog_tests`, `map_value_completion_tests`,
`borrowed_source_publication_tests`); `normal_callable_semantic_package`
README paragraph; this card and the pointer.

Non-claims: `handle.block_id` formal-field-read index admission is the
named sibling — app `isLiveHandle`/`release`/`resizeInPlace` stay
unadmitted; declared-DirectArrayI64 arm still blocked on the
`FieldContractUnsupported` provider prerequisite; no physical read owner
or `Callee::Method` publication route; no condition-position or
nested-receiver admission; production caller switch, selected legacy
retirement, unchanged-app frontier and the finite product goal remain
owed. Parked lanes unchanged; protected sibling WIP intact.

## Formal-field index integrated Decision / 2026-10-06

Design-stop audit for `field-array-get/formal-index-decision` (read-only,
same-thread; no worker needed — all uncertainty resolved against sealed
source). App inventory: every `handle` formal in `release`/`isLiveHandle`/
`resizeInPlace` is untyped (`OpaqueHandle`); `handle.block_id` reads are all
dominated by `if handle == null` guards (admitted `FieldReadOperand` draft
shape already — liveness is that lane's product, not this arm's). The only
index leaf missing from `integer_source_at` is a formal field read:
`handle` carries no declared type and its actuals are Dynamic
(`handles.get(0)`), so neither parameter contracts nor call-site provenance
can prove the class. Within the package's ordinary-box coverage `block_id`
is declared exactly once, non-weak `i64`, on `HakoAllocHandle`;
`HakoAllocFastPathHandle.block_id` lives in a different module outside the
coverage. `handle.page_id`/`requested_size` are ambiguous or write-only
and stay unproven — their sites are conditions/foreign stores already
owned elsewhere.

Decision: admit `formal.<field>` as an integer source when the formal
binding is a package parameter (non-`me`, non-local) and `field` resolves
to exactly one non-weak `i64` declaration across
`ParserOrdinaryBoxSourceCoverageV1::rows()` via the existing
`source_declared_field` lookup — issued as one new leaf inside
`integer_source_at` shared by the census write-value check, the prefix
get-index arm and `capture_field_array_get`. Execution row
MIRBUILDER-FIELD-ARRAY-GET-I64-FORMAL-INDEX-S0.

Source authority + canonical issuer: sealed `FieldAccess` object binding
(`ResolvedLexicalRefV1::Local` parameter kind), the ordinary-box coverage
row set, and constructor-batch field declarations; canonical issuer
`integer_source_at` in `array_i64_fields.rs` with the coverage lookup
carried on the existing co-seal closure channel.

Non-authority: declared parameter types (absent), call-site actual
classes (Dynamic for `handles.get(0)`), null-guard narrowing (liveness,
not class), MIR/layout tags, typed-formal `DeclaredObject` proofs.

Fail-fast boundary: zero or ambiguous declarations, weak or non-i64
declared type, unknown type name, local/alias receivers, and `me`-rooted
objects all keep the existing `false` — `source-not-i64` for armed
borrowed returns, manifest `Dynamic` elsewhere.

Smallest next slice: the single leaf plus focused pins — `get`/`set`
index positions and `set`/`push` value positions in the census fixpoint
(`free_stack.set(me.free_top, handle.block_id)`), ambiguous `page_id`
staying unproven, and the unchanged app frontier observation.

Non-claims: `handle.page_id` ambiguity resolution, formal field stores
(`handle.requested_size = x`), typed-parameter class proofs, alias-chain
receivers (`local h = handle; h.block_id`), condition-position admission,
the physical read owner, app EXE acceptance, production switch and the
finite goal.

## FORMAL-INDEX-S0 landing / 2026-10-06

Shipped boundary differs from the integrated Decision in one place:
`capture_field_array_get` does not credit the formal leaf. Arming it
sealed `return me.block_used.get(handle.block_id)` inside
`HakoAllocPageModel.isLiveHandle`, which grounded the callee and forced
incoming coverage for `me.small_page.isLiveHandle(handle)`; `me.<name>`
receiver calls are outside the lexical need inventory, so the real app
package froze on `borrowed-formal/incoming-coverage` then
`stored-child/result-source-missing`. That receiver-call coverage is its
own bounded row (needs → targets → argument edges), so this slice keeps
the leaf for exactly the census write-value check, the prefix get-index
arm and the argument-neutrality gate, and pins the capture decline as
`source-not-i64` — identical to the pre-leaf Dynamic-get boundary.

Landed evidence: `cargo test --lib -- array_i64` 27/27 green (positive
formal index, `set`/`push` write value, ambiguous/non-integer/local-alias
declines, unarmed borrowed-return pin, and the pre-leaf publication
rows); `page_heap_fixture` 4/4 green — matching the `c887c073d9` baseline
which fails identically only on `map_value_get_*` Unknown contract debt;
`ordinary_new_coseal_issue.rs` 796 / `_source.rs` 785 lines stay inside
the 800 hard stop via the shared `formal_i64_index_consult_v1` factory.

Next owed: `me.<name>` receiver call coverage (sibling row — enables
arming the leaf in the borrowed-result capture), then physical read
owner, production switch, legacy retirement.
