#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 800000;
  const char text[] = "line-seed-abcdef";
  const char suffix[] = "xy";
  const int text_len = 16;
  const int suffix_len = 2;
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    char out[32];
    memcpy(out, text, (size_t)text_len);
    memcpy(out + text_len, suffix, (size_t)suffix_len);
    out[text_len + suffix_len] = '\0';
    acc += (int64_t)(text_len + suffix_len);
  }

  return (int)((acc + text_len + suffix_len) & 0xFF);
}
