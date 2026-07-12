# Analysis Layer

Status: Active
Scope: read-only, representation-neutral analysis artifacts.

This layer may define immutable observations, bounded traversal contracts, and
diagnostic outcomes. It must not parse source text, mutate AST/JSON, resolve
symbols, infer source types, emit MIR, allocate MIR IDs, select routes or
plans, lower backends, or execute runtime behavior.

`bounded_body_snapshot_v0` observes the accepted ProgramV0 wire quotient. It
does not own source syntax and must not recover source distinctions erased by
Program(JSON v0).
