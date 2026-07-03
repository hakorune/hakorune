---
Status: SSOT
Date: 2026-07-03
Scope: Hakorune implementation naming, stage-term disambiguation, and
  `nyash` -> `hakorune` rename task order.
Related:
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/tools/check-scripts-index.md
  - tools/checks/naming_charter_guard.sh
---

# Hakorune Naming And Rename Task Order

## Purpose

This SSOT stops new naming drift before starting broad rename work.

The current problem is not one typo. It is three overlapping vocabularies:

1. implementation identity: Rust-built compiler vs `.hako` compiler;
2. execution route / runner phase / lowering pass wording;
3. legacy `nyash` package, binary, crate, environment, and plugin names.

Do not solve this with one broad search-and-replace commit. Rename work must be
split by surface and keep compatibility aliases until the selected surface has a
green gate.

## Naming Charter

### Product And Implementations

```text
Hakorune:
  product / language / user-visible project name

RHako:
  Rust implementation of Hakorune
  owns src/, bootstrap reference behavior, compatibility materializers, and
  Rust-side developer tooling while they remain active

HHako:
  `.hako` implementation of Hakorune
  owns lang/src/compiler/ and self-hosted compiler authority when adopted
```

Rules:

- `RHako` and `HHako` are implementation names, not execution route names.
- Do not use `RHako` / `HHako` to mean a runner, backend, gate, adoption state,
  or converter output.
- `Hakorune` remains the product name even while legacy crates or binaries still
  contain `nyash`.

### Stage Term Reservation

The naked term `stage` is reserved for future bootstrap numbering only.

Do not introduce new unqualified names such as:

```text
Stage-A
Stage-B
Stage0
Stage1
stage1
stage2
stage3
```

unless the name is historical compatibility, an existing path, or a bootstrap
artifact explicitly covered by this SSOT.

Use layer-specific words instead:

```text
compiler mode:
  mode-A / mode-B

runner phase:
  phase-1 / phase-2

lowering or MIR pass:
  lower-resolver / program-json-lowerer / mir-lower-pass

frontend syntax level:
  syntax-3

bootstrap sequence:
  boot-0 / boot-1 / boot-2
```

One-line rule:

```text
Qualify by layer. Naked "stage" is forbidden for new names.
```

If a new term reuses an existing noun in a different layer, reject it and add a
layer prefix.

### Pipeline Names

Keep these names separate:

```text
run-pipeline:
  execute `.hako`

converter:
  create `.hako` from Rust or another source

adoption-plan:
  decide which family becomes HHako authority
```

Do not call the converter a selfhost gate. Do not call a run-pipeline an
adoption decision. Do not call adoption-plan output a converter success.

## Nyash To Hakorune Rename Policy

`nyash` is legacy compatibility vocabulary. New user-facing work should prefer
`hakorune`, but existing internals must be migrated by surface.

Allowed current legacy surfaces:

```text
nyash binary:
  compatibility alias while `hakorune` is primary

nyash-rust crate/package:
  legacy package identity until a Cargo/package rename slice is selected

NYASH_* environment variables:
  compatibility variables until HAKORUNE_* aliases are introduced through
  src/config/env and documented

nyash.toml:
  compatibility config filename; hako.toml is preferred

nyash plugin/crate/path names:
  compatibility names until plugin ABI and packaging rename inventory lands

nyash.* helper symbols:
  ABI/helper compatibility surface; rename requires backend and ABI inventory
```

Forbidden:

- new docs that present `nyash` as the primary product name;
- new Rust code that reads new `NYASH_*` variables directly outside
  `src/config/env`;
- direct deletion of `nyash` aliases without a compatibility gate;
- mixing package rename, env rename, plugin rename, and ABI helper rename in one
  commit.

## Task Order

### NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001

Status: active in this slice.

Scope:

- define `RHako` and `HHako`;
- reserve naked `stage` for bootstrap only;
- distinguish run-pipeline, converter, and adoption-plan;
- add a lightweight guard that checks this SSOT is present and indexed.

Non-goals:

- no source tree rename;
- no binary rename;
- no environment variable rename;
- no historical doc rewrite.

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/checks/dev_gate.sh quick
```

### NYASH-TO-HAKORUNE-RENAME-ROADMAP-001

Status: defined, not implementation.

Purpose: cut the broad rename into safe surfaces.

Subtasks:

```text
1. HAKORUNE-USER-FACING-DOCS-CANONICALIZATION-001
   Make primary user docs say Hakorune first and nyash only as compatibility.

2. HAKORUNE-BINARY-PRIMARY-CUTOVER-INVENTORY-001
   Verify `hakorune` is the primary binary and `nyash` is an alias.

3. HAKO-TOML-CONFIG-CANONICALIZATION-001
   Keep hako.toml primary and nyash.toml compatibility fallback.

4. HAKORUNE-ENV-ALIAS-INVENTORY-001
   Introduce HAKORUNE_* aliases through src/config/env only; keep NYASH_*
   compatibility until deprecation gates are green.

5. HAKORUNE-CARGO-PACKAGE-RENAME-INVENTORY-001
   Inventory Cargo package/crate names and decide which can be renamed without
   breaking downstream scripts, features, or workspace package references.

6. HAKORUNE-PLUGIN-PATH-RENAME-INVENTORY-001
   Inventory plugin package/path names and ABI helper names. Do not rename ABI
   symbols by text replacement.

7. HAKORUNE-ABI-HELPER-VOCABULARY-DECISION-001
   Decide whether `nyash.*` helper symbols remain ABI compatibility forever or
   get versioned aliases.
```

Each subtask must be its own commit or short series. Do not mix them.

### STAGE-TERM-SELFHOST-SMOKE-COMMENT-WORDING-001

Status: landed smoke comment cut.

Scope:

- update active selfhost smoke comments and human-facing diagnostics from
  unqualified `Stage-B`, `Stage1`, and `Stage-3` wording to `mode-B
  compatibility`, `phase-1 compatibility`, and `syntax-3`;
- keep compatibility script names, fixture names, exact expected stderr, and
  `StageB*` / `Stage1UsingResolverBox` names unchanged;
- add naming guard checks so the old comment/diagnostic phrases do not return.

Contract:

```text
comments/diagnostics:
  describe the layer explicitly as mode-B, phase-1, or syntax-3

compatibility surfaces:
  file names, Box names, route tokens, and expected stderr remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-CHECK-SCRIPTS-INDEX-WORDING-001

Status: landed docs index cut.

Scope:

- update `docs/tools/check-scripts-index.md` descriptions for guard rows whose
  owning scripts already migrated to mode-B, phase-1, or concrete
  GlobalCallTarget wording;
- keep guard script names unchanged because they remain compatibility surfaces;
- do not attempt a full historical index rewrite in this slice.

Contract:

```text
active migrated guard descriptions:
  use mode-B compatibility / phase-1 compatibility / GlobalCallTarget wording

compatibility surfaces:
  stage0/stage1/stageb script names remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-SYNTAX3-RUST-ENV-COMMENT-WORDING-001

Status: landed Rust env comment cut.

Scope:

- update Rust environment flag comments to describe the parser surface as
  `syntax-3`;
- keep existing `stage3` feature tokens, env variable names, and helper
  function names unchanged as compatibility surfaces;
- document `--syntax-3` as the selfhost child flag while preserving `--stage3`
  as compatibility alias.

Contract:

```text
comments:
  say syntax-3 for the parser surface

compatibility surfaces:
  stage3 feature tokens and env/function names remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-HHAKO-PARSER-BUILD-COMMENT-WORDING-001

Status: landed HHako parser/build comment cut.

Scope:

- update HHako parser/build/MIR builder comments and local README wording from
  `Stage-A` / `Stage-B` / `Stage-3` to `mode-A compatibility`,
  `mode-B compatibility`, or `syntax-3`;
- keep `stage3` fields/functions, trace strings, file names, Box names, and
  route tokens unchanged;
- do not touch PHI / LocalSSA / variable-map implementation internals.

Contract:

