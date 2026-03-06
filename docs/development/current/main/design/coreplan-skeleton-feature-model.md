---
Status: SSOT
Scope: Facts→Recipe→CorePlan の「非重複」設計（JoinIR/Plan/Frag）
Related:
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- docs/development/current/main/design/join-explicit-cfg-construction.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
---

# CorePlan Skeleton/Feature Model (SSOT)

目的: “pattern 列挙の重なり” を増やさずに、Facts→Recipe→Verifier→CorePlan を **一意・合成可能**な形に収束させる。

結論:
- **CorePlan は構造SSOT**（emit/merge は CorePlan/Frag 以外を再解析しない）
- **Recipe / VerifiedRecipe は意味の入口**（受理契約はここで閉じる）
- numbered route labels は入口の分岐名ではなく、**(Skeleton, FeatureSet)** の合成へ落とす

## 1. Skeleton（骨格）= まず一意に決める

Skeleton は “構造カテゴリ” のみ。ここが一意に決まらない場合は `Freeze` の対象。

推奨の最小集合:
- `LoopSkeleton`（自然ループ: header/latch/exit が確定できる）
- `If2Skeleton`（2分岐 + join）
- `BranchNSkeleton`（match/switch 相当: N分岐 + join）
- `StraightLine`（plan 対象外 → `Ok(None)`）

## 2. Features（直交特徴）= 骨格の上に足す

Features は “別パターン” を増やさずに足す（重なりの根治）。

例（代表）:
- `ExitMap`（Return/Break/Continue の出口集合。loop_break / loop_continue_only / loop_true_early_exit を “別pattern” にしない）
- `ExitBranch`（If/BranchN/Loop 内の “exit 付きブランチ” を共通化: prelude + ExitKind を 1 箱で抽出/正規化する。SSOT: `docs/development/current/main/design/exit-branch-feature-ssot.md`）
- `ValueJoin`（join 値が必要 = post-phi 表現）
- `ContinueEdges`（continue が複数箇所から飛ぶ等。latch 一意に拘らない表現）
  - 例: per-edge carrier merge（`ContinueWithPhiArgs` + step join PHI）で “continue による未定義 ValueId” を構造で解消
- `Cleanup`（return/break/continue で走る cleanup。将来 `Unwind` も同語彙へ）
- `CondShape` / `StepShape`（normalize 済みの “形”）
- `AlgorithmIntent`（scan/split/predicate 等のアルゴリズム意図: Recipe/feature slot に置く）

補足:
- `ExitIf` / `match` / “loop 内の if-exit” で exit 判定/前処理/phi 引数などのロジックが重複しがちなので、
  `ExitBranch`（analysis-only の判定 + CorePlan 生成）を先にレゴ化しておくと “例外パターンの堆積” を防げる。

## 2.2 分類レンズ（非SSOT）: “直交軸” はタグとして使う

目的:
- どこが重複しているか/どの部品が足りないか、を **計測・整理**しやすくする
- ただし “直交軸の組み合わせ” を SSOT にして巨大化させない（**SSOTの主語は Skeleton/FeatureSet のみ**）

運用:
- “6直交軸” は **Feature slot 設計のレンズ**として使う（タグ/メタ情報）
- “6軸の組み合わせ = 受理形SSOT” にはしない（組み合わせ爆発を防ぐ）

翻訳例（レンズ → slot）:

| レンズ（例） | SSOT側の置き場所 | slot（例） |
|---|---|---|
| 条件種別（単純/複合） | Canon/View | `CondBlockView` / `CondCanon` |
| 脱出（break/continue/return） | FeatureSet | `ExitMap` / `ExitBranch` / `ExitKind(depth)` |
| join/PHI（if-join、carrier merge） | FeatureSet | `ValueJoin`（post-phi） / `ContinueEdges`（per-edge merge） |
| ネスト（if内/loop内） | Skeleton/Feature | `LoopSkeleton`（再帰） + `NestedLoopFeature(depth)` |
| 状態更新（single/conditional/multi） | Canon/View + FeatureSet | `UpdateCanon` + `StepMode/StepPlacement` |

