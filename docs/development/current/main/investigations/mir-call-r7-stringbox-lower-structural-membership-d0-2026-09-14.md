---
Status: accepted__hako_structural_membership__2026-09-14
Task: MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-D0
Date: 2026-09-14
Priority: replace unbounded StringBox recognition with one Hako-owned structural contract
Parent: mir-call-r7-stringbox-caller-switch-d0-2026-09-14.md
NextCard: MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-I0
Implementation permission: true for the Hako lowerer structural-membership I0 only
---

# StringBox lower structural membership D0

## Six-line brief

```text
Decision: replace LowerReturnMethodStringLengthBox's first-string and unbounded class scans with exact JSON object/array membership.
Source authority + canonical issuer: Hako Program(JSON v0) lowerer and its existing direct-call and New(StringBox) boxcall recipes.
Non-authority: first matching string token, method spelling alone, JSON/MIR indices, Rust artifact reconstruction, registry order, fallback, and retry.
Fail-fast boundary: malformed or structurally mismatched Return/Method/receiver/constructor/args returns null before any MIR string is emitted.
Smallest next slice: exact direct/New StringBox length|size plus the required phase17 New(StringBox).indexOf one-argument compatibility shape.
Non-claims: no Rust artifact transport, source-entry cutover, shared-schema retirement, backend parity, or whole-R7 caller-zero.
```

## Bounded census

The boundary is the Hako `Program(JSON v0)` lowerer
`LowerReturnMethodStringLengthBox.try_lower`, its three existing MIR emitters,
and the registry/fallback callers that retain body-only compatibility. It
includes one `Return` expression and the receiver/method/argument objects that
belong to it. It excludes source parsing, Rust source-artifact issuance,
Program(JSON) caller migration, direct MIR schema readers, and backend work.

| Input shape | Hako disposition |
| --- | --- |
| `Return(Method(Str|String literal, length|size, []))` | existing canonical `const -> call(Method) -> ret` recipe |
| `Return(Method(New(StringBox([one direct string literal, no extra fields]), length|size, []))` | existing `const -> newbox -> boxcall -> ret` recipe |
| Exact `New(StringBox(...)).indexOf(one direct string literal)` with one method argument | existing phase17 `boxcall` compatibility recipe |
| `indexOf` with zero, multiple, embedded, or nonliteral arguments | `null` before emission |
| Extra constructor args, field initializers/type arguments, wrong receiver/class, or nonempty length/size args | `null` before emission |
| Empty direct string literal | accepted in the same selected direct/New row |
| Malformed, incomplete, or unrecognized JSON | `null`, preserving the caller's existing next-authority behavior |

The predicate must validate every lookup inside its owning JSON object or
array. A matching token elsewhere in the document cannot satisfy a row.

## Authority and structural contract

The Hako Program(JSON v0) lowerer remains the sole authority for this
compatibility recipe. The existing `JsonCursorBox.seek_obj_end` and
`seek_array_end` delimiters, together with the fragment readers, may be reused
only as bounded readers. No source artifact, JSON/MIR index, or registry order
may be used as semantic evidence.

The selected predicate must prove, before calling an emitter:

1. the owning `Return.expr` is exactly one `Method` object;
2. the receiver is exactly a direct `Str`/`String` literal or one `New` object;
3. a `New` receiver has exactly class `StringBox`, one direct string
   constructor argument, and no extra constructor metadata, field initializer,
   or type argument;
4. the selector is `length`, `size`, or the required `indexOf` shape;
5. `length` and `size` have an empty method-argument array;
6. `indexOf` has exactly one direct string literal argument.

The three current emitters remain the canonical recipes:
`_emit_length_mir`, `_emit_size_mir`, and
`_emit_new_stringbox_boxcall0`/`_emit_new_stringbox_boxcall1_string`. The
structural predicate chooses a recipe; it does not infer a new one or rewrite
the emitted MIR.

## Authorized I0 task order

1. Freeze the membership table above and map each selected row to its existing
   emitter and terminal output.
2. Add bounded receiver, constructor, method-selector, and method-argument
   readers using object/array end positions owned by the current node.
3. Replace `_read_first_string_literal_between` and
   `_read_method_first_string_arg`; remove the unbounded `k_class` search and
   first-string receiver extraction from `try_lower`.
4. Preserve the direct `length`/`size`, New-box `length`/`size`, and phase17
   `indexOf` output shapes, including empty literals.
5. Add Hako-owned positive/negative evidence, then update the MirBuilder
   lowerer README and the current MIR call reference in the same bounded
   implementation series.

## Exact cleanup and retained edges

The I0 delete set is limited to the old recognition responsibility:

- `_read_first_string_literal_between`;
- `_read_method_first_string_arg`;
- the unbounded `k_class` search and first-string receiver extraction in
  `try_lower`;
- any equivalent substring-based membership branch introduced by that scan.

Keep the three emitters, the Hako registry and fallback owners, body-only
`parse(&str)`, explicit Program(JSON) entry points, `indexOf` outside the exact
phase17 row, and all unrelated lowerers. The caller-switch D0 has no deletion
tuple for this card; a later caller row may delete an edge only after a live
source/terminal route and retained body-only callers are separately proven.

## D0 acceptance

```text
the finite direct/New length|size/indexOf membership and null cases are fixed
every lookup is bounded to its owning Return/Method/receiver/args object or array
the existing call and boxcall recipes and phase14/17 results are preserved
malformed, wrong-class, extra-argument, embedded, and opaque shapes emit no partial MIR
the I0 delete set removes first-string and unbounded-class recognition only
registry/fallback/body-only callers remain named compatibility owners
one positive, one negative, and one structural no-reentry guard are executable
README/reference/pointer updates are listed for the implementation closeout
```

CI-independent design evidence is sufficient for this stop. Cargo or a remote
workflow is not a prerequisite for accepting the contract; I0 must run one
focused Hako path and record any red under the existing classified-red rules.

## Non-claims

The Rust source-artifact I0 at `6adbfa8abb` remains comparison evidence and is
not promoted to a Hako issuer. The caller-switch D0 is retained/closed with an
empty delete set. This card does not switch `MirBuilderBox` callers, remove
the shared `LegacyCallV0` schema, delete registry/fallback compatibility, add a
new fallback or retry, migrate `indexOf` generally, or claim whole-MirBuilder
completion.

## D0 decision

The membership table, bounded-reader contract, recipe mapping, exact cleanup
set, and retained caller inventory are accepted. The successor I0 may edit
`lang/src/mir/builder/internal/lower_return_method_string_length_box.hako`, its
focused Hako evidence, and the owning README/reference. It may not switch the
Hako caller, transport the Rust artifact, or remove registry/fallback owners.
