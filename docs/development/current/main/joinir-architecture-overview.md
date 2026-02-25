# JoinIR Architecture Overview (2025‑12‑08)

このドキュメントは、JoinIR ライン全体（Loop/If lowering, ExitLine, Boundary, 条件式 lowering）の
「箱」と「契約」を横串でまとめた設計図だよ。selfhost / JsonParser / hako_check など、
どの呼び出し元から見てもここを見れば「JoinIR 層の責務と流れ」が分かるようにしておく。

変更があったら、Phase ドキュメントではなく **このファイルを随時更新する** 方針。

併用ドキュメント（役割分担）:

- **設計の正本（契約/不変条件/箱の責務）**: この `joinir-architecture-overview.md` を SSOT とする。
- **実装導線の地図（どのファイルを触るか/入口一覧/追加手順）**:  
  `docs/development/current/main/design/joinir-design-map.md` を参照する（ここには“場所”を書き、契約本文は本ファイルに寄せる）。
- docs の置き場所ルール（SSOT）: `docs/development/current/main/DOCS_LAYOUT.md`
- **JoinIR の役割 SSOT（観測レイヤ）**:  
  `docs/development/current/main/design/joinir-observation-layer-ssot.md` を参照する（JoinIR は観測のみで、Recipe/Verifier/Lower の SSOT は持たない）。

---

## 0.0 収束形（Target Shape / Convergence）

JoinIR/CFG 合成が「裾広がり」せずに収束していくための **目標形（target shape）** をここで固定する。
通称: **Plan→Frag パイプライン**（会話・作業ログではこの短い呼び名を使う）。

狙いは「要素が増えても、本線が増えない」こと：
- 増えてよい: extractor（検出）だけ
- 増えてはいけない: block/value/PHI/terminator 生成の分岐点（= 生成本線の if/分岐増殖）

### 0.0.1 パイプライン（収束形）

```
           ┌──────────────────────────────┐
AST/Stmt → │  Plan Extractor Box (pure)   │
           │  - patterns are just extractors
           │  - no block/value allocation
           │  - returns: Ok(None)/Ok(Plan)/Err
           └──────────────┬───────────────┘
                          │
                          v
           ┌──────────────────────────────┐
           │  Plan Verifier Box (fail-fast)│
           │  - phase gating (P0/P1)
           │  - invariants (no ambiguity)
           └──────────────┬───────────────┘
                          │
                          v
           ┌──────────────────────────────┐
           │  Plan Lowerer Box (only builder)│
           │  - alloc blocks/values/phi
           │  - builds Frag via small comb.
           │  - uses Expr/Scope boxes inside blocks
           └──────────────┬───────────────┘
                          │
                          v
           ┌──────────────────────────────┐
           │  EdgeCFG Frag + emit_frag()  │
           │  - terminator SSOT
           │  - succ/pred sync
           └──────────────┬───────────────┘
                          │
                          v
           Verify / DCE / CFG update / print
           (terminator operand only)
```

注記（名前の揺れを防ぐ）:
- **PlanFreeze** という呼び名を使う場合、意味は **Normalizer + Verifier の合成（凍結点）**。
- 目的は「一致宣言（Ok(Some)）した後は後戻りせず、**DomainPlan →（変換+検証）→ FrozenCorePlan** に確定する」こと。
- 実装上は分割して持ってもよいが、会話や指示書では “凍結点” を **PlanFreeze** と呼ぶと迷子が減る。

PlanFreeze 版（同じ収束形、箱名だけ明確化）:

```
           ┌──────────────────────────────┐
AST/Stmt → │  Plan Extractor (pure)       │
           │  - Ok(None)/Ok(DomainPlan)/Err
           └──────────────┬───────────────┘
                          │
                          v
           ┌──────────────────────────────┐
           │  PlanFreeze (=Normalizer+Verifier) │
           │  - DomainPlan → FrozenCorePlan     │
           │  - close-but-unsupported => Err    │
           └──────────────┬───────────────┘
                          │
                          v
           ┌──────────────────────────────┐
           │  Plan Lowerer (only builder) │
           └──────────────┬───────────────┘
                          v
           ┌──────────────────────────────┐
           │  Frag + compose::* + emit_frag() │
           └──────────────────────────────┘
```

### 0.0.2 「収束している」と呼ぶ条件（定義）

この定義を満たしている状態を、JoinIR/CFG 合成の「収束」と呼ぶ：

1) **pattern は Plan 抽出へ降格する**
   - pattern は「一致判定 + Plan を返す」だけ（builder を触らない）。

2) **CFG 生成（block/value/PHI）は PlanLowerer に一本化する**
   - `next_block_id()` / `next_value_id()` / PHI 挿入 / `emit_frag()` 呼び出しは PlanLowerer 側だけに置く。

3) **terminator 生成点は `emit_frag()` のみ（SSOT）**
   - Branch/Jump/Return を “別の場所で” 生成しない。

4) **増えても本線が増えない（成長境界）**
   - 新しい loop/if の形が増えても、増えるのは extractor（薄いファイル）だけ。
   - Plan の語彙と Lowerer/emit の本線は増殖しない（分岐増殖は設計上のバグとして扱う）。

参照（相談メモ / 背景）:
- `docs/development/current/main/investigations/phase-272-frag-plan-architecture-consult.md`

### 0.0.3 現状の実装（2本の lowering line）

現状は移行期間のため、入口が 2 本ある（=「2本のコンパイラ」になりやすい状態）。ここを **Phase 286** で 1 本に収束させる。

- **Plan line（route=plan）**
  - 対象: Pattern6/7（Plan-based）
  - 入口: `src/mir/builder/control_flow/joinir/patterns/router.rs`（`route=plan ...`）
  - SSOT: `src/mir/builder/control_flow/plan/*` → `Frag + compose::* + emit_frag()`

- **JoinIR line（route=joinir）**
  - 対象: Pattern1–5,8–9（JoinIR table-based）
  - 入口: `src/mir/builder/control_flow/joinir/patterns/router.rs`（`route=joinir ...`）
  - 共通入口（変換/merge の集約点）: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`

注意:
- 「return/break/continue のような “大きい出口語彙”」を実装する時、どちらの line に効く修正かを先に固定しないと迷子が再発する。
- Phase 284 は “ExitKind へ収束” を先に決め、Phase 286 で line 自体を吸収して 1 本化する。

## 0. 読み方ガイド（Reader's Guide）

このファイルは情報量が多いので、「何を知りたいか」で読む場所を分けると楽だよ：

- 注意: 本文のセクション番号は歴史的経緯で重複することがあるため、参照は「見出し名」を基本にする。

- **不変条件だけ押さえたいとき**
  - セクション `1. 不変条件（Invariants）` を読む。
  - JoinIR 全体で絶対に守るルール（ValueId 領域 / PHI 契約 / Fail‑Fast）がここにまとまっている。

- **箱ごとの責務と流れを掴みたいとき**
  - セクション `2. 主な箱と責務` を読む。
  - LoopPattern / Pattern lowerer / ConditionEnv / ExitLine / Boundary など、構造箱・判定箱・生成箱の入口がここ。

- **Normalized JoinIR（JoinIR→JoinIR 正規化）を知りたいとき**
  - 見出し `JoinIR 第1章：基盤完成サマリ` と  
    小節 `Structured vs Normalized`（Structured/Normalized の関係）を読む。
  - 詳細な完成サマリは `PHASE_43_245B_NORMALIZED_COMPLETION.md` にまとまっているので、そちらも合わせて参照してね。

- **JsonParser/selfhost のループ状況を見たいとき**
  - 見出し `JoinIR 第1章：基盤完成サマリ` 内の JsonParser/selfhost 表と、`phase181-jsonparser-loop-roadmap.md` を見る。

- **各 Phase ごとの細かい経緯・ログが欲しいとき**
  - `docs/development/current/main/phase*-*.md` 系の Phase ドキュメントを読む。
  - 本ファイルの `3.x` にある Phase ロードマップは「索引」としての位置づけで、詳細は各 Phase doc 側が SSOT だよ。

---

## 0.1 用語・デバッグフラグ（Glossary & Debug Flags）

### 用語（routing / fallback / fail-fast）

本文では「fallback」という単語が文脈でぶれやすいので、先に用語を固定する：

- **routing**: JoinIR Core 内での正規な経路選択（Pattern ルータ、if-sum mode ↔ legacy P3 など）。理由が分かる形でログ/タグを付ける。
- **soft fallback**: 任意箱（pre-validation / optional optimizer）が失敗したときに、同等意味論の別 JoinIR 経路へ退避すること（JoinIR Core 外へは出ない）。
- **prohibited fallback**: 非 JoinIR への退避（例: LoopBuilder）・サイレント退避・契約違反の握りつぶし。
- **SSOT** (Single Source of Truth): 設計・実装・テストの情報が集約される唯一の場所（e.g., `is_joinir_debug()` for debug flag reading）。Phase ドキュメントより SSOT ドキュメントを優先参照。
- **Fail-Fast**: エラー条件を明示的に検出・報告し、サイレント退避や暗黙的フォールバックを禁止する設計原則。

### デバッグフラグ（Phase 82 SSOT 統一）

JoinIR デバッグログを有効にするには **`HAKO_JOINIR_DEBUG=1`** を使用（`NYASH_JOINIR_DEBUG=1` は deprecated）:

```bash
# 推奨（Phase 82 以降）
HAKO_JOINIR_DEBUG=1 cargo test --release --lib

