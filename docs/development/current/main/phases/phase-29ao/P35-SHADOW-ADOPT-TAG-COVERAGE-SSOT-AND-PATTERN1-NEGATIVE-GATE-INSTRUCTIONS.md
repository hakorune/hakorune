---
Status: Ready
Scope: strict/dev の shadow adopt 観測を SSOT 化し、negative ケースを回帰で固定する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P35: Shadow-adopt tag coverage SSOT + Pattern1 negative gate

## 目的

- strict/dev の shadow adopt は「通った/通ってない」が出力で観測できるが、回帰で “どの smoke が何を保証するか” が曖昧になりやすい。
- P35 は「タグの必須/禁止」の責務と対応 smoke を SSOT として固定し、抜けを 1 件（Pattern1 subset reject）埋める。

## 非目的

- release の既定挙動・エラー文字列・恒常ログの変更
- 新しい環境変数追加
- by-name のパターン名分岐追加

## 成果物

1. SSOT 追加（表形式）
   - `docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md`
2. negative gate 追加（Pattern1 subset reject）
   - `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh`
3. 参照導線の追記（任意）
   - `docs/development/current/main/design/planfrag-ssot-registry.md`

## 実装手順

### Step 1: coverage SSOT を追加

- 新規作成:
  - `docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md`
- 内容:
  - “必須タグ / 禁止タグ / 対応 smoke / raw output 参照の理由” を 1 枚にまとめる

### Step 2: Pattern1 subset reject を negative gate にする

- 変更:
  - `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh`
- 追加するチェック:
  - strict/dev 実行の raw output に `[coreplan/shadow_adopt:pattern1_simplewhile]` が出たら FAIL
  - 既存の exit code=3 期待は維持（副作用 drop 誤マッチ検出の本体）

### Step 3: SSOT registry に参照追加（任意）

- 更新:
  - `docs/development/current/main/design/planfrag-ssot-registry.md`
- `References` に coverage SSOT を 1 行追加

### Step 4: Phase 29ao の進捗を更新

- 更新:
  - `docs/development/current/main/phases/phase-29ao/README.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/30-Backlog.md`
  - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Next のみ）

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p35): ssot shadow-adopt tag coverage and pattern1 negative gate"`
