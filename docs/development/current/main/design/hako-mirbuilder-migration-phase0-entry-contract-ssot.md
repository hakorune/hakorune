---
Status: Draft (Phase-0 SSOT; implementation staged)
Scope: `.hako` mirbuilder migration — entrypoints, contracts, fail-fast tags, and verification routes
Related:
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
  - docs/reference/language/block-expressions-and-map-literals.md
  - docs/development/current/main/design/cond-block-view-prelude-ssot.md
  - src/cli/args.rs
  - src/runner/dispatch.rs
  - src/runner/mir_json_v0.rs
  - src/runner/mir_json_emit/mod.rs
---

# .hako MirBuilder Migration — Phase-0 (Entry / Contract / Verification) (SSOT)

## Goal (Phase-0)

Phase-0 は “実装を進められる入口” を固定するフェーズ:

- 入出力（I/O）契約を SSOT 化して、以降の実装を fixture 駆動で 1個ずつ増やせる状態にする
- MIR JSON を中間成果物として、Rust VM で実行し stdout で同値検証できる導線を確立する

## Non-goals (Phase-0)

- lowering の受理形を増やすこと（Phase-0 は BoxCount ではなく導線整備）
- SSA / rewrite / 最適化を `.hako` mirbuilder に持ち込むこと
- silent fallback（成功に見せる回避）
- async/並行性の “本格実装”（Phase-0 では意味論・state-machine・スケジューラ連携を扱わない）

## Read order (SSOT)

1) ルート: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`  
2) 言語境界: `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`  
3) 構文: `docs/reference/language/block-expressions-and-map-literals.md`  
4) cond prelude: `docs/development/current/main/design/cond-block-view-prelude-ssot.md`

## Operating rule (Phase-0)

- 1ブロッカー = 1コミット（fixture + gate で pin してから次へ）
- 失敗は fail-fast: `[freeze:contract][hako_mirbuilder] ...` を先頭に統一

## Concurrency note (Phase-0; SSOT)

- `nowait` / `await` は既存構文だが、Phase-0 の `.hako mirbuilder` 移植の必須要件には含めない（selfhost compiler が依存しない前提）。
- ただし “使わない” は “消す/無視する” ではない。VM+LLVM の整合 pin は別 SSOT で扱う。
  - SSOT: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- Phase-0 で async 構文に遭遇した場合は、回避せず `[freeze:contract][hako_mirbuilder] ...` で fail-fast する（silent fallback 禁止）。

## Phase-0 Steps (recommended order)

### 0) Baseline (mandatory)

- `git status -sb`
- `cargo check --bin hakorune`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`

### 1) Rust side: MIR JSON emit/run route (use existing functionality)

Phase-0 の前提として、Rust 側はすでに次の導線を持つ:

- Emit: `--emit-mir-json <FILE>`（MIR JSON を出して終了）
- Exec: `--mir-json-file <FILE>`（MIR JSON を読み、VM で実行して exit code を返す）

SSOT entrypoints:

- CLI args: `src/cli/args.rs`
- Dispatch: `src/runner/dispatch.rs`（`--mir-json-file` は schema v1 → v0 の順に試行して実行）
- Minimal v0 loader: `src/runner/mir_json_v0.rs`
- MIR JSON emitter: `src/runner/mir_json_emit/mod.rs`

Example (release build):

- `cargo build --release --bin hakorune`
- `./target/release/hakorune --emit-mir-json /tmp/out.json apps/tests/phase29bq_blockexpr_basic_min.hako`
- `./target/release/hakorune --mir-json-file /tmp/out.json`

Note:

- `--mir-json-file` は schema v1 / v0 の両方を受理し得る（dispatch が v1 を優先）。
- `.hako` mirbuilder の出力形式は **MIR JSON v0 に固定**する（Decision）。
  - SSOT: `docs/development/current/main/20-Decisions.md`
  - 理由: v0 loader（`src/runner/mir_json_v0.rs`）で検証距離を短くし、Phase-0 の揺れを消す。