# Legacy（Phase 82 以前、後方互換保証）
NYASH_JOINIR_DEBUG=1 cargo test --release --lib
```

実装: `src/config/env/joinir_flags.rs::is_joinir_debug()` が両者をチェック。
詳細: `docs/development/current/main/phase82-83-debug-flag-ssot-summary.md`

## 0.2 Pattern Number Absorption Destination (Phase 280)

**Status**: Active (2025-12-23)
**Purpose**: Stop pattern enumeration proliferation by establishing Frag composition as SSOT

### The Problem: Pattern Enumeration Proliferation

Pattern numbers (1-9+) became architectural decision points:
- Router branches exploded (17+ patterns across JoinIR/Plan routes)
- Each pattern duplicated CFG construction logic (block allocation, PHI insertion, terminator emission)
- "Pattern-specific" knowledge leaked into lowering layers
- Adding new loop shapes required full-stack pattern additions

**Symptom**: Pattern numbers drove architecture instead of being test labels

### The Solution: Frag Composition SSOT

**Key Insight**: Pattern numbers → symptom labels (test names), CFG construction → Frag composition API

**Architecture shift**:
- **Before Phase 280**: Pattern number → entire lowering pipeline (extractor + allocator + emitter)
- **After Phase 280**: Pattern number → extractor only, all lowering uses Frag composition SSOT

**Composition API as absorption destination**:
- `seq(a, b)`: Sequential composition (Normal wiring)
- `if_(header, cond, t, e, join)`: Conditional composition (Branch wiring)
- `loop_(loop_id, header, after, body)`: Loop composition (Break/Continue wiring)
- `cleanup(main, cleanup_frag, normal_target, ret_target)`: Cleanup composition (Normal/Return wiring; Phase 281)

**Reference**: `docs/development/current/main/design/edgecfg-fragments.md` (Active SSOT)

### JoinIR vs Plan: Different Extraction, Same SSOT

Both routes converge on the same Frag composition SSOT:

| Route | Extraction Source | Pattern Knowledge | Composition SSOT |
|-------|-------------------|-------------------|------------------|
| **JoinIR** | cf_loop structure (Structured JoinIR) | JoinIR-specific (cf_loop DSL) | **Frag API** (seq/if/loop) |
| **Plan** | DomainPlan (Pattern6/7 extractors) | Domain-specific (ScanWithInit/SplitScan) | **Frag API** (same) |

**Key principle**: Different extraction strategies, converged CFG construction

**Why separate routes?**:
- JoinIR route: Handles cf_loop-based patterns (Pattern1-5, 8-9) via Structured JoinIR
- Plan route: Handles complex scan patterns (Pattern6/7) via DomainPlan → CorePlan → Frag
- Both routes use same Frag composition API for CFG lowering (no duplication)

### Pattern Absorption Status (Phase 281)

| Pattern | Structure | Status | Absorption Target |
|---------|-----------|--------|-------------------|
| **Pattern6** | ScanWithInit (early-exit) | ✅ compose-based (Phase 281) | absorbed (hand-roll removed) |
| **Pattern7** | SplitScan (then/else join) | ✅ compose-based (Phase 281) | absorbed (hand-roll removed) |
| Pattern8 | BoolPredicateScan (is_integer) | JoinIR-based | Phase 283+ (migration planned) |
| Pattern9 | AccumConstLoop (bridge) | JoinIR-based (Phase 271) | 撤去条件 defined (minimal loop SSOT) |
| Pattern1-5 | Legacy (SimpleWhile, Break, IfPhi, Continue, InfiniteEarlyExit) | JoinIR-based | Test/error stubs (not absorbed) |

**Phase 281 Result**: Pattern6/7 absorbed (compose SSOT), hand-rolled Frag eliminated

**Absorption criteria** (Pattern6/7 → Frag composition):
1. Hand-rolled Frag construction identified (function name + 識別コメント)
2. TODO comments documenting future compose_* migration path
3. Behavior-preserving refactor completed in Phase 281

### Absorption Timeline

**Phase 280 (Complete)**: SSOT positioning + 導線固定
- A: Documentation (edgecfg-fragments.md → Active SSOT)
- B: API solidification (compose.rs contract verification)
- C: Pattern preparation (Pattern6/7 hand-rolled locations documented)
- **Goal**: Establish Frag composition as THE absorption destination
- **Non-Goal**: Full Pattern6/7 migration (deferred to Phase 281)

**Phase 281 (Complete)**: Full Pattern6/7 absorption
- Pattern7: body cond_match → `compose::if_()`
- Pattern6: early-exit → `compose::cleanup()`（Normal/Return wiring）
- Behavior-preserving verification (VM/LLVM smokes)

**Phase 282 (Planned)**: Router shrinkage
- Pattern numbers → test labels only
- Router uses Frag composition for all CFG construction
- Pattern extractors remain as thin detection layer

**Phase 283+ (Future)**: Pattern8 and beyond
- Migrate Pattern8 to Plan route or Frag-based JoinIR
- Evaluate Pattern9 撤去 (if minimal loop SSOT achieved)
- Continue pattern number reduction

---

## 1. 不変条件（Invariants）

JoinIR ラインで守るべきルールを先に書いておくよ：

**ValueId の 3 つの世界** (明文化):
1. **JoinIR 内部**: JoinParam (100-999) / JoinLocal (1000+) のみ。alloc_param()/alloc_local() が SSOT。
2. **MIR builder**: builder.variable_map で管理。JoinInst の operand には持ち込まない。
3. **PHI dst (メタ)**: LoopHeader PHI の dst は MIR builder が割り当て。JoinIR の reserve_phi() は衝突防止マークのみ。

**語彙統一**:
- `alloc_value()` は `alloc_local()` の alias（実装上、概念上は Local を増やす）
- `LOCAL_MAX` は debug/verifier 上限（概念上無限ではない、図と実装の関係を明記）

**Routing vs Fallback の語彙 SSOT** (Section 0.1 と整合):
- **routing**: JoinIR Core 内での正規な経路選択（Pattern1/2/3/4 ルータ、if-sum mode 選択等）
- **soft fallback**: 任意箱失敗時の同等意味論退避（例: pre-validation 失敗 → 別 JoinIR 経路）
- **prohibited fallback**: 非 JoinIR への退避（LoopBuilder へ落ちる等）、サイレント退避、契約違反握りつぶし

不変条件 6 (Fail-Fast) はこの定義に基づく: prohibited fallback は絶対禁止、soft fallback は理由ログ必須。

---

1. **JoinIR 内部は JoinIR ValueId だけ**
   - JoinIR lowering が発行する `JoinInst` の `ValueId` は JoinValueSpace の領域（Param/Local）のみを使う。
     - `alloc_param()`（Param 100–999）/ `alloc_local()`（Local 1000+）が SSOT。
     - 多くの lowerer が使う `alloc_value()` は「実装上の別名（Local を増やす）」として扱う（概念上は `alloc_local()`）。
   - Rust/MIR 側の ValueId（例: `builder.variable_map`）は、JoinInst の operand としては持ち込まない。
   - 例外（ただし JoinInst には出現しない）: LoopHeader PHI dst は merge(MIR) 段階で確保される MIR ValueId で、JoinValueSpace は `reserve_phi()` で衝突防止のマークだけを行う。

2. **host ↔ join の橋渡しは JoinInlineBoundary 系だけ**
   - host から JoinIR への入力（ループ変数 / 条件専用変数）は
     - `JoinInlineBoundary.join_inputs + host_inputs`
     - `JoinInlineBoundary.condition_bindings`（ConditionBinding）
     だけで接続する。
   - 出力（キャリアの出口）は `JoinInlineBoundary.exit_bindings` に一本化する。

3. **LoopHeader PHI を SSA の単一源泉にする**
   - ループ変数とキャリアは **LoopHeaderPhiBuilder/LoopHeaderPhiInfo** でヘッダ PHI を作り、これを「現在値」の SSOT にする。
   - exit 用の PHI も組むが、変数再接続や expr_result 収集はヘッダ PHI を経由して行う（SSA‑undef 防止）。

4. **式としての戻り値とキャリア更新を分離する**
   - 「ループが式として値を返す」ケース（例: `let r = loop_min_while(...)`）の出口は **exit_phi_builder** が扱う。
   - 「ループが状態更新だけする」ケース（例: `trim` の `start/end`）の出口は **ExitLine（ExitMeta / ExitBinding / ExitLineReconnector）** だけが扱う。
   - promoted carriers（例: DigitPos/Trim の `ConditionOnly`）は exit PHI から除外し、ExitLineReconnector は LoopState のみを reconnect する（Phase 81 で E2E 検証）。
     - 参照: `docs/development/current/main/phase81-pattern2-exitline-contract.md`

5. **ループ制御 vs 条件式の分離**
   - ループの「形」（Pattern1–4, LoopFeatures）は control-flow 専用の箱が担当。
   - 条件式（`i < len && (ch == " " || ch == "\t")` 等）は **BoolExprLowerer / condition_to_joinir** が担当し、
     ループパターンは boolean ValueId だけを受け取る。

6. **Fail‑Fast**
   - JoinIR が対応していないループパターン / if パターンは、必ず `[joinir/freeze]` 等で明示的にエラーにする。
   - LoopBuilder 等へのサイレントフォールバックは禁止。

7. **Param の役割（ParamRole）を分ける**
   - JoinIR 側で扱うパラメータは概念的に 3 種類に分かれる：
     - 条件専用（Condition param）: 継続条件や break 条件だけに使う値
     - キャリア（Carrier param）: ループ状態（pos/result など）として更新される値
     - 式結果（Expr param）: ループが式として返す値
   - ExitLine / Header PHI / InstructionRewriter は **Carrier param だけ** を対象にし、Condition/Expr param は上書きしない。
   - 現状は ConditionBinding/ExitMeta/JoinFragmentMeta で役割を区別しており、将来 ParamRole enum として明示する予定。

8. **LoopHeader PHI dst は予約領域（上書き禁止）**
   - LoopHeaderPhiBuilder が生成したヘッダ PHI の dst ValueId は「現在のループ値」の SSOT として扱い、
     BoundaryInjector や InstructionRewriter が `Copy` などで二度書きすることを禁止する。
   - merge ラインでは「ヘッダ PHI dst に対する新しい定義が出てきたら debug モードで panic する」ことで契約違反を早期検出する。

9. **ParamRole の不変条件（Phase 200-A 追加）**
   - **Condition 役のパラメータは「PHI dst にしてはいけない」**
     - 理由: 条件専用変数はループ内で更新されない（例: `digits` in `_atoi()`）
     - LoopHeaderPhiBuilder は Condition 役の変数に対して header PHI を生成しない
   - **Condition 役のパラメータは「ExitLine の対象にも入れない」**
     - 理由: 条件専用変数はループ外で使われない（ループ内でのみ参照）
     - ExitLineReconnector は Condition 役の変数を exit_bindings から除外
   - **ParamRole の分類**:
     - `LoopParam`: ループ制御変数（例: `i` in `loop(i < len)`）→ header PHI + exit_bindings
      - `Condition`: 条件専用変数（例: `digits` in `digits.indexOf(ch)`）→ condition_bindings のみ
      - `Carrier`: 状態更新変数（例: `sum`, `count`）→ header PHI + exit_bindings
      - `ExprResult`: ループ戻り値 → exit_phi_builder で処理

10. **JoinIR Core は常時 ON**
    - LoopBuilder は物理削除済み。JoinIR を OFF にする経路や prohibited fallback（非 JoinIR への退避）は存在しない。
    - `NYASH_JOINIR_CORE` は deprecated（0 を指定しても警告して無視）。JoinIR の OFF トグルは提供しない。

11. **Phase 220: P3 if-sum の ConditionEnv 統合完了**
    - **LoopBuilder へのフォールバック経路は Phase 187 で完全削除済み**。
    - **P3 if-sum には ConditionPatternBox + ConditionEnv が必須**:
      - 単純条件（`var CmpOp literal`）のみ if-sum mode へルーティング（Phase 219-fix）。
      - 複雑条件（`i % 2 == 1` 等）は legacy P3 経路へ routing（JoinIR Core 内）。
    - **Phase 220-D で loop 条件の変数サポート完了**:
      - `loop(i < len)` のような変数条件を ConditionEnv で解決。
      - `extract_loop_condition()` を ValueOrLiteral 対応に拡張。
    - **Phase 220 で if-sum の expr-result exit contract が P2 と揃った**:
      - ExprResultResolver Box で expr_result routing を統一化（Phase 221-R）。
      - phase212_if_sum_min.hako → RC=2 達成。

12. **Lexical scope（Phase 68）を host 側で実在化する**
    - JoinIR の正当性は「上流の束縛モデル」が壊れていないことを前提にする。
    - MIR builder は `{...}`（Program）/ `ScopeBox` を lexical scope として扱い、`local` shadowing を正しく復元する。
    - “未宣言名への代入はエラー” を SSOT（quick-reference/LANGUAGE_REFERENCE）に揃えて Fail-Fast 化する。
    - 参照:
      - `docs/development/current/main/phase67-mir-var-identity-survey.md`
      - `docs/reference/language/variables-and-scope.md`
    - 次の焦点（Phase 73+）:
      - JoinIR lowering の `ScopeManager` を「名前ベース」から「BindingId ベース」に段階移行し、shadowing/束縛同一性を JoinIR 側でも安全に扱えるようにする。
      - 参照: `docs/development/current/main/phase73-scope-manager-design.md`
      - promoted carriers（DigitPos/Trim）の “synthetic name” 問題を BindingId で接続する（dev-only）:
        - 参照: `docs/development/current/main/phase78-bindingid-promoted-carriers.md`
      - 次の配線計画（P3/P4 拡張）:
        - 参照: `docs/development/current/main/phase80-bindingid-p3p4-plan.md`

13. **Ownership/Relay（Phase 56–71）: carrier/capture/relay を契約化する（dev-only）**
    - `OwnershipPlan` は carriers/captures/relay_writes の SSOT として使い、混線を Fail-Fast で検出する。
    - multihop relay は plan 層では受理できるが、runtime 側で未対応の間は標準タグで Fail-Fast に落とす（Phase 70-A）。
      - 標準タグ: `[ownership/relay:runtime_unsupported]`
    - `OwnershipPlanValidator` を箱として隔離し、Pattern lowerer 側から再利用できる形にする（Phase 71-Pre）。
    - 参照:
      - `docs/development/current/main/phase56-ownership-relay-design.md`
      - `docs/development/current/main/phase65-ownership-relay-multihop-design.md`
      - `docs/development/current/main/phase70-relay-runtime-guard.md`

---

## 1.9 ValueId Space Management (Phase 201)

JoinIR の ValueId 割り当ては **JoinValueSpace** で一元管理され、3つの領域に分離されているよ：

### 1.9.1 ValueId 空間のダイアグラム

```
JoinValueSpace Memory Layout:

 0          100        1000                     LOCAL_MAX
 ├──────────┼──────────┼──────────────────────────┤
 │  PHI     │  Param   │       Local             │
 │  Reserved│  Region  │       Region            │
 └──────────┴──────────┴──────────────────────────┘

PHI Reserved (0-99):
  - LoopHeader PHI dst 用の「予約マーカー領域」（JoinValueSpace 側の契約）
  - `reserve_phi(dst)` で「この MIR ValueId を PHI dst として予約した」というマークだけを行う（JoinValueSpace は PHI dst を割り当てない）
  - 注: 実際の PHI dst は host MirBuilder が割り当てるため、0-99 に入ることは “保証” ではない（Phase 72 観測: `docs/development/current/main/phase72-phi-reserved-observation.md`）

Param Region (100-999):
  - alloc_param() で割り当て
  - 使用箇所: ConditionEnv, CarrierInfo.join_id, CapturedEnv

Local Region (1000..=LOCAL_MAX):
  - alloc_local() で割り当て
  - 使用箇所: Pattern lowerers (Const, BinOp, etc.)
```

### 1.9.2 JoinValueSpace の役割

- **単一の真実源 (SSOT)**: すべての JoinIR ValueId 割り当てを一箇所で管理
- **領域分離**: Param ID、Local ID、PHI dst が決して重複しない
- **契約検証**: デバッグモードで違反を早期検出
- **後方互換性**: 既存 API は継続動作
- 注意: `LOCAL_MAX` はデバッグ/検証用の上限。概念上は Local 領域は拡張可能で、必要なら上限を増やす。

### 1.9.3 各コンポーネントと ValueId 領域の対応表

| コンポーネント | 使用領域 | 割り当て方法 | 用途 |
|--------------|---------|------------|------|
| ConditionEnv | Param (100-999) | `alloc_param()` | ループ条件変数の JoinIR ValueId |
| CarrierInfo.join_id | Param (100-999) | `alloc_param()` | キャリア変数の JoinIR ValueId |
| CapturedEnv | Param (100-999) | `alloc_param()` | 関数スコープ変数の JoinIR ValueId |
| Pattern 1 lowerer | Local (1000+) | `alloc_local()` | 中間値（Const, Compare, etc.） |
| Pattern 2 lowerer | Local (1000+) | `alloc_local()` | 中間値（Const, BinOp, etc.） |
| Pattern 3 lowerer | Local (1000+) | `alloc_local()` | 中間値（PHI, Select, etc.） |
| Pattern 4 lowerer | Local (1000+) | `alloc_local()` | 中間値（Select, BinOp, etc.） |
| LoopHeaderPhiBuilder | PHI Reserved (0-99) | `reserve_phi()` | PHI dst ID 保護（上書き防止） |

### 1.9.4 設計原則

1. **領域の固定境界**
   - 明確な境界（100, 1000）で領域を分離
   - デバッグが容易（ValueId を見ればどの領域か一目瞭然）
   - アロケータ間の調整不要

2. **reserve_phi() vs alloc_phi()**
   - LoopHeader PHI dst は merge(MIR) 段階で確保される **MIR 側 ValueId**（JoinInst の ValueId とは世界が違う）。
   - JoinValueSpace は PHI dst を「割り当てる」箱ではなく、`reserve_phi(dst)` で予約マークし、JoinIR Param/Local の割当と衝突しないようにする。
   - `reserve_phi()` はマーカーのみ（「この ID を JoinIR 側で上書き/再利用するな」という契約）。

3. **value_id_ranges.rs との関係**
   - `value_id_ranges.rs`: **モジュールレベルの分離**（min_loop, skip_ws 等の各モジュールに大きな固定範囲を割り当て）
   - `JoinValueSpace`: **lowering 内部の分離**（param vs local vs PHI）
   - 両者は相補的な役割

### 1.9.5 Phase 205: 領域契約の検証強化

**追加された Box-First 機能**:

1. **衝突検出（debug-only）**
   - 全ての割り当てられた ValueId を追跡（`allocated_ids: HashSet<u32>`）
   - 重複割り当てを即座に検出し panic（Fail-Fast 原則）
   - `check_collision()` で実装

2. **領域検証（debug-only）**
   - `verify_region(id, expected_region)` で ValueId が期待される領域にいるか検証
   - 違反時は明確なエラーメッセージと修正ヒントを提供
   - 例: "ValueId(500) is in Param region, expected Local. Hint: Use alloc_local() for JoinIR values"

3. **RegionVerifier Box**
   - 場所: `src/mir/builder/control_flow/joinir/merge/mod.rs::verify_valueid_regions()`
   - 責務: merge 時に boundary と loop_info の ValueId 領域契約を検証
   - 検証項目:
     - 全ての `boundary.join_inputs` が Param 領域（100-999）にいる
     - 全ての `condition_bindings[].join_value` が Param 領域にいる
     - 全ての `carrier_phis[].phi_dst` が「JoinIR 側の Param/Local と衝突しない」こと
       - 現状は “偶発的な非衝突（MirBuilder が低番、JoinValueSpace が 100+）” で安定している（Phase 72 観測: `docs/development/current/main/phase72-phi-reserved-observation.md`）。
       - 検証強化の是非は Phase 72 を SSOT にして別フェーズで決める（ここで前提を固定しない）。

4. **明示的な領域定数**
   ```rust
   pub const PHI_RESERVED_MIN: u32 = 0;
   pub const PHI_RESERVED_MAX: u32 = 99;
   pub const PARAM_MIN: u32 = 100;
   pub const PARAM_MAX: u32 = 999;
   pub const LOCAL_MIN: u32 = 1000;
   pub const LOCAL_MAX: u32 = 100000;
   ```

**Fail-Fast 原則の実装**:
- 領域違反は即座に panic（デバッグモード）
- フォールバックやサイレント修正は一切行わない
- エラーメッセージに具体的な修正方法を含める

詳細は `src/mir/join_ir/lowering/join_value_space.rs` と `phase205-valueid-regions-design.md` を参照。

---

## 2. 主な箱と責務

### 2.1 Loop 構造・検出ライン

- **LoopFeatures / LoopPatternKind / router**
  - ファイル:
    - `src/mir/loop_pattern_detection.rs`
    - `src/mir/builder/control_flow/joinir/patterns/router.rs`
    - `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`
  - 責務:
    - AST から break/continue/if‑else PHI などの特徴を抽出（ast_feature_extractor）。
    - `classify(&LoopFeatures)` で Pattern1–4 に分類し、テーブル駆動の `LOOP_PATTERNS` でルーティング。
    - ルータ順序は P4(continue) → P3(if‑phi) → P1(simple) → P2(break) で固定（優先度フィールドはデバッグ用）。

- **Pattern Lowerers (Pattern1–4)**
  - ファイル例:
    - `pattern1_minimal.rs`（Simple while）
    - `pattern2_with_break.rs`（break 付き / Trim 昇格パスを含む）
    - `pattern3_with_if_phi.rs`（if‑phi キャリア）
    - `pattern4_with_continue.rs`（continue を Select で表現）
  - 責務:
    - LoopScopeShape / AST / LoopFeatures を入力として JoinIR の `JoinModule` を構築。
    - `JoinFragmentMeta{ expr_result, exit_meta }` を返し、出口情報を ExitLine に渡す。
    - host/MIR の ValueId は一切扱わない（JoinIR ローカルの ValueId のみ）。

- **Scope / Env Builders**
  - `loop_scope_shape_builder.rs`: ループ本体ローカルの収集、LoopScopeShape 統一生成。
  - `condition_env_builder.rs`: 条件専用変数の環境と ConditionBinding を一括構築。

- **CommonPatternInitializer** (Phase 33-22)
  - ファイル: `src/mir/builder/control_flow/joinir/patterns/common_init.rs`
  - 責務:
    - 全 Pattern 共通の初期化ロジック統一化（ループ変数抽出 + CarrierInfo 構築）。
    - 全パターンで boundary.loop_var_name を確実に設定し、SSA‑undef を防ぐ。

- **PatternPipelineContext** (Phase 179-B)
  - ファイル: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`
  - 責務:
    - 全 Pattern の前処理結果を格納する「解析済みコンテキスト箱」。
    - CommonPatternInitializer + LoopScopeShapeBuilder の結果を統一的に保持。
    - Pattern 1-4 の共通データ（loop_var_name, loop_var_id, carrier_info, loop_scope）を提供。
    - Pattern 2/4 専用データ（condition_env, carrier_updates, trim_helper）は Option<T> で保持。
    - **Analyzer-only dependencies**: 解析ロジックのみ依存、JoinIR emission ロジックは含まない。

- **TrimLoopLowerer (P5 Dedicated Module)** (Phase 180)
  - ファイル: `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs`
  - 責務:
    - Trim/CharComparison (Pattern 5) 専用の lowering ロジックを一箇所に集約。
    - Pattern2/4 から呼ばれ、LoopBodyLocal 変数を carrier に昇格し、Trim パターンの break 条件を置き換える。
    - TrimPatternValidator/TrimPatternLowerer を内部で利用し、carrier 初期化コード生成・条件式置換を実行。
  - 入力:
    - MirBuilder, LoopScopeShape, loop_cond, break_cond, body, loop_var_name, carrier_info, alloc_join_value
  - 出力:
    - `Some(TrimLoweringResult)`: Trim パターン検出・lowering 成功時（置換後条件、更新 carrier_info、condition_bindings）
    - `None`: Trim パターンでない場合（通常ループ処理に戻る）
    - `Err`: Trim パターン検出したが lowering 失敗時
  - 使用元:
    - Pattern2 (pattern2_with_break.rs): Trim/P5 ロジックを完全委譲（~160 行削減）
    - Pattern4 (pattern4_with_continue.rs): 将来の Phase 172+ で Trim lowering 実装時に利用予定
  - デザイン原則:
    - Pure analysis container（前処理結果のみ保持、emission なし）
    - Pattern-specific variants（Option<T> でパターン固有データ管理）
    - Single source of truth（全パターンが同じ前処理経路を使用）

