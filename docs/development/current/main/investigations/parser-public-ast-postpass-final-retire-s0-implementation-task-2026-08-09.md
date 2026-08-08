Status: accepted execution task; implementation not landed
Date: 2026-08-09
Row: PARSER-PUBLIC-AST-POSTPASS-FINAL-RETIRE-S0
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`

# FINAL-RETIRE-S0: caller-zero helper retirement

## Scope

This is the first bounded implementation slice after FINAL-D0. It removes
only production-dead helpers proven by the top-down census:

```text
src/parser/source_gate_prune.rs
  = caller-zero legacy source-gate cursor/prune owner

NyashParser::explain_build_gate_program
  = caller-zero old explain helper in build_cfg/predicate.rs
```

The shared decision-set projection remains the sole postpass owner. The
grammar-evidence selector, `parse_from_string_with_source_seal`, the explicit
`postpass_compatibility::lower` arm, and the decision-set predicate evaluator
remain in scope and are not replaced or deleted here.

## Required changes

1. Remove the `source_gate_prune` module declaration and file after verifying
   non-test caller-zero status.
2. Remove only the unused `explain_build_gate_program` helper and its private
   recursive walker/import from `build_cfg/predicate.rs`.
3. Do not change receipt types, NoElse handling, path semantics, grammar
   evidence, compatibility lowering, public caller selection, or fallback
   behavior in this row.
4. Keep all source files below the 760-line split trigger.

## Acceptance evidence

```text
rg --glob '*.rs' --glob '!target/**' source_gate_prune src/parser
  -> no production code reference

rg --glob '*.rs' --glob '!target/**' explain_build_gate_program src/parser
  -> no production code reference

focused parser BuildCfg/postpass/source-seal tests
pointer guard
FINAL retirement guard
cargo fmt --all -- --check
git diff --check
```

The known nested member-gate source-path baseline red remains parked and must
not be weakened by this row.

## Explicit nonclaims

```text
NoElse receipt implementation
NoElse -> Else conversion policy
SourceBuildGateBranchV1 extension
grammar-evidence redesign
compatibility-arm replacement
resolver/runtime/Builder/MIR activation
public production switch
retry/reparse/fallback
```

The same implementation commit updates this card's closeout, the parser
README, the FINAL guard/index, and CURRENT_STATE. The next row is the separate
NoElse receipt implementation task.

