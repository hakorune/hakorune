---
Status: SSOT
Decision: accepted
Date: 2026-06-18
Scope: Build-time reduction through crate split planning.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1076-BUILD-CRATE-SPLIT-PLAN-001.md
---

# Build Crate Split Plan SSOT

## Problem

The main `nyash-rust` crate is too large to compile efficiently.

Observed audit:

```text
main_crate_lines=469k
main_crate_files=2370
total_build_time_sec=41.6
main_crate_compile_time_sec=33.8
main_crate_compile_time_percent=81
src_mir_lines=278k
src_mir_percent_of_main_crate=59
```

One giant crate limits parallelism and forces unrelated compiler/runtime edits
through the same compile unit.

## Decision

Adopt a staged crate split, but do not start with the deepest lowering code.

The first goal is build-time leverage with low architectural risk:

```text
stage_0=mir_core_growth
stage_1=hakorune_mir_plans
stage_2=hakorune_backend
stage_3=hakorune_frontend
stage_4=box_core_config
stage_5=hakorune_lowering
stage_6=runtime_boxes
```

## Ranking

| Rank | Crate | Approx Size | Effect | Effort | Risk | Decision |
|---:|---|---:|---|---|---|---|
| 1 | `hakorune-mir-plans` | 40-45k lines | high | medium | low | first real split |
| 2 | grow `mir_core` | 1.2k -> larger | medium | small | low | prerequisite |
| 3 | `hakorune-backend` | 18k lines | medium | medium | low | after plans |
| 4 | `hakorune-frontend` | 17.5k lines | medium | medium | medium | after backend |
| 5 | `box-core + config` | 6k lines | medium | medium | medium | only after boundary audit |
| 6 | `hakorune-lowering` | 82k lines | high | large | high | last compiler split |
| 7 | `runtime + boxes` | 46k lines | medium | large | high | last overall split |

## Stage 0: mir_core Growth

Purpose:

```text
move_stable_mir_data_types=1
move_plan_independent_value_types=1
move_report_contract_types_when_dependency_free=1
```

Allowed:

```text
MirType / ValueId / BlockId style shared primitives
small plan-neutral enums
serde-compatible metadata structs with no builder/backend dependency
```

Forbidden:

```text
builder control-flow logic
lowering logic
runtime boxes
backend emitters
policy decisions with active owners
```

## Stage 1: hakorune-mir-plans

Purpose:

```text
extract plan vocabularies and passive plan data from src/mir
keep lowering/building behavior in main crate at first
```

Candidate families:

```text
object_storage_plan
local_fastpath_fact
map_repr_plan passive data
route plan data models after dependency audit
plan report vocabularies
```

Non-goals:

```text
do not move control_flow lowering yet
do not move MIRBuilder yet
do not move runtime Box implementations
do not change behavior while splitting
```

## Guardrail

Each split row must be BoxShape-only:

```text
behavior_changed=0
public_api_changed_only_for_crate_boundary=1
cargo_build_release_bin_hakorune_green=1
quick_smoke_green_when_slice_ready=1
```

No language acceptance shape, optimizer rule, or runtime behavior change may be
mixed into the crate split commits.

## Next Task

```text
latest_done=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-004
next_task=BUILD-FRONTEND-PARSER-EXPR-CURSOR-OWNER-BUNDLE-MOVE-001
purpose=move ExprParserWithCursor and helper modules into hakorune-frontend-parser behind compatibility facade
implementation_allowed=1
default_feature_change_allowed=0
full_no_default_plugin_stub_fix_allowed=0
```

## Frontend Parser Next Boundary Preflight 004 Result

```text
selected_family=parser_expr_cursor
selected_owner_bundle=ExprParserWithCursor,precedence,primary,record
selected_destination=crates/hakorune_frontend_parser/src/parser/expr_cursor.rs
NyashParser_owner_required=0
selected_next_task=BUILD-FRONTEND-PARSER-EXPR-CURSOR-OWNER-BUNDLE-MOVE-001
```

## Frontend Parser Cursor Passive Split Result

```text
new_owner=crates/hakorune_frontend_parser/src/parser/cursor.rs
compat_facade=src/parser/cursor.rs
types_moved=TokenCursor,NewlineMode
newline_policy_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-004
```

## Frontend Parser Next Boundary Preflight 003 Result

```text
selected_family=parser_cursor
selected_types=TokenCursor,NewlineMode
selected_destination=crates/hakorune_frontend_parser/src/parser/cursor.rs
expr_cursor_deferred=1
selected_next_task=BUILD-FRONTEND-PARSER-CURSOR-PASSIVE-SPLIT-001
```

## Frontend Parser Parse Error Passive Split Result

```text
new_owner=crates/hakorune_frontend_parser/src/parser/error.rs
compat_facade=src/parser/mod.rs
type_moved=ParseError
error_message_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-003
```

## Frontend Parser Next Boundary Preflight 002 Result

```text
selected_family=parser_error
selected_type=ParseError
selected_destination=crates/hakorune_frontend_parser/src/parser/error.rs
TokenCursor_deferred=1
ParserMetadata_deferred=1
selected_next_task=BUILD-FRONTEND-PARSER-PARSE-ERROR-PASSIVE-SPLIT-001
```

## Frontend Parser Build Config Passive Split Result

```text
new_owner=crates/hakorune_frontend_parser/src/parser/build_config.rs
compat_facade=src/parser/mod.rs
types_moved=BuildMode,ParserBuildConfig
active_build_cfg_impls_moved=0
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-002
```

## Frontend Parser Next Boundary Preflight Result

```text
selected_family=parser_build_config
selected_types=BuildMode,ParserBuildConfig
selected_destination=crates/hakorune_frontend_parser/src/parser/build_config.rs
ParserMetadata_deferred=1
selected_next_task=BUILD-FRONTEND-PARSER-BUILD-CONFIG-PASSIVE-SPLIT-001
```

