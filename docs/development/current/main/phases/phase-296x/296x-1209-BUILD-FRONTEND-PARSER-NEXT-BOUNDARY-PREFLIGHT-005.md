---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1208-BUILD-FRONTEND-PARSER-EXPR-CURSOR-OWNER-BUNDLE-MOVE-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-005

## Result

The next safe parser-side passive split is parser metadata:

```text
selected_family=parser_metadata
selected_type=ParserMetadata
selected_destination=crates/hakorune_frontend_parser/src/parser/metadata.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
ParserMetadata_is_passive_data=1
RuneAttr_owner=hakorune_frontend_ast
metadata_producer_stays_main_crate=1
```

Not selected:

```text
NyashParser
reason=active_parser_owner_bundle

rune_parser_impls
reason=active_NyashParser_impls_and_should_not_move_with_passive_metadata
```

## Decision

Move `ParserMetadata` into `hakorune-frontend-parser` and keep the historical
`crate::parser::ParserMetadata` path as a compatibility re-export.

```text
selected_next_task=BUILD-FRONTEND-PARSER-METADATA-PASSIVE-SPLIT-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_rune_parser_impls=1
do_not_move_NyashParser=1
do_not_change_metadata_contents=1
```