### 2) .hako mirbuilder I/O contract (pin SSOT)

#### Input

- Stage-B が出す Program(JSON v0)（既存）
- 入力経路は “Stage-B entry を通す” を前提にする（selfhost bootstrap route に従う）

#### Program(JSON v0) — shape (Phase-2 minimal vocab SSOT)

以下は `--emit-program-json-v0` が実際に出す形（`.hako mirbuilder` の Phase-2 受理形）:

- Top-level:
  - `{"kind":"Program","version":0,"body":[ ... ]}`
- Object key ordering (contract):
  - **`Int` 以外**の stmt/expr object は `"type":"..."` が **その object 内で最後**に来る（Phase-2 の string-scan 実装が前提にしている）。
  - `Int` は `{"type":"Int","value":<i64>}`（`type` → `value` の順）。

Examples (each node):

- `Local{name, init}` (init is stored in `expr`):
  - `{"expr":{"type":"Int","value":0},"name":"x","type":"Local"}`
- `Assignment{name, expr}`:
  - `{"expr":{"lhs":{"name":"x","type":"Var"},"op":"+","rhs":{"type":"Int","value":1},"type":"Binary"},"name":"x","type":"Assignment"}`
- `Print{expr}`:
  - `{"expr":{"type":"Int","value":0},"type":"Print"}`
- `Return{expr}`:
  - `{"expr":{"type":"Int","value":0},"type":"Return"}`
- `Binary{op:"+", lhs, rhs}`:
  - `{"lhs":{"name":"x","type":"Var"},"op":"+","rhs":{"type":"Int","value":1},"type":"Binary"}`
- `Var{name}`:
  - `{"name":"x","type":"Var"}`

#### Output

- MIR JSON（`--mir-json-file` で直接実行可能な形）
- “中間成果物としての MIR JSON” を必ずファイルに落とし、再実行できること

#### Fail-fast (required)

- `.hako` mirbuilder 側の失敗は必ず `Err`（or Error JSON）で止め、先頭タグを統一する:
  - `[freeze:contract][hako_mirbuilder] ...`
- silent fallback 禁止（空の MIR を出す / 既存経路に逃げて成功扱い等は禁止）

### 3) .hako side: placement + responsibility boundary

配置は “責務が迷わない” ことを最優先にする:

- New root: `lang/src/compiler/mirbuilder/`
- README: `lang/src/compiler/mirbuilder/README.md`
  - ここは “Program(JSON v0) → MIR JSON” の変換だけ
  - SSA / rewrite / optimizer / macro 展開はここでやらない（別層の責務）
  - 失敗タグ SSOT: `[freeze:contract][hako_mirbuilder]`

### 4) Minimal fixture: end-to-end pin (Phase-0 acceptance)

Phase-0 の “最小1本” を fast gate に組み込み、stdout を固定する。

Pin fixture (SSOT):

- `apps/tests/phase29bq_blockexpr_basic_min.hako`
  - 最小理由: 必要語彙が `local` + BlockExpr + 単純な式（const/binop）+ `print` に閉じており、if/loop/join/phi/map/cond-prelude を要求しない。

要件:

- `.hako mirbuilder` → MIR JSON を生成
- 生成された MIR JSON を `--mir-json-file` で実行
- stdout が既存 fast gate fixture と一致

運用:

- まず上の pin fixture を必ず通す（ここを “Phase-0 完了の最小証明” とする）
- 追加する fixture/gate は 1コミットで 1本だけ

### 5) Scope expansion (after Phase-0)

対応範囲の拡張は “順序” と “固定方法” を守る:

1) statement/effect（`local` / `assign` / call / `print`）
2) if（join なし shape から）
3) loop（最小 shape）
4) join/phi（最後）

各段階で:

- 受理形は 1つだけ増やす
- fixture + gate を 1つだけ増やす
- 失敗は `[freeze:contract][hako_mirbuilder]` で短距離化する

