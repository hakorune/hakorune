---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer crate extraction preflight v3.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1182-BUILD-FRONTEND-CLI-VERBOSE-LOCAL-SEAM-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-003

## Result

The parser/tokenizer env/log cleanup is materially better:

```text
parser_tokenizer_rust_file_count=93
parser_tokenizer_total_lines=15280
parser_tokenizer_direct_config_refs=0
parser_tokenizer_direct_runtime_refs=0
parser_tokenizer_direct_grammar_refs=0
parser_tokenizer_direct_syntax_refs=0
parser_tokenizer_direct_semantics_refs=0
```

But direct crate extraction is still blocked:

```text
frontend_env_refs_from_parser_tokenizer=4
frontend_log_refs_from_parser_tokenizer=5
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
parser_ast_tokenizer_parser_crate_path_refs=463
direct_parser_tokenizer_crate_extraction_allowed=0
```

The 463 root-path references are mostly compatible with a future frontend crate
that owns `ast`, `parser`, and `tokenizer` modules together. The real blocker is
the host seam: frontend env/log still route to main-crate runtime behavior.

## Decision

Do not scaffold the parser/tokenizer crate yet. First define the crate-ready
frontend host boundary.

```text
selected_next_task=BUILD-FRONTEND-HOST-ADAPTER-DESIGN-001
purpose=define frontend env/log host boundary for future parser-tokenizer crate extraction
implementation_allowed=design_only
```

Non-goals:

```text
do_not_mass_rewrite_crate_ast_parser_tokenizer_paths=1
do_not_replace_runtime_logger_behavior=1
do_not_extract_parser_tokenizer_crate=1
```
