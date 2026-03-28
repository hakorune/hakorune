#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 180000;
  const char seed[] = "line-seed-abcdef";
  const int len = (int)(sizeof(seed) - 1);
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    const int split = len / 2;
    char out[40];
    memcpy(out, seed, (size_t)split);
    out[split] = 'x';
    out[split + 1] = 'x';
    memcpy(out + split + 2, seed + split, (size_t)(len - split));
    out[len + 2] = '\0';
    acc += (int64_t)(len + 2);
  }

  return (int)((acc + len) & 0xFF);
}
