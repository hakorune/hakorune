#include <stdint.h>
#include <string.h>

int main(void) {
  const int rows = 64;
  const int64_t ops = 180000;
  const char seed[] = "line-seed-abcdef";
  const int len = (int)(sizeof(seed) - 1);
  char lines[64][32];
  volatile int64_t total = 0;

  for (int i = 0; i < rows; i++) {
    memcpy(lines[i], seed, (size_t)len + 1);
  }

  for (int64_t i = 0; i < ops; i++) {
    const int row = (int)(i % rows);
    const int split = len / 2;
    char out[40];
    memcpy(out, lines[row], (size_t)split);
    out[split] = 'x';
    out[split + 1] = 'x';
    memcpy(out + split + 2, lines[row] + split, (size_t)(len - split));
    out[len + 2] = '\0';

    memcpy(lines[row], out + 1, (size_t)len);
    lines[row][len] = '\0';
    total += (int64_t)len;
  }

  return (int)((total + rows) & 0xFF);
}
