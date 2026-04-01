# JoinIR Design Map（現役の地図）

Status: SSOT（navigation）
Scope: JoinIR の「Loop/If を JoinIR 化して MIR に統合する」導線（検出→shape guard→lower→merge→契約検証）
Related:
- SSOT: [`docs/development/current/main/joinir-architecture-overview.md`](../joinir-architecture-overview.md)
- SSOT: [`docs/development/current/main/loop_pattern_space.md`](../loop_pattern_space.md)（historical route-label ledger）
- SSOT: [`docs/development/current/main/joinir-boundary-builder-pattern.md`](../joinir-boundary-builder-pattern.md)
- SSOT: [`docs/development/current/main/design/loop-canonicalizer.md`](./loop-canonicalizer.md)
- SSOT: [`docs/development/current/main/design/recipe-first-entry-contract-ssot.md`](./recipe-first-entry-contract-ssot.md) ← Recipe-first 主軸
- SSOT: [`docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`](./joinir-extension-dual-route-contract-ssot.md) ← 拡張時の固定順序

このドキュメントは Phase ログではなく、「JoinIR を触る人が迷子にならず、どこを直すべきかが一発で分かる」ための設計図（地図）です。  
詳細な経緯・作業ログは `docs/development/current/main/phases/` と `docs/development/current/main/investigations/` に分離します。

## 役割分担（joinir-architecture-overview との分離）

このファイルは「実装導線の地図」の SSOT です（navigation SSOT）。  
意味論・契約・不変条件の本文（normative）は `docs/development/current/main/joinir-architecture-overview.md` を SSOT とします。

使い分け:

- 「JoinIR が何を保証し、何を Fail-Fast で落とすべきか」→ `joinir-architecture-overview.md`
- 「どのファイルを触るべきか」「入口はどこか」「追加手順は？」→ この `joinir-design-map.md`
- 「Rust/.hako を同一契約でどう拡張するか（docs-first + gate-first）」→ `joinir-extension-dual-route-contract-ssot.md`
- 「経緯/ログ/切り分け」→ `docs/development/current/main/phases/` と `docs/development/current/main/investigations/`

---

## Design Notes（箱理論の“効いてるところ”）

このプロジェクトで JoinIR が効いている理由は、「PHI/CFG を一枚岩にせず、“意味の境界”を箱で分けている」点にある。

- スコープ解決の SSOT を固定する（検索順）: `ConditionEnv → LoopBodyLocalEnv → CapturedEnv → CarrierInfo`
  - 「なぜこの変数が見える/見えないか」を、層の契約として説明できるようにする
- ループは route family / shape contract を言語化して段階投入する（fixture + shape guard + Fail-Fast）
  - numbered route label を増やす代わりに、policy（family）で “同型” を吸収する
- Capability は “解禁の順序” を SSOT 化する（最小形→回帰で積み上げ）
  - 未対応は best-effort で誤魔化さず、Fail-Fast で理由を固定する

最近の改善（完了）:
- policy Reject の "hint" を `error_tags` に集約して、修正方針を 1 行で出せるようにした（Phase 109）
- 構造SSOT（LoopSkeleton + StepTree）へ寄せて、policy/step箱の増殖先を “構造” に集約する足場を追加した（Phase 110–112）
- StepTree 側を “変換可能なSSOT” へ段階的に拡張（cond AST handle / facts→contract / shadow parity / if-only emit / reads facts / return var）（Phase 119–124）

## Error Tags with Hints (Phase 109)

**SSOT**: error_tags is the single source for "tag + message + hint" errors.

**Policy**:
- policy/validator/merge use error_tags (no raw strings)
- hint is "1-line fix suggestion" only (no long explanations)
- Format: `[joinir/<category>/<tag>] <message>  Hint: <hint>`

**Examples**:
- `[joinir/phase107/balanced_depth_scan/missing_tail_inc] ... Hint: add 'i = i + 1' at top-level`
- `[joinir/phase100/pinned/reassigned] ... Hint: remove reassign, or promote to carrier`

## Recipe-first への収束（最終方向）

**JoinIR は「観測レイヤ」として薄くなる方向**。番号付き pattern 分岐は Recipe-first の route selection に置き換わる。

