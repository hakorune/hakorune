---
Status: closed; parser/source carrier landed; Home semantics and production 0
Date: 2026-08-09
Parent: `OWN-HOME-SYNTAX-D0`
Ceremony: T2 language/parser boundary
---

# OWN-HOME-RELEASE-SOURCE-I0

## Change

Implement exactly one contextual statement shape in both parsers:

```hako
release root
```

Use a dedicated syntax-only `ASTNode::Release { root, span }`; never lower it
to `FunctionCall("release")`.  The Rust one-shot body transaction additionally
issues one non-`Clone` `ParserReleaseStatementSourceCatalogV1` containing
parser provenance once and rows keyed by exact method source site plus direct
body ordinal.  Root spelling is lexical input; `Span` is diagnostic only.

Before adding the node, split the 796-line roundtrip decoder in one separate,
behavior-neutral BoxShape commit.  The Release implementation is the following
single BoxCount commit: Rust + Hako parsing, source catalog, normalized parity,
unsupported semantic classifications, focused tests, and same-slice reference
closeout.  Do not commit one-sided parser activation.

## Contract

```text
contextual:
  statement-head IDENT("release") HSPACE IDENT statement-end

HSPACE:
  zero or more spaces/tabs; comment trivia is outside I0

ordinary/non-selected:
  release(...), release = x, local release = x, obj.release(),
  Build.release(), release\nroot

committed syntax error:
  release root.field / root[index] / root() / root + x / root, x / me
  stable tag = parser/release_exact_root_required
```

`release` remains `IDENT`; no `TokenType::RELEASE`.  The Hako classifier owns a
private horizontal-trivia scanner and must not use its newline-skipping
`skip_ws`.  Roundtrip v2 preserves `{kind/type: Release, root}`; legacy JoinIR
emits explicit Unsupported and never decodes or executes Release.  Hako
Program(JSON) is descriptive parity evidence only; `source_carrier_v1` stays
disconnected.

The parser catalog is built once inside
`ParserResolverBodyTransactionV1::with_direct_method_syntax`; extend that sole
callback rather than adding another parse/AST entry.  Empty and multiple
direct rows are valid coverage.  Any nested Release in the selected I0 body is
an issuance error, never an omitted event.

Release remains explicitly unsupported in semantic shadow resolution, normal
Script production admission, and the bounded AST wire oracle.  This row owns
no `BindingRef`, `HomeRoot`, Home state, FunctionOwner, Recipe, Builder, MIR,
runtime, fallback, or production route.

## Done

- Rust AST/source tests prove dedicated syntax, exact provenance/method/body
  identity, ordinary-name preservation, empty/multiple coverage, and nested
  rejection.
- Rust AST leaf utilities preserve Release; semantic/physical classifiers
  reject it deliberately rather than treating it as an expression.
- Roundtrip v2 preserves Release; legacy JSON returns Unsupported.
- Canonical and Compat2025 grammar rows normalize both parsers to
  `{kind:"ReleaseStatement", value:"root", children:[]}`.  Ordinary shapes
  use `parser/release_contextual_not_selected`; malformed committed shapes use
  `parser/release_exact_root_required`.
- `cargo build --release --bin hakorune`, focused parser/AST/roundtrip tests,
  `tools/checks/language_v1_grammar_contract_substrate_guard.sh`, and
  `tools/checks/current_state_pointer_guard.sh` are green.
- The implementation commit updates language ownership/EBNF/status/quick
  references plus parser and AST-JSON owner READMEs before marking the syntax
  parser-live.  Home capability/Flow/DropPlan/execution remain 0.

## Closeout receipt

- The prerequisite decoder split landed separately as `6ec011e1ba`.
- Rust and Hako recognize the same dedicated exact-root statement and retain
  ordinary call/name spellings. `release me`, projections, calls, and trailing
  expressions fail with the stable exact-root tag after syntax commitment; a
  line terminator prevents commitment.
- The Rust one-shot body transaction issues one non-`Clone` release-source
  catalog. Direct rows preserve parser provenance, exact method source site,
  direct body ordinal, and lexical root; nested rows fail rather than vanish.
- Roundtrip v2 preserves `Release`; legacy JSON and every semantic/physical
  consumer remain explicitly unsupported.
- This closes syntax/source carriage only. `BindingRef`, Home capability,
  Home Flow, DropPlan, lowering, and runtime behavior remain zero.

## Stop

Stop as `NoSafeSlice` instead of implementing if the slice requires an
ordinary-call desugar, raw text/JSON repair, inventory ordinal identity,
post-hoc AST reconstruction, nested-event omission, Hako generic `skip_ws`,
one-parser activation, semantic Home defaults, or any source file at/above
800 lines.
