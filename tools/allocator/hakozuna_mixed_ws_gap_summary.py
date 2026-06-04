"""Summarize Hakozuna mixed-ws compare reports into a short gap ladder."""

from __future__ import annotations

import argparse
from pathlib import Path

from hakozuna_mixed_ws_gap_summary_report import emit_summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("compare_report", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    report = emit_summary(args.compare_report)
    args.out.write_text(report, encoding="utf-8")
    print(report, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
