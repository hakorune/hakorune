# Hakorune Stage Term Existing Name Migration Inventory

Token: `STAGE-TERM-EXISTING-NAME-INVENTORY-001`

Status: classification-only inventory.

Parent task: `STAGE-TERM-EXISTING-NAME-MIGRATION-001`

This inventory classifies existing `stage` names before any migration work.
It does not rename files, flags, modules, routes, or docs. direct renames are forbidden until the affected occurrence is classified and its compatibility surface is known.

## Method

Snapshot command:

```bash
rg -n '\b(Stage-[AB]|Stage[0-9]|stage[0-9]|stage-[ab]|--stage[0-9]|--stage-[ab]|stage1_using_resolver|stage3)\b' \
  . \
  -g '!target/**' \
  -g '!.git/**' \
  -g '!docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md'
```

This is a representative path-family inventory, not a line-by-line rename
manifest. It is intended to prevent accidental broad replacements.

## Current Distribution Snapshot

The largest current buckets are:

```text
docs/development/**         historical/current design references
tools/selfhost/**           selfhost route scripts and bootstrap helpers
lang/src/**                 HHako source and compatibility labels
tools/smokes/**             route/gate scripts
src/**                      RHako modules, env, runner, MIR, tests
docs/tools/**               guard index and script index
crates/nyash_kernel/**      kernel export/config compatibility modules
tests/**                    fixture and compatibility tests
```

## Classification Table

| Path family | Observed terms | Classification | Current decision |
| --- | --- | --- | --- |
| `tools/selfhost/**`, `Makefile`, `target/selfhost/*stage1*` references | `stage0`, `stage1`, `stage2`, `stage3`, `Stage1`, `Stage2`, `Stage3` | bootstrap sequence / selfhost artifact labels | Keep for now. Rename only through a bootstrap compatibility card. |
| `tools/selfhost/proof/run_stageb_compiler_vm.sh`, `tools/smokes/v2/lib/stageb_helpers.sh`, related docs | `Stage-B`, `stage-b`, `--stage-b` | compiler mode / compatibility route | Do not rename directly. Future migration must add mode-B aliases before removing compatibility spellings. |
| `nyash.toml` pipeline_v2 entries | `stage1_*` | internal pipeline module compatibility | Do not rename directly. Requires manifest/module migration plan. |
| `crates/nyash_kernel/**`, `src/config/env/stage1.rs`, `src/stage1/**` | `stage1` modules and debug/export labels | ABI/module/debug compatibility path | Do not rename directly. Requires ABI/log compatibility inventory. |
| `docs/tools/check-scripts-index.md`, `docs/tools/script-index.md` | `Stage1`, `Stage0`, `Stage-B`, `Stage-3` | guard index / historical route docs | Rename only when owning scripts are migrated in the same slice. |
| `projects/nyash-wasm/enhanced_playground.html` | `Stage1`, `Stage2`, `Stage3` | frontend demo UI label | Out of compiler naming migration. Needs UI/product review before any rename. |
| `docs/guides/exception-handling.md`, `docs/reference/**` | `Stage0`, `Stage1`, `stage3` | reference/history wording | Rename only after a reference-doc decision records the new term. |
| `tools/ny_stage2_shortcircuit_smoke.sh` and temp file names | `stage2` | legacy compatibility script/temp naming | Do not rename directly. Archive or compatibility task required. |
| `tests/**` and parser fixture names | `stage3`, `Stage-B`, `stage1` | test fixture / compatibility flag | Rename only after replacement flags or aliases exist. |
| `tools/plugins/stage-built.sh` | `stage-built` | ordinary English / build staging | Not part of the compiler stage-term migration. |

## Landed Alias Slice

`STAGE-TERM-SYNTAX3-ALIAS-001` adds `--syntax-3` as the canonical frontend
syntax-level CLI spelling while keeping `--stage3` as a compatibility alias.

Scope:

```text
Rust CLI:
  --syntax-3 is a visible alias for the existing stage3 parser flag

HHako compiler entry:
  --syntax-3 and --stage3 both enable the same parser surface

Rust selfhost child spawn:
  new child invocations use --syntax-3

Selfhost proof/quickstart:
  representative proof command and quickstart examples use --syntax-3

Reference docs:
  --syntax-3 is documented first; --stage3 remains compatibility wording
```

Non-claims:

```text
--stage3 removed = 0
NYASH_NY_COMPILER_STAGE3 renamed = 0
parser internal stage3 API renamed = 0
```