- **JoinIRConversionPipeline** (Phase 33-22)
  - ファイル: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`
  - 責務:
    - JoinIR → MIR 変換フロー統一化（JoinModule → MirModule → merge_joinir_mir_blocks）。
    - JoinIR/MIR の関数数・ブロック数をログ出力し、全パターンが同じ入口でマージする。

### 2.1.2 Plan-Based Patterns (Pattern6-7, Phase 273 P3)

**Phase 273 P3 Status**: SSOT for Pattern6/7 lowering

Pattern6/7 は **Plan line（Extractor → Normalizer → Verifier → Lowerer）** を使用する。これは Pattern1-5 の JoinIR 経路とは独立した、新しい SSOT アーキテクチャである。

- **Plan Extractor Box** (pure)
  - ファイル:
    - `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs::extract_scan_with_init_plan()`
    - `src/mir/builder/control_flow/joinir/patterns/pattern7_split_scan.rs::extract_split_scan_plan()`
  - 責務:
    - Pattern 固有構造を抽出（builder アクセスなし）。
    - DomainPlan（pattern-specific）または None を返す。
    - Pure function（builder を触らない）。
  - 入力: AST（condition, body, post_loop_code）
  - 出力: `Result<Option<DomainPlan>, String>`
    - `Ok(Some(domain_plan))`: Pattern マッチ成功
    - `Ok(None)`: 不一致（次の pattern へフォールスルー）
    - `Err(...)`: close-but-unsupported（fail-fast）

- **PlanNormalizer Box** (SSOT for pattern knowledge)
  - ファイル: `src/mir/builder/control_flow/plan/normalizer.rs`
  - 責務:
    - DomainPlan（pattern-specific）→ CorePlan（fixed vocabulary）変換。
    - Pattern 固有知識の展開（SSOT）:
      - ScanWithInit (Pattern6): スキャン方向、init block、body effects、step
      - SplitScan (Pattern7): Split セマンティクス、accumulator updates
    - 中間 ValueId 生成（Const/BinOp/Compare/MethodCall）。
    - CoreLoopPlan 構築（block_effects/phis/frag/final_values）。
  - 入力: `DomainPlan`, `MirBuilder`, `LoopPatternContext`
  - 出力: `CorePlan`

- **PlanVerifier Box** (fail-fast validation)
  - ファイル: `src/mir/builder/control_flow/plan/verifier.rs`
  - 責務:
    - CorePlan 不変条件検証（曖昧さなし）。
    - Phase gating（P0/P1/P2/P3 specific checks）。
    - 不正な Plan の早期検出。
  - 検証項目（V2-V9）:
    - V2: Condition validity (valid ValueId)
    - V3: Exit validity (Return in function, Break/Continue in loop)
    - V4: Seq non-empty
    - V5: If completeness (then_plans non-empty)
    - V6: ValueId validity (all ValueIds pre-generated)
    - V7: PHI non-empty (loops require at least one carrier)
    - V8: Frag entry matches header_bb
    - V9: block_effects contains header_bb

- **PlanLowerer Box** (pattern-agnostic)
  - ファイル: `src/mir/builder/control_flow/plan/lowerer.rs`
  - 責務:
    - CorePlan のみを処理（DomainPlan 知識なし）。
    - Block 割り当て（preheader/header/body/step/after/found）。
    - PHI node 挿入（LoopHeaderPhiBuilder）。
    - Block emission（body_bb, step_bb）。
    - EdgeCFG Frag 構築（emit_frag() で terminator SSOT）。
    - variable_map 更新（final_values）。
  - 入力: `CorePlan`, `MirBuilder`, `LoopPatternContext`
  - 出力: `Result<Option<ValueId>, String>`

- **Routing in route_loop_pattern()**
  - 場所: `src/mir/builder/control_flow/joinir/patterns/router.rs::route_loop_pattern()` (lines 294-354)
  - Entry points（LOOP_PATTERNS table より前にチェック）:
    1. Try Plan-based Pattern6 (extract_scan_with_init_plan)
    2. Try Plan-based Pattern7 (extract_split_scan_plan)
    3. Fall through to LOOP_PATTERNS table (Pattern1-5, 8-9)

**Plan line vs JoinIR line**:

| 項目 | Plan line (Pattern6/7) | JoinIR line (Pattern1-5) |
|------|------------------------|-------------------------|
| 入口 | Extractor (pure) | Pattern Lowerer (builder access) |
| 中間表現 | DomainPlan → CorePlan | JoinModule (JoinIR) |
| Pattern 知識 | Normalizer SSOT | Pattern Lowerer 各自 |
| Terminator | emit_frag() SSOT | 各 Pattern 固有 emission |
| Block/Value 生成 | Normalizer (pre-allocated) | Pattern Lowerer |
| 収束性 | ✅ 完全収束（P3） | 🔄 段階的収束中 |

**SSOT 特性**:
- Normalizer が Pattern 知識を一元管理（scan/split セマンティクス）
- Lowerer は CorePlan のみを処理（pattern-agnostic）
- emit_frag() が terminator 生成の唯一の入口（SSOT）
- Pattern6/7 は JoinIR を経由せず直接 MIR へ

### 2.2 条件式ライン（式の箱）

- **BoolExprLowerer / condition_to_joinir**
  - ファイル:
    - `src/mir/join_ir/lowering/bool_expr_lowerer.rs`
    - `src/mir/join_ir/lowering/condition_to_joinir.rs`
  - 責務:
    - 通常の if/while 条件を MIR Compare/BinOp/UnaryOp へ lowering。
    - ループ lowerer 用の「AST 条件 → JoinIR Compute 命令列」を ConditionEnv とセットで構築。

- **ConditionEnv/ConditionBinding + ConditionEnvBuilder**
  - ファイル:
    - `src/mir/join_ir/lowering/condition_env.rs`
    - `src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`
  - 責務:
    - 変数名→JoinIR ValueId の環境を組み立て、host↔join の橋渡しを ConditionBinding に明示する。
    - Pattern 2 では break 条件の全変数をスキャンし、JoinInlineBoundary.condition_bindings に渡す。

- **LoopConditionScopeBox（Phase 170-D 実装済み）**
  - ファイル: `src/mir/loop_pattern_detection/loop_condition_scope.rs`
  - 責務:
    - 条件式の各変数を LoopParam / OuterLocal / LoopBodyLocal に分類。
    - 関数パラメータ誤分類バグは `condition_var_analyzer.rs` の修正で解消済み（OuterLocal として扱う）。

- **ConditionPatternBox（Phase 219-fix / Phase 222 拡張完了）**
  - ファイル: `src/mir/join_ir/lowering/condition_pattern.rs`
  - 責務:
    - if 条件の複雑度を判定（SimpleComparison vs Complex）。
    - **Phase 222**: 条件を正規化（var on left の canonical form）。
      - `normalize_comparison()` で左右反転をサポート:
        - `0 < i` → `i > 0` (literal < var → var > literal)
        - `i > j` (var CmpOp var) - そのまま受理
    - 複雑条件（`i % 2 == 1`、MethodCall 等）は false を返し、legacy P3 経路へルーティング。
  - 使用箇所:
    - PatternPipelineContext.is_if_sum_pattern() で条件複雑度をチェック。
    - P3 if-sum mode は単純比較のみ受理し、複雑条件は legacy P3 route へ routing（JoinIR Core 内）。

- **MethodCallLowerer（Phase 224-B / 224-C / 225 実装完了）**
  - ファイル: `src/mir/join_ir/lowering/method_call_lowerer.rs`
  - 責務:
    - AST MethodCall ノードを JoinIR BoxCall に lowering（メタデータ駆動）。
    - CoreMethodId の `is_pure()`, `allowed_in_condition()`, `allowed_in_init()` などのメタでホワイトリスト判定。
    - Phase 224-B: 引数なしメソッド（`length()` 等）対応。
    - Phase 224-C: 引数付きメソッド（`substring(i,j)`, `indexOf(ch)` 等）対応。
    - Phase 225: body-local init 用の MethodCall lowering も完全に CoreMethodId メタ駆動に統一（メソッド名ハードコード/Box 名ハードコードを削除）。
  - 設計原則:
    - **メソッド名ハードコード禁止**: CoreMethodId メタデータのみ参照。
    - **Fail-Fast**: ホワイトリストにないメソッドは即座にエラー。
    - **Box-First**: 単一責任（"このMethodCallをJoinIRにできるか？"）だけを担当。
  - 使用箇所:
    - `condition_lowerer.rs` の `lower_value_expression()` から呼び出し。
    - `loop_body_local_init.rs` の init lowering からも呼び出され、body‑local init での substring/indexOf などを lowering。
    - Pattern 2/3/4 のループ条件式や body‑local init で `s.length()`, `s.substring(...)`, `digits.indexOf(ch)` 等をサポート可能（メタ条件を満たす範囲で）。

- **LoopBodyCarrierPromoter（Phase 171-C-2 実装済み）**
  - ファイル: `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`
  - 責務:
    - LoopBodyLocal を Trim パターンとして bool キャリアへ昇格（substring + equality 連鎖を検出）。
    - 昇格成功 → CarrierInfo に統合し Pattern 2/4 へ橋渡し。昇格失敗は Fail‑Fast。
    - Pattern 2 は安全な Trim なら実際に前処理（substring 生成 + 空白比較の初期化）を emit してから JoinIR lowering。
    - Pattern 4 は Trim 昇格が起きた場合はガード付きでエラーにし、未実装を明示（Fail‑Fast）。
  - 汎用性:
    - Phase 173 で `_skip_whitespace`（JsonParser）が Trim パターンで動作確認済み。
    - Phase 174 で `_parse_string` 最小化版（終端クォート検出）でも動作確認済み。
    - → 空白文字以外の文字比較ループにも対応可能（TrimLoopHelper の汎用性実証）。

- **LoopBodyCondPromoter（Phase 223-3 + 223.5 + 224 実装完了）**
  - ファイル: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs`
  - 責務:
    - ループ条件（header/break/continue）に出てくる LoopBodyLocal を carrier に昇格する統一 API。
    - Pattern 2/Pattern 4 両対応の薄いコーディネーター箱（detection は専門 Promoter に委譲）。
    - **Phase 224: Two-tier strategy** - A-3 Trim → A-4 DigitPos の二段階 routing（昇格箱の順序を固定）。
  - Phase 223-3 実装内容:
    - `extract_continue_condition()`: body 内の if 文から continue 条件を抽出。
    - `try_promote_for_condition()`: LoopBodyCarrierPromoter を使った昇格処理。
    - Pattern4 への統合完了: LoopBodyLocal 条件の昇格成功時に lowering を続行（以前は Fail-Fast）。
  - Phase 223.5 実装内容:
    - Pattern2 への統合完了: header/break 条件を分析し昇格を試みる。

- **ScopeManager / ExprLowerer（Phase 231 パイロット実装完了）**
  - ファイル:
    - `src/mir/join_ir/lowering/scope_manager.rs`
    - `src/mir/join_ir/lowering/expr_lowerer.rs`
  - 責務:
    - **ScopeManager trait**: 変数参照を統一的に扱う trait（ConditionEnv / LoopBodyLocalEnv / CapturedEnv / CarrierInfo を統合）。
    - **Pattern2ScopeManager**: Pattern2 専用の薄いラッパー（promoted_loopbodylocals 対応含む）。
    - **ExprLowerer**: 式 lowering を1箇所に集約（Phase 231: Condition context のみ、General context は将来実装）。
  - Phase 231 実装内容:
    - Pattern2 break 条件の **pre-validation** として ExprLowerer を試行（soft fallback を前提にした検証ルート）。
    - 簡単な条件式（`i >= 5` など）を正常に検証し、未対応は UnsupportedNode エラーとして上位 router が legacy route を選ぶ。
    - 箱化・モジュール化の原則に準拠（ScopeManager は trait、ExprLowerer は再利用可能）。
  - 設計原則:
    - **Box-First**: ScopeManager は trait-based "box" で変数解決を抽象化。
    - **Fail-Safe**: 未対応 AST ノードは明示的エラーとして扱い、JoinIR Core 内の routing で安全に退避する。
    - **Incremental Adoption**: Phase 231 は検証専用、Phase 232+ で実際の lowering 置き換え予定。
  - 使用箇所:
    - `pattern2_with_break.rs` の break 条件 lowering 前に pre-validation として実行。
    - 将来は Pattern1/Pattern3/Pattern4 にも拡大予定（Phase 232）。

- **DigitPosConditionNormalizer（Phase 224-E 実装完了）**
  - ファイル: `src/mir/join_ir/lowering/digitpos_condition_normalizer.rs`
  - 責務:
    - digit_pos 条件を正規化（`digit_pos < 0` → `!is_digit_pos`）。
    - Pattern2 の break 条件 lowering 前に呼び出され、promoted variable の条件を bool 形式に変換。
  - Phase 224 実装内容（Core Implementation Complete ⚠️）:
    - **Two-tier promotion**: Step1 で A-3 Trim 試行 → 失敗なら Step2 で A-4 DigitPos 試行 → 両方失敗で Fail-Fast。
    - **DigitPosPromoter 統合**: cascading indexOf パターン（substring → indexOf → comparison）の昇格をサポート。
    - **Unit test 完全成功**: 6/6 PASS（promoter 自体は完璧動作）。
    - **残りの制約**: body-local init の MethodCall（`substring` 等）の lowering は Phase 193/224-B/C のスコープ外で、今後の Phase で対応。
  - 設計原則:
    - **Thin coordinator**: 専門 Promoter（LoopBodyCarrierPromoter / DigitPosPromoter）に昇格ロジックを委譲。
    - **Pattern-agnostic**: Pattern2 (break) / Pattern4 (continue) の統一入口として機能。
    - **Fail-Fast with clear routing**: A-3 → A-4 順で試行し、両方失敗なら明示的エラー。
  - 入出力:
    - 入力: `ConditionPromotionRequest`（loop_param_name, cond_scope, break_cond/continue_cond, loop_body）
    - 出力: `ConditionPromotionResult::Promoted { carrier_info, promoted_var, carrier_name }` または `CannotPromote { reason, vars }`
  - 使用元（Phase 223.5 実装完了、Phase 224 拡張済み）:
    - Pattern4: promotion-first（昇格試行 → 成功なら CarrierInfo merge → 失敗なら Fail-Fast）
    - Pattern2: promotion-first（同上、break条件を分析対象とする）

- **DigitPosPromoter（Phase 224 実装完了 ⚠️ Core Complete）**
  - ファイル: `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs`（467 lines）
  - 責務:
    - **A-4 pattern**: Cascading LoopBodyLocal with indexOf（substring → indexOf → comparison）の昇格。
    - **Pattern detection**: `local ch = s.substring(...); local digit_pos = digits.indexOf(ch); if digit_pos < 0 { break }`
    - **Comparison operators**: `<`, `>`, `<=`, `>=`, `!=` をサポート（equality `==` は A-3 Trim 領域）。
    - **Dependency validation**: indexOf() が別の LoopBodyLocal に依存していることを検証（cascading pattern）。
  - Phase 224 実装内容:
    - `try_promote()`: A-4 パターン検出 & bool carrier 昇格（`digit_pos` → `is_digit_pos`）。
    - **Unit tests**: 6/6 PASS（basic pattern, non-indexOf rejection, no dependency rejection, comparison operators, equality rejection）。
    - **Integration**: LoopBodyCondPromoter から A-3 Trim 失敗後の二段階 routing で呼ばれる。
  - 設計原則:
    - **One Box, One Question**: A-4 DigitPos パターン専用（A-3 Trim は LoopBodyCarrierPromoter に残す）。
    - **Separation of Concerns**: Trim（equality-based）と DigitPos（comparison-based）を分離。
    - **Bool carrier consistency**: A-3 Trim と同じく bool carrier に昇格（`is_digit_pos`）。
  - 入出力:
    - 入力: `DigitPosPromotionRequest`（cond_scope, break_cond/continue_cond, loop_body）
    - 出力: `DigitPosPromotionResult::Promoted { carrier_info, promoted_var, carrier_name }` または `CannotPromote { reason, vars }`
  - **Phase 224-E 完了（AST 条件正規化）**:
    - **DigitPosConditionNormalizer Box**: `digit_pos < 0` → `!is_digit_pos` の AST 変換。
    - **実装箇所**: `src/mir/join_ir/lowering/digitpos_condition_normalizer.rs`（173 lines）。
    - **統合**: Pattern2 で promotion 成功後に自動適用（`pattern2_with_break.rs` line 332-344）。
    - **単体テスト**: 5/5 PASS（happy path, wrong operator/variable/constant, non-binary-op）。
    - **E2E テスト**: `phase2235_p2_digit_pos_min.hako` で型エラー解消確認。
    - **回帰テスト**: digitpos (11 tests), trim (32 tests) 全て PASS。
    - **digit_pos 正規化ライン**: DigitPosPromoter + CarrierInfo.promoted_loopbodylocals + DigitPosConditionNormalizer で `digit_pos < 0` を bool キャリア `is_digit_pos` ベースの条件（`!is_digit_pos`）に直してから ConditionEnv / ConditionPatternBox / ExprLowerer 系の条件 lowering に渡す。
  - 参考:
    - 設計ドキュメント: `docs/development/current/main/phase224-digitpos-condition-normalizer.md`
    - 実装サマリ: `docs/development/current/main/PHASE_224_SUMMARY.md`
    - テストケース: `apps/tests/phase2235_p2_digit_pos_min.hako`