```
┌─────────────────────────────────────────────────────────────┐
│                     最終アーキテクチャ                       │
└─────────────────────────────────────────────────────────────┘

  AST (ソースコード)
   │
   │  ⛔ 直接触らない
   │
   ▼
┌─────────────────────────────────────────┐
│  JoinIR (観測レイヤ) ← 薄い！            │
│  ┌─────────────────────────────────────┐│
│  │ StepTree      │ AST の構造を安定化   ││
│  │ ControlForm   │ 制御フローを抽象化   ││
│  │ CondBlockView │ 条件の観測ビュー     ││
│  └─────────────────────────────────────┘│
│  役割: AST → Facts の橋渡しだけ          │
│  ⚠️ SSOT はここに置かない                │
└─────────────────────────────────────────┘
   │
   ▼
┌─────────────────────────────────────────┐
│  Facts 層                               │
│  ┌───────────────┐ ┌─────────────────┐ │
│  │ BodyShape     │ │ CondProfile     │ │
│  │ (enum: CFG用) │ │ (パラメータ化)  │ │
│  └───────────────┘ └─────────────────┘ │
└─────────────────────────────────────────┘
   │
   ▼
┌─────────────────────────────────────────┐
│  Verifier (唯一の受理) ← SSOT ここ       │
└─────────────────────────────────────────┘
   │
   ▼
┌─────────────────────────────────────────┐
│  VerifiedRecipe ← SSOT ここ             │
└─────────────────────────────────────────┘
   │
   ▼
┌─────────────────────────────────────────┐
│  Lower (機械的 CFG 生成)                 │
└─────────────────────────────────────────┘
   │
   ▼
  MIR (CFG + SSA)
```

**役割分担**:

| 層 | 役割 | SSOT? |
|----|------|-------|
| **JoinIR** | AST を安全に観測 | ❌ |
| **Facts** | Shape + CondProfile 抽出 | ❌ |
| **Verifier** | 受理判定 | ✅ |
| **Recipe** | 設計図 | ✅ |
| **Lower** | CFG 生成 | ❌ (Recipe を消費するだけ) |

SSOT: [`recipe-first-entry-contract-ssot.md`](./recipe-first-entry-contract-ssot.md)

---

## 1枚図: レイヤー（AST → JoinIR observation → Recipe/Verifier → MIR）【現行図】

```mermaid
flowchart LR
  A[AST] -->|Frontend observation| J1[JoinIR / StepTree / CondBlockView]
  J1 -->|Facts extraction| P1[Facts]
  P1 -->|Recipe build| P2[Recipe / VerifiedRecipe]
  P2 -->|Lower / Merge| M1[MIR Module]
  M1 --> B[Backend\n(VM / LLVM / Cranelift)]

  subgraph Frontend
    A
    J1
  end

  subgraph Plan
    P1
    P2
  end

  subgraph MIR Builder
    M1
  end
```

読み方:
- 「Loop/If の形が認識されない」: route 判定（Feature/Kind）と shape guard を見る
- 「JoinIR は生成できるが統合で壊れる」: Merge（ValueId/PHI/ExitLine/Boundary）と契約検証を見る
- 「なぜこのエラータグが出たか」: ErrorTags（SSOT）を起点に呼び出し元へ辿る

---

## North Star: Join-Explicit CFG Construction

JoinIR/MIR 間に生える “暗黙 ABI（順序/長さ/名前/役割）” を減らし、Join を第一級として扱う CFG へ収束させる。

SSOT（設計目標）:
- `docs/development/current/main/design/join-explicit-cfg-construction.md`

---

## “箱”の責務マップ（担当境界）

