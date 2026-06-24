---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer owner-bundle design.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1193-BUILD-FRONTEND-TOKENIZER-NEXT-MOVE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-DESIGN-001

## Decision

Move tokenizer implementation as an owner bundle, not as individual impl files.

```text
selected_shape=NyashTokenizer_owner_bundle
owner_type=NyashTokenizer
move_together=mod,cursor,engine,env,log,lex_ident,lex_number,lex_string,whitespace
kinds_already_moved=1
single_impl_file_move_allowed=0
```

Reason:

```text
Rust_inherent_impl_requires_defining_type_owner=1
remaining_tokenizer_files_are_NyashTokenizer_impls=1
```

## Host Boundary Requirement

The frontend parser crate must not depend on the main crate or runtime.

```text
frontend_parser_depends_on_runtime=0
frontend_parser_depends_on_main_crate=0
```

Logging and alias-warning behavior must remain host-routed:

```text
tokenizer_log_calls_host_boundary=1
tokenizer_env_alias_warnings_call_host_boundary=1
default_host=NoopFrontendHost
main_crate_runtime_host_adapter=RuntimeFrontendHost
```

Implementation shape:

```text
frontend_parser_crate_owns_host_trait=1
frontend_parser_crate_exposes_install_host_adapter=1
main_crate_frontend_host_installs_runtime_adapter=1
```

The default `NoopFrontendHost` is acceptable only before the main crate installs
its runtime adapter. The tokenizer bundle move row must wire the main crate
facade so normal parser/tokenizer entry points preserve the current runtime
logging sink.

## Bundle Move Shape

Allowed in the implementation row:

```text
move_NyashTokenizer_struct=1
move_tokenizer_impl_modules=1
turn_src_tokenizer_into_facade=1
add_host_adapter_installation=1
```

Forbidden:

```text
drop_tok_trace_logging_silently=0
drop_grammar_diff_warnings_silently=0
add_runtime_dependency_to_frontend_parser=0
change_tokenization_behavior=0
move_parser_files=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-HOST-INSTALL-SEAM-001
purpose=add the host installation seam before moving NyashTokenizer owner bundle
implementation_allowed=1
```

Rationale:

```text
host_install_seam_first=1
owner_bundle_move_after_host_seam=1
```

The host seam is the smaller safe step. Moving the full tokenizer bundle before
host installation would risk silently degrading tokenizer debug/warn behavior.