## Landed mode-B Compatibility Wording Slice

`STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001` changes live env docs/comments to
`mode-B compatibility` wording while keeping existing `STAGEB` environment
names and route tokens as compatibility aliases.

Scope:

```text
src/config/env/verification_flags.rs:
  dev verify toggle comments say mode-B/selfhost compatibility

src/runner/stage1_bridge/env/parser_stageb.rs:
  bridge env section comments say mode-B compatibility

docs/reference/environment-variables.md:
  HAKO_STAGEB_* is documented as compatibility alias wording
```

Non-claims:

```text
NYASH_STAGEB_DEV_VERIFY renamed = 0
HAKO_STAGEB_* renamed = 0
Stage-B route token removed = 0
runtime behavior changed = 0
```

## Landed mode-B Proof Route Wording Slice

`STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001` changes the explicit proof-only
selfhost route docs/diagnostics to `mode-B compatibility` wording while keeping
the existing script names and route tokens.

Scope:

```text
tools/selfhost/proof/run_stageb_compiler_vm.sh:
  proof-only gate comments and proof-only diagnostic say mode-B compatibility

tools/selfhost/proof/selfhost_smoke.sh:
  proof emission comment says mode-B compatibility

tools/selfhost/README.md:
  proof gate and retired Program(JSON v0) probe wording says mode-B compatibility
```

Non-claims:

```text
--stage-b removed = 0
stageb-delegate renamed = 0
run_stageb_compiler_vm.sh renamed = 0
runtime behavior changed = 0
```

## Landed Stage-1 Bridge mode-B Wording Slice

`STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001` changes Stage-1 bridge module
payload comments/docs to `mode-B compatibility` wording while keeping existing
`HAKO_STAGEB_*` environment names and bridge file names as compatibility
surfaces.

Scope:

```text
src/runner/stage1_bridge/README.md:
  module payload ownership line says mode-B compatibility

src/runner/stage1_bridge/env.rs:
  child env facade comment says mode-B compatibility aliases

src/runner/stage1_bridge/modules.rs:
  HAKO_STAGEB_* doc comments say mode-B compatibility alias/readers
```

Non-claims:

```text
HAKO_STAGEB_* renamed = 0
parser_stageb.rs renamed = 0
modules.rs behavior changed = 0
runtime behavior changed = 0
```

## Landed mode-A Compatibility Route Wording Slice

`STAGE-TERM-MODEA-COMPAT-ROUTE-WORDING-001` changes Rust selfhost compat route
comments/diagnostics to `mode-A compatibility` wording while keeping
`stage-a-compat` runtime-mode tokens and `stage_a_*` file/function names as
compatibility surfaces.

Scope:

```text
src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs:
  compat bridge boundary and diagnostics say mode-A compatibility

src/runner/modes/common_util/selfhost/stage_a_route.rs:
  route helper comments say mode-A compatibility

src/runner/modes/common_util/selfhost/stage_a_policy.rs:
  policy helper comments say mode-A compatibility

src/runner/modes/common_util/selfhost/stage_a_spawn.rs:
  child payload comments say mode-A/mode-B compatibility as appropriate

src/runner/modes/common_util/selfhost/json.rs:
  payload ownership boundary says mode-A compatibility

src/runner/modes/common_util/selfhost/stage0_capture_route.rs:
  retained route wrapper comments say mode-A compatibility

src/runner/selfhost.rs:
  high-level route sequencing comment says mode-A compatibility
```

Non-claims:

```text
stage-a-compat runtime-mode alias renamed = 0
stage_a_* file names renamed = 0
function names renamed = 0
runtime behavior changed = 0
```

## Landed HHako mode-B Entry Wording Slice

`STAGE-TERM-MODEB-HHAKO-ENTRY-WORDING-001` changes HHako compiler entry
comments/docs to `mode-B compatibility` wording while keeping `StageB*` Box
names, trace strings, file names, and `--stage-b` route tokens unchanged.

Scope:

```text
lang/src/compiler/README.md:
  compiler entry ownership line says mode-B compatibility

lang/src/compiler/entry/compiler_stageb.hako:
  adapter lane comments say mode-B compatibility

lang/src/compiler/entry/stageb_args_box.hako:
  args/source resolution comment says mode-B compatibility

lang/src/compiler/entry/stageb_build_options_box.hako:
  BuildBox option packaging comment says mode-B compatibility

lang/src/compiler/entry/stageb_compile_adapter_box.hako:
  BuildBox handoff comment says mode-B compatibility

lang/src/compiler/entry/stageb_output_box.hako:
  Program(JSON v0) output boundary comment says mode-B compatibility
```

