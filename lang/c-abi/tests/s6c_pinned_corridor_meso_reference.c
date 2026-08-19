#include <stdint.h>

#if defined(__GNUC__) || defined(__clang__)
#define HAKO_NOINLINE __attribute__((noinline, aligned(64)))
#else
#define HAKO_NOINLINE
#endif

static uint64_t width_at(const uint8_t* text) {
  if (text[0] < 0x80) return 1;
  if (text[0] < 0xe0) return 2;
  if (text[0] < 0xf0) return 3;
  return 4;
}

static uint8_t scalar_eq(
    const uint8_t* lhs, uint64_t width, const uint8_t* rhs, uint64_t rhs_len) {
  if (width != rhs_len || width < 1 || width > 4) return 0;
  if (lhs[0] != rhs[0]) return 0;
  if (width == 1) return 1;
  if (lhs[1] != rhs[1]) return 0;
  if (width == 2) return 1;
  if (lhs[2] != rhs[2]) return 0;
  if (width == 3) return 1;
  return lhs[3] == rhs[3];
}

HAKO_NOINLINE int64_t hako_s6c_c_meso(
    const uint8_t* subject,
    uint64_t subject_len,
    const uint8_t* needle,
    uint64_t needle_len) {
  uint64_t offset = 0;
  int64_t index = 0;
  while (offset < subject_len) {
    uint64_t width = width_at(subject + offset);
    if (scalar_eq(subject + offset, width, needle, needle_len)) return index;
    offset += width;
    index++;
  }
  return -1;
}
