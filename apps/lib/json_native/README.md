Layer Guard — json_native

Scope and responsibility
- This layer implements a minimal native JSON library in Ny.
- Responsibilities: scanning, tokenizing, and parsing JSON; building node structures.
- Forbidden: runtime/VM specifics, code generation, non‑JSON language concerns.

Canonical ownership
- `core/node.hako`: JSON values, factories, mutation, and serialization only.
- `parser/iterative_engine_v1.hako`: the sole text-to-tree grammar engine.
- `parser/parser.hako`: the sole public facade; it delegates without retry.
- Compatibility and strict entries share the tokenizer, engine, tree path,
  resource limit, and typed error sites. Policy owns only its admitted checks.

Imports policy (SSOT)
- Dev/CI: file-using allowed for development convenience.
- Prod: use only `nyash.toml` using entries (no ad‑hoc file imports).

Notes
- Public parser errors are MapBox projections from typed parser errors.
- Stable error contracts are kind/code/site fields; English prose is presentation.
- `JsonNode.parse` and recursive parser helpers are retired; do not recreate a
  second text-to-tree entry in Core, examples, tests, or compatibility code.