Non-claims:

```text
StageB* Box names renamed = 0
trace strings renamed = 0
--stage-b removed = 0
runtime behavior changed = 0
```

## Landed HHako mode-B Compat Fixture Wording Slice

`STAGE-TERM-MODEB-HHAKO-COMPAT-FIXTURE-WORDING-001` changes HHako legacy
fixture/adapter comments to `mode-B compatibility` wording where the comments
explicitly defer live source-to-Program authority to BuildBox.

Scope:

```text
lang/src/compiler/entry/bundle_resolver.hako:
  legacy bundling resolver fixture comments say mode-B compatibility

lang/src/compiler/entry/stageb_body_extractor_box.hako:
  legacy body extractor / bundle comments say mode-B compatibility

lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako:
  keyword cleanup comment says mode-B compatibility
```

Non-claims:

```text
StageB* Box names renamed = 0
HAKO_STAGEB_* env names renamed = 0
trace strings renamed = 0
runtime behavior changed = 0
```

## Landed HHako Compiler Route Wording Slice

`STAGE-TERM-HHAKO-COMPILER-ROUTE-WORDING-001` changes `compiler.hako` route
comments and the string-indexing diagnostic to `mode-A compatibility` /
`mode-B compatibility` wording while keeping route tokens, trace strings, and
Box names unchanged.

Scope:

```text
lang/src/compiler/entry/compiler.hako:
  route comments and one string-indexing diagnostic say mode-A/mode-B compatibility
```

Non-claims:

```text
StageB* Box names renamed = 0
stage_b field renamed = 0
trace strings renamed = 0
--stage-b removed = 0
```

## Landed HHako mode-B Helper Comment Wording Slice

`STAGE-TERM-MODEB-HHAKO-HELPER-COMMENT-WORDING-001` changes HHako helper
comments that describe mode-B/source-route responsibilities to
`mode-B compatibility` wording while keeping `StageB*` Box names, env names,
trace strings, file names, and route tokens unchanged.

Scope:

```text
lang/src/compiler/entry/stageb_driver_guard_box.hako:
  driver trace/depth guard helper comment says mode-B compatibility

lang/src/compiler/entry/stageb_trace_box.hako:
  trace helper comments say mode-B compatibility

lang/src/compiler/entry/stageb_main_detection_box.hako:
  main/body detection and fallback dependency comments say mode-A/mode-B compatibility

lang/src/compiler/entry/stageb/stageb_rune_box.hako:
  Rune helper comment says mode-B compatibility

lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako:
  user-box declaration scanner comment says mode-B compatibility
```

Non-claims:

```text
StageB* Box names renamed = 0
HAKO_STAGEB_* env names renamed = 0
trace strings renamed = 0
stageb_* file names renamed = 0
runtime behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
```

## Landed mode-B Capture Caller Guard Wording Slice

`STAGE-TERM-MODEB-CAPTURE-CALLER-GUARD-WORDING-001` changes the active
Program(JSON) capture caller guard comments/diagnostics and quick gate label to
`mode-B compatibility` wording while keeping the guard script name and allowed
caller surfaces unchanged.

Scope:

```text
tools/checks/stageb_program_json_capture_caller_guard.sh:
  comments and failure diagnostic say mode-B compatibility

tools/checks/lib/dev_gate_quick_steps.sh:
  quick gate label says mode-B compatibility
```

Non-claims:

```text
stageb_program_json_capture_caller_guard.sh renamed = 0
stageb_program_json_capture.sh renamed = 0
allowed caller list changed = 0
runtime behavior changed = 0
```

## Landed phase-1 Program(JSON) Guard Wording Slice

`STAGE-TERM-PHASE1-PROGRAM-JSON-GUARD-WORDING-001` changes active Stage1
Program(JSON) guard comments/diagnostics and quick gate labels to
`phase-1 compatibility` wording while keeping script names, fixture names, and
helper symbols unchanged.

Scope:

```text
tools/checks/stage1_emit_program_json_runtime_helper_guard.sh:
  runtime-helper guard comment says phase-1 compatibility

tools/checks/stage1_program_json_compat_caller_guard.sh:
  probe-only route comment and failure diagnostic say phase-1 compatibility

tools/checks/lib/dev_gate_quick_steps.sh:
  quick gate labels say phase-1 compatibility
```

Non-claims:

