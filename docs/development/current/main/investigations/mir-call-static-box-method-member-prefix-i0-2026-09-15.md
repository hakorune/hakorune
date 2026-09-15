---
Status: closed__method_member_modifier_admitted__2026-09-15
Task: MIR-CALL-STATIC-BOX-METHOD-MEMBER-PREFIX-I0
Date: 2026-09-15
Priority: stop the `method` member modifier from issuing a phantom Field row in static boxes so the A1 static-parent seal can cover the real merged cohort
Parent: mir-call-static-compatibility-a1-static-parent-co-seal-d0-2026-09-14.md
NextCard: mir-call-resolver-if-source-result-product-issuer-d7-reentry-2026-09-15.md
Implementation permission: true for the parser static-box member classifier and its focused guards only
---

# Static-box `method` member prefix I0

## Six-line brief

```text
Decision: treat `method name(...)` inside a static box as a member modifier, not as a field named `method`.
Source authority + canonical issuer: src/parser/declarations/static_def owns the static member loop; the sealed member rows feed the existing PreparedParserStaticBoxParentSourceV1 unchanged.
Non-authority: ordinary-box member parsing, tokenizer changes, new token kinds, MIR/Builder products, VM, compatibility fallback.
Fail-fast boundary: `method` not followed by `name (` keeps its existing field classification; every other member shape is unchanged.
Smallest next slice: adopt the `name (` head after a leading `method` identifier in the existing member dispatch, so the member row is one DirectMethod and no phantom Field is committed.
Non-claims: ordinary-box `method` cleanup, field/init/static-initializer admission, production acceptance of the merged cohort, and the D7 re-entry source-result slice.
```

`Census boundary: selected merged program entry -> all 128 static parent
declarations; includes the 33 parents whose members use the `method` prefix;
excludes ordinary boxes, nested declarations, and non-member statements.`

## Root cause

`method` is not a token kind; it tokenizes as an identifier. The static-box
member loop dispatches `method` to `try_parse_method_or_field`, which sees a
following identifier instead of `(` and commits a phantom
`ParserStaticBoxMemberKindV1::Field` row. The real method still parses on the
next loop turn, so the AST looked correct, but the sealed member rows carry the
phantom field. The A1 co-seal (`976b6792c3`) then returns
`Outside(UnsupportedMemberKind)`, and the normal source-plan surface rejects the
MixedProgram cohort with `IntegrityInvalid(Surface(StaticParentSourceSealMissing))`.

Diagnostic receipt (temporary parser test against the dumped merged entry
source, 723244 bytes / 131 top-level declarations):

```text
STATIC_PARENT = Outside(UnsupportedMemberKind)
ROOT_EXEC     = IntegrityInvalid(Surface(StaticParentSourceSealMissing))
```

Per-file census shows ~50 `.hako` sources produce the same disposition; inside
the merged cohort every non-method member row traces to the `method` prefix —
no `field`, `init`, or `static {}` member appears in the merged static boxes.
The three merged ordinary boxes do not use the prefix, so the ordinary path is
not part of this blocker.

## Ordered implementation

1. In the static member dispatch, when the member identifier is `method`,
   look ahead past newlines for `name (`; on match, adopt that name and let
   the existing `try_parse_method_or_field` produce the DirectMethod row. Any
   other shape keeps the current Field classification.
2. Positive focused tests: `static box` with `method`-prefixed members yields
   only DirectMethod rows, a Ready parent seal, and a Ready normal-root
   execution disposition.
3. Negative focused tests: a bare `method` field member keeps the existing
   `Outside(UnsupportedMemberKind)` classification; non-`method` members are
   unchanged.
4. Re-run the merged entry through the source-backed root consumer and record
   the next named terminal; this card's finish line is crossing the
   `StaticParentSourceSealMissing` boundary, not solving what follows it.
5. Update the owner README and this card with receipts.

## Acceptance

Focused quick-profile lib tests in one Cargo process with at most four build
jobs prove the modifier admission and the unchanged field path. The merged
entry no longer stops at `StaticParentSourceSealMissing`; whatever named
terminal it reaches next is recorded as the following bounded boundary.
`git diff --check` and the current-state pointer guard pass. No ordinary-box
parse change, tokenizer change, physical product, or D7 source-result work is
claimed here.

## Implementation checkpoint

`take_method_modifier_name` in `static_def/members.rs` adopts the following
`name (` head after a leading `method` identifier; the member loop in
`static_def/mod.rs` then routes the real name through the existing
`try_parse_method_or_field`, so the cursor commits one `DirectMethod` row and
no phantom `Field`. A `method` identifier without that head (bare member,
field assignment, or a method literally named `method`) keeps the existing
classification.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib static_box_source_tests
12 passed; 0 failed; 7936 filtered out; 536 existing warnings (baseline)
```

Route evidence: the rebuilt quick binary drives the merged entry
(723244 bytes, 131 top-level declarations) through `--backend mir` past the
previous `IntegrityInvalid(Surface(StaticParentSourceSealMissing))` terminal.
The next named boundary is

```text
[mir/callable-semantic-package/issue] ParameterContract {
  _error: UnsupportedDeclaredType { declaration: 436, parameter: 0 } }
```

—the parameter contract accepts only `i64` and `StringBox` declared types,
while merged static parents such as `BoxHelpers` declare `ArrayBox`/`MapBox`
parameters. That declared-type boundary is a separate bounded slice and is not
claimed by this card.