- **ContinueBranchNormalizer / LoopUpdateAnalyzer**
  - ファイル:
    - `src/mir/join_ir/lowering/continue_branch_normalizer.rs`
    - `src/mir/join_ir/lowering/loop_update_analyzer.rs`
  - 責務:
    - else-continue を then-continue へ正規化し、Select ベースの continue を簡潔にする。
    - ループ本体で実際に更新されるキャリアだけを抽出（Pattern 4 で不要キャリアを排除）。

- **LoopBodyLocalEnv / UpdateEnv / CarrierUpdateEmitter（Phase 184, 191統合完了）**
  - ファイル:
    - `src/mir/join_ir/lowering/loop_body_local_env.rs`
    - `src/mir/join_ir/lowering/update_env.rs`
    - `src/mir/join_ir/lowering/carrier_update_emitter/mod.rs`
    - `src/mir/join_ir/lowering/loop_with_break_minimal.rs`（Phase 191統合）
  - 責務:
    - **LoopBodyLocalEnv**: ループ本体で宣言された body-local 変数の名前→ValueId マッピングを管理（箱化設計）。
    - **UpdateEnv**: 条件変数（ConditionEnv）と body-local 変数（LoopBodyLocalEnv）を統合した変数解決層。
      - Priority order: 1. Condition variables（高優先度） → 2. Body-local variables（フォールバック）
    - **CarrierUpdateEmitter**: UpdateExpr を JoinIR 命令に変換する際、UpdateEnv を使用して body-local 変数をサポート。
      - `emit_carrier_update_with_env()`: UpdateEnv 版（Phase 184 新規）
      - `emit_carrier_update()`: ConditionEnv 版（後方互換）
    - **LoopBodyLocalInitLowerer**: Phase 191 で Pattern2 に統合完了。
      - 対応済み init 式: 整数リテラル、変数参照、二項演算（`local digit = i + 1`）
      - UpdateEnv の優先順位により ConditionEnv → LoopBodyLocalEnv の順で変数解決
  - 設計原則:
    - **箱理論**: 各 Box が単一責任を持ち、境界明確。
    - **決定性**: BTreeMap 使用で一貫した順序保証（PHI 生成の決定性）。
  - **Phase 192完了**: Complex addend（`v = v*10 + f(x)`）は ComplexAddendNormalizer で temp に分解してから NumberAccumulation に載せる。
    - `complex_addend_normalizer.rs`: Pure AST transformer（前処理箱）
    - Pattern2 統合完了、emission ライン再利用（変更なし）
    - 制約: MethodCall を含む init 式は Phase 193 で対応予定

### 2.3 キャリア / Exit / Boundary ライン

- **Phase 200-B: FunctionScopeCaptureAnalyzer (完了)**
  - ファイル: `src/mir/loop_pattern_detection/function_scope_capture.rs`
  - 責務: 関数スコープの「実質定数」を検出
  - 判定条件:
    1. 関数トップレベルで 1 回だけ定義
    2. ループ内で再代入なし
    3. 安全な初期式（文字列/整数リテラル）のみ
  - 結果: CapturedEnv に name, host_id, is_immutable を格納
  - **ConditionEnvBuilder v2**:
    - 責務: CapturedEnv から ParamRole::Condition として ConditionEnv に追加
    - 経路: analyze_captured_vars → build_with_captures → ConditionEnv.captured
    - 不変条件: Condition role は Header PHI / ExitLine の対象にならない
  - **Pattern 2 統合**: Phase 200-C で完了 ✅
    - MirBuilder.fn_body_ast フィールド追加
    - LoopPatternContext.fn_body 経由で Pattern 2 lowerer に渡す
    - analyze_captured_vars_v2() で構造的ループ検索（ポインタ比較 → AST 構造比較）

- **Phase 200-C: digits.indexOf E2E 連携 (完了)**
  - 目的: 200-A/B インフラを実際に Pattern 2 経路に統合
  - 実装:
    - fn_body を MirBuilder → LoopPatternContext → Pattern 2 に渡す
    - analyze_captured_vars_v2() で構造的マッチング（AST Debug 文字列比較）
    - digits / s 等の関数ローカル定数が CapturedEnv に正しく捕捉される
  - 検証結果:
    - capture 検出: ✅ PASS
    - E2E 実行: ❌ BLOCKED（テストケースが Pattern 5+ 必要）
  - テストケース制約:
    - phase200_digits_atoi_min.hako: body-local `pos` を条件 `if pos < 0` で使用
    - → Pattern 5 (body-local promotion) が必要

- **Phase 200-D: digits capture "実戦 1 本" 検証 (完了)**
  - 目的: capture 経路の E2E 検証（body-local なしのシンプルケース）
  - 検証結果:
    - capture 検出: ✅ PASS（base, limit, n 等が正しく CapturedEnv に）
    - ConditionEnv 統合: ✅ PASS（captured vars が ConditionEnv.captured に追加）
    - 実行: ⚠️ 別の制約でブロック（substring 未対応、キャリア更新型問題）
  - 成果:
    - capture 経路（analyze_captured_vars_v2 → ConditionEnv → Pattern 2）が正常動作
    - 関数スコープ定数が正しく検出・統合される
  - テストファイル: phase200d_capture_minimal.hako, phase200d_capture_in_condition.hako

- **CarrierInfo / LoopUpdateAnalyzer / CarrierUpdateEmitter**
  - ファイル:
    - `src/mir/join_ir/lowering/carrier_info.rs`
    - `src/mir/join_ir/lowering/loop_update_analyzer.rs`
    - `src/mir/join_ir/lowering/carrier_update_emitter/mod.rs`
  - 責務:
    - ループで更新される変数（carrier）を検出し、UpdateExpr を保持。
    - Pattern 4 では実際に更新されるキャリアだけを残す。
    - **Phase 188 完了** ✅: String 更新（StringAppendChar/StringAppendLiteral）を UpdateRhs ベースのホワイトリストで受理し、JoinIR BinOp を emit。
      - 許可: `UpdateRhs::Const`, `UpdateRhs::Variable`, `UpdateRhs::StringLiteral`
      - 拒否: `UpdateRhs::Other`（method call / nested BinOp 等の複雑パターンのみ）
      - Pattern2/4 の can_lower() で選別、carrier_update_emitter で JoinIR 生成
  - 現状制約（重要 / PoC の前提）:
    - Pattern2 の「出口で値が持ち帰られる変数」は、基本的に **carrier として認識されたもの**のみが `exit_bindings` 経由で再接続される。
    - 現状の carrier 認識は “mutable-acc” 形（例: `result = result + rhs`）を中心に成立しており、`result = 42` のような **direct assignment** は carrier として扱われないケースがある。
    - その場合、break 後に `return result` しても「ループ前の値」が返りうる（再接続されないため）。
    - Phase 286 P2 の PoC fixture は、この制約を踏んで **mutable-acc 形で固定**する（direct assignment 対応は別フェーズで箱化して拡張する）。

- **ExitMeta / JoinFragmentMeta**
  - ファイル: `carrier_info.rs`
  - 責務:
    - JoinIR lowerer が出口の JoinIR ValueId を記録（expr_result とキャリアを明確に分離）。

- **LoopHeader PHI Builder**
  - ファイル:
    - `src/mir/builder/control_flow/joinir/merge/loop_header_phi_info.rs`
    - `loop_header_phi_builder.rs`
  - 責務:
    - ループ変数とキャリアの PHI をヘッダブロックに生成し、entry/latch の 2 入力で SSA を確立。
    - instruction_rewriter が latch 側を埋めた後に finalize して挿入する。

- **JoinInlineBoundary**
  - ファイル: `src/mir/join_ir/lowering/inline_boundary.rs`
  - 主フィールド:
    - `join_inputs / host_inputs`：ループパラメータの橋渡し
    - `condition_bindings`：条件専用変数の橋渡し（JoinIR ValueId を明示）
    - `exit_bindings`：キャリア出口の橋渡し（carrier 名を明示）
    - `expr_result` / `loop_var_name`：expr result / ヘッダ PHI 生成用のメタ情報
  - 責務:
    - 「host ↔ JoinIR」の境界情報の SSOT。各パターン lowerer がここに全て詰めてから merge する。

- **BoundaryInjector**
  - ファイル: `src/mir/builder/joinir_inline_boundary_injector.rs`
  - 責務:
    - `join_inputs` と `condition_bindings` を entry block に Copy で注入し、JoinIR ローカル ID と host ID を接続。

- **ExitLine (ExitMetaCollector / ExitLineReconnector / ExitLineOrchestrator)**
  - ファイル:
    - `src/mir/builder/control_flow/joinir/merge/exit_line/mod.rs`
    - `exit_line/meta_collector.rs`
    - `exit_line/reconnector.rs`
  - 責務:
    - ExitMeta から exit_bindings を構築（Collector）。
    - 変数再接続はヘッダ PHI の dst を使って `builder.variable_map` を更新（Reconnector）。
    - expr 用の PHI には一切触れない（carrier 専用ライン）。
  - **ConditionOnly キャリア**: header PHI の entry は CarrierInit（BoolConst(false) 等）を起点にし、ExitLine では variable_map や ExprResult への書き戻しを行わずヘッダ PHI 経由に限定。

- **ExprResultResolver（Phase 221-R 実装済み）**
  - ファイル: `src/mir/builder/control_flow/joinir/merge/expr_result_resolver.rs`
  - 責務:
    - expr_result ValueId を exit_bindings/carrier_phis と照合し、適切な出口値を解決。
    - expr_result が carrier の場合: carrier PHI dst を返す（変数再接続と統一）。
    - expr_result が非 carrier の場合: remapped ValueId を返す（expr-only の値）。
  - 特徴:
    - Phase 33 モジュール化パターン準拠（ExitMetaCollector/ExitLineReconnector と同様）。
    - 4つのシナリオを unit test でカバー（carrier/non-carrier/None/error）。
    - merge/mod.rs から 64 行を抽出、-37 行のネット削減。

- **JoinInlineBoundaryBuilder（Phase 200-2 / Phase 201 完了）**
  - ファイル: `src/mir/join_ir/lowering/inline_boundary_builder.rs`
  - 責務:
    - JoinInlineBoundary の構築を Builder パターンで統一化。
    - フィールド直書きの散乱を防ぎ、inputs/outputs/condition_bindings/exit_bindings/loop_var_name/expr_result の設定を fluent API で実施。
    - **Phase 201 で Pattern1/2/3/4 全てに適用完了**（境界情報組み立てを 1 箇所に集約）。

- **JoinIRVerifier（Phase 200-3 追加）**
  - ファイル: `src/mir/builder/control_flow/joinir/merge/mod.rs`（debug_assertions 専用関数）
  - 責務:
    - LoopHeader PHI / ExitLine 契約をデバッグビルドで検証する門番。
    - `verify_loop_header_phis()`: loop_var_name がある場合にヘッダ PHI が存在するか確認。
    - `verify_exit_line()`: exit_bindings が exit block に対応しているか確認。
    - `verify_joinir_contracts()`: merge_joinir_mir_blocks() の最後で全契約を一括チェック。
    - release ビルドでは完全に除去される（`#[cfg(debug_assertions)]`）。

- **FunctionScopeCaptureAnalyzer / CapturedEnv（Phase 200-A 追加）**
  - ファイル: `src/mir/loop_pattern_detection/function_scope_capture.rs`
  - 責務:
    - 関数スコープで宣言され、ループ内で不変な変数（"実質定数"）を検出。
    - 例: `local digits = "0123456789"` in `JsonParser._atoi()`
    - CapturedVar: `{ name, host_id, is_immutable }`
    - CapturedEnv: 検出された変数のコレクション
  - **Phase 200-A**: 型と空実装のみ（skeleton）。
  - **Phase 200-B**: 実際の検出ロジックを実装予定（AST スキャン + 再代入チェック）。

- **ParamRole enum（Phase 200-A 追加）**
  - ファイル: `src/mir/join_ir/lowering/inline_boundary_builder.rs`
  - 責務:
    - JoinInlineBoundary のパラメータ役割を明示的に区別。
    - LoopParam / Condition / Carrier / ExprResult の 4 種類。
  - **不変条件**:
    - **Condition 役**: PHI dst にしてはいけない（ループ内で更新されない）。
    - **Condition 役**: ExitLine の対象にも入れない（ループ外で使われない）。
    - 理由: 条件専用変数（例: `digits`）はループ内でのみ参照され、不変。
  - **Phase 200-A**: enum 定義のみ。
  - **Phase 200-B**: ルーティングロジック実装予定（CapturedEnv 統合時）。

- **ConditionEnvBuilder::build_with_captures（Phase 200-A 追加）**
  - ファイル: `src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`
  - 責務:
    - 将来 CapturedEnv を受け取り、ConditionEnv に統合する v2 入口。
  - **Phase 200-A**: 既存実装に委譲する skeleton。
  - **Phase 200-B**: CapturedEnv の変数を condition_bindings に追加する実装予定。

- **Phase 86 SSOT Modules (2025-12-13) ✅**

  - **carrier_init_builder.rs**: CarrierInit → ValueId 生成の SSOT
    - Location: `src/mir/builder/control_flow/joinir/merge/carrier_init_builder.rs`
    - Function: `init_value(builder, init, host_id, name, debug) -> ValueId`
    - Purpose: FromHost/BoolConst/LoopLocalZero の統一変換（34+ 行の重複 match 削減）
    - Tests: 8 unit tests

  - **error_tags.rs**: JoinIR エラーメッセージ整形の SSOT
    - Location: `src/mir/join_ir/lowering/error_tags.rs`
    - Functions: `freeze()`, `exit_line_contract()`, `ownership_relay_unsupported()`, `pattern_detection_failed()`, `lowering_error()`
    - Purpose: 一貫したエラータグフォーマット（typo 防止、重複文字列削減）
    - Tests: 5 unit tests

### 2.4 expr result ライン（式としての戻り値）

- **exit_phi_builder**
  - ファイル: `src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs`
  - 責務:
    - JoinIR fragment が `expr_result` を持つときに exit ブロックへ PHI を生成。
    - carrier_inputs も受け取り exit ブロックに PHI を作るが、再接続の SSOT は LoopHeader PHI（ExitLine はヘッダ PHI を使用）。

- **InstructionRewriter**
  - ファイル: `instruction_rewriter.rs`
  - 責務:
    - continuation 関数（k_exit）をスキップし、Return → exit ブロック Jump に変換。
    - `JoinFragmentMeta.expr_result` と exit_bindings をヘッダ PHI 経由で収集し、`exit_phi_inputs` / `carrier_inputs` を復活させた（SSA‑undef 修正済み）。
    - tail call を Branch/Jump に書き換えつつ、LoopHeaderPhiInfo に latch 入力を記録する。
  - **Select 展開の不変条件（Phase 196）**:
    - PHI の入力 ValueId は必ず `remapper.remap_instruction()` で remap 済みの MIR ValueId を使用。
    - InstructionRewriter では ValueId の二重 remap を行わない（block ID のみ remap）。
    - 詳細: [phase196-select-bug-analysis.md](./phase196-select-bug-analysis.md)

---

## 3. JoinIR → MIR 統合の全体フロー

1. Pattern router が AST/LoopFeatures から Pattern1–4 を選択し、各 lowerer が
   `(JoinModule, JoinFragmentMeta)` を生成。