## Frontend Build Gate Report Passive Split Result

```text
new_owner=crates/hakorune_frontend_parser/src/parser/build_cfg.rs
compat_facade=src/parser/build_cfg.rs
type_moved=BuildGateExplainReport
active_build_cfg_impls_moved=0
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-001
```

## Frontend Post-Tokenizer Move Preflight Result

```text
selected_family=build_gate_explain_report
selected_type=BuildGateExplainReport
selected_destination=crates/hakorune_frontend_parser/src/parser/build_cfg.rs
active_parser_impl_move_allowed=0
selected_next_task=BUILD-FRONTEND-BUILD-GATE-REPORT-PASSIVE-SPLIT-001
```

## Frontend Tokenizer Owner Bundle Move Result

```text
new_owner=crates/hakorune_frontend_parser/src/tokenizer.rs
moved_files=cursor,engine,env,log,lex_ident,lex_number,lex_string,whitespace,kinds
main_facade=src/tokenizer/mod.rs
main_facade_shape=wrapper
frontend_parser_depends_on_runtime=0
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-POST-TOKENIZER-MOVE-PREFLIGHT-001
```

## Frontend Tokenizer Facade Wrapper Design Result

```text
main_facade_type=src/tokenizer/mod.rs::NyashTokenizer
inner_type=hakorune_frontend_parser::tokenizer::NyashTokenizer
wrapper_new_installs_runtime_host=1
wrapper_delegates_tokenize=1
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-001
```

## Frontend Tokenizer Owner Bundle Move Preflight Result

```text
direct_owner_bundle_move_allowed=0
selected_shape=main_crate_tokenizer_wrapper
wrapper_owner=src/tokenizer/mod.rs
inner_owner=crates/hakorune_frontend_parser/src/tokenizer/mod.rs
wrapper_new_installs_runtime_host=1
selected_next_task=BUILD-FRONTEND-TOKENIZER-FACADE-WRAPPER-DESIGN-001
```

## Frontend Tokenizer Host Install Seam Result

```text
host_registry_owner=crates/hakorune_frontend_parser/src/frontend_host.rs
runtime_dependency_added_to_frontend_parser=0
main_runtime_adapter=RuntimeFrontendHost
existing_runtime_host_entry_installs_frontend_parser_host=1
NyashTokenizer_moved=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-PREFLIGHT-001
```

## Frontend Tokenizer Owner Bundle Design Result

```text
selected_shape=NyashTokenizer_owner_bundle
move_together=mod,cursor,engine,env,log,lex_ident,lex_number,lex_string,whitespace
host_install_seam_first=1
frontend_parser_depends_on_runtime=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-HOST-INSTALL-SEAM-001
```

## Frontend Tokenizer Next Move Preflight Result

```text
remaining_tokenizer_impl_files=cursor,engine,lex_ident,lex_number,lex_string,whitespace
remaining_tokenizer_impl_owner=NyashTokenizer
single_impl_file_move_allowed=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-DESIGN-001
```

## Frontend Tokenizer Kinds Passive Split Result

```text
new_owner=crates/hakorune_frontend_parser/src/tokenizer/kinds.rs
compat_facade=src/tokenizer/kinds.rs
types_moved=TokenType,Token,TokenizeError
tokenizer_engine_moved=0
NyashTokenizer_moved=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-NEXT-MOVE-PREFLIGHT-001
```

## Frontend Parser/Tokenizer File Move Preflight Result

```text
selected_family=tokenizer_kinds
selected_source=src/tokenizer/kinds.rs
selected_destination=crates/hakorune_frontend_parser/src/tokenizer/kinds.rs
selected_types=TokenType,Token,TokenizeError
direct_crate_refs=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-KINDS-PASSIVE-SPLIT-001
```

## Frontend Parser/Tokenizer Crate Scaffold Result

```text
crate_name=hakorune-frontend-parser
crate_path=crates/hakorune_frontend_parser
root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host,grammar
root_macro=must_advance
parser_files_moved=0
tokenizer_files_moved=0
runtime_dependency_added_to_frontend_parser=0
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-FILE-MOVE-PREFLIGHT-001
```

## Frontend Parser/Tokenizer Crate Scaffold Design Result

```text
crate_name=hakorune-frontend-parser
crate_path=crates/hakorune_frontend_parser
root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host
root_macro=must_advance
file_move_allowed=0
rewrite_parser_tokenizer_imports_allowed=0
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-001
```

## Frontend Parser/Tokenizer Crate Preflight v4 Result

```text
parser_tokenizer_direct_config_refs=0
parser_tokenizer_direct_runtime_refs=0
host_runtime_refs_owner=src/frontend_host.rs
crate_parser_refs=275
crate_ast_refs=121
crate_tokenizer_refs=67
future_crate_root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-DESIGN-001
```

## Frontend Host Boundary Wiring Result

```text
adapter_owner=src/frontend_host.rs
runtime_adapter=RuntimeFrontendHost
frontend_env_direct_runtime_refs_after=0
frontend_log_direct_runtime_refs_after=0
frontend_host_runtime_refs=2
parser_tokenizer_direct_config_refs_after=0
parser_tokenizer_direct_runtime_refs_after=0
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-004
```

## Frontend Host Boundary Wiring Preflight Result

```text
selected_shape=RuntimeFrontendHost_adapter
adapter_owner=src/frontend_host.rs
frontend_env_direct_runtime_ref_after_target=0
frontend_log_direct_runtime_refs_after_target=0
runtime_logger_behavior_preserved=1
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-WIRING-001
```

## Frontend Host Boundary Vocabulary Result

```text
new_owner=src/frontend_host.rs
new_type=FrontendLogLevel
new_trait=FrontendHostBoundary
new_default=NoopFrontendHost
frontend_env_wiring_changed=0
frontend_log_wiring_changed=0
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-WIRING-PREFLIGHT-001
```

