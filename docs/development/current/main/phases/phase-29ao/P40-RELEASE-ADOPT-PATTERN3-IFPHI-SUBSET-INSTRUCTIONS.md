---
Status: Ready
Scope: Stage-2（release既定）を IfPhiJoin の “planner subset” へ拡張する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P40: Stage-2 — release adopt IfPhiJoin subset (planner-derived only)

## 目的

- P36–P39 で LoopSimpleWhile / scan_with_init / split_scan / LoopBreak の planner subset を release 既定で composer 経由に寄せられた。
- P40 は同じ安全方針で、IfPhiJoin（historical label 3）のうち **planner subset** を release 既定で `facts → composer → CorePlan` に寄せる。
- strict/dev の shadow adopt は引き続き「タグ必須」で回帰固定する（観測/Fail-Fast 維持）。

## 非目的

- IfPhiJoin の subset 拡張（誤マッチ防止のため、既存 subset のまま）
- 新しい env var 追加
- release でタグ出力（恒常ログ増加）
- エラー文字列の変更

## 実装（P37/P38/P39と同型）

### Step 1: composer に IfPhiJoin release adopt 入口を追加

対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
- `src/mir/builder/control_flow/plan/composer/mod.rs`（re-export）

追加する関数（例）:
- `try_release_adopt_core_plan_for_if_phi_join(...) -> Result<Option<CorePlan>, String>`

採用条件（安全ゲート）:
- historical plan variant token `DomainPlan::Pattern3IfPhi(_)` のときだけ
- `outcome.plan` が `Some(DomainPlan::Pattern3IfPhi(_))` のときだけ（planner 由来のみ採用）
- `outcome.facts` が存在し、`facts.facts.if_phi_join` が `Some` のときだけ

合成:
- 既存の `compose_coreplan_for_if_phi_join(builder, facts, ctx)` を再利用

失敗時（release既定）:
- `Ok(None)` または `Err(_)` はすべて `Ok(None)` に丸めて、従来経路へフォールバック（仕様不変）

### Step 2: router の非strict経路で IfPhiJoin release adopt を接続

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

位置:
- `if !strict_or_dev { ... }` ブロック内（LoopSimpleWhile / scan_with_init / split_scan / LoopBreak の後）
- `lower_via_plan(...)` の前

処理:
- `PlanVerifier::verify(&core_plan)?; PlanLowerer::lower(...)`
- タグ出力はしない（strict/dev のみ tag）

### Step 3: 非strict smoke を追加して gate に入れる

狙い:
- 既存の IfPhiJoin smoke は strict/dev でタグ検証に寄っているため、release adopt 経路が踏まれない。
- “strictを付けない” integration smoke を 1 本追加して、非strict 実行でも安定することを固定する。

追加（例）:
- `tools/smokes/v2/profiles/integration/joinir/if_phi_join_release_adopt_vm.sh`

要件:
- `HAKO_JOINIR_STRICT=1` を設定しない（`env -u` で外す）
- fixture は current semantic alias `apps/tests/if_phi_join_min.hako` を使用（historical pin token は `joinir-legacy-fixture-pin-inventory-ssot.md` を参照）
- 期待:
  - exit code = 0
  - output 数値が `12`
- タグは出ない（releaseでタグ出力禁止。exact legacy tag suffix token は coverage SSOT 側の traceability lane を参照）

配線:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に current semantic wrapper filter 1 行追加
- `docs/development/current/main/phases/phase-29ae/README.md` の pack 項目追記

### Step 4: docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P40完了/Next）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p40): release adopt if-phi-join subset"`
