# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-04
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Read the workstream/task-order doc named by `latest_workstream_card`, when present.
4. If `current_blocker_token` names an explicit design-stop frontier, also
   read `docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md`
   before selecting any new family-specific task.
5. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the current code slice is ready.

## Current Fields

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_workstream_card`
- `latest_card_path`
- `current_blocker_token`

If `current_blocker_token` names the explicit design-stop frontier, do not
invent a new executable owner from historical mirrors. Review the frontier
card first and keep the goal open until the frontier names a concrete next
owner.

Current implementation details, acceptance, parked items, and non-claims live in
the active card and task-order SSOT. Do not duplicate them here.

## Active Pivot Slice

Status: active.

Current route:

```text
SOURCE-SELFHOST-RUST-TO-HAKO-CONVERTER-ROLE-PIVOT-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-001
  -> MIRBUILDER-STORAGE-CLASS-CLASSIFIER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-STORAGE-CLASS-CLASSIFIER-PARITY-GATE-001
  -> MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-001
  -> MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-PARITY-GATE-001
  -> MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-002
  -> MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-PARITY-GATE-001
  -> MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-003
  -> MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-PARITY-GATE-001
  -> MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-004
  -> MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-PARITY-GATE-001
  -> MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-005
  -> MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-PARITY-GATE-001
  -> MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-006
  -> MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-PARITY-GATE-001
  -> MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-007
  -> MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-PARITY-GATE-001
  -> MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-008
  -> MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-PARITY-GATE-001
  -> MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-009
  -> MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001
  -> MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
  -> MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-PARITY-GATE-001
  -> MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-ADOPTION-DECISION-001
  -> MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-010
