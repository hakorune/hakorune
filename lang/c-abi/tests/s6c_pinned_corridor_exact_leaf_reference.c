#include <stdint.h>

#if defined(__GNUC__) || defined(__clang__)
#define HAKO_NOINLINE __attribute__((noinline))
#else
#define HAKO_NOINLINE
#endif

HAKO_NOINLINE uint8_t hako_s6c_c_exact_leaf(
    const uint8_t* subject,
    uint64_t offset,
    uint64_t width,
    const uint8_t* needle,
    uint64_t needle_len) {
  if (width != needle_len || width < 1 || width > 4) return 0;
  switch (width) {
    case 1:
      return subject[offset] == needle[0];
    case 2:
      return subject[offset] == needle[0] &&
          subject[offset + 1] == needle[1];
    case 3:
      return subject[offset] == needle[0] &&
          subject[offset + 1] == needle[1] &&
          subject[offset + 2] == needle[2];
    case 4:
      return subject[offset] == needle[0] &&
          subject[offset + 1] == needle[1] &&
          subject[offset + 2] == needle[2] &&
          subject[offset + 3] == needle[3];
    default:
      return 0;
  }
}
