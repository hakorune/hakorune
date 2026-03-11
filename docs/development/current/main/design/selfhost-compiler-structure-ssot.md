---
Status: SSOT
Scope: selfhost / MIR-direct / de-Rust mainline の compiler structure と ownership を `.hako` / Rust 横断で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - lang/src/compiler/README.md
  - lang/src/compiler/mirbuilder/README.md
  - src/mir/builder/README.md
  - src/runner/json_v0_bridge/README.md
---

# Selfhost Compiler Structure (SSOT)

## Goal

最終目標を `.hako` / Rust 横断で迷わず読めるようにする。

North-star:

`parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM`

この文書は「いま何が authority で、何が compat で、どこが retire target か」を
compiler structure の観点で固定する。

## Reading Order

restart / handoff では次の順に読む。

1. `CURRENT_TASK.md`
   - current blocker / next owner / latest accepted truth
2. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
   - final goal と migration order
3. `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
   - current route authority / compatibility boundaries / acceptance
4. `docs/development/current/main/phases/phase-29ch/README.md`
   - active MIR-direct bootstrap unification slice
5. `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
   - `.hako` / Rust ownership map（この文書）

## Ownership Map

### 1. `.hako` compiler mainline

Primary owner:
- `lang/src/compiler/**`
- `lang/src/mir/**`

Responsibility:
- selfhost compiler の意味決定
- parser / mirbuilder / recipe / lower の mainline
- MIR-direct authority へ寄せるための本命実装

Important entry docs:
- `lang/src/compiler/README.md`
- `lang/src/compiler/mirbuilder/README.md`
- `lang/src/compiler/pipeline_v2/README.md`

Current note:
- final authority は `.hako` mainline に寄せる
- `.hako` 側 workaround で route を増やさない

### 2. `stage1` selfhost authority entry

Primary owner:
- `lang/src/runner/stage1_cli_env.hako`

Responsibility:
- reduced bootstrap の current authority entry
- source-only `emit-mir` authority input を `MirBuilderBox.emit_from_source_v0(...)` へ渡す
- explicit supplied `Program(JSON)` は compat-only input shape として受ける

Must not:
- new authority route を増やさない
- launcher-specific / ad-hoc postprocess を authority path に戻さない

### 3. Rust compiler reference / structural SSOT

Primary owner:
- `src/mir/builder/**`

Responsibility:
- Rust AST -> MIR builder の構造 SSOT
- Context ownership / ValueId / fail-fast 契約の reference implementation
- `.hako` mirbuilder migration の写経元 / cleanliness source

Important entry doc:
- `src/mir/builder/README.md`

Current note:
- current reduced selfhost authority そのものではない
- ただし compiler structure / MIR semantics の重要な reference SSOT である

### 4. Rust bootstrap compatibility boundary

Primary owner:
- `src/runner/json_v0_bridge/**`
- `src/stage1/program_json_v0.rs`
- `src/host_providers/mir_builder.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`

Responsibility:
- bootstrap-only `Program(JSON v0)` boundary
- compiled stage1 artifact が still linked な provider / module dispatch support
- current reduced authority の compat keep を narrow に維持する

Important entry doc:
- `src/runner/json_v0_bridge/README.md`

Must not:
- current authority をここへ戻さない
- generic fallback / silent rescue を増やさない

### 5. Shell orchestration / proof contract

Primary owner:
- `tools/selfhost/build_stage1.sh`
- `tools/selfhost/lib/stage1_contract.sh`
- `tools/selfhost/lib/identity_routes.sh`
- `tools/selfhost_identity_check.sh`
- `tools/selfhost/run_stage1_cli.sh`

Responsibility:
- proof-first bootstrap contract
- route ids / capability probes / G1 identity compare
- authority evidence と compat keep の切り分け

Current note:
- shell は authority の説明責任を持つ
- compiler meaning 自体の owner ではない

## Current Structure Truth (2026-03-11)

### Authority

- active phase: `phase-29ch`
- reduced proof source: `lang/src/runner/stage1_cli_env.hako`
- current authority route:
  - `tools/selfhost/build_stage1.sh`
  - `tools/selfhost/lib/stage1_contract.sh`
  - env-mode `stage1_contract_exec_mode ... emit-mir <entry> <source_text>`
  - `lang/src/runner/stage1_cli_env.hako`
  - `MirBuilderBox.emit_from_source_v0(...)`
  - `tools/ny_mir_builder.sh`

### Compat keep

- explicit supplied `Program(JSON)` input
  - live compat route: `stage1-env-mir-program`
  - current transport: text-only
- cold compat keeps:
  - `stage1-env-mir-legacy`
  - `stage1-subcmd-mir-program`
- raw `run_stage1_cli.sh ... --from-program-json`
  - user-facing file input shape only
  - transport itself is now text-only

### Future retire targets

- `Program(JSON v0)` as current authority
- linked Rust stage1 bridge lane (`src/runner/stage1_bridge/mod.rs` / embedded `stage1_cli.hako`)
- delegate route
- raw/subcmd direct `stage1-cli emit ...` as authority candidate

## Fixed Order

1. keep `stage1-env-mir-source` green as current authority
2. thin supplied `Program(JSON)` compat surface
3. keep `Program(JSON v0)` as bootstrap-only boundary
4. finish MIR-direct bootstrap unification
5. only then cut a separate `Program(JSON v0)` retirement phase

## Non-goals

- de-Rust を “Rust source delete” と混同しない
- bootstrap compat keep を authority へ戻さない
- `.hako` mainline ではなく shell/provider 側へ意味決定を逃がさない
