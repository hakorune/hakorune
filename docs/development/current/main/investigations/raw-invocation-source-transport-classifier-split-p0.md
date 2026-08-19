# Raw Invocation Source Transport Classifier Split P0

Status: landed
Parent: `brand-constructor-consumer-cutover-d0.md`
Row: `RAW-INVOCATION-SOURCE-TRANSPORT-CLASSIFIER-SPLIT-P0`
Classification: BoxShape

## Execution brief

Decision: Extract the statement-location classification policy from the
760-line raw source transport owner into one private child without changing any
located or compatibility disposition.
Source authority + canonical issuer: `RawInvocationSourceContextV1` remains the
sole source-path owner; the extracted child only returns the existing finite
located/unlocated classification for one AST statement.
Non-authority: Brand relations, call spelling, resolver products, AST spans,
tests, and future consumer needs cannot change a classification in this row.
Fail-fast boundary: Every AST variant must retain its current disposition and
stable `CallObject` reason; missing or duplicate classifier coverage fails the
focused exhaustive matrix before production use.
Smallest next slice: Move `reason_for_non_box_statement` and the four
`is_located_*` helpers into a bounded private child, keep the parent path
mechanics unchanged, and add a reusable structural/behavior guard.
Non-claims: No newly located FunctionCall/MethodCall, Brand projection or
consumer, raw probe retirement, semantic receipt, compatibility retirement,
runtime behavior, or other admission change.

## Acceptance

- The parent falls below 760 lines and the new child remains bounded.
- All existing located scalar/control/lambda/zero-child rows stay located.
- Every existing compatibility row retains `CallObject` and identical source
  transport behavior.
- Exact child/body path construction and temporal scope restoration are
  unchanged.
- Focused positive/negative tests and a reusable guard are green.

## Landed evidence

- `raw_invocation_source_transport.rs` is 655 lines; the extracted private
  classifier child is 197 lines.
- Bare `FunctionCall` and `MethodCall` remain `CallObject` compatibility rows.
  Scalar, control, lambda, and zero-child classifications are unchanged.
- Three focused classifier tests, thirteen existing source-transport tests,
  and `raw_invocation_source_transport_classifier_split_guard.sh` are green.
- This row creates room only. It issues no Brand disposition and changes no
  active source path.