2. `JoinInlineBoundary` が:
   - ループ入力（join_inputs/host_inputs）
   - 条件変数（condition_bindings）
   - キャリア出口（exit_bindings）
   を保持。
3. `merge_joinir_mir_blocks` が（マルチ関数対応）:
   - 全関数の BlockID を再割り当て（block_allocator）し、ValueId はパラメータを除外して収集。
   - Boundary の condition/exit Bindings の JoinIR ValueId も remap 対象に追加。
   - LoopHeader PHI を生成（loop_header_phi_builder）し、latch 側は instruction_rewriter が埋める。
   - instruction_rewriter で関数をマージしつつ Call→Jump に変換、k_exit 関数はスキップ。
   - BoundaryInjector で entry block に Copy を注入（join_inputs + condition_bindings）。
   - Header PHI を finalize → exit_phi_builder で expr_result/carrier の exit PHI を構築。
   - ExitLineOrchestrator がヘッダ PHI dst を使って variable_map を更新。
   - host の現在ブロックから JoinIR entry へ jump を張り、exit ブロックに切り替える。

この全体フローの詳細は `src/mir/builder/control_flow/joinir/merge/mod.rs` と
`phase-189-multi-function-mir-merge/README.md` を参照。

---

## 4. JsonParser / Trim / P5 ループの現在地（Phase 170–181）

### 4.1 JsonParser ループ空間と P1–P5

Phase 181 で JsonParserBox 内の 11 ループを棚卸しした結果、
構造的には **すべて JoinIR Pattern1–4 (+ P5) で扱える** ことが分かったよ。

- 既に JoinIR 経路で動作しているもの:
  - `_skip_whitespace`（P2 + P5 Trim）
  - `_trim` leading/trailing（P2 + P5 Trim）
- **Phase 182 で P1/P2 パターン検証完了** ✅:
  - Pattern1 (Simple): `_match_literal` 系ループで動作確認済み（apps/tests/phase182_p1_match_literal.hako）
  - Pattern2 (Break): 整数演算ループで動作確認済み（apps/tests/phase182_p2_break_integer.hako）
  - **ブロッカー発見**: 実際の JsonParser ループには 2 つの制約が必要:
    1. LoopBodyLocal 変数の扱い（`ch`, `digit_pos`, `pos` など）
       - 現状は Trim pattern 専用の carrier 昇格を試みてエラーになる
       - P1/P2 では純粋なローカル変数（昇格不要）として扱うべき
    2. 文字列連結フィルタ（Phase 178）
       - `num_str = num_str + ch` のような string concat を保守的に reject
       - JsonParser では必須の操作なので段階的に有効化が必要
       - **設計原則**:
         - string は「特別扱いのパターン」ではなく、あくまで MirType の 1 種類として扱う。
         - Pattern2/4 側で型名や変数名（`"result"`, `"num_str"` など）に依存した分岐は入れない。
         - LoopUpdateAnalyzer の `UpdateKind` / `UpdateRhs` で「安全な更新パターン」を列挙し、
           そのうち string にも適用可能なものだけを **ホワイトリストで許可**する。
         - 実際の lowering は CarrierUpdateLowerer / 式 Lowerer 側で行い、JoinIR のループ形（P1–P4）は増やさない。
    3. 数値の桁展開（Phase 190 設計完了）
       - `v = v * 10 + digit` のような NumberAccumulation パターンを UpdateKind で whitelist 制御。
       - 型制約: Integer のみ許可（String は StringAppendChar 使用）。
       - 検出: AST 構造解析（名前依存禁止）、Complex パターンは Fail-Fast で reject。
- **Phase 183 で LoopBodyLocal 役割分離完了** ✅:
  - **設計**: LoopBodyLocal を 2 カテゴリに分類:
    - **Condition LoopBodyLocal**: ループ条件（header/break/continue）で使用 → Trim 昇格対象
    - **Body-only LoopBodyLocal**: ループ本体のみで使用 → 昇格不要、pure local 扱い
  - **実装**: TrimLoopLowerer に `is_var_used_in_condition()` ヘルパー追加
    - 条件で使われていない LoopBodyLocal は Trim 昇格スキップ
    - 5 つの unit test で変数検出ロジックを検証
  - **テスト**: `apps/tests/phase183_body_only_loopbodylocal.hako` で動作確認
    - `[TrimLoopLowerer] No LoopBodyLocal detected` トレース出力で body-only 判定成功
  - **次の課題（→Phase 184 で対応）**: body-local 変数の MIR lowering 対応（`local temp` in loop body）
    - Phase 183 では "Trim promotion しない" 判定まで完了
    - 実際の MIR 生成インフラは Phase 184 で実装済み（Pattern2/4 への統合は次フェーズ）

### 4.2 Body-local 変数の MIR lowering 基盤（Phase 184）

Phase 184 では、「条件には出てこない LoopBodyLocal 変数」を安全に JoinIR→MIR に落とすためのインフラ箱だけを追加したよ。

- **LoopBodyLocalEnv**
  - 責務: ループ本体内で `local` 定義された変数の「JoinIR 側 ValueId のみ」を管理する。
  - 入力: ループ本体 AST / JoinIR ビルダー。
  - 出力: `name -> join_value_id` のマップ。
  - 特徴: host 側との対応は持たない。ConditionEnv とは完全に分離された「本体専用ローカル環境」。

- **UpdateEnv**
  - 責務: UpdateExpr lowering 時の変数解決順序をカプセル化する。
  - 仕様: `ConditionEnv`（条件・キャリア）と `LoopBodyLocalEnv`（本体ローカル）を中で束ねて、
    `resolve(name)` で「条件→ローカル」の順に ValueId を返す。
  - 利用箇所: CarrierUpdateEmitter / CarrierUpdateLowerer が、変数名ベースで UpdateExpr を JoinIR に落とす時に利用。

- **CarrierUpdateEmitter 拡張**
  - 責務: `LoopUpdateSummary`（UpdateKind）に応じて、int 系キャリア更新を JoinIR 命令に変換する。
  - 変更点: 直接 `variable_map` を読むのではなく、`UpdateEnv` 経由で名前解決するように変更。
  - 効果: 本体専用の LoopBodyLocal 変数（`temp` 等）を、Pattern2/4 から安全に扱える土台が整った。

このフェーズではあくまで「ストレージ・名前解決・emit の箱」までで止めてあり、
Pattern2/4 への統合（実際に Body-local 更新を使うループを JoinIR 経路に載せる）は次フェーズ（Phase 185 以降）の仕事として分離している。
- 構造的に P1–P4 で対応可能（代表例）:
  - `_parse_number` / `_atoi`（P2 Break）- Phase 182 でブロッカー特定済み
  - `_match_literal`（P1 Simple while）- Phase 182 で動作確認済み ✅
  - `_parse_string` / `_parse_array` / `_parse_object`
    （P2 + P4 + P5 の組み合わせで表現可能なことを設計上確認済み）
- 低優先度だが理論上は P1–P4 からの拡張で吸収可能:
  - `_unescape_string` など、複雑な continue / 条件付き更新を含むループ
    - Phase 88（dev-only）で、`i+=2 + continue`（かつ continue 分岐側で `acc` 更新）を最小フィクスチャとして抽出し、
      frontend の continue pattern を「`i = i + const` の差分加算」に限定して段階拡張した。

方針:

- **ループの「形」は P1–P4 から増やさない**。
  複雑さ（LoopBodyLocal 条件、OR chain、continue 多用など）は BoolExprLowerer /
  ContinueBranchNormalizer / TrimLoopLowerer (P5) といった補助箱側で吸収する。
- JsonParser 側の P5 適用（Trim / `_skip_whitespace` / `_parse_string` 最小版）は実証済み。
  残りのループは Phase 17x–18x で、P1–P4+P5 の組み合わせとして段階的に実装していく。

### 4.3 JsonParser 実戦カバレッジ（Phase 221 時点）

Phase 210–221 で「数値ループ＋if-sum」を実戦投入し、JoinIR インフラが **本番級に動作する** ことを確認したよ：

- **実戦確認済みループ**（9/13 loops ≒ 69%）:
  - ✅ `_skip_whitespace` (P2 + P5 Trim, Phase 173)
  - ✅ `_trim` leading/trailing (P2 + P5 Trim, Phase 171/172)
  - ✅ `_match_literal` 最小版 (P1 Simple, Phase 210)
  - ✅ `_atoi` 最小版 (P2 Break, NumberAccumulation, Phase 190)
  - ✅ `_parse_number` 最小版 (P2 Break, NumberAccumulation, Phase 190)
  - ✅ if-sum 最小版 (P3 IfPHI, variable condition, Phase 212/220)
  - ✅ captured vars 最小版 (P2 Break, function-local const, Phase 200-D)
  - ✅ digits accumulate 最小版 (P2 Simple accumulation, Phase 200-D)

- **Phase 210–221 の成果**:
  - 8 本すべて JoinIR → MIR → Runtime 完全成功（RC 正常）
  - Pattern1/2/3 自動ルーティング正常動作
  - NumberAccumulation (Mul+Add), if-sum (if条件付き更新), captured vars すべて正常
  - ConditionEnv/ConditionPatternBox/ExprResultResolver 統合完了
  - **制約発見ゼロ（基本パス）** - Phase 190/200/220 の統合が完璧に機能

- **Phase 221 制約発見** (2025-12-09):
  - ⚠️ **LoopBodyLocal in condition**: break/continue 条件で loop-body-local 変数を使用（Pattern 5+ 必要）
  - ⚠️ **MethodCall whitelist**: body-local init で `substring` 未対応（Phase 193: indexOf/get/toString のみ）
  - ~~⚠️ **if condition pattern**: if-sum mode は `var CmpOp literal` のみ（`i > 0` は OK、`0 < i` や `i > j` は NG）~~ → **Phase 222 で解決済み✅**

- **Phase 222 解決済み制約** (2025-12-10):
  - ✅ **if condition pattern**: ConditionPatternBox 正規化で左リテラル・変数同士の比較をサポート
    - `0 < i`, `len > i` → `i > 0`, `i < len` に正規化
    - `i > j` (var CmpOp var) - 直接サポート
    - テスト: phase222_if_cond_left_literal_min.hako → RC=2 達成

- **残りループ** (Phase 222+ で段階的対応予定):
  - `_parse_array`, `_parse_object` (MethodCall 複数)
  - `_unescape_string` (複雑なキャリア処理)
  - `_atoi`/`_parse_number` 本体（LoopBodyLocal in condition 対応後）

**結論**:
- JoinIR 数値ループ基盤（NumberAccumulation + captured const + if-sum）は **実戦投入可能な成熟度** に到達 ✨
- **Phase 221 で 3 種の既知制約を整理** - 次フェーズで Pattern 5+ 拡張 / MethodCall whitelist 拡張が候補

---

## 6. Roadmap（JoinIR の今後のゴール）

ここから先の JoinIR の「目指す形」を、箱レベルでざっくり書いておくよ。フェーズ詳細は各 phase ドキュメントに分散させて、このセクションは常に最新の方向性だけを保つ。

### 6.1 直近（Phase 176-177 まわり）

- **P5（Trim/JsonParser 系）ループの複数キャリア対応** ✅ Phase 176 完了 (2025-12-08)
  - 完了内容:
    - Pattern2 lowerer を全キャリア対応に拡張（ヘッダ PHI / ループ更新 / ExitLine）。
    - CarrierUpdateLowerer ヘルパで UpdateExpr → JoinIR 変換を統一。
    - 2キャリア（pos + result）E2E テスト完全成功。
  - 技術的成果:
    - CarrierInfo / ExitMeta / ExitLine / LoopHeaderPhiBuilder の multi-carrier 対応を Pattern2 lowerer で完全活用。
    - Trim pattern の「キャリア = ループ変数」という誤解を解消（loop_var は特殊キャリア）。
  - 次のステップ (Phase 177):
    - JsonParser `_parse_string` 本体を P2+P5 で通す（pos + result の 2 キャリアで実ループ動作確認）。

### 6.2 中期（selfhost depth‑2 / JsonParser 本体）

- **JsonParserBox / Trim 系ループの本線化**
  - 目標:
    - `_trim` / `_skip_whitespace` / `_parse_string` / `_parse_array` などの主要ループが、すべて JoinIR Pattern1–4 + P5 で通ること。
    - LoopConditionScopeBox + LoopBodyCarrierPromoter + TrimLoopHelper の上で安全に正規化できるループを広げていく。
  - 方針:
    - 「ループの形」は P1–P4 から増やさず、複雑さは BoolExprLowerer / ContinueBranchNormalizer / P5 系の補助箱で吸収する。
    - LoopPatternSpace の P6/P7/P12 候補（break+continue 同時 / 複数キャリア条件更新 / early return）は、実アプリで必要になった順に小さく足す。

- **selfhost depth‑2（.hako JoinIR/MIR Frontend）**
  - 目標:
    - `.hako → JsonParserBox → Program/MIR JSON → MirAnalyzerBox/JoinIrAnalyzerBox → VM/LLVM` の深度 2 ループを、日常的に回せるようにする。
    - Rust 側の JoinIR は「JSON を受け取って実行・検証するランナー層」、.hako 側が「JoinIR/MIR を構築・解析する言語側 SSOT」という役割分担に近づける。

- **Phase 230（ExprLowerer / ScopeManager 設計フェーズ）**
  - 目標: 条件式 / init 式 / carrier 更新式の lowering を将来ひとつの ExprLowerer + ScopeManager に統合できるよう、既存の散在する lowering/API/Env を設計レベルで整理する（このフェーズではコード変更なし）。

- **Phase 233（loop_update_summary テスト刷新フェーズ）**
  - 目標: deprecated `analyze_loop_updates()` 依存のテストを廃止し、`analyze_loop_updates_from_ast()` に揃えたユニットで if-sum 判定ラインの期待値を固定する。

- **Phase 232（Failing Tests の棚卸しフェーズ）**
  - 目標: `cargo test --release` で残っている 7 件の FAIL を「どの箱 / どのパターン / どのレイヤ」の問題かで整理し、P0/P1/P2 に分類して次フェーズのターゲットを決める（core バグを見つけるフェーズではなく、未対応領域の可視化フェーズ）。

- **Phase 234（ArrayFilter / Pattern3 設計フェーズ）**
  - 目標: ArrayExtBox.filter 系 3 テストを P3 if-PHI の正式対象に含めるか、当面は PoC 領域として Fail-Fast を仕様として維持するかを docs ベースで決める（コード変更なし）。

- **Phase 235（ExprLowerer / ScopeManager パイロット実装 & テスト）**
  - 目標: ExprLowerer/ScopeManager の Condition 文脈を小さな範囲でユニットテストしつつ、Pattern2 の break 条件で validation-only の経路として動かし、既存の legacy lowering と挙動が一致することを確認する（本番 lowering の置き換えは後続フェーズ）。

- **Phase 236-EX（ExprLowerer / ScopeManager 本番導入 - Pattern2 break 条件）**
  - 目標: Pattern2 の break 条件 lowering を ExprLowerer/ScopeManager 経由の本番経路に切り替え、既存の condition_to_joinir ベース実装と RC / JoinIR 構造 / ログ挙動が一致することを確認する（影響範囲は Pattern2 break 条件のみに限定）。

- **Phase 237-EX（ExprLowerer 条件パターン棚卸しフェーズ）**
  - 目標: JsonParser / selfhost のループ条件・break/continue 条件をカタログ化し、ExprLowerer/ScopeManager で優先的に扱うパターンと後回しにするパターンを SSOT 化する（コード変更なし）。

- **Phase 238-EX（ExprLowerer / ScopeManager Scope Boundaries）**
  - 目標: ExprLowerer / ScopeManager / ConditionEnv / LoopBodyLocalEnv / UpdateEnv の責務と参照範囲を文書化し、「誰がどこまで見てよいか」を SSOT として固定する（コード変更なし、ガイドライン整備）。***

---

## selfhost / .hako JoinIR Frontend との関係

JoinIR は Rust 側だけでなく、将来的に .hako selfhost コンパイラ側でも生成・解析される予定だよ：

- .hako 側の JsonParser/分析箱は、Program JSON / MIR JSON v1 を読んで JoinIR/MIR を解析する。
- Rust 側 JoinIR ラインの設計変更（特に ValueId/ExitLine/Boundary 周り）は、
  **必ずこのファイルを更新してから** .hako 側にも段階的に反映する方針。

「JoinIR の仕様」「箱の責務」「境界の契約」は、このファイルを SSOT として運用していく。

---

## 5. 関連ドキュメント

- `docs/development/current/main/10-Now.md`
  - 全体の「いまどこ」を短くまとめたダッシュボード。
- `docs/development/current/main/phase78-85-boxification-feedback.md`
  - Boxification（箱化）の振り返りと、Phase 86 の推奨（小粒リファクタ）。
