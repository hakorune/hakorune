# Stage1 Bridge Child Env

Scope: child environment wiring under `src/runner/stage1_bridge/env.rs`.

## Sections

- `env.rs`: facade entrypoint (`configure_stage1_env(...)`) only
- `runtime_defaults.rs`: runtime defaults and mainline MIR builder locks
- `stage1_aliases.rs`: `NYASH_STAGE1_*` propagation, child guard, entry/backend alias handling
- `parser_stageb.rs`: parser feature propagation and `HAKO_STAGEB_APPLY_USINGS` / using toggles
- `../modules.rs`: `HAKO_STAGEB_MODULES_LIST` / `HAKO_STAGEB_MODULE_ROOTS_LIST` payload generation + child-env apply

## Forbidden

- mode inference outside `args.rs`
- backend CLI hint parsing outside `args.rs`
- raw argv window parsing here
- `Program(JSON v0)` parse/lower policy here
- rebuilding command-line routing policy inside child-env helpers
