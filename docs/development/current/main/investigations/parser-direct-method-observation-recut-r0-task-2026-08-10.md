# PARSER-DIRECT-METHOD-OBSERVATION-RECUT-R0

Status: parked BoxShape Refactor Series; before Share source I0
Date: 2026-08-10

## Goal

Observe each selected direct method exactly once inside the parser transaction
and stop growing positional callback arguments, without merging the semantic
owners of declaration, body, Release, or future Share.

## Private structure

```text
AST + final parser seals
  -> one exact traversal
  -> PreparedDirectMethodObservationBatchV1<'ast>
       method source site
       declaration/header view
       body root
       ordered direct body items
       borrowed syntax
  -> deterministic projections
       handoff
       body envelope
       syntax lease
       Release catalog
       future Share catalog
```

`PreparedDirectMethodObservationBatchV1` is private unpublished staging. It is
not a Verified product, source authority, semantic catalog, or public Plan.
The same private batch builder serves both
`into_ast_and_resolver_source_handoff` and the body transaction. An already
built handoff is an output projection, never a batch input.

Bundle callback transport only:

```text
ParserDirectMethodObservationPartsV1<'ast> {
  handoff,
  body,
  syntax,
  release,
}
```

The callback consumes/destructures the parts once. Future products extend the
transport struct, not the positional callback signature.

## Acceptance

- one Box/method association traversal replaces
  `source_resolver_handoff.rs::build_resolver_source_handoff` /
  `collect_explicit_methods`, `body_source.rs::collect_body_rows`, and
  `collect_syntax_lease` reconstruction for these ingress APIs;
- existing declaration/body/syntax/Release products and parser provenance are
  unchanged;
- all existing body-source, owner, facts, function-carrier, and Release tests
  pass with equality-identical source identities/order;
- a structural guard proves only the private batch walks
  `iter_selected_declaration_order` for these ingress APIs;
- no AST borrow escapes the higher-ranked callback;
- no semantic product is merged and no accepted syntax changes;
- owner README/task receipt update in the same Refactor Series;
- all touched source files remain below 800 lines.

## Nonclaims

```text
Take/Share syntax
Home meaning
nested Release support
resolver target/Recipe/Builder/MIR
```
