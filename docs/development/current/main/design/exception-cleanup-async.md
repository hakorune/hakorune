# Catch / Cleanup / Async — Join-Explicit CFG extensions

Status: Draft（design SSOT candidate）  
Last updated: 2025-12-20

Related:
- North star: `docs/development/current/main/design/join-explicit-cfg-construction.md`
- Phase 260 roadmap: `docs/development/current/main/phases/phase-260/README.md`
- Pre-selfhost async stabilization (VM+LLVM; existing `nowait/await`): `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

Note (pre-selfhost):
- This document is the **long-term** design: `await` should not remain as a CFG vocabulary after AsyncLowerBox (state-machine lowering).
- Today, Nyash already has surface `nowait/await` and MIR `FutureNew/FutureSet/Await` in the pipeline; pre-selfhost work focuses on making VM+LLVM behavior consistent first.
- Do not use this doc as a reason to introduce ad-hoc async behavior; follow the pre-selfhost SSOT for Phase‑0 semantics.

## 目的

Nyash/Hakorune の表面文法（主に postfix `catch/cleanup`）に合わせて、例外/後始末/中断（async）を追加するときに JoinIR→MIR の暗黙ABI（推測/メタ/例外的分岐）を再増殖させないための設計メモ。

注: `try { ... }` は言語資料上は legacy/非推奨として扱われることがあるが、この設計は **`try` の存在を前提にしない**（catch/cleanup を正規化の入口にする）。

ポイントは 2 つだけ:

1. **制御フローは edge を明示し、値は edge-args（block params）で運ぶ**
2. **“意味SSOT” と “配線SSOT” を分離し、Fail-Fast の verify を常設する**

## 実装タイミング（推奨）

前提（Phase 260 で固める）:

- MIR で edge-args が terminator operand にあり、`BasicBlock.jump_args` に依存しない（併存→移行→削除の P2 到達が理想）
- “読む側” の参照点が `out_edges()`/`edge_args_to(target)` に一本化されている（Branch 含む）
- “書く側” の terminator 設定が API で一元化されている（successors キャッシュ同期漏れを構造で潰している）
- DCE/verify/printer が terminator operand を SSOT として扱う（メタ追いが不要）

順序（迷子が減る順）:

1. `catch/cleanup`（例外）: `Invoke(ok_edge, err_edge)` を追加（例外 edge を明示）
2. `cleanup/defer`（後始末）: “脱出 edge 正規化” を追加（Return/Throw/Break/Continue を cleanup に寄せる）
3. `async/await`: CFG 語彙に混ぜず **state-machine lowering**（AsyncLowerBox）で分離

## 用語（この文書の範囲）

- **edge-args**: branch/jump の edge に紐づく引数。ターゲット block の params と 1:1 で対応する。
- **Invoke**: 正常継続（ok）と例外継続（err）を持つ呼び出し terminator。
- **cleanup normalizer**: cleanup/defer を実現するために「スコープ外へ出る edge」を cleanup ブロックに集約する正規化箱。
- **async lowering**: `await` を state machine に落としてから CFG（MIR）にする箱。

## catch（最小語彙：例外 edge）

### 目標

- 例外経路を “暗黙” にせず、CFG の edge として明示する。
- 例外値は catch block の params（edge-args）で受ける。

### 最小追加語彙（案）

- `MirTerminator::Invoke { callee, args, ok: (bb_ok, ok_args), err: (bb_err, err_args) }`

設計ノート:

- ok/err 両方が必須（片側欠落を許さない）
- ok/err の args は “役割付きABI” で解釈する（将来 `JoinAbi`/`ContSigId` へ）

### throw の扱い（最小）

MIR で `Throw` を増やさずに済む形:

- 正規化（Normalizer）が “現在の例外継続” を知っており、`throw e` を `Jump(unwind_bb, [e, ...])` に正規化する

### verify（Fail-Fast）

- `Invoke` は terminator（block の最後）であること
- ok/err のターゲット block params と args の数が一致すること
- err 側の先頭 param は例外値（role=Exception）であること（最低限の役割固定）
- “may_throw な呼び出し” を `Call` で表していないこと（暫定: 当面は全部 Invoke に倒しても良い）

## cleanup（脱出 edge 正規化）

### 目標（finally の後継としての cleanup）

- return/break/continue/throw 等の “脱出” を cleanup 経由に統一して、後始末漏れを構造で潰す。
- 例外/return の payload は edge-args（block params）で運ぶ（PHI/メタに逃げない）。

### 最小形（案）

スコープ S ごとに次の 2 ブロック（または 1 ブロック + dispatch）を作る:

- `cleanup_entry_S(tag, payload..., carriers...)`
- `cleanup_dispatch_S(tag, payload..., carriers...)`

`ExitTag`（例）:

- `Return`
- `Throw`
- `Break`
- `Continue`
- `Cancel`（async の drop/cancel 用に予約）

Note:
- `ExitTag::Cancel` is reserved only in the current tree.
- pre-selfhost VM `await` now exposes a runtime cancellation path as `Cancelled(reason)` for scope-owned futures.
- pre-selfhost VM `await` still does not expose a timeout payload.
- pre-selfhost VM futures may now expose `TaskFailed(error)` as a failed terminal state, but that is distinct from `Cancel`.
- current pre-selfhost cancellation only covers scope-owned futures and surfaces as `Cancelled(reason)`; it is not yet cleanup/state-machine `Cancel`.
- detached-task policy and the implicit root-scope policy are pinned in the pre-selfhost async SSOT, not in this long-term lowering document.
- current sibling-failure policy is also pinned in the pre-selfhost async SSOT; it is a future-owner rule, not cleanup/state-machine unwind.

### verify（Fail-Fast）

- S の内部ブロックから “S の外” への edge が存在したら落とす（例外: cleanup_dispatch のみ）
- `Invoke.err` など “例外 edge” も漏れなく cleanup に寄せられていること
- `ExitTag` の分岐が未処理になっていないこと（Unknown は即死）

## async/await（state machine lowering）

### 目標

- `await` を CFG 語彙に混ぜず、AsyncLowerBox が責務として消す（残ったら verify で即死）。
- cancel/drop が必要なら `ExitTag::Cancel` と cleanup を接続して後始末を一貫化する。

### 最小インターフェース（案）

- `await` は “前段IR（AsyncPrep）” にのみ存在してよい
- AsyncLowerBox で state machine 化した後、MIR は `Jump/Branch/Return/Invoke/Call` の語彙だけにする

### verify（Fail-Fast）

- AsyncLowerBox 後に `await` が 1 つでも残っていたら落とす
- state dispatch が全 state をカバーしていること（未到達 state は削除可）
