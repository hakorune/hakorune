# 293x-220: C206c ArrayBox Inline-Record Probe Negative

Status: Complete

## Scope

C206c fixes the first negative contract for the explicit test-only
`ArrayInlineRecordProbe`:

- equal-height columns may construct an inline-record probe array;
- ragged columns are rejected with `None`;
- visible record materialization remains disabled.

## Non-Goals

C206c does not:

- enable compiler auto-use of `ArrayStorage::InlineRecord`;
- expose a public ArrayBox constructor or source-level API;
- migrate `hako_alloc` metadata stores to packed ArrayBox storage;
- add backend, `.inc`, LLVM, provider, hook, or native allocator behavior.

## Acceptance

Run:

```bash
bash tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh
```

The guard must cover both successful explicit probe construction and ragged
column rejection.
