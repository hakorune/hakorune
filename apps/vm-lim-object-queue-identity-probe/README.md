# VM-LIM-001 object queue identity probe

Decision: accepted as a diagnostic probe, not an acceptance guard.

This app intentionally exercises the object-heavy VM route suspected by
`VM-LIM-001`:

```text
ArrayBox.push(page object)
ArrayBox.get(0)
returned page object becomes method receiver
```

The mimalloc lane must not depend on this route being VM-green. MIMAP-011+ uses
LLVM/EXE primary acceptance for object-heavy page/facade routes.
