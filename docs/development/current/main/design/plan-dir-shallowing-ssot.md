# plan/ ディレクトリ浅層化設計 SSOT

## Scope

`src/mir/builder/control_flow/plan/` (440ファイル) のディレクトリ構造を2階層以内に収める設計。

**実装は別フェーズ** - このドキュメントは設計のみ。

注記: 本文で出る `PatternX` は historical/filename 用の legacy label。説明文は route/semantic 名を優先する。

---

## Goal

- 現在の3-5階層を**2階層以内**に収める
- 深い階層の「塊」をフラット化
- モジュール名の衝突を回避

---

## Non-goals

- ファイルの数を減らすこと（移動のみ）
- モジュールの責務を変更すること
- パブリックAPIを変更すること

---

## Current Layout (深い階層の代表例)

### 深さ4の例: break-on-condition facts ルート（legacy label: Pattern2）

```
facts/pattern2_break_facts/
  ├── core.rs
  ├── helpers.rs
  ├── mod.rs
  ├── pattern2_break_loopbodylocal.rs
  ├── pattern2_break_parse_integer.rs
  ├── pattern2_break_read_digits.rs
  ├── pattern2_break_realworld.rs
  ├── pattern2_break_step_before_break.rs
  ├── pattern2_break_trim_whitespace.rs
  ├── tests.rs
  └── types.rs
```

**→ 提案**: break-on-condition facts を `facts/pattern2_break_*.rs` へ集約 (12ファイルをフラット化、legacy label: Pattern2)

### 深さ4の例: features/loop_cond_break_continue_pipeline/item_lowering/

```
features/loop_cond_break_continue_pipeline/item_lowering/
  ├── item.rs
  ├── mod.rs
  ├── stmt.rs
  └── util.rs
```

**→ 提案**: `features/loop_cond_bc_item_*.rs` (4ファイルをフラット化)

### 深さ4の例: loop_cond_unified/variants/break_continue/

```
loop_cond_unified/variants/break_continue/
  ├── accept_kind.rs
  ├── classify.rs
  ├── entry.rs
  ├── facts.rs
  ├── helpers.rs
  ├── item.rs
  ├── mod.rs
  ├── recipe.rs
  ├── tree.rs
  ├── types.rs
  └── validators/
      ├── conditional_update.rs
      ├── else_only.rs
      ├── exit_if.rs
      ├── mod.rs
      └── prelude.rs
```

**→ 提案**: `loop_cond/break_continue_*.rs` (16ファイルをフラット化)

### 深さ3の例: generic_loop/facts/body_check/

```
generic_loop/facts/body_check/
  ├── expr_matchers.rs
  ├── extractors.rs
  ├── mod.rs
  ├── shape_detectors.rs
  ├── shape_resolution.rs
  └── tests.rs
```

**→ 提案**: `generic_loop/body_check_*.rs` (6ファイルをフラット化)

### 深さ3の例: normalizer/cond_lowering/

```
normalizer/cond_lowering/
  ├── entry.rs
  ├── freshen.rs
  ├── if_plan.rs
  ├── loop_header.rs
  ├── mod.rs
  ├── prelude.rs
  └── value_expr.rs
```

**→ 提案**: `normalizer/cond_lowering_*.rs` (8ファイルをフラット化)

### 深さ3の例: recipe_tree/builders/ と composer/（route別 builder/composer、legacy labels: Pattern1-9）

```
recipe_tree/builders/
  ├── mod.rs
  ├── pattern1_array_join.rs
  ├── pattern1_char_map.rs
  ├── pattern1_simple_while.rs
  ├── pattern2_break.rs
  ├── pattern3_ifphi.rs
  ├── pattern4_continue.rs
  ├── pattern5_infinite_early_exit.rs
  ├── pattern6_scan_with_init.rs
  ├── pattern7_split_scan.rs
  ├── pattern8_bool_predicate_scan.rs
  └── pattern9_accum_const_loop.rs

recipe_tree/composer/
  ├── accum_const_loop.rs
  ├── bool_predicate_scan.rs
  ├── generic_loop.rs
  ├── if_phi_join.rs
  ├── loop_break_recipe.rs
  ├── loop_cond.rs
  ├── loop_continue_only.rs
  ├── loop_simple_while.rs
  ├── loop_true.rs
  ├── loop_true_early_exit.rs
  ├── mod.rs
  ├── scan_with_init.rs
  └── split_scan.rs
```

