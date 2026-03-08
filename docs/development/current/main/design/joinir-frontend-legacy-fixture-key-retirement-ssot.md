# JoinIR Frontend Legacy Fixture Key Retirement SSOT

Status: Phase E accepted
Date: 2026-03-08
Scope: `src/mir/join_ir/frontend/ast_lowerer/route.rs` の legacy/historical by-name fixture key 互換契約

## Purpose

`route.rs` に残っていた pattern-era の関数名 key を、runtime を壊さずに段階撤去し、semantic key 契約へ収束させる。

Canonical key groups:

### Pattern-era old keys

- `pattern3_if_sum_multi_min`
- `jsonparser_if_sum_min`
- `selfhost_if_sum_p3`
- `selfhost_if_sum_p3_ext`

### Unused selfhost dev keys

- `selfhost_token_scan_p2`
- `selfhost_token_scan_p2_accum`
- `selfhost_args_parse_p2`
- `selfhost_stmt_count_p3`

### Historical semantic keys

- `if_phi_join_multi_min`
- `jsonparser_if_phi_join_min`
- `selfhost_if_phi_join`
- `selfhost_if_phi_join_ext`
- `selfhost_verify_schema_p2`
- `selfhost_detect_format_p3`
- `jsonparser_unescape_string_step2_min`

## Current Contract

- Program JSON frontend は `defs[0].name` を `resolve_function_route()` へ直結する。
- 旧 key は Phase C までは **by-name allowlist 契約**として live だった。
- 現行 mainline の active tests / fast gate はこれらを直接 pin していない。
- Phase B で semantic alias を追加し、Phase C で managed private fixtures/docs を semantic key へ移行した。
- Phase D で pattern-era old key は retire 済み。runtime で受理するのは semantic/current key のみ。
- `selfhost_token_scan_p2` / `selfhost_token_scan_p2_accum` / `selfhost_args_parse_p2` / `selfhost_stmt_count_p3` は repo-local caller 0 の selfhost dev fixture key だったため、semantic alias を足さずに reject lane へ退避した。
- Phase E で normalized-dev historical semantic key (`if_phi_join_*`, `selfhost_verify_schema_p2`, `selfhost_detect_format_p3`, `jsonparser_unescape_string_step2_min`) も retire 済み。
- current runtime は active Program JSON frontend entrypoint だけを受理し、historical fixture key は retirement ledger / `CURRENT_TASK` / archive/history / explicit rejection test に限定する。

### Current live key buckets (2026-03-08 audit)

| Bucket | Keys | Current status | Retirement rule |
| --- | --- | --- | --- |
| current runtime keep | `test`, `local`, `_read_value_from_pair`, `simple` | repo-local current caller / current runtime examplesあり | keep |
| retired Program JSON compat key | `filter`, `print_tokens`, `map`, `reduce`, `fold` | repo-local Program JSON caller 0 / explicit reject test added | retired from `route.rs`; AST/frontend route-family support stays elsewhere |
| retired historical docs/private-only key | `jsonparser_skip_ws_mini`, `jsonparser_skip_ws_real`, `jsonparser_atoi_mini`, `jsonparser_atoi_real`, `jsonparser_parse_number_real` | repo-local current caller 0 / docs-private historical fixture only | retired from `route.rs`; docs/private replay lane is historical-only |
| current dev key | `nested_if_merge`, `read_quoted` | repo-local current tests / dev fixtures が current lane として利用 | keep while dev-gated route lane is active |
| retired dev-gated compat key | `parse_loop`, `read_quoted_from` | repo-local Program JSON caller 0 / explicit reject test added | retired from `route.rs`; source/app method symbol usage is unrelated |

Audit notes:
- repo-local Program JSON caller audit では `map` / `filter` / `print_tokens` / `reduce` / `fold` の current `.program.json` caller は 0。Phase 29ce で `route.rs` allowlist から削除し、reject test へ移した。
- `jsonparser_*` keys は repo-local current caller 0 を確認できたため、Phase 29ce で `route.rs` から retire 済み。残る caller は `docs/private` historical fixture lane のみ。
- repo-local current tests/dev fixtures は `nested_if_merge` / `read_quoted` を current dev key として使う。
- `parse_loop` は `docs/private/roadmap2/phases/phase-41-if-phi-level3/fixtures/nested_if_merge_simple.program.json` に historical/dev fixture caller が残るが、repo-local current Program JSON caller は 0 のため retire 済み。
- `read_quoted_from` は route token としての repo-local current caller が 0 だったため retire 済み。`apps/**` / `apps/libs/**` の `read_quoted_from` は language/app symbol usage であり、Program JSON route key とは別契約。

Code anchors:
- `src/mir/join_ir/frontend/ast_lowerer/mod.rs`
- `src/mir/join_ir/frontend/ast_lowerer/route.rs`