## Daily commands (Phase-0+)

- `cargo check --bin hakorune`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- MIR JSON の再実行（必須）:
  - `./target/release/hakorune --mir-json-file <OUT.json>`

## Drift checks (pre-implementation hardening)

`.hako` mirbuilder の fail-fast tag が散逸しないこと（先頭が必ず `[freeze:contract][hako_mirbuilder]` で始まる）:

- After `lang/src/compiler/mirbuilder/` が作られた後に実行する:
  - must-have: `rg -n --glob '*.hako' '\\[freeze:contract\\]\\[hako_mirbuilder\\]' lang/src/compiler/mirbuilder`
  - must-not: `rg --pcre2 -n --glob '*.hako' '\\[freeze:contract\\](?!\\[hako_mirbuilder\\])' lang/src/compiler/mirbuilder`
    - expected: 0 hits

`.hako` box を `using` で追加したら、**workspace export を正本にする**（root `hako.toml` / `nyash.toml` の `[modules]` 直追加で通さない）:

- 追加した box の module key は対応する `*/hako_module.toml` の `[exports]` に登録する（例: `lang/src/compiler/hako_module.toml` の `lang.compiler.mirbuilder.*`）
- Stage1 bridge embedded snapshot がその key を必要とする場合は `bash tools/selfhost/refresh_stage1_module_env_snapshot.sh` を同コミットで実行する
- compat alias を root `[modules]` に残す必要がある場合だけ、`tools/checks/module_registry_*_top_only_allowlist.txt` の明示更新つきでレビューする
- drift check（登録漏れ / direct add 検知）:
  - `rg -n "lang/src/compiler/mirbuilder/" lang/src/compiler/hako_module.toml`
  - `bash tools/checks/module_registry_hygiene_guard.sh`

Box naming (SSOT; fail-fast distance reduction):

- Rule: `static box FooBox { ... }` は **必ず** `FooBox` として参照する（Box 名は runtime contract の一部）。
  - `using ... as FooBox` は OK（名前一致）
  - `using ... as Alias` は禁止（alias で Box 名を変えない）
- Incident class this prevents:
  - `.hako` 側の alias が `NewBox Alias` を生成し、VM で `Unknown Box type: Alias` になって落ちる（診断距離が伸びる/再発しやすい）。

Pin precheck (optional; decision):

- Decision (current): do not add an extra “NewBox name precheck” to Phase-1 pin smoke yet.
- Rationale:
  - The failure already happens at the correct boundary (Phase-1 entry execution) with a clear error: `Unknown Box type: <name>`.
  - The structural fix is the Box naming SSOT above; duplicating checks in the smoke script risks drift.
- Revisit when:
  - We see repeated box-name accidents, or
  - The error message becomes noisy/ambiguous and we need a shorter, more targeted failure summary.

Pin smoke の前提（planner-required）:

- 目的: `.hako` 側の loop 形が JoinIR planner によって fail-fast されることを前提にしており、fallback の有無で挙動がブレるとデバッグ距離が伸びる。
- ルール: Phase-0/1/2 の pin smoke は `HAKO_JOINIR_PLANNER_REQUIRED=1` を明示する。
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase0_pin_vm.sh`
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase1_min_vm.sh`
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase2_min_vm.sh`
- pin smoke の入出力は `HAKO_PROGRAM_JSON_FILE` を使う（`HAKO_PROGRAM_JSON="$(cat ...)"` を避けて env 長地雷を消す）

Pin smoke drift check（env長地雷の再発防止）:

- expected: 0 hits
  - `rg -n "HAKO_PROGRAM_JSON=\\$\\(cat" tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase*`

Program(JSON v0) scanner SSOT（Phase-3）:

- Phase-1/2 で見つかった “直スキャンの罠”（例: `read_digits("1},...") == "10"`）を call site から撤去し、Program(JSON v0) の読み取りを 1 箇所に集約する。
- SSOT: `lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako`
