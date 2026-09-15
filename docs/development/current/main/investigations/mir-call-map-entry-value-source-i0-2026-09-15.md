Task: MIR-CALL-MAP-ENTRY-VALUE-SOURCE-I0
Parent: mir-call-map-return-site-flow-i0-2026-09-15.md
NextCard: mir-call-map-nested-entry-slot-i0-2026-09-15.md (nested
`%{...}` EntryValue destination); then array-literal and
non-scalar-local entry classes; then C5 lifecycle arm
Implementation permission: landed
---

# Map entry-value source-class coverage I0

## Entry contract

F1 route evidence (merged `/tmp/merged_entry.hako`): all 12
`return %{...}` sites now produce flow rows, but every one rejects as
`Unavailable` via `MapCandidateNotCovered(EntryValue(n))`. The existing
value path in `observe_map` admits only
`OrdinaryObservation::{Integer,Bool,TrivialLocal}` and
`TransferHome` bindings.

Merged entry-value classes actually needed (census: `return %{` sites
around lines 14652-14714):

```text
"op"=>"const"             string literal          (dominant)
"dst"=>dst                local (scalar or non-scalar)
"value"=>%{ ... }         nested map literal      (recursion question)
"locals"=>[]              array literal
arg_ids/insts/blocks      locals holding arrays/maps
```

