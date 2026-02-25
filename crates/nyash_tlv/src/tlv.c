#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// Minimal C shim: identity round‑trip (copy input to a new buffer).
// Returns size written to *out_ptr. Caller must free via ny_tlv_free.
size_t ny_tlv_identity(const uint8_t* in_ptr, size_t len, uint8_t** out_ptr) {
  if (!in_ptr || !out_ptr) return 0;
  uint8_t* buf = (uint8_t*)malloc(len == 0 ? 1 : len);
  if (!buf) return 0;
  if (len > 0) memcpy(buf, in_ptr, len);
  *out_ptr = buf;
  return len;
}

void ny_tlv_free(uint8_t* ptr) {
  if (ptr) free(ptr);
}

