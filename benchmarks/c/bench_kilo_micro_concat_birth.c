#include <stdint.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  const int64_t ops = 800000;
  const char text[] = "line-seed-abcdef";
  const char suffix[] = "xy";
  const size_t text_len = 16;
  const size_t suffix_len = 2;
  char *last = NULL;

  for (int64_t i = 0; i < ops; i++) {
    char *out = (char *)malloc(text_len + suffix_len + 1);
    if (out == NULL) {
      free(last);
      return 2;
    }
    memcpy(out, text, text_len);
    memcpy(out + text_len, suffix, suffix_len);
    out[text_len + suffix_len] = '\0';
    free(last);
    last = out;
  }

  const int rc = (int)(((int64_t)strlen(last) + ops) & 0xFF);
  free(last);
  return rc;
}
