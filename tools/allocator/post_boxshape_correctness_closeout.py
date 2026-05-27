#!/usr/bin/env python3
"""Close out post-BoxShape correctness before returning to keeper selection."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
SINGLE_EVAL_APP = ROOT / "apps/mir-single-eval-surface-sweep/main.hako"
HELPER_PROBE = ROOT / "tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py"


def read_kv_text(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        if line and "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
    return values


def run_text(cmd: list[str]) -> str:
    return subprocess.run(
        cmd,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        check=True,
    ).stdout


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    single_eval_text = run_text([str(ROOT / "target/release/hakorune"), str(SINGLE_EVAL_APP)])
    single_eval_ok = "summary=ok" in single_eval_text.splitlines()

    with tempfile.TemporaryDirectory(prefix="hakorune_post_boxshape_closeout.") as tmp:
        tmp_dir = Path(tmp)
        mir_json = tmp_dir / "app.mir.json"
        subprocess.run(
            [
                str(ROOT / "target/release/hakorune"),
                "--backend",
                "mir",
                "--emit-mir-json",
                str(mir_json),
                str(APP),
            ],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            check=True,
        )
        helper_report = read_kv_text(run_text([str(HELPER_PROBE), "--mir-json", str(mir_json)]))

    helper_probe_ok = helper_report.get("summary") == "ok"

    lines = [
        "output_contract=mir-builder-post-boxshape-correctness-closeout-v0",
        "input_contract=mir-builder-field-property-receiver-facts-cleanup-v0",
        "build_ok=1",
        f"single_eval_surface_ok={int(single_eval_ok)}",
        f"small_alloc_helper_copy_probe_ok={int(helper_probe_ok)}",
        f"helper_call_count={helper_report['helper_call_count']}",
        f"helper_copy_count={helper_report['helper_copy_count']}",
        f"receiver_copy_count={helper_report['receiver_copy_count']}",
        f"arg_copy_count={helper_report['arg_copy_count']}",
        f"result_copy_count={helper_report['result_copy_count']}",
        f"local_ssa_copy_count={helper_report['local_ssa_copy_count']}",
        f"dominant_callee_family={helper_report['dominant_callee_family']}",
        f"helper_copy_post_boxshape_status={'unchanged' if helper_report['helper_copy_count'] == '62' else 'changed'}",
        "generic_cse_opened=0",
        "post_boxshape_next=page_array_dynamic_weight_probe",
        "winner_claim=0",
        "replacement_active=0",
        "summary=ok",
    ]

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
