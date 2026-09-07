#!/usr/bin/env python3
"""Link the real selected archive to an ABI stub; no compiler-body claim."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile

archive = Path(sys.argv[1]).resolve()
assert archive.is_file(), archive


def symbols(path):
    result = subprocess.run(["llvm-nm-18", "-g", "--defined-only", str(path)],
                            check=True, text=True, capture_output=True)
    return [line.split()[-1] for line in result.stdout.splitlines() if len(line.split()) == 3]

names = symbols(archive)
assert names.count("main") == 1, "lifecycle archive must define exactly one main"
assert names.count("nyash_lifecycle_entry_abi_v1") == 1
assert names.count("nyash_runtime_abi_descriptor_v1") == 1
print("lifecycle archive: one main, one entry ABI, one runtime ABI")
cases = [(archive, value, status, status) for value, status in
         [(0, 0), (30, 30), (255, 255), (-1, 70), (256, 70)]]
if len(sys.argv) > 2:
    legacy = symbols(Path(sys.argv[2]).resolve())
    assert legacy.count("main") == 1, "legacy archive must define exactly one main"
    assert "nyash_lifecycle_entry_abi_v1" not in legacy
    print("legacy archive: one main, no lifecycle entry ABI")
    cases.extend((Path(sys.argv[2]).resolve(), value, value % 256, value)
                 for value in [0, 30, 255, -1, 256])

with tempfile.TemporaryDirectory(prefix="nyash-launcher-") as directory:
    work = Path(directory)
    for selected_archive, value, expected, printed in cases:
        source = work / "entry.c"
        source.write_text('''#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
static int calls;
static void verify(void) {
    if (calls != 1) _Exit(99);
    puts("ENTRY_CALLS=1");
}
int64_t ny_main(void) {
    if (calls++ != 0) _Exit(99);
    if (atexit(verify) != 0) _Exit(98);
    return VALUE;
}
'''.replace("VALUE", str(value)))
        exe = work / "entry"
        subprocess.run(["cc", str(source), str(selected_archive), "-lpthread", "-ldl", "-lm", "-o", str(exe)], check=True)
        env = dict(os.environ)
        env.update({
            "NYASH_NYRT_RING0_INIT": "off",
            "NYASH_NYRT_ENTRY_PATH_PREP": "off",
            "NYASH_NYRT_RUNTIME_BUILD": "off",
            "NYASH_NYRT_RUNTIME_HOOKS": "off",
            "HAKO_NYRT_PLUGIN_HOST": "off",
            "NYASH_GC_METRICS": "0",
            "NYASH_GC_METRICS_JSON": "0",
            "NYASH_NYRT_SILENT_RESULT": "0",
        })
        result = subprocess.run([str(exe)], env=env, text=True, capture_output=True)
        assert result.returncode == expected, (value, result)
        assert result.stdout.count("ENTRY_CALLS=1") == 1, result
        assert f"Result: {printed}\n" in result.stdout, result
        print(f"{selected_archive.name}: {value} -> {expected}; calls=1")
