# Docs Layout (SSOT)

Status: SSOT  
Scope: `docs/development/current/` 以下の「置き場所ルール」と、SSOT/履歴メモの混在を防ぐための最小ガイド。

## 目的

- 入口（読む順序）と、詳細（設計図/調査/Phaseログ）を分離して迷子を防ぐ。
- “Phase 文書が増えても” SSOT が埋もれないようにする。
- 大規模移動はしない（リンク切れ回避）。以後の追加分から秩序を作る。

## Current Ownership Contract

- `CURRENT_TASK.md`
  - root restart anchor
  - the only live status pointer
  - current order / current next / canonical owner links only
- `10-Now.md`
  - docs-side thin mirror/dashboard
  - one-screen summary + links only
- `15-Workstream-Map.md`
  - one-screen lane order mirror
- `05-Restart-Quick-Resume.md`
  - fastest reboot path only
- `design/kernel-implementation-phase-plan-ssot.md`
  - canonical rough task-order SSOT
- `design/kernel-replacement-axis-ssot.md`
  - `K-axis` / artifact / task-placement vocabulary owner
- `lang/README.md`
  - source-root / logical-layer placement contract
- `tools/smokes/v2/README.md`
  - smoke profile / suite placement contract
- `main/phases/**`
  - execution detail / blocker history / narrow ledgers

Rule:

- do not let `CURRENT_TASK.md` or `10-Now.md` regrow into landed-history ledgers.
- do not let `05-Restart-Quick-Resume.md` or `15-Workstream-Map.md` regrow into landed-history ledgers either.
- if a block already has a better owner, replace it with a short summary plus a link.
- phase closeout should normally touch:
  - the active phase docs
  - `CURRENT_TASK.md`
  - optionally `10-Now.md` only when `Now/Next/After Next` changes
- `phases/README.md` is an index, not a full chronology.
- archive historical docs per area:
  - `docs/development/current/main/design/archive/`
  - `docs/development/current/main/phases/archive/`
- when physically moving a doc, keep a short stub at the old path with:
  - `Status: Historical`
  - `Moved to: ...`
  - optional pointer to the current owner

## ディレクトリの役割（推奨）

### `docs/development/current/main/`（入口・現状）

ここは「まず読む」入口を置く場所。SSOT を全部ここに置かない。

- 入口（例）: `00-Overview.md`, `01-JoinIR-Selfhost-INDEX.md`, `10-Now.md`, `20-Decisions.md`, `30-Backlog.md`
- `CURRENT_TASK.md` は root の machine-readable anchor、`10-Now.md` は docs 側の薄い mirror/dashboard。

### `docs/development/current/main/design/`（設計図・SSOT寄り）

設計の SSOT / 長期参照の設計図を置く。

- 原則: Phase 依存のログ/作業記録は置かない（それは phases へ）。
- 例: JoinIR の設計、Boundary/ExitLine の契約、Loop パターン空間、runtime/box 解決の地図。
- よく参照する設計SSOT:
  - Join-Explicit CFG Construction（north star）: `docs/development/current/main/design/join-explicit-cfg-construction.md`
  - EdgeCFG Flow Fragments（Structured→CFG lowering SSOT）: `docs/development/current/main/design/edgecfg-fragments.md`

### `docs/development/current/main/design/archive/`（historical design）

歴史化した設計メモ・移行 ledger を置く。

- current owner ではない historical docs を移す。
- 旧パスには short stub を残す。
- curated top からは外すが、traceability は保持する。

### `docs/development/current/main/investigations/`（調査ログ）

不具合調査のログ、切り分け、暫定メモを置く。

- 原則: “結論” は `10-Now.md` / `20-Decisions.md` / 該当 design doc に反映し、調査ログ自体は参照用に残す。
- 原則: 調査ログを SSOT にしない（参照元を明記して“歴史化”できる形にする）。
- よく参照する調査ログ:
  - Phase 259: block-parameterized CFG / ABI/contract 相談パケット: `docs/development/current/main/investigations/phase-259-block-parameterized-cfg-consult.md`

### `docs/development/current/main/phases/`（Phaseログ）

Phase ごとの記録・完了サマリ・実装チェックリストを置く。

- 推奨構造:
  - `docs/development/current/main/phases/phase-131/`
  - `docs/development/current/main/phases/phase-131/131-03-llvm-lowering-inventory.md`
  - `docs/development/current/main/phases/phase-131/131-11-case-c-summary.md`

### `docs/development/current/main/phases/archive/`（historical phase fronts）

closeout / accepted monitor-only / parked / superseded の phase front を置く。

- current active phase front は `phase-*/README.md` に残す。
- archived phase front は `phases/archive/<phase>/README.md` に移す。
- phase 配下の child docs は必要な限り元の場所に残してよい。

### `docs/private/development/current/main/`（private canonical）

公開したくない計画本文・作業メモの正本を置くローカル領域。

- public 側には同名 path の stub を残し、`Private Canonical Path` を明記する。
- machine parse される anchor 文書（`CURRENT_TASK.md` など）は public 側に残す。
- 境界SSOT: `docs/development/current/main/design/private-doc-boundary-migration-ssot.md`

## ドキュメントの種別（ファイル先頭に明記）

追加/更新する文書の先頭に、最低限これを付ける。

```
Status: SSOT | Active | Historical
Scope: ...
Related:
- <入口/SSOT>
```

- `SSOT`: 現行の正本（同じテーマの“別名ファイル”を増やさない）。
- `Active`: 現行だが SSOT ではない（実装の手順書/チェックリスト等）。
- `Historical`: 参照用（当時の調査・ログ）。入口や Now から “歴史” としてリンクする。

## 移行ポリシー（リンク切れ防止）

既存のファイルは大量移動しない。移動が必要な場合は必ず旧パスに“転送スタブ”を残す。

例（旧ファイルの内容を最小化）:

```
# Moved

Status: Historical
Moved to: docs/development/current/main/phases/phase-131/131-03-llvm-lowering-inventory.md
```

## 命名（推奨）

- Phase 文書: `phase-<N>/` + `<N>-<NN>-<topic>.md`（同一フェーズ内で並べ替えが自然）
- 調査ログ: `<topic>-investigation-YYYY-MM-DD.md` など（時系列が分かる形）
- 入口/SSOT: “Phase番号を入れない” ことを基本にする（寿命が長いので）

## 運用の最小ルール

- 新しい Phase 文書は `main/phases/` に入れる（`main/` 直下に増やさない）。
- 設計図（SSOT）は `main/design/` に寄せる（Phase の完了サマリと混ぜない）。
- `10-Now.md` は「現状の要約＋正本リンク」に徹し、詳細ログの本文は抱え込まない。
- `CURRENT_TASK.md` は root anchor なので、重要な blocker / current priority はまずそこへ置く。
- `05-Restart-Quick-Resume.md` は restart 手順と読む順だけに徹し、landed chronicle は抱え込まない。
- `15-Workstream-Map.md` は rough order の one-screen mirror に徹し、phase detail は抱え込まない。
- `phases/README.md` は current / guardrail / recent landed の index に徹し、repo-wide landed ledger を再掲しない。
- historical phase fronts are archived under `docs/development/current/main/phases/archive/`.
- current active phase fronts are linked from `CURRENT_TASK.md` and `15-Workstream-Map.md`.
