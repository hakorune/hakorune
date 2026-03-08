---
Status: Ready
Scope: code（strict/dev only, 既定挙動不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P17: LoopSimpleWhile を Facts→CorePlan へ寄せる（strict/dev のみ shadow adopt）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 既定挙動（release/非strict）は不変。strict/dev のときだけ LoopSimpleWhile の CorePlan を Facts から direct compose して採用する。

## 目的

- “CorePlan で組み立てる” への移行を 1 本だけ進める（LoopSimpleWhile / historical label 1 のみ）。
- 既存の DomainPlan→Normalizer 経路を壊さず、strict/dev でのみ **Facts→CorePlan(skeleton)** を採用できるようにする。
- 対象外（Ok(None)）や不一致は Fail-Fast（strict/dev）で検出し、silent fallback を増やさない。

## 非目的

- 他の route family の composer 化
- header PHI を block_params 化（frag だけでは preheader incoming を観測できないため段階未到達）
- 新 env var 追加、恒常ログ追加、エラー文字列変更（strict/dev の Fail-Fast は既存方針）

## 対象

- `src/mir/builder/control_flow/joinir/route_entry/router.rs`
- `src/mir/builder/control_flow/plan/planner/outcome.rs`
- `src/mir/builder/control_flow/plan/normalizer/simple_while_coreloop_builder.rs`（既存: LoopSimpleWhile subset の direct skeleton helper）

## 方針（最小・安全）

### 언제採用するか（strict/dev のみ）

- historical plan variant token `DomainPlan::Pattern1SimpleWhile(_)` が選ばれた場合のみ対象
- strict/dev のときだけ、同じ AST から `planner::build_plan_with_facts_ctx(..)` をもう一度呼び、
  `outcome.facts` が `Some(CanonicalLoopFacts)` かつ `facts.facts.loop_simple_while.is_some()` のときだけ
  `PlanNormalizer::normalize_loop_skeleton_from_facts(..)` を呼んで `CorePlan` を作る
- 生成した CorePlan は `PlanVerifier::verify` を通し、`PlanLowerer::lower` で実行する

### 失敗時

- strict/dev のとき:
  - “LoopSimpleWhile が選ばれたのに facts から合成できない” は Err で Fail-Fast（SSOT ずれ検出）
- 非strict:
  - 何もしない（既定挙動不変）

## 実装手順

### Step 1: router の plan 経路で LoopSimpleWhile だけ composer を先に試す（strict/dev only）

- `route_loop()` の `if let Some(domain_plan) = ...` ブロックの直後に以下を追加：
  - `if strict_or_dev && matches!(domain_plan, DomainPlan::Pattern1SimpleWhile(_)) { ... }`
  - planner outcome を作って facts を取り出し、`normalize_loop_skeleton_from_facts` を呼ぶ
  - 成功したらその CorePlan を採用（verifier→lowerer）
  - 失敗したら Err（strict/dev）

注意: historical debug token `route=plan strategy=extract pattern=pattern1` は維持すること（追加ログは増やさない）。

### Step 2: 最小の確認

- strict/dev で LoopSimpleWhile が通ること（手元で 1 ケース）
  - 例: `HAKO_JOINIR_STRICT=1` をつけて VM 実行（既存の loop fixture でよい）

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p17): strict/dev adopt composer for pattern1"`

## 次（P18候補）

- P18: LoopSimpleWhile の composer 化を “strict/dev only” から “既定経路” へ昇格（ただし観測/挙動不変の確認が前提）。