```

Rule:

```text
full Rust-to-Hako MirBuilder converter is no longer the Source Selfhost main path
Rust remains bootstrap oracle / parity reference
small hand-authored .hako native owner pilots are selected manually
correctness must be proven by Rust-oracle parity fixtures
Source Selfhost claim remains 0
first owner pilot is storage_class_classifier
return_prelude_policy is harness smoke only
storage_class is HakoAdopted as a narrow parity pilot; next is pilot selection rerun
placement_effect_tag_formatter is selected as the second parity pilot; next is its Rust-oracle fixture
placement_effect_tag_formatter has a 25-row Rust-oracle fixture; next is hand-authored .hako implementation
placement_effect_tag_formatter has hand-authored .hako implementation; next is parity gate
placement_effect_tag_formatter parity gate is green; next is HakoAdoption decision
placement_effect_tag_formatter is HakoAdopted as the second parity pilot; next is pilot selection rerun 002
static_scalar_fact_classifier is selected as the third parity pilot; next is its Rust-oracle fixture
static_scalar_fact_classifier has an 8-row Rust-oracle fixture; next is hand-authored .hako implementation
static_scalar_fact_classifier has hand-authored .hako implementation; next is parity gate
static_scalar_fact_classifier parity gate is green; next is HakoAdoption decision
static_scalar_fact_classifier is HakoAdopted as the third parity pilot; next is pilot selection rerun 003
string_corridor_name_vocabulary_classifier is selected as the fourth parity pilot; next is its Rust-oracle fixture
string_corridor_name_vocabulary_classifier has an 18-row Rust-oracle fixture; next is hand-authored .hako implementation
string_corridor_name_vocabulary_classifier has hand-authored .hako implementation; next is parity gate
string_corridor_name_vocabulary_classifier parity gate is green; next is HakoAdoption decision
string_corridor_name_vocabulary_classifier is HakoAdopted as the fourth parity pilot; next is pilot selection rerun 004
same_module_definition_kind_formatter is selected as the fifth parity pilot; next is its Rust-oracle fixture
same_module_definition_kind_formatter has a 2-row Rust-oracle fixture; next is hand-authored .hako implementation
same_module_definition_kind_formatter has hand-authored .hako implementation; next is parity gate
same_module_definition_kind_formatter parity gate is green; next is HakoAdoption decision
same_module_definition_kind_formatter is HakoAdopted as the fifth parity pilot; next is pilot selection rerun 005
user_box_method_type_label_formatter is selected as the sixth parity pilot; next is its Rust-oracle fixture
user_box_method_type_label_formatter has a 10-row Rust-oracle fixture; next is hand-authored .hako implementation
user_box_method_type_label_formatter has hand-authored .hako implementation; next is parity gate
user_box_method_type_label_formatter parity gate is green; next is HakoAdoption decision
user_box_method_type_label_formatter is HakoAdopted as the sixth parity pilot; next is pilot selection rerun 006
core_method_carrier_token_formatter is selected as the seventh parity pilot; next is its Rust-oracle fixture
core_method_carrier_token_formatter has a 32-row Rust-oracle fixture; next is hand-authored .hako implementation
core_method_carrier_token_formatter has hand-authored .hako implementation; next is parity gate
core_method_carrier_token_formatter parity gate is green; next is HakoAdoption decision
core_method_carrier_token_formatter is HakoAdopted as the seventh parity pilot; next is pilot selection rerun 007
generic_method_route_fact_token_formatter is selected as the eighth parity pilot; next is its Rust-oracle fixture
generic_method_route_fact_token_formatter has a 12-row Rust-oracle fixture; next is hand-authored .hako implementation
generic_method_route_fact_token_formatter has hand-authored .hako implementation; next is parity gate
generic_method_route_fact_token_formatter parity gate is green; next is HakoAdoption decision
generic_method_route_fact_token_formatter is HakoAdopted as the eighth parity pilot; next is pilot selection rerun 008
closure_call_shape_classifier is selected as the ninth parity pilot; next is its Rust-oracle fixture
closure_call_shape_classifier has a 4-row Rust-oracle fixture; next is hand-authored .hako implementation
closure_call_shape_classifier has hand-authored .hako implementation; next is parity gate
closure_call_shape_classifier parity gate is green; next is HakoAdoption decision
closure_call_shape_classifier is HakoAdopted as the ninth parity pilot; next is pilot selection rerun 009
region_ref_slot_kind_classifier is selected as the tenth parity pilot; next is its Rust-oracle fixture
region_ref_slot_kind_classifier has a 10-row Rust-oracle fixture; next is hand-authored .hako implementation
region_ref_slot_kind_classifier has hand-authored .hako implementation; next is parity gate
region_ref_slot_kind_classifier parity gate is green; next is HakoAdoption decision
region_ref_slot_kind_classifier is HakoAdopted as the tenth parity pilot; next is pilot selection rerun 010
current latest card is MIRBUILDER-SUPPORTED-VALUE-EXPR-CLASSIFIER-HAKO-ADOPTION-DECISION-001
fastmem_access_plan_kind_label_formatter is HakoAdopted as the seventy-fifth parity pilot; next is pilot selection rerun 075
```

## Immediate Maintenance Slice

Status: closed for the Hakorune naming cleanup guard follow-up.

The completed guard follow-up is tracked in:

```text
docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md
```

Current state:

- all `HAKORUNE-*` naming cleanup slices in the task-order SSOT are landed;
- the only `active in this slice` entry is the always-on
  `NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001` guardrail;
- broad package/env/ABI renames remain out of scope for maintenance slices;
- PHI / LocalSSA / variable-map internals remain out of scope for naming
  cleanup.

Stage-term existing-name migration has a classification inventory at:

```text
docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md
```

The first classified stage-term migration slice is landed:

```text
STAGE-TERM-SYNTAX3-ALIAS-001
STAGE-TERM-SYNTAX3-DIAGNOSTIC-WORDING-001
STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001
STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001
STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001
STAGE-TERM-MODEA-COMPAT-ROUTE-WORDING-001
STAGE-TERM-MODEB-HHAKO-ENTRY-WORDING-001
STAGE-TERM-MODEB-HHAKO-COMPAT-FIXTURE-WORDING-001
STAGE-TERM-HHAKO-COMPILER-ROUTE-WORDING-001
STAGE-TERM-MODEB-HHAKO-HELPER-COMMENT-WORDING-001
STAGE-TERM-MODEB-CAPTURE-CALLER-GUARD-WORDING-001
STAGE-TERM-PHASE1-PROGRAM-JSON-GUARD-WORDING-001
STAGE-TERM-STAGE0-SHAPE-GATE-LABEL-WORDING-001
STAGE-TERM-MODEB-HHAKO-FUNC-SCANNER-COMMENT-WORDING-001
STAGE-TERM-MODEB-K2-WIDE-GUARD-DIAGNOSTIC-WORDING-001
STAGE-TERM-SELFHOST-SMOKE-COMMENT-WORDING-001
STAGE-TERM-CHECK-SCRIPTS-INDEX-WORDING-001
STAGE-TERM-SYNTAX3-RUST-ENV-COMMENT-WORDING-001
STAGE-TERM-HHAKO-PARSER-BUILD-COMMENT-WORDING-001
STAGE-TERM-JSON-V0-BRIDGE-COMMENT-WORDING-001
STAGE-TERM-HHAKO-BUILD-TEST-COMMENT-WORDING-001
STAGE-TERM-APP-BINARY-ONLY-SMOKE-COMMENT-WORDING-001
STAGE-TERM-APP-SMOKE-PHASE-COMMENT-WORDING-001
STAGE-TERM-STAGE0-CAPTURE-COMMENT-WORDING-001
STAGE-TERM-PIPELINE-V2-COMMENT-WORDING-001
STAGE-TERM-JOINIR-LOWERING-COMMENT-WORDING-001
STAGE-TERM-LANG-README-PHASE-WORDING-001
STAGE-TERM-DOCS-TOOLS-QUICK-ENTRY-WORDING-001
STAGE-TERM-STAGE1-BRIDGE-PHASE-COMMENT-WORDING-001
STAGE-TERM-ENV-REFERENCE-PHASE-WORDING-001
STAGE-TERM-RUST-STAGE1-ENV-HELPER-COMMENT-WORDING-001
STAGE-TERM-RUST-STAGE1-BOUNDARY-COMMENT-WORDING-001
STAGE-TERM-RUST-STAGE1-PROGRAM-JSON-TEST-WORDING-001
STAGE-TERM-CHECK-SCRIPTS-INDEX-PHASE-ENV-WORDING-001
STAGE-TERM-STAGE1-BRIDGE-ALIAS-COMMENT-WORDING-001
HAKORUNE-README-MODEB-USER-FACING-WORDING-001
HAKORUNE-README-MODEB-LINE-QUICKGUIDE-WORDING-001
HAKORUNE-REFERENCE-DOCS-CANONICALIZATION-DECISION-001
HAKORUNE-REFERENCE-DOCS-FIRST-CUT-001
HAKORUNE-REFERENCE-DOCS-ENTRY-INDEX-WORDING-001
HAKORUNE-REFERENCE-DOCS-INVARIANTS-CONSTRAINTS-WORDING-001
HAKORUNE-REFERENCE-DOCS-MIR-GC-WORDING-001
HAKORUNE-REFERENCE-DOCS-PLUGIN-INDEX-WORDING-001
HAKORUNE-REFERENCE-DOCS-STRINGS-BOXES-WORDING-001
HAKORUNE-REFERENCE-DOCS-ARCHITECTURE-OVERVIEW-WORDING-001
HAKORUNE-REFERENCE-DOCS-VM-GUIDE-TITLE-WORDING-001
HAKORUNE-REFERENCE-DOCS-RESIDUAL-NAMING-DEFER-INVENTORY-001
```

`--syntax-3` is now the frontend syntax-level spelling; `--stage3` remains a
compatibility alias. Live MIR builder hints now say `syntax-3` and `mode-B`
compatibility routes. Live env docs/comments now describe `STAGEB` names as
mode-B compatibility aliases without renaming those compatibility surfaces.
The explicit proof-only selfhost route docs/diagnostics now also say mode-B
compatibility while keeping `--stage-b`, `stageb-delegate`, and script names as
compatibility surfaces. Stage-1 bridge module payload comments/docs also say
mode-B compatibility while keeping `HAKO_STAGEB_*` env names and bridge file
names unchanged. Rust selfhost compat route comments/diagnostics now say
mode-A compatibility while keeping `stage-a-compat` runtime-mode aliases and
`stage_a_*` file/function names unchanged. HHako compiler entry comments now
say mode-B compatibility while keeping `StageB*` Box names, trace strings,
file names, and `--stage-b` route tokens unchanged. HHako legacy fixture and
adapter comments that defer authority to BuildBox now also say mode-B
compatibility while keeping env names, trace strings, and Box names unchanged.
`compiler.hako` route comments and the string-indexing diagnostic now say
mode-A/mode-B compatibility while keeping `stage_b`, trace strings, Box names,
and route tokens unchanged. HHako helper comments for driver guard, trace,
main/body detection, Rune helper, and user-box declaration scanner now also say
mode-B/mode-A compatibility while keeping `StageB*` Box names, trace strings,
env names, file names, and route tokens unchanged. The active Program(JSON)
capture caller guard comments/diagnostics and quick gate label now say mode-B
compatibility while keeping compatibility script names and allowed caller
surfaces unchanged. Active Stage1 Program(JSON) guard comments/diagnostics and
quick gate labels now say phase-1 compatibility while keeping script names,
fixture names, and helper symbols unchanged. The Stage0-named shape inventory
script now appears in quick gate output as `GlobalCallTarget shape inventory
guard` while keeping script and inventory doc paths unchanged. FuncScanner
comments now say mode-B compatibility while keeping PHI / LocalSSA /
variable-map internals untouched. Two active K2-wide guard diagnostics now say
mode-B compatibility while keeping script names, StageB Box names, and guard
logic unchanged. Active selfhost smoke comments and human-facing diagnostics
now say mode-B compatibility, phase-1 compatibility, or syntax-3 while keeping
smoke file names, compatibility route tokens, exact expected stderr, and
StageB/Stage1 Box names unchanged. The check-scripts index rows for
already-migrated active guards now also use mode-B / phase-1 / GlobalCallTarget
wording while keeping guard script names unchanged. Rust env comments now say
syntax-3 for the parser surface while keeping `stage3` feature tokens and env
names as compatibility surfaces. HHako parser/build/MIR builder comments now
also use mode-A/mode-B compatibility or syntax-3 wording while keeping `stage3`
fields/functions, trace strings, file names, and PHI/LocalSSA/variable-map
internals untouched. Rust JSON v0 bridge comments and one local freeze
diagnostic now use mode-B compatibility / bootstrap wording while keeping
`try_lower_stageb_*` names, route symbols, and behavior unchanged. Remaining
HHako build/test comments now also use bootstrap / mode-B compatibility wording
while keeping `stageb_*` test file names and StageB Box names unchanged. The
binary-only app selfhost readiness smoke comments now use phase-1 / phase-2
proxy wording while keeping pass names and `stage1.mir` / `stage2.mir`
artifact filenames unchanged. Remaining app smoke comments for binary-only
run, no-compat mainline, perf split, and shared app helpers now use phase-1 or
mode-A compatibility wording while keeping `stage1-cli`, `stage-a-compat`,
`stage3`, and artifact filenames unchanged. Stage0-named capture helper
comments now say bootstrap capture while keeping `stage0_capture*` file,
function, and test names unchanged. Pipeline V2 comments now use phase-1 /
phase-2 / phase-3, syntax-3, or mode-B compatibility wording while keeping
`Stage1*` Box names, `stage1_*` file/module names, and `lower_stage1_*` APIs
unchanged. JoinIR lowering comments now use lower-resolver, phase-1
compatibility, or mode-B compatibility wording while keeping `stage1_*`,
`stageb_*`, `Stage1*`, and `StageB*` compatibility identifiers unchanged.
`lang/README.md` now uses phase-1 / K2+ distribution wording while keeping
`build_stage1.sh`, `target/selfhost/hakorune`, and legacy `stage0`/`stage1`
boundary references explicit compatibility vocabulary. `docs/tools` quick
entries now use phase-1 / syntax-3 wording while preserving compatibility
script names and not rewriting the historical guard ledger. Stage1 bridge
README and file-header comments now use phase-1 compatibility wording while
preserving `stage1_*`, `Stage1*`, env names, log tags, and expected stderr.
`docs/reference/environment-variables.md` now uses phase-1 compatibility,
syntax-3, and bootstrap wording for the active environment variable reference
while preserving `NYASH_STAGE1_*`, `STAGE1_*`, `NYASH_FEATURES=stage3`, and
`NYASH_NY_COMPILER_STAGE3` compatibility names.
`src/config/env/stage1.rs` helper comments now use phase-1 compatibility
wording while preserving the `stage1` module path, env names, helper function
names, and behavior.
`src/stage1` boundary README and Program(JSON v0) header comments now use
phase-1 compatibility wording while preserving the `src/stage1` path,
`stage1_bridge` helper names, and explicit legacy Stage1/Stage2 artifact
label explanation.
Selected `src/stage1/program_json_v0` assertion/expectation messages now use
phase-1 compatibility wording while preserving test function names, fixture
file names, helper function names, and `Stage1*` Box names.
Selected active `docs/tools/check-scripts-index.md` rows now use phase-1
compatibility / bootstrap wording for selfhost surface, NyRT env P0, and
cleanup/catch boundary guard descriptions while preserving script names.
Selected `src/runner/stage1_bridge` env/module alias comments now use phase-1
compatibility CLI / alias wording while preserving module names, types, env
names, and `[stage1-cli]` log tags.
The root README developer quickstart now says mode-B compatibility for the
current MIR emit helper route while preserving `nyash` syntax fences,
`NYASH_*` compatibility env names, and `ny-llvmc` tool names.
The remaining root README ny-llvm line quickstart label now also says mode-B
compatibility while preserving the compatibility guide path and `ny-llvmc`
tool name.
Reference docs now have a Hakorune-first canonicalization decision, and the
first entry-doc cut updates quick-reference, core-language README, and PHI/SSA
architecture titles/phrasing while preserving env, ABI, historical, and
compatibility names.
Reference docs entry/index wording now also covers docs/reference/README,
language/README, and EBNF titles/support-profile pointers while preserving
stage-profiles.md as the compatibility support manual path.
Reference invariants and constraints spec titles now also use Hakorune-first
wording while preserving `NYASH_*` compatibility env names and internal
compatibility identifiers in the entries.
Reference MIR instruction-set and runtime GC docs now also use Hakorune-first
titles/intro wording while preserving `NYASH_*`, `nyash.toml`, NyRT, and ABI
compatibility names.
The plugin-system reference index title and core-developer heading now use
Hakorune-first wording while preserving `nyash.toml`, `nyash_box.toml`,
plugin package paths, and repository compatibility names.
Reference strings docs now use Hakorune-first wording, and the Box-system index
title now marks the folder as Hakorune historical documentation while preserving
Nyash-era historical text and compatibility names.
The architecture overview title/opening now mark it as a Hakorune historical
Nyash design snapshot while preserving runtime type names and older design
content.
The VM guide title now uses Hakorune-first wording while preserving `nyash`
CLI examples and `NYASH_*` compatibility environment names.
Remaining `docs/reference` Nyash/Stage headings are now classified as
ABI/compatibility, historical Box/plugin/parser material, or stage-profile
policy surfaces; do not rewrite them without a dedicated decision slice.
Next safe naming work, if explicitly selected, must pick a
different classified layer from that inventory and keep compatibility aliases
or replacement routes in the same slice.

## Latest MirBuilder Hako-Native Parity Pilot

Latest adopted owner:

```text
MIRBUILDER-ARRAY-RMW-WINDOW-PROOF-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

Status:

```text
array_rmw_window_proof_label_formatter is HakoAdopted as the twenty-fifth
narrow Rust-oracle parity pilot after a green 1-row `.hako` EXE parity gate.
Source Selfhost remains unclaimed.
```

Boundary:

```text
Adopted:
  constructor call route label vocabulary formatting
  planner rule tag/display/route label vocabulary formatting
  loop legacy observer deny reason/owner label vocabulary formatting
  array RMW window proof label vocabulary formatting

Still Rust:
  constructor route collection
  callee classification
  planner order
  planner rule selection
  legacy observer shadow decision
  loop route candidate collection
  runtime route selection
  array RMW window matching
  array receiver proof
  backend lowering
  MIR mutation
```

Next:

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-025
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
