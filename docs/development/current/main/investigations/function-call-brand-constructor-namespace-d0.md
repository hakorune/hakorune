# FunctionCall Brand Constructor Namespace D0

Status: closed NoSafeSlice
Parent: `function-call-special-namespace-source-registry-d0.md`
Row: `FUNCTION-CALL-BRAND-CONSTRUCTOR-NAMESPACE-D0`

## Question

Can the declared-Brand inventory issue exact constructor membership before raw
argument effects, so the Builder stops classifying constructors by a late name
probe without changing Brand shadowing or arity diagnostics?

Audit declaration/source identity, resolver products, constructor arity and
result type, collisions with TypeOp/Math/FastMem/str, and every production
consumer. Name the one source authority and fail-fast boundary before selecting
any BoxCount or BoxShape implementation.

This is not Fast path because the current raw Builder probe may be both Brand
membership and collision priority authority; independent read-only premise
audits must establish the existing source issuer and every competing arm before
an implementation row is selected.

## Decision brief

Decision: Do not replace/delete the raw Brand branch yet. The repository has no
single Brand declaration catalog or site-keyed constructor relation; current
Stage1 and MIRBuilder independently rescan source and resolver treats every
ordinary FunctionCall as a FreeStatic candidate.
Source authority + canonical issuer: The accepted BRAND-002 policy makes
top-level Brand declarations and Stage1-owned `BrandConstruct`/`BrandUnwrap`
the semantic authority, but a shared AST-free catalog issuer is still missing.
Non-authority: `CompilationContext::brand_decls`, `is_brand_declared`, raw
classifier order, Stage1's private BTreeMap, callable lookup misses, forged
`mem.addr` tests, underlying-type strings, and transparent MIR ValueIds.
Fail-fast boundary: Declared membership, collision disposition, and exact-one
arity must be fixed before argument descent; Brand and FreeStatic rows may not
both be issued and no lookup miss may fall back into another special arm.
Smallest next slice: `BRAND-DECLARATION-NAMESPACE-AND-RESULT-CONTRACT-D1`
fixes duplicates, program visibility, collision precedence, and the semantic
Brand-result contract before any catalog or consumer code is added.
Non-claims: No parser AST change, catalog/receipt, MIR type, Stage1/JSON bridge,
raw branch deletion, fallback, nominal coercion, or production switch.

## Census result

- `PreparedNormalProgramDeclarationFactsV1` collects Brand declarations but is
  consumed into mutable `CompilationContext.brand_decls`; it exposes no Brand
  demand view and retains no call-site relation.
- Stage1 separately builds `BTreeMap<name, underlying>` and emits
  `BrandConstruct`/`BrandUnwrap`; the JSON-v0 bridge does not currently consume
  those expression variants.
- Current raw lowering gives Brand first priority, rejects arity other than one
  before children, then returns the sole child's ValueId without publishing a
  nominal Brand fact.
- Natural collisions exist with FreeStatic, TypeOp names, Math names, and
  `str`; dotted `mem.addr` is not a legal Brand identifier and is only a forged
  raw-AST test condition.
- Duplicate declarations are currently last-write-wins by map behavior, not an
  explicit language Decision.

## Ordered task ladder

1. `BRAND-DECLARATION-NAMESPACE-AND-RESULT-CONTRACT-D1` — language Decision.
2. `BRAND-GRAMMAR-REGISTRY-CLOSEOUT-R0` — registry/corpus synchronization.
3. `BRAND-PROGRAM-DECLARATION-CATALOG-I0` — one AST-free program catalog.
4. `BRAND-CONSTRUCTOR-SOURCE-RELATION-I0` — exact site relation before effects.
5. `BRAND-CONSTRUCTOR-CONSUMER-CUTOVER-R0` — consume relation and retire raw
   probe/variant in one BoxShape.
6. `BRAND-LEGACY-MAP-RETIREMENT-R0` — remove remaining duplicate maps only at
   caller-zero.

BoxCount catalog/site activation and BoxShape raw retirement must not be mixed.
