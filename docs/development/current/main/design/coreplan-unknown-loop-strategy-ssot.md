---
Status: SSOT
Scope: CorePlan / JoinIR — unknown loop strategy (decompose & compose)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/design/selfhost-tools-loopless-subset-ssot.md
---

# CorePlan unknown loop strategy (SSOT)

## Goal

「既知の意味ルート（legacy label: Pattern1–N）に一致しない一般ループ」が `[joinir/freeze]` で止まる状況を、
CorePlan を “第二のCFG言語” に肥大化させずに解消する。

結論: **“General Loop ノード追加” ではなく、FlowBox プリミティブ（Skeleton + Feature + 最小制御effect）への分解と合成で吸う**。

## Decision (SSOT)

### A. Prefer decomposition & composition (hybrid C)

- CorePlan は **FlowBox 合成系**として小さい語彙を維持する
- 入口の識別は legacy label: PatternX ではなく **(Skeleton, FeatureSet)** の合成へ縮退させる
- unknown loop は “未対応だからFreeze” ではなく、**定義域内なら標準のCorePlan合成で受理**する

### B. Avoid: “GeneralLoop = second CFG language”

以下は避ける（CorePlan を汎用CFG命令セットへ変質させるため）:
- 任意 goto / 任意ラベル分岐
- CorePlan 内に AST/CFG 再解析ロジックを持ち込む
- emit/merge が CorePlan 以外（Facts/AST/CFG）を覗き直して穴埋めする

## Primitive set (SSOT)

### Skeleton（構造カテゴリ）

最小集合を固定する（増やすのは “新しい構造カテゴリ” が必要な場合のみ）:

- `Seq`
- `If2`
- `BranchN`（match/switch 相当）
- `Loop`（natural loop: header/latch/exit が一意に取れる）
- `LeafEffects`（制御を持たない効果列）

### Features（直交特徴）

別ルート語彙を増やさずに属性として付与する:

- `ExitMap`（Return/Break/Continue/Unwind予約）
- `ValueJoin`（join payload = post-phi final form）
- `Cleanup`（wrapper combinator）
- `ExitUsage`（観測/Freeze境界の材料）

### Minimal control inside LeafEffects

LeafEffects 内で許可する制御は **ガード付き脱出のみ**（CFG言語化を防ぐ）:

- `ExitIf { cond, kind: ExitKind, payload }`
  - `kind` は `Return/Break/Continue/Unwind予約` のみ
  - normal へ飛ばせない（禁止: 任意 goto）

注: `return-in-loop-minimal-ssot.md` の “ExitIfReturn” は、この一般形（ExitIf）へ統合できる。

## What becomes “unknown loop”

旧来の “ルート表（legacy label: PatternX）” で表現しづらいが、構造としては普通に起きるループ群:

- 文字列走査などの一般スキャン（条件や更新が subset の shape enum から外れる）
- break/continue/return が混ざる（ただし構造化できる範囲）
- step が `i = i + 1` 以外（`i = i - 1` / `i = i / 10` 等）でも、pure effect として表現可能なもの

## Acceptance domain (when we should accept, not freeze)

unknown loop でも **次を満たすなら受理**する:

- Skeleton が一意（natural loop / reducible）
- cond / step が “pure expression / pure effects” として表現でき、局所verifyが可能
- Exit は ExitMap/ExitIf 経由で ports に落ちる（暗黙phiなし）

この “受理” は fallback ではなく、**標準のCorePlan表現力**として扱う。

## Freeze boundary (strict/dev)

Freeze に落とす主因は “未実装” ではなく **定義域外**へ寄せる:

- `Unstructured`（irreducible / multi-entry loop）
- `Inconsistent`（Facts矛盾）
- `Ambiguous`（Skeletonが複数成立）
- `Unsupported`（例: Unwind未実装など、設計上は定義済みだが未対応）
- `BugInvariant`（内部不変条件破れ）

`Ok(None)` は “対象外（plan不要）” のみで使い、「対象っぽいのに落とす」用途に使わない。

## Tooling (selfhost) policy alignment

短期（bringup）:
- selfhost tooling は `NYASH_DISABLE_PLUGINS=1` の決定性を優先し、`loopless subset` をSSOTとして運用してよい
  - `docs/development/current/main/design/selfhost-tools-loopless-subset-ssot.md`

中長期:
- 本SSOTの unknown loop strategy が実装された後は、tooling 側も
  - “restricted loop（Loop + LeafEffects + ExitIf）” の範囲で自然なループ表現に戻すことを許可する
  - ただし “構造化できない” 形は strict/dev で `flowbox/freeze` に収束させる

