---
Status: ready; implementation 0
Date: 2026-08-09
Decision: BoxShape only; correct parser source-session ownership before H2 connects
Parent: `HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0`
---

# HAKO-PARSER-BOX-SOURCE-SESSION-H2-S0

## Goal

Replace the disconnected H1 caller-token/global-member-cursor proof shape with
the smallest durable parser-private session boundary:

```text
one program parse
  -> one parser invocation brand
  -> ordered Box source sites
  -> one fresh member cursor per exact Box
```

This row does not connect the ordinary Box parser and does not parse `take`.
It changes no accepted source shape.

## Structural implementation

Keep the owner in `lang/src/compiler/parser/source_carrier_v1/` and extend the
existing declaration source vocabulary rather than creating a second carrier.
Prefer a focused file such as:

```text
parser_source_session_v1.hako
```

The public shape inside this parser-private module is:

```text
ParserProgramSourceSessionV1
  -> next_box_site()
  -> open_member_cursor(exact_box_site)
  -> finish()

ParserBoxMemberSourceCursorV1
  -> next_direct_member_site()
  -> finish()
```

The cursor owns exactly one Box site and begins at member ordinal zero.
Selected build-gate paths remain closed. Production parsing remains
disconnected in S0; the H1 fixture is updated to exercise the corrected owner.

## Required invariants

```text
one program session owns one invocation brand
Box statement ordinals are program-scoped and ordered
member ordinals are Box-scoped and reset for each Box
cursor cannot issue a site for a foreign Box/session
program session cannot finish with a live cursor
cursor/session finish exactly once
no issuance or mutation after finish
failure publishes no partial declaration seal
```

The old invocation-wide `_next_member` path must not remain as a competing
source-site authority. Compatibility factories may remain only when the guard
shows they cannot enter the future connected H2 branch and their retirement
condition is recorded.

## Files and size boundary

- do not grow the existing 787-line `parser_box.hako`;
- keep every touched/new Hako source below 800 lines;
- update `source_carrier_v1/README.md` in this commit;
- extend the existing H1 guard rather than adding a second source-authority
  manifest unless a focused S0 guard is materially clearer.

## Focused acceptance

```text
positive one-program/two-Box session
both Boxes issue member ordinal 0 for their first member
second member in one Box receives ordinal 1
foreign Box cursor use rejects
foreign session/site rejects
double cursor finish rejects
program finish with live cursor rejects
double program finish and post-finish issuance reject
existing one-Box H1 declaration seal remains green
all touched source files < 800 lines
```

Required commands:

```bash
bash tools/checks/hako_parser_box_declaration_h1_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Add the smallest focused fixture/guard command to the source-carrier README if
the H1 guard cannot express the two-Box session matrix cleanly.

## Nonclaims

```text
ordinary Box parser connection
parameter list or type_ref parsing
Take/Ordinary parameter rows
rich body product
final H3 seal changes
selected build-gate/generated/delegate support
Rust/Hako parity
resolver/Home/Recipe/Builder/MIR/runtime
new language acceptance
```

## Done

- [ ] program-owned session is the sole invocation-brand issuer;
- [ ] member cursors are exact-Box-scoped and reset per Box;
- [ ] invocation-wide member ordinal authority is removed or quarantined;
- [ ] focused positive/negative tests are green;
- [ ] owner README and guard encode the boundary;
- [ ] current pointers name `H2-S1` only after this receipt closes;
- [ ] implementation, tests, docs, and pointer closeout land together.
