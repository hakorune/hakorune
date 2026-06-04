"""Raw C sources for provider-backed LD_PRELOAD replacement smoke probes."""

from __future__ import annotations

from provider_package_ldpreload_replacement_shim_source import SHIM_C

SMOKE_C = r"""
#include <stdlib.h>
#include <string.h>

int main(void) {
  unsigned char* p = (unsigned char*)malloc(32);
  if (!p) return 2;
  memset(p, 0xA5, 32);
  unsigned char* q = (unsigned char*)calloc(4, 8);
  if (!q) return 3;
  for (int i = 0; i < 32; i++) {
    if (q[i] != 0) return 4;
  }
  unsigned char* r = (unsigned char*)realloc(p, 64);
  if (!r) return 5;
  for (int i = 0; i < 32; i++) {
    if (r[i] != 0xA5) return 6;
  }
  free(q);
  free(r);
  return 0;
}
"""