**→ 提案**: route別 builder/composer を `recipe_tree/pattern*_builder.rs` と `recipe_tree/pattern*_composer.rs` へ集約 (23ファイルをフラット化、legacy labels: Pattern1-9)

### 深さ3の例: composer/coreloop_v0/ と coreloop_v1/

```
composer/coreloop_v0/
  ├── mod.rs
  └── tests.rs

composer/coreloop_v1/
  ├── mod.rs
  └── tests.rs
```

**→ 提案**: `composer/coreloop_v0_*.rs` と `composer/coreloop_v1_*.rs` (6ファイルをフラット化)

---

## Proposed Layout (2階層以内)

### カテゴリ別フラット化案

#### 1. facts/ (60+ファイル → 1階層)

**現在**:
```
facts/pattern2_break_facts/*.rs (12ファイル)
facts/loop_facts/*.rs (9ファイル)
facts/expr/*.rs (5ファイル)
facts/pattern*_facts.rs (20+ファイル)
```

**提案**:
```
facts/pattern2_break_*.rs (12ファイル)
facts/loop_*.rs (9ファイル)
facts/expr_*.rs (5ファイル)
facts/pattern*_facts.rs (既存維持)
```

**命名規則**: `facts/<category>_<module>.rs`

#### 2. features/ (57ファイル → 1階層)

**現在**:
```
features/loop_cond_break_continue_pipeline/*.rs (14ファイル)
features/loop_cond_continue_only_pipeline/*.rs (9ファイル)
features/coreloop_skeleton/*.rs (2ファイル)
features/steps/*.rs (2ファイル)
features/*_pipeline.rs (10+ファイル)
```

**提案**:
```
features/loop_cond_bc_*.rs (14ファイル)
features/loop_cond_continue_only_*.rs (9ファイル)
features/coreloop_skeleton_*.rs (2ファイル)
features/steps_*.rs (2ファイル)
features/*_pipeline.rs (既存維持)
```

**命名規則**: `features/<pipeline>_<component>.rs`

#### 3. loop_cond_unified/ (29ファイル → 1階層)

**現在**:
```
loop_cond_unified/variants/break_continue/*.rs (16ファイル)
loop_cond_unified/variants/continue_only/*.rs (3ファイル)
loop_cond_unified/variants/continue_with_return/*.rs (3ファイル)
loop_cond_unified/variants/return_in_body/*.rs (3ファイル)
```

**提案**:
```
loop_cond/break_continue_*.rs (16ファイル)
loop_cond/continue_only_*.rs (3ファイル)
loop_cond/continue_with_return_*.rs (3ファイル)
loop_cond/return_in_body_*.rs (3ファイル)
loop_cond/true_break_continue.rs
```

**命名規則**: `loop_cond/<variant>_<component>.rs`

#### 4. generic_loop/ (14ファイル → 1階層)

**現在**:
```
generic_loop/facts/body_check/*.rs (6ファイル)
generic_loop/facts/*.rs (7ファイル)
generic_loop/normalizer.rs
```

**提案**:
```
generic_loop/body_check_*.rs (6ファイル)
generic_loop/facts_*.rs (7ファイル)
generic_loop/normalizer.rs
```

**命名規則**: `generic_loop/<component>_<module>.rs`

#### 5. composer/ (13ファイル → 1階層)

**現在**:
```
composer/coreloop_v0/*.rs (3ファイル)
composer/coreloop_v1/*.rs (3ファイル)
composer/*.rs (7ファイル)
```

