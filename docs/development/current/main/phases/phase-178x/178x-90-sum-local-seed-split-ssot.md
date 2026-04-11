# Phase 178x SSOT: sum local seed split

## Goal

Reduce `lang/c-abi/shims/hako_llvmc_ffi_sum_local_seed.inc` from a monolithic 2k-line seed file to a facade plus focused include units, while preserving current pure-first variant/local behavior exactly.

## Why Now

- the file is already larger than the current string exact bridge surface
- current variant/local seeds are semantically stable enough that structure cleanup can happen without changing contracts
- this cleanup is a prerequisite for later generic bridge shrink because it isolates:
  - metadata reading
  - exact IR emitters
  - tag matcher families
  - project matcher families

## Contract

The split is allowed to change only physical structure.

It is not allowed to change:

- function names exported within the current compilation unit
- `compile_json_compat_pure()` match order
- current metadata authority:
  - `thin_entry_selections`
  - `sum_placement_facts`
  - `sum_placement_selections`
  - `sum_placement_layouts`
- current direct emit behavior for existing variant/local keepers

## Target Shape

Keep:

- `hako_llvmc_ffi_sum_local_seed.inc`

Split under it:

- `hako_llvmc_ffi_sum_local_seed_metadata_helpers.inc`
- `hako_llvmc_ffi_sum_local_seed_emitters.inc`
- `hako_llvmc_ffi_sum_local_seed_matchers_tag.inc`
- `hako_llvmc_ffi_sum_local_seed_matchers_project_copy.inc`
- `hako_llvmc_ffi_sum_local_seed_matchers_project_local.inc`

Dependency order:

1. metadata helpers
2. emitters
3. tag matchers
4. project matchers

## Do Not Do

- do not move pure-compile dispatch rows in this phase
- do not merge variant metadata helpers into string/generic helper files
- do not add new benchmark-specific matchers
- do not rewrite the matchers into a new metadata DSL in this phase

## Acceptance

- facade include is visibly thin
- current target routes still compile
- focused variant/local smoke or targeted build checks stay green