- `docs/private/roadmap2/phases/phase-180-joinir-unification-before-selfhost/README.md`
  - JoinIR 統一フェーズ全体のロードマップと進捗。
- 各 Phase 詳細:
  - 185–188: Strict mode / LoopBuilder 削除 / Pattern1–4 基盤
  - 189–193: Multi-function merge / Select bridge / ExitLine 箱化
  - 171–172 / 33‑10/13: ConditionEnv, ConditionBinding, JoinFragmentMeta, ExitLineRefactor 等
- `docs/development/current/main/loop_pattern_space.md`
  - JoinIR ループパターン空間の整理メモ。
    どの軸（継続条件 / break / continue / PHI / 条件変数スコープ / 更新パターン）でパターンを分けるか、
    そして P1–P4 / Trim(P5) の位置づけと、今後追加候補のパターン一覧がまとまっている。

---

## Roadmap 詳細版（JoinIR の今後のゴール）

ここから先の JoinIR の「目指す形」を、箱レベルでざっくり書いておくよ。フェーズ詳細は各 phase ドキュメントに分散させて、このセクションは常に最新の方向性だけを保つ。

### 直近（Phase 176-177 まわり）

- **P5（Trim/JsonParser 系）ループの複数キャリア対応** ✅ Phase 176 完了 (2025-12-08)
  - 完了内容:
    - Pattern2 lowerer を全キャリア対応に拡張（ヘッダ PHI / ループ更新 / ExitLine）。
    - CarrierUpdateLowerer ヘルパで UpdateExpr → JoinIR 変換を統一。
    - 2キャリア（pos + result）E2E テスト完全成功。
  - 技術的成果:
    - CarrierInfo / ExitMeta / ExitLine / LoopHeaderPhiBuilder の multi-carrier 対応を Pattern2 lowerer で完全活用。
    - Trim pattern の「キャリア = ループ変数」という誤解を解消（loop_var は特殊キャリア）。
  - 次のステップ (Phase 177):
    - JsonParser `_parse_string` 本体を P2+P5 で通す（pos + result の 2 キャリアで実ループ動作確認）。

### 6.2 中期（selfhost depth‑2 / JsonParser 本体）

- **JsonParserBox / Trim 系ループの本線化**
  - 目標:
    - `_trim` / `_skip_whitespace` / `_parse_string` / `_parse_array` などの主要ループが、すべて JoinIR Pattern1–4 + P5 で通ること。
    - LoopConditionScopeBox + LoopBodyCarrierPromoter + TrimLoopHelper の上で安全に正規化できるループを広げていく。
  - 方針:
    - 「ループの形」は P1–P4 から増やさず、複雑さは BoolExprLowerer / ContinueBranchNormalizer / P5 系の補助箱で吸収する。
    - LoopPatternSpace の P6/P7/P12 候補（break+continue 同時 / 複数キャリア条件更新 / early return）は、実アプリで必要になった順に小さく足す。

- **selfhost depth‑2（.hako JoinIR/MIR Frontend）**
  - 目標:
    - `.hako → JsonParserBox → Program/MIR JSON → MirAnalyzerBox/JoinIrAnalyzerBox → VM/LLVM` の深度 2 ループを、日常的に回せるようにする。
    - Rust 側の JoinIR は「JSON を受け取って実行・検証するランナー層」、.hako 側が「JoinIR/MIR を構築・解析する言語側 SSOT」という役割分担に近づける。
  - 前提:
    - 本ドキュメント（joinir-architecture-overview.md）を .hako 側の JoinIR 実装の参照設計として維持し、仕様変更は必ずここを更新してから .hako にも反映する。

### 6.3 当面やらないこと（Non‑Goals）

- ループパターンを闇雲に増やすこと
  - P1–P4（構造）＋ P5（body‑local 条件を昇格する補助パス）を「骨格」とみなし、  
    新しいパターンが必要になったときは LoopPatternSpace に追記してから、小さな箱で補う方針。
- LoopBuilder の復活や、JoinIR 以外の別ラインによるループ lowering
  - LoopBuilder 系は Phase 186–187 で完全に削除済み。  
    ループに関する新しい要件はすべて JoinIR 側のパターン/箱の拡張で扱う。
- JoinIR の中に言語固有のハードコード（特定 Box 名や変数名）を戻すこと
  - Trim/JsonParser 系は、構造（パターン）と補助箱（Promoter/Helper）で扱い、  
    「sum」「ch」など名前ベースの判定は LoopUpdateSummary / TrimLoopHelper の内部に閉じ込める。

この Roadmap は、JoinIR 層の変更や selfhost 深度を進めるたびに更新していくよ。

---

## 7. JoinIR 第1章：基盤完成サマリ（2025-12-09 時点）

### 7.1 現在の対応状況

**Pattern サポート**:
| Pattern | 説明 | 状態 |
|---------|------|------|
| P1 Simple | `loop(cond) { body }` | ✅ 完成 |
| P2 Break | `loop(cond) { if(...) break }` | ✅ 完成 |
| P3 If-PHI | `loop { if(...) a else b; use(φ) }` | ✅ 完成 |
| P4 Continue | `loop { if(...) continue }` | ✅ 完成 |
| P5 Trim | LoopBodyLocal → bool carrier 昇格 | ✅ 基本完成 |

**UpdateKind サポート**:
| UpdateKind | 例 | 状態 |
|------------|-----|------|
| CounterLike | `i = i + 1` | ✅ |
| AccumulationLike | `sum = sum + x` | ✅ |
| StringAppendChar | `s = s + ch` | ✅ |
| StringAppendLiteral | `s = s + "lit"` | ✅ |
| NumberAccumulation | `v = v * 10 + digit` | ✅ (Phase 190) |
| Complex | method call 含む | ❌ Fail-Fast |

**アーキテクチャ SSOT ライン**:
- ✅ LoopHeaderPhiBuilder: ループ変数・キャリアの PHI を SSOT で生成
- ✅ ExitLineReconnector: PHI dst → variable_map 接続
- ✅ JoinInlineBoundaryBuilder: 全パターンで Builder パターン統一
- ✅ JoinIRVerifier: デバッグビルドで契約検証
- ✅ ExitLine Contract Verifier: PHI 配線検証（Phase 190-impl-D）

### 7.2 残タスク（Phase 192+ で対応予定）

1. **✅ body-local 変数の init + update lowering** → Phase 191 完了
   - `local digit = i + 1` のような body-local 変数の JoinIR/MIR 生成完了
   - 対応済み: 整数リテラル、変数参照、二項演算
   - テスト: `phase191_body_local_atoi.hako` → 期待値 123 ✅

2. **✅ Complex addend 対応** → Phase 192 完了
   - `v = v * 10 + digits.indexOf(ch)` のような method call を含む NumberAccumulation対応
   - ComplexAddendNormalizer で `temp = f(x)` に分解してから NumberAccumulation に載せる実装完了
   - テスト: phase192_normalization_demo.hako → 123 ✅
   - 制約: MethodCall を含む init 式は Phase 193 で対応予定

3. **✅ MethodCall を含む init 式の対応** → Phase 193 完了、Phase 225 でメタ駆動化
   - `local digit = digits.indexOf(ch)` のような MethodCall init の lowering 完了
   - LoopBodyLocalInitLowerer 拡張（BoxCall emission）
   - **Phase 193**: ハードコードされた whitelist: indexOf, get, toString 対応
   - **Phase 225**: CoreMethodId メタ駆動化によりハードコード完全削除 ✅
     - `SUPPORTED_INIT_METHODS` 定数削除（メソッド名 whitelist）
     - Box 名の match 文削除（`indexOf → StringBox` 等のハードコード）
     - MethodCallLowerer への委譲により単一責任原則達成
     - `allowed_in_init()` メタデータで whitelist 管理
     - **substring メソッド追加**: Phase 225 で substring が body-local init で使用可能に
     - **コード削減**: -82 行（158削除 - 76追加）
   - 制約: body-local init のみ対応、condition 内の MethodCall は Phase 200+

4. **✅ JsonParser 実戦投入（P1/P2/P5 検証）** → Phase 194 完了
   - 4/10 ループが JoinIR 経路で動作確認 (40% coverage)
   - Target loops: _skip_whitespace, _trim (x2), _match_literal
   - Deferred loops: _parse_number, _atoi (ConditionEnv constraint)
   - Deferred loops: _parse_string, _unescape_string (complex carriers)
   - Deferred loops: _parse_array, _parse_object (multiple MethodCalls)
   - 詳細: phase194-loop-inventory.md, phase194-jsonparser-deployment.md

5. **Pattern 3 拡張（複数キャリア対応）** → Phase 195 + 196 完了 ✅
   - 目的: P3（If-Else PHI）で 2-3 個の Carrier を同時処理
   - **Phase 195**: Lowerer 側完了（multi-carrier PHI 生成: sum + count）✅
   - **Phase 196**: Select 二重 remap バグ修正 ✅
     - 根本原因: `instruction_rewriter.rs` で PHI inputs を二重 remap
     - 修正: block ID のみ remap、ValueId は既に remap 済み
   - **E2E 結果**: phase195_sum_count.hako → 93 ✅
   - 詳細: phase196-select-bug-analysis.md

6. **✅ JoinIR 実戦適用（軽量ループ検証）** → Phase 197 完了 ✅
   - 目的: Phase 196 までの安定基盤を実戦の小さいループで検証
   - 対象ループ（5本）:
     1. `_match_literal` (P1) - JsonParser 単純 while ✅
     2. `_skip_whitespace` (P2) - JsonParser break パターン ✅
     3. `phase195_sum_count.hako` (P3 multi-carrier) ✅
     4. `loop_if_phi.hako` (P3 single-carrier) ✅
     5. `loop_min_while.hako` (P1 minimal) ✅
   - 結果:
     - [x] routing 確認: 全ループ whitelisted ✅
     - [x] E2E 実行: 4/5 ループで期待値出力、1/5 routing 確認 ✅
     - [x] 退行なし: Phase 190-196 テスト全 PASS ✅
   - 詳細: phase197-lightweight-loops-deployment.md

7. **JsonParser/selfhost 実戦 JoinIR 適用状況** (2025-12-09 更新 → Phase 42 で棚卸し済み)

| Function | Pattern | Status | Note |
|----------|---------|--------|------|
| `_match_literal` | P1 | ✅ JoinIR OK | Phase 197 検証済み（E2E PASS）|
| `_skip_whitespace` | P2 | ✅ JoinIR OK | Phase 197 routing 確認（whitelisted）|
| `_trim` (leading) | P5 | ✅ JoinIR OK | Phase 173 実証済み |
| `_trim` (trailing) | P5 | ✅ JoinIR OK | Phase 173 実証済み |
| `phase195_sum_count` | P3 | ✅ JoinIR OK | Phase 196 検証済み（multi-carrier）|
| `loop_if_phi` | P3 | ✅ JoinIR OK | Phase 196 検証済み（single-carrier）|
| `loop_min_while` | P1 | ✅ JoinIR OK | Phase 165 基本検証済み |
| `_parse_number` | P2 | ✅ JoinIR OK | Phase 245B-IMPL: P2-Mid（num_str LoopState キャリア）を Structured→Normalized(dev, direct) で固定 |
| `_atoi` | P2 | ✅ JoinIR OK | Phase 246-EX で NumberAccumulation パターンとして統合（P2-Mid、Normalized: mini + real(dev, 符号対応) / canonical 準備中）|
| `_parse_string` | P3 | ⚠️ Deferred | 複雑キャリア（Phase 195+ 拡張後）|
| `_unescape_string` | P3 | ⚠️ Deferred | 複雑キャリア（Phase 195+ 拡張後）|
| `_parse_array` | - | ⚠️ Deferred | 複数 MethodCall（Phase 195+）|
| `_parse_object` | - | ⚠️ Deferred | 複数 MethodCall（Phase 195+）|

**Coverage**: 9/13 ループ JoinIR 対応済み（約 69%）
**Verification**: 代表ループ（P1/P2 Core + Trim/P3）については E2E テストで挙動確認済み。詳細なケースごとの状況は各 Phase ドキュメント（Phase 197/245/246 など）を参照。

8. **JsonParser 残り複雑ループへの適用（Phase 198+, 200+）**
   - Phase 200+: ConditionEnv 拡張 (function-scoped variables) → _parse_number, _atoi
   - Phase 198+: Pattern 3 拡張 (multi-flag carriers) → _parse_string, _unescape_string
   - Phase 198+: MethodCall 拡張 (multiple calls in body) → _parse_array, _parse_object
   - selfhost `.hako` コンパイラの全ループを JoinIR で処理 (Phase 210+)

9. **Pattern 3 If-Sum AST ベース実装** → Phase 213-217 完了 ✅
   - **Phase 213**: If-sum パターン AST ベース lowerer 実装
     - `loop_with_if_phi_if_sum.rs`: AST 抽出 + JoinIR 生成
     - Dual-mode 構成: if-sum mode / legacy mode
   - **Phase 214**: Dynamic join inputs 対応（hardcoded 3-input 除去）
   - **Phase 215**: ExprResult exit contract 確立
   - **Phase 216**: Selfhost if-sum production test 検証
   - **Phase 217**: Multi-carrier if-sum 動作確認（追加実装ゼロ行）
   - 詳細: phase213-if-sum-implementation.md, phase217-if-sum-multi.md

10. **Phase 218: JsonParser If-Sum 適用調査** → 🔍 調査完了
   - **目的**: JsonParser-style if-sum パターン (`sum = sum + digit`) への Pattern 3 適用
   - **結果**: パターン認識ギャップを発見
     - Phantom `count` carrier が誤検出され、`is_if_sum_pattern()` が false
     - AST ベース lowerer は実装済みだが起動されていない
     - 根本原因: carrier 検出ロジックの name heuristic が脆弱
   - **次フェーズ**: Carrier 検出修正（Phase 219）
   - 詳細: phase218-jsonparser-if-sum-min.md

---

## JoinIR 正規化レイヤ（構想 / Phase 26-H ライン）

### Structured JoinIR と Normalized JoinIR（概念レベル）

JoinIR ラインは、今後は「同じ JoinIR をフェーズごとに正規化していく」二段構成で扱う想定だよ：

- **Structured JoinIR（現行層）**
  - Pattern1–5 / CarrierInfo / ConditionEnv / UpdateEnv / Boundary / ExitLine までを終えた状態。
  - while/if/break/continue/return がまだ「構造」として残っていてよい層。
  - いまの JoinIR lowering（Pattern lowerer 群）が出力しているものはここ。

- **Normalized JoinIR（JoinIR’ / CPS 風層）**
  - 制御構造をすべて「関数＋継続＋ Env」の形に正規化した状態。
  - ループは `loop_step(env, k_exit)` と exit 継続のペア、if は `if_branch(env, k_then, k_else)` ＋ join 継続で表現。
  - while/break/continue/return は **TailCallFn/TailCallKont + If** だけに還元され、goto/生の branch は現れない。
  - Env は「ループキャリア + DigitPos/num_str などの Derived + captured 変数」を 1 つの struct としてまとめる。

パイプラインのイメージ：

```text
AST
  ↓
JoinIR(Structured)   // Pattern1–5, CarrierInfo, ConditionEnv/UpdateEnv, Boundary/ExitLine
  ↓ JoinIR パス A: 正規化（JoinIR → JoinIR）
JoinIR(Normalized)   // 関数 + 継続 + Env のみ（TailCall-only）
  ↓ JoinIR→MIR bridge
MIR
```

型レベルでは、次の二案のどちらかで扱う想定だよ（詳細は今後の Phase で選択）：

- **案① 型を分ける**
  - `JoinModuleRaw`（Structured）、`JoinModuleCps`（Normalized）を別型にする。
  - Bridge（JoinIR→MIR）は `JoinModuleCps` だけを受け取る。
- **案② 型は 1 つ＋フェーズフラグ**
  - `JoinModule { phase: JoinIrPhase, ... }` として、`phase = Structured / Normalized` をメタデータで持つ。
  - JoinIR パスはすべて `fn run(&mut JoinModule)`（= joinir→joinir）で構成し、
    Verifier が「phase=Normalized なら while/if/break/continue/return 禁止」などの不変条件をチェックする。

どちらの案でも本質は同じで、「JoinIR の下に JoinIR’（= 正規形フェーズ）を 1 段挟む」という設計、というのがポイントだよ。

### 3.2 Normalized JoinIR の基本モデル（ラフスケッチ）

Normalized JoinIR では、制御構造を次の 3 要素だけで表現する想定だよ：

- **Env（環境）**
  - そのループ/if に必要な情報を 1 つにまとめた struct。
  - フィールド種別の例:
    - `Carrier`（LoopState キャリア: i, sum, result, num_state 等）
    - `Derived`（DigitPos 二重値, NumberAccumulation 等）
    - `Captured`（外側のローカルや関数パラメータ: s, len 等）

