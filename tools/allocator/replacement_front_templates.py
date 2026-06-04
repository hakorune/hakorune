"""Replacement-front benchmark templates and facade exports."""

from __future__ import annotations

from replacement_front_bins_templates import generate_replacement_front_bins_shim_c
from replacement_front_shim_templates import REPLACEMENT_FRONT_SHIM_C
from replacement_front_smoke_templates import (
    REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C,
    REPLACEMENT_FRONT_MALLOC_FAMILY_SMOKE_C,
)
