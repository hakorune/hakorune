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