```text
comments:
  use layer-qualified wording

compatibility surfaces:
  stage3 implementation fields and route/file names remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-JSON-V0-BRIDGE-COMMENT-WORDING-001

Status: landed JSON v0 bridge comment cut.

Scope:

- update Rust JSON v0 bridge comments and one local freeze diagnostic to
  `mode-B compatibility` / `bootstrap` wording;
- keep `try_lower_stageb_*` function names, route symbols, and JSON v0 bridge
  behavior unchanged;
- do not touch lowering semantics beyond the diagnostic string.

Contract:

```text
comments/diagnostics:
  use mode-B compatibility or bootstrap wording

compatibility surfaces:
  stageb function names and route symbols remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-HHAKO-BUILD-TEST-COMMENT-WORDING-001

Status: landed HHako build/test comment cut.

Scope:

- update remaining HHako build/test comments from `Stage0` / `Stage-B` wording
  to `bootstrap` or `mode-B compatibility`;
- keep test file names and `StageBFuncScannerBox` compatibility Box names
  unchanged;
- do not change test behavior.

Contract:

```text
comments:
  use bootstrap or mode-B compatibility wording

compatibility surfaces:
  stageb test file names and StageB* Box names remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-APP-BINARY-ONLY-SMOKE-COMMENT-WORDING-001

Status: landed app binary-only smoke comment cut.

Scope:

- update the app-level binary-only selfhost readiness smoke comments from
  unqualified `stage1` / `Stage1` / `Stage2` wording to phase-1 / phase-2
  proxy wording;
- keep pass names and `stage1.mir` / `stage2.mir` artifact filenames
  unchanged;
- do not change binary-only readiness behavior.

Contract:

```text
comments:
  use phase-1 / phase-2 proxy wording

compatibility surfaces:
  stage1.mir and stage2.mir artifact filenames remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-APP-SMOKE-PHASE-COMMENT-WORDING-001

Status: landed app smoke phase comment cut.

Scope:

- update remaining app smoke comments that used unqualified `stage1`,
  `Stage1`, or `stage-a-compat` wording to phase-1 / mode-A compatibility
  wording;
- keep `stage1-cli` log tags, `stage-a-compat` runtime-mode tokens,
  `stage3` feature/env tokens, and artifact filenames unchanged;
- do not change smoke behavior.

Contract:

```text
comments:
  use phase-1 or mode-A compatibility wording

compatibility surfaces:
  stage1-cli, stage-a-compat, and stage3 tokens remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-STAGE0-CAPTURE-COMMENT-WORDING-001

Status: landed Stage0-named capture comment cut.

Scope:

- update Stage0-named capture helper comments to bootstrap capture wording;
- keep `stage0_capture*` file names, `build_stage0_*` function names, and
  existing tests unchanged;
- do not change capture behavior.

Contract:

```text
comments:
  use bootstrap capture wording

compatibility surfaces:
  stage0_capture file/function/test names remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-PIPELINE-V2-COMMENT-WORDING-001

Status: landed Pipeline V2 comment cut.

Scope:

- update Pipeline V2 comments and README wording to phase-1 / phase-2 /
  phase-3, syntax-3, or mode-B compatibility wording;
- keep `Stage1*` Box names, `stage1_*` file/module names, `lower_stage1_*`
  APIs, and `stage3_flag` compatibility fields unchanged;
- do not change Pipeline V2 behavior.

Contract:

```text
comments:
  use phase-1 / phase-2 / phase-3, syntax-3, or mode-B compatibility wording

compatibility surfaces:
  Stage1* Box names, stage1_* files/modules, lower_stage1_* APIs, and
  stage3_flag remain unchanged
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-ENV-ALIAS-INVENTORY-001

Status: landed foundation.

Scope:

- add common alias helper entrypoints in `src/config/env.rs`;
- define primary-wins behavior for future `HAKORUNE_*` / `HAKO_*` variables
  with `NYASH_*` compatibility aliases;
- keep existing `NYASH_*` variables untouched until each subsystem gets its own
  inventory;
- do not introduce new environment variables in this slice.

Contract:

```text
primary env var set:
  use primary value

primary unset and alias set:
  use alias value
  emit one deprecation warning through warn_alias_once(alias, primary)

both unset:
  return the helper default / None / false
```

Acceptance:

```bash
cargo test -q --lib env_alias_helpers
bash tools/checks/naming_charter_guard.sh
tools/checks/dev_gate.sh quick
```

### HAKORUNE-ENV-ALIAS-FIRST-CUT-001

Status: landed first cut.

Scope:

- move the already-documented `HAKO_ROOT` / `NYASH_ROOT` and `HAKO_BIN` /
  `NYASH_BIN` compatibility pairs through the common env alias helpers;
- keep `HAKO_*` as the preferred spelling and `NYASH_*` as compatibility alias;
- preserve existing trimmed-string behavior, where empty or whitespace-only
  values are treated as unset;
- do not rename package names, binaries, plugin paths, or ABI helper symbols.

Contract:

```text
HAKO_ROOT / HAKO_BIN set to non-empty:
  use preferred value

preferred value empty or unset and NYASH_ROOT / NYASH_BIN non-empty:
  use compatibility alias
  emit warn_alias_once(alias, preferred)

both values empty or unset:
  return None
```

Acceptance:

```bash
cargo test -q --lib hako_root
cargo test -q --lib hako_bin
cargo test -q --lib env_alias_helpers
bash tools/checks/naming_charter_guard.sh
tools/checks/dev_gate.sh quick
```

### HAKORUNE-USER-FACING-DOCS-CANONICALIZATION-001

Status: landed root README cut.

Scope:

- make the root `README.md` present Hakorune as the primary product and binary
  spelling;
- use `target/release/hakorune` or `$HAKO_BIN` in user-facing command examples;
- keep `nyash`, `NYASH_*`, `ny-llvmc`, and `nyash.toml` where they are
  compatibility, ABI, package, crate, historical, or existing-tool names;
- do not rename binaries, packages, paths, syntax highlighting tags, ABI helper
  symbols, or historical sections.

Contract:

```text
root README top note:
  prefer target/release/hakorune or $HAKO_BIN
  state $NYASH_BIN remains compatibility alias

primary command examples:
  use target/release/hakorune

compat / internal names:
  may remain when explicitly scoped as compatibility, historical, package,
  crate, ABI, env, or tool names
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/checks/dev_gate.sh quick
```

### HAKORUNE-README-MODEB-USER-FACING-WORDING-001

Status: landed.

Scope:

```text
README.md:
  current developer quickstart MIR emit route says mode-B compatibility
```

Non-claims:

```text
syntax highlighting fences renamed = 0
NYASH_* env names renamed = 0
ny-llvmc / nyash-llvm-compiler tool names renamed = 0
historical Nyash Era section rewritten = 0
runtime behavior changed = 0
```

### HAKORUNE-README-MODEB-LINE-QUICKGUIDE-WORDING-001

Status: landed.

Scope:

```text
README.md:
  ny-llvm line quickstart label says mode-B compatibility