```text
stage1_* script names renamed = 0
stage1_* helper symbols renamed = 0
fixture paths renamed = 0
runtime behavior changed = 0
```

## Landed Stage0 Shape Gate Label Wording Slice

`STAGE-TERM-STAGE0-SHAPE-GATE-LABEL-WORDING-001` changes the quick gate label
for the Stage0-named shape inventory script to its concrete responsibility:
`GlobalCallTarget shape inventory guard`.

Scope:

```text
tools/checks/lib/dev_gate_quick_steps.sh:
  quick gate label says GlobalCallTarget shape inventory guard
```

Non-claims:

```text
stage0_shape_inventory_guard.sh renamed = 0
stage0-llvm-line-shape-inventory-ssot.md renamed = 0
GlobalCallTargetShape behavior changed = 0
runtime behavior changed = 0
```

## Landed HHako FuncScanner Comment Wording Slice

`STAGE-TERM-MODEB-HHAKO-FUNC-SCANNER-COMMENT-WORDING-001` changes FuncScanner
comments that describe the mode-B compiler/VM path to `mode-B compatibility`
wording. The PHI-related implementation is not changed.

Scope:

```text
lang/src/compiler/entry/func_scanner.hako:
  scanner ownership comment says mode-B compatibility

lang/src/compiler/entry/func_scanner_helpers.hako:
  one loop exit/PHI shape comment says mode-B compatibility VM path
```

Non-claims:

```text
FuncScanner behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
StageB* Box names renamed = 0
runtime behavior changed = 0
```

## Landed mode-B K2-Wide Guard Diagnostic Wording Slice

`STAGE-TERM-MODEB-K2-WIDE-GUARD-DIAGNOSTIC-WORDING-001` changes two active
K2-wide guard failure diagnostics from `Stage-B` wording to
`mode-B compatibility` wording.

Scope:

```text
tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh:
  failure diagnostic says mode-B compatibility user_box_decls scanner

tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh:
  failure diagnostic says mode-B compatibility parser route
```

Non-claims:

```text
k2_wide_stageb_* script names renamed = 0
StageB* Box names renamed = 0
guard logic changed = 0
runtime behavior changed = 0
```

## Landed Selfhost Smoke Comment Wording Slice

`STAGE-TERM-SELFHOST-SMOKE-COMMENT-WORDING-001` changes active selfhost smoke
comments and human-facing diagnostics to `mode-B compatibility`,
`phase-1 compatibility`, and `syntax-3` wording where the old wording used
unqualified stage terms.

Scope:

```text
tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh:
  wrapper diagnostic and Rust VM lane comment say mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh:
  FuncScanner delegated box header comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh:
  FuncScanner method-decl boundary comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh:
  legacy lambda pair comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh:
  FuncScanner parity comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh:
  steady-state route parity comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh:
  route parity comparison comment says mode-B compatibility

tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stage1_contract_smoke_vm.sh:
  bootstrap capability comment says phase-1 compatibility

tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh:
  stable paths smoke label says syntax-3
```

Non-claims:

```text
stageb_* smoke file names renamed = 0
StageB* Box names renamed = 0
Stage1UsingResolverBox renamed = 0
exact expected Stage1 stderr changed = 0
runtime behavior changed = 0
```

## Landed Check Scripts Index Wording Slice

`STAGE-TERM-CHECK-SCRIPTS-INDEX-WORDING-001` changes `docs/tools/check-scripts-index.md`
descriptions for already-migrated active guards to `mode-B compatibility`,
`phase-1 compatibility`, or concrete `GlobalCallTarget` wording.

Scope:

```text
docs/tools/check-scripts-index.md:
  stage1_emit_program_json_runtime_helper_guard description says phase-1 compatibility
  stage1_program_json_compat_caller_guard description says phase-1 compatibility
  stage0_shape_inventory_guard description says GlobalCallTarget shape inventory
  stageb_program_json_capture_caller_guard description says mode-B compatibility
  k2_wide_stageb_* descriptions say mode-B compatibility
```

Non-claims:

```text
guard script names renamed = 0
stage0_shape_inventory_guard.sh renamed = 0
stage1_* guard script names renamed = 0
stageb_* guard script names renamed = 0
docs/tools/check-scripts-index.md historical rows fully migrated = 0
runtime behavior changed = 0
```

## Landed syntax-3 Rust Env Comment Wording Slice

`STAGE-TERM-SYNTAX3-RUST-ENV-COMMENT-WORDING-001` changes Rust environment
flag comments from `Stage-3` wording to `syntax-3` wording while keeping
existing `stage3` feature tokens and env names as compatibility surfaces.