Pinned assets:
- `docs/private/roadmap2/phases/normalized_dev/fixtures/if_phi_join_multi_min.program.json`
- `docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_if_phi_join_min.program.json`
- `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_phi_join.program.json`
- `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_phi_join_ext.program.json`

## Decision

`rename/delete` を直で行わない。

採用方針:
1. alias-first
2. old key 維持
3. fixture/doc migration を先行
4. 最後に retire

禁止:
- 旧 key の即時 rename
- 旧 key の即時 delete
- Program JSON frontend に新しい by-name special case を増やすこと

## Phase Order

### Phase A: Inventory / Decision Lock

目的:
- 対象 key・依存資産・撤去順を SSOT に固定する

変更対象:
- この SSOT
- `CURRENT_TASK.md`
- 必要なら `route.rs` の comment

受け入れ条件:
- 対象 key が 4 件で固定されている
- `alias-first` 方針が明文化されている
- `rename/delete` 禁止が明文化されている

### Phase B: Add Semantic Aliases

目的:
- semantic key を追加し、旧 key と新 key の両方を受理できるようにする

Alias map:
- pattern-era old keys -> historical semantic keys
  - `pattern3_if_sum_multi_min` -> `if_phi_join_multi_min`
  - `jsonparser_if_sum_min` -> `jsonparser_if_phi_join_min`
  - `selfhost_if_sum_p3` -> `selfhost_if_phi_join`
  - `selfhost_if_sum_p3_ext` -> `selfhost_if_phi_join_ext`

変更対象:
- `src/mir/join_ir/frontend/ast_lowerer/route.rs`
- 必要なら frontend unit test

ルール:
- old key は残す
- new key だけを足す
- runtime behavior は不変

受け入れ条件:
- old/new 両 key が同じ `FunctionRoute` へ解決される
- `cargo check --tests` が緑
- `cargo build --release --bin hakorune` が緑
- `phase29bq_fast_gate_vm.sh --only bq` が緑

### Phase C: Fixture / Doc Migration

目的:
- private/historical JSON fixture と docs を semantic key へ移行する

変更対象:
- private JSON fixture
- private design docs
- 必要なら active pin 1 本

ルール:
- この phase が終わるまで old key は消さない
- `CURRENT_TASK` に migrated/not-migrated を残す

受け入れ条件:
- in-repo の managed fixture/doc 参照が semantic key に寄る
- old key の残りが `route.rs` / retirement ledger / archive/history に縮む

### Phase D: Retire Pattern-Era Old Keys

目的:
- `route.rs` の old key を remove する

前提:
- Phase C 完了
- in-repo fixture/doc で old key 参照が 0
- retire 対象と rollback が明文化済み

受け入れ条件:
- `route.rs` の old key が削除されている
- `rg` で in-repo current assets の old key が 0
- `cargo check --tests` / `cargo build --release --bin hakorune` / `phase29bq_fast_gate_vm.sh --only bq` が緑

### Phase E: Retire Historical Semantic Fixture Keys

目的:
- normalized-dev や historical selfhost prototype 向けに残っていた semantic by-name key を current runtime から外す

対象:
- historical semantic keys

前提:
- repo-local current callers が 0
- active docs/gates が exact by-name key を current contract として使っていない
- `docs/private` 側の remaining references は historical/normalized-dev ledger 扱い

受け入れ条件:
- `route.rs` TABLE から上記 key が削除されている
- explicit rejection test が追加されている
- `cargo check --tests` / `cargo build --release --bin hakorune` / `phase29bq_fast_gate_vm.sh --only bq` が緑

## Drift Checks

Old-key retirement inventory:
```bash
rg -n "pattern3_if_sum_multi_min|jsonparser_if_sum_min|selfhost_if_sum_p3|selfhost_if_sum_p3_ext|selfhost_token_scan_p2|selfhost_token_scan_p2_accum|selfhost_args_parse_p2|selfhost_stmt_count_p3|if_phi_join_multi_min|jsonparser_if_phi_join_min|selfhost_if_phi_join|selfhost_if_phi_join_ext|selfhost_verify_schema_p2|selfhost_detect_format_p3|jsonparser_unescape_string_step2_min" \
  src tests tools docs/development/current/main docs/private CURRENT_TASK.md
```

Historical semantic assets:
```bash
rg -n "if_phi_join_multi_min|jsonparser_if_phi_join_min|selfhost_if_phi_join|selfhost_if_phi_join_ext" \
  docs/private/development/current/main docs/private/roadmap2/phases/normalized_dev/fixtures
```

Runtime contract:
```bash
rg -n "resolve_function_route|lower_program_json" \
  src/mir/join_ir/frontend/ast_lowerer
```

## Recommended Next Step

この removal phase は **closed**。

理由:
- alias 追加、managed asset migration、pattern-era old key retire、historical semantic key retire まで完了した
- runtime の by-name contract は current active Program JSON entrypoint に収束した
- 旧 key/歴史 key は retirement ledger / `CURRENT_TASK` / archive-history / docs/private historical assets の traceability に限定できた