| 領域 | 役割（何を決めるか） | 主な入口/箱（SSOT寄り） | 主な出力 | Fail-Fast（典型） |
|---|---|---|---|---|
| Route判定 | ループ形を分類し、どの recipe/composer 経路に渡すか決める | active module surface `crate::mir::builder::control_flow::joinir::route_entry::router`, active module surface `crate::mir::builder::control_flow::joinir::route_entry::registry`, [`src/mir/builder/control_flow/plan/ast_feature_extractor.rs`](../../../../../src/mir/builder/control_flow/plan/ast_feature_extractor.rs), [`src/mir/builder/control_flow/plan/policies/`](../../../../../src/mir/builder/control_flow/plan/policies/), active module surface `crate::mir::loop_route_detection`（legacy physical path lane は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照） | `LoopRouteKind` / route hint / recipe entry | 「分類不能」→ 明示的に Err（サイレントな非JoinIR退避は禁止） |
| route contract / verifier | 「この route なら lower/merge 契約が成立する」を保証する | [`src/mir/builder/control_flow/plan/verifier/mod.rs`](../../../../../src/mir/builder/control_flow/plan/verifier/mod.rs), [`src/mir/builder/control_flow/plan/composer/coreloop_gates.rs`](../../../../../src/mir/builder/control_flow/plan/composer/coreloop_gates.rs), active module surface `crate::mir::builder::control_flow::joinir::route_entry::registry::predicates`（legacy physical path lane は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照） | verified route contract / 詳細診断 | 契約不一致を握りつぶさず Err |
| lowering | JoinIR/MIR へ機械的に落とす | [`src/mir/join_ir/lowering/mod.rs`](../../../../../src/mir/join_ir/lowering/mod.rs), [`src/mir/builder/control_flow/plan/composer/mod.rs`](../../../../../src/mir/builder/control_flow/plan/composer/mod.rs), [`src/mir/builder/control_flow/plan/lowerer/mod.rs`](../../../../../src/mir/builder/control_flow/plan/lowerer/mod.rs) | `JoinModule` / MIR frag | 未対応の構造は `error_tags::freeze(...)` 等で Err |
| merge | JoinIR→MIR 変換後、ホスト関数に統合する | [`src/mir/builder/control_flow/plan/conversion_pipeline.rs`](../../../../../src/mir/builder/control_flow/plan/conversion_pipeline.rs), [`src/mir/builder/control_flow/joinir/merge/mod.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/mod.rs) | ホスト MIR のブロック/ValueId 更新 | ValueId 競合、ExitLine 未接続、PHI 破綻を Err |
| ExitMeta | 「出口でどの carrier をどの host slot に戻すか」のメタ | [`src/mir/join_ir/lowering/carrier_info.rs`](../../../../../src/mir/join_ir/lowering/carrier_info.rs), [`src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs) | `ExitMeta` / `exit_bindings` | carrier 不整合（不足/過剰）を Err |
| CarrierInit | carrier 初期化の SSOT（FromHost/Const/LoopLocal） | [`src/mir/builder/control_flow/joinir/merge/carrier_init_builder.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/carrier_init_builder.rs), [`src/mir/join_ir/lowering/carrier_info.rs`](../../../../../src/mir/join_ir/lowering/carrier_info.rs) | 初期値 `ValueId` | 初期化経路の分岐が散らばらない（SSOT を使う） |
| ErrorTags | エラータグ整形の SSOT（検索性・一貫性） | [`src/mir/join_ir/lowering/error_tags.rs`](../../../../../src/mir/join_ir/lowering/error_tags.rs) | 文字列タグ | 文字列ハードコードを避け、タグを一元化 |

注:
- route 判定は「関数名 by-name 分岐」に依存しない（構造で決める）。必要なら dev/診断限定のガードに閉じ込める。
- shape guard は「OK なら後工程が前提にできる契約」を固定する場所で、曖昧な許容をしない。

---

## 入口（コード側のエントリポイント）

### Loop（builder 側の導線）

- Router（builder 入口）: [`src/mir/builder/control_flow/joinir/routing.rs`](../../../../../src/mir/builder/control_flow/joinir/routing.rs)
  - `MirBuilder::try_cf_loop_joinir(...)`（JoinIR ルートへ入る最初の関数）
  - `MirBuilder::cf_loop_joinir_impl(...)`（route classification → recipe-first router → plan lowering）
- Route router: active module surface `joinir::route_entry::router`（legacy physical path lane は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照）
- registry: active module surface `joinir::route_entry::registry`（legacy physical path lane は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照）
- Feature extraction: [`src/mir/builder/control_flow/plan/ast_feature_extractor.rs`](../../../../../src/mir/builder/control_flow/plan/ast_feature_extractor.rs)
- Planner/Composer/Lowerer（代表）:
  - [`src/mir/builder/control_flow/plan/single_planner/mod.rs`](../../../../../src/mir/builder/control_flow/plan/single_planner/mod.rs)
  - [`src/mir/builder/control_flow/plan/composer/mod.rs`](../../../../../src/mir/builder/control_flow/plan/composer/mod.rs)
  - [`src/mir/builder/control_flow/plan/lowerer/mod.rs`](../../../../../src/mir/builder/control_flow/plan/lowerer/mod.rs)
- 変換パイプライン（JoinIR→MIR→Merge の統一導線）:
  - [`src/mir/builder/control_flow/plan/conversion_pipeline.rs`](../../../../../src/mir/builder/control_flow/plan/conversion_pipeline.rs)
- Merge（統合の本体）: [`src/mir/builder/control_flow/joinir/merge/mod.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/mod.rs)
  - ExitLine: [`src/mir/builder/control_flow/joinir/merge/exit_line/mod.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/exit_line/mod.rs)
  - Merge の契約検証（debug）: [`src/mir/builder/control_flow/joinir/merge/contract_checks.rs`](../../../../../src/mir/builder/control_flow/joinir/merge/contract_checks.rs)

### JoinIR（IR/bridge）

- JoinIR 定義・入口: [`src/mir/join_ir/mod.rs`](../../../../../src/mir/join_ir/mod.rs)
- Route contract / verifier: [`src/mir/builder/control_flow/plan/verifier/mod.rs`](../../../../../src/mir/builder/control_flow/plan/verifier/mod.rs)
- JoinIR → MIR bridge: [`src/mir/join_ir_vm_bridge/mod.rs`](../../../../../src/mir/join_ir_vm_bridge/mod.rs)

### 共通（診断とタグ）

- Trace（JoinIR ルートの統一トレース）: [`src/mir/builder/control_flow/joinir/trace.rs`](../../../../../src/mir/builder/control_flow/joinir/trace.rs)
- Error tags（SSOT）: [`src/mir/join_ir/lowering/error_tags.rs`](../../../../../src/mir/join_ir/lowering/error_tags.rs)
- Loop Canonicalizer（前処理 SSOT）: [`src/mir/loop_canonicalizer/mod.rs`](../../../../../src/mir/loop_canonicalizer/mod.rs)
- ConditionOnly Derived Slot（Phase 93）: [`src/mir/join_ir/lowering/common/condition_only_emitter.rs`](../../../../../src/mir/join_ir/lowering/common/condition_only_emitter.rs)
- BodyLocalDerived Slot（Phase 94 / P5b）: [`src/mir/join_ir/lowering/common/body_local_derived_emitter.rs`](../../../../../src/mir/join_ir/lowering/common/body_local_derived_emitter.rs)

---

## 不変条件（Fail-Fast）

JoinIR を触るときは、次を破ったら「即エラーで止める」前提で設計・実装する。

### 形状（shape）

- Route recognizer は「認識できる shape だけ」を通し、曖昧な許容をしない。
- 形状が合わないときは `Ok(None)` で静かに進めない（非JoinIRへの退避を作らない）。
  - 例外: 明確な “routing” で「別 JoinIR 経路」を選ぶのは可（同一層内での選択）。
- depth-scan（Phase 107: `find_balanced_*`）は loop_break route policy で受理し、break 条件は “derived 値（depth_delta/depth_next）” から合成して SSOT 化する（by-name 分岐は禁止）。

### ValueId / PHI / Boundary の世界

- JoinIR 内部（JoinValueSpace 等）と host MIR builder の ValueId を混ぜない。
- Boundary は JoinIR↔host の橋渡し契約:
  - `join_inputs` と `host_inputs` の対応が明示される
  - Exit 側は “carrier 名” をキーにして reconnection される（ExitMeta/exit_bindings）
- PHI は「誰が確保するか」を固定し、衝突を許さない（PHI dst の予約・再利用禁止）。

### ExitLine 契約

- ExitLine は「出口へ集約する」ための契約であり、未接続の経路を残さない。
- carrier/slot の不足・余剰・不整合は `error_tags::exit_line_contract(...)` 等で即エラーにする。

### Allocator SSOT（Phase 135）

- **原則**: すべての ValueId 発行は単一の allocator（`ConditionContext.alloc_value`）を経由する
- **禁止事項**: ConditionLoweringBox / ExprLowerer での内部カウンタ使用
- **理由**: JoinIR params (ValueId(1000+)) と衝突し、merge 時に header PHI dst を上書きする
- **検出**: `--verify` で "Value %N defined multiple times" エラー
- **修正例**: Phase 135 P0 - ConditionLoweringBox が `&mut ConditionContext` を受け取り、alloc_value を必ず使用

### Boundary Injection SSA（Phase 135）

- **原則**: condition_bindings は alias を許すが、注入 Copy の dst は重複させない
- **Fail-Fast**: 異なる source が同一 dst に来る場合は即座にエラー
- **理由**: MIR SSA を破壊し、VM/LLVM で未定義動作を引き起こす
- **検出**: `joinir_inline_boundary_injector.rs` の重複排除ロジック
- **修正例**: Phase 135 P0 - Boundary Copy deduplication by dst

### Box 実装チェックリスト

Box を新規実装・変更した際は以下を必ず確認：

1. ✅ `--verify` で契約違反が検出されるか（SSA 破綻・ValueId 衝突）
2. ✅ smoke test で退行が出ないか（phase132/133/135 など）
3. ✅ allocator を bypass していないか（ConditionContext.alloc_value を使っているか）
4. ✅ boundary injection で dst 重複を防いでいるか

---

## 追加手順チェックリスト（新しいループ形を飲み込む最小手順）

「新しいループ形」を JoinIR で扱えるようにするときの最小手順。

1. Fixture を追加（再現可能に固定）
   - `apps/tests/` または `apps/smokes/` に最小の `.hako` を追加（対象形が一目で分かるもの）
2. Route/feature を追加（検出）
   - `src/mir/builder/control_flow/plan/ast_feature_extractor.rs`（必要なら feature 抽出を拡張）
   - active module surface `crate::mir::loop_route_detection`（legacy physical path lane は `docs/development/current/main/design/archive/route-physical-path-legacy-lane-ssot.md` を参照。分類/補助解析が必要ならここに追加）
3. shape guard を追加（契約の固定）
   - 形状・前提条件を validator として分離し、失敗は Err にする
4. lower を追加（JoinIR を生成）
   - 既存 pattern のコピーではなく、「今回の shape が要求する最小構成」にする
   - エラーは `src/mir/join_ir/lowering/error_tags.rs` を使いタグを固定する
5. merge/ExitLine を接続（契約が満たされるように）
   - carrier/ExitMeta/Boundary が揃っているか確認する
6. Tests を追加（仕様固定）
   - unit: route/validator/merge の局所テスト
   - smoke: `tools/smokes/v2/` の profile に軽いケースを追加（quick を重くしない）
7. Docs を更新（地図を更新）
   - `docs/development/current/main/loop_pattern_space.md`（legacy route-label ledger に追記が必要なら）
   - `docs/development/current/main/joinir-architecture-overview.md`（箱/契約が増えたなら）
   - 本ファイル（入口・責務マップの更新）

---

## Smoke（LLVM EXE）SSOT（integration）

LLVM EXE の integration smoke は、原則として共通ヘルパーに寄せる（重複禁止 / SKIP 規約の統一）。

- SSOT helper: `tools/smokes/v2/lib/llvm_exe_runner.sh`
  - LLVM 前提チェック（`llvm-config-18` / `llvmlite` / `--backend llvm`）
  - 必須プラグインの dlopen gating + 必要時だけ `tools/plugins/build-all.sh`
  - build → run → 数値行だけ抽出して比較（デバッグログ混入耐性）

## スコープ解決の SSOT（Pinned Read‑Only Captures）

JoinIR lowering では「ループ内で参照される値」を次の層で解決する（探索順 SSOT）:

1. `ConditionEnv`（条件式に必要な値）
2. `LoopBodyLocalEnv`（ループ body 内で初期化される一時変数）
3. `CapturedEnv`（ループ外から入ってくる read‑only 入力）

`CapturedEnv` は “読み取り専用入力” の SSOT として扱い、内部で区別する:
- `Explicit`（従来の capture）
- `Pinned`（ループ外 local を loop 内 receiver として参照するための read‑only capture）

Fail‑Fast（Pinned）:
- loop body 内で再代入される変数は pinned 禁止
- loop entry 時点で host 側 ValueId が無い場合は拒否（黙って skip しない）

設計メモ: `docs/development/current/main/phases/phase-100/README.md`
