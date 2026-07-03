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

### HAKORUNE-BINARY-PRIMARY-CUTOVER-INVENTORY-001

Status: active in this slice.

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
  default-run = absent
  next safe cut = HAKORUNE-BINARY-DEFAULT-RUN-CUTOVER-001

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

### STAGE-TERM-EXISTING-NAME-MIGRATION-001

Status: defined, not implementation.

Purpose: rename existing non-bootstrap `stage` terms only after an inventory.

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
