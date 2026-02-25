---
Status: SSOT
Scope: selfhost を進める前に compiler 側（Facts/Normalize/CorePlan）で表現力を増やす開発方針
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/20-Decisions.md
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/phases/phase-29br/README.md
---

# Policy: Compiler expressivity first (pause selfhost workarounds)

## 結論

selfhost（`.hako` 側の回避的対応）を優先せず、先に compiler 側（Facts 前段の正規化・loop 構造箱・analysis-only view）で表現力を増やす。

## 目的

- JoinIR / CorePlan の構造を収束させる
- 疑似Effect・暫定ルール・レガシーコードを将来的に削除可能にする
- 構造の歪みが原因で発生する、数時間単位の Rust デバッグを根本から無くす

## 方針

- raw の条件式・更新式は **実行コードとして書き換えない**
- analysis-only view として `CondCanon` / `UpdateCanon` を conservative（保守的）に追加する
  - 正規化できない形は推測しない（`None`）
  - strict/dev は従来どおり検出する（no silent fallback）
- selfhost gate は opt-in の canary として扱い、`.hako` 側の回避を優先しない
- selfhost を先行すると、ツールコードが compiler の弱点を回避するためのレガシーとして残りやすい

## No-rewrite safety checklist (SSOT)

「見た目の等価変形」は、評価順/回数や数値規約（overflow/NaN 等）で意味が変わり得るため禁止する。
analysis-only view は次を満たすこと:

- AST を作り直さない（raw の `ASTNode` を変更しない）
- 新しい演算を追加しない（runtime work を増やさない）
- 評価順/評価回数を変えない（副作用を前提にしない）
- overflow/NaN/ゼロ割など、数値規約に依存する代数変形をしない

## Recommended order (roadmap)

1. `CondCanon/UpdateCanon` を導入して “候補が空でfreeze” を減らす（analysis-only view）
2. step 抽出（loop update）を `UpdateCanon` ベースへ段階移行する（raw rewrite 禁止のまま受理範囲だけ増やす）
3. `Loop` を “構造箱” として扱える v1（LoopFrame + `Break/Continue(depth)`）へ進み、nested loop の自然な合成を可能にする（Phase 29bs）
4. selfhost/.hako 側の回避コードを減らし、削除可能な形へ収束させる

## CorePlan minimal building blocks (v1 planned)

CorePlan 周辺の表現力が足りない状態でパターンを増やすと、`.hako` 側 workaround が増えてレガシー化しやすい。
このリポジトリでは、次の “最小部品セット” を先に揃える方針を優先する。

- `Block/Seq`（n-ary）: “effect列” を構造として一本化し、lowerer/verifier の契約を単純化する
- `Branch/If` + `ExitKind`（Return/Break/Continue）: loop body の制御を CorePlan だけで完結させる
- `LoopFrame` + `Break/Continue(depth)`（depth=1 既定）: nested loop と break/continue のスコープを by-name なしで固定する
- `CleanupWrap`（defer/cleanup 階層）: exit の階層性を CorePlan 語彙で表し、nested loop で意味論が混線しないようにする

運用ルール（SSOT）:

- 追加は strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` 限定で行い、release 既定は不変に保つ
- raw AST は書き換えず、analysis-only view（`CondCanon`/`UpdateCanon`）と CorePlan 合成で受理範囲を増やす
- 1部品ごとに fixture + fast gate を追加し、契約（受理条件/ログ/期待出力）を固定してから次へ進む

## Stop-the-line triage (BoxCount vs BoxShape)

「1時間以上デバッグして進まない」場合は、まず原因を 2 択で分類してから次の一手を決める（迷走防止）。

- BoxCount（箱の数不足）:
  - 症状: `planner_required` 下で `None→freeze`。同じ freeze 点が再現し続ける。受理形を 1 つ増やせば前進できる。
  - 次の一手: 最小の “受理形（plan rule / 小語彙）” を追加し、fixture+gate で契約を固定する（release 既定不変、strict/dev 限定）。
- BoxShape（箱の形が悪い）:
  - 症状: 受理形はあるのに責務混線/SSOT不足/不変条件が局所検証できず、修正が連鎖して前進しない。
  - 次の一手: 箱を増やさず、責務分割・入口集約・SSOT化・ログ契約の固定を優先する（構造でFail-Fast）。

### Logging template (paste into `CURRENT_TASK.md` / phase README)

- Stop理由: `[stuck] 60min+ debug`
- 判定: `BoxCount` / `BoxShape`
- 根拠: （1行で）`planner_required None→freeze` / `責務混線で修正が連鎖` など
- 次の一手: （1つだけ）`最小箱Xを追加してgate固定` / `箱Yを分割してSSOT+入口集約` など

## Follow-up cleanup (planned)

Loop を “構造箱” として使えるようになった後は、CorePlan 周辺の歪み（workaround）を段階的に削除する。

- fixture を “受理形に寄せるための歪み” から自然形へ戻す（`continue-if + break` など）
- `flatten_body_effects()` / effect-only 前提を意味論の前提から外し、派生ビュー/最適化へ降格する
- パターン増殖を抑え、`Skeleton + FeatureSet` の合成に寄せる（Facts は “観測 + canon(view)” を維持）
- CleanupWrap（defer/cleanup の階層）を先に CorePlan 語彙として固定し、nested loop の exit で意味論が混線しないようにする

## SSOT

- generic loop v0 の受理範囲・契約: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`