## Frontend Host Adapter Design Result

```text
selected_shape=FrontendHostBoundary
host_owns_logging=1
host_owns_alias_warning_sink=1
frontend_owns_feature_parsing=1
parser_struct_threading_now=0
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-VOCAB-001
```

## Frontend Parser/Tokenizer Crate Preflight v3 Result

```text
parser_tokenizer_rust_file_count=93
parser_tokenizer_total_lines=15280
parser_tokenizer_direct_config_refs=0
parser_tokenizer_direct_runtime_refs=0
frontend_env_refs_from_parser_tokenizer=4
frontend_log_refs_from_parser_tokenizer=5
parser_ast_tokenizer_parser_crate_path_refs=463
direct_parser_tokenizer_crate_extraction_allowed=0
selected_next_task=BUILD-FRONTEND-HOST-ADAPTER-DESIGN-001
```

## Frontend CLI Verbose Local Seam Result

```text
parser_tokenizer_direct_config_refs_after=0
parser_env_cli_verbose_config_delegate_after=0
parser_tokenizer_direct_runtime_refs_after=0
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-003
```

## Frontend Host Seam Preflight Result

```text
parser_env_remaining_config_ref=cli_verbose_enabled
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
selected_next_shape=cli_verbose_local_env_seam
host_adapter_introduction=defer
selected_next_task=BUILD-FRONTEND-CLI-VERBOSE-LOCAL-SEAM-001
```

## Frontend Logging Shared Facade Result

```text
new_owner=src/frontend_log.rs
parser_log_facade_runtime_refs_after=0
tokenizer_log_facade_runtime_refs_after=0
frontend_log_runtime_refs=3
parser_tokenizer_direct_runtime_refs_after=0
selected_next_task=BUILD-FRONTEND-HOST-SEAM-PREFLIGHT-001
```

## Frontend Logging Facade Preflight Result

```text
selected_shape=shared_frontend_log_facade
new_owner=src/frontend_log.rs
parser_tokenizer_direct_runtime_refs_outside_log_facades=0
call_site_rewrite_required=0
selected_next_task=BUILD-FRONTEND-LOGGING-SHARED-FACADE-001
```

## Frontend Stage-3 Env Shared Facade Result

```text
new_owner=src/frontend_env.rs
parser_stage3_config_delegate_after=0
tokenizer_stage3_config_delegate_after=0
parser_env_facade_main_config_refs_after=1
tokenizer_env_facade_main_config_refs_after=0
remaining_parser_config_ref=cli_verbose_enabled
selected_next_task=BUILD-FRONTEND-LOGGING-FACADE-PREFLIGHT-001
```

## Frontend Parser Stage-3 Env Seam Preflight Result

```text
selected_shape=shared_frontend_env_stage3_facade
new_owner=src/frontend_env.rs
parser_tokenizer_duplicate_stage3_logic=0
stage3_alias_warning_behavior_preserved=1
runtime_logger_behavior_preserved=1
selected_next_task=BUILD-FRONTEND-STAGE3-ENV-SHARED-FACADE-001
```

## Frontend Parser Standalone Env Simple Flags Result

```text
parser_env_facade_main_config_refs_before=11
parser_env_facade_main_config_refs_after=5
tokenizer_env_facade_main_config_refs_before=7
tokenizer_env_facade_main_config_refs_after=1
stage3_alias_warning_behavior_preserved=1
runtime_logger_behavior_preserved=1
cargo_check_default_green=1
selected_next_task=BUILD-FRONTEND-PARSER-STAGE3-ENV-SEAM-PREFLIGHT-001
```

## Frontend Parser Env/Log Abstraction Preflight v2 Result

```text
parser_direct_std_env_reads_outside_facade=0
parser_env_facade_main_config_refs=11
tokenizer_env_facade_main_config_refs=7
parser_log_facade_runtime_refs=3
tokenizer_log_facade_runtime_refs=2
selected_shape=standalone_facade_simple_flags_first
trait_backed_adapter=defer
runtime_logger_adapter=defer
selected_next_task=BUILD-FRONTEND-PARSER-STANDALONE-ENV-SIMPLE-FLAGS-001
```

## Frontend Parser Direct Env Seam Result

```text
parser_env_facade=src/parser/env.rs
parser_direct_std_env_reads_outside_facade_after=0
parser_env_facade_direct_read_count=9
env_default_changed=0
logging_behavior_changed=0
cargo_check_default_green=1
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-002
```

## Frontend Parser Env/Log Abstraction Preflight Result

```text
parser_env_facade=src/parser/env.rs
parser_log_facade=src/parser/log.rs
tokenizer_env_facade=src/tokenizer/env.rs
tokenizer_log_facade=src/tokenizer/log.rs
parser_direct_std_env_reads=present
parser_env_facade_complete=0
env_log_abstraction_allowed=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-DIRECT-ENV-SEAM-001
```

## Frontend Parser/Tokenizer Crate Preflight v2 Result

```text
parser_tokenizer_rust_file_count=92
parser_tokenizer_total_lines=15144
parser_direct_grammar_refs=0
parser_direct_sugar_refs=0
parser_direct_prelude_refs=0
parser_ast_facade_refs=121
parser_tokenizer_refs=67
parser_internal_crate_path_refs=259
parser_tokenizer_env_log_facade_refs=present
direct_parser_tokenizer_crate_extraction_allowed=0
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-001
```

## Frontend Parser Prelude Consumer Import Switch Result

```text
parser_direct_result_option_prelude_refs_before=1
parser_direct_result_option_prelude_refs_after=0
consumer_import_target=hakorune_frontend_ast::result_option_prelude
cargo_check_default_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-002
```

## Frontend Result/Option Prelude Passive Split Result