Scope:

```text
src/config/env/parser_flags.rs:
  parser gate comments say syntax-3 and identify `stage3` / `parser-stage3`
  as compatibility tokens

src/config/env/selfhost_flags.rs:
  selfhost child-arg comment says `--syntax-3`, with `--stage3` as
  compatibility alias
```

Non-claims:

```text
NYASH_FEATURES=stage3 removed = 0
NYASH_PARSER_STAGE3 renamed = 0
HAKO_PARSER_STAGE3 renamed = 0
NYASH_NY_COMPILER_STAGE3 renamed = 0
feature_stage3_enabled renamed = 0
runtime behavior changed = 0
```

## Landed HHako Parser/Build Comment Wording Slice

`STAGE-TERM-HHAKO-PARSER-BUILD-COMMENT-WORDING-001` changes HHako parser,
build, and MIR builder documentation comments from unqualified stage terms to
`mode-A compatibility`, `mode-B compatibility`, or `syntax-3` wording.

Scope:

```text
lang/src/compiler/build/README.md:
  live bundle entry comment says mode-B compatibility

lang/src/compiler/build/build_bundle_facade_box.hako:
  BuildBundleFacade comment says mode-B compatibility

lang/src/compiler/mirbuilder/README.md:
  bridge caller and Program(JSON v0) descriptions say mode-A/mode-B compatibility

lang/src/compiler/parser/**:
  parser comments say mode-B compatibility or syntax-3 where they previously
  used Stage-B / Stage-3 wording
```

Non-claims:

```text
stage3 fields/functions renamed = 0
trace strings renamed = 0
parser behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
runtime behavior changed = 0
```

## Landed JSON v0 Bridge Comment Wording Slice

`STAGE-TERM-JSON-V0-BRIDGE-COMMENT-WORDING-001` changes Rust JSON v0 bridge
comments and one freeze diagnostic from `Stage-B` / `Stage0` wording to
`mode-B compatibility` or `bootstrap` wording.

Scope:

```text
src/runner/json_v0_bridge/ast.rs:
  BlockExpr tail schema comment says mode-B compatibility

src/runner/json_v0_bridge/lowering/if_legacy.rs:
  if-not legacy encoding comment and diagnostic say mode-B legacy

src/runner/json_v0_bridge/lowering/lambda_legacy.rs:
  lambda legacy encoding comment says mode-B compatibility

src/runner/json_v0_bridge/lowering/loop_.rs:
  loop lowering dev trace comment says mode-B compatibility / JSON v0

src/runner/json_v0_bridge/lowering/loop_range.rs:
  stop-line comment says no bootstrap desugar

src/runner/json_v0_bridge/lowering/program.rs:
  static `me.method(...)` dispatch comment says mode-B compatibility JSON

src/runner/json_v0_bridge/lowering/expr/block_expr.rs:
  BlockExpr tail comment says mode-B compatibility
```

Non-claims:

```text
try_lower_stageb_* function names renamed = 0
JSON v0 bridge behavior changed = 0
route symbols renamed = 0
runtime behavior changed = 0
```

## Landed HHako Build/Test Comment Wording Slice

`STAGE-TERM-HHAKO-BUILD-TEST-COMMENT-WORDING-001` changes remaining HHako
build/test comments from `Stage0` / `Stage-B` wording to `bootstrap` or
`mode-B compatibility` wording.

Scope:

```text
lang/src/compiler/build/build_bundle_facade_box.hako:
  BuildBox source-only comment says bootstrap source execution

lang/src/compiler/tests/funcscanner_skip_ws_min.hako:
  delegate-call test comments say mode-B compatibility

lang/src/compiler/tests/stageb_min_sample.hako:
  minimal test harness comments say mode-B compatibility
```

Non-claims:

```text
stageb_min_sample.hako renamed = 0
StageBFuncScannerBox renamed = 0
test behavior changed = 0
runtime behavior changed = 0
```

## Landed App Binary-Only Smoke Comment Wording Slice

`STAGE-TERM-APP-BINARY-ONLY-SMOKE-COMMENT-WORDING-001` changes the app-level
binary-only selfhost readiness smoke comments from unqualified stage terms to
phase-1 / phase-2 proxy wording.

Scope:

```text
tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh:
  contract comment says phase-1 repo dependencies
  pass1/pass2 comments say phase-1 / phase-2 proxy
```

Non-claims:

