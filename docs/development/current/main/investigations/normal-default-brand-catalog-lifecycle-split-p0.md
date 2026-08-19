# Normal Default Brand Catalog Lifecycle Split P0

Status: landed
Parent: `brand-program-declaration-catalog-d0.md`
Row: `NORMAL-DEFAULT-BRAND-CATALOG-LIFECYCLE-SPLIT-P0`
Classification: BoxShape

## Execution brief

Decision: Move the normal default catalog lifecycle's existing test module to
one bounded child before adding Brand catalog wiring; production behavior and
test inventory remain byte-for-byte equivalent in meaning.
Source authority + canonical issuer: The existing lifecycle owner remains the
sole production owner; Rust module inclusion owns only test placement.
Non-authority: File location, line count, test helper names, rustfmt, and the
future Brand catalog cannot issue behavior in this split.
Fail-fast boundary: The parent exports no new API and every moved test still
compiles against the same private owner; missing/duplicate tests or production
diff stops before the catalog I0.
Smallest next slice: Replace the inline `#[cfg(test)] mod tests` body with one
`#[cfg(test)] mod tests;` declaration and move that exact body to a child file.
Non-claims: No Brand catalog, duplicate rejection, Stage1/MIR change, resolver
loan, constructor relation, raw-route retirement, or production switch.

## Acceptance

- Parent falls below the 760-line split trigger and child remains below 760.
- Production portion of the parent is unchanged.
- Existing lifecycle tests run with the same names and outcomes.
- Pointer guard, rustfmt, diff check, and a reusable structural guard are green.

## Baseline classification

The full eight-test module has five green tests and three existing semantic
reds. The same three names and diagnostics reproduce on parent `998940a569`:

- `verified_expansion_disposition_reaches_script_and_app_root_lowering`
- `source_backed_selected_callable_uses_the_installed_package_port`
- `parser_scan_package_reaches_the_existing_physical_blocker_without_fallback`

The reusable split guard compiles the complete module and runs the five green
tests while explicitly skipping only these classified baseline debts. It does
not claim that the semantic reds are repaired by this BoxShape.