- **Fn（通常の関数）**
  - 典型例: `loop_step(env, k_exit)` / `loop_body(env, k_exit)`。
  - 末尾は常に `TailCallFn` または `TailCallKont`。

- **Kont（継続）**
  - 典型例: ループ exit 後の処理、if 後の join、関数 return など。
  - 末尾は別の Fn/Kont への tail-call。

制御の不変条件（Normalized フェーズ）：

1. 制御フローは `TailCallFn` / `TailCallKont` / `If(cond, then_k, else_k, env)` のみで表現する。
2. 各ループは「`loop_step(env, k_exit)` 関数 + `k_exit(env)` 継続」のペアとして現れる（break は必ず k_exit へ TailCall）。
3. continue は `loop_step` への TailCall として現れる（ヘッダ PHI 相当は Env の書き込み順で表現）。
4. return は専用の `return_kont(env)` への TailCall で表現される。
5. EnvLayoutVerifier（検証箱）が、常に正しい EnvLayout が使われていることをチェックする。

これにより、現在 JoinIR 層で苦労している「PHI 配線」「exit_bindings/jump_args 整合性」「評価順のねじれ」は、
Normalized JoinIR 側では「Env フィールドの更新順」と「どの継続を呼ぶか」に還元される想定だよ。

#### 3.2.1 P1〜P4 と Normalized ループ形

Normalized JoinIR のゴールは、「P1〜P4 すべてのループを同じループ骨格（`loop_step(env, k_exit)` + 継続）で表現する」ことだよ：

- P1 Simple:
  - `loop_step(env, k_exit)` だけを持つ最小形（break/continue なし）。
  - 条件が false になったら `k_exit(env)` に TailCall、true なら body を 1 ステップ進めて再び `loop_step` を呼ぶ。
- P2 Break:
  - P1 の形に「body 後の break 判定」と `k_exit(env)` への分岐が加わったもの。
  - break/continue/return 自体はすべて継続呼び出し（`TailCallKont` / `TailCallFn`）として表現され、型レベルでは P1 の上位互換。
- P3 If‑PHI:
  - ループ本体に `if_branch(env, k_then, k_else)` ＋ join 継続を挟んだ形として表現し、PHI は「Env のフィールド更新順＋ join 継続」で表す。
- P4 Continue:
  - `continue` は「更新後に `loop_step(env', k_exit)` へ TailCall」するだけの構造として扱い、break/return と同じく継続レベルで閉じる。

設計としては **Normalized IR の型と不変条件は P1〜P4 で共通**で、違いは Structured 層からどのように `loop_step` / 継続を合成するか（`LoopCpsSynthesizer` 側の分岐）だけに閉じ込める前提だよ。

- Phase 26‑H / 28–43 では、まず P1/P2（特に JsonParser の P2-Core/P2-Mid）だけを Normalized に載せて設計を固定した。
- P3/P4 の Normalized 対応は今後のフェーズ（NORM‑P3 / NORM‑P4 ライン）で、上の骨格に揃える形で段階的に進める。

この方針により、「先に P1/P2 だけ Normalized を実装しても、後から P3/P4 を同じ器に寄せられる」ことを文書上で保証しておく。

### 3.3 Normalized JoinIR 導入のメリットと初期コスト

Normalized JoinIR を 1 段挟むと、開発の手触りがどう変わるかをここでまとめておくよ。

#### 3.3.1 楽になるポイント

- **デバッグ時の「問いの分離」**
  - Structured JoinIR だけで `_parse_number` / `_atoi` を追うときは、常に同時に 4 つくらいの問いを抱えることになる：
    1. Pattern 判定は正しいか（P1/P2/P3/P4/P5）  
    2. CarrierInfo / LoopState / LoopLocalZero / FromHost の設計は妥当か  
    3. 評価順（header → body-init → break → updates → tail）は正しいか  
    4. PHI / exit_bindings / jump_args / variable_map の整合は取れているか  
  - Normalized JoinIR を挟むと、これが層ごとに分割される：
    - 上（Structured）: Pattern 判定 + CarrierInfo/Condition/Update の設計  
    - 中（Normalized）: EnvLayout + step/kont 合成（制御骨格）  
    - 下（Bridge）: TailCall＋Env から MIR/VM の PHI/branch を作る  
  - これにより、「これは Pattern/Carrier 問題」「これは Env＋継続の組み立て問題」「これは Bridge 問題」と、デバッグ時にレイヤ単位で切り分けやすくなる。

- **PHI まわりの思考負荷軽減**
  - Structured JoinIR では、LoopHeader PHI / Exit PHI / Select 展開 PHI と、exit_bindings/jump_args/variable_map 再接続が絡み合う。
  - Normalized フェーズでは、キャリアや DigitPos/NumberAccumulation などはすべて Env フィールドになり、PHI は JoinIR’ には現れない：
    - 「どこから値が来たか？」は「どの Env フィールドがどの順番で更新されたか」を見るだけでよくなる。
    - PHI/exit_bindings は Bridge 側の責務として明示的に分離される。

- **while/if/break/continue/return の骨格が共通化される**
  - Pattern1–5 ごとに似たロジックが散らばりがちな現在と違い、Normalized では：
    - ループ: `loop_step(env, k_exit)` ＋ `loop_body(env, k_exit)`  
    - if: 条件判定 → then/else 継続 → join 継続  
    - break: `TailCallKont(k_exit, env)`  
    - continue: `TailCallFn(loop_step, env', k_exit)`  
  - こうした「世界共通のテンプレ」を 1 箱にまとめておけるので、新しい Pattern や JsonParser/selfhost のループを追加するときも、構造箱（Structured）側だけで頑張ればよくなる。

- **「1箱 = 1質問」を守りやすくなる**
  - Structured JoinIR では、PatternLowerer が一時的に「構造＋評価順＋PHI 配線＋exit_bindings」を抱える瞬間がある。
  - Normalized を挟むと：
    - Structured: 「どの値を Env に載せるか」を決める箱  
    - Normalized: 「Env と継続にどう分解するか」を決める箱  
    - Bridge: 「TailCall＋Env を MIR/VM にどう落とすか」を決める箱  
  - という分担にできるので、「この箱は何の質問に答えるか」が清潔に保ちやすくなる。

#### 3.3.2 初期コストと注意点

- **新しい IR モデルを 1 セット覚える必要がある**
  - Env＋Fn＋Kont＋TailCall の世界観を JoinIR 層の前提として追加することになる。
  - 一度定着すれば Structured より思考負荷は下がるが、導入初期は「型・不変条件・ダンプの読み方」に慣れるまで少しコストが乗る。

- **移行期間中は 2 経路の比較が発生する**
  - AST → JoinIR(Structured) → MIR  
  - AST → JoinIR(Structured) → JoinIR(Normalized) → MIR  
  - の 2 経路をしばらく併走させ、dev で結果比較をするフェーズが必要になる。
  - Phase 26-H / 28-NORM-P2 のように「小さいサブセットから比較用にだけ回す」方針で進めることで、コストは制御可能にする。

- **導入フェーズごとに Fail-Fast とテストが必須**
  - Structured→Normalized は対応範囲外の Structured JoinModule については必ず Fail-Fast（panic/Err）し、サイレントに正規化を試みない。
  - `normalized_dev` feature 配下で、Structured↔Normalized↔Structured の roundtrip と VM 実行結果比較を常に追加していく運用にする。

全体として、短期的には「新しいレイヤと比較テストを抱えるコスト」が増えるけれど、中長期では Pattern2〜4 や JsonParser 系の「評価順＋PHI＋DigitPos/num_str」が絡むループで、デバッグと設計の負荷をかなり下げることを狙った設計だよ。

### 3.4 Phase 26-H のフェーズ分割（案）

この正規化レイヤを一度に入れるのは重いので、Phase 26-H ラインとして小さく段階分けして進める方針だよ。
（番号は仮で、実際の Phase 番号は current の進行に合わせて調整する想定）

1. **Phase 26-H.A – モデル定義と Doc 固定（コード変更最小）**
   - JoinIR Architecture Overview（このファイル）に Structured / Normalized の二層構造を明文化（本節）。
   - `JoinIrPhase` などのメタ情報だけを追加し、既存 lowering / Bridge の挙動は変えない。
   - Normalized JoinIR の不変条件（TailCall-only / EnvLayout 整合性）を Verifier の設計として書き出す。

2. **Phase 26-H.B – 最小サブセットでの JoinIR→JoinIR パス導入**
   - Pattern1（単純 while, break/continue なし）だけを対象にした Normalized パスを実装。
   - パイプライン:
     - 既存: `JoinIR(Structured) → MIR`
     - 新規: `JoinIR(Structured) → JoinIR(Normalized, small subset) → MIR`
   - 新規パスは dev フラグの裏側で比較用にのみ使い、E2E で結果が一致することを確認（既定経路は従来のまま）。

3. **Phase 26-H.C – Pattern2–4 / DigitPos / JsonParser への拡張**
   - Pattern2/3/4 と DigitPos / NumberAccumulation / Trim を Normalized パスの対象に広げる。
   - JsonParser `_parse_number` / `_atoi` を優先対象にし、JoinIR(Normalized) 経由での実行を dev フラグ付きで有効化。
   - JoinIR(Structured) → JoinIR(Normalized) の各ステップに構造テストを追加し、「Env の更新」「継続の呼び出し順」が期待どおりかを固定。

4. **Phase 26-H.D – Normalized JoinIR を canonical route に昇格**
   - JoinIR→MIR Bridge が「Normalized フェーズの JoinIR だけ」を受け取るように変更。
   - Structured JoinIR → MIR の直接パスは「比較テスト専用」として残すか、段階的に削除。
   - selfhost / JsonParser / hako_check の代表ループはすべて Normalized JoinIR 経由で通ることを確認。

各サブフェーズでは「既存意味論を変えない」「Fail-Fast 原則を維持する」「新旧経路の比較テストを先に用意する」をガードとして運用するよ。
詳細な API/型設計は、Phase 26-H.A/B の中で `EnvLayoutBox` / `LoopStepSynthesizer` / `JpVerifier` 等の箱として段階的に固めていく想定。

### 3.5 Phase 27-CLEAN – Pattern2–4 の軽量整理

- Pattern2〜4/loop_with_break_minimal まわりで可視性・ログ・補助関数を整理し、joinir_dev フラグ配下のデバッグログに寄せる。意味論は変えずに「読みやすさ」「追いやすさ」を優先するクリーンアップフェーズだよ。

### 3.6 Phase 28-NORM-P2 – Normalized JoinIR プロトタイプ拡張（dev-only）

- Phase 26-H で用意した Normalized JoinIR の極小サブセットを、Pattern1 に続いて Pattern2（最小 break ループ）まで拡張。
- Structured → Normalized → Structured の往復と VM 実行比較を dev フィーチャ (`normalized_dev` + debug) でテスト済み。
- 対象は joinir_min_loop 相当の「ループ変数1つ＋breakのみ」のミニケースに限定し、本番経路（Structured→MIR）は不変。
- normalize_pattern2_minimal は対応外の Structured JoinModule では Fail-Fast するようにガードを追加し、対応範囲をテストで固定。

### 3.7 Phase 29-NORM-P2-APPLY – Pattern2 実ループへの dev 適用

- Phase 34 の fixture `loop_frontend_break.program.json`（`i/acc/n` のシンプル break ループ）を Structured→Normalized→Structured の往復経路に載せ、VM 実行結果が Structured 直経路と一致することを dev テストで確認。
- `normalize_pattern2_minimal` のガードを 3 パラメータ（loop var + acc + host）まで許容する形に緩めつつ、DigitPos/Trim などの heavy carrier は依然として非対応に固定。
- すべて `normalized_dev` feature + debug_assertions 配下の実験経路に閉じ、本番 Structured→MIR パスの挙動は不変。

### 3.8 Phase 30-NORM-P2-DEV-RUN – runner で Normalized を試走（dev）

- JoinIR runner に dev 専用の `NYASH_JOINIR_NORMALIZED_DEV_RUN=1` スイッチを追加し、Pattern1/2 のミニケースだけ Structured→Normalized→Structured を噛ませてから実行できるようにした（`normalized_dev` feature + debug ビルド限定）。
- runner 経路でも Structured 直実行との stdout/結果が一致することをテスト（loop_min_while と Phase 34 break fixture）で確認。フラグ OFF 時の挙動は従来と同じ。

### 3.9 Phase 31-NORM-JP-MINI – JsonParser ミニ P2 を Normalized dev 経由で試走

- JsonParser 系のシンプルな P2 ループ（skip_whitespace 簡易版）を Structured→Normalized→Structured の dev ランナー経路に載せ、通常経路との実行結果一致を比較。
- `jsonparser_skip_ws_mini.program.json`（docs/private/roadmap2/phases/normalized_dev/fixtures 配下）由来の JoinModule を使い、`NYASH_JOINIR_NORMALIZED_DEV_RUN=1` + `normalized_dev` + debug 限定で切替可能にした。
- 本番経路（Structured→MIR）は引き続き不変で、Normalized は dev 比較専用のまま。

### 3.10 Phase 32-NORM-CANON-PREP – Normalized 本番導入の下地づくり

- JoinIR→MIR ブリッジに Structured/Normalized の入口を分けた `bridge_joinir_to_mir_*` を用意し、conversion pipeline / VM ランナーがこの 1 箇所で dev roundtrip を切替できるように整理。
- Normalized dev スイッチを `normalized_dev_enabled()` に集約（feature `normalized_dev` + `NYASH_JOINIR_NORMALIZED_DEV_RUN=1`）。P1/P2 ミニ + JsonParser mini で env ON/OFF の比較テストを追加し、いつでも canonical route に昇格できる状態を固めた。

### 3.11 Phase 33-NORM-CANON-TEST – P1/P2/JP mini をテスト必須ラインへ

- `bridge_joinir_to_mir` / JoinIR runner は `shape_guard` で P1/P2 ミニ + JsonParser skip_ws mini を検知した場合、`normalized_dev_enabled()` が ON なら必ず Structured→Normalized→Structured の dev roundtrip を経由（正規化失敗は dev panic）。未対応形状は静かに Structured 直通。
- tests/normalized_joinir_min.rs を Phase 33 前提に拡張し、P1/P2/JP mini の runner/VM 比較テストを env ON で実行。Normalized が壊れればこのスイートが必ず赤になる構造にした（feature OFF の CI は従来どおり無関係）。
- 本番 CLI 挙動は Structured→MIR のまま維持しつつ、Normalized を canonical に昇格させる前段階として dev テストで SSOT 相当の役割を担わせている。

### 3.12 Phase 34-NORM-ATOI-DEV – JsonParser `_atoi` ミニを dev 正規化経路へ

- JsonParser `_atoi` の最小 P2 ループ（digit_pos → digit_value + NumberAccumulation）を normalized_dev で Structured→Normalized→Structured に往復させ、VM 実行結果を Structured 直経路と比較するテストを追加。
- フィクスチャ `jsonparser_atoi_mini.program.json` を `shape_guard::JsonparserAtoiMini` で検知し、dev roundtrip が必ず通るようにした（正規化失敗は dev panic）。本番 CLI は引き続き Structured→MIR 既定のまま。

### 3.13 Phase 35-NORM-BRIDGE-MINI – Normalized→MIR 直ブリッジ（P1/P2 ミニ + JP mini/atoi）

- normalized_dev 有効時に、P1/P2 ミニ・JsonParser skip_ws/atoi ミニを Structured→Normalized→MIR（Structured に戻さない）で実行する dev 専用ブリッジを追加。
- `bridge_joinir_to_mir` が shape_guard 対応形状では Normalized→MIR 直経路を使い、従来の Structured→MIR と VM 実行結果が一致することを比較テストで固定（env OFF 時は従来経路のまま）。

### 3.14 Phase 36-NORM-BRIDGE-DIRECT – Normalized→MIR を direct 実装に収束（P1/P2 ミニ + JP mini/atoi）

- Normalized ブリッジを direct 実装と Structured 再構成の二段に分離し、shape_guard で direct 対象（P1/P2 ミニ + JsonParser skip_ws/atoi ミニ）だけを Normalized→MIR 直接生成に切り替えた。
- direct 経路は `normalized_bridge::direct` に閉じ込め、非対応形状は `[joinir/normalized-bridge/fallback]` ログ付きで Structured 再構成経路に落とす構造に整理。dev テストでは direct 経路の VM 出力が従来経路と一致することを固定。

### 3.15 Phase 37-NORM-JP-REAL – JsonParser `_skip_whitespace` 本体を dev Normalized で比較

