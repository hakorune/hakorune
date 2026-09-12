/* Native child observer: getenv distinguishes absent from present-empty. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void json_value(FILE* out, const char* value) {
  if (!value) { fputs("null", out); return; }
  fputc('"', out);
  for (const unsigned char* p = (const unsigned char*)value; *p; ++p) {
    if (*p == '"' || *p == '\\') fputc('\\', out);
    if (*p < 32) fprintf(out, "\\u%04x", *p);
    else fputc(*p, out);
  }
  fputc('"', out);
}

int main(int argc, char** argv) {
  const char* output = NULL;
  const char* record = getenv("HAKO_AOT_CHILD_ENV_RECORD");
  for (int i = 1; i + 1 < argc; ++i)
    if (!strcmp(argv[i], "--out")) output = argv[i + 1];
  if (!record || !output) return 2;
  FILE* f = fopen(record, "wb");
  if (!f) return 3;
  fputs("{\"hako\":", f);
  json_value(f, getenv("HAKO_LLVM_OPT_LEVEL"));
  fputs(",\"nyash\":", f);
  json_value(f, getenv("NYASH_LLVM_OPT_LEVEL"));
  fputs("}\n", f);
  if (fclose(f)) return 4;
  f = fopen(output, "wb");
  if (!f) return 5;
  fputs("probe", f);
  return fclose(f) ? 6 : 0;
}
