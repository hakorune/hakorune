// APP-PERF-02: kilo kernel benchmark (small) - C reference
// Deterministic text-buffer workload inspired by enhanced_kilo_editor
// 60,000 iterations with string edit + replace scan

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void insert_chunk(char **lines, int row, const char *text) {
    char *line = lines[row];
    size_t line_len = strlen(line);
    size_t text_len = strlen(text);
    size_t split = line_len / 2;
    char *out = (char *)malloc(line_len + text_len + 1);
    memcpy(out, line, split);
    memcpy(out + split, text, text_len);
    memcpy(out + split + text_len, line + split, line_len - split + 1);
    free(line);
    lines[row] = out;
}

static void append_if_contains(char **lines, const char *pattern, const char *suffix) {
    size_t suffix_len = strlen(suffix);
    for (int i = 0; i < 64; i++) {
        if (strstr(lines[i], pattern) == NULL) {
            continue;
        }
        size_t old_len = strlen(lines[i]);
        char *out = (char *)malloc(old_len + suffix_len + 1);
        memcpy(out, lines[i], old_len);
        memcpy(out + old_len, suffix, suffix_len + 1);
        free(lines[i]);
        lines[i] = out;
    }
}

int main(void) {
    const int rows = 64;
    volatile int64_t ops = 60000;
    int64_t undo = 0;
    char *lines[64];

    for (int i = 0; i < rows; i++) {
        char buf[32];
        snprintf(buf, sizeof(buf), "line-%d", i);
        lines[i] = strdup(buf);
    }

    int64_t i = 0;
    int64_t ops_val = ops;
    while (i < ops_val) {
        int row = (int)(i % rows);
        insert_chunk(lines, row, "xx");
        undo++;

        if ((i % 8) == 0) {
            append_if_contains(lines, "line", "ln");
        }

        i++;
    }

    int64_t total = 0;
    for (int j = 0; j < rows; j++) {
        total += (int64_t)strlen(lines[j]);
    }
    for (int j = 0; j < rows; j++) {
        free(lines[j]);
    }

    int64_t result = total + undo;
    return (int)(result & 0xFF);
}