```text
stage1.mir / stage2.mir artifact filenames renamed = 0
pass1/pass2 behavior changed = 0
binary-only readiness contract changed = 0
runtime behavior changed = 0
```

## Landed App Smoke Phase Comment Wording Slice

`STAGE-TERM-APP-SMOKE-PHASE-COMMENT-WORDING-001` changes remaining app smoke
comments that used unqualified `stage1` / `Stage1` / `stage-a-compat` wording
to phase-1 or mode-A compatibility wording.

Scope:

```text
tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh:
  binary-only run route contract comment says phase-1

tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh:
  no-compat runtime probe comment says mode-A compatibility
  separate diagnostic owner comment says phase-1 compatibility

tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_compile_run_split_contract_vm.sh:
  binary-only direct route comment says phase-1

tools/smokes/v2/profiles/integration/apps/lib/README.md:
  shared helper comment says binary-only phase-1 probes
```

Non-claims:

```text
stage1-cli log tags renamed = 0
stage-a-compat runtime-mode token renamed = 0
stage3 feature/env tokens renamed = 0
artifact filenames renamed = 0
runtime behavior changed = 0
```

## Landed Stage0 Capture Comment Wording Slice

`STAGE-TERM-STAGE0-CAPTURE-COMMENT-WORDING-001` changes Stage0-named capture
helper comments to bootstrap capture wording while keeping the compatibility
file/function/test names unchanged.

Scope:

```text
src/runner/modes/common_util/selfhost/stage0_capture.rs:
  route-neutral capture comment says bootstrap capture

src/runner/modes/common_util/selfhost/stage0_capture_route.rs:
  route builder comments say bootstrap capture
```

Non-claims:

```text
stage0_capture.rs renamed = 0
stage0_capture_route.rs renamed = 0
build_stage0_* function names renamed = 0
stage0_capture tests renamed = 0
runtime behavior changed = 0
```

## Landed Pipeline V2 Comment Wording Slice

`STAGE-TERM-PIPELINE-V2-COMMENT-WORDING-001` changes Pipeline V2 comments and
README wording from unqualified stage terms to phase-1 / phase-2 / phase-3,
syntax-3, or mode-B compatibility wording.

Scope:

```text
lang/src/compiler/pipeline_v2/README.md:
  pipeline input / guard wording says phase-1 / phase-2 / phase-3
  frontend parser acceptance wording says syntax-3 where relevant

lang/src/compiler/pipeline_v2/*.hako:
  comment-only wording uses phase-1 JSON/scanner/args, syntax-3 parser
  acceptance, or mode-B compatibility entry wording
```

Non-claims:

```text
Stage1* Box names renamed = 0
stage1_* file/module names renamed = 0
lower_stage1_* API names renamed = 0
stage3_flag field/argument renamed = 0
pipeline behavior changed = 0
runtime behavior changed = 0
```

## Landed JoinIR Lowering Comment Wording Slice

`STAGE-TERM-JOINIR-LOWERING-COMMENT-WORDING-001` changes selected JoinIR
lowering comments and dev-log wording from unqualified stage terms to
lower-resolver, phase-1 compatibility, or mode-B compatibility wording.

Scope:

```text
src/mir/join_ir/lowering/value_id_ranges.rs:
  ValueId range notes say lower-resolver or mode-B compatibility

src/mir/join_ir/lowering/mod.rs:
  lowering target comments say lower-resolver, phase-1 compatibility, or
  mode-B compatibility

src/mir/join_ir/lowering/if_lowering_router.rs:
  whitelist comments say phase-1 compatibility instead of Stage-1 rollout

src/mir/join_ir/lowering/generic_case_a/** and loop_view_builder.rs:
  lowerer comments/dev logs say lower-resolver lowerer
```

Non-claims:

```text
stage1_using_resolver file/module/function names renamed = 0
stageb_body / stageb_funcscanner file/module/function names renamed = 0
Stage1* / StageB* Box names renamed = 0
JoinIR lowering behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
```

## Landed Lang README Phase Wording Slice

`STAGE-TERM-LANG-README-PHASE-WORDING-001` changes current `lang/README.md`
distribution/launcher wording from unqualified Stage1/Stage2 phrasing to
phase-1 / K2+ wording.

Scope:

```text
lang/README.md:
  dev line says phase-1 core compatibility
  stable snapshot says phase-1 bridge/proof reading
  distribution line says K2+ instead of Stage2+
  legacy stage0/stage1 Program(JSON v0) boundary terms remain explicit legacy
  compatibility vocabulary
```

