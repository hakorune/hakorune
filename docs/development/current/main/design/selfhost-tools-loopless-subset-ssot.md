---
Status: SSOT
Scope: Selfhost tooling (`tools/*`) under `NYASH_DISABLE_PLUGINS=1`
---

# Selfhost tooling policy: loopless subset (no general `loop` / `while`)

## Goal

`tools/hako_check/*` などの selfhost ツールを、`NYASH_DISABLE_PLUGINS=1` 環境でも **確実にコンパイル・実行できる**最小サブセットで固定する。

このポリシーは「コンパイラの表現力を削る」ものではなく、**ツール側がコンパイラに汎用ループ実装を要求してしまう**のを防ぎ、CorePlan の“合成（FlowBox）”を小さく保つためのもの。

## Policy (SSOT)

### Stage note

この “loopless subset” は **bringup の段階0** として有効（CorePlan を汎用CFGへ太らせないための安全弁）。

中長期では、CorePlan 側が unknown loop を “標準の合成” として受理できるようになった後に、
tooling も restricted loop（Loop + LeafEffects + ExitIf）へ段階的に戻す余地がある。
その設計SSOTは次を参照:

- `docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md`

### A. Tooling code must be loopless

selfhost ツール（特に `tools/hako_check/cli.hako` とその依存）は、原則として以下を満たす:

- `loop(...)` / `loop(true)` / `while (...)` を使わない
- 反復が必要な場合は **再帰**、または **小さい純関数**の再帰合成で表現する

理由:
- `tools/*` の素朴な一般ループを受理するために CorePlan を“汎用CFG言語”へ肥大化させない
- strict/dev の `flowbox/freeze` による穴埋めサイクルを、言語側（stdlib/アプリ）に集中させる

### B. Allowed alternatives

- 再帰（末尾再帰を推奨）
- 既存の “subset で通る” helper（例: StringUtils 系、既存 Plan/Composer がカバーしているもの）
- 必要なら **小さい helper を追加**して呼び出す（ただし helper 側も loopless）

### C. Exceptions

例外を許可するのは、次のいずれかに限る:

- ツール以外（stdlib/アプリ）の挙動として必要なループで、CorePlan の対象として吸収すべきもの
- 既に CorePlan/Composer が SSOT として吸収済みの loop shape で、strict/dev gate で固定されているもの

## Loopless subset fixtures (SSOT)

`tools/hako_check/*` の loopless subset を固定するために、次の fixtures を対象として扱う。

- `tools/hako_check/cli.hako` (tool entrypoint)
- `tools/hako_check/tests/HC011_dead_methods/`
- `tools/hako_check/tests/HC012_dead_static_box/`
- `tools/hako_check/tests/HC013_duplicate_method/`
- `tools/hako_check/tests/HC013_duplicate_method_edge/`
- `tools/hako_check/tests/HC014_missing_entrypoint/`
- `tools/hako_check/tests/HC014_missing_entrypoint_case2/`
- `tools/hako_check/tests/HC015_arity_mismatch/`
- `tools/hako_check/tests/HC015_arity_mismatch_case_ok/`
- `tools/hako_check/tests/HC016_unused_alias/`
- `tools/hako_check/tests/HC017_non_ascii_quotes/` (gate: skipped)
- `tools/hako_check/tests/HC018_top_level_local/`
- `tools/hako_check/tests/HC019_dead_code/` (deadcode smoke uses `ok.hako`/`ng.hako`)
- `tools/hako_check/tests/HC019_dead_code/ok.hako` (reason: no-false-positive baseline for deadcode smoke)
- `tools/hako_check/tests/HC019_dead_code/ng.hako` (reason: deadcode detection baseline for deadcode smoke)
- `tools/hako_check/tests/HC021_analyzer_io_safety/`
- `tools/hako_check/tests/HC022_stage3_gate/`
- `tools/hako_check/tests/HC031_brace_heuristics/`
- `tools/hako_check/tests/HC032_restricted_loop/`

### Change rules

- 追加は “なぜ loopless subset に必要か” を 1 行で添える
- 削除は “なぜ不要になったか” を 1 行で添える
- HC 番号順で並べる

## Enforcement (recommended)

将来的に以下のどちらかを gate に入れて違反を早期検知する:

- `rg -n "\\b(loop|while)\\b" tools/hako_check` がヒットしたら FAIL
- もしくは専用スクリプト `tools/hako_check_loopless_gate.sh` を追加

## Related

- CorePlan優先ポリシー（stdlib/共通ヘルパー向け）: `docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md`
- unknown loop strategy（CorePlan合成）: `docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md`
