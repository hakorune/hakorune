# Phase 86–90 Summary — Loop Frontends (dev-only fixtures)

目的: 実アプリ由来のループ形を「fixture + shape guard + fail-fast」で段階的に JoinIR frontend に取り込み、
Normalized-dev の回帰テストで固定する。

このファイルは Phase 86–90 の“ループ前線”だけを 1 枚に集約するサマリ。
詳細ログや設計の背景は各 Phase 文書に委譲し、このサマリでは **到達点 / SSOT / fixture / 未検証**だけを書く。

## SSOT（参照の優先順位）

- JoinIR 全体SSOT: `docs/development/current/main/joinir-architecture-overview.md`
- いまの状態: `docs/development/current/main/10-Now.md`
- タスク優先度: `CURRENT_TASK.md`

## Phase 86 — Carrier Init Builder + Error Tags ✅

- 目的: ValueId 生成とエラー語彙を SSOT 化し、段階移行ラインの土台を固める
- SSOT modules:
  - `src/mir/builder/control_flow/joinir/merge/carrier_init_builder.rs`
  - `src/mir/join_ir/lowering/error_tags.rs`

## Phase 87 — LLVM exe line SSOT ✅

- 目的: `.hako → executable` の手順を `tools/build_llvm.sh` に統一し、Smoke を 1 本に固定する
- SSOT:
  - `tools/build_llvm.sh`
  - `docs/development/current/main/phase87-selfhost-llvm-exe-line.md`

## Phase 88 — continue + 可変ステップ（dev-only fixture）✅

- 目的: `continue` 分岐で `i` が可変ステップ更新される形（`i = i + const`）を段階拡張し、回帰を固定する
- 追加: continue 分岐側での carrier 更新（例: `acc`）を許可
- Fail-Fast: const 以外の step 更新は拒否
- Fixture:
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_unescape_string_step2_min.program.json`

## Phase 89 — ContinueReturn（detector + lowering）✅

- 目的: `continue + early return`（loop 内 return）を Pattern4 と誤認しないように shape を分離し、frontend lowering を追加する
- Shape guard:
  - Pattern4 detector を厳格化（誤爆防止）
  - ContinueReturn 用 detector を追加（dev-only）
- Fixtures:
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/pattern_continue_return_min.program.json`
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/continue_return_multi_min.program.json`（同一値の複数 return-if）

## Phase 90 — ParseStringComposite（dev-only fixture）✅

- 目的: `_parse_string` の制御骨格（escape continue + close-quote return）を “制御だけ抽出” した合成 fixture として固定する
- Fixture:
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/parse_string_composite_min.program.json`
- 追加（実ループ寄せの土台、制御抽出）:
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/parse_array_min.program.json`
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/parse_object_min.program.json`

## Refactor（Phase 89–90 の維持性向上）

- Fixture 名・パス・ルーティングの SSOT:
  - `src/mir/join_ir/normalized/dev_fixtures.rs`
- Shape detector の共通化（Inspector 等）は `shape_guard.rs` を参照

## 未検証（SSOT にしない）

- 実コード（`tools/hako_shared/json_parser.hako`）の `_parse_string/_parse_array/_parse_object` を、
  JoinIR frontend で “そのまま” E2E 実行するライン（dev-only での段階投入）
- 文字列・配列・マップなど Box の意味論を含む大域 E2E（fixture は制御抽出が主目的）
