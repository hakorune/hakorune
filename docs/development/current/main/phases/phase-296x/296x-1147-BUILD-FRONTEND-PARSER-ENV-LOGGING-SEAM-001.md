Status: Done
Date: 2026-06-18
Scope: isolate parser config/env and runtime logging references behind parser facades
Related:
  - docs/development/current/main/phases/phase-296x/296x-1146-BUILD-FRONTEND-AST-PASSIVE-SEAM-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-PARSER-ENV-LOGGING-SEAM-001

## Change

```text
output_contract=build-frontend-parser-env-logging-seam-v0

new_module=src/parser/env.rs
new_module=src/parser/log.rs

parser_config_env_direct_refs_outside_facade=0
parser_runtime_logger_direct_refs_outside_facade=0
ast_external_refs_outside_literal_box_bridge=0

behavior_changed=0
parser_env_policy_changed=0
parser_logging_behavior_changed=0
```

Parser modules now read configuration through `crate::parser::env` and write
logs through `crate::parser::log`. The facades still delegate to the existing
main-crate config/runtime owners, but parser implementation files no longer
carry those cross-layer imports directly.

This keeps the parser extraction boundary narrow: future frontend crate work
can replace the facade implementation without changing parser call sites.

## Proof

```bash
cargo check -q
rg -n "crate::runtime::get_global_ring0\\(\\)|crate::config::env::" \
  src/parser src/ast -g '!src/parser/env.rs' -g '!src/parser/log.rs'
rg -n "crate::(parser|tokenizer|config|runtime|backend|mir|box_trait|boxes)" \
  src/ast -g '!src/ast/literal_box_bridge.rs'
```

## Result

```text
cargo_check_default_green=1
parser_direct_config_runtime_refs_outside_facade=0
ast_passive_external_refs=0
selected_next_task=BUILD-FRONTEND-AST-PASSIVE-CRATE-SCAFFOLD-001
summary=ok
```

## Stop Lines

```text
do_not_move_parser_to_new_crate=1
do_not_change_parser_env_policy=1
do_not_change_logging_behavior=1
do_not_change_language_acceptance=1
```
