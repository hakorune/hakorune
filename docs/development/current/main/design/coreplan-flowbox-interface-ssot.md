---
Status: SSOT
Scope: CorePlan を “FlowBox（制御の箱）” の合成系として扱うための最小インターフェース（ports / ExitMap / join payload）
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/coreloop-exitmap-composition-ssot.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
---

# CorePlan FlowBox Interface (SSOT)

目的: “箱理論（制御の箱）” を CorePlan に当てるとき、CorePlan node を **FlowBox（ports を持つ部品）**として定義し、
post-phi / Exit / cleanup を **ports と合成規則**だけで完結させる。

この文書は「箱の最小インターフェース」を SSOT として固定し、Facts/Recipe/Planner が FlowBox の責務を奪ってしまう事故を防ぐ。historical planner-payload wording はここでは扱わない。

## 1. 用語

- **FlowBox**: 制御構造を表す箱（CorePlan node）。メモリの BoxRef/WeakRef とは概念を分ける。
- **Port**: 箱の入出力端子。入口（entry）と出口（normal / ExitMap）を持つ。
- **ExitKind**: `Return/Break/Continue/(Unwind予約)` の出口種別。
- **Join payload**: post-phi の最終表現。pred/port ごとの “どの値がどこへ流れるか” を明示する。

## 2. FlowBox の最小インターフェース（ports）

FlowBox は最低限、次を持つ（概念インターフェース）。

- `entry`: 入口 port（1つ）
- `normal`: 通常の出口 port（0 or 1）
- `exits: ExitMap<ExitKind, Port>`: 例外的出口（0..N）
- `join payload`: port から出る値の束（post-phi を暗黙にしない）

重要:
- **emit/merge は FlowBox 以外を再解析しない**。CFG/AST/Facts に戻って “穴埋め” をしてはならない。
- “phi を暗黙にしない” は **join payload を ports で表す**ことで達成する。

## 3. Skeleton / Feature との関係

- Skeleton（骨格）は FlowBox の “構造カテゴリ”（Loop/If2/BranchN/Seq/LeafEffects 等）。
- 291x-757: loop-facts `SkeletonKind` は Loop/StraightLine に縮退済み。
  FlowBox の If2/BranchN 語彙は `CorePlan::{If,BranchN}` から観測する。
- Feature（直交特徴）は FlowBox の “属性”。
  - ExitMap / Cleanup / ValueJoin 等は、**別パターンを増やさず** FlowBox に付与する。

## 4. cleanup の表現（wrapper combinator）

cleanup は箱の “内部命令” として散らさず、合成規則として統一する。

- `CleanupWrap(inner_box, cleanup)` のような **構造ラッパ**として定義する（概念）。
- `normal` を含む **すべての出口**（Return/Break/Continue/Unwind予約）が cleanup を経由して流れる。

この方針により「cleanup はどこで走るか？」が FlowBox 合成規則だけで決まり、SSOT が崩れない。

## 5. return-heavy（loop 内 early return）の最小語彙追加方針

目的: `loop { if cond { return ... } }` のような return-heavy を、Loop.body の “effect-only” 制約を崩さずに吸収する。

推奨（最小）:
- “汎用 goto” を追加しない。
- 追加するなら **ガード付き脱出**のみ（例）:
  - `ExitIf { cond, kind: ExitKind, payload }`
    - `kind` は `Return/Break/Continue/Unwind予約` のみ（normal へは飛ばせない）
    - 発火したらその箱の以降の effect は実行されない（箱の意味論）

狙い:
- `is_integer` のような形を “Return port に落とす” だけで表現できる。
- “ExitIf が増殖してCFG命令セット化する” 事故を防ぐ（禁止: 任意ラベルへの分岐）。

## 6. BranchN / Unwind（予約）

- `BranchNSkeleton` は FlowBox 的に自然（Choice箱）。未使用でも “語彙として予約” する価値が高い。
- `ExitKind::Unwind` は未実装でも **出口として予約**しておく（cleanup 合成規則が後から揺れない）。

## 7. observability（strict/dev の安定タグ）

推奨: pattern名・rule名ではなく、FlowBox スキーマに寄せる。

- `region_key`（安定ID）
- `box_kind`（Loop/If2/BranchN/...）
- `feature_set`（ExitMap/ValueJoin/Cleanup 等）
- `freeze_code`（Ambiguous/Inconsistent/Unstructured/Unsupported/BugInvariant）

release 既定では恒常ログを増やさない。strict/dev のみで安定タグを出す。