`Census boundary: merged entry program -> `%{` EntryValue expressions;
includes all return-position maps observed by F1; excludes call-arg and
array-element map sites (separate positions, later cards).`

Entry-value class census (all 48 `%{` sites in merged, ~747 entries;
nested maps counted once as a site and once as a parent entry value):

```text
string literal   283   uncovered
nested %{...}    104   uncovered (also own sites -> loop1 rows)
array literal    126   uncovered
int literal      107   covered (Integer)
local ident      127   partially covered (TrivialLocal only;
                       non-scalar locals uncovered)
bool literal       0   covered (Bool)
```

## Open design questions (worker audit — answered)

1. Which value classes should `MapValueSource`/`MapEntryOwnership`
   admit as exact source facts without claiming new physical ABI?
2. Does a nested `%{...}` entry value require its own flow row
   (recursive observation) or is it a value-source leaf?
3. Array literals and non-scalar locals: existing observation contract
   to reuse, or genuinely uncovered class?
4. Does admission of wider value classes change the transfer/home
   contract (`used`/`compatible`/`remaining` in `observe_map`)?

## Worker audit (d99916bc, code-verified)

1. `observe` admits Integer/Bool literals + `TrivialLocal`/`Handle`
   bindings only; `ResolvedLiteralSourceV1::String` is a unit variant —
   the payload is deliberately dropped at seal
   (`expression_source.rs:88,526`). `ExprClassV1` has `{I64,String,Bool,
   Null,Opaque}` — no Array/Map class; array/map bindings classify
   `Opaque`.
2. A nested `%{...}` gets **its own `MapLiteral` row** AND an
   `EntryValue` relation on the parent (`shadow/expr.rs:100-117`;
   `body_shape_map_tests.rs` proves two rows). Preflight loop1 demands a
   `Complete` row for every `MapLiteral` site → nested maps need a new
   `MapDestinationV1::EntrySlot` + recursion + observation plumbing —
   NOT a pure value admit. Separate card.
3. Param contract kinds (`OpaqueHandle`/`DeclaredHandle`/`ExactText`)
   collapse into `StoredLocal::Handle` at install; only **self-rooted
   param handles** are unambiguous borrows. Home/Map-backed handles are
   a transfer-vs-shared ownership question — separate card.
4. A new `MapValueSource` variant in the `locals.observe` arm does not
   touch `used`/`compatible`/`remaining` (those are TransferHome-only).
   But `scalar_kind()` gates downstream (`map_install_owners`,
   `begin_map_emission`, `validate_map_emission`, emitter) mean any new
   class needs `MapValueKind` extension + emission before it can install
   — this card stays Facts-level, fail-closed downstream.
5. Upstream blocker flagged: bodies with `local x = new ArrayBox()` /
   `[...]` initializers fail at `PrefixNotCovered` before the entry
   check — a separate prefix-coverage card is needed for most merged
   bodies.

## Six-line brief

```text
Decision: admit two Facts-level entry-value classes — (a) string
literals as `MapValueSource::String` (site-only; the sealed literal
payload stays source-owned and emission re-reads it later), and
(b) self-rooted parameter handles as `MapValueSource::BorrowedHandle`
— verified self-rooted (`locals[root] == Handle(root)`) so no live
Home/Map ownership is bypassed. All other classes stay Unavailable.
Source authority + canonical issuer: observe_map's entry loop in
resolved_semantics/home_map_flow.rs owns value-source classification;
PrefixLocalFlow owns local observation and gains the self-rooted
handle query.
Non-authority: no physical emission claim, no runtime layout reads,
no payload unsealing, no widening of TransferHome semantics; a Value
entry never consumes a home.
Fail-fast boundary: nested `%{...}`, array literals, home/map-backed
handles, and any other class stay Unavailable with the named issue;
membership/path checks unchanged; `scalar_kind()` stays None so
downstream install/emission remains fail-closed.
Smallest next slice: the two admits above + focused positive/negative
tests.
Non-claims: Map return ABI, per-owner gate (C5), nested-map EntrySlot
destination, array-literal construction, prefix coverage for
`new ArrayBox()`/`[...]` initializers, non-return map positions.
```

## Acceptance

Focused tests: each admitted class produces a `Complete` row with exact
value-source evidence; uncovered classes stay `Unavailable`; existing
map tests stay green. Route evidence: merged entry's first failing site
advances past `MapCandidateNotCovered` — either loop1 completes for all
return-position maps or fails on a later, different named check.

## Implementation receipt (2026-09-15, landed)

Code:

- `home_prefix_local_flow.rs`: `is_self_rooted_handle(root)` — true only
  when `locals[root] == StoredLocal::Handle(root)`; distinguishes a
  parameter-installed self borrow from any live Home/Map-backed local.
- `home_map_flow.rs`: `MapValueSource::{String, BorrowedHandle(root)}`;
  entry classification admits `Handle(root)` only when self-rooted and
  `ResolvedLiteralSourceV1::String` (unit variant — sealed payload stays
  source-owned, Facts record the site only). `MapHomeEntry::binding()`
  exposes the borrowed root; `String` exposes none.
- `ordinary_new_local_commit/map.rs` + `ordinary_new_admission/
  selected/map.rs`: literal/projected-value matches generalised so
  String/BorrowedHandle fall through to `map-value-consumer-missing`;
  `scalar_kind()` stays `None` — downstream install/emission remains
  fail-closed by construction.

Focused tests (`map_home_flow_tests.rs`, +3; quick profile, 73/73):

- `return_boundary_map_admits_string_literal_and_borrowed_param_handle`
- `return_boundary_map_reuses_one_borrowed_param_across_entries`
- `return_boundary_map_rejects_uncovered_entry_value_classes`
  (nested `%{...}`, `[]`, `local a = p` Home alias, `local a = m`
  map-installed alias — all stay `Unavailable`)
- Updated pin: `borrowed_formals_..._do_not_issue_map_ownership` now
  expects the borrowed-formal entry row `Complete` (Facts admit ≠
  ownership claim; install still stops at `MapLifecycleConsumerMissing`).

Route evidence (merged `/tmp/merged_entry.hako`, quick binary,
temporary eprintln instrumentation — removed):

- Outer label unchanged: `MapLifecycleConsumerMissing`.
- First failing site advanced within the same named check:
  `MapCandidateNotCovered(EntryValue(0))` → `EntryValue(2)` on
  `[Body(0), Value]` (the `make_const` `"value"=>%{...}` nested map).
- Residual uncovered sites: 4 total —
  `Body(0),Value EntryValue(2)` ×2 (nested map class),
  `Body(2),Value EntryValue(2)` ×1,
  `Initializer(0) EntryValue(0)`/`(1)` ×2 (array literal /
  non-scalar-local classes).
- Every residual class is a card-listed exclusion; the route cannot
  pass loop1 until the nested-map `EntrySlot` destination, array
  literal and non-scalar-local ownership cards land — then C5's
  per-owner arm becomes reachable.

Red classification: the single pre-update failure
(`borrowed_formals_...` expected `complete=false`) was a
current-change expectation update to the new contract — the test name's
invariant (borrowed formals never issue map *ownership*) still holds;
`assert_install_stop` unchanged and green.
