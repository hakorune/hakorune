# Phase 29ai P2: SSOT Registry + Freeze Taxonomy (docs-only) — Instructions

Status: Ready for execution  
Scope: docs-first（仕様不変）

## Goal

Phase 29ai の “single-planner” を長期で壊れない形にするために、SSOT（真実の所在）と Freeze（Fail-Fast）の分類規約を
1枚に固定する。

## Non-goals

- コード変更（docs-only）
- 既存エラー文言の変更
- 新しいトグル/環境変数の追加

## Deliverables

1) SSOT Registry（真実の所在の表）を追加
   - 新規: `docs/development/current/main/design/planfrag-ssot-registry.md`
   - 最低限含める項目（列）:
     - Layer（Facts / Normalize / Planner / Plan / Emit / Frag）
     - SSOT（真実）: どのデータが唯一の根拠か
     - Forbidden（禁止）: その層が“覗いてはいけない”もの（再解析/再推論など）
     - Verification（検証）: 破れたらどこで Fail-Fast するか（strict/dev の扱いも）

2) Freeze taxonomy（分類）を SSOT 化
   - 新規: `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
   - 最低限のタグ（推奨）:
     - `plan/freeze:contract`（契約違反・形が崩れている）
     - `plan/freeze:ambiguous`（複数解釈で一意化できない）
     - `plan/freeze:unsupported`（対象だが未対応、将来対応予定）
     - `plan/freeze:bug`（不変条件が壊れている/到達してはいけない状態）
   - `Ok(None)` と `Err(Freeze)` の境界を例つきで固定する

3) Phase 29ai README を更新（P2 のリンク追加）

## References (SSOT)

- Plan/Frag の設計入口: `docs/development/current/main/design/edgecfg-fragments.md`
- Pattern6/7 契約: `docs/development/current/main/design/pattern6-7-contracts.md`
- Phase 29ai 入口: `docs/development/current/main/phases/phase-29ai/README.md`
- Phase 29ai P1（Freeze/候補集合のコード側規約）: `docs/development/current/main/phases/phase-29ai/P1-PLANNER-CANDIDATES-FREEZE-SSOT-INSTRUCTIONS.md`

## Acceptance Criteria

- docs-only 変更であること（ビルド不要だが、`./tools/smokes/v2/run.sh --profile quick` が緑を維持）
- 新規2文書が “入口SSOTとして参照される前提” で読める（表 + 例 + 禁止事項が明確）

