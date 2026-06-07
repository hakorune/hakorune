#!/usr/bin/env python3
"""Emit FastMemory MIR-to-LLVM producer evidence from a MIR JSON file.

This is the CLI wrapper. The implementation lives in
fastmem_mir_to_llvm_producer_report_impl.py.
"""

from __future__ import annotations

from fastmem_mir_to_llvm_producer_report_impl import main


if __name__ == "__main__":
    raise SystemExit(main())
