# 3474 - LANGV1-HAKO-MATCH-RECORD-DELIMITER-OWNER-001

## Status

Active code-facing implementation card after 3473 closes explicit Hako
compatibility-transport exclusion.

Decision: accepted by 3471 Decision B.

## Selected Contract

```text
delimiter owner:
  Match parser

expression API:
  explicit delimiter-aware ExprContext

Match scrutinee policy:
  MatchScrutineeStopsBeforeTopLevelBrace

general expression policy:
  GeneralExpression
```

`match value { ... }` must parse `value` as the scrutinee and the top-level
`{` as the Match arm delimiter. General expression parsing must retain record
literals. A record literal used as a Match scrutinee must be explicitly nested,
for example `match (Value { field: 1 }) { _ => 0 }`.

## Structural Implementation

1. Introduce one small expression-context owner instead of adding policy state
   throughout `parser_expr_box.hako`, which is already near the 800-line source
   boundary.
2. Pass the context explicitly per parse invocation; do not use ambient or
   process-global parser state.
3. Make `ParserMatchBox` invoke expression parsing with
   `MatchScrutineeStopsBeforeTopLevelBrace`.
4. Keep recursive and general expression entrypoints on `GeneralExpression`
   unless they intentionally establish a nested delimiter context.
5. Stop record-literal recognition at a top-level brace only under the Match
   scrutinee policy.
6. Preserve existing record literal and record update behavior outside that
   policy.

## Forbidden Designs

```text
source slicing and reparse
Canonical reject followed by alternate parse
declared-record inventory as grammar authority
capitalization or identifier-name heuristic
AST rewrite disambiguation
process-global expression context
fixture-specific source checks
canonical Match syntax change
runtime or backend fallback
```

## Fail-Fast Tags

```text
parser/match_scrutinee_brace_context_missing
parser/match_arm_block_expected
parser/match_record_delimiter_drift
parser/record_literal_reparse_forbidden
parser/record_inventory_gate_forbidden
parser/name_heuristic_forbidden
parser/match_scrutinee_unclosed
parser/match_arm_arrow_expected
```

## Fixture Matrix

```text
match value { Ready(x) => x, Idle => 0 }
  -> Match with Name(value) scrutinee

match getState() { Ready(x) => x, Idle => 0 }
  -> Match with Call scrutinee

match obj.field { Ready(x) => x, Idle => 0 }
  -> Match with Field scrutinee

let r = Value { field: 1 }
  -> RecordLiteral in general expression context

match (Value { field: 1 }) { _ => 0 }
  -> Match with parenthesized RecordLiteral scrutinee

declared-record inventory absent or present
  -> same parse result

malformed Match arm or unclosed Match
  -> stable fail-fast, no loop and no record reparse
```

## Acceptance

```text
match_delimiter_owner_count = 1
match_delimiter_owner = ParserMatchBox
match_scrutinee_context_explicit = 1
general_record_literal_retained = 1
record_inventory_grammar_authority = 0
source_slicing_reparse_fallback = 0
name_heuristic_count = 0
ast_rewrite_disambiguation = 0
parser_source_over_800_lines = 0
docs_only_closeout = forbidden
```

Verification must include focused Hako parser unit/corpus fixtures, the shared
grammar substrate guard in normal and full modes, the current-state pointer
guard, and `git diff --check`.

## Non-Claims

```text
match_canonical_syntax_change = 0
declared_record_inventory_authority = 0
broad_parser_rewrite = 0
rust_parser_behavior_changed = 0
runtime_backend_changes = 0
language_v1_grammar_closeout = 0
selfhost_claim = 0
```

## Next

After this card is green, proceed directly to the accepted
`MIR-CONVERGENCE-ROUTE-FAMILY-GRAPH-SHADOW-001` task. Do not create a separate
inventory or rerun card.
