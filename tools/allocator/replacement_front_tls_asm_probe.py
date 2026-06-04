#!/usr/bin/env python3
"""Inspect replacement-front TLS malloc/free assembly shape.

This is a report-only probe. It reads an already-built replacement-front shared
library through objdump and classifies the hot malloc/free metadata shape; it
does not build, preload, or execute the allocator.
"""

from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path


def objdump_text(shared_library: Path) -> str:
    return subprocess.check_output(
        [
            "objdump",
            "-d",
            "--demangle",
            "--section=.text",
            str(shared_library),
        ],
        text=True,
    )


def function_block(text: str, name: str) -> list[str]:
    pattern = re.compile(rf"^[0-9a-f]+ <{re.escape(name)}>:$")
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if not pattern.match(line.strip()):
            continue
        block: list[str] = []
        for block_line in lines[index + 1 :]:
            if re.match(r"^[0-9a-f]+ <[^>]+>:$", block_line.strip()):
                break
            if block_line.strip():
                block.append(block_line)
        return block
    return []


def count_contains(lines: list[str], needle: str) -> int:
    return sum(1 for line in lines if needle in line)


def has_any(lines: list[str], needles: tuple[str, ...]) -> int:
    return int(any(needle in line for line in lines for needle in needles))


def render_report(shared_library: Path) -> str:
    text = objdump_text(shared_library)
    malloc_lines = function_block(text, "malloc")
    free_lines = function_block(text, "free")
    malloc_fs_ref_count = count_contains(malloc_lines, "%fs:")
    free_fs_ref_count = count_contains(free_lines, "%fs:")
    malloc_stack_frame = has_any(malloc_lines, ("sub    $0x10,%rsp", "push   %rbp"))
    free_stack_frame = has_any(free_lines, ("sub    $0x8,%rsp", "push   %rbp"))
    malloc_requested_size_store = has_any(malloc_lines, ("mov    %rdi,%fs:",))
    free_requested_size_clear = has_any(free_lines, ("movq   $0x0,%fs:",))
    free_magic_division = has_any(
        free_lines,
        (
            "0xfc0fc0fc0fc0fc1",
            "0x3f03f03f03f03f",
            "imul",
            "mul",
        ),
    )
    free_remote_registry_path = has_any(
        free_lines,
        ("arena_registry", "pthread_mutex_lock", "lock cmpxchg"),
    )
    selected_owner = "none"
    selected_next_action = "none"
    if free_magic_division:
        selected_owner = "thread_local_replacement_front_free_slot_index_decode"
        selected_next_action = (
            "probe free slot-index decode shape before retrying metadata-store changes"
        )
    elif malloc_requested_size_store:
        selected_owner = "thread_local_replacement_front_malloc_requested_size_store"
        selected_next_action = "do_not_retry_without_new_use-site evidence"

    lines = [
        "output_contract=replacement-front-tls-asm-probe-v0",
        f"shared_library={shared_library}",
        "probe_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        f"malloc_symbol_present={int(bool(malloc_lines))}",
        f"free_symbol_present={int(bool(free_lines))}",
        f"malloc_instruction_count={len(malloc_lines)}",
        f"free_instruction_count={len(free_lines)}",
        f"malloc_fs_ref_count={malloc_fs_ref_count}",
        f"free_fs_ref_count={free_fs_ref_count}",
        f"malloc_stack_frame={malloc_stack_frame}",
        f"free_stack_frame={free_stack_frame}",
        f"malloc_requested_size_store={malloc_requested_size_store}",
        f"free_requested_size_clear={free_requested_size_clear}",
        f"free_slot_index_magic_division={free_magic_division}",
        f"free_remote_registry_path={free_remote_registry_path}",
        f"selected_owner={selected_owner}",
        f"selected_next_action={selected_next_action}",
        "summary=ok",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--shared-library", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    shared_library = args.shared_library.resolve()
    if not shared_library.is_file():
        raise SystemExit(f"missing shared library: {shared_library}")
    report = render_report(shared_library)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
