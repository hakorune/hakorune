# Recipe File Naming Unification SSOT

**Purpose**: Pattern 数字ベースの file naming から、意味ベース（Recipe/Lego 語彙）への移行結果を固定する。

**Status**: Historical mapping ledger (implemented 2026-01-30)
**Last updated**: 2026-03-07

---

## Scope

Note:
- Current canonical file names are already implemented.
- This document is kept as a historical mapping ledger for traceability, not as an active rename plan.

### In Scope

- `src/mir/builder/control_flow/plan/recipe_tree/*_builder.rs`
- `src/mir/builder/control_flow/plan/recipe_tree/*_composer.rs`（語彙統一のみ）

### Out of Scope

- ログ/TSV/エラーメッセージの文言
- Facts/Features/Normalizer など recipe_tree 外のファイル名
- historical planner-payload-era 型名・ファイル名

---

## Naming Rules (Historical Decision)

1. **Pattern 数字を排除**: `pattern1_` などの接頭辞を使わない。
2. **Recipe 名をベースにする**: `LoopSimpleWhileRecipe` → `loop_simple_while_*`。
3. **Suffix を固定**:
   - Builder: `*_builder.rs`
   - Composer: `*_composer.rs`
4. **語彙の一貫性**:
   - `ifphi` → `if_phi_join`
   - `continue` → `continue_only`
   - `infinite_early_exit` → `true_early_exit`

---

## Mapping (Historical -> Current)

### Builder files

| Historical | Current |
|---------|-----|
| `pattern1_simple_while_builder.rs` | `loop_simple_while_builder.rs` |
| `pattern1_char_map_builder.rs` | `char_map_builder.rs` |
| `pattern1_array_join_builder.rs` | `array_join_builder.rs` |
| `pattern2_break_builder.rs` | `loop_break_builder.rs` |
| `pattern3_ifphi_builder.rs` | `if_phi_join_builder.rs` |
| `pattern4_continue_builder.rs` | `loop_continue_only_builder.rs` |
| `pattern5_infinite_early_exit_builder.rs` | `loop_true_early_exit_builder.rs` |
| `pattern6_scan_with_init_builder.rs` | `scan_with_init_builder.rs` |
| `pattern7_split_scan_builder.rs` | `split_scan_builder.rs` |
| `pattern8_bool_predicate_scan_builder.rs` | `bool_predicate_scan_builder.rs` |
| `pattern9_accum_const_loop_builder.rs` | `accum_const_loop_builder.rs` |

### Composer files

| Historical | Current |
|---------|-----|
| `loop_break_recipe_composer.rs` | `loop_break_composer.rs` |

---

## Migration Record

### Phase 1: Builder rename (11 files)

- Use `git mv` to rename each builder file to the new name.
- Update `recipe_tree/mod.rs` module declarations.
- Update all `use` paths in `recipe_tree/*` and external modules.
- Commit rule: **1 recipe = 1 commit** (small diffs, easy rollback).

### Phase 2: Composer rename (1 file)

- Rename `loop_break_recipe_composer.rs` → `loop_break_composer.rs`.
- Update module declaration and all `use` paths.
- Commit rule: **1 file = 1 commit**.

### Phase 3: Cleanup checks

- Verify no `pattern[1-9]_` builder filenames remain.
- Build verification: `cargo build --release --bin hakorune`.

---

## Acceptance Criteria

- `find src/mir/builder/control_flow/plan/recipe_tree -name 'pattern*_builder.rs'` is empty.
- `rg -n "pattern[1-9]_.*_builder" src/` returns no hits.
- Build passes with no new warnings.

Current reading rule:
- For runtime/current architecture, prefer `recipe-tree-and-parts-ssot.md`.
- Use this document only when a historical file rename mapping is needed.

---

## References

- `docs/development/current/main/design/pattern-naming-migration-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`

---

## Changelog

| Date | Change |
|------|--------|
| 2026-01-30 | ✅ Phase 1 complete: All 11 builder files renamed (1-1 through 1-11). |
| 2026-01-30 | ✅ Phase 2 complete: Composer file renamed (`loop_break_recipe_composer` → `loop_break_composer`). |
| 2026-01-30 | ✅ Phase 3 complete: Verification and SSOT documentation updated. |
| 2026-01-29 | Initial design SSOT (no implementation). |
