# Phase 163x: primitive and user-box fast path

- Status: Active
- 目的: primitive semantic builtin family と user-box field access を current implementation lane として進め、`.hako` surface を変えずに compiler/MIR 主導の typed fast path を広げる。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
  - `src/mir/storage_class.rs`
  - `src/mir/instruction.rs`
  - `src/llvm_py/instructions/binop.py`
  - `src/llvm_py/instructions/compare.py`

## Decision Now

- this is the current implementation lane
- `phase-137x` stays active as string guardrail / borrowed-corridor validation lane
- `field_decls` is the typed authority
- names-only `fields` stays as compatibility mirror only
- `sink` stays landed in the string lane; do not delete it here
- do not add new `.hako` syntax or widen `@rune`
- do not start with flattening

## Restart Handoff

- design owner:
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- sibling guardrail lane:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- landed upstream facts:
  - typed `field_decls` now survive `.hako parser -> AST -> Stage1 Program JSON -> MIR metadata -> MIR JSON`
  - canonical MIR now has `FieldGet` / `FieldSet`
  - `FieldGet.declared_type` now seeds `value_types` and `StorageClass`
  - LLVM lowering now has the first typed primitive pilot for `IntegerBox` / `BoolBox` via `nyash.integer.get_h` / `nyash.bool.get_h`
- fixed authority:
  - `field_decls` = source of truth for typed field declarations
  - `fields` = names-only compatibility mirror for old payloads and old runtime consumers
- current next cut:
  1. add a narrow user-box local perf gate: `kilo_micro_userbox_point_add`
  2. pilot typed user-box field access on the internal path
  3. validate with the new user-box gate while keeping string split pack green

## Fixed Task Order

1. keep `field_decls` as authority and stop treating names-only `fields` as design truth
2. add the user-box local micro before wider typed lowering
3. pilot typed user-box field access only for `declared_type = IntegerBox | BoolBox`
4. keep plugin / reflection / ABI / weak-field paths on generic fallback
5. do not reopen flattening until typed user-box access has a keeper

## Guardrails

- `tools/perf/build_perf_release.sh` stays mandatory before perf/asm probes
- string split pack remains guardrail:
  - `kilo_micro_substring_only`
  - `kilo_micro_substring_views_only`
  - `kilo_micro_len_substring_views`
- any typed user-box slice must not silently redefine string lane ownership or restart order
