# Parser support boundary

This directory owns small behavior-neutral helpers used by the Hako parser
facade. Helpers here may transform already-owned scalar text, but they do not
recognize grammar, issue source identity, build resolver facts, or decode
ProgramJSON into parser truth.

`ParserBox` remains the stateful coordinator. Support boxes are stateless and
must not duplicate parser position, feature, rune, declaration, or body state.

Current owner:

```text
ParserCompatTextBox
  compatibility JSON escaping
```

Do not place rich body products, contextual `take` recognition, source
carriers, or semantic policy in this directory.
