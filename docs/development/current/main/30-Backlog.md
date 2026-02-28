# Self Current Task — Backlog (main)

Status: Public Stub
Private Canonical Path: `docs/private/development/current/main/30-Backlog.md`

## Purpose

- Public repo には次候補の短い公開サマリだけを置く。
- 実際の backlog 本文（長文・運用メモ）は private canonical で管理する。

## Public Backlog Summary

- Cargo-less CI lane（future, binary-only route, cost-aware）
- Selfhost / de-rust stabilization follow-ups
- portability preflight の運用最適化
- Python AOT / HybridPy AOT（Py2MIR subset, PEP523 integration）は research backlog（not active）
- Translation Validation（Alive2系）と ReproBuild/Provenance は research backlog（not active）
- IntrinsicForge / StringView-Rope / WasmMIR 追加拡張は de-rust mainline 完了後の候補（not active）

## Migration Rule

- 新規 backlog は private canonical に先に追加する。
- public 側には必要最小限の見出しと summary だけを同期する。
- backlog 項目を実装開始する時は docs-first で `CURRENT_TASK.md` の blocker/lane に昇格してから着手する（直接実装開始しない）。
- optimization annotation 系は `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md` の Activation Rule を満たすまで `not active` 固定。
