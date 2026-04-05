#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t size = 128;
  const int64_t ops = 800000;
  const int base_len = 16; /* "line-seed-abcdef" */
  char arr[128][32];
  char text[32] = "line-seed-abcdef";

  for (int64_t i = 0; i < size; i++) {
    memcpy(arr[i], text, (size_t)base_len + 1);
  }

  volatile int64_t sum = 0;
  for (int64_t i = 0; i < ops; i++) {
    const int64_t idx = i % size;
    char out[32];
    memcpy(out, text, (size_t)base_len);
    out[base_len] = 'x';
    out[base_len + 1] = 'y';
    out[base_len + 2] = '\0';

    memcpy(arr[idx], out, (size_t)base_len + 3);
    sum += (int64_t)strlen(arr[idx]);

    memcpy(text, out + 2, (size_t)base_len);
    text[base_len] = '\0';
  }

  return (int)(sum & 0xFF);
}
