---
Status: execution-task
Date: 2026-08-01
Decision: Accept-corrected
Ceremony: T2
Row: RAW-SCRIPT-RECORD-SCHEMA-ADMISSION0-I0-R0
---

# Record schema admission: atomic execution task

## Objective

Replace the selected Script Record schema compatibility edge with one
source-only declaration-facts product, one immutable schema loan, and one
RootLower installation of that same product.

```text
CatalogSeal
-> PreparedNormalProgramDeclarationFactsV1::collect(Program) once
-> immutable RecordSchemaDemandViewV1 borrow
-> ScriptSemanticSeal
-> CatalogInstall
-> RootLower consumes and installs the same prepared facts once
```

## Selected closure

```text
FullyExplicitRecordLiteralV1 ::= RecordLiteral {
  record_type_name = known non-generic effective record R,
  fields = every declared field of R exactly once,
  every field value = ScriptLexicalCore expression,
}
```

The same record-schema family also transfers its source producer:

```text
BoxDeclaration { is_record: true, is_static: false, is_sync: false }
-> Transferred(ProgramRecordDeclaration)
-> existing runtime lifecycle retained once
```

## Owner contract

`PreparedNormalProgramDeclarationFactsV1` remains the only schema owner. Its
single `collect_operations` traversal records the effective record operation
per name; the last source operation wins, exactly as later `install_into` does.

The private schema view implements a **neutral resolved-semantics admission
trait/vocabulary**. The builder-owned view type must not appear in a public
resolved-semantics input, because that would reverse the dependency direction.
The neutral interface exposes only a positive admission receipt:

```text
RecordSchemaDemandViewV1::admit_fully_explicit_literal(...)
-> Option<FullyExplicitRecordLiteralAdmissionV1 { explicit_field_count }>
```

It exposes neither default ASTs, record clones, type contracts, nor diagnostic
details. `None` means Deferred, never a ScriptSemanticSeal user diagnostic.

Existing record lowering remains the only operational owner of schema
preflight, field contract checks, defaults, and `RecordValuePublish`.

## Atomic deletion and replacement

Delete only these selected reachabilities:

```text
record declaration -> Deferred -> bare script_root()
FullyExplicitRecordLiteralV1 -> Deferred -> bare script_root()
selected RootLower -> collect declaration facts
```

Replace them with:

```text
record declaration -> typed transfer + retained existing terminal
fully explicit record -> Complete + RecordFieldValue(i) receipts
selected lifecycle -> collect once + immutable loan
selected RootLower -> move/install once
```

Raw/reference routes keep their existing collection/install timing.

## Required implementation boundaries

- `program_declaration_facts.rs`: effective-record index and closure-scoped
  immutable view; the product is still move-only for installation.
- `resolved_semantics`: neutral Script schema-admission trait/vocabulary,
  positive receipt/coverage, and RecordLiteral profile gate before child
  traversal. It must not import Builder types.
- `raw_expression_dispatch`: use a structured child scope only when the sealed
  positive receipt exists; keep the current direct record route otherwise.
- selected lifecycle/root lowering: collect/borrow/move handoff, no second
  collector. Do not modify `program_root_work_plan.rs`.

## Focused evidence

1. record declaration plus fully explicit literal is Complete.
2. explicit source order differs from declaration/publication order as today.
3. fully explicit use of a record with defaults does not demand a default.
4. omitted-default, unknown/generic/invalid fields, RecordUpdate, and `New`
   remain Deferred and preserve RootLower diagnostics.
5. missing explicit field-value stops before later field/default and publishes
   no record value.
6. legacy/selected parity includes MIR, record contract instructions, and
   metadata; fresh compiler reuse succeeds after failure.
7. effective schema uses the final duplicate declaration.
8. raw/reference behavior stays unchanged.

## Hard stops

- second Program scan, clone, independent schema collector, or mutable Builder
  / `CompilationContext` read from semantic traversal;
- a builder-owned schema view type in the resolved-semantics public API or any
  other `resolved_semantics -> builder` dependency;
- default AST exposed as a RecordLiteral child receipt;
- any record diagnostic moved into ScriptSemanticSeal;
- Complete-to-Deferred fallback or retry;
- unconditional structured port for every RecordLiteral;
- RecordUpdate or constructor `New` activation;
- a touched source/check file reaching 800 lines.
