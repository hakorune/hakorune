---
Status: SSOT
Scope: CorePlan purity Stage-2（release 既定で CorePlan を唯一の構造SSOTにする）
Related:
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# CorePlan Purity Stage-2 (SSOT)

目的: release 既定の経路でも「構造の真実 = CorePlan」を成立させ、fallback を最終的に 0 に収束させる。

## 定義（Stage-2 の“完了”）

以下が同時に成立していること:

1. 構造SSOTが `CorePlan`（emit/merge は Facts/AST/JoinIR を再解析しない）
2. JoinIR のルーティングは `plan/composer` 入口のみ（legacy loop table / pattern名分岐が残らない）
3. release 既定で、回帰対象（gate の全ケース）が CorePlan 合成経路で通る
4. strict/dev では “対象っぽいのに一意にできない” を Freeze/Fail-Fast で可視化できる（silent fallback 禁止）

## 非目的（Stage-2 ではやらない）

- 仕様拡張（新しい言語機能の追加）
- 例外/Unwind の実装（予約は OK、意味論の導入は別）
- 最適化パスの追加/拡張（effect法典の運用は維持）

## Acceptance（SSOT）

- `cargo build --release` が通る
- `./tools/smokes/v2/run.sh --profile quick` が PASS
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が PASS
- release の恒常ログ/エラー文字列が増えない（タグは strict/dev の raw output のみ）

## 進め方（安全順）

1. 回帰 pack（phase-29ae）に “CorePlan release adopt” を段階的に追加し、タグは strict/dev のみで固定
2. router の legacy 分岐を削り、入口を composer に一本化（Done criteria に反映）
3. 残った Ok(None) を棚卸しし、対象外/Freeze の境界を SSOT と smoke で固定
