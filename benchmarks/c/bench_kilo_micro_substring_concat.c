#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 300000;
  const int len = 16; /* "line-seed-abcdef" */
  char text[32] = "line-seed-abcdef";
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    const int split = len / 2;
    char out[40];
    memcpy(out, text, (size_t)split);
    out[split] = 'x';
    out[split + 1] = 'x';
    memcpy(out + split + 2, text + split, (size_t)(len - split));
    out[len + 2] = '\0';

    acc += (int64_t)(len + 2);

    /* Rotate while preserving base length. */
    memcpy(text, out + 1, (size_t)len);
    text[len] = '\0';
  }

  return (int)(acc & 0xFF);
}