注:
- “同じ構文なのに経路が違う” は、patternの境界曖昧さではなく **入口の view/canon の差**で起きることが多い。
- まず Canon/View の入口統一（analysis-only / no rewrite）を優先し、feature slot の追加で表現力を増やす。

## 2.1 “箱理論” 対応（FlowBox）

CorePlan node を “FlowBox（制御の箱）” として扱う場合、SSOT は Skeleton/Feature のみではなく、
**ports（entry/normal/exits）＋join payload** を明示して “phi/merge の暗黙化” を防ぐ必要がある。

FlowBox の最小インターフェースは次を SSOT とする:
- `docs/development/current/main/design/coreplan-flowbox-interface-ssot.md`

## 3. Freeze（Fail-Fast）にすべき “要注意パターン”

“通らない” または “通るが設計が壊れやすい” を SSOT として明示する。

### A. Unstructured CFG（Irreducible / multi-entry loop）

症状:
- Skeleton が一意に決まらない（複数入口ループ / irreducible）

扱い:
- `Freeze(unstructured)`（strict/dev は即Fail）
- 既定挙動を変えない場合は `Ok(None)` に落とすのも可だが、strict/dev では **タグ付きで理由を観測**する

### B. Unwind / 例外 / finally（“別系統の出口エッジ”）

症状:
- Return/Break/Continue 以外の出口（unwind）で cleanup が必要になる

扱い:
- 設計として `ExitKind::Unwind` を想定し、`ExitMap + Cleanup` で吸収できることを前提にする
- 未実装の段階では `Freeze(unsupported:unwind)`（“対象っぽいのにNone” にしない）

### C. Coroutine / async generator（yield）

症状:
- 関数内で制御が閉じない（skeleton だけでは表現できない）

扱い:
- `Freeze(unsupported:coroutine)`（将来は別 Skeleton/別パイプライン）

### D. 多分岐（match）を If2 で潰そうとする

症状:
- planner が肥大化し、normalize が崩れる（pattern 爆発の起点）

扱い:
- `BranchNSkeleton` を設計語彙として追加し、normalize で分岐順序を安定化する

### E. “plan が JoinIR 専用パターンを飲み込む” 事故

実例:
- nested loop（phase1883）が plan 側の `loop_simple_while` family に誤マッチして JoinIR の `NestedLoopMinimal` が選ばれない

扱い:
- “より一般的な route family” は **上位形（nested loop 等）を `Ok(None)` へ倒す**（route 側の責務）
- 入口での by-name 分岐ではなく、Facts/Extractor の **構造条件**で遮断する

### F. return-heavy（loop 内 early return）

症状:
- loop 内で `return false` 等の early return を多用する（`is_integer` 等）

扱い:
- ScanWithInit/LoopBreak/SplitScan に無理に押し込まず、FlowBox の **出口（Return port）**として表現する。
- 最小語彙追加は “ガード付き脱出（ExitIf）” のみに限定し、汎用 goto 化を禁止する。
- 詳細は `docs/development/current/main/design/coreplan-flowbox-interface-ssot.md` を SSOT とする。

## 4. SSOT 運用ルール（設計側の約束）

- Planner は “骨格推論→特徴推論” を意識して実装し、complete pattern 追加を最小化する
- Emit は CorePlan 以外を見ない（再解析禁止）
- `Ok(None)` / `Freeze` の境界は `docs/.../planfrag-freeze-taxonomy.md` を SSOT として従う

## 5. Next (docs-first)

次に SSOT として固める候補（レバレッジ順）:
1. post-phi（join 入力の最終表現）不変条件（局所 verify）
2. effect 分類（pure/control/rc/obs）と “許される変形” の法典
