---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer host installation seam.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1194-BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-DESIGN-001.md
---

# BUILD-FRONTEND-TOKENIZER-HOST-INSTALL-SEAM-001

## Result

Added a runtime-free host registry to `hakorune-frontend-parser`:

```text
host_registry_owner=crates/hakorune_frontend_parser/src/frontend_host.rs
default_host=NoopFrontendHost
runtime_dependency_added_to_frontend_parser=0
frontend_log_routes_through_host=1
frontend_env_alias_warnings_route_through_host=1
```

The main crate now adapts `RuntimeFrontendHost` to the frontend parser crate
host trait:

```text
main_runtime_adapter=RuntimeFrontendHost
adapter_owner=src/frontend_host.rs
install_seam=install_frontend_parser_host
existing_runtime_host_entry_installs_frontend_parser_host=1
```

## Boundary

This row prepares tokenizer movement only:

```text
NyashTokenizer_moved=0
tokenizer_impl_files_moved=0
tokenization_behavior_changed=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-PREFLIGHT-001
purpose=verify host seam and root facade shape before moving NyashTokenizer bundle
implementation_allowed=preflight_only
```