```text
new_owner=crates/hakorune_frontend_ast/src/result_option_prelude.rs
compat_facade=src/semantics/result_option_prelude.rs
compat_import_path_preserved=1
cargo_check_default_green=1
cargo_test_frontend_ast_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-PRELUDE-CONSUMER-IMPORT-SWITCH-001
```

## Frontend Sugar Consumer Import Switch Result

```text
parser_direct_syntax_sugar_refs_before=2
parser_direct_syntax_sugar_refs_after=0
consumer_import_target=hakorune_frontend_grammar::sugar_config
cargo_check_default_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-RESULT-OPTION-PRELUDE-PASSIVE-SPLIT-001
```

## Frontend Sugar Config Passive Split Result

```text
new_owner=crates/hakorune_frontend_grammar/src/sugar_config.rs
compat_facade=src/syntax/sugar_config.rs
compat_import_path_preserved=1
cargo_check_default_green=1
cargo_test_frontend_grammar_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-SUGAR-CONSUMER-IMPORT-SWITCH-001
```

## Frontend Parser Syntax/Prelude Seam Preflight Result

```text
syntax_sugar_config_lines=95
result_option_prelude_lines=39
parser_direct_syntax_sugar_refs=2
parser_direct_result_option_prelude_refs=1
sugar_config_owner=hakorune-frontend-grammar
result_option_prelude_owner=hakorune-frontend-ast
behavior_changed=0
selected_next_task=BUILD-FRONTEND-SUGAR-CONFIG-PASSIVE-SPLIT-001
```

## Frontend Grammar Consumer Import Switch Result

```text
parser_tokenizer_crate_grammar_imports_before=6
parser_tokenizer_crate_grammar_imports_after=0
consumer_import_target=hakorune_frontend_grammar::engine
cargo_check_default_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-SYNTAX-PRELUDE-SEAM-PREFLIGHT-001
```

## Frontend Grammar Crate Scaffold Result

```text
new_crate=hakorune-frontend-grammar
new_engine_owner=crates/hakorune_frontend_grammar/src/engine.rs
new_generated_owner=crates/hakorune_frontend_grammar/src/generated.rs
compat_facade=src/grammar/mod.rs
src_grammar_facade_lines=9
build_rs_generated_path=crates/hakorune_frontend_grammar/src/generated.rs
cargo_check_default_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-GRAMMAR-CONSUMER-IMPORT-SWITCH-001
```

## Frontend Grammar Engine Seam Preflight Result

```text
grammar_rust_file_count=3
grammar_total_lines=168
main_crate_dependency_count=0
parser_grammar_engine_callsite_count=5
tokenizer_grammar_engine_callsite_count=1
grammar_crate_scaffold_selected=1
build_rs_generated_path_update_required=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-GRAMMAR-CRATE-SCAFFOLD-001
```

## Frontend Tokenizer Env/Logging Seam Result

```text
tokenizer_env_facade=src/tokenizer/env.rs
tokenizer_log_facade=src/tokenizer/log.rs
tokenizer_direct_config_env_refs_after=0
tokenizer_direct_runtime_log_refs_after=0
tokenizer_direct_grammar_engine_refs=1
parser_direct_grammar_engine_refs=5
cargo_check_default_green=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-GRAMMAR-ENGINE-SEAM-PREFLIGHT-001
```

## Frontend Parser Crate Preflight Result

```text
parser_tokenizer_rust_file_count=90
parser_tokenizer_total_lines=15091
direct_parser_crate_extraction_allowed=0
tokenizer_direct_config_env_refs=present
tokenizer_direct_runtime_log_refs=present
tokenizer_direct_grammar_engine_refs=present
parser_direct_grammar_engine_refs=present
parser_direct_syntax_sugar_refs=present
parser_direct_result_option_prelude_refs=present
behavior_changed=0
selected_next_task=BUILD-FRONTEND-TOKENIZER-ENV-LOGGING-SEAM-001
```

## Frontend AST Split Closeout Result

```text
new_crate=hakorune-frontend-ast
src_ast_mod_rs_lines=11
src_ast_literal_box_bridge_rs_lines=50
src_ast_facade_file_count=2
frontend_ast_crate_main_crate_refs=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-CRATE-PREFLIGHT-001
```

## Frontend AST Recursive Graph Passive Split Result

```text
moved_types=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
moved_wrappers=AssignStmt,ReturnStmt,IfStmt,BinaryExpr,CallExpr,MethodCallExpr
moved_inherent_utils=span,node_type,info,classification,traversal,analysis
new_owner=crates/hakorune_frontend_ast/src/ast_node.rs
new_owner=crates/hakorune_frontend_ast/src/node_wrappers.rs
new_owner=crates/hakorune_frontend_ast/src/utils/**
src_ast_facade_file_count=2
frontend_ast_main_crate_refs=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-SPLIT-CLOSEOUT-001
```

## Frontend AST Recursive Graph Preflight Result

```text
src_ast_mod_rs_lines=550
src_ast_nodes_rs_lines=263
src_ast_utils_impl_lines=956
remaining_graph_uses_main_crate_runtime=0
remaining_graph_uses_main_crate_parser=0
remaining_graph_uses_main_crate_mir=0
selected_bundle=ast_recursive_graph_with_methods
move_types=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
move_wrappers=AssignStmt,ReturnStmt,IfStmt,BinaryExpr,CallExpr,MethodCallExpr
move_inherent_utils=span,node_type,info,classification,traversal,analysis
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PASSIVE-SPLIT-001
```

## Frontend AST FieldDecl Boundary Design Result

```text
fielddecl_standalone_split_selected=0
generic_field_signature_split_selected=0
reason=FieldDecl.default_value carries ASTNode
recursive_graph_bundle=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PREFLIGHT-001
```

## Frontend AST Simple Decls Passive Split Result

