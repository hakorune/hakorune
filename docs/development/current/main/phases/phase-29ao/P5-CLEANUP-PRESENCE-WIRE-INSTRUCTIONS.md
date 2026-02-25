---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
  - docs/development/current/main/design/effect-classification-ssot.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P5: Cleanup presence → ExitKind vocabulary（未接続・仕様不変）

Date: 2025-12-30  
Status: Ready for execution  
Scope: `cleanup_kinds_present` を “ExitKind 語彙” として扱える形へ投影し、将来の cleanup wiring を壊れにくくする

## 目的

- SSOT（`exitkind-cleanup-effect-contract-ssot.md`）どおり、cleanup を “pattern 個別” に散らさず **ExitKind 語彙として扱う**。
- Phase 29ao P4 で `exit_kinds_present → Frag.exits` は語彙として固定できたので、P5 は
  `cleanup_kinds_present → (必要な ExitKind) → Frag.exits` の “presence 投影” を追加し、次の wiring（P6以降）の土台を作る。

## 非目的

- 既存ルーティング/挙動/ログ/エラー文字列の変更（P5 は未接続）
- cleanup 命令（ReleaseStrong 等）の挿入や移動（RC insertion など別フェーズ）
- 未配線 exits を emit する（今は `emit_frag()` が `frag.exits` を使わないので安全に語彙だけ先行できる）

## 設計（SSOTの要点）

- cleanup は `Effect=Mut` として扱い、DCE/CSE/再順序で壊れない境界を守る（実装は P6+）
- cleanup は Exit 境界（return/break/continue の直前/直後）に限定（実装は P6+）
- いまは “presence（存在情報）だけ” を ExitKind 語彙へ投影する

## 実装手順

### Step 1: cleanup presence を ExitKindFacts に写像する（純関数）

対象:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

追加（案）:

- `fn exit_kind_facts_from_cleanup_kind(kind: CleanupKindFacts) -> ExitKindFacts`
- `fn union_exit_kinds(exit_kinds_present: &BTreeSet<ExitKindFacts>, cleanup_kinds_present: &BTreeSet<CleanupKindFacts>) -> BTreeSet<ExitKindFacts>`

写像:
- `CleanupKindFacts::{Return,Break,Continue}` → `ExitKindFacts::{Return,Break,Continue}`

### Step 2: Frag.exits の生成を “Exit + Cleanup” の union に変更

対象:
- `build_exitmap_from_presence(...)`（P4 で追加済みの exits ビルダ）

変更:
- これまで: `exit_kinds_present` だけを基に `Frag.exits` を作っていた
- これから: `union_exit_kinds(exit_kinds_present, cleanup_kinds_present)` を基に `Frag.exits` を作る

注意:
- P4 と同様、stub は `EdgeStub::without_args(from, kind)` で **target=None** のまま
- `LoopId` の取り扱いは P4 と同じ（未接続なので `LoopId(0)` で OK）

### Step 3: unit test（“cleanup が ExitKind 語彙に属する” を固定）

対象（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`（または `plan/composer/mod.rs`）

追加テスト:
- `cleanup_kinds_present` が空 → exits は P4 と同じ（回帰）
- `cleanup_kinds_present={Return}` のとき、`Frag.exits` に `ExitKind::Return` が必ず含まれる（stub.target=None）

注:
- 実動作では `cleanup_kinds_present` はまだ未接続が多いので、このテストは “語彙の固定” が目的。

### Step 4: docs 更新

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P5 指示書リンク + 完了後の Next を P6 へ）
- `docs/development/current/main/10-Now.md`（Next の更新）
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p5): project cleanup presence into exitkind vocabulary (unconnected)"`

## 次（P6）

P6 は `value_join_needed`（post-phi SSOT）を CorePlan/Frag 側へ寄せて、局所 verify を強化する。