Non-claims:

```text
tools/selfhost/mainline/build_stage1.sh renamed = 0
target/selfhost/hakorune path renamed = 0
lang/bin/hakorune behavior changed = 0
legacy stage0/stage1 boundary vocabulary deleted = 0
```

## Landed Docs Tools Quick Entry Wording Slice

`STAGE-TERM-DOCS-TOOLS-QUICK-ENTRY-WORDING-001` changes the active tools quick
entry/index rows from unqualified stage terms to phase-1 / syntax-3 wording.

Scope:

```text
docs/tools/README.md:
  bug-origin triage labels say phase1-route and phase-1 compiler side

docs/tools/script-index.md:
  active selfhost script descriptions say phase-1 compatibility
  syntax-level same-result helper says syntax-3
```

Non-claims:

```text
tools/selfhost/* script names renamed = 0
stage1_cli_env.hako compatibility fixture renamed = 0
docs/tools/check-scripts-index.md historical ledger rewritten = 0
runtime behavior changed = 0
```

## Landed Stage1 Bridge Phase Comment Wording Slice

`STAGE-TERM-STAGE1-BRIDGE-PHASE-COMMENT-WORDING-001` changes current
`src/runner/stage1_bridge/**` README titles and Rust file-header comments from
unqualified Stage-1 / Stage1 wording to phase-1 compatibility wording.

Scope:

```text
src/runner/stage1_bridge/README.md:
  bridge ownership and route-facade comments say phase-1 compatibility

src/runner/stage1_bridge/**/README.md:
  helper-local README titles use phase-1 compatibility bridge wording

src/runner/stage1_bridge/**/*.rs:
  selected file-header comments use phase-1 compatibility wording
```

Non-claims:

```text
stage1_bridge directory renamed = 0
Stage1* type names renamed = 0
stage1_* module/function names renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
[stage1-cli] log tags changed = 0
smoke expected stderr changed = 0
embedded_stage1_modules_snapshot.json touched = 0
```

## Landed Environment Reference Phase Wording Slice

`STAGE-TERM-ENV-REFERENCE-PHASE-WORDING-001` changes the active environment
variable reference from unqualified Stage-1 / Stage-3 / Stage0 wording to
phase-1 compatibility, syntax-3, or bootstrap wording where the variables are
compatibility surfaces.

Scope:

```text
docs/reference/environment-variables.md:
  JSON v0 / phase-1 compatibility route descriptions use phase-1 wording
  parser feature descriptions use syntax-3 wording while keeping `stage3`
  compatibility tokens
  historical cleanup / explicit keep lane descriptions use bootstrap wording
```

Non-claims:

```text
NYASH_STAGE1_* / STAGE1_* env names renamed = 0
NYASH_FEATURES=stage3 compatibility token removed = 0
NYASH_PARSER_STAGE3 / HAKO_PARSER_STAGE3 env names renamed = 0
NYASH_NY_COMPILER_STAGE3 env name renamed = 0
runtime behavior changed = 0
```

## Landed Rust Stage1 Env Helper Comment Wording Slice

`STAGE-TERM-RUST-STAGE1-ENV-HELPER-COMMENT-WORDING-001` changes
`src/config/env/stage1.rs` helper comments from unqualified Stage-1 wording to
phase-1 compatibility wording while keeping the env helper module path as a
compatibility surface.

Scope:

```text
src/config/env/stage1.rs:
  module doc comment says phase-1 compatibility / selfhost CLI env helper
  helper doc comments say phase-1 compatibility for stub, mode, emit, input,
  backend, entry, child-args, and debug surfaces
```

Non-claims:

```text
src/config/env/stage1.rs renamed = 0
stage1 module path renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
STAGE1_* legacy env aliases removed = 0
runtime behavior changed = 0
```

## Landed Rust Stage1 Boundary Comment Wording Slice

`STAGE-TERM-RUST-STAGE1-BOUNDARY-COMMENT-WORDING-001` changes the
`src/stage1` boundary README and Program(JSON v0) header comments from
unqualified Stage1 wording to phase-1 compatibility wording. The explicit
`Stage1` / `Stage2` legacy artifact-label explanation remains as compatibility
vocabulary.

Scope:

```text
src/stage1/README.md:
  boundary title and responsibility lines use phase-1 compatibility wording
  legacy Stage1/Stage2 artifact/proof label explanation remains explicit

src/stage1/mod.rs:
src/stage1/program_json_v0.rs:
src/stage1/program_json_v0/routing.rs:
src/stage1/program_json_v0/README.md:
  module/header comments use phase-1 compatibility wording
```

