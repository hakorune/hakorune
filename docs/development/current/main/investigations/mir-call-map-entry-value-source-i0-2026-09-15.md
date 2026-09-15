Task: MIR-CALL-MAP-ENTRY-VALUE-SOURCE-I0
Parent: mir-call-map-return-site-flow-i0-2026-09-15.md
NextCard: per-owner lifecycle undertaking (C5 arm of map-lifecycle parent)
Implementation permission: pending six-line brief + worker audit
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

## Open design questions (worker audit)

1. Which value classes should `MapValueSource`/`MapEntryOwnership`
   admit as exact source facts without claiming new physical ABI?
2. Does a nested `%{...}` entry value require its own flow row
   (recursive observation) or is it a value-source leaf?
3. Array literals and non-scalar locals: existing observation contract
   to reuse, or genuinely uncovered class?
4. Does admission of wider value classes change the transfer/home
   contract (`used`/`compatible`/`remaining` in `observe_map`)?

## Six-line brief

```text
Decision: pending worker audit — admit the minimum exact entry-value
classes the merged cohort needs; no fake homes, no inferred kinds.
Source authority + canonical issuer: observe_map's entry loop in
resolved_semantics/home_map_flow.rs owns value-source classification;
PrefixLocalFlow owns local observation.
Non-authority: no physical emission claim, no runtime layout reads,
no widening of TransferHome semantics.
Fail-fast boundary: uncovered classes stay Unavailable with the named
issue; membership/path checks unchanged.
Smallest next slice: census per-class counts, pick the covered class
set, extend the value path, focused positive/negative tests.
Non-claims: Map return ABI, per-owner gate (C5), non-return map
positions (call-arg/array-element).
```

## Acceptance

Focused tests: each admitted class produces a `Complete` row with exact
value-source evidence; uncovered classes stay `Unavailable`; existing
map tests stay green. Route evidence: merged entry's first failing site
advances past `MapCandidateNotCovered` — either loop1 completes for all
return-position maps or fails on a later, different named check.