- JsonParser 本体の `_skip_whitespace` ループを Program(JSON) フィクスチャ化し、`shape_guard` で real 版を検知して Structured→Normalized→MIR(direct) の dev 経路に通すように拡張。`extract_value` は `&&`/`||` を BinOp として受け付けるようにした。
- Break パターンのパラメータ推定を柔軟化（loop_var/acc/n が無いケースでも loop_var を優先し、acc が無ければ同一キャリアとして扱う）し、skip_ws real の構造で panic しないようにした。
- tests/normalized_joinir_min.rs に `_skip_whitespace` real フィクスチャの VM 比較テストを追加し、env ON 時は Structured→Normalized→MIR(direct) と Structured 直経路の stdout が一致することを固定（env OFF は既存経路のまま）。
- normalized_dev 用フィクスチャは `docs/private/roadmap2/phases/normalized_dev/fixtures/` に配置し、Program(JSON) から `AstToJoinIrLowerer` で読み込む運用に統一した。

### 3.16 Phase 38-NORM-OBS – Normalized dev ログ/Fail‑Fast の整備

- Normalized/JoinIR dev 経路のログカテゴリを `[joinir/normalized-bridge/*]` / `[joinir/normalized-dev/shape]` に統一し、`JOINIR_TEST_DEBUG` フラグ下のみ詳細を出すよう静音化。Verifier/Fail‑Fast メッセージも shape/役割付きに整理してデバッグ観測性を強化。

### 3.17 Phase 40-NORM-CANON-TESTS – テスト側で Normalized を“当たり前”に通す

- `normalized_dev_enabled()` と env ガードを整理し、P1/P2 ミニ + JsonParser skip_ws/atoi ミニ/real の代表テストは「Normalized dev 経路が必ず通る」前提にする（壊れたら normalized_* スイートが赤になる）。
- 既存の Structured 直経路は比較用に維持しつつ、tests/normalized_joinir_min.rs 経路では Structured→Normalized→MIR(direct) が第一観測点になるように整備（本番 CLI は Structured→MIR のまま）。

### 3.18 Phase 41-NORM-CANON-P2-CORE – Pattern2 コアケースの canonical Normalized 化

- Pattern2 のコアセット（P2 ミニ + JsonParser skip_ws/atoi ミニ/real）について、JoinIR→MIR Bridge の既定を Normalized→MIR に寄せ、Structured→MIR は比較テスト用/soft fallback（JoinIR 内）の位置づけにする（Fail‑Fast ポリシーは維持）。
- `shape_guard` で「Normalized 対応と宣言した P2 コアループ」は常に Normalized 経路を通すようにし、Normalized 側の invariant 破損は dev では panic、本番では明示エラーで早期検出する設計に寄せる。

### 3.19 Phase 42-NORM-P2-INVENTORY – P2 コア/ミドル/ヘビーの棚卸し

- JsonParser / selfhost で **現役の P2 ループ** を洗い出し、次の 3 クラスに整理した：
  - **P2-Core**（すでに Normalized canonical なもの）
    - test fixture 系: `loop_min_while` P2 ミニ, Phase 34 break fixture (`i/acc/n`)
    - JsonParser 系: `_skip_whitespace` mini/real, `_atoi` mini
    - これらは Phase 36–41 で **Structured→Normalized→MIR(direct)** が canonical になっており、`bridge_joinir_to_mir` でも優先的に Normalized 経路が選ばれる。
  - **P2-Mid**（次に Normalized を当てる候補）
    - JsonParser: `_parse_number`, `_atoi` 本体, `_atof_loop`
    - いずれも Pattern2 Break で JoinIR(Structured) には載っており（Phase 245/246 系）、Normalized への写像は今後の拡張対象として扱う。
  - **P2-Heavy**（複数 MethodCall / 複雑キャリアを持つもの）
    - JsonParser: `_parse_string`, `_parse_array`, `_parse_object`, `_unescape_string`
    - P2/P3/P4 が混在し、複雑なキャリアや MethodCall 多数のため、Phase 43 以降の後続フェーズで設計する。
- P2-Core については Phase 41 で canonical Normalized 化が完了しており、Structured→MIR は比較テスト用 / soft fallback（JoinIR 内）の経路として扱う。
- P2-Mid のうち、Phase 43 ではまず `_parse_number` を第 1 候補、`_atoi` 本体を第 2 候補として扱い、Normalized→MIR(direct) に必要な追加インフラ（EnvLayout 拡張 / JpInst パターン拡張）を段階的に入れていく前提を整理した。

### 3.20 Phase 43-NORM-CANON-P2-MID – JsonParser 本命 P2（_parse_number/_atoi）への適用 ✅ COMPLETE

**完全サマリ**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)

- JsonParser `_parse_number` / `_atoi` 本体の Pattern2 ループを、既存インフラ（DigitPos dual 値, LoopLocalZero, StepScheduleBox, ExprLowerer/MethodCall, Normalized ブリッジ）上で Structured→Normalized→MIR(direct) に載せる。
- dev で Structured 直経路との VM 実行結果一致を固定した上で、段階的に「この関数だけ Normalized canonical」とみなすプロファイル/フラグを導入し、最終的に JsonParser P2 の canonical route を Normalized 側に寄せるための足場にする。
- Phase 43-A（dev 専用）: `_atoi` 本体を Program(JSON) フィクスチャ `jsonparser_atoi_real` で Structured→Normalized→MIR(direct) に通し、Structured 直経路との VM 出力一致を比較テストで固定（符号あり/なしの簡易パスまで対応。canonical 化は後続フェーズで検討）。
- Phase 43-C（dev 専用）: `_parse_number` 本体を Program(JSON) フィクスチャ `jsonparser_parse_number_real` で Structured→Normalized→MIR(direct) に通し、`num_str = num_str + ch` の LoopState キャリアを含めた状態で Structured 直経路との VM 出力一致を比較テストで固定。

### 3.21 Phase 44-SHAPE-CAP – shape_guard の能力ベース化 ✅ COMPLETE (2025-12-12)

**完全サマリ**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
**設計詳細**: [phase44-shape-capabilities-design.md](./phase44-shape-capabilities-design.md)

- ✅ **ShapeCapabilityKind 導入完了**: P2CoreSimple / P2CoreSkipWs / P2CoreAtoi / P2MidParseNumber の 4 種類
- ✅ **Capability-based API**: `capability_for_shape()`, `is_canonical_shape()`, `is_p2_core_capability()`, `is_supported_by_normalized()`
- ✅ **Shape-level と Capability-level の二層 API**: 正確なマッチング vs 広い能力ファミリ判定
- ✅ **拡張性確保**: 将来の carrier_roles, method_calls フィールド用の struct 設計
- ✅ **既存挙動完全保持**: 937/937 tests PASS

### 3.22 Phase 45-NORM-MODE – JoinIR モードの一本化 ✅ COMPLETE (2025-12-12)

**完全サマリ**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
**設計詳細**: [phase45-norm-mode-design.md](./phase45-norm-mode-design.md)

- ✅ **JoinIrMode enum 導入完了**: StructuredOnly / NormalizedDev / NormalizedCanonical
- ✅ **current_joinir_mode() 統一**: バラバラだったフラグ/feature を一箇所に集約
- ✅ **Canonical-first routing**: P2-Core canonical shapes は mode 無視で常に Normalized→MIR(direct)
- ✅ **Mode-based 分岐統一**: bridge/runner の pattern matching で一貫した挙動
- ✅ **既存挙動完全保持**: 937/937 tests PASS

### 3.23 Phase 46-NORM-CANON-P2-MID – Normalized Canonical P2-Mid 昇格 ✅ COMPLETE (2025-12-12)

**設計詳細**: [phase46-norm-canon-p2-mid.md](./phase46-norm-canon-p2-mid.md)

P2-Mid パターン（_atoi real, _parse_number real）を canonical Normalized→MIR(direct) ルートに昇格。

**Canonical set 拡張（Phase 46）**:
- P2-Core: Pattern2Mini, skip_ws mini/real, atoi mini
- **P2-Mid: atoi real, parse_number real** (NEW)

JsonParser _skip_whitespace / _atoi / _parse_number が**すべて canonical Normalized** に。Structured→MIR は P2 ラインにおいてレガシー/比較用途のみ。

**スコープ外**: P3/P4 Normalized 対応（NORM-P3/NORM-P4 フェーズで実施）。

**変更ファイル**:
- `shape_guard.rs`: `is_canonical_shape()` 拡張（+2 パターン）
- `bridge.rs`: コメント更新（Phase 41 → Phase 46）
- `normalized_joinir_min.rs`: canonical set 検証テスト追加

**テスト**: 938/938 PASS

### 3.24 Phase 47-NORM-P3 – Normalized P3 (If-Sum) Support 🏗️ DESIGN + MINIMAL DEV + DIRECT (2025-12-12)

**設計詳細**: [phase47-norm-p3-design.md](./phase47-norm-p3-design.md)

Pattern3 (if-sum) ループを Normalized JoinIR に対応させる。P2 と同じ ConditionEnv/CarrierInfo/ExitLine インフラを再利用。

**Phase 220 基盤**: P3 if-sum は Phase 220 で ConditionEnv 統合済み（既に P2 と同じ Structured JoinIR foundation を持つ）

**Key difference**: P3 は **conditional carrier updates**（if 内でのみキャリア更新）vs P2 の unconditional updates before break

**Phase 47-A**: Minimal sum_count（dev-only 正規化・第1ステップ）
- ✅ AST ベース if-sum lowerer（`loop_with_if_phi_if_sum.rs`）で `phase212_if_sum_min.hako` 相当の Structured JoinModule（3 関数構成）を生成
- ✅ Normalized dev ランナー経路で Structured→Normalized→Structured roundtrip を通し、JoinIR Runner の実行結果が一致することをテストで固定  
  （`build_pattern3_if_sum_min_structured_for_normalized_dev` + `normalized_pattern3_if_sum_minimal_runner_dev_switch_matches_structured`）
- ✅ StepScheduleBox を汎用化し、P3 用 StepKind（`IfCond` / `ThenUpdates` / `ElseUpdates`）を追加
- ✅ shape_guard に `Pattern3IfSumMinimal` を追加し、構造ベースで P3 最小 if-sum 形状を検出
- ✅ `normalize_pattern3_if_sum_minimal` を通じて P3 最小ケースを Normalized→MIR(direct) パイプラインに載せ、P1/P2 と同じ direct ブリッジで実行結果一致を確認（dev-only）

**Phase 47-B**: Extended dev（sum+count / JsonParser if-sum mini）
- ✅ フィクスチャ追加: `pattern3_if_sum_multi_min`（sum+count）/ `jsonparser_if_sum_min`（JsonParser 由来）
- ✅ ShapeGuard: `Pattern3IfSumMulti` / `Pattern3IfSumJson` 追加、capability=P3IfSum
- ✅ Normalizer/Bridge: P3 if-sum multi/json を Structured→Normalized→MIR(direct) で dev A/B（Structured と一致）
- ✅ VM Bridge テスト: `normalized_pattern3_if_sum_multi_vm_bridge_direct_matches_structured` / `normalized_pattern3_json_if_sum_min_vm_bridge_direct_matches_structured`

**Phase 47-C**: Canonical promotion（P3 minimal/multi/json → canonical Normalized）
- Canonical set 拡張: P3 if-sum minimal/multi/json を `is_canonical_shape()` に追加、mode/env 無視で Normalized→MIR(direct) ルートを使用
- Bridge/runner: canonical shapes は Structured fallback せず fail-fast（dev logs は normalized-dev プレフィックスに統一）

**スコープ外**: P4 (continue) 対応（NORM-P4 フェーズで実施）、Complex P3 patterns（後続フェーズ）

### 3.25 Phase 48-NORM-P4 – Normalized P4 (Continue) 🏗️ DESIGN + PHASE 48-A/B/C CANONICAL COMPLETE (2025-12-12 → 2026-01-XX)

**設計詳細 / 実装サマリ**: [phase48-norm-p4-design.md](./phase48-norm-p4-design.md)

P4 (continue) は P1/P2/P3 と同じ `loop_step(env, k_exit)` 骨格を使う設計だよ。  
**Key insight**: `continue` = 「更新済み Env での `TailCallFn(loop_step, env', k_exit)`」で表現できる（新しい命令種別は不要）。

**Target loops** (JsonParser):
- ◎ `_parse_array` (skip whitespace) – PRIMARY（Phase 48-A 対象）
- ○ `_parse_object` (skip whitespace) – Extended
- △ `_unescape_string`, `_parse_string` – Later

**Infrastructure reuse**: P2/P3 Normalized の 95% 以上をそのまま再利用
- 共通: EnvLayout / ConditionEnv / CarrierInfo / ExitLine / JpInst
- 追加: StepScheduleBox に `ContinueCheck` step kind を追加

**Phase 48-A（Minimal continue, dev-only）実装ステータス**:
- Fixture: `pattern4_continue_min.program.json`（`i == 2` を `continue` でスキップする最小 P4 ループ）
- ShapeGuard: `NormalizedDevShape::Pattern4ContinueMinimal` を追加し、構造ベースで minimal continue 形状を検出
- StepSchedule: `HeaderCond → ContinueCheck → Updates → Tail` の順序を固定
- Normalized lowering: `normalize_pattern4_continue_minimal()` を実装し、P2 正規化ロジックを約 95% 再利用
- テスト:
  - Normalized dev スイートに P4 minimal の比較テストを 4 本追加  
    （Structured→Normalized→MIR(direct) vs Structured→MIR / runner / VM bridge）
  - `cargo test --release` ベースで **939/939 tests PASS**（Phase 48-A 実装時点）

**Phase 48-B（JsonParser continue skip_ws、dev-only）実装ステータス**:
- Fixtures: `jsonparser_parse_array_continue_skip_ws.program.json` / `jsonparser_parse_object_continue_skip_ws.program.json`
- ShapeGuard: JsonParser continue ループ用の shape を追加（array/object 両方）
- Normalized lowering: `normalize_jsonparser_parse_array_continue_skip_ws` / `_parse_object_...` で Structured→Normalized→MIR(direct) を dev 比較
- テスト: normalized dev スイートに VM bridge 比較テストを追加（Structured 直経路と stdout 一致を確認）

**Phase 48-C（canonical 昇格）実装ステータス**:
- Canonical set 拡張: Pattern4 continue minimal / JsonParser skip_ws array/object を `is_canonical_shape()` に追加
- Bridge/runner: P4 canonical shapes は env 無しでも Normalized→MIR(direct) を必ず通る（Structured fallback 無し、fail-fast）
- テスト: canonical ルート（env OFF）と Structured 直経路の stdout 一致を比較するテストを追加

**Phase 48 doc is SSOT** for P4 Normalized design + 48-A/B/C サマリだよ。  
P1〜P4 の代表ループがすべて canonical Normalized パイプラインに載った状態になった。

### 3.26 Phase 49-SELFHOST-NORM-DEPTH2 – Selfhost depth2 Normalized 設計フェーズ（docsのみ）

- SSOT: [phase49-selfhost-joinir-depth2-design.md](./phase49-selfhost-joinir-depth2-design.md)
- 目的: selfhost ラインでも `.hako → Program/MIR JSON → JoinIR(Structured) → Normalized → MIR → VM/LLVM` の depth2 パイプラインを踏めるように、対象ループ（軽い P2/P3 を 1〜2 本）と適用方針を設計で固定する。
- スコープ: selfhost の P2/P3 軽量ループのみ（トークン走査系 P2・if-sum 系 P3 を候補化）。heavy ループや P5/Trim 系は Phase 50+ に回す。
- 設計アウトプット: 対象ループ↔Pattern/shape マッピング表、Program JSON/fixture/test 計画、depth2 パイプラインの責務整理（コード変更なし）。

### 3.27 Phase 50-SELFHOST-NORM-DEV – selfhost P2/P3 の dev Normalized 実装

- 対象: selfhost_token_scan_p2（P2 break カウンタループ）/ selfhost_if_sum_p3（P3 if-sum sum+count）の 2 本に限定。
- Fixtures: `selfhost_token_scan_p2.program.json` / `selfhost_if_sum_p3.program.json` を normalized_dev フィクスチャ群に追加。
- ShapeGuard: `SelfhostTokenScanP2` / `SelfhostIfSumP3` 形状を追加し、canonical P2/P3 とは分離（Pattern2/3 minimal に吸われないようガード）。
- Normalizer/Bridge: 既存 Pattern2/3 normalizer を流用して Structured→Normalized→MIR(direct) を dev 実行、構造/VM 出力を Structured 直経路と比較。
- テスト: normalized_joinir_min.rs に selfhost P2/P3 の VM ブリッジ比較テストを追加（normalized_dev 前提）、shape_guard の検出テストも拡張。
