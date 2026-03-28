#include <stdint.h>
#include <string.h>

int main(void) {
  const int rows = 64;
  const int64_t ops = 180000;
  const char seed[] = "line-seed-abcdef";
  const int len = (int)(sizeof(seed) - 1);
  char src[64][32];
  char dst[64][40];
  volatile int64_t total = 0;

  for (int i = 0; i < rows; i++) {
    memcpy(src[i], seed, (size_t)len + 1);
    dst[i][0] = '\0';
  }

  for (int64_t i = 0; i < ops; i++) {
    const int row = (int)(i % rows);
    const int split = len / 2;
    memcpy(dst[row], src[row], (size_t)split);
    dst[row][split] = 'x';
    dst[row][split + 1] = 'x';
    memcpy(dst[row] + split + 2, src[row] + split, (size_t)(len - split));
    dst[row][len + 2] = '\0';
    total += (int64_t)(len + 2);
  }

  return (int)((total + rows) & 0xFF);
}
