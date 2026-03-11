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

## Target End State

de-Rust / selfhost の最終形は、単に Rust source を減らすことではない。
最終的には **compiler も plugin も `.hako` mainline で完結**し、
Rust は host/runtime/backend の最小面に縮退する。

Final target:

- `.hako` mainline
  - parser
  - resolver / using
  - mirbuilder
  - compiler orchestration
  - plugin mainline implementation
- Rust host
  - backend / VM / process / file / env / ABI
  - `.hako` が使う最小 host surface
- compat quarantine
  - bootstrap-only bridge
  - temporary wrappers
  - explicit compat keep

The key rule is:

- language meaning lives in `.hako`
- host capability lives in Rust
- bootstrap debt lives in compat quarantine and remains a retire target

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

### 0. Three-way split (fixed)

Every file in the selfhost lane should be explainable as exactly one of:

1. `.hako` mainline
   - final SSOT for compiler meaning
2. Rust host
   - runtime/ABI/backend support only
3. compat quarantine
   - bootstrap-only keep / future retire target

Do not leave files in an unnamed middle state.

### 1. `.hako` compiler mainline

Primary owner:
- `lang/src/compiler/**`
- `lang/src/mir/**`
- `lang/src/*plugin*` / `.hako` plugin migration targets

Responsibility:
- selfhost compiler の意味決定
- parser / mirbuilder / recipe / lower の mainline
- plugin logic / box behavior の selfhost mainline
- MIR-direct authority へ寄せるための本命実装

Important entry docs:
- `lang/src/compiler/README.md`
- `lang/src/compiler/mirbuilder/README.md`
- `lang/src/compiler/pipeline_v2/README.md`

Current note:
- final authority は `.hako` mainline に寄せる
- `.hako` 側 workaround で route を増やさない
- plugin implementation も最終的にはここへ寄せる
- 新しい言語判断や plugin meaning を Rust 側へ増やさない

### 2. `stage1` selfhost authority entry

Primary owner:
- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli_env.hako::Stage1ProgramJsonCompatBox` (`compat quarantine`, not authority)

Responsibility:
- reduced bootstrap の current authority entry
- source-only `emit-mir` authority input を `MirBuilderBox.emit_from_source_v0(...)` へ渡す
- explicit supplied `Program(JSON)` は compat-only input shape として受ける
- explicit compat MIR call と mixed-input fail-fast gate は `Stage1ProgramJsonCompatBox` へ隔離する

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
- final destination is still `.hako`; Rust here is a reference / migration source, not the end state

### 4. Rust host / runtime minimal surface

Primary owner:
- `src/backend/**`
- `src/runtime/**`
- `src/host_providers/**`
- `crates/nyash_kernel/**`

Responsibility:
- backend / VM / runtime execution
- file/env/process/plugin ABI
- `.hako` mainline から呼ばれる最小 host capability

Must not:
- new compiler meaning を持たない
- parser / mirbuilder / plugin semantics の authority に戻らない
- selfhost progress を理由に ad-hoc language logic を増やさない

Current note:
- plugin host / ABI / runtime handle surface はまだ Rust owner
- ただし plugin behavior 本体は `.hako` side へ寄せるのが end state

### 5. Rust bootstrap compatibility boundary

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

### 6. Shell orchestration / proof contract

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

## Plugin Migration Rule

Plugin migration is part of the same de-Rust direction.

Target:
- plugin behavior / box semantics live in `.hako`
- Rust keeps only host/ABI/runtime glue needed to execute them

Current rule:
- new plugin semantics should not be added to Rust unless they are strictly host-only
- when a plugin-related feature is still Rust-owned, label it as either:
  - `Rust host minimal surface`
  - `compat quarantine`

Do not leave plugin ownership ambiguous.

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
  - explicit compat route: `stage1-env-mir-program`
  - current transport: text-only
  - quarantined `.hako` owner:
    - `lang/src/runner/stage1_cli_env.hako::Stage1ProgramJsonCompatBox`
  - current callers:
    - `tools/selfhost/run_stage1_cli.sh --from-program-json`
    - `tools/dev/phase29ch_program_json_compat_route_probe.sh`
    - `tools/dev/phase29ch_program_json_text_only_probe.sh`
- alternate supplied-Program caller shapes
  - diagnostics-only aliases over `stage1-env-mir-program`
- raw `run_stage1_cli.sh ... --from-program-json`
  - user-facing file input shape only
  - sugar over `stage1-env-mir-program`

### Future retire targets

- `Program(JSON v0)` as current authority
- linked Rust stage1 bridge lane (`src/runner/stage1_bridge/mod.rs` / embedded `stage1_cli.hako`)
- delegate route
- raw/subcmd direct `stage1-cli emit ...` as authority candidate

### Long-term convergence

- `.hako` mainline grows toward full compiler + plugin ownership
- Rust host shrinks toward runtime/backend/ABI only
- compat quarantine shrinks monotonically and must not gain new authority

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