```text
moved_types=ParamDecl,DelegateExposeDecl,DelegateDecl,TransitionDecl,ContractKind
new_owner=crates/hakorune_frontend_ast/src/decls.rs
compat_reexport=src/ast/decls.rs
deferred_types=CatchClause,FieldDecl,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-FIELD-DECL-BOUNDARY-DESIGN-001
```

## Frontend AST Nodes Passive Preflight Result

```text
src_ast_mod_rs_lines=627
src_ast_nodes_rs_lines=263
astnode_direct_extraction_allowed=0
selected_type_family=ast_simple_decls
selected_types=ParamDecl,DelegateExposeDecl,DelegateDecl,TransitionDecl,ContractKind
deferred_types=CatchClause,FieldDecl,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-SIMPLE-DECLS-PASSIVE-SPLIT-001
```

## Frontend AST LiteralValue Passive Split Result

```text
moved_type=LiteralValue
new_owner=crates/hakorune_frontend_ast/src/literal.rs
display_impl_owner=hakorune-frontend-ast
compat_reexport=src/ast/syntax.rs
runtime_conversion_owner=src/ast/literal_box_bridge.rs
bridge_api=literal_to_nyash_box,literal_from_nyash_box
inherent_runtime_conversion_methods_preserved=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-NODES-PASSIVE-PREFLIGHT-001
```

## Frontend AST LiteralValue Bridge Design Result

```text
literal_data_owner_selected=hakorune-frontend-ast
runtime_conversion_owner=src/ast/literal_box_bridge.rs
orphan_rule_blocks_inherent_method_compat=1
internal_to_nyash_box_callsite_count=0
internal_literal_from_nyash_box_callsite_count=0
bridge_api=literal_to_nyash_box,literal_from_nyash_box
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-LITERAL-VALUE-PASSIVE-SPLIT-001
```

## Frontend AST Attrs Passive Split Result

```text
moved_types=RuneAttr,DeclarationAttrs,RuneProfileExpansion
new_owner=crates/hakorune_frontend_ast/src/attrs.rs
new_owner=crates/hakorune_frontend_ast/src/rune_profile.rs
compat_reexport=src/ast/attrs.rs
compat_reexport=src/rune_profile_registry.rs
historical_import_path_preserved=crate::ast::{RuneAttr,DeclarationAttrs}
historical_profile_path_preserved=crate::rune_profile_registry::*
old_ast_rune_profile_bridge_retired=1
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-LITERAL-VALUE-BRIDGE-DESIGN-001
```

## Frontend AST Attrs Profile Seam Result

```text
new_module=src/ast/rune_profile_bridge.rs
attrs_direct_rune_profile_registry_refs=0
ast_external_refs_outside_bridges=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-ATTRS-PASSIVE-SPLIT-001
```

## Frontend AST Syntax Passive Split Result

```text
moved_types=UnaryOperator,BinaryOperator,BuildPredicate
new_owner=crates/hakorune_frontend_ast/src/operators.rs
new_owner=crates/hakorune_frontend_ast/src/build_predicate.rs
compat_reexport=src/ast/syntax.rs
literal_value_moved=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-ATTRS-PROFILE-SEAM-001
```

## Frontend AST Next Passive Type Selection Result

```text
selected_type_family=syntax_operator_predicate
selected_types=UnaryOperator,BinaryOperator,BuildPredicate
literal_value_moved=0
literal_value_deferred_reason=main_crate_runtime_box_conversion_inherent_impl
selected_next_task=BUILD-FRONTEND-AST-SYNTAX-PASSIVE-SPLIT-001
```

## Frontend AST Span Passive Split Result

```text
moved_type=Span
new_owner=crates/hakorune_frontend_ast/src/span.rs
compat_reexport=src/ast/span.rs
historical_import_path_preserved=crate::ast::Span
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-NEXT-PASSIVE-TYPE-SELECTION-001
```

## Frontend AST Passive Crate Scaffold Result

```text
new_crate=hakorune-frontend-ast
new_crate_scope=passive_frontend_ast_data
root_dependency_added=1
active_ast_moved=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-SPAN-PASSIVE-SPLIT-001
```

## Frontend Parser Env/Logging Seam Result

```text
new_module=src/parser/env.rs
new_module=src/parser/log.rs
parser_config_env_direct_refs_outside_facade=0
parser_runtime_logger_direct_refs_outside_facade=0
ast_external_refs_outside_literal_box_bridge=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-AST-PASSIVE-CRATE-SCAFFOLD-001
```

## Frontend AST Passive Seam Result

```text
new_module=src/ast/literal_box_bridge.rs
passive_literal_data_owner=src/ast/syntax.rs
runtime_box_conversion_owner=src/ast/literal_box_bridge.rs
syntax_rs_runtime_ref_count=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOGGING-SEAM-001
```

## Frontend Crate Preflight Result

```text
parser_ast_total_lines=16308
parser_ast_file_count=92
parser_ast_mir_ref_count=0
parser_ast_backend_ref_count=0
parser_ast_runtime_ref_count=28
parser_ast_config_box_runner_ref_count=37
full_frontend_crate_split_selected=0
selected_first_slice=ast_passive_data_boundary
selected_next_task=BUILD-FRONTEND-AST-PASSIVE-SEAM-001
```

## Crate Split Next Boundary Selection Result

```text
selected_next_boundary=hakorune_frontend
parser_ast_frontend_total_lines=16308
parser_ast_file_count=92
parser_ast_cross_layer_reference_count=356
direct_extraction_allowed=0
selected_next_task=BUILD-FRONTEND-CRATE-PREFLIGHT-001
```

## VM Reference Default-Off Closeout Result

```text
vm_reference_default_off_closed=1
default_features=["cli","plugins"]
vm_reference_feature_remains_available=1
rust_vm_product_route_reopened=0
default_off_cold_build_real_sec=149.82
latest_default_baseline_cold_build_real_sec=161.28
default_off_real_delta_sec=-11.46
build_time_winner_claim=1
selected_next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002
```

