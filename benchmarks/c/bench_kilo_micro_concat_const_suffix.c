#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 600000;
  const int base_len = 16; /* "line-seed-abcdef" */
  char text[32] = "line-seed-abcdef";
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    char out[32];
    memcpy(out, text, (size_t)base_len);
    out[base_len] = 'x';
    out[base_len + 1] = 'y';
    out[base_len + 2] = '\0';

    acc += (int64_t)(base_len + 2);

    memcpy(text, out + 2, (size_t)base_len);
    text[base_len] = '\0';
  }

  return (int)(acc & 0xFF);
}
