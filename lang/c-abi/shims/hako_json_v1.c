#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "hako_json_v1.h"

// Reuse vendored yyjson under plugins/nyash-json-plugin/c/yyjson (passed via -I)
#include "yyjson.h"

static int set_err_owned(char** err_out, const char* msg) {
    if (!err_out) return -1;
    if (!msg) { *err_out = NULL; return -1; }
    size_t n = strlen(msg);
    char* p = (char*)malloc(n + 1);
    if (!p) { *err_out = NULL; return -1; }
    memcpy(p, msg, n + 1);
    *err_out = p;
    return -1;
}

#include "hako_json_v1_borrowed.inc"

int hako_json_v1_validate_file(const char* path, char** err_out) {
    yyjson_doc* doc = hako_json_v1_read_owned_file(path, err_out);
    if (!doc) return -1;
    int rc = hako_json_v1_validate_root(yyjson_doc_get_root(doc), err_out);
    yyjson_doc_free(doc);
    return rc;
}