## VM Reference Default-Off Measure Result

```text
default_features=["cli","plugins"]
vm_reference_enabled_by_default=0
cold_build_real_sec=149.82
latest_default_baseline_cold_build_real_sec=161.28
default_off_real_delta_sec=-11.46
build_time_winner_claim=1
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-CLOSEOUT-001
```

## VM Reference Default-Off Implementation Result

```text
default_features=["cli","plugins"]
vm_reference_removed_from_default=1
vm_reference_feature_removed=0
cargo_check_default_green=1
cargo_check_features_vm_reference_green=1
cargo_check_no_default_cli_plugins_green=1
emit_mir_json_default_no_vm_green=1
vm_terminal_without_feature_failfast=1
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-MEASURE-001
```

## VM Reference Default-Off Preflight Result

```text
candidate_default_features=["cli","plugins"]
removed_default_feature=vm-reference
vm_reference_feature_remains_available=1
full_no_default_support_claim=0
plugin_stub_fix_in_scope=0
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001
```

## VM Reference Build Measure Result

```text
feature_profile=cli,plugins
vm_reference_enabled=0
cold_build_real_sec=151.21
latest_default_baseline_cold_build_real_sec=161.28
candidate_real_delta_sec=-10.07
build_time_candidate_visible=1
default_feature_changed=0
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-PREFLIGHT-001
```

## VM Reference Gate Closeout Result

```text
vm_reference_feature_scaffold_closed=1
vm_reference_stays_default_on=1
vm_reference_default_off_claim=0
vm_direct_import_error_count_cli_plugins_without_vm_reference=0
cargo_check_no_default_cli_plugins_green=1
cargo_check_no_default_cli_plugins_warning_count=0
cargo_check_no_default_features_green=0
remaining_no_default_failure=plugins_disabled_stub_surface
```

## Stage 0 Result

```text
mir_core_growth_first_slice=control_flow_id_newtypes
moved_types=LoopId,ExitEdgeId,ContinueEdgeId
compat_reexport=src/mir/control_form.rs
behavior_changed=0
```

## Stage 1 First Slice Result

```text
hakorune_mir_plans_created=1
first_family=object_storage_plan
main_crate_compat_facade=src/object_storage_plan.rs
behavior_changed=0
```

## Baseline Result

```text
baseline_card=BUILD-TIME-BASELINE-MEASURE-001
cold_build_real_sec=157.37
cold_build_user_sec=208.27
cold_build_sys_sec=9.49
large_file_count=0
```

## Stage 1 Second Slice Result

```text
second_family=aggregate_storage_plan
owner=crates/hakorune_mir_plans/src/aggregate_storage_plan.rs
main_crate_compat_facade=src/aggregate_storage_plan.rs
behavior_changed=0
```

## Stage 1 Third Slice Result

```text
third_family=map_repr_plan_pure_data_subset
owner=crates/hakorune_mir_plans/src/map_repr_plan
main_crate_builder_facade=src/mir/map_repr_plan/plans.rs
refresh_logic_owner=src/mir/map_repr_plan/refresh.rs
candidate_detection_owner=src/mir/map_repr_plan/candidates.rs
behavior_changed=0
```

## Stage 1 Fourth Slice Result

```text
fourth_family=local_fastpath_fact_pure_aggregator
owner=crates/hakorune_mir_plans/src/local_fastpath_fact.rs
main_crate_assignment_facade=src/mir/local_fastpath_fact.rs
moved_function=build_local_fastpath_facts_from_map_repr_plans
mirfunction_assignment_owner_preserved=1
behavior_changed=0
```

## Stage 1 Fifth Slice Result

```text
fifth_family=typed_field_storage_vocabulary
owner=crates/hakorune_mir_plans/src/typed_field_storage.rs
main_crate_compat_reexport=crate::mir::function::TypedObjectFieldStorage
storage_inference_moved=0
behavior_changed=0
```

## Stage 1 Sixth Slice Result

```text
sixth_family=array_record_passive_bundle
owner=crates/hakorune_mir_plans/src/array_record_plan.rs
main_crate_compat_reexport=crate::mir::function::*
producer_logic_moved=0
behavior_changed=0
```

## Stage 1 Seventh Slice Result

```text
seventh_family=object_state_passive_bundle
owner=crates/hakorune_mir_plans/src/object_state_plan.rs
main_crate_compat_reexport=crate::mir::function::*
declaration_inventory_moved=0
producer_logic_moved=0
behavior_changed=0
```

## Stage 1 Eighth Slice Result

```text
eighth_family=function_fact_passive_bundle
owner=crates/hakorune_mir_plans/src/function_fact_plan.rs
main_crate_compat_reexport=crate::mir::function::*
producer_logic_moved=0
refresh_logic_moved=0
behavior_changed=0
```

## Stage 1 Closeout Result

```text
closed_stage=hakorune_mir_plans_stage_1
remaining_low_risk_passive_bundle_count=0
next_task=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
behavior_changed=0
```

## Post Stage 1 Measurement Result

```text
post_stage1_card=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
cold_build_real_sec=158.95
cold_build_user_sec=212.73
cold_build_sys_sec=11.59
baseline_cold_build_real_sec=157.37
build_time_winner_claim=0
main_crate_still_dominant=1
recommended_next_stage=hakorune_backend_preflight
```

## Backend Split Preflight Result

```text
preflight_card=BUILD-BACKEND-CRATE-PREFLIGHT-001
src_backend_wholesale_split_selected=0
selected_next_boundary=runner_mir_json_emit
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
reason=product_exe_route_uses_mir_json_emit_before_ny_llvmc
behavior_changed=0
```

## MIR JSON Emit Preflight Result

```text
preflight_card=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
src_runner_mir_json_emit_rs_total_lines=10033
crate_mir_reference_count=372
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
reason=emitter_input_view_boundary_required
behavior_changed=0
```

