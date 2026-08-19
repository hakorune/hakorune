# FunctionCall Special Namespace Source Registry D0

Status: closed NoSafeSlice
Scope: source/context-owned identity for existing explicit special call forms
Parent: `function-call-lexical-callee-classification-d0.md`
Row: `FUNCTION-CALL-SPECIAL-NAMESPACE-SOURCE-REGISTRY-D0`

## Final decision

Decision: One special namespace registry is NoSafeSlice because the raw arms are
not one semantic namespace; retain their distinct source authorities and retire
the raw priority chain cohort by cohort.
Source authority + canonical issuer: Grammar owns rejected `weak(...)` and
explicit `externcall`; declaration inventory owns Brand constructors; FastMem
source scope owns dialect admission. TypeOp, Math, and `str` need separate
compatibility/library decisions before they can leave the raw classifier.
Non-authority: Raw branch order, AST name/arity, Builder Brand/FastMem probes,
callable misses, tests, MIR, C, and ASM cannot unify or prioritize these meanings.
Fail-fast boundary: No aggregate registry or default Ordinary arm may reclassify
one authority as another. Each cohort must name its issuer before arguments and
remove only its own raw branch without fallback.
Smallest next slice: Close exact source-site identity for canonical explicit
`externcall`, the only currently documented explicit special call in this set.
Non-claims: No universal special enum, lexical callable capability, FreeStatic
activation, TypeOp/Math/str migration, Brand/FastMem migration, or parser rewrite.

## Census and semantic split

- `weak(...)`: canonical and Compat2025 grammar both reject it. The raw MIRBuilder
  rejection is compatibility defense, not an accepted call namespace.
- `externcall`: documented low-level explicit source capability. Its first string
  operand and remaining arguments define a distinct source form.
- Brand constructor: admitted by the declared-Brand inventory. Current precedence
  lets a Brand named `sin`, `isType`, `mem.addr`, or `str` shadow later raw arms.
- `isType` / `asType`: admitted only for exact arity and a string-shaped type
  operand; malformed shapes fall through to Ordinary today.
- Math names: a fixed builtin-name list with no grammar/source-site issuer.
- `mem.*`: special only inside an active FastMem source scope, but the raw owner
  currently observes a physical `FastMemRegionId` from Builder state.
- `str/1`: selected inside the Ordinary completion after every earlier arm.

The observed priority is
`weak > externcall > Brand > TypeOp > Math > FastMem > str/Ordinary`.
Tests prove this legacy behavior, but they do not make it language authority.

## Ordered follow-up tasks

1. `FUNCTION-CALL-EXPLICIT-EXTERNCALL-SOURCE-IDENTITY-D0`
2. `FUNCTION-CALL-WEAK-PAREN-RAW-REJECT-RETIREMENT-D0`
3. `FUNCTION-CALL-BRAND-CONSTRUCTOR-NAMESPACE-D0`
4. `FUNCTION-CALL-TYPEOP-GLOBAL-COMPAT-D0`
5. `FUNCTION-CALL-MATH-AND-STR-LIBRARY-MIGRATION-D0`
6. `FUNCTION-CALL-FASTMEM-SCOPE-IDENTITY-D0`
7. Script FreeStatic callable-index handoff after exclusions are source-issued.

Only the first row is active. The order after it may be re-audited from the new
current state; this list is not permission to batch semantic changes.
