# Brand Declaration Namespace and Result Contract D1

Status: accepted
Parent: `function-call-brand-constructor-namespace-d0.md`
Row: `BRAND-DECLARATION-NAMESPACE-AND-RESULT-CONTRACT-D1`

## Question

Complete the accepted BRAND-002 language contract before implementation:

1. Are top-level Brand declarations program-wide regardless of source order?
2. Are duplicate Brand names rejected, or is one explicit declaration selected?
3. Does a declared Brand constructor take precedence over same-named FreeStatic,
   TypeOp, Math, and `str` spellings?
4. Which semantic product preserves Brand identity when physical MIR reuses the
   underlying scalar representation?
5. Must constructor and unwrap be issued from one shared catalog and excluded
   from ordinary direct-call resolution at the exact source site?

This requires design consultation because BRAND-002 says “branded scalar” and
Stage1-owned semantic nodes, while current MIRBuilder erases the Brand identity
and current resolver has no Brand namespace input. No code, fixture, catalog,
new receipt, or raw retirement is authorized until one six-line Decision fixes
all five points and names the first bounded executable row.

## Decision brief

Decision: Use one program-wide effective Brand namespace after gate pruning;
reject duplicate names, give declared Brand sites exclusive precedence, and
retain nominal Brand identity even when physical storage is the underlying scalar.
Source authority + canonical issuer: One AST-free catalog issued from effective
top-level Brand declarations owns identity; resolver issues constructor/unwrap
relations from the same catalog at exact sites.
Non-authority: Source order, last-write maps, raw classifier priority, FreeStatic
misses, Stage1-private maps, underlying strings, Program JSON, and ValueId/MIR
representation cannot issue Brand meaning.
Fail-fast boundary: Duplicate declarations reject before resolution; exact
membership and arity exclude FreeStatic/special rows before arguments, and
foreign/missing relations never fall back.
Smallest next slice: `BRAND-GRAMMAR-DECLARATION-CAPSULE-I0` registers only the
Brand declaration spelling and adds the missing metadata-only Hako witness
capsule; contextual constructor and unwrap remain semantic relations, not
duplicate grammar rows.
Non-claims: No catalog/site product, MIR opcode/type, conversion checker,
Program-JSON bridge, raw cutover, runtime wrapper, or production switch.

## Accepted answers

1. Visibility is program-wide after build-gate pruning.
2. Duplicate effective names reject with `[brand/duplicate-declaration]`.
3. A declared Brand exclusively owns the bare constructor site over FreeStatic,
   TypeOp, Math, and `str`; dedicated externcall and impossible dotted names are
   outside the collision set.
4. The semantic product carries declaration identity, name, and underlying type;
   physical scalar reuse is representation only.
5. Constructor and unwrap use one catalog and exact site relations, and Brand
   sites are absent from the ordinary direct-call ledger.
