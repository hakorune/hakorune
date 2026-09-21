---
Status: closed__2026-09-21__WarningCallableContractDispositionSyntaxDelete
Task: MIRBUILDER-WARNING-CALLABLE-CONTRACT-DISPOSITION-SYNTAX-DELETE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i92-2026-09-21.md
Implementation permission: true for deleting the unused syntax accessor only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I93
---

# Warning cleanup: callable contract disposition syntax accessor

## Six-line brief

```text
Decision: delete CallableContractSourceDispositionV1::syntax(); production
  callers already consume the enum through its canonical pattern match.
Source authority + canonical issuer: callable_contract_syntax.rs plus the
  I92 warning and repository-wide caller census.
Non-authority: a replacement accessor, AST rescan, parser semantic change,
  visibility redesign, suppression, or warning-count guess.
Fail-fast boundary: any caller, compile error, focused red, or warning-count
  mismatch rejects the deletion.
Smallest next slice: remove the one unused method and run parser contract
  tests plus the fixed warning refresh.
Non-claims: no callable-contract semantics, source ownership, ABI, or broad
  warning cleanup change.
```

## Precondition and delete set

I92 records lib **1,712** and lib-test **553**. The selected warning is the
unused method at `src/parser/callable_contract_syntax.rs:42`.
Repository-wide search found no caller of `CallableContractSourceDispositionV1::syntax()`;
the production owner consumes `OutsideDirectDeclaredInstanceMethod` and
`DirectDeclaredInstanceMethod` with direct pattern matching. The delete set is
exactly that accessor body; the enum, its issuer, and its existing source-site
contract remain.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::callable_contract_syntax -- --nocapture
```

The focused filter must execute nonzero named tests and pass. The stable
refresh must reduce lib warnings from **1,712** to **1,711**, keep lib-test at
**553** with no new diagnostics, and preserve callable-contract parser tests.
Run fmt, diff, and the current-state pointer guard before closeout. Do not
replace the deleted accessor or alter the production enum authority.

## Closeout evidence

The delete set was exactly `CallableContractSourceDispositionV1::syntax()`;
no replacement accessor or semantic authority was added. `cargo check
--profile quick --lib -j4` passed with lib **1,711** warnings,
`cargo test --profile quick --lib --no-run -j4` passed with lib-test **553**,
and the focused parser contract filter passed **6/6**. The remaining warning
surface is baseline debt; no warning suppression was introduced.

## Deletion-lane handoff

This is the first physical source deletion in the warning cohort. The next
baseline card must evaluate `Delete > Stop > Promote > 760-line split > T0`
before adding another warning facade. The proposed old-edge deletion lane is
bounded to one caller-zero source-side `route_loop_break_recipe` edge: census
all callers, delete one physical edge, run the existing or one-line absence
guard, and record build/focused/guard receipts. Non-source callers and all
other old edges remain outside that slice.