## MIR JSON Emit Boundary SSOT Result

```text
boundary_card=BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
future_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
behavior_changed=0
```

## MIR JSON Export Model Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
new_owner=src/runner/mir_json_export_model.rs
new_vocabulary=MirJsonExportSchema,MirJsonExportRootKind,MirJsonExportModelSummary
mir_json_emit_behavior_changed=0
future_crate_created=0
```

## MIR JSON DTO Closeout Result

```text
closeout_card=BUILD-MIR-JSON-DTO-CLOSEOUT-001
dto_document_constructed=1
mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001
```

## MIR JSON DTO Serializer Design Result

```text
design_card=BUILD-MIR-JSON-DTO-SERIALIZER-DESIGN-001
serializer_input=MirJsonExportDocument
serializer_output=serde_json::Value
serializer_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001
```

## MIR JSON DTO Serializer Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001
serializer_function=mir_json_export_model::serialize_document
serializer_reads_mir_directly=0
root_builder_wired_to_serializer=0
json_output_changed=0
```

## MIR JSON DTO Serializer Parity Wiring Result

```text
wiring_card=BUILD-MIR-JSON-DTO-SERIALIZER-PARITY-WIRING-001
serializer_called_from_root_builder=1
serializer_parity_debug_assert=1
root_builder_returns_existing_payload=1
json_output_changed=0
```

## MIR JSON DTO Serializer Return Switch Result

```text
wiring_card=BUILD-MIR-JSON-DTO-SERIALIZER-RETURN-SWITCH-001
serializer_payload_returned_from_root_builder=1
serializer_parity_debug_assert=1
legacy_root_builder_payload_kept_as_parity_oracle=1
json_output_changed=0
future_crate_created=0
```

## MIR JSON DTO Serializer Closeout Result

```text
closeout_card=BUILD-MIR-JSON-DTO-SERIALIZER-CLOSEOUT-001
serializer_seam_closed=1
mir_json_emit_direct_mir_reference_count=378
direct_mir_json_emit_crate_extraction_selected=0
future_crate_package_name=hakorune-mir-json-emit
future_crate_scope=json_ready_dto_serializer_only
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001
```

## MIR JSON Emit Crate Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001
new_crate=hakorune-mir-json-emit
new_crate_scope=json_ready_dto_serializer_only
new_crate_reads_mir_directly=0
main_crate_dependency_added=0
json_output_changed=0
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001
```

## MIR JSON Emit Crate Facade Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001
main_crate_dependency_added=1
compat_facade=src/runner/mir_json_export_model.rs
serialization_owner=hakorune_mir_json_emit
projection_owner=main_crate
json_output_changed=0
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001
```

## MIR JSON Emit Crate Closeout Result

```text
closeout_card=BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001
new_crate=hakorune-mir-json-emit
serialization_owner=hakorune_mir_json_emit
projection_owner=main_crate
new_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
```

## MIR JSON Emit Post-Split Measurement Result

```text
measure_card=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
cold_build_real_sec=161.28
cold_build_user_sec=213.71
cold_build_sys_sec=10.49
baseline_cold_build_real_sec=157.37
post_stage1_cold_build_real_sec=158.95
build_time_winner_claim=0
selected_next_task=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001
```

## Backend Next Boundary Selection Result

```text
selection_card=BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001
selected_next_boundary=backend_aot
backend_aot_lines=950
backend_aot_dependency_refs=4
selected_next_task=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001
```

## Backend AOT Crate Preflight Result

```text
preflight_card=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001
full_backend_aot_crate_split_selected=0
full_split_blocked_by=MirModule,WasmBackend
selected_first_slice=aot_passive_config_executable_error
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001
```

## Backend AOT Passive Crate Scaffold Result

```text
scaffold_card=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001
new_crate=hakorune-backend-aot
new_crate_scope=aot_error_config_executable_builder
new_crate_reads_mir_directly=0
new_crate_depends_on_wasm_backend=0
main_crate_dependency_added=0
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001
```

## Backend AOT Passive Facade Wiring Result

```text
wiring_card=BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001
main_crate_dependency_added=1
dependency_feature_gate=wasm-backend
passive_aot_support_owner=hakorune_backend_aot
compiler_pipeline_owner=main_crate
removed_main_crate_files=src/backend/aot/config.rs,src/backend/aot/executable.rs
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001
```

## Backend AOT Passive Closeout Result

```text
closeout_card=BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001
passive_aot_support_split_closed=1
post_split_default_cold_build_measure_selected=0
reason=aot_boundary_is_optional_feature_not_default_build_owner
selected_next_task=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001
```

## VM MIR Interpreter Compile Audit Result

```text
audit_card=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001
mir_interpreter_default_compiled=1
mir_interpreter_file_count=66
mir_interpreter_lines=12944
vm_product_route_retired=1
vm_semantic_reference_subset_alive=1
vm_types_live_outside_interpreter=1
immediate_mir_interpreter_delete_selected=0
immediate_mir_interpreter_feature_gate_selected=0
selected_next_task=BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001
```

## VM MIR Interpreter Feature Gate Design Result

```text
design_card=BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001
feature_name=vm-reference
initial_feature_default=on
vm_types_feature_gated=0
mir_interpreter_feature_gated=planned
backend_vm_alias_feature_gated=planned
default_off_selected_now=0
selected_next_task=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
```

## VM Reference Feature Scaffold Result

```text
scaffold_card=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
feature_name=vm-reference
feature_in_default=1
vm_types_feature_gated=0
mir_interpreter_module_feature_gated=1
backend_mirinterpreter_export_feature_gated=1
backend_vm_alias_feature_gated=1
default_off_claim=0
no_default_features_check_green=0
selected_next_task=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
```

## VM Runner Caller Classification Result

