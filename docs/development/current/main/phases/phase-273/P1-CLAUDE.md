# Phase 273 P1: Plan語彙固定 + PlanVerifier（fail-fast）（Claude Code 指示書）

Status: instructions / design-first

目的:
- P0 の pattern-specific Plan（`Plan::ScanWithInit`）を、増殖しない固定語彙へ畳む。
- Extractor は pure を維持しつつ、Plan の整合性は Verifier で fail-fast に固定する。

スコープ:
- Plan 型の再設計（固定語彙への移行）
- PlanNormalizer（DomainPlan → CorePlan）の追加（SSOT）
- PlanVerifier の追加（境界で fail-fast）
- Pattern6 を “固定語彙 CorePlan” で表現し直す（PoC 維持）

Non-goals:
- 全 pattern の移行（P2+）
- 新しい言語機能
- env var 追加

参照:
- Phase 273 README: `docs/development/current/main/phases/phase-273/README.md`
- Phase 273 P0 completion: `docs/development/current/main/phases/phase-273/P0-COMPLETION.md`
- Frag SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

---

## Step 1: Plan の固定語彙を導入する

方針:
- `Plan` の variant を “構造語彙” に固定する（増殖しない）。
- pattern-specific variant は廃止方向（P1では Pattern6 から移行）。

最低限（P1）:
- `Plan::Seq(Vec<Plan>)`
- `Plan::Loop(LoopPlan)`
- `Plan::If(IfPlan)`
- `Plan::Effect(EffectPlan)`（副作用の順序だけ）
- `Plan::Exit(ExitPlan)`（Return/Break/Continue…）

注意:
- P1では “式の意味” を Plan に入れすぎない（expr は参照/ID/ASTNode を持つだけにする）。
- **式を `String` にしない**（Lowerer 側に「文字列式の解釈器」を生やさない）。

---

## Step 1.5: DomainPlan → CorePlan の 2層 + PlanNormalizer（SSOT）

狙い:
- “固定語彙にしたいが、scan固有データも欲しい” の葛藤を構造で解消する。
- Lowerer を **CorePlan-only** にして、pattern固有知識の混入を防ぐ。

方針:
- `DomainPlan`: pattern 固有の最小表現（短命・増殖は許容するが「削る側」）
- `CorePlan`: 固定語彙のみ（増殖禁止）
- `PlanNormalizer` が **唯一** `DomainPlan → CorePlan` を行う（SSOT）

重要:
- `EffectPlan::ScanInit` のような **scan専用 variant** は禁止
- 「あとで MethodCall に統一する」前提の例外は作らない（裾広がりの入口になる）

---

## Step 2: PlanVerifier を追加する（fail-fast）

新規モジュール案:
- `src/mir/builder/control_flow/plan/verifier.rs`

役割:
- Extractor が返した Plan が “Lowerer が扱える契約” を満たすかを検証する
- close-but-unsupported を `Err` にして誤コンパイル余地を潰す

P1で入れる最小の不変条件（例）:
- Loop の header params / carriers が揃っている
- Exit が Loop 外へ飛ぶ場合の kind が妥当
- Effect が順序上の制約を破っていない（最低限：Effect がブロック境界で落ちない）

---

## Step 3: Pattern6 を固定語彙 Plan で表現し直す

方針:
- Extractor は `DomainPlan` を返す（pure）。
- `PlanNormalizer` が `DomainPlan → CorePlan` を行い、固定語彙 `CorePlan::{Seq,Loop,If,Exit,Effect}` の組み合わせに落とす。
- Lowerer は `CorePlan` のみを処理する（pattern固有知識を持たない）。

禁止（P1）:
- scan固有情報を `EffectPlan` の payload/variant として持つこと（固定語彙の侵食）
- 式を `String` として Plan に埋め込むこと（第二の処理系が生える）

---

## Step 4: Router の責務を整理する

方針:
- router は `extract_*` を呼び、`PlanVerifier` を通し、`PlanLowerer` に渡すだけ。
- router が builder を触らない（制御の入口は薄く保つ）。

推奨の順序（P1）:
1. `extract_*`（pure, DomainPlan）→ `Ok(None)` / `Ok(Some(domain))` / `Err`
2. `PlanNormalizer::normalize(domain) -> CorePlan`（SSOT）
3. `PlanVerifier::verify(&core)`（fail-fast）
4. `PlanLowerer::lower(core)`（CorePlan-only）

---

## Step 5: 回帰確認（P0 と同等）

最低限:
- `tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh` PASS

---

## 完了条件（P1）

- Plan enum の variant が固定語彙に収束している（増殖しない）
- PlanVerifier が境界で fail-fast を提供している
- Pattern6 が固定語彙 Plan で表現され、P0のPoCが維持されている
