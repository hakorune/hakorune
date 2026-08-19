# Brand Declaration Namespace and Result Contract D1

Status: selected design stop
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
