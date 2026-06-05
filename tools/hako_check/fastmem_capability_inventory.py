#!/usr/bin/env python3
"""Thin CLI wrapper for FastMemory capability inventory.

The implementation lives in `fastmem_capability_inventory_impl.py` so the
entrypoint stays small and easy to navigate.
"""

from fastmem_capability_inventory_impl import *  # noqa: F401,F403


if __name__ == "__main__":
    raise SystemExit(main())
