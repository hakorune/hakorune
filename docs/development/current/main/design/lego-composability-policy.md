---
Status: SSOT
Scope: compiler 側の表現力を「レゴ部品（小さなSSOT）」として伸ばし、完成品キット（大箱の増殖）を負債化させないための運用方針
Related:
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/joinir-design-map.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- docs/development/current/main/design/boxcount-new-box-addition-checklist-ssot.md
- src/mir/builder/control_flow/plan/REGISTRY.md
---

# Policy: Lego composability first (avoid “finished kit” box accretion)

## 目的

selfhost canary を「ブロッカー抽出の入口」として使いながら、最優先は **compiler 側の表現力（CorePlan / Facts / Normalizer）** を
“レゴ部品” として増やすこと。  
fast gate / fixture が増えても、設計が “組み立てやすく・分解不要” な形に収束することを優先する。

## 用語（SSOT）

- **レゴ部品**: 使い回せる最小の構造・契約（例: `coreloop_skeleton` の LoopFrame/PHI/continue 入口、共通 walker、共通 join など）。
- **完成品キット**: 1つのブロッカー形だけを吸うために、既存の skeleton / merge / walker を箱ごと複製して増える大箱。
  - 例: cluster3/4/5 のように “数だけ” で増殖し、配線変更が分散するもの。

## 最終形（Recipe-first Lego）の定義（SSOT pointer）

「レゴ部品」を最小化する最終形は、制御構造の語彙を増やさずに **契約（Verifier）**で表現力を増やす。

- **RecipeTree（構造SSOT）**: `Seq / Stmt / If / Loop / Exit(Break/Continue/Return)` だけ。
  - `else` は `If` の一部（then-only は else 省略可だが、join_payload 非空なら completeness policy で freeze）。
- **VerifiedRecipe（配線可能性SSOT）**: `PortSig / Obligation / JoinPayload / CarrierList` を Verifier が再帰合成して固定。
- **Lower/Parts**: VerifiedRecipe を **機械的に配線するだけ**（再判定・例外処理なし）。
- **Entry**: coherence（候補一意）。曖昧なら strict/dev(+planner_required) で freeze（順序依存は禁止）。

関連SSOT:
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`

## 原則（必須）

1. **BoxShape → BoxCount の順**
   - まず “部品不足/責務混線” を BoxShape で解く（入口集約・SSOT化・共通部品化・ログ契約固定）。
   - それでも受理形が足りない場合だけ BoxCount（最小箱）を追加する。

2. **完成品キット禁止（原則）**
   - 既存の部品（LoopFrame/PHI/CondBlockView/共通 walker/joins）を使わずに、似た骨格を複製する箱の追加は禁止。
   - 例外は `planner_required` の unblock に必要で、かつ「次の BoxShape で分解・統合する計画」が SSOT に書かれている場合のみ。

3. **“増殖しやすい軸” は parameter/table 化を先に検討**
   - nested loop “個数” / “深さ” / “call の有無” のように軸が分かっている場合、箱を増やすより先に
     `*_profile.rs` / table SSOT を作る（追加点を 1 箇所に集約）。

4. **契約の固定は fixture+fast gate だが、増やし方を制御する**
   - ブロッカー再現は fixture で固定する。
   - ただし “単発の形のためだけに箱を増やす” のではなく、**部品の契約**を増やす（共通部品のテストを pin する）。

## 判断テンプレ（PR/コミット前の自己チェック）

- この変更は「新しい箱」を増やしているか？
  - Yes → その箱は “完成品キット” になっていないか？（骨格/PHI/walker を複製していないか）
  - もし複製しているなら、まず共通部品へ切り出せるか（BoxShape）を検討し、最小の部品を先に作る。
- この変更は「数だけで増える軸（clusterN）」か？
  - Yes → まず table/profile SSOT の導入で追加点を 1 箇所にできないか？
- 失敗の原因は “受理不足” か “配線/契約の弱さ” か？
  - 受理不足（BoxCount）: 1形だけ追加して fixture+gate で固定。
  - 契約の弱さ（BoxShape）: 入口SSOT・共通部品・ログ契約を先に固める。

## 入口ガード（planned）

AI が "reject したのに accept してしまう" などを起こさないため、入口に Fail-Fast ガードを置く（仕様だけ先に固定）。

- Facts の `Ok(None)` は **理由SSOT（enum）** を伴う（dev/strict で `[plan/reject]` 1行）。
- reject_reason → "次に試す箱（handoff）" を **テーブルSSOT** に集約し、順序依存で偶然通る状態を減らす。
- composer の allowlist 例外（`shadow_adopt` など）は、エラーメッセージに `DomainPlan` 名を含める（罠を即特定）。

## 実例: clusterN の table/profile SSOT 化

cluster3/4/5（nested loop 個数だけで分かれる箱）は、
`src/mir/builder/control_flow/plan/facts/nested_loop_profile.rs` の `CLUSTER_PROFILES` を SSOT とする。

- cluster6+ 追加時は `CLUSTER_PROFILES` に1行追加するだけ
- `build.rs` と `loop_facts.rs` がこの table を走査して抽出・選択
- CLUSTER_PROFILES は降順（5→4→3）。build.rs で優先順走査するため
- `required_count` は「正確に N 個の nested loop」を意味する
- **Phase 3 完了**: DomainPlan variant を統合（Cluster3/4/5 → LoopCondBreakContinue）
  - cluster3/4/5 は `facts/nested_loop_profile.rs` の表で識別（required_count=3/4/5）
  - planner_tag（TSV rule 文字列）は CLUSTER_PROFILES から引き継ぎ、挙動不変
- **Phase 4 完了**: cluster3/4/5 フォルダ削除・dead_code warning 0
  - `loop_cond_break_continue_cluster{3,4,5}/` フォルダ削除（git rm -rf）
  - mod.rs の cluster mod 宣言削除
  - テストコードの LoopFacts 初期化から cluster フィールド削除
  - `cargo check --bin hakorune` warning 0 確認済み、fast gate 全緑
