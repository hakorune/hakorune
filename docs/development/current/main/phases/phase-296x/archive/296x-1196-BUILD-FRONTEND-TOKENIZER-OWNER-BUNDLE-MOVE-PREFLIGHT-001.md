---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer owner-bundle move preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1195-BUILD-FRONTEND-TOKENIZER-HOST-INSTALL-SEAM-001.md
---

# BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-PREFLIGHT-001

## Result

Direct owner-bundle move is still too early:

```text
direct_owner_bundle_move_allowed=0
reason=main_crate_runtime_host_install_not_guaranteed_for_direct_tokenizer_new
```

Current parser entry points create tokenizers directly:

```text
parser_tokenizer_new_call_sites=3
owner=src/parser/mod.rs
```

The existing host install seam is reachable through `src/frontend_host.rs`, but
a simple future `pub use hakorune_frontend_parser::tokenizer::NyashTokenizer`
would not install the main runtime host when users call
`crate::tokenizer::NyashTokenizer::new(...)` directly.

## Decision

Use a main-crate tokenizer facade wrapper during the owner-bundle move.

```text
selected_shape=main_crate_tokenizer_wrapper
wrapper_owner=src/tokenizer/mod.rs
inner_owner=crates/hakorune_frontend_parser/src/tokenizer/mod.rs
wrapper_new_installs_runtime_host=1
wrapper_delegates_tokenize=1
```

This keeps the public main-crate tokenizer entry point responsible for host
installation while the extracted crate remains runtime-free.

## Next

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-FACADE-WRAPPER-DESIGN-001
purpose=define the wrapper API before moving NyashTokenizer into the frontend parser crate
implementation_allowed=design_only
```

Non-goals:

```text
do_not_move_tokenizer_bundle_yet=1
do_not_reexport_NyashTokenizer_directly_from_main_crate=1
do_not_add_runtime_dependency_to_frontend_parser=1
```
