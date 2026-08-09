# OWN-HOME-NESTED-RELEASE-SOURCE-PATH-P2

Status: parked polish; not a Take/Share blocker
Date: 2026-08-10

Before nested Release is admitted, replace boolean `contains_release` reporting
with an exact parser-owned structural `ParserBodyStatementPathV1` (or an
equivalent parser-local name) containing the top-level ordinal, branch/Loop arm
steps, and nested ordinal. Preserve current fail-fast behavior; this row does
not activate nested Release or Home Flow.

Acceptance requires exact-path positive/negative tests, no AST path after the
parser transaction, no default nested acceptance, no import/reuse of
`src/mir/resolved_semantics::SourcePath*` inside the parser, and same-slice
reference and README updates.
