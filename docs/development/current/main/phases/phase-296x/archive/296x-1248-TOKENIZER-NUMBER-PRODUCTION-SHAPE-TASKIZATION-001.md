---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Taskize the tokenizer/scanner NUMBER production shape blocker and keep compiler Recipe/CorePlan acceptance separate from the rust-subset app-front shape lane.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1244-COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1247-JSON-NATIVE-TOKEN-TEXT-PAYLOAD-STORAGE-PROBE-001.md
  - apps/rust-subset-to-hako/STATUS.md
  - apps/lib/json_native/lexer/scanner.hako
  - apps/lib/json_native/lexer/tokenizer.hako
---

# TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001

## Decision

Keep the accepted token-payload stability route active while taskizing the
stronger tokenizer/scanner NUMBER production shape.

```text
active_stability_route=token_payload_plus_small_number_materializer
nonzero_number_blocker_remains=1
bool_return_validation_must_not_be_blocked=1
compiler_recipe_backlog_separate=1
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
```

The stronger `read_next_number_literal()` / `read_number()` family should be
accepted structurally, but not by restoring WIP parser code or adding by-name
special cases.

## Why This Row Exists

The minimal staged loop/break compiler canary is already green under
`planner_required`:

```text
LoopSimpleWhile + flowbox/adopt break
```

That result proves the small captured shape is accepted. It does not yet prove
that every scanner/tokenizer production shape used by `json_native` is accepted
by the current EXE/AOT backend recipe path.

The direct `JsonToken` storage probe is green:

```text
dynamic substring -> JsonToken("NUMBER", dynamic_text, ...) -> ArrayBox -> get_value()
```

The tokenizer-number probe is not yet a stable owner signal because it fails at
compile shape:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
observed=unsupported pure shape for current backend recipe
```

Therefore the next work must split:

```text
1. keep current stable token payload route
2. capture smaller production-shape probes
3. only then decide whether Recipe/CorePlan needs new acceptance
```

## Task Ladder

### 1. JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001

Purpose:

```text
Probe JsonScanner.read_number() directly without JsonTokenizer.tokenize().
```

Acceptance:

```text
scanner_read_number_probe_exists=1
scanner_read_number_exe_aot_green=<0|1>
tokenizer_array_publication_involved=0
implementation_allowed=0
```

Interpretation:

```text
green:
  scanner substring production is not the current owner

red_compile_shape:
  owner moves to scanner read_number production shape

red_payload:
  owner moves to scanner-derived substring publication/materialization
```

### 2. JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001

Purpose:

```text
Probe JsonTokenizer.next_token() for a single NUMBER without tokenizing the
whole stream into the tokenizer token ArrayBox.
```

Acceptance:

```text
next_token_number_probe_exists=1
next_token_number_exe_aot_green=<0|1>
tokenizer_token_array_publication_involved=0
implementation_allowed=0
```

Interpretation:

```text
green:
  tokenize() loop / token ArrayBox publication is the next suspect

red_compile_shape:
  owner moves to tokenize_number / next_token backend recipe shape

red_payload:
  owner moves to tokenize_number stable_number materialization
```

### 3. JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001

Purpose:

```text
Inventory the exact compile-shape blocker for tokenize()->ArrayBox NUMBER
production, using the existing investigation probe as evidence.
```

Output:

```text
first_reject_owner=<pure_backend_recipe|route_plan|object_publication|unknown>
first_reject_shape=<shape token>
implementation_allowed=0
```

### 4. COREPLAN-SCANNER-PRODUCTION-SHAPE-RECIPE-ACCEPTANCE-001

Open only if the previous probes prove a compiler acceptance owner.

Purpose:

```text
Accept the smallest scanner/tokenizer production shape through Recipe/CorePlan.
```

Non-goals:

```text
do not add read_next_number_literal by-name branches
do not widen RustSubset app-front shape support
do not change converter_core.hako
do not expand JsonNumberTextMaterializer as a fake fix
```

### 5. JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001

Open only after arbitrary scanner-derived NUMBER payloads survive the accepted
route.

Purpose:

```text
Remove the small numeric dictionary materializer and keep JSON number payload
stability through the real scanner/tokenizer path.
```

## Other Known Not-Yet-Accepted / Not-Yet-Stable Shapes

Keep these visible, but do not mix them into the NUMBER production row:

```text
compiler_acceptance:
  continue inside staged loop
  nested loop break/continue interactions beyond the captured canary
  loop-carried PHI scanner shapes exposed by real parser bodies
  tokenizer tokenize() loop with EOF/error break if it differs from the canary

rust_subset_app_front:
  else-if source spelling
  returnless void function body hardening
  Vec method calls such as push/len/get
  Rust match / trait / generic items remain unsupported handoff unless selected

json_native_hardening:
  critical key materializer retire
  small number materializer retire
  FileBox/smoke serialization guard
```

## Stop Lines

```text
do not restore read_next_number_literal WIP before the production-shape probes are classified
do not claim all loop/break shapes are accepted from the minimal canary alone
do not expand the number materializer dictionary
do not use function or method names as compiler acceptance proof
do not mix RustSubset source-shape support with Recipe/CorePlan acceptance
do not run smoke/regression commands that rebuild libhako_llvmc_ffi.so in parallel
```

## Contract

```text
output_contract=tokenizer-number-production-shape-taskization-v0

token_payload_stability_route_preserved=1
nonzero_number_blocker_remains=1
bool_return_validation_unblocked=1
production_shape_probe_ladder_recorded=1
recursive_recipe_direction_recorded=1
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
next_task=JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001

summary=ok
```
