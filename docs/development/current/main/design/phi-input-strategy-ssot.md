---
Status: SSOT (design-only)
Scope: Loop header/step PHI の入力パターン（型で固定するための設計メモ）
Related:
- docs/development/current/main/design/feature-helper-boundary-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/edgecfg-fragments.md
- docs/development/current/main/design/loop-canonicalizer.md
- src/mir/builder/control_flow/plan/features/loop_carriers.rs
---

# PhiInputStrategy (SSOT, design-only)

目的: “PHI を作るときの入力パターン” を、文字列/コメント/その場の分岐ではなく **型で表現**して固定する。
将来の BoxShape 作業で「同じ形の PHI が別ファイルで増殖する」事故を止める。

## 背景（現状）

`features/loop_carriers.rs` に PHI 構築 helper を SSOT 化したが、呼び出し側にはまだ「どの PHI を作るべきか」という判断が残りやすい。
判断が散ると、次の事故が起きる:

- pipeline ごとに微妙に違う “PHI 入力の構造” を作り始める
- その差分が gate に現れづらく、後で mismatch として爆発する
- “似てるけど違う” helper が増える（BoxShape 逆流）

## 非目標

- 型推論・静的検査を追加しない（Nyash の動的型方針とは別問題）
- lowering の意味論（実行時の挙動）を変えない
- 新しい PHI 形を増やさない（今ある形を “名前をつけて固定” する）

## Strategy の定義（案）

下記の “入力の形” を列挙し、`enum` として表現する。

### A. PreheaderOnly（1-input header PHI）

- 意味: 初期値は preheader から 1 回だけ来る（header 側はまだ更新入力を持たない）
- 典型: header の “キャリア初期化” / `use_header_continue_target=true` 周り
- 既存 helper: `loop_carriers::build_preheader_only_phi_info(...)`

### B. HeaderTwoInput（2-input header PHI）

- 意味: header は preheader と step（または header 継続側）の 2 入力を持つ
- 典型: classic loop-carrier header PHI
- 既存 helper: `loop_carriers::build_loop_phi_info(...)`

### C. StepJoinEmpty（0-input step join PHI）

- 意味: step join は “入力なし” の PHI を置いておき、後段の wiring で埋める（または join payload のための slot）
- 典型: step block の join 点に先置きする形
- 既存 helper: `loop_carriers::build_step_join_phi_info(...)`

## 受け入れ基準（この SSOT の役割）

この文書が “どの PHI 形が存在して良いか” の SSOT になる。

- 新しい PHI 形を導入する場合は、必ずここに Strategy を追加してから実装する
- pipeline 直書きでの PHI 構築は禁止（`feature-helper-boundary-ssot.md` に従う）
- 実装側 SSOT は `features/loop_carriers.rs` と一致させる（Strategy ↔ helper の対応が 1 対 1）

## 次アクション（将来の BoxShape）

“この Strategy を enum にする” こと自体は Phase を区切って行う。

1. **docs-only**: この SSOT を確定（本書）
2. **thin adapter**: `loop_carriers` に `build_phi_info(strategy, ...)` 的な入口を追加（既存 helper を呼ぶだけ）
3. **gradual replace**: pipeline から “どの helper を呼ぶか” の分岐を Strategy に置換（挙動不変）