**提案**:
```
composer/coreloop_v0_*.rs (3ファイル)
composer/coreloop_v1_*.rs (3ファイル)
composer/*.rs (既存維持)
```

**命名規則**: `composer/<version>_<component>.rs`

#### 6. recipe_tree/ (28ファイル → 1階層; route別 builder/composer)

**現在**:
```
recipe_tree/builders/pattern*.rs (11ファイル)
recipe_tree/composer/*.rs (12ファイル)
recipe_tree/*.rs (5ファイル)
```

**提案**:
```
recipe_tree/pattern*_builder.rs (11ファイル)
recipe_tree/pattern*_composer.rs (12ファイル)
recipe_tree/*.rs (既存維持)
```

**命名規則**: `recipe_tree/<pattern>_<role>.rs`（`<pattern>` は legacy label。意味は route名で管理）

#### 7. normalizer/ (24ファイル → 1階層)

**現在**:
```
normalizer/cond_lowering/*.rs (8ファイル)
normalizer/pattern*.rs (16ファイル)
```

**提案**:
```
normalizer/cond_lowering_*.rs (8ファイル)
normalizer/pattern*.rs (既存維持)
```

**命名規則**: `normalizer/<component>_<module>.rs`

#### 8. loop_*/ (各5ファイル → 1階層)

**現在**:
```
loop_*_v0/{facts, mod, pipeline, recipe}.rs
```

**提案**:
```
loop_*_v0_facts.rs
loop_*_v0_mod.rs
loop_*_v0_pipeline.rs
loop_*_v0_recipe.rs
```

**命名規則**: `loop_<variant>_<version>_<component>.rs`

---

## Migration Rules

### 1. 挙動不変

- ファイル移動のみ、内容は変更しない
- `mod` 宣言のみ更新
- `use` パスは移動後に一括更新

### 2. 1カテゴリ=1コミット

例:
- facts/ のフラット化 (1コミット)
- features/ のフラット化 (1コミット)
- loop_cond_unified/ のフラット化 (1コミット)

### 3. 入口/SSOT の README 追加

各カテゴリの `mod.rs` にコメント追加:
```rust
// category: Facts extraction
// shallowing: moved from subdirs (see design SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md)
```

### 4. モジュール名衝突回避

- 同名ファイルが存在する場合は接頭辞を追加
- 例: `classifiers.rs` → `expr_classifiers.rs`, `loop_classifiers.rs`

---

## Risks

### 1. 参照パス変更

**リスク**: `use` パスの大量更新が必要

**対策**:
- 移動後に `rg` で一括置換
- コンパイルエラーで漏れを検出

### 2. モジュール名衝突

**リスク**: フラット化で同名ファイルが衝突

**対策**:
- 事前に `sort | uniq -d` で衝突検出
- 接頭辞/接尾辞で一意化

### 3. 歴史的経緯の消失

**リスク**: ディレクトリ構造から意図が読めなくなる

**対策**:
- ファイル名で route 意味を明示（必要時は legacy label: PatternX を併記）
- 各カテゴリの `mod.rs` にコメントを追加

---

## Out-of-scope

- 実装は別フェーズ
- ファイル内容の変更
- APIの変更
- テストの変更（移動のみ）

---

## 次のステップ（実装フェーズ）

1. カテゴリを1つ選択 (例: facts/)
2. 衝突チェック (`find ... -name "*.rs" | xargs -n1 basename | sort | uniq -d`)
3. ファイル移動 + `mod` 宣言更新
4. `use` パス一括更新 (`rg` + sed)
5. コンパイル確認 (`cargo build --release`)
6. コミット
7. 次のカテゴリへ

---

## 参考ファイル

- 現行一覧: `find src/mir/builder/control_flow/plan -type f -name "*.rs" | sort`
- 設計SSOT: 本ファイル
- 移行計画: 本ファイルの Proposed Layout セクション
