# Normal source-plan compatibility origin

`NormalCallableCompatibilityOriginV1` in `compatibility_origin.rs` carries one already-issued parser lineage, macro
compatibility reason, and transformed compatibility `ASTNode` through the
existing request/root boundary.  Its sole co-seal issuer is the shared runner
materializer; the carrier is non-`Clone` and owns no semantic meaning beyond
transport evidence.

The prepared root keeps this carrier in an explicit `TypedCompatibility`
state.  It remains on the existing Compatibility-only lifecycle and cannot
issue a resolver package, Brand/FunctionCall target, Recipe, Join, physical
Call, fallback, or retry.  Source-free AST/JSON/VM/REPL inputs continue to use
the separate AST-only Compatibility state (`Unavailable` for this origin).

Do not reconstruct the carrier from names, ordinals, filenames, or AST shape;
do not add a second issuer.  The active contract is the
`callable-compatibility-source-transport-p0` card and the finite-state rule in
`agent-current-entry-contract-ssot.md`.
