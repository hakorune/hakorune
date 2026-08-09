# Hako selfhost statement parser boxes

This directory owns small statement-syntax classifiers for the Hako parser.
It does not resolve bindings, issue Home capabilities, infer effects, or lower
MIR.

`release_source.hako` recognizes only same-line `release IDENT` with spaces or
tabs between the words. Parenthesized calls and newline-separated spellings
remain ordinary syntax. Once the exact-root statement shape is committed,
projections, calls, receiver keywords, and trailing expressions fail with
`parser/release_exact_root_required`. Its JSON is descriptive parity evidence,
not a semantic or executable release plan.