```text
classification_card=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
terminal_vm_execution_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_vm_execution_owner_fan_in=high
explicit_vm_repl_keep_joinir_classified_as_vm_reference=1
product_and_bridge_routes_still_use_vm_terminal=1
vm_reference_remove_from_default_allowed=0
selected_next_task=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
```

## VM Terminal Execution Route Design Result

```text
design_card=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
terminal_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_owner_role=vm_reference_terminal
vm_reference_disabled_terminal_behavior=fail_fast
silent_vm_to_aot_fallback=0
silent_aot_to_vm_fallback=0
selected_next_task=BUILD-VM-TERMINAL-FAILFAST-SEAM-001
```

## VM Terminal Fail-Fast Seam Result

```text
implementation_card=BUILD-VM-TERMINAL-FAILFAST-SEAM-001
central_terminal_failfast_added=1
execute_mir_module_quiet_exit_cfg_split=1
execute_mir_module_cfg_split=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
hidden_aot_fallback_added=0
no_default_features_vm_error_count_after=6
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
```

## VM Direct Caller Gate Selection Result

```text
selection_card=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
selected_family=runner_repl_vm_reference_gate
selected_next_task=BUILD-VM-REPL-REFERENCE-GATE-001
reason=single_public_entry_and_no_product_exe_aot_terminal_overlap
default_off_claim=0
```

## VM REPL Reference Gate Result

```text
implementation_card=BUILD-VM-REPL-REFERENCE-GATE-001
repl_eval_line_cfg_split=1
repl_vm_import_outside_cfg=0
default_behavior_changed=0
no_default_features_vm_error_count_after=5
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-002
```

## VM Direct Caller Gate Selection 002 Result

```text
selection_card=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-002
selected_family=join_ir_runner_vm_reference_gate
selected_next_task=BUILD-VM-JOINIR-RUNNER-REFERENCE-GATE-001
reason=structure_only_runner_has_small_public_api_and_is_separate_from_joinir_vm_bridge
default_off_claim=0
```

## VM JoinIR Runner Reference Gate Result

```text
implementation_card=BUILD-VM-JOINIR-RUNNER-REFERENCE-GATE-001
join_ir_runner_api_cfg_split=1
join_ir_runner_exec_cfg_split=1
join_ir_runner_vm_import_outside_cfg=0
default_behavior_changed=0
no_default_features_vm_error_count_after=3
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-003
```

## VM Direct Caller Gate Selection 003 Result

```text
selection_card=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-003
selected_family=join_ir_vm_bridge_reference_gate
selected_next_task=BUILD-VM-JOINIR-BRIDGE-REFERENCE-GATE-001
reason=single_public_run_joinir_via_vm_entry_can_fail_fast_without_retyping_bridge_conversion
default_off_claim=0
```

## VM JoinIR Bridge Reference Gate Result

```text
implementation_card=BUILD-VM-JOINIR-BRIDGE-REFERENCE-GATE-001
run_joinir_via_vm_cfg_split=1
bridge_conversion_modules_gated=0
join_ir_bridge_vm_import_outside_cfg=0
default_behavior_changed=0
no_default_features_vm_error_count_after=2
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-004
```

## VM Direct Caller Gate Selection 004 Result

```text
selection_card=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-004
selected_family=runner_common_vm_helpers_reference_gate
selected_next_task=BUILD-VM-COMMON-HELPERS-REFERENCE-GATE-001
reason=last_remaining_vm_direct_import_family_and_owned_by_keep_vm_routes
default_off_claim=0
```

## VM Common Helpers Reference Gate Result

```text
implementation_card=BUILD-VM-COMMON-HELPERS-REFERENCE-GATE-001
vm_user_factory_mirinterpreter_import_cfg_split=1
vm_execution_mirinterpreter_import_cfg_split=1
vm_execution_no_feature_failfast=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
no_default_features_vm_error_count_after=0
remaining_no_default_failure=plugins_disabled_stub_surface
selected_next_task=BUILD-VM-REFERENCE-GATE-CLOSEOUT-001
```

## MIR JSON Export Model Root Summary Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EXPORT-MODEL-ROOT-SUMMARY-WIRING-001
summary_helper=mir_json_export_model::summarize_root
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```

## MIR JSON DTO Root Projection Wiring Result

```text
wiring_card=BUILD-MIR-JSON-DTO-ROOT-PROJECTION-WIRING-001
dto_document_constructed=1
dto_source=current_json_ready_values
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Closeout Result

```text
closeout_card=BUILD-MIR-JSON-EXPORT-MODEL-CLOSEOUT-001
export_model_seam_closed=1
mir_json_emit_direct_mir_reference_count=378
direct_crate_extraction_selected=0
selected_next_task=BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001
behavior_changed=0
```

## MIR JSON DTO Boundary Design Result

```text
design_card=BUILD-MIR-JSON-DTO-BOUNDARY-DESIGN-001
dto_boundary_required=1
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
future_crate_reads_mir_directly=0
selected_next_task=BUILD-MIR-JSON-DTO-SCAFFOLD-001
```

## MIR JSON DTO Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-DTO-SCAFFOLD-001
new_vocabulary=MirJsonExportDocument,MirJsonExportFunction,MirJsonExportBlock,MirJsonExportInstruction,MirJsonExportSurface
instruction_payload_type=serde_json::Value
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Function Summary Scaffold Result

```text
scaffold_card=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-SCAFFOLD-001
new_vocabulary=MirJsonFunctionExportSummary
function_summary_wired_to_root=0
json_output_changed=0
future_crate_created=0
```

## MIR JSON Export Model Function Summary Wiring Result

```text
wiring_card=BUILD-MIR-JSON-EXPORT-MODEL-FUNCTION-SUMMARY-WIRING-001
summary_helper=mir_json_export_model::summarize_function
summary_consumer=src/runner/mir_json_emit/root.rs
json_output_changed=0
future_crate_created=0
```
