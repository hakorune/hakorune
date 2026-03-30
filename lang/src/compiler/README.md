# Hakorune Compiler — Layout and Responsibilities

Pointers:
- selfhost compiler ownership map (repo-wide SSOT):
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- file-level responsibility inventory:
  - `docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md`
- current bootstrap/authority contract:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`

Structure (target)
- emit/
  - mir_emitter_box.hako — high-level MIR emitter entry
  - common/ — shared emit helpers (mir_emit/json_emit/call_emit/header_emit/newbox_emit)
- parser/ — lexer/parser (to be moved from apps/* in later steps)
- builder/, ssa/, rewrite/, pipeline_v2/ — existing compiler stages (move gradually)

Policy
- Compiler lives under `lang/src/compiler/`.
- VM engines live under `lang/src/vm/engines/` (Hakorune/Mini), with shared helpers in `vm/boxes/`.
- Keep imports across these boundaries minimal and documented.
- `BuildBox.emit_program_json_v0(...)` is the current `source -> Program(JSON v0)` authority.
- `entry/compiler_stageb.hako` is the Stage-B emit/adapter lane and should shrink toward entry-only behavior instead of acting like a second authority.

Grammar Notes (parser parity)
- Semicolons are accepted as optional statement separators (default ON).
  - Both newline and `;` delimit statements; trailing `};` is allowed.
  - Consecutive `;;` are treated as empty statements (no-op).
  - Env toggle (opt-out): set `NYASH_PARSER_ALLOW_SEMICOLON=0|false|off` to disable.