```

Non-claims:

```text
selfhost_exe_stageb_quick_guide.md path renamed = 0
ny-llvmc tool name renamed = 0
historical PyVM stage2 smoke path renamed = 0
runtime behavior changed = 0
```

### HAKORUNE-REFERENCE-DOCS-CANONICALIZATION-DECISION-001

Status: landed.

Decision:

```text
Reference docs should be Hakorune-first for current product/language wording.
Legacy Nyash terms remain allowed only when they name compatibility binaries,
NYASH_* environment variables, nyash.toml compatibility config, ny-llvmc /
nyash-llvm-compiler tool/package names, ABI helper symbols, historical Nyash
Era material, or archived pages.
```

Guardrail:

```text
do not broad-rewrite docs/reference/**
do not rename stage-profiles.md or compatibility path names in this decision
```

### HAKORUNE-REFERENCE-DOCS-FIRST-CUT-001

Status: landed.

Scope:

```text
docs/reference/language/quick-reference.md:
  title and purpose are Hakorune-first
  current backend mention says NyRT instead of Ny
  prod config guidance prefers hako.toml with nyash.toml compatibility
  LoopRange profile wording says phase-1 / bootstrap

docs/reference/core-language/README.md:
  title is Hakorune-first
  Stage Profiles link description says bootstrap / phase-1

docs/reference/architecture/phi-and-ssa.md:
  title and overview are Hakorune-first
```

Non-claims:

```text
docs/reference/** broadly rewritten = 0
stage-profiles.md renamed = 0
NYASH_* env names renamed = 0
nyash.toml compatibility name removed = 0
ny-llvmc / nyash-llvm-compiler names renamed = 0
historical Nyash pages rewritten = 0
runtime behavior changed = 0
```

### HAKORUNE-REFERENCE-DOCS-ENTRY-INDEX-WORDING-001

Status: landed.

Scope:

```text
docs/reference/README.md:
  support-profile pointers say bootstrap / phase-1

docs/reference/language/README.md:
  title and entry sentence are Hakorune-first
  support-profile pointers say bootstrap / phase-1
  bootstrap no-match and phase-1/selfhost support wording replace Stage0/Stage1

docs/reference/language/EBNF.md:
  title is Hakorune-first
  support status says bootstrap / phase-1
```

Non-claims:

```text
stage-profiles.md renamed = 0
Stage-2 historical note removed = 0
NYASH_* env names renamed = 0
nyash.toml compatibility name removed = 0
runtime behavior changed = 0
```

### HAKORUNE-REFERENCE-DOCS-INVARIANTS-CONSTRAINTS-WORDING-001

Status: landed.

Scope:

```text
docs/reference/invariants.md:
  title is Hakorune-first

docs/reference/constraints.md:
  title is Hakorune-first
```

Non-claims:

```text
NYASH_* env names renamed = 0
internal compatibility identifiers renamed = 0
constraints entries broadly rewritten = 0
runtime behavior changed = 0
```

### HAKORUNE-REFERENCE-DOCS-MIR-GC-WORDING-001

Status: landed.

Scope:

```text
docs/reference/mir/INSTRUCTION_SET.md:
  title and opening sentence are Hakorune-first

docs/reference/runtime/gc.md:
  title and overview sentence are Hakorune-first
```

Non-claims:

```text
NYASH_* env names renamed = 0
nyash.toml compatibility name removed = 0
NyRT / ABI names renamed = 0
MIR instruction vocabulary changed = 0
runtime behavior changed = 0
```

### HAKORUNE-BINARY-PRIMARY-CUTOVER-INVENTORY-001

Status: landed inventory cut.

Scope:

- inventory the current Cargo binary surface for Hakorune vs legacy `nyash`;
- verify the `hakorune` binary exists and is checked by quick gate;
- verify the legacy `nyash` binary still exists only as compatibility surface;
- verify `nyash` invocation has an explicit deprecation / allow gate;
- do not rename the Cargo package, library crate, plugin crates, `ny-llvmc`,
  ABI helper symbols, or existing script internals in this slice.

Current inventory:

```text
Cargo package:
  package.name = nyash-rust
  status = legacy package identity, not renamed in this slice
  default-run = hakorune

library crate:
  lib.name = nyash_rust
  status = legacy crate identity, not renamed in this slice

primary user-facing binary:
  [[bin]] name = hakorune
  path = src/bin/hakorune.rs
  implementation = thin include of src/main.rs
  quick gate = cargo check --bin hakorune

legacy compatibility binary:
  [[bin]] name = nyash
  path = src/main.rs
  invocation policy = deprecated; requires explicit allow env for legacy use

compat helper binary:
  [[bin]] name = hakorune-compat
  path = src/bin/hakorune_compat.rs
  status = compatibility wrapper, not product primary

tool wrapper:
  tools/bin/hako
  resolution = target/release/hakorune first, target/release/nyash fallback
```

Allowed compatibility:

```text
NYASH_BIN:
  env compatibility variable, default should point at target/release/hakorune

target/release/nyash:
  legacy binary fallback only, not primary docs route

ny-llvmc:
  backend compiler tool name, not the product binary
```

Known remaining drift:

```text
Windows / PowerShell build scripts:
  some still build or invoke --bin nyash
  next action = HAKORUNE-WINDOWS-BUILD-SCRIPT-CUTOVER-INVENTORY-001

Rust build helpers:
  src/runner/build_product.rs and src/runner/build_engineering.rs still refer
  to target/<profile>/nyash
  next action = HAKORUNE-RUNNER-BUILD-HELPER-BINARY-RESOLUTION-001

plugin/package/ABI surfaces:
  nyash-* packages, nyash_kernel, ny-llvmc, and nyash.* ABI helper symbols stay
  out of this slice
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/checks/dev_gate.sh quick
```

### HAKORUNE-BINARY-DEFAULT-RUN-CUTOVER-001

Status: landed Cargo default-run cut.

Scope:

- make plain `cargo run` resolve to the primary `hakorune` binary;
- keep explicit `cargo run --bin nyash` available only as legacy compatibility;
- do not rename the Cargo package, library crate, plugin packages, backend tools,
  ABI helper symbols, or script internals in this slice.

Contract:

```text
Cargo package:
  package.name = nyash-rust
  default-run = hakorune

primary command:
  cargo run -- <args>
  resolves to [[bin]] name = hakorune

legacy command:
  cargo run --bin nyash -- <args>
  remains explicit compatibility only
  requires the existing nyash deprecation / allow policy
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
cargo run -q -- --version
tools/checks/dev_gate.sh quick
```

### HAKORUNE-RUNNER-BUILD-HELPER-BINARY-RESOLUTION-001

Status: landed runner helper cut.

Scope:

- route product / engineering build helper child-process execution through a
  single Hakorune-first binary resolver;
- prefer `target/<profile>/hakorune` when present;
- fall back to `target/<profile>/nyash` only when the primary binary is absent;
- keep package/crate/plugin/ABI/tool names untouched in this slice.

Contract:

```text
resolver:
  src/runner/build_shared.rs::hakorune_cli_bin_path

primary:
  target/<profile>/hakorune(.exe)

compat fallback:
  target/<profile>/nyash(.exe)
  used only if primary is absent

callers:
  src/runner/build_product.rs
  src/runner/build_engineering.rs
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
cargo test -q --lib hakorune_cli_bin_path
tools/checks/dev_gate.sh quick
```

### HAKORUNE-WINDOWS-BUILD-SCRIPT-CUTOVER-INVENTORY-001

Status: landed Windows script cut.

Scope:

- make Windows build scripts build / invoke `hakorune` as the primary binary;
- replace direct `cargo build --bin nyash` with `cargo build --bin hakorune`;
- replace direct `target\release\nyash.exe` invocation with a Hakorune-first
  resolver that falls back to `nyash.exe` only when the primary binary is absent;
- keep historical script filenames, plugin package names, ABI symbols, backend
  tool names, and `nyash` compatibility fallback names untouched.

Contract:

```text
Windows primary binary:
  target\release\hakorune.exe

legacy fallback:
  target\release\nyash.exe
  allowed only behind explicit Hakorune-first resolver

forbidden current command:
  cargo build --bin nyash
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-HAKO-CHECK-BINARY-RESOLUTION-001

Status: landed hako-check wrapper cut.

Scope:

- make `tools/hako-check/hako-check.sh` resolve `hakorune` before legacy
  `nyash`;
- keep `HAKO_BIN` as the explicit override;
- keep `tools/bin/hako` as preferred local wrapper when present;
- keep `target/release/nyash` only as compatibility fallback when the primary
  binary is absent.

Contract:

```text
resolution order:
  HAKO_BIN
  tools/bin/hako
  tools/bin/hakorune
  target/release/hakorune
  target/release/nyash
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/hako-check/hako-check.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-ROOT-POWERSHELL-BUILD-SCRIPT-CUTOVER-001

Status: landed root PowerShell script cut.

Scope:

- make root PowerShell build scripts invoke `hakorune` as the primary binary;
- keep legacy `nyash.exe` only behind an explicit Hakorune-first resolver;
- keep historical `NYASH_*` object-output env names untouched in this slice;
- keep plugin/package/crate names untouched in this slice.

Contract:

```text
root PowerShell scripts:
  tools/build_llvm.ps1
  tools/build_aot.ps1

primary:
  target\release\hakorune.exe

compat fallback:
  target\release\nyash.exe
  used only if primary is absent

resolver:
  Resolve-HakoruneCli
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-DEV-SELFHOST-SMOKE-BINARY-RESOLUTION-001

Status: landed dev/selfhost smoke cut.

Scope:

- make lightweight using/selfhost dev smoke scripts invoke `hakorune` as the
  primary binary;
- keep `target/release/nyash` only behind explicit Hakorune-first resolver
  fallback;
- keep legacy `NYASH_*` behavior/env names untouched in this slice;
- keep plugin/package/crate names untouched in this slice.

Scripts:

```text
tools/using_unresolved_smoke.sh
tools/using_resolve_smoke.sh
tools/using_strict_path_fail_smoke.sh
tools/dev_selfhost_loop.sh
```

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/using_unresolved_smoke.sh
tools/using_resolve_smoke.sh
tools/using_strict_path_fail_smoke.sh
tools/dev_selfhost_loop.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-ENGINEERING-PARITY-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/engineering/parity.sh` invoke `hakorune` as the primary binary;
- keep `target/release/nyash` only behind explicit Hakorune-first resolver
  fallback;
- keep legacy `NYASH_BIN` override behavior untouched in this slice;
- keep parity modes and backend routes untouched in this slice.

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent

override:
  NYASH_BIN
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/engineering/parity.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SELFHOST-EXE-STAGEB-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- rename `tools/selfhost_exe_stageb.sh` internal binary resolver to
  Hakorune-first terminology;
- keep `target/release/nyash` only behind explicit Hakorune-first resolver
  fallback;
- keep legacy `NYASH_BIN` override behavior untouched in this slice;
- keep direct and stageb-delegate emit routes untouched in this slice.

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent

override:
  NYASH_BIN
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/selfhost_exe_stageb.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-CORE-EMIT-HELPER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/hakorune_emit_mir.sh` spell direct binary detection in
  Hakorune-first terms;
- keep `NYASH_BIN` as the historical compatibility override consumed by the
  helper and smoke routes;
- keep legacy `target/release/nyash` only as a named compatibility fallback;
- keep MIR emit route behavior unchanged.

Affected script:

```text
tools/hakorune_emit_mir.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/hakorune_emit_mir.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SELFHOST-EXE-STAGEB-SOURCE-WORDING-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/selfhost_exe_stageb.sh` describe `.hako` input as a Hakorune
  source instead of a Nyash source;
- keep script name, Stage-B compatibility route names, and executable behavior
  unchanged.

Affected script:

```text
tools/selfhost_exe_stageb.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/selfhost_exe_stageb.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NAMING-GUARD-REQUIRED-FILES-READABILITY-001

Status: landed guard-structure cut.

Scope:

- split the `tools/checks/naming_charter_guard.sh` required-file list into a
  shell array;
- keep guard behavior unchanged;
- make future naming cleanup additions produce readable diffs instead of one
  very long `guard_require_files` line.

Affected script:

```text
tools/checks/naming_charter_guard.sh
```

Acceptance:

```bash
bash -n tools/checks/naming_charter_guard.sh
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NAMING-GUARD-REQUIRED-FILES-DUPLICATE-CHECK-001

Status: landed guard-structure cut.

Purpose: keep the naming guard's required-file list fail-fast when future
cleanup slices accidentally add the same required file twice.

Scope:

- check the `tools/checks/naming_charter_guard.sh` `REQUIRED_FILES` array for
  duplicate expanded paths before calling `guard_require_files`;
- keep required-file membership and guard behavior unchanged for non-duplicate
  lists;
- keep the duplicate diagnostic specific to the naming guard.

Affected script:

```text
tools/checks/naming_charter_guard.sh
```

Acceptance:

```bash
bash -n tools/checks/naming_charter_guard.sh
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NAMING-GUARD-DUPLICATE-CHECK-HELPER-001

Status: landed guard-structure cut.

Purpose: keep naming guard duplicate checks on one implementation path.

Scope:

- add one local `guard_require_unique_values` helper to
  `tools/checks/naming_charter_guard.sh`;
- make required-file, SSOT-token, and diff-allowlist duplicate checks call the
  same helper;
- keep duplicate diagnostics label-specific;
- keep guarded lists and non-duplicate behavior unchanged.

Affected script:

```text
tools/checks/naming_charter_guard.sh
```

Acceptance:

```bash
bash -n tools/checks/naming_charter_guard.sh
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NAMING-GUARD-SSOT-TOKEN-LIST-READABILITY-001

Status: landed guard-structure cut.

Scope:

- split the `tools/checks/naming_charter_guard.sh` SSOT task token checks into
  a shell array;
- fail fast if the guard token list contains a duplicate token;
- keep guard behavior unchanged;
- make future task-token additions produce readable diffs instead of one long
  block of repeated `require_fixed` calls.

Affected script:

```text
tools/checks/naming_charter_guard.sh
```

Acceptance:

```bash
bash -n tools/checks/naming_charter_guard.sh
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NAMING-GUARD-DIFF-ALLOWLIST-SSOT-001

Status: landed guard-structure cut.

Purpose: keep the naming guard's allowed diff paths in one local SSOT so the
unstaged/cached diff scan and untracked-file scan cannot drift.

Scope:

- add one `NAMING_DIFF_ALLOWED_PATHS` array to
  `tools/checks/naming_charter_guard.sh`;
- make the shell untracked-file scan and the awk diff scan consume the same
  path list;
- fail fast if the allowed path list contains a duplicate path;
- keep guard behavior unchanged;
- do not broaden the allowed path set.

Affected script:

```text
tools/checks/naming_charter_guard.sh
```

Acceptance:

```bash
bash -n tools/checks/naming_charter_guard.sh
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SELFHOST-ROUTE-BINARY-DIAGNOSTICS-001

Status: landed naming-cleanup cut.

Scope:

- keep selfhost route/build executable resolution behavior unchanged;
- keep `$NYASH_BIN` as the historical compatibility override name;
- make missing-binary diagnostics name Hakorune as the current executable;
- add naming guard coverage so selfhost route/build diagnostics do not regress
  to legacy-only `nyash` wording.

Affected scripts:

```text
tools/selfhost/proof/run_stageb_compiler_vm.sh
tools/selfhost/lib/selfhost_run_routes.sh
tools/selfhost/selfhost_build.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/selfhost/proof/run_stageb_compiler_vm.sh
bash -n tools/selfhost/lib/selfhost_run_routes.sh
bash -n tools/selfhost/selfhost_build.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SELFHOST-MAINLINE-STAGE1-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/selfhost/mainline/build_stage1.sh` spell bootstrap executable
  resolution in Hakorune-first terms;
- keep `NYASH_BIN` as the historical compatibility override consumed by the
  Stage1 mainline script;
- keep legacy `target/release/nyash` only as a named compatibility fallback;
- keep Stage1 artifact names, Stage-B route wording, and build behavior
  unchanged.

Affected script:

```text
tools/selfhost/mainline/build_stage1.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/selfhost/mainline/build_stage1.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SELFHOST-RUN-DIRECT-MODE-B-DIAGNOSTIC-001

Status: landed naming-cleanup cut.

Scope:

- make the day-to-day `tools/selfhost/run.sh --direct` README description use
  `mode-B` instead of `Stage-B`;
- keep explicit proof-only Stage-B route names and script names unchanged;
- keep `tools/selfhost/run.sh` behavior unchanged.

Affected doc:

```text
tools/selfhost/README.md
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PARSER-BRIDGE-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/ny_parser_bridge_smoke.sh` use Hakorune-first resolver variable
  names;
- keep `target/release/nyash` only behind explicit `LEGACY_NYASH_BIN`
  fallback;
- rename temporary smoke output files from `nyash-bridge-*` to
  `hakorune-bridge-*`;
- keep parser bridge behavior and expected rc values untouched in this slice.

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/ny_parser_bridge_smoke.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PHI-TRACE-RUNNER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/debug/phi/phi_trace_run.sh` invoke `hakorune` before legacy
  `nyash`;
- keep `target/release/nyash` only behind explicit `LEGACY_NYASH_BIN`
  fallback;
- keep PHI trace environment and checker behavior untouched in this slice;
- do not edit PHI lifecycle or MIR PHI construction code in this slice.

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent

override:
  HAKORUNE_BIN
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/debug/phi/phi_trace_run.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-TEST-SHLIB-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/test/lib/shlib.sh` emit-json helper invoke `hakorune` before
  legacy `nyash`;
- keep `target/release/nyash` only behind explicit Hakorune-first resolver
  fallback;
- keep legacy helper function names untouched for compatibility in this slice;
- keep test route behavior untouched in this slice.

Contract:

```text
primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent

override:
  HAKORUNE_BIN
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/test/lib/shlib.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SMOKE-EMIT-MIR-ROUTE-BINARY-ALIAS-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/smokes/v2/lib/emit_mir_route.sh` accept `HAKO_BIN` as the
  preferred alias for the historical `NYASH_BIN`;
- keep `target/release/nyash` only behind explicit Hakorune-first resolver
  fallback;
- keep route behavior and argument contract untouched in this slice;
- keep direct/hako-mainline/hako-helper route semantics untouched in this slice.

Contract:

```text
preferred override:
  HAKO_BIN

compat override:
  NYASH_BIN

primary:
  target/release/hakorune

compat fallback:
  target/release/nyash
  used only if primary is absent
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/smokes/v2/lib/emit_mir_route.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-BRIDGE-CANONICALIZE-STABLE-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make bridge canonicalize stable smoke scripts use the `NYASH_BIN` resolver
  from `tools/smokes/v2/lib/test_runner.sh`;
- keep `target/release/nyash` only behind the shared Hakorune-first resolver
  fallback;
- keep v1 JSON fixtures and expected rc/message behavior untouched in this
  slice;
- keep canonicalize semantics untouched in this slice;
- leave stale opt-in `canonicalize_off/on` canaries unchanged in this slice.

Affected scripts:

```text
tools/smokes/v2/profiles/quick/core/bridge/canonicalize_noop_method_on_vm.sh
tools/smokes/v2/profiles/quick/core/bridge/canonicalize_fail_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/smokes/v2/profiles/quick/core/bridge/canonicalize_noop_method_on_vm.sh
SMOKES_ENABLE_BRIDGE_CANON=1 bash tools/smokes/v2/profiles/quick/core/bridge/canonicalize_fail_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-WRAPPER-EXECUTABLE-BIT-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/bin/hako` executable so opt-in Hako smokes do not skip only
  because the wrapper has mode `100644`;
- keep wrapper content and Hakorune-first binary resolution unchanged in this
  slice;
- add naming guard coverage for the executable bit.

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
tools/bin/hako --version
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-MIN-OPTIN-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make Hako minimum opt-in smokes invoke `$HAKO_BIN` instead of direct
  `target/release/nyash`;
- fix the HHako main detection helper arity mismatch surfaced by running those
  opt-in smokes;
- keep `.hako` fixtures and expected outputs unchanged in this slice;
- add naming guard coverage so these smokes do not regress to direct legacy
  binary calls.

Affected scripts:

```text
tools/smokes/v2/profiles/quick/core/hako_min_binop_vm.sh
tools/smokes/v2/profiles/quick/core/hako_min_if_vm.sh
tools/smokes/v2/profiles/quick/core/index_operator_hako.sh
tools/smokes/v2/profiles/quick/core/hako_min_compile_return_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
SMOKES_ENABLE_HAKO_BINOP=1 bash tools/smokes/v2/profiles/quick/core/hako_min_binop_vm.sh
SMOKES_ENABLE_HAKO_IF=1 bash tools/smokes/v2/profiles/quick/core/hako_min_if_vm.sh
SMOKES_ENABLE_HAKO_INDEX=1 bash tools/smokes/v2/profiles/quick/core/index_operator_hako.sh
bash tools/smokes/v2/profiles/quick/core/hako_min_compile_return_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-QUICK-SMOKE-MODE-A-DIAGNOSTIC-001

Status: landed naming-cleanup cut.

Scope:

- make quick opt-in smoke comments/diagnostics use `mode-A` instead of
  `Stage-A`;
- keep smoke execution behavior and `.hako` fixtures unchanged;
- keep broader Stage-A/Stage-B compatibility naming outside this diagnostic
  slice.

Affected scripts:

```text
tools/smokes/v2/profiles/quick/core/hako_min_compile_return_vm.sh
tools/smokes/v2/profiles/quick/core/hako_map_escape_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/profiles/quick/core/hako_min_compile_return_vm.sh
bash -n tools/smokes/v2/profiles/quick/core/hako_map_escape_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-QUICK-SMOKE-MODE-B-DIAGNOSTIC-001

Status: landed naming-cleanup cut.

Scope:

- make shared quick smoke helper comments/diagnostics use `mode-B` instead of
  `Stage-B`;
- keep smoke execution behavior and helper function names unchanged;
- keep broader Stage-A/Stage-B compatibility naming outside this diagnostic
  slice.

Affected script:

```text
tools/smokes/v2/lib/stageb_helpers.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/lib/stageb_helpers.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-MAP-ESCAPE-OPTIN-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make the Hako map escape opt-in smoke invoke `$HAKO_BIN` instead of direct
  `target/release/nyash`;
- keep boundary-case diagnostics and `.hako` fixtures unchanged in this slice;
- add naming guard coverage so the smoke does not regress to direct legacy
  binary calls.

Affected script:

```text
tools/smokes/v2/profiles/quick/core/hako_map_escape_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
SMOKES_ENABLE_STAGEA_BOUNDARY=1 bash tools/smokes/v2/profiles/quick/core/hako_map_escape_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-GATE-C-NYVM-WRAPPER-SMOKE-BINARY-NAMING-001

Status: landed naming-cleanup cut.

Scope:

- make Gate-C v1 file and NyVM wrapper smokes spell the Hakorune-first
  executable resolver explicitly;
- keep legacy `nyash` only as a named compatibility fallback;
- keep JSON fixtures and expected smoke behavior unchanged in this slice;
- add naming guard coverage so these smokes do not regress to direct legacy
  binary naming.

Affected scripts:

```text
tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh
tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
SMOKES_ENABLE_GATE_C_V1=1 bash tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh
SMOKES_ENABLE_NYVM_WRAPPER=1 bash tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PARSER-INTEGRATION-SMOKE-BINARY-NAMING-001

Status: landed naming-cleanup cut.

Scope:

- make selected parser integration smokes spell the Hakorune-first executable
  resolver explicitly;
- keep legacy `nyash` only as a named compatibility fallback;
- keep parser fixtures and expected smoke behavior unchanged in this slice;
- add naming guard coverage so these smokes do not regress to direct legacy
  binary naming.

Affected scripts:

```text
tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PARSER-TRY-COMPAT-SMOKE-BINARY-NAMING-001

Status: landed naming-cleanup cut.

Scope:

- make the parser try-compat smoke spell the Hakorune-first executable
  resolver explicitly;
- keep legacy `nyash` only as a named compatibility fallback;
- keep parser fixtures and expected freeze-tag behavior unchanged in this slice;
- add naming guard coverage so the smoke does not regress to direct legacy
  binary naming.

Affected script:

```text
tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PARSER-INTEGRATION-EXTRA-SMOKE-BINARY-NAMING-001

Status: landed naming-cleanup cut.

Scope:

- make additional parser integration smokes spell Hakorune-first binary
  resolution and diagnostics;
- keep legacy `nyash` only as a named compatibility fallback;
- keep legacy `$NYASH_BIN` override behavior untouched in this slice;
- keep parser fixture behavior unchanged in this slice.

Affected scripts:

```text
tools/smokes/v2/profiles/integration/parser/parser_min_methods_ok.sh
tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/profiles/integration/parser/parser_min_methods_ok.sh
bash -n tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-GOLDEN-MACRO-BINARY-RESOLVER-001

Status: landed naming-cleanup cut.

Scope:

- introduce one shared Hakorune-first resolver for macro golden scripts;
- keep legacy `target/release/nyash` only as a named compatibility fallback;
- keep `$NYASH_BIN` as the historical compatibility override;
- keep macro golden fixtures and comparison semantics unchanged in this slice;
- add naming guard coverage so generated/golden scripts do not regress to
  direct legacy binary paths or legacy-only diagnostics.

Affected scripts:

```text
tools/test/golden/macro/lib/resolve_hakorune.sh
tools/test/golden/macro/*_golden.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
for f in tools/test/golden/macro/*.sh; do bash -n "$f"; done
git diff --check
tools/checks/dev_gate.sh quick
```

Executing the macro golden scripts is intentionally outside this naming slice:
current AST JSON output may differ from older golden baselines. This slice only
changes binary resolution and diagnostics.

### HAKORUNE-CURRENT-DIAGNOSTIC-BINARY-WORDING-001

Status: landed naming-cleanup cut.

Scope:

- make current hako-check / selfhost diagnostic scripts name Hakorune first;
- keep `NYASH_BIN` as the historical compatibility override;
- keep archive scripts and historical snapshots untouched in this slice;
- keep route behavior and fixture semantics unchanged.

Affected files:

```text
tools/hako_check/fastmem_source_manifest_runner.py
tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh
tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
python3 -m py_compile tools/hako_check/fastmem_source_manifest_runner.py
bash -n tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh
bash -n tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SMOKE-TEST-RUNNER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/smokes/v2/lib/test_runner.sh` spell the shared smoke binary
  resolver in Hakorune-first terms;
- keep `NYASH_BIN` as the historical compatibility variable consumed by
  existing smoke bodies;
- keep legacy `target/release/nyash` only as a named compatibility fallback;
- keep smoke execution behavior unchanged.

Affected file:

```text
tools/smokes/v2/lib/test_runner.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/lib/test_runner.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SMOKE-AUTO-DETECT-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/smokes/v2/configs/auto_detect.conf` spell CLI binary detection
  in Hakorune-first terms;
- keep `NYASH_BIN_RESOLVED` and `NYASH_BIN` as historical compatibility
  overrides consumed by existing smoke config;
- keep legacy `./target/release/nyash` only as a named compatibility fallback;
- keep smoke auto-detection behavior unchanged.

Affected file:

```text
tools/smokes/v2/configs/auto_detect.conf
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/configs/auto_detect.conf
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-GATE-C-OOB-STRICT-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make the Gate-C OOB strict opt-in smoke invoke `$HAKO_BIN` instead of direct
  `target/release/nyash`;
- keep Stage-B source fixtures and strict OOB expectations unchanged in this
  slice;
- add naming guard coverage so the smoke does not regress to direct legacy
  binary calls.

Affected script:

```text
tools/smokes/v2/profiles/quick/core/gate_c_oob_strict_fail_vm.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
SMOKES_ENABLE_OOB_STRICT=1 bash tools/smokes/v2/profiles/quick/core/gate_c_oob_strict_fail_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-NY-MIR-BUILDER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make the `tools/ny_mir_builder.sh --emit ll` helper spell the Hakorune-first
  executable resolver explicitly;
- keep the legacy `nyash` binary only as a named compatibility fallback;
- keep backend selection and LLVM harness behavior unchanged in this slice;
- add naming guard coverage so the helper does not regress to a direct legacy
  binary fallback.

Affected script:

```text
tools/ny_mir_builder.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/ny_mir_builder.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-PHASE29X-CACHE-HELPER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make the Phase29x L1/L2 cache helpers spell the Hakorune-first executable
  resolver explicitly;
- keep caller-provided `$NYASH_BIN` as the historical override alias;
- keep legacy `nyash` only as a named compatibility fallback;
- keep cache key, MIR emit, and object emit behavior unchanged in this slice;
- add naming guard coverage so cache helpers do not regress to ambiguous direct
  legacy fallback.

Affected scripts:

```text
tools/cache/phase29x_l1_mir_cache.sh
tools/cache/phase29x_l2_object_cache.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/cache/phase29x_l1_mir_cache.sh --help
bash tools/cache/phase29x_l2_object_cache.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SMOKE-SHARED-PREFLIGHT-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make shared smoke preflight/plugin helpers spell the Hakorune-first
  executable resolver explicitly;
- keep caller-provided `$NYASH_BIN` as the historical override alias;
- keep legacy `nyash` only as a named compatibility fallback;
- keep preflight/plugin behavior unchanged in this slice;
- add naming guard coverage so shared smoke helpers do not regress to ambiguous
  direct legacy fallback.

Affected scripts:

```text
tools/smokes/v2/lib/preflight.sh
tools/smokes/v2/lib/plugin_manager.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/lib/preflight.sh
bash -n tools/smokes/v2/lib/plugin_manager.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-SMOKE-PREFLIGHT-STAGE-TERM-DIAGNOSTIC-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/smokes/v2/lib/preflight.sh` user-facing build guidance say
  `Hakorune bootstrap CLI` instead of `Stage0 CLI`;
- keep bootstrap implementation and binary resolver behavior unchanged;
- keep `stage0` path/module names out of scope for this diagnostic-only slice.

Affected script:

```text
tools/smokes/v2/lib/preflight.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/lib/preflight.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-HAKO-CHECK-WRAPPER-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/hako-check/hako-check.sh` spell the Hakorune-first executable
  resolver explicitly;
- keep caller-provided `$HAKO_BIN` as the preferred override alias;
- keep legacy `nyash` only as a named compatibility fallback;
- keep hako-check parse/MIR/verify behavior unchanged in this slice;
- add naming guard coverage so the wrapper does not regress to ambiguous direct
  legacy fallback.

Affected script:

```text
tools/hako-check/hako-check.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash tools/hako-check/hako-check.sh --help
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-FASTMEM-HAKO-CHECK-SMOKE-BINARY-RESOLUTION-001

Status: landed naming-cleanup cut.

Scope:

- make `tools/hako_check` fastmem smoke scripts spell the Hakorune-first
  executable resolver explicitly;
- keep caller-provided `$HAKO_BIN` as the preferred override alias;
- keep caller-provided `$NYASH_BIN` as historical compatibility override;
- keep legacy `nyash` only as a named compatibility fallback;
- keep fastmem source / terminal ladder semantics unchanged in this slice;
- add naming guard coverage so these smokes do not regress to ambiguous direct
  legacy fallback or `hakorune/nyash` mixed error wording.

Affected scripts:

```text
tools/hako_check/fastmem_source_syntax_smoke.sh
tools/hako_check/fastmem_terminal_ladder_smoke.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/hako_check/fastmem_source_syntax_smoke.sh
bash -n tools/hako_check/fastmem_terminal_ladder_smoke.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### HAKORUNE-COLLECTION-QUICK-SMOKE-BINARY-WORDING-001

Status: landed naming-cleanup cut.

Scope:

- keep quick collection smoke execution behavior unchanged;
- keep caller-provided `$NYASH_BIN` as the historical compatibility override;
- make the missing-binary diagnostic name the current Hakorune executable;
- add naming guard coverage so these smokes do not regress to legacy-only
  wording.

Affected scripts:

```text
tools/smokes/v2/profiles/quick/collections/map_get_shares_map.sh
tools/smokes/v2/profiles/quick/collections/map_get_shares_array.sh
tools/smokes/v2/profiles/quick/collections/string_size_alias.sh
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
bash -n tools/smokes/v2/profiles/quick/collections/map_get_shares_map.sh
bash -n tools/smokes/v2/profiles/quick/collections/map_get_shares_array.sh
bash -n tools/smokes/v2/profiles/quick/collections/string_size_alias.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Runtime execution of these quick collection scripts still depends on the
currently available VM reference route. This naming slice does not alter that
route.

### STAGE-TERM-EXISTING-NAME-INVENTORY-001

Status: classification-only inventory recorded.

Purpose: classify existing `stage` terms before any rename, so future slices do
not perform broad text replacement or mix layers.

Inventory:

```text
docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Non-claims:

```text
existing_stage_terms_renamed = 0
direct_global_replacement = 0
stage_term_rename_without_classification = 0
```

### STAGE-TERM-SYNTAX3-ALIAS-001

Status: landed compatibility alias.

Purpose: introduce `--syntax-3` as the frontend syntax-level spelling while
keeping `--stage3` as a compatibility alias.

Scope:

```text
Rust CLI:
  --syntax-3 visible alias for the existing stage3 parser flag

HHako compiler entry:
  --syntax-3 and --stage3 both accepted

Rust selfhost child spawn:
  new child args use --syntax-3

Selfhost proof/quickstart:
  representative proof command and quickstart examples use --syntax-3

Reference docs:
  --syntax-3 documented first, --stage3 retained as compatibility
```

Non-claims:

```text
--stage3 removed = 0
NYASH_NY_COMPILER_STAGE3 renamed = 0
parser internal stage3 API renamed = 0
```

Acceptance:

```bash
cargo test -q --lib syntax3_alias_sets_stage3_parser_flag
cargo test -q --features vm-reference --test phase29ci_stageb_body_extract stageb_compiler_no_longer_falls_back_to_full_source_for_hello_simple_fixture
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-SYNTAX3-DIAGNOSTIC-WORDING-001

Status: landed diagnostic wording cleanup.

Purpose: use `syntax-3` / `mode-B compatibility routes` in live MIR builder
undefined-variable hints, without changing parser internals or compatibility
env names.

Affected file:

```text
src/mir/builder/builder_build.rs
```

Non-claims:

```text
parser internal stage3 API renamed = 0
NYASH_FEATURES=stage3 renamed = 0
HAKO_PARSER_STAGE3 renamed = 0
Stage-B route token removed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001

Status: landed env/comment wording cleanup.

Purpose: use `mode-B compatibility` wording in live env docs/comments while
keeping existing `STAGEB` environment names and route tokens as compatibility
aliases.

Affected files:

```text
src/config/env/verification_flags.rs
src/runner/stage1_bridge/env/parser_stageb.rs
docs/reference/environment-variables.md
```

Non-claims:

```text
NYASH_STAGEB_DEV_VERIFY renamed = 0
HAKO_STAGEB_* renamed = 0
Stage-B route token removed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001

Status: landed proof-route wording cleanup.

Purpose: use `mode-B compatibility` wording for the explicit proof-only
selfhost route while keeping the external route tokens and script names
unchanged.

Affected files:

```text
tools/selfhost/proof/run_stageb_compiler_vm.sh
tools/selfhost/proof/selfhost_smoke.sh
tools/selfhost/README.md
```

Non-claims:

```text
--stage-b removed = 0
stageb-delegate renamed = 0
run_stageb_compiler_vm.sh renamed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001

Status: landed bridge wording cleanup.

Purpose: use `mode-B compatibility` wording for Stage-1 bridge module payload
alias comments while keeping existing `HAKO_STAGEB_*` environment names and
bridge file names unchanged.

Affected files:

```text
src/runner/stage1_bridge/README.md
src/runner/stage1_bridge/env.rs
src/runner/stage1_bridge/modules.rs
```

Non-claims:

```text
HAKO_STAGEB_* renamed = 0
parser_stageb.rs renamed = 0
modules.rs behavior changed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEA-COMPAT-ROUTE-WORDING-001

Status: landed route wording cleanup.

Purpose: use `mode-A compatibility` wording for the Rust selfhost compat route
comments/diagnostics while keeping existing file names, function names, and
`stage-a-compat` runtime-mode compatibility tokens unchanged.

Affected files:

```text
src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs
src/runner/modes/common_util/selfhost/stage_a_route.rs
src/runner/modes/common_util/selfhost/stage_a_policy.rs
src/runner/modes/common_util/selfhost/stage_a_spawn.rs
src/runner/modes/common_util/selfhost/json.rs
src/runner/modes/common_util/selfhost/stage0_capture_route.rs
src/runner/selfhost.rs
```

Non-claims:

```text
stage-a-compat runtime-mode alias renamed = 0
stage_a_* file names renamed = 0
function names renamed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-HHAKO-ENTRY-WORDING-001

Status: landed HHako entry wording cleanup.

Purpose: use `mode-B compatibility` wording in HHako compiler entry comments
while keeping `StageB*` Box names, trace strings, file names, and `--stage-b`
route tokens unchanged.

Affected files:

```text
lang/src/compiler/README.md
lang/src/compiler/entry/compiler_stageb.hako
lang/src/compiler/entry/stageb_args_box.hako
lang/src/compiler/entry/stageb_build_options_box.hako
lang/src/compiler/entry/stageb_compile_adapter_box.hako
lang/src/compiler/entry/stageb_output_box.hako
```

Non-claims:

```text
StageB* Box names renamed = 0
trace strings renamed = 0
--stage-b removed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-HHAKO-COMPAT-FIXTURE-WORDING-001

Status: landed HHako compat fixture wording cleanup.

Purpose: use `mode-B compatibility` wording in HHako legacy fixture/adapter
comments that explicitly defer live source-to-Program authority to BuildBox,
while keeping `StageB*` Box names, env names, trace strings, file names, and
route tokens unchanged.

Affected files:

```text
lang/src/compiler/entry/bundle_resolver.hako
lang/src/compiler/entry/stageb_body_extractor_box.hako
lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako
```

Non-claims:

```text
StageB* Box names renamed = 0
HAKO_STAGEB_* env names renamed = 0
trace strings renamed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-HHAKO-COMPILER-ROUTE-WORDING-001

Status: landed HHako compiler route wording cleanup.

Purpose: use `mode-A compatibility` / `mode-B compatibility` wording in
`compiler.hako` route comments and the string-indexing diagnostic while keeping
`StageB*` Box names, `stage_b` fields, trace strings, and `--stage-b` route
tokens unchanged.

Affected file:

```text
lang/src/compiler/entry/compiler.hako
```

Non-claims:

```text
StageB* Box names renamed = 0
stage_b field renamed = 0
trace strings renamed = 0
--stage-b removed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-HHAKO-HELPER-COMMENT-WORDING-001

Status: landed HHako helper comment wording cleanup.

Purpose: use `mode-B compatibility` / `mode-A compatibility` wording in HHako
helper comments that describe mode-B/source-route responsibilities, while
keeping compatibility names and behavior unchanged.

Affected files:

```text
lang/src/compiler/entry/stageb_driver_guard_box.hako
lang/src/compiler/entry/stageb_trace_box.hako
lang/src/compiler/entry/stageb_main_detection_box.hako
lang/src/compiler/entry/stageb/stageb_rune_box.hako
lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako
```

Non-claims:

```text
StageB* Box names renamed = 0
HAKO_STAGEB_* env names renamed = 0
trace strings renamed = 0
stageb_* file names renamed = 0
runtime behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-CAPTURE-CALLER-GUARD-WORDING-001

Status: landed capture caller guard wording cleanup.

Purpose: use `mode-B compatibility` wording in the active Program(JSON)
capture caller guard comments/diagnostics and quick gate label while keeping
compatibility script names and allowed caller surfaces unchanged.

Affected files:

```text
tools/checks/stageb_program_json_capture_caller_guard.sh
tools/checks/lib/dev_gate_quick_steps.sh
```

Non-claims:

```text
stageb_program_json_capture_caller_guard.sh renamed = 0
stageb_program_json_capture.sh renamed = 0
allowed caller list changed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-PHASE1-PROGRAM-JSON-GUARD-WORDING-001

Status: landed phase-1 Program(JSON) guard wording cleanup.

Purpose: use `phase-1 compatibility` wording in active Stage1 Program(JSON)
guard comments/diagnostics and quick gate labels while keeping script names,
fixture names, and helper symbols unchanged.

Affected files:

```text
tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
tools/checks/stage1_program_json_compat_caller_guard.sh
tools/checks/lib/dev_gate_quick_steps.sh
```

Non-claims:

```text
stage1_* script names renamed = 0
stage1_* helper symbols renamed = 0
fixture paths renamed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-STAGE0-SHAPE-GATE-LABEL-WORDING-001

Status: landed quick gate label wording cleanup.

Purpose: use the concrete `GlobalCallTarget shape inventory guard` label for
the Stage0-named shape inventory script while keeping script and inventory doc
paths unchanged.

Affected file:

```text
tools/checks/lib/dev_gate_quick_steps.sh
```

Non-claims:

```text
stage0_shape_inventory_guard.sh renamed = 0
stage0-llvm-line-shape-inventory-ssot.md renamed = 0
GlobalCallTargetShape behavior changed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-HHAKO-FUNC-SCANNER-COMMENT-WORDING-001

Status: landed FuncScanner comment wording cleanup.

Purpose: use `mode-B compatibility` wording in FuncScanner comments that
describe the mode-B compiler/VM path while leaving PHI-related implementation
unchanged.

Affected files:

```text
lang/src/compiler/entry/func_scanner.hako
lang/src/compiler/entry/func_scanner_helpers.hako
```

Non-claims:

```text
FuncScanner behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
StageB* Box names renamed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-MODEB-K2-WIDE-GUARD-DIAGNOSTIC-WORDING-001

Status: landed K2-wide guard diagnostic wording cleanup.

Purpose: use `mode-B compatibility` wording in two active K2-wide guard failure
diagnostics while keeping script names, StageB Box names, and guard logic
unchanged.

Affected files:

```text
tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh
tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh
```

Non-claims:

```text
k2_wide_stageb_* script names renamed = 0
StageB* Box names renamed = 0
guard logic changed = 0
runtime behavior changed = 0
```

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

### STAGE-TERM-EXISTING-NAME-MIGRATION-001

Status: inventory-only; no implementation rename selected.

Purpose: rename existing non-bootstrap `stage` terms only after an inventory.

Inventory:

```text
docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md
```

Initial mapping:

```text
Stage-A / Stage-B compiler modes:
  mode-A / mode-B, after compatibility docs and scripts are inventoried

runner stage1:
  runner phase-1 or HHako runner phase-1, depending on owner

stage1_using_resolver:
  lower-resolver

--stage3:
  --syntax-3 or an equivalent frontend syntax-level flag

stage0/stage1/stage2 bootstrap names:
  keep only when the artifact is truly bootstrap sequencing
```

Migration must first classify each occurrence as:

```text
historical
compatibility path
bootstrap sequence
compiler mode
runner phase
lowering pass
frontend syntax level
```

Only classified non-bootstrap current names may be renamed.

#### STAGE-TERM-JOINIR-LOWERING-COMMENT-WORDING-001

Status: landed.

Scope:

```text
src/mir/join_ir/lowering/**:
  clarify comment/dev-log wording for JoinIR lowering routes
```

Decision:

```text
stage1_using_resolver comments:
  use lower-resolver compatibility wording

Stage-1 practical function comments:
  use phase-1 compatibility wording

Stage-B practical function/body/FuncScanner comments:
  use mode-B compatibility wording
```

Non-claims:

```text
stage1_using_resolver module names renamed = 0
stageb_body / stageb_funcscanner module names renamed = 0
Stage1* / StageB* Box names renamed = 0
JoinIR lowering behavior changed = 0
PHI / LocalSSA / variable-map internals touched = 0
```

#### STAGE-TERM-LANG-README-PHASE-WORDING-001

Status: landed.

Scope:

```text
lang/README.md:
  clarify current user-facing stage vocabulary as phase-1 / K2+ distribution
  wording
```

Non-claims:

```text
build_stage1.sh renamed = 0
target/selfhost/hakorune path renamed = 0
legacy stage0/stage1 boundary vocabulary removed = 0
distribution artifact behavior changed = 0
```

#### STAGE-TERM-DOCS-TOOLS-QUICK-ENTRY-WORDING-001

Status: landed.

Scope:

```text
docs/tools/README.md:
  bug-origin quick-entry route labels say phase-1

docs/tools/script-index.md:
  day-to-day selfhost/script rows say phase-1 compatibility or syntax-3
```

Non-claims:

```text
tools/selfhost/mainline/build_stage1.sh renamed = 0
tools/selfhost/compat/run_stage1_cli.sh renamed = 0
tools/selfhost/mainline/stage1_mainline_smoke.sh renamed = 0
tools/selfhost/stage3_same_result_check.sh renamed = 0
historical check-scripts ledger rewritten = 0
```

#### STAGE-TERM-STAGE1-BRIDGE-PHASE-COMMENT-WORDING-001

Status: landed.

Scope:

```text
src/runner/stage1_bridge/**:
  update README titles and Rust file-header comments to phase-1 compatibility
  wording
```

Non-claims:

```text
stage1_bridge path renamed = 0
Stage1* Rust type names renamed = 0
stage1_* modules/functions renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
[stage1-cli] log tags changed = 0
expected stderr/smoke output changed = 0
```

#### STAGE-TERM-ENV-REFERENCE-PHASE-WORDING-001

Status: landed.

Scope:

```text
docs/reference/environment-variables.md:
  active environment variable reference wording uses phase-1 compatibility,
  syntax-3, and bootstrap where the variables are compatibility surfaces
```

Non-claims:

```text
NYASH_STAGE1_* / STAGE1_* env names renamed = 0
NYASH_FEATURES=stage3 compatibility token removed = 0
NYASH_NY_COMPILER_STAGE3 env name renamed = 0
runtime behavior changed = 0
```

#### STAGE-TERM-RUST-STAGE1-ENV-HELPER-COMMENT-WORDING-001

Status: landed.

Scope:

```text
src/config/env/stage1.rs:
  helper module/file comments use phase-1 compatibility wording
```

Non-claims:

```text
stage1 module path renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
helper function names renamed = 0
runtime behavior changed = 0
```

#### STAGE-TERM-RUST-STAGE1-BOUNDARY-COMMENT-WORDING-001

Status: landed.

Scope:

```text
src/stage1/README.md:
  owner-boundary wording uses phase-1 compatibility and keeps legacy
  Stage1/Stage2 artifact labels as explicit compatibility vocabulary

src/stage1/mod.rs:
src/stage1/program_json_v0.rs:
src/stage1/program_json_v0/routing.rs:
src/stage1/program_json_v0/README.md:
  file/module header comments use phase-1 compatibility wording
```

Non-claims:

```text
src/stage1 path renamed = 0
src/stage2 path created = 0
stage1_bridge helper names renamed = 0
Program(JSON v0) behavior changed = 0
```

#### STAGE-TERM-RUST-STAGE1-PROGRAM-JSON-TEST-WORDING-001

Status: landed.

Scope:

```text
src/stage1/program_json_v0/tests/stage1_sources.rs:
src/stage1/program_json_v0/tests/classification_contract.rs:
  selected assertion / expect_err messages use phase-1 compatibility wording
```

Non-claims:

```text
test function names renamed = 0
stage1_cli_env.hako fixture path renamed = 0
Stage1* Box names renamed = 0
helper function names renamed = 0
runtime behavior changed = 0
```

#### STAGE-TERM-CHECK-SCRIPTS-INDEX-PHASE-ENV-WORDING-001

Status: landed.

Scope:

```text
docs/tools/check-scripts-index.md:
  selected active guard descriptions for selfhost surface, NyRT env P0, and
  cleanup/catch boundary use phase-1 compatibility or bootstrap wording
```

Non-claims:

```text
guard script names renamed = 0
historical check-scripts ledger broadly rewritten = 0
guard behavior changed = 0
runtime behavior changed = 0
```

#### STAGE-TERM-STAGE1-BRIDGE-ALIAS-COMMENT-WORDING-001

Status: landed.

Scope:

```text
src/runner/stage1_bridge/env/parser_stageb.rs:
  parser feature propagation comment says phase-1 compatibility alias promotion

src/runner/stage1_bridge/modules.rs:
  well-known alias comments say phase-1 compatibility CLI
```

Non-claims:

```text
stage1_bridge path renamed = 0
Stage1* Rust type names renamed = 0
stage1_* modules/functions renamed = 0
NYASH_STAGE1_* / HAKO_STAGE1_* env names renamed = 0
[stage1-cli] log tags changed = 0
runtime behavior changed = 0
```

## Guard Policy

`tools/checks/naming_charter_guard.sh` is a lightweight reusable guard. It does
not scan and rewrite old history. It verifies:

- this SSOT exists;
- the required vocabulary is present;
- the check index lists the guard;
- the quick dev gate includes the guard;
- uncommitted additions outside approved naming docs do not introduce obvious
  new unqualified `Stage*` / `stage*` terms.

The guard is a growth brake, not proof that the old repository is already
renamed.

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
project_wide_rename_completed = 0
nyash_alias_removed = 0
user_facing_docs_full_canonicalization_completed = 0
binary_primary_cutover_full_rename_completed = 0
cargo_package_rename_completed = 0
abi_helper_rename_completed = 0
stage_term_existing_migration_completed = 0
```
