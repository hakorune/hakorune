# Phase 29ai P10: Move loop_break extraction to plan layer（historical instruction SSOT）

Date: 2025-12-29
Status: Historical reference (implemented)
Scope: loop_break route（break / body-local / promotion）抽出の SSOT を plan 層へ移した migration slice（仕様不変）
Goal: JoinIR 側の route-specific extraction knowledge を削減し、依存方向を `joinir -> plan` の一方向に固定する

Historical note:
- P10 は `route_entry` / `loop_route_detection` / recipe-first cleanup より前の instruction だよ。
- 以下に出てくる `DomainPlan::Pattern2Break`, `joinir/patterns/extractors/pattern2.rs`,
  `single_planner/legacy_rules/pattern2.rs` は 2025-12 時点の migration-time token で、current runtime API ではないよ。

## Objective

loop_break route の migration-time payload（`DomainPlan::Pattern2Break`）を
`joinir/patterns/extractors/*` から `plan/extractors/*` に移し、JoinIR 側は薄い wrapper
（re-export）に縮退させる。`single_planner` の legacy_rules も plan 側 extractor を参照するように統一する。

この P10 は “移設だけ” を目的にし、抽出ロジックの意味論は変えない（pure extraction のまま）。

## Non-goals（この P10 ではやらない）

- Facts→Planner へ loop_break route を吸収する（P11+）
- promotion policy / NotApplicable / Freeze など契約内容の変更
- 新しい fixture/smoke 追加（既存回帰で担保）
- env var / デバッグトグル追加
- by-name ディスパッチの追加（禁止）

## Historical State At Execution Time

- loop_break extractor（JoinIR 側 / historical Pattern2 label）:
  - `src/mir/builder/control_flow/joinir/patterns/extractors/pattern2.rs`
  - `-> Result<Option<DomainPlan>, String>` で `DomainPlan::Pattern2Break(Pattern2BreakPlan { ... })` を返す
- single_planner legacy:
  - `src/mir/builder/control_flow/plan/single_planner/legacy_rules/pattern2.rs`
  - 現状は JoinIR 側 extractor を呼んでいる

## Historical Target Architecture (2025-12-29)

```
src/mir/builder/control_flow/plan/extractors/
  ├── mod.rs
  ├── pattern6_scan_with_init.rs
  ├── pattern7_split_scan.rs
  └── pattern2_break.rs          ✨ NEW（P10）

src/mir/builder/control_flow/joinir/patterns/extractors/
  ├── pattern2.rs                (縮退: plan 側へ delegate / re-export)
  └── mod.rs                     (必要なら export 更新)
```

Current note:
- current runtime では `joinir/route_entry/`, `plan/facts/loop_break_*`,
  `single_planner::try_build_outcome` が live lane だよ。
- 上の path は P10 実行時の migration-time path token として残しているよ。

## Implementation Steps（Critical Order）

### Step 1: plan 側へ extractor を追加（pure, 既存コードの移設）

追加:
- `src/mir/builder/control_flow/plan/extractors/pattern2_break.rs`（historical path token）

方針:
- `joinir/patterns/extractors/pattern2.rs` の実装を **意味論そのまま** 移植（import path を plan 側に合わせるだけ）。
- 返り値型/引数/エラーメッセージは変更しない。

### Step 2: plan/extractors/mod.rs に登録

更新:
- `src/mir/builder/control_flow/plan/extractors/mod.rs`

追加:
- `pub(in crate::mir::builder) mod pattern2_break;`
- `pub(in crate::mir::builder) use pattern2_break::extract_pattern2_break_plan;`（既存命名に合わせる）

### Step 3: JoinIR 側 extractor を wrapper 化（historical `joinir/patterns/*` lane）

更新:
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern2.rs`

方針:
- 関数はそのまま残す（外部呼び出し互換）。
- 中身は plan 側 extractor を呼ぶだけにする（または re-export）。
- JoinIR 側の helper/private 関数が必要なら、先に plan 側へ移して SSOT を plan に寄せる。

### Step 4: single_planner legacy_rules の参照先を plan 側に統一

更新:
- `src/mir/builder/control_flow/plan/single_planner/legacy_rules/pattern2.rs`

方針:
- `crate::mir::builder::control_flow::plan::extractors::pattern2_break::*` を呼ぶように変更。
- 返り値/ログ/順序は維持。

### Step 5: SSOT docs 更新

更新:
- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

最低限:
- P10 の scope（移設のみ・仕様不変）を明記。
- Next（P11）候補を 1 行だけ（例: loop_break route を Facts→Planner の subset に吸収）。

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

期待:
- 既定挙動不変で PASS
- 新しいログ増加なし
- 新しい warning 増加なし