Non-claims:

```text
src/stage1 directory renamed = 0
src/stage2 directory created = 0
stage1_bridge helper/function names renamed = 0
Program(JSON v0) behavior changed = 0
runtime behavior changed = 0
```

## Landed Rust Stage1 Program JSON Test Wording Slice

`STAGE-TERM-RUST-STAGE1-PROGRAM-JSON-TEST-WORDING-001` changes selected
human-facing assertion / expectation messages under `src/stage1/program_json_v0`
from unqualified stage1 wording to phase-1 compatibility wording.

Scope:

```text
src/stage1/program_json_v0/tests/stage1_sources.rs:
  program-json caller and source authority assertion messages say phase-1
  compatibility

src/stage1/program_json_v0/tests/classification_contract.rs:
  bridge strict-parse expectation message says phase-1 compatibility
```

Non-claims:

```text
test function names renamed = 0
stage1_cli_env.hako fixture path renamed = 0
Stage1* Box names renamed = 0
helper function names renamed = 0
runtime behavior changed = 0
```

## Landed Check Scripts Index Phase/Env Wording Slice

`STAGE-TERM-CHECK-SCRIPTS-INDEX-PHASE-ENV-WORDING-001` changes selected active
`docs/tools/check-scripts-index.md` guard descriptions from unqualified
Stage1 / Stage-1 / Stage0 wording to phase-1 compatibility or bootstrap
wording.

Scope:

```text
docs/tools/check-scripts-index.md:
  selfhost surface guard description says phase-1 compatibility selfhost sources
  NyRT env P0 centralization row says phase-1 compatibility bridge defaults
  cleanup/catch boundary row says bootstrap cleanup/catch boundary
```

Non-claims:

```text
guard script names renamed = 0
historical check-scripts ledger broadly rewritten = 0
guard behavior changed = 0
runtime behavior changed = 0
```

## Landed Stage1 Bridge Alias Comment Wording Slice

`STAGE-TERM-STAGE1-BRIDGE-ALIAS-COMMENT-WORDING-001` changes selected
`src/runner/stage1_bridge` env/module comments from unqualified Stage-1 CLI
wording to phase-1 compatibility CLI / alias wording.

Scope:

```text
src/runner/stage1_bridge/env/parser_stageb.rs:
  phase-1 compatibility alias promotion

src/runner/stage1_bridge/modules.rs:
  well-known aliases required by phase-1 compatibility CLI
```

Non-claims:

```text
stage1_bridge path renamed = 0
Stage1* Rust type names renamed = 0
stage1_* modules/functions renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
[stage1-cli] log tags changed = 0
runtime behavior changed = 0
```

## Landed README mode-B User-Facing Wording Slice

`HAKORUNE-README-MODEB-USER-FACING-WORDING-001` changes the root README's
current developer quickstart from Stage-B wording to mode-B compatibility
wording.

Scope:

```text
README.md:
  Quick Emit MIR helper route
  Performance quickstart MIR emit bench label
```

Non-claims:

```text
syntax highlighting fences renamed = 0
NYASH_* env names renamed = 0
ny-llvmc / nyash-llvm-compiler tool names renamed = 0
historical Nyash Era section rewritten = 0
runtime behavior changed = 0
```

## Migration Eligibility

An occurrence is eligible for a future rename only if all conditions hold:

- it is classified as `compiler mode`, `runner phase`, `lowering pass`, or
  `frontend syntax level`;
- the compatibility surface is known;
- an alias or replacement route exists when external users may reference it;
- the owning guard can prove the old and new names are not both new SSOTs;
- the slice changes one naming layer only.

## Forbidden Moves

```text
direct global replacement = 0
stage term rename without classification = 0
bootstrap stage rename without compatibility plan = 0
Stage-B route removal without mode-B alias = 0
stage1 module removal without ABI/log inventory = 0
--stage3 removal without syntax-level replacement = 0
```

## Next Safe Work

The next migration slice may pick exactly one classified layer:

```text
compiler mode:
  Stage-A / Stage-B -> mode-A / mode-B, with compatibility aliases

runner phase:
  runner stage1 -> runner phase-1, after route ownership is known

lowering pass:
  stage1_using_resolver -> lower-resolver

frontend syntax level:
  --stage3 -> --syntax-3, after CLI aliasing is defined
```

If more than one layer appears necessary, keep this inventory as the boundary
and split the work.

## Acceptance

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
